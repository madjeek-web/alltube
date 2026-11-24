// =============================================================================
//  downloads.rs : la FILE DE TÉLÉCHARGEMENTS (le coeur du logiciel)
// =============================================================================
//  Ce fichier gère la vie complète d'un téléchargement :
//
//    En attente -> Téléchargement -> (Conversion) -> Terminé   ou   Erreur
//
//  Il s'occupe de :
//   - vérifier le lien tapé par l'utilisateur,
//   - respecter le nombre de téléchargements simultanés (Paramètres),
//   - respecter le délai entre deux téléchargements (Paramètres),
//   - lancer yt-dlp, lire sa progression, puis lancer ffmpeg si besoin,
//   - garder un historique (le fichier history.json),
//   - annuler / retirer / tout effacer,
//   - ouvrir le fichier ou son dossier.
//
//  IL N'Y A AUCUNE LIMITE JOURNALIÈRE ICI : pas de compteur, pas de compte,
//  pas de serveur. Tu peux télécharger autant que tu veux.
// =============================================================================

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_notification::NotificationExt;
use tauri_plugin_opener::OpenerExt;
use tokio::io::BufReader;
use tokio::sync::Notify;

use crate::engine;
use crate::formats::{self, CustomCodecs, Plan};
use crate::media;
use crate::settings::{self, Settings};

/// On garde au maximum 100 téléchargements dans l'historique.
const MAX_HISTORY: usize = 100;

/// Un "Mutex" est un cadenas : un seul morceau du programme à la fois peut
/// toucher aux données qu'il protège. Cette petite fonction prend le cadenas
/// (et ne plante pas si un autre morceau a eu un problème pendant qu'il le tenait).
pub fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(|e| e.into_inner())
}

/// Les différentes étapes de la vie d'un téléchargement.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Queued,      // en attente de son tour
    Downloading, // en cours de téléchargement
    Converting,  // ffmpeg convertit le fichier
    Done,        // terminé
    Error,       // raté
}

/// Une ligne de la liste "Téléchargements récents".
/// Cette fiche est envoyée telle quelle à la page web (en JSON).
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Download {
    pub id: u64,
    pub title: String,
    pub uploader: String,
    pub duration: Option<f64>,
    pub thumbnail: Option<String>,
    pub size: Option<u64>,
    pub format_id: String,
    pub format_label: String,
    pub status: Status,
    pub progress: f32, // de 0.0 à 1.0
    pub path: Option<String>,
    pub error: Option<String>,
}

/// Les infos dont un téléchargement a besoin pour s'exécuter.
#[derive(Clone)]
struct Job {
    plan: Plan,
    target: String,        // le lien (ou "ytsearch1:mots clés")
    settings: Settings,    // les réglages au moment où on a cliqué sur OK
    cancel: Arc<Notify>,   // la "cloche" pour annuler
}

/// La MÉMOIRE de l'application, partagée entre tous les morceaux du programme.
pub struct AppState {
    pub settings: Mutex<Settings>,
    pub downloads: Mutex<Vec<Download>>,
    jobs: Mutex<HashMap<u64, Job>>,
    next_id: AtomicU64,
    not_before: Mutex<Instant>, // pas de nouveau téléchargement avant cet instant
    pub config_dir: PathBuf,
}

impl AppState {
    /// Au démarrage : relit les réglages et l'historique.
    pub fn new(config_dir: PathBuf) -> Self {
        let settings = Settings::load(&config_dir);
        let history = load_history(&config_dir);
        let next = history.iter().map(|d| d.id).max().unwrap_or(0) + 1;
        AppState {
            settings: Mutex::new(settings),
            downloads: Mutex::new(history),
            jobs: Mutex::new(HashMap::new()),
            next_id: AtomicU64::new(next),
            not_before: Mutex::new(Instant::now()),
            config_dir,
        }
    }
}

/// Relit history.json. Un téléchargement qui était "en cours" quand on a fermé
/// le logiciel est marqué "interrompu".
fn load_history(dir: &Path) -> Vec<Download> {
    let mut list: Vec<Download> = std::fs::read_to_string(dir.join("history.json"))
        .ok()
        .and_then(|text| serde_json::from_str(&text).ok())
        .unwrap_or_default();
    for d in list.iter_mut() {
        if d.status != Status::Done && d.status != Status::Error {
            d.status = Status::Error;
            d.error = Some("interrupted".to_string());
        }
    }
    list.truncate(MAX_HISTORY);
    list
}

/// Sauvegarde l'historique (seulement les téléchargements terminés ou en erreur).
fn persist(state: &AppState) {
    let list: Vec<Download> = lock(&state.downloads)
        .iter()
        .filter(|d| d.status == Status::Done || d.status == Status::Error)
        .take(MAX_HISTORY)
        .cloned()
        .collect();
    if let Err(e) = settings::write_json_atomic(&state.config_dir.join("history.json"), &list) {
        eprintln!("Impossible de sauvegarder l'historique : {e}");
    }
}

/// Modifie une ligne de la liste, puis PRÉVIENT la page web pour qu'elle se mette à jour.
fn update<F: FnOnce(&mut Download)>(app: &AppHandle, id: u64, change: F) {
    let state = app.state::<AppState>();
    let snapshot = {
        let mut list = lock(&state.downloads);
        list.iter_mut().find(|d| d.id == id).map(|d| {
            change(d);
            d.clone()
        })
    };
    if let Some(d) = snapshot {
        let _ = app.emit("download-update", &d);
    }
}

/// Dossier de téléchargement par défaut : Téléchargements/ALLTube.
pub fn default_download_dir(app: &AppHandle) -> PathBuf {
    let base = app.path().download_dir().unwrap_or_else(|_| std::env::temp_dir());
    base.join("ALLTube")
}

/// Dossier choisi dans les paramètres (ou le dossier par défaut).
pub fn download_dir(app: &AppHandle, s: &Settings) -> PathBuf {
    if !s.download_dir.is_empty() {
        let p = PathBuf::from(&s.download_dir);
        if p.is_absolute() {
            return p;
        }
    }
    default_download_dir(app)
}

/// Trouve un nom de fichier libre : "video.mp4", puis "video (2).mp4", etc.
/// (Pour ne jamais écraser un fichier existant.)
pub fn unique_path(dir: &Path, stem: &str, ext: &str) -> PathBuf {
    let first = dir.join(format!("{stem}.{ext}"));
    if !first.exists() {
        return first;
    }
    for n in 2..10_000 {
        let candidate = dir.join(format!("{stem} ({n}).{ext}"));
        if !candidate.exists() {
            return candidate;
        }
    }
    first
}

// =============================================================================
//  Démarrer, ordonnancer, exécuter
// =============================================================================

/// Appelée quand l'utilisateur clique sur OK (ou colle un lien).
pub fn start(app: &AppHandle, input: String, format_id: String, custom: Option<CustomCodecs>) -> Result<Download, String> {
    let state = app.state::<AppState>();
    let target = crate::input::normalize_input(&input)?;
    if !engine::missing().is_empty() {
        return Err("engine_missing".to_string());
    }
    let settings = lock(&state.settings).clone();
    let plan = formats::build_plan(&format_id, custom.as_ref(), &settings)?;

    let id = state.next_id.fetch_add(1, Ordering::SeqCst);
    let item = Download {
        id,
        // En attendant de connaître le vrai titre, on affiche ce que l'utilisateur a tapé.
        title: input.trim().chars().take(200).collect(),
        uploader: String::new(),
        duration: None,
        thumbnail: None,
        size: None,
        format_id,
        format_label: plan.label.clone(),
        status: Status::Queued,
        progress: 0.0,
        path: None,
        error: None,
    };
    lock(&state.downloads).insert(0, item.clone());
    lock(&state.jobs).insert(id, Job { plan, target, settings, cancel: Arc::new(Notify::new()) });
    let _ = app.emit("download-update", &item);
    pump(app);
    Ok(item)
}

/// Le "chef d'orchestre" : regarde qui attend, combien tournent déjà,
/// et lance les suivants si la place le permet.
/// On l'appelle à chaque fois que quelque chose change (nouveau, fini, réglage...).
pub fn pump(app: &AppHandle) {
    let state = app.state::<AppState>();
    let (max_parallel, delay) = {
        let s = lock(&state.settings);
        (s.parallel as usize, s.delay_secs)
    };

    // Délai entre deux téléchargements : seulement quand on en fait un à la fois.
    if max_parallel == 1 && delay > 0 {
        let wait = lock(&state.not_before).saturating_duration_since(Instant::now());
        if !wait.is_zero() {
            let app2 = app.clone();
            tauri::async_runtime::spawn(async move {
                tokio::time::sleep(wait).await;
                pump(&app2);
            });
            return;
        }
    }

    // On choisit (dans l'ordre d'arrivée) ceux qu'on peut démarrer maintenant.
    let to_start: Vec<u64> = {
        let mut list = lock(&state.downloads);
        let running = list
            .iter()
            .filter(|d| d.status == Status::Downloading || d.status == Status::Converting)
            .count();
        let free = max_parallel.saturating_sub(running);
        let mut ids = Vec::new();
        // .rev() : la liste est du plus récent au plus ancien, on veut l'inverse.
        for d in list.iter_mut().rev() {
            if ids.len() >= free {
                break;
            }
            if d.status == Status::Queued {
                d.status = Status::Downloading;
                ids.push(d.id);
            }
        }
        ids
    };

    for id in to_start {
        tauri::async_runtime::spawn(run_job(app.clone(), id));
    }
}

/// Exécute UN téléchargement du début à la fin.
async fn run_job(app: AppHandle, id: u64) {
    let state = app.state::<AppState>();
    let job = lock(&state.jobs).get(&id).cloned();
    let Some(job) = job else { return }; // retiré entre-temps
    update(&app, id, |_| {}); // prévient la page : "ça démarre"

    let result = execute(&app, id, &job).await;
    lock(&state.jobs).remove(&id);

    match result {
        Ok(path) => finish_ok(&app, id, &path),
        Err(e) if e == "cancelled" => {} // l'utilisateur a retiré la ligne : rien à faire
        Err(e) => update(&app, id, move |d| {
            d.status = Status::Error;
            d.error = Some(e);
        }),
    }

    // Délai entre deux téléchargements.
    let (parallel, delay) = {
        let s = lock(&state.settings);
        (s.parallel, s.delay_secs)
    };
    if parallel == 1 && delay > 0 {
        *lock(&state.not_before) = Instant::now() + Duration::from_secs(delay as u64);
    }
    persist(&state);
    pump(&app); // la place est libre : au suivant !
}

/// Le vrai travail : lance yt-dlp, lit sa progression, convertit si besoin.
/// Retourne le chemin du fichier final, ou un code d'erreur.
async fn execute(app: &AppHandle, id: u64, job: &Job) -> Result<PathBuf, String> {
    let ytdlp = engine::sidecar("yt-dlp").ok_or("engine_missing")?;
    let dir = download_dir(app, &job.settings);
    std::fs::create_dir_all(&dir).map_err(|e| format!("io:{e}"))?;

    // ---------- 1. On construit la liste d'arguments pour yt-dlp ----------
    let mut args: Vec<String> = [
        "--ignore-config",   // ignore tout fichier de config trouvé sur l'ordinateur
        "--no-playlist",     // un lien de playlist = seulement la vidéo du lien
        "--newline",         // une ligne par progression (facile à lire)
        "--progress",        // affiche la progression même en mode silencieux
        "--windows-filenames", // retire les caractères interdits dans les noms de fichiers
        "--no-overwrites",   // n'écrase jamais un fichier existant
        "--no-mtime",        // la date du fichier = aujourd'hui (pas la date de la vidéo)
        "-N", "4",           // 4 morceaux téléchargés en parallèle (plus rapide)
        "--retries", "5",
        "--fragment-retries", "5",
        "--socket-timeout", "30",
        "-o", "%(title).150B.%(ext)s", // nom du fichier = titre de la vidéo
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    args.push("-P".to_string());
    args.push(dir.to_string_lossy().to_string());

    // Messages "maison" que yt-dlp écrira et qu'on saura relire.
    //  ALLTUBE_PROGRESS <octets reçus> <total> <total estimé> <codec vidéo>
    args.push("--progress-template".to_string());
    args.push(
        "download:ALLTUBE_PROGRESS %(progress.downloaded_bytes)s %(progress.total_bytes)s %(progress.total_bytes_estimate)s %(info.vcodec)s"
            .to_string(),
    );
    //  ALLTUBE_META {titre, auteur, durée, miniature} avant le début du téléchargement
    args.push("--print".to_string());
    args.push("before_dl:ALLTUBE_META %(.{title,uploader,duration,thumbnail})j".to_string());
    //  ALLTUBE_FILE <chemin du fichier final>
    args.push("--print".to_string());
    args.push("after_move:ALLTUBE_FILE %(filepath)s".to_string());

    if let Some(ff_dir) = engine::ffmpeg_dir() {
        args.push("--ffmpeg-location".to_string());
        args.push(ff_dir.to_string_lossy().to_string());
    }
    if job.settings.use_cookies {
        // Le navigateur vient de la liste autorisée (voir settings.rs).
        args.push("--cookies-from-browser".to_string());
        args.push(job.settings.cookies_browser.clone());
    }
    args.extend(job.plan.ytdlp_args.iter().cloned());

    // "--" = "fin des options". Tout ce qui suit est UN LIEN, jamais une option.
    // (Protection contre un lien qui commencerait par un tiret.)
    args.push("--".to_string());
    args.push(job.target.clone());

    // ---------- 2. On lance yt-dlp ----------
    let mut cmd = engine::command(&ytdlp);
    cmd.args(&args);
    let mut child = cmd.spawn().map_err(|e| format!("engine_start:{e}"))?;
    let err_task = engine::collect_stderr(child.stderr.take());
    let stdout = child.stdout.take().ok_or("engine_start")?;
    let mut reader = BufReader::new(stdout);
    let mut buf: Vec<u8> = Vec::new();

    let mut cancelled = false;
    let mut final_path: Option<PathBuf> = None;
    let mut seen_video = false;
    let mut progress: f32 = 0.0;
    let mut last_emit: Option<Instant> = None;

    // ---------- 3. On lit ce que yt-dlp écrit, ligne après ligne ----------
    loop {
        let line = tokio::select! {
            l = engine::read_line(&mut reader, &mut buf) => l,
            _ = job.cancel.notified() => { cancelled = true; None }
        };
        let Some(line) = line else { break };

        if let Some(rest) = line.strip_prefix("ALLTUBE_PROGRESS ") {
            let parts: Vec<&str> = rest.split_whitespace().collect();
            let number = |i: usize| parts.get(i).and_then(|v| v.parse::<f64>().ok());
            let total = number(1).or(number(2));
            if let (Some(done), Some(total)) = (number(0), total) {
                if total > 0.0 {
                    let fraction = (done / total).clamp(0.0, 1.0) as f32;
                    // Une vidéo = d'abord l'image (90 % du travail), puis le son (10 %).
                    let is_audio_stream = parts.get(3).is_some_and(|v| *v == "none");
                    let overall = if !is_audio_stream {
                        seen_video = true;
                        fraction * 0.9
                    } else if seen_video {
                        0.9 + fraction * 0.1
                    } else {
                        fraction
                    };
                    progress = progress.max(overall); // la barre ne recule jamais
                    // On prévient la page au maximum 4 fois par seconde.
                    if last_emit.map_or(true, |t| t.elapsed() >= Duration::from_millis(250)) {
                        last_emit = Some(Instant::now());
                        let p = progress;
                        update(app, id, move |d| d.progress = p);
                    }
                }
            }
        } else if let Some(json) = line.strip_prefix("ALLTUBE_META ") {
            if let Ok(v) = serde_json::from_str::<serde_json::Value>(json) {
                let title = v["title"].as_str().map(|s| s.chars().take(300).collect::<String>());
                let uploader = v["uploader"].as_str().map(|s| s.chars().take(200).collect::<String>());
                let duration = v["duration"].as_f64();
                // On accepte seulement une miniature en https (sécurité).
                let thumbnail = v["thumbnail"].as_str().filter(|u| u.starts_with("https://")).map(String::from);
                update(app, id, move |d| {
                    if let Some(t) = title {
                        d.title = t;
                    }
                    d.uploader = uploader.unwrap_or_default();
                    d.duration = duration;
                    d.thumbnail = thumbnail;
                });
            }
        } else if let Some(path) = line.strip_prefix("ALLTUBE_FILE ") {
            final_path = Some(PathBuf::from(path.trim()));
        }
    }

    // ---------- 4. Fin de yt-dlp ----------
    if cancelled {
        engine::kill_tree(&mut child).await;
        return Err("cancelled".to_string());
    }
    let status = child.wait().await.map_err(|e| format!("io:{e}"))?;
    let last_error = err_task.await.unwrap_or_default();
    if !status.success() {
        return Err(format!("download_failed:{}", engine::clean_error(&last_error)));
    }
    let mut path = final_path.ok_or("download_failed:")?;

    // Sécurité : le fichier doit bien se trouver DANS notre dossier de téléchargement.
    let inside = match (std::fs::canonicalize(&path), std::fs::canonicalize(&dir)) {
        (Ok(p), Ok(d)) => p.starts_with(d),
        _ => false,
    };
    if !inside {
        return Err("download_failed:unexpected file location".to_string());
    }

    // ---------- 5. Conversion éventuelle avec ffmpeg (3GP, FLV, custom) ----------
    if let Some(conv) = &job.plan.convert {
        update(app, id, |d| {
            d.status = Status::Converting;
            d.progress = 0.0;
        });
        let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("video").to_string();
        // On écrit d'abord dans un fichier temporaire, puis on remplace.
        let tmp = dir.join(format!("{stem}.alltube-converting.{}", conv.extension));

        let mut ff_args: Vec<String> = vec!["-i".to_string(), path.to_string_lossy().to_string()];
        ff_args.extend(conv.ffmpeg_args.iter().cloned());
        ff_args.push(tmp.to_string_lossy().to_string());

        let total = engine::probe_duration(&path).await;
        let app2 = app.clone();
        media::run_ffmpeg(ff_args, total, Some(job.cancel.clone()), move |f| {
            update(&app2, id, move |d| d.progress = f);
        })
        .await
        .inspect_err(|_| {
            // Échec : on supprime le fichier temporaire à moitié écrit.
            let _ = std::fs::remove_file(&tmp);
        })?;

        let _ = std::fs::remove_file(&path); // on supprime l'original (avant conversion)
        let final_name = unique_path(&dir, &stem, conv.extension);
        std::fs::rename(&tmp, &final_name).map_err(|e| format!("io:{e}"))?;
        path = final_name;
    }
    Ok(path)
}

/// Appelée quand tout s'est bien passé.
fn finish_ok(app: &AppHandle, id: u64, path: &Path) {
    let size = std::fs::metadata(path).map(|m| m.len()).ok();
    let path_text = path.to_string_lossy().to_string();
    update(app, id, move |d| {
        d.status = Status::Done;
        d.progress = 1.0;
        d.size = size;
        d.path = Some(path_text);
        d.error = None;
    });

    // Notification Windows / Mac / Linux, si elle est activée dans les paramètres.
    let state = app.state::<AppState>();
    let (notify, lang) = {
        let s = lock(&state.settings);
        (s.notify_done, s.language.clone())
    };
    if notify {
        let name = lock(&state.downloads).iter().find(|d| d.id == id).map(|d| d.title.clone()).unwrap_or_default();
        let title = match lang.as_str() {
            "en" => "Download complete",
            "zh" => "下载完成",
            _ => "Téléchargement terminé",
        };
        let _ = app.notification().builder().title(title).body(name).show();
    }
}

// =============================================================================
//  Actions du menu "..." et du bouton "Tout effacer"
// =============================================================================

/// "Retirer" : enlève une ligne (et arrête le téléchargement s'il est en cours).
pub fn remove(app: &AppHandle, id: u64) {
    let state = app.state::<AppState>();
    if let Some(job) = lock(&state.jobs).remove(&id) {
        job.cancel.notify_one(); // sonne la cloche : yt-dlp sera arrêté
    }
    lock(&state.downloads).retain(|d| d.id != id);
    persist(&state);
    pump(app);
}

/// "Tout effacer" : vide la liste (en arrêtant ce qui tourne).
/// Les fichiers déjà téléchargés restent sur le disque, bien sûr.
pub fn clear(app: &AppHandle) {
    let state = app.state::<AppState>();
    for (_, job) in lock(&state.jobs).drain() {
        job.cancel.notify_one();
    }
    lock(&state.downloads).clear();
    persist(&state);
}

/// Ajoute directement une ligne "terminée" (utilisé par la découpe de vidéo).
pub fn add_finished(app: &AppHandle, mut item: Download) -> Download {
    let state = app.state::<AppState>();
    item.id = state.next_id.fetch_add(1, Ordering::SeqCst);
    lock(&state.downloads).insert(0, item.clone());
    persist(&state);
    let _ = app.emit("download-update", &item);
    item
}

/// Retrouve le fichier d'une ligne terminée.
/// La page web envoie seulement un NUMÉRO (id), jamais un chemin : elle ne peut
/// donc pas nous faire ouvrir un fichier au hasard.
pub fn file_of(app: &AppHandle, id: u64) -> Result<PathBuf, String> {
    let state = app.state::<AppState>();
    let path = lock(&state.downloads)
        .iter()
        .find(|d| d.id == id && d.status == Status::Done)
        .and_then(|d| d.path.clone())
        .ok_or("file_not_found")?;
    media::validate_media_path(&path)
}

/// "Ouvrir" : lance le fichier avec le lecteur par défaut.
pub fn open_file(app: &AppHandle, id: u64) -> Result<(), String> {
    let path = file_of(app, id)?;
    app.opener()
        .open_path(path.to_string_lossy().to_string(), None::<&str>)
        .map_err(|e| format!("io:{e}"))
}

/// "Ouvrir le dossier" : montre le fichier dans l'explorateur.
pub fn reveal_file(app: &AppHandle, id: u64) -> Result<(), String> {
    let path = file_of(app, id)?;
    app.opener().reveal_item_in_dir(&path).map_err(|e| format!("io:{e}"))
}
