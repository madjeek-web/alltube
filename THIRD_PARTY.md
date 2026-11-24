# Logiciels tiers utilisés par ALLTube

ALLTube lance ces programmes comme des outils séparés (il ne les modifie pas) :

- **yt-dlp** : licence Unlicense (domaine public). https://github.com/yt-dlp/yt-dlp
- **FFmpeg** (ffmpeg et ffprobe) : builds "GPL" de BtbN, licence GPL v3. https://github.com/BtbN/FFmpeg-Builds
  Code source de FFmpeg : https://ffmpeg.org/download.html
- **Tauri** : licence MIT ou Apache 2.0. https://tauri.app

Ces outils sont téléchargés depuis leurs dépôts officiels par `scripts/fetch-sidecars.ps1`, qui vérifie leur empreinte SHA-256.
Quand ALLTube sera publié, il faudra garder ce fichier dans l'installateur et mentionner le lien vers le code source de FFmpeg.
