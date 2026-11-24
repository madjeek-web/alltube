// =============================================================================
//  engine.rs : les "MOTEURS" (yt-dlp, ffmpeg, ffprobe) et comment les lancer
// =============================================================================
//  ALLTube ne sait pas, tout seul, parler à YouTube, TikTok, etc. Ces sites
//  changent tout le temps. Il existe un outil libre et célèbre qui suit ces
//  changements : yt-dlp. (noTube fonctionne aussi comme ça.)
//  Notre rôle : lui donner les bons ordres et afficher joliment le résultat.
//
//    - yt-dlp  : trouve et télécharge la vidéo ou le son.
//    - ffmpeg  : convertit et découpe les fichiers audio/vidéo.
//    - ffprobe : lit les informations d'un fichier (sa durée par exemple).
//
//  SÉCURITÉ, 4 règles qu'on respecte partout :
//   1. On ne passe JAMAIS par un "shell" (cmd, PowerShell, bash). On lance le
//      programme directement avec une liste d'arguments. Un lien piégé comme
//      "video; del *.*" reste alors un simple texte, il n'est jamais exécuté.
//   2. On ne cherche les moteurs QUE dans le dossier de l'application (pas dans
//      le PATH de l'ordinateur), pour qu'un faux "yt-dlp.exe" planqué ailleurs
//      ne puisse pas prendre sa place.
//   3. Aucune fenêtre noire ne s'ouvre (sinon ça fait peur et ça clignote).
//   4. Les moteurs sont lancés avec les droits normaux de l'utilisateur.
// =============================================================================

use std::path::{Path, PathBuf};
use std::process::Stdio;

use tokio::io::{AsyncBufRead, AsyncBufReadExt, BufReader};
use tokio::process::{Child, ChildStderr, Command};

/// Drapeau Windows qui veut dire "ne crée pas de fenêtre de console".
#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x0800_0000;

/// Retrouve l'emplacement d'un moteur ("yt-dlp", "ffmpeg" ou "ffprobe").
/// Quand l'application est installée, les moteurs sont posés à côté de
/// l'exécutable principal (alltube.exe).
pub fn sidecar(name: &str) -> Option<PathBuf> {
    let file = if cfg!(windows) { format!("{name}.exe") } else { name.to_string() };

    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let candidate = dir.join(&file);
            if candidate.is_file() {
                return Some(candidate);
            }
        }
    }

    // Pendant les tests du développeur seulement (cargo tauri dev), on accepte
    // un moteur installé sur l'ordinateur. Dans la version finale : NON.
    if cfg!(debug_assertions) {
        return Some(PathBuf::from(file));
    }
    None
}

/// Liste des moteurs introuvables (vide = tout va bien).
pub fn missing() -> Vec<String> {
    ["yt-dlp", "ffmpeg", "ffprobe"]
        .iter()
        .filter(|name| match sidecar(name) {
            Some(p) => p.components().count() > 1 && !p.is_file(),
            None => true,
        })
        .map(|name| name.to_string())
        .collect()
}

/// Dossier qui contient ffmpeg (on le donne à yt-dlp pour qu'il le trouve).
pub fn ffmpeg_dir() -> Option<PathBuf> {
    let dir = sidecar("ffmpeg")?.parent()?.to_path_buf();
    if dir.as_os_str().is_empty() { None } else { Some(dir) }
}

/// Prépare une commande "propre" pour lancer un moteur.
pub fn command(program: &Path) -> Command {
    let mut cmd = Command::new(program);
    cmd.stdin(Stdio::null()) // le moteur ne peut pas nous demander de taper quoi que ce soit
        .stdout(Stdio::piped()) // on lit ce qu'il écrit (la progression)
        .stderr(Stdio::piped()) // on lit ses messages d'erreur
        .kill_on_drop(true) // si on oublie le moteur, il est arrêté
        // Force l'UTF-8 pour que les accents et les caractères chinois
        // des titres arrivent sans être abîmés.
        .env("PYTHONIOENCODING", "utf-8")
        .env("PYTHONUTF8", "1");
    #[cfg(windows)]
    cmd.creation_flags(CREATE_NO_WINDOW);
    cmd
}

/// Arrête un moteur ET tous ses "enfants".
/// Sous Windows, yt-dlp.exe lance un second processus. Si on tuait seulement
/// le premier, le second continuerait à télécharger en cachette !
/// "taskkill /T" règle ça.
pub async fn kill_tree(child: &mut Child) {
    #[cfg(windows)]
    {
        if let Some(pid) = child.id() {
            // Chemin complet vers taskkill.exe (jamais cherché dans le PATH).
            let root = std::env::var("SystemRoot").unwrap_or_else(|_| "C:\\Windows".to_string());
            let taskkill = PathBuf::from(root).join("System32").join("taskkill.exe");
            let pid_text = pid.to_string();
            let mut cmd = Command::new(taskkill);
            cmd.arg("/PID").arg(&pid_text).arg("/T").arg("/F");
            cmd.creation_flags(CREATE_NO_WINDOW);
            let _ = cmd.output().await;
        }
    }
    let _ = child.kill().await;
}

/// Lit UNE ligne de texte envoyée par un moteur.
/// Retourne None quand il n'y a plus rien à lire.
/// On lit des octets puis on convertit "avec tolérance" : si un caractère est
/// invalide, on le remplace au lieu de planter.
pub async fn read_line<R: AsyncBufRead + Unpin>(reader: &mut R, buf: &mut Vec<u8>) -> Option<String> {
    buf.clear();
    match reader.read_until(b'\n', buf).await {
        Ok(0) | Err(_) => None,
        Ok(_) => Some(String::from_utf8_lossy(buf).trim_end().to_string()),
    }
}

/// Lit en tâche de fond les messages d'erreur d'un moteur et garde la
/// ligne la plus utile (la dernière qui commence par "ERROR", sinon la dernière).
/// Il FAUT vider ce tuyau, sinon le moteur se bloque quand il est plein.
pub fn collect_stderr(stderr: Option<ChildStderr>) -> tauri::async_runtime::JoinHandle<String> {
    tauri::async_runtime::spawn(async move {
        let mut last = String::new();
        if let Some(s) = stderr {
            let mut reader = BufReader::new(s);
            let mut buf = Vec::new();
            while let Some(line) = read_line(&mut reader, &mut buf).await {
                if line.trim().is_empty() {
                    continue;
                }
                if line.starts_with("ERROR") || !last.starts_with("ERROR") {
                    last = line;
                }
            }
        }
        last
    })
}

/// Nettoie un message d'erreur pour l'afficher : enlève "ERROR:" et le raccourcit.
pub fn clean_error(text: &str) -> String {
    let t = text.trim().trim_start_matches("ERROR:").trim();
    t.chars().take(300).collect()
}

/// Demande à ffprobe la durée d'un fichier, en secondes.
pub async fn probe_duration(path: &Path) -> Option<f64> {
    let ffprobe = sidecar("ffprobe")?;
    let mut cmd = command(&ffprobe);
    cmd.args([
        "-v", "error", "-show_entries", "format=duration",
        "-of", "default=noprint_wrappers=1:nokey=1", "-i",
    ]);
    cmd.arg(path);
    let out = cmd.output().await.ok()?;
    String::from_utf8_lossy(&out.stdout).trim().parse::<f64>().ok().filter(|d| d.is_finite() && *d > 0.0)
}
