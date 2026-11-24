// =============================================================================
//  ALLTube : main.rs  (le point d'entrée, là où tout commence)
// =============================================================================
//  COMMENT LIRE CE PROJET (le plan de la maison) :
//
//    src/                  -> le "VISAGE" : la page web (HTML, CSS, JavaScript)
//                             que l'utilisateur voit et sur laquelle il clique.
//    src-tauri/src/        -> le "CERVEAU" : le code Rust (ce dossier).
//       main.rs            -> démarre le programme + liste des commandes (ce fichier)
//       settings.rs        -> les réglages (page Paramètres)
//       formats.rs         -> les formats MP4, MP3, FLAC... et leurs recettes
//       input.rs           -> vérifie le lien ou les mots-clés tapés
//       engine.rs          -> comment lancer yt-dlp et ffmpeg en toute sécurité
//       downloads.rs       -> la file de téléchargements
//       media.rs           -> ffmpeg : conversion et découpe (les ciseaux)
//
//  Le visage et le cerveau se parlent avec des "commandes" :
//    la page dit  invoke("start_download", ...)  et le Rust répond.
//  Le Rust peut aussi envoyer des "événements" à la page
//    (par exemple "ce téléchargement vient d'avancer").
// =============================================================================

// Sous Windows, cette ligne cache la fenêtre noire de console derrière
// l'application. Elle ne s'active que dans la version finale (release).
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// On déclare nos fichiers : ce sont les "modules" du programme.
mod downloads;
mod engine;
mod formats;
mod input;
mod media;
mod settings;

use tauri::{AppHandle, Emitter, Manager, State};
use tauri_plugin_dialog::DialogExt;

use downloads::{lock, AppState, Download};
use formats::{CustomCodecs, FormatInfo, FORMATS};
use settings::Settings;

// =============================================================================
//  LES COMMANDES : tout ce que la page web a le droit de demander.
//  `#[tauri::command]` = "cette fonction peut être appelée depuis la page".
//  Règle de sécurité : chaque commande revérifie TOUT ce qu'elle reçoit.
// =============================================================================

/// Donne les réglages actuels.
#[tauri::command]
fn get_settings(state: State<'_, AppState>) -> Settings {
    lock(&state.settings).clone()
}

/// Reçoit de nouveaux réglages, les nettoie, les enregistre, et les renvoie.
#[tauri::command]
fn save_settings(app: AppHandle, state: State<'_, AppState>, mut settings: Settings) -> Result<Settings, String> {
    settings.sanitize(); // on ne fait pas confiance à la page !
    settings.save(&state.config_dir).map_err(|e| format!("io:{e}"))?;
    *lock(&state.settings) = settings.clone();
    downloads::pump(&app); // le nombre de téléchargements simultanés a peut-être changé
    Ok(settings)
}

/// La liste des formats (pour le menu déroulant).
#[tauri::command]
fn get_formats() -> Vec<FormatInfo> {
    FORMATS.to_vec()
}

/// La liste des téléchargements récents.
#[tauri::command]
fn get_downloads(state: State<'_, AppState>) -> Vec<Download> {
    lock(&state.downloads).clone()
}

/// Le dossier de téléchargement par défaut (bouton "Dossier par défaut").
#[tauri::command]
fn get_default_download_dir(app: AppHandle) -> String {
    downloads::default_download_dir(&app).to_string_lossy().to_string()
}

/// Quels moteurs manquent ? (liste vide = tout va bien)
#[tauri::command]
fn engine_status() -> Vec<String> {
    engine::missing()
}

/// Numéro de version affiché en bas des paramètres.
#[tauri::command]
fn app_version(app: AppHandle) -> String {
    app.package_info().version.to_string()
}

/// Fenêtre "choisir un dossier" (bouton Parcourir).
#[tauri::command]
async fn choose_folder(app: AppHandle) -> Option<String> {
    let picked = app.dialog().file().blocking_pick_folder()?;
    picked.into_path().ok().map(|p| p.to_string_lossy().to_string())
}

/// Fenêtre "choisir une vidéo ou un son" (pour les ciseaux).
#[tauri::command]
async fn choose_media(app: AppHandle) -> Option<String> {
    let picked = app
        .dialog()
        .file()
        .add_filter("Media", &media::MEDIA_EXTENSIONS)
        .blocking_pick_file()?;
    picked.into_path().ok().map(|p| p.to_string_lossy().to_string())
}

/// Clic sur OK : démarre un téléchargement.
#[tauri::command]
fn start_download(
    app: AppHandle,
    input: String,
    format_id: String,
    custom: Option<CustomCodecs>,
) -> Result<Download, String> {
    downloads::start(&app, input, format_id, custom)
}

/// Menu "..." > Retirer.
#[tauri::command]
fn remove_download(app: AppHandle, id: u64) {
    downloads::remove(&app, id);
}

/// Bouton "Tout effacer".
#[tauri::command]
fn clear_downloads(app: AppHandle) {
    downloads::clear(&app);
}

/// Menu "..." > Ouvrir.
#[tauri::command]
fn open_download(app: AppHandle, id: u64) -> Result<(), String> {
    downloads::open_file(&app, id)
}

/// Menu "..." > Ouvrir le dossier.
#[tauri::command]
fn reveal_download(app: AppHandle, id: u64) -> Result<(), String> {
    downloads::reveal_file(&app, id)
}

/// Menu "..." > Découper la vidéo : donne le chemin du fichier à charger dans les ciseaux.
#[tauri::command]
fn download_media_path(app: AppHandle, id: u64) -> Result<String, String> {
    downloads::file_of(&app, id).map(|p| p.to_string_lossy().to_string())
}

/// Ciseaux : durée du fichier choisi.
#[tauri::command]
async fn probe_media(path: String) -> Result<f64, String> {
    media::probe(&path).await
}

/// Ciseaux : découpe entre `start` et `end` (secondes).
#[tauri::command]
async fn cut_media(app: AppHandle, path: String, start: f64, end: f64, precise: bool) -> Result<Download, String> {
    media::cut(&app, path, start, end, precise).await
}

// =============================================================================
//  main() : la toute première fonction exécutée.
// =============================================================================
fn main() {
    tauri::Builder::default()
        // On branche les 3 petits modules officiels.
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_opener::init())
        // Au démarrage : on prépare la mémoire de l'application.
        .setup(|app| {
            // Dossier où ALLTube range ses petits fichiers (réglages, historique).
            let config_dir = app.path().app_config_dir()?;
            std::fs::create_dir_all(&config_dir)?;
            app.manage(AppState::new(config_dir));
            Ok(())
        })
        // Quand l'utilisateur dépose un fichier sur la fenêtre, on prévient la page
        // (elle l'utilisera pour les ciseaux).
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::DragDrop(tauri::DragDropEvent::Drop { paths, .. }) = event {
                let list: Vec<String> = paths
                    .iter()
                    .filter_map(|p| p.to_str().map(|s| s.to_string()))
                    .collect();
                let _ = window.emit("files-dropped", list);
            }
        })
        // La liste de TOUTES les commandes autorisées. Ce qui n'est pas ici est interdit.
        .invoke_handler(tauri::generate_handler![
            get_settings,
            save_settings,
            get_formats,
            get_downloads,
            get_default_download_dir,
            engine_status,
            app_version,
            choose_folder,
            choose_media,
            start_download,
            remove_download,
            clear_downloads,
            open_download,
            reveal_download,
            download_media_path,
            probe_media,
            cut_media,
        ])
        .run(tauri::generate_context!())
        .expect("Erreur au lancement d'ALLTube");
}
