// =============================================================================
//  media.rs : tout ce qui touche aux FICHIERS audio/vidéo avec ffmpeg
// =============================================================================
//   - run_ffmpeg : lance ffmpeg et suit sa progression (utilisé pour
//                  convertir un fichier ET pour découper une vidéo).
//   - validate_media_path : vérifie qu'un fichier est un vrai média autorisé.
//   - cut : la fonction des CISEAUX (découper une vidéo).
// =============================================================================

use std::path::{Path, PathBuf};
use std::sync::Arc;

use tauri::{AppHandle, Emitter, Manager};
use tokio::io::BufReader;
use tokio::sync::Notify;

use crate::downloads::{self, lock, AppState, Download, Status};
use crate::engine;

/// Les seules extensions (fin du nom de fichier) qu'on accepte d'ouvrir ou de découper.
/// Un fichier ".exe" ou ".bat" est donc refusé d'office.
pub const MEDIA_EXTENSIONS: [&str; 17] = [
    "mp4", "mkv", "webm", "mov", "avi", "flv", "3gp", "m4v", "ts", "mp3", "m4a", "ogg", "opus", "flac",
    "wav", "aac", "wmv",
];

/// Vérifie qu'un chemin de fichier est sûr : absolu, existant, et de type média.
pub fn validate_media_path(text: &str) -> Result<PathBuf, String> {
    if text.len() > 4096 || text.contains('\0') {
        return Err("bad_file_type".to_string());
    }
    let path = PathBuf::from(text);
    // Un chemin ABSOLU ne peut jamais commencer par "-", donc jamais être pris
    // pour une option par ffmpeg ou ffprobe.
    if !path.is_absolute() || !path.is_file() {
        return Err("file_not_found".to_string());
    }
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase())
        .unwrap_or_default();
    if !MEDIA_EXTENSIONS.contains(&ext.as_str()) {
        return Err("bad_file_type".to_string());
    }
    Ok(path)
}

/// Lance ffmpeg avec les arguments donnés et appelle `on_progress` (0.0 à 1.0).
/// `total_secs` = durée totale prévue (pour calculer le pourcentage).
/// `cancel` = si quelqu'un "sonne" cette cloche, on arrête ffmpeg.
pub async fn run_ffmpeg<F: FnMut(f32)>(
    args: Vec<String>,
    total_secs: Option<f64>,
    cancel: Option<Arc<Notify>>,
    mut on_progress: F,
) -> Result<(), String> {
    let ffmpeg = engine::sidecar("ffmpeg").ok_or("engine_missing")?;
    let mut cmd = engine::command(&ffmpeg);
    cmd.args(["-hide_banner", "-loglevel", "error", "-nostdin", "-y", "-progress", "pipe:1", "-nostats"]);
    cmd.args(&args);

    let mut child = cmd.spawn().map_err(|e| format!("engine_start:{e}"))?;
    let err_task = engine::collect_stderr(child.stderr.take());
    let stdout = child.stdout.take().ok_or("engine_start")?;
    let mut reader = BufReader::new(stdout);
    let mut buf: Vec<u8> = Vec::new();
    let mut cancelled = false;

    loop {
        // On attend soit une nouvelle ligne de ffmpeg, soit l'ordre d'annuler.
        let line = match &cancel {
            Some(bell) => {
                tokio::select! {
                    l = engine::read_line(&mut reader, &mut buf) => l,
                    _ = bell.notified() => { cancelled = true; None }
                }
            }
            None => engine::read_line(&mut reader, &mut buf).await,
        };
        let Some(line) = line else { break };

        // ffmpeg écrit des lignes comme "out_time_us=4500000" (4,5 secondes déjà traitées).
        if let Some(value) = line.strip_prefix("out_time_us=") {
            if let (Ok(us), Some(total)) = (value.trim().parse::<f64>(), total_secs) {
                if total > 0.0 {
                    on_progress(((us / 1_000_000.0) / total).clamp(0.0, 1.0) as f32);
                }
            }
        }
    }

    if cancelled {
        engine::kill_tree(&mut child).await;
        return Err("cancelled".to_string());
    }
    let status = child.wait().await.map_err(|e| format!("io:{e}"))?;
    let last_error = err_task.await.unwrap_or_default();
    if !status.success() {
        return Err(format!("convert_failed:{}", engine::clean_error(&last_error)));
    }
    Ok(())
}

/// Les CISEAUX : découpe le fichier `path` entre `start` et `end` (en secondes).
/// `precise` = false : très rapide, on copie sans recompresser (la coupe se fait
///                     à l'image-clé la plus proche, donc à ~1 seconde près).
/// `precise` = true  : plus lent, on recompresse pour couper à l'image près.
pub async fn cut(app: &AppHandle, path: String, start: f64, end: f64, precise: bool) -> Result<Download, String> {
    let input = validate_media_path(&path)?;
    if !start.is_finite() || !end.is_finite() || start < 0.0 || end <= start {
        return Err("bad_time_range".to_string());
    }

    // Où écrire le résultat : dans le dossier de téléchargement, avec "_cut" dans le nom.
    let settings = lock(&app.state::<AppState>().settings).clone();
    let dir = downloads::download_dir(app, &settings);
    std::fs::create_dir_all(&dir).map_err(|e| format!("io:{e}"))?;
    let stem = input.file_stem().and_then(|s| s.to_str()).unwrap_or("video").to_string();
    let ext = input.extension().and_then(|s| s.to_str()).unwrap_or("mp4").to_lowercase();
    let output = downloads::unique_path(&dir, &format!("{stem}_cut"), &ext);

    let length = end - start;
    let mut args: Vec<String> = vec![
        "-ss".to_string(),
        format!("{start:.3}"),
        "-i".to_string(),
        input.to_string_lossy().to_string(),
        "-t".to_string(),
        format!("{length:.3}"),
    ];
    if !precise {
        args.extend(["-c", "copy", "-avoid_negative_ts", "make_zero"].map(String::from));
    }
    args.push(output.to_string_lossy().to_string());

    let app_for_progress = app.clone();
    run_ffmpeg(args, Some(length), None, move |fraction| {
        let _ = app_for_progress.emit("cut-progress", fraction);
    })
    .await
    .map_err(|e| e.replace("convert_failed", "cut_failed"))?;

    // Le résultat apparaît dans la liste des téléchargements, comme un fichier terminé.
    let size = std::fs::metadata(&output).map(|m| m.len()).ok();
    let title = output.file_stem().and_then(|s| s.to_str()).unwrap_or("video").to_string();
    let item = Download {
        id: 0, // sera remplacé par un vrai numéro dans add_finished
        title,
        uploader: String::new(),
        duration: Some(length),
        thumbnail: None,
        size,
        format_id: "cut".to_string(),
        format_label: "cut".to_string(),
        status: Status::Done,
        progress: 1.0,
        path: Some(output.to_string_lossy().to_string()),
        error: None,
    };
    Ok(downloads::add_finished(app, item))
}

/// Durée d'un média (pour régler les curseurs de découpe).
pub async fn probe(path: &str) -> Result<f64, String> {
    let file = validate_media_path(path)?;
    engine::probe_duration(Path::new(&file)).await.ok_or_else(|| "bad_file_type".to_string())
}
