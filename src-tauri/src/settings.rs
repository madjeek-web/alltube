// =============================================================================
//  settings.rs : les RÉGLAGES de l'application (la page "Paramètres")
// =============================================================================
//  Ce fichier fait trois choses :
//   1. Il décrit à quoi ressemble la "fiche de réglages" (la struct Settings).
//   2. Il VÉRIFIE chaque réglage (fonction sanitize). C'est une règle d'or de
//      la sécurité : on ne fait jamais confiance à ce que la page web envoie.
//      Même si quelqu'un trafique la page, le Rust remet tout d'équerre.
//   3. Il sauvegarde et relit les réglages dans un fichier settings.json.
// =============================================================================

use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::Path;

use crate::formats;

/// Les 3 langues de l'interface : français, anglais, mandarin (chinois simplifié).
pub const UI_LANGUAGES: [&str; 3] = ["fr", "en", "zh"];

/// Les thèmes (les couleurs) possibles.
pub const THEMES: [&str; 2] = ["dark", "light"];

/// Navigateurs dans lesquels yt-dlp sait lire les cookies.
/// On autorise UNIQUEMENT ces noms : jamais un texte libre envoyé par la page.
pub const BROWSERS: [&str; 8] = [
    "firefox", "chrome", "edge", "brave", "opera", "vivaldi", "chromium", "safari",
];

/// Nombre maximum de langues de sous-titres.
const MAX_SUBTITLE_LANGS: usize = 10;

/// Une "struct" est comme une fiche avec plusieurs cases.
/// Chaque case a un nom et un type (texte, nombre, vrai/faux...).
/// `#[serde(default)]` veut dire : si une case manque dans le fichier
/// (par exemple après une mise à jour), on met la valeur par défaut.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default)]
pub struct Settings {
    /// Dossier où les fichiers sont enregistrés. Vide = dossier par défaut.
    pub download_dir: String,
    /// "Récupérer les sous-titres" : interrupteur.
    pub subtitles: bool,
    /// Langues des sous-titres voulues (codes comme "fr", "en", "zh").
    pub subtitle_langs: Vec<String>,
    /// "Autres pistes audio" : récupérer les autres langues audio si elles existent.
    pub extra_audio: bool,
    /// "Notification de fin".
    pub notify_done: bool,
    /// "Téléchargements simultanés" : combien en même temps.
    pub parallel: u8,
    /// "Délai entre les téléchargements" en secondes (seulement si parallel = 1).
    pub delay_secs: u32,
    /// Langue de l'interface : "fr", "en" ou "zh".
    pub language: String,
    /// Thème : "dark" ou "light".
    pub theme: String,
    /// "Utiliser les cookies du navigateur" (pour les vidéos privées).
    pub use_cookies: bool,
    /// Quel navigateur utiliser pour les cookies.
    pub cookies_browser: String,
    /// Le dernier format choisi (pour le retrouver au prochain lancement).
    pub last_format: String,
}

/// Les valeurs de départ, quand l'application est lancée pour la première fois.
impl Default for Settings {
    fn default() -> Self {
        Settings {
            download_dir: String::new(),
            subtitles: false,
            subtitle_langs: vec!["fr".to_string()],
            extra_audio: false,
            notify_done: true,
            parallel: 2,
            delay_secs: 0,
            // Le logiciel démarre en français, comme demandé.
            language: "fr".to_string(),
            theme: "dark".to_string(),
            use_cookies: false,
            cookies_browser: "firefox".to_string(),
            last_format: "mp4_fhd".to_string(),
        }
    }
}

impl Settings {
    /// Remet les réglages "propres" : corrige tout ce qui est bizarre.
    /// Appelée à CHAQUE fois qu'on reçoit ou qu'on relit des réglages.
    pub fn sanitize(&mut self) {
        // --- Dossier de téléchargement ---
        // Il doit être un chemin ABSOLU (ex : C:\Users\Moi\Downloads), pas trop long,
        // sans caractère nul. Sinon on l'efface et on prendra le dossier par défaut.
        self.download_dir = self.download_dir.trim().to_string();
        if !self.download_dir.is_empty()
            && (self.download_dir.len() > 1000
                || self.download_dir.contains('\0')
                || !Path::new(&self.download_dir).is_absolute())
        {
            self.download_dir.clear();
        }

        // --- Langues des sous-titres ---
        // On garde seulement les codes "propres" : lettres, chiffres et tiret.
        // Ça empêche d'envoyer à yt-dlp des choses dangereuses.
        let mut propres: Vec<String> = Vec::new();
        for code in &self.subtitle_langs {
            let code = code.trim().to_string();
            let valide = (2..=12).contains(&code.len())
                && code.chars().next().is_some_and(|c| c.is_ascii_alphabetic())
                && code.chars().all(|c| c.is_ascii_alphanumeric() || c == '-');
            if valide && !propres.contains(&code) && propres.len() < MAX_SUBTITLE_LANGS {
                propres.push(code);
            }
        }
        self.subtitle_langs = propres;

        // --- Nombres : on les garde dans une plage raisonnable ---
        self.parallel = self.parallel.clamp(1, 8);
        self.delay_secs = self.delay_secs.min(600);

        // --- Textes : doivent faire partie de la liste autorisée ---
        if !UI_LANGUAGES.contains(&self.language.as_str()) {
            self.language = "fr".to_string();
        }
        if !THEMES.contains(&self.theme.as_str()) {
            self.theme = "dark".to_string();
        }
        if !BROWSERS.contains(&self.cookies_browser.as_str()) {
            self.cookies_browser = "firefox".to_string();
        }
        if formats::find(&self.last_format).is_none() {
            self.last_format = "mp4_fhd".to_string();
        }
    }

    /// Lit le fichier settings.json. S'il n'existe pas ou s'il est abîmé,
    /// on repart des valeurs par défaut (le logiciel ne plante jamais pour ça).
    pub fn load(dir: &Path) -> Settings {
        let mut s: Settings = fs::read_to_string(dir.join("settings.json"))
            .ok()
            .and_then(|text| serde_json::from_str(&text).ok())
            .unwrap_or_default();
        s.sanitize();
        s
    }

    /// Enregistre les réglages dans settings.json.
    pub fn save(&self, dir: &Path) -> std::io::Result<()> {
        write_json_atomic(&dir.join("settings.json"), self)
    }
}

/// Écrit un fichier JSON de façon "atomique" :
/// on écrit d'abord dans un fichier temporaire, PUIS on le renomme.
/// Comme ça, si l'ordinateur s'éteint en plein milieu, l'ancien fichier
/// reste intact au lieu de devenir à moitié écrit (donc cassé).
pub fn write_json_atomic<T: Serialize>(path: &Path, value: &T) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(value).map_err(std::io::Error::other)?;
    let tmp = path.with_extension("json.tmp");
    let mut file = fs::File::create(&tmp)?;
    file.write_all(json.as_bytes())?;
    file.sync_all()?;
    fs::rename(&tmp, path)
}

// =============================================================================
//  TESTS (lancer avec : cargo test)
// =============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_french_dark_with_two_parallel_downloads() {
        let s = Settings::default();
        assert_eq!(s.language, "fr");
        assert_eq!(s.theme, "dark");
        assert_eq!(s.parallel, 2);
    }

    #[test]
    fn sanitize_fixes_everything_weird() {
        let mut s = Settings {
            download_dir: "..\\..\\relatif".into(),
            subtitle_langs: vec!["fr".into(), "fr".into(), "en; rm -rf".into(), "--exec".into(), "zh-Hans".into(), "x".into()],
            parallel: 200,
            delay_secs: 99999,
            language: "klingon".into(),
            theme: "rainbow".into(),
            cookies_browser: "../../evil".into(),
            last_format: "virus".into(),
            ..Settings::default()
        };
        s.sanitize();
        assert_eq!(s.download_dir, "");
        assert_eq!(s.subtitle_langs, vec!["fr".to_string(), "zh-Hans".to_string()]);
        assert_eq!(s.parallel, 8);
        assert_eq!(s.delay_secs, 600);
        assert_eq!(s.language, "fr");
        assert_eq!(s.theme, "dark");
        assert_eq!(s.cookies_browser, "firefox");
        assert_eq!(s.last_format, "mp4_fhd");
        let mut zero = Settings { parallel: 0, ..Settings::default() };
        zero.sanitize();
        assert_eq!(zero.parallel, 1);
    }

    #[test]
    fn the_three_ui_languages_are_accepted() {
        for l in UI_LANGUAGES {
            let mut s = Settings { language: l.into(), ..Settings::default() };
            s.sanitize();
            assert_eq!(s.language, l);
        }
    }

    #[test]
    fn missing_fields_in_a_file_get_default_values() {
        let s: Settings = serde_json::from_str(r#"{"language":"en"}"#).unwrap();
        assert_eq!(s.language, "en");
        assert_eq!(s.parallel, 2);
    }
}
