# Changelog

All notable changes to ALLTube are written here. The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and the project uses [Semantic Versioning](https://semver.org/).
Tous les changements importants d'ALLTube sont notés ici.

## [1.0.0] - 2026-11-23

First public version. / Première version publique.

### Added / Ajouté
- Desktop app for **Windows 11** (Rust + Tauri 2), free, no account, **no daily download limit**.
- Download videos and music from the sites supported by [yt-dlp](https://github.com/yt-dlp/yt-dlp), from a link or keywords.
- 14 formats: MP4 SD / HD / FHD / 2K, 3GP, FLV, custom codecs (MP4, MKV, WebM with H.264, H.265, VP9, AV1 + AAC, MP3, Opus), MP3 128 / 192 / 320 kbps, M4A, OGG, FLAC, WAV.
- Cut a video or an audio file (fast copy mode or precise mode).
- Settings: download folder, subtitles (with languages), other audio tracks, completion notification, simultaneous downloads, delay between downloads, language, theme (dark / light), browser cookies.
- Interface in **French** (default), **English** and **Chinese (Mandarin)**.
- Recent downloads list with progress, open file, open folder, cut, remove, clear all.
- Security by design (no shell, validated inputs, strict CSP, minimal permissions). See [SECURITY.md](SECURITY.md).
- Automatic tests (20), CI, CodeQL, Dependabot, dependency audit.

### Known issues / Problèmes connus
- A **red error** can appear even though the file was saved correctly in `Downloads/ALLTube`. The cause is still being investigated. If you see it, check the folder: the file is usually there. Please copy the full red message into a bug report.
  *Une **erreur rouge** peut s'afficher alors que le fichier est bien enregistré dans `Téléchargements/ALLTube`. La cause est en cours d'analyse. Si tu la vois, regarde dans le dossier : le fichier y est en général. Copie le message rouge complet dans un ticket.*
- The installer is not code-signed yet: Windows SmartScreen may warn you.
- macOS and Linux installers are not published yet (see [ROADMAP.md](ROADMAP.md)).
- The cutting tool has no video preview.

[1.0.0]: https://github.com/madjeek-web/alltube/releases/tag/v1.0.0
