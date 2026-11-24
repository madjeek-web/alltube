// =============================================================================
//  formats.rs : la liste des FORMATS (MP4, MP3, FLAC...) et leurs "recettes"
// =============================================================================
//  Pour chaque format que l'utilisateur peut choisir, ce fichier sait :
//   - son nom et son badge (ex : "MP4 FHD" et "1080p"),
//   - quels réglages donner à yt-dlp (le moteur qui télécharge),
//   - s'il faut ensuite convertir le fichier avec ffmpeg (3GP, FLV, custom).
//
//  SÉCURITÉ : la page web envoie seulement un IDENTIFIANT ("mp4_fhd").
//  C'est ICI, dans le Rust, qu'on transforme l'identifiant en vrais réglages.
//  Jamais un texte libre de la page ne finit dans une commande.
// =============================================================================

use serde::{Deserialize, Serialize};

use crate::settings::Settings;

/// Groupe d'affichage dans le menu des formats.
#[derive(Serialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Group {
    Video,
    Audio,
}

/// Ce que la page web a besoin de savoir pour AFFICHER un format.
#[derive(Serialize, Clone, Copy)]
pub struct FormatInfo {
    pub id: &'static str,
    pub label: &'static str,
    pub badge: &'static str,
    pub group: Group,
}

/// La liste complète, dans le même ordre que sur noTube.
pub const FORMATS: &[FormatInfo] = &[
    // ----- Vidéo -----
    FormatInfo { id: "mp4_sd", label: "MP4 SD", badge: "480p", group: Group::Video },
    FormatInfo { id: "mp4_hd", label: "MP4 HD", badge: "720p", group: Group::Video },
    FormatInfo { id: "mp4_fhd", label: "MP4 FHD", badge: "1080p", group: Group::Video },
    FormatInfo { id: "mp4_2k", label: "MP4 2K", badge: "1440p", group: Group::Video },
    FormatInfo { id: "3gp", label: "3GP", badge: "", group: Group::Video },
    FormatInfo { id: "flv", label: "FLV", badge: "", group: Group::Video },
    FormatInfo { id: "custom", label: "Codecs custom", badge: "", group: Group::Video },
    // ----- Audio -----
    FormatInfo { id: "mp3_lq", label: "MP3 LQ", badge: "128 kbps", group: Group::Audio },
    FormatInfo { id: "mp3_mq", label: "MP3 MQ", badge: "192 kbps", group: Group::Audio },
    FormatInfo { id: "mp3_hq", label: "MP3 HQ", badge: "320 kbps", group: Group::Audio },
    FormatInfo { id: "m4a", label: "M4A", badge: "", group: Group::Audio },
    FormatInfo { id: "ogg", label: "OGG", badge: "", group: Group::Audio },
    FormatInfo { id: "flac", label: "FLAC", badge: "", group: Group::Audio },
    FormatInfo { id: "wav", label: "WAV", badge: "", group: Group::Audio },
];

/// Cherche un format par son identifiant.
pub fn find(id: &str) -> Option<&'static FormatInfo> {
    FORMATS.iter().find(|f| f.id == id)
}

/// Les choix faits dans la fenêtre "Codecs custom".
/// (Un "codec", c'est la méthode utilisée pour compresser l'image ou le son.)
#[derive(Deserialize, Clone, Debug)]
pub struct CustomCodecs {
    pub container: String, // "mp4", "mkv" ou "webm"
    pub video: String,     // "h264", "h265", "vp9" ou "av1"
    pub audio: String,     // "aac", "mp3" ou "opus"
    pub max_height: u32,   // 0 = la meilleure qualité, sinon 480, 720, 1080...
}

/// Une conversion à faire avec ffmpeg APRÈS le téléchargement.
#[derive(Clone, Debug)]
pub struct Conversion {
    /// Extension du fichier final (sans le point), ex : "3gp".
    pub extension: &'static str,
    /// Réglages ffmpeg (placés entre le fichier d'entrée et celui de sortie).
    pub ffmpeg_args: Vec<String>,
}

/// La "recette" complète pour un téléchargement.
#[derive(Clone, Debug)]
pub struct Plan {
    /// Réglages pour yt-dlp (le format, la qualité...).
    pub ytdlp_args: Vec<String>,
    /// Conversion éventuelle avec ffmpeg.
    pub convert: Option<Conversion>,
    /// Petit texte affiché dans la liste, ex : "mp4 fhd".
    pub label: String,
}

/// Transforme une liste de &str en liste de String (plus pratique pour construire une commande).
fn to_strings(list: &[&str]) -> Vec<String> {
    list.iter().map(|s| s.to_string()).collect()
}

/// Réglages communs à tous les formats VIDÉO.
/// `with_extras` = on ajoute les sous-titres et les autres pistes audio
/// (seulement pour les 4 formats MP4, car les autres sont convertis ensuite).
fn video_args(max_height: u32, with_extras: bool, s: &Settings, merge_to: &str) -> Vec<String> {
    let mut a: Vec<String> = Vec::new();

    // -S = "sort" : comment choisir parmi les qualités proposées par le site.
    //   res:1080  -> la meilleure résolution qui ne dépasse pas 1080p
    //   vcodec:h264 -> on préfère le codec h264 (lisible partout)
    //   acodec:m4a  -> et le son m4a (AAC), lisible partout aussi
    let mut tri = String::new();
    if max_height > 0 {
        tri.push_str(&format!("res:{max_height},"));
    }
    tri.push_str("vcodec:h264,acodec:m4a");
    a.push("-S".to_string());
    a.push(tri);

    // -f = "format" : "bv*+ba/b" veut dire "meilleure vidéo + meilleur audio,
    // et si ce n'est pas possible, le meilleur fichier tout-en-un".
    a.push("-f".to_string());
    if with_extras && s.extra_audio {
        // Recette officielle de yt-dlp pour garder TOUTES les pistes audio.
        a.push("bv*+mergeall[vcodec=none]/bv*+ba/b".to_string());
        a.push("--audio-multistreams".to_string());
    } else {
        a.push("bv*+ba/b".to_string());
    }

    // La vidéo et le son sont téléchargés séparément, puis collés ensemble
    // dans un seul fichier (c'est le "merge").
    a.push("--merge-output-format".to_string());
    a.push(merge_to.to_string());

    // Sous-titres intégrés dans la vidéo.
    if with_extras && s.subtitles && !s.subtitle_langs.is_empty() {
        a.extend(to_strings(&["--write-subs", "--write-auto-subs", "--embed-subs"]));
        a.push("--sub-langs".to_string());
        a.push(s.subtitle_langs.join(","));
    }
    a
}

/// Recette pour un format AUDIO.
fn audio_plan(codec: &str, quality: Option<&str>, label: &str, prefer_m4a: bool) -> Plan {
    let mut a: Vec<String> = Vec::new();
    a.push("-f".to_string());
    a.push(if prefer_m4a { "ba[ext=m4a]/ba/b" } else { "ba/b" }.to_string());
    // -x = "extract audio" : on garde seulement le son.
    a.push("-x".to_string());
    a.push("--audio-format".to_string());
    a.push(codec.to_string());
    if let Some(q) = quality {
        a.push("--audio-quality".to_string());
        a.push(q.to_string());
    }
    Plan { ytdlp_args: a, convert: None, label: label.to_string() }
}

/// Recette pour les 4 formats MP4 (SD, HD, FHD, 2K).
fn mp4_plan(max_height: u32, label: &str, s: &Settings) -> Plan {
    Plan { ytdlp_args: video_args(max_height, true, s, "mp4"), convert: None, label: label.to_string() }
}

/// Réglages ffmpeg pour l'encodeur vidéo choisi.
fn video_encoder(codec: &str, container: &str) -> Option<Vec<String>> {
    let args = match codec {
        "h264" => vec!["-c:v", "libx264", "-preset", "medium", "-crf", "20", "-pix_fmt", "yuv420p"],
        "h265" => {
            let mut v = vec!["-c:v", "libx265", "-preset", "medium", "-crf", "24", "-pix_fmt", "yuv420p"];
            // Cette étiquette aide les appareils Apple à lire le fichier MP4.
            if container == "mp4" {
                v.extend(["-tag:v", "hvc1"]);
            }
            v
        }
        "vp9" => vec!["-c:v", "libvpx-vp9", "-crf", "32", "-b:v", "0", "-row-mt", "1"],
        "av1" => vec!["-c:v", "libsvtav1", "-crf", "32", "-preset", "8"],
        _ => return None,
    };
    Some(to_strings(&args))
}

/// Réglages ffmpeg pour l'encodeur audio choisi.
fn audio_encoder(codec: &str) -> Option<Vec<String>> {
    let args = match codec {
        "aac" => vec!["-c:a", "aac", "-b:a", "192k"],
        "mp3" => vec!["-c:a", "libmp3lame", "-b:a", "192k"],
        "opus" => vec!["-c:a", "libopus", "-b:a", "160k"],
        _ => return None,
    };
    Some(to_strings(&args))
}

/// Recette du format "Codecs custom".
fn custom_plan(custom: Option<&CustomCodecs>, s: &Settings) -> Result<Plan, String> {
    let c = custom.ok_or("bad_codecs")?;

    // Quels codecs sont acceptés dans quel conteneur ?
    // (Ces règles sont aussi dans le fichier js/app.js, gardez-les identiques.)
    let compatible = match c.container.as_str() {
        "mp4" => matches!(c.video.as_str(), "h264" | "h265" | "av1") && matches!(c.audio.as_str(), "aac" | "mp3"),
        "mkv" => true,
        "webm" => matches!(c.video.as_str(), "vp9" | "av1") && c.audio == "opus",
        _ => false,
    };
    if !compatible || ![0, 480, 720, 1080, 1440, 2160].contains(&c.max_height) {
        return Err("bad_codecs".to_string());
    }

    let extension: &'static str = match c.container.as_str() {
        "mp4" => "mp4",
        "webm" => "webm",
        _ => "mkv",
    };
    let mut ff = video_encoder(&c.video, &c.container).ok_or("bad_codecs")?;
    ff.extend(audio_encoder(&c.audio).ok_or("bad_codecs")?);
    if extension == "mp4" {
        // Permet de commencer la lecture avant d'avoir tout chargé.
        ff.extend(to_strings(&["-movflags", "+faststart"]));
    }

    Ok(Plan {
        // On télécharge dans un conteneur MKV (il accepte tout), puis ffmpeg convertit.
        ytdlp_args: video_args(c.max_height, false, s, "mkv"),
        convert: Some(Conversion { extension, ffmpeg_args: ff }),
        label: format!("{} {} {}", c.container, c.video, c.audio),
    })
}

/// LA fonction principale : identifiant de format -> recette complète.
pub fn build_plan(format_id: &str, custom: Option<&CustomCodecs>, s: &Settings) -> Result<Plan, String> {
    match format_id {
        // ----- Vidéo -----
        "mp4_sd" => Ok(mp4_plan(480, "mp4 sd", s)),
        "mp4_hd" => Ok(mp4_plan(720, "mp4 hd", s)),
        "mp4_fhd" => Ok(mp4_plan(1080, "mp4 fhd", s)),
        "mp4_2k" => Ok(mp4_plan(1440, "mp4 2k", s)),
        // 3GP : vieux format de téléphone. On télécharge petit, puis ffmpeg convertit.
        "3gp" => Ok(Plan {
            ytdlp_args: video_args(360, false, s, "mp4"),
            convert: Some(Conversion {
                extension: "3gp",
                ffmpeg_args: to_strings(&[
                    "-c:v", "libx264", "-profile:v", "baseline", "-level", "3.0", "-pix_fmt", "yuv420p",
                    "-vf", "scale=320:-2", "-c:a", "aac", "-b:a", "64k", "-ar", "44100", "-ac", "1",
                ]),
            }),
            label: "3gp".to_string(),
        }),
        // FLV : vieux format de Flash.
        "flv" => Ok(Plan {
            ytdlp_args: video_args(480, false, s, "mp4"),
            convert: Some(Conversion {
                extension: "flv",
                ffmpeg_args: to_strings(&[
                    "-c:v", "libx264", "-preset", "medium", "-crf", "23", "-pix_fmt", "yuv420p",
                    "-c:a", "aac", "-b:a", "128k", "-ar", "44100",
                ]),
            }),
            label: "flv".to_string(),
        }),
        "custom" => custom_plan(custom, s),
        // ----- Audio -----
        "mp3_lq" => Ok(audio_plan("mp3", Some("128K"), "mp3 lq", false)),
        "mp3_mq" => Ok(audio_plan("mp3", Some("192K"), "mp3 mq", false)),
        "mp3_hq" => Ok(audio_plan("mp3", Some("320K"), "mp3 hq", false)),
        "m4a" => Ok(audio_plan("m4a", None, "m4a", true)),
        "ogg" => Ok(audio_plan("vorbis", None, "ogg", false)),
        "flac" => Ok(audio_plan("flac", None, "flac", false)),
        "wav" => Ok(audio_plan("wav", None, "wav", false)),
        // Tout le reste est refusé.
        _ => Err("bad_format".to_string()),
    }
}

// =============================================================================
//  TESTS : de petits contrôles automatiques (lancer avec : cargo test)
//  Si on casse quelque chose par erreur, un test devient rouge et nous prévient.
// =============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    fn custom(container: &str, video: &str, audio: &str, max_height: u32) -> CustomCodecs {
        CustomCodecs { container: container.into(), video: video.into(), audio: audio.into(), max_height }
    }

    fn has(plan: &Plan, wanted: &str) -> bool {
        plan.ytdlp_args.iter().any(|a| a == wanted)
    }

    #[test]
    fn every_listed_format_has_a_plan() {
        for f in FORMATS {
            let c = if f.id == "custom" { Some(custom("mp4", "h264", "aac", 1080)) } else { None };
            assert!(build_plan(f.id, c.as_ref(), &Settings::default()).is_ok(), "format {}", f.id);
        }
    }

    #[test]
    fn the_14_formats_of_notube_are_all_there() {
        assert_eq!(FORMATS.len(), 14);
        assert_eq!(FORMATS.iter().filter(|f| matches!(f.group, Group::Video)).count(), 7);
        assert_eq!(FORMATS.iter().filter(|f| matches!(f.group, Group::Audio)).count(), 7);
    }

    #[test]
    fn unknown_format_is_refused() {
        assert_eq!(build_plan("exe", None, &Settings::default()).unwrap_err(), "bad_format");
        assert_eq!(build_plan("", None, &Settings::default()).unwrap_err(), "bad_format");
    }

    #[test]
    fn mp4_fhd_is_limited_to_1080p_and_merged_in_mp4() {
        let p = build_plan("mp4_fhd", None, &Settings::default()).unwrap();
        assert!(has(&p, "res:1080,vcodec:h264,acodec:m4a"));
        assert!(has(&p, "mp4"));
        assert!(p.convert.is_none());
    }

    #[test]
    fn mp3_hq_is_320k() {
        let p = build_plan("mp3_hq", None, &Settings::default()).unwrap();
        assert!(has(&p, "320K") && has(&p, "mp3") && has(&p, "-x"));
    }

    #[test]
    fn subtitles_are_added_only_when_enabled() {
        let mut s = Settings::default();
        assert!(!has(&build_plan("mp4_hd", None, &s).unwrap(), "--embed-subs"));
        s.subtitles = true;
        s.subtitle_langs = vec!["fr".into(), "en".into()];
        let p = build_plan("mp4_hd", None, &s).unwrap();
        assert!(has(&p, "--embed-subs") && has(&p, "fr,en"));
        // Pas de sous-titres pour l'audio.
        assert!(!has(&build_plan("mp3_lq", None, &s).unwrap(), "--embed-subs"));
    }

    #[test]
    fn extra_audio_tracks_option_works() {
        let s = Settings { extra_audio: true, ..Settings::default() };
        assert!(has(&build_plan("mp4_fhd", None, &s).unwrap(), "--audio-multistreams"));
    }

    #[test]
    fn gp3_and_flv_are_converted_with_ffmpeg() {
        assert_eq!(build_plan("3gp", None, &Settings::default()).unwrap().convert.unwrap().extension, "3gp");
        assert_eq!(build_plan("flv", None, &Settings::default()).unwrap().convert.unwrap().extension, "flv");
    }

    #[test]
    fn custom_codecs_valid_combinations() {
        let s = Settings::default();
        for (c, v, a) in [("mp4", "h265", "aac"), ("mkv", "vp9", "opus"), ("webm", "av1", "opus"), ("mp4", "av1", "mp3")] {
            let p = build_plan("custom", Some(&custom(c, v, a, 0)), &s).unwrap();
            assert_eq!(p.convert.unwrap().extension, c);
        }
    }

    #[test]
    fn custom_codecs_bad_combinations_are_refused() {
        let s = Settings::default();
        for (c, v, a, h) in [
            ("webm", "h264", "opus", 720),
            ("mp4", "h264", "opus", 720),
            ("avi", "h264", "aac", 720),
            ("mp4", "h264; calc", "aac", 720),
            ("mp4", "h264", "aac", 999),
        ] {
            assert_eq!(build_plan("custom", Some(&custom(c, v, a, h)), &s).unwrap_err(), "bad_codecs");
        }
        assert_eq!(build_plan("custom", None, &s).unwrap_err(), "bad_codecs");
    }
}
