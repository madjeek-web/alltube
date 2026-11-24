#!/usr/bin/env bash
# =============================================================================
#  fetch-sidecars.sh : récupère les "moteurs" d'ALLTube pour LINUX (64 bits)
# =============================================================================
#  Même rôle que fetch-sidecars.ps1 (Windows) :
#   1. télécharge yt-dlp depuis le dépôt officiel,
#   2. télécharge ffmpeg et ffprobe (builds BtbN, licence GPL),
#   3. VÉRIFIE l'empreinte SHA-256 de chaque téléchargement avec la liste
#      officielle publiée à côté (si ça ne correspond pas : on s'arrête),
#   4. range les fichiers dans src-tauri/binaries avec le nom attendu par Tauri.
#
#  Utilisation :   bash scripts/fetch-sidecars.sh
#  (pour tester sans toucher au projet : DEST=/tmp/test bash scripts/fetch-sidecars.sh)
#
#  macOS n'est pas encore pris en charge (voir ROADMAP.md).
# =============================================================================
set -euo pipefail

# Le "triple" est le nom officiel de l'ordinateur visé (ici Linux 64 bits).
TRIPLE="x86_64-unknown-linux-gnu"
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DEST="${DEST:-$ROOT/src-tauri/binaries}"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT          # on nettoie les fichiers temporaires dans tous les cas
mkdir -p "$DEST"

# Vérifie qu'un fichier correspond à la ligne "empreinte  nom" d'un fichier de sommes.
verify() { # $1 = dossier, $2 = fichier de sommes, $3 = nom du fichier à vérifier
  local expected
  expected="$(grep -E "^[0-9a-fA-F]{64}[[:space:]]+\*?$3\$" "$1/$2" | head -n1 | awk '{print $1}')"
  [ -n "$expected" ] || { echo "ERREUR : empreinte introuvable pour $3" >&2; exit 1; }
  echo "$expected  $1/$3" | sha256sum --check --status \
    || { echo "ERREUR : l'empreinte de $3 est différente de celle publiée. Fichier refusé." >&2; exit 1; }
  echo "  OK : empreinte vérifiée pour $3"
}

echo "1/2 yt-dlp"
YT="https://github.com/yt-dlp/yt-dlp/releases/latest/download"
curl -fsSL "$YT/yt-dlp_linux"  -o "$TMP/yt-dlp_linux"
curl -fsSL "$YT/SHA2-256SUMS"  -o "$TMP/SHA2-256SUMS"
verify "$TMP" SHA2-256SUMS yt-dlp_linux
install -m 0755 "$TMP/yt-dlp_linux" "$DEST/yt-dlp-$TRIPLE"

echo "2/2 ffmpeg et ffprobe"
FF_NAME="ffmpeg-master-latest-linux64-gpl.tar.xz"
FF="https://github.com/BtbN/FFmpeg-Builds/releases/latest/download"
curl -fsSL "$FF/$FF_NAME"          -o "$TMP/$FF_NAME"
curl -fsSL "$FF/checksums.sha256"  -o "$TMP/checksums.sha256"
verify "$TMP" checksums.sha256 "$FF_NAME"
tar -xJf "$TMP/$FF_NAME" -C "$TMP"
install -m 0755 "$TMP"/ffmpeg-master-latest-linux64-gpl/bin/ffmpeg  "$DEST/ffmpeg-$TRIPLE"
install -m 0755 "$TMP"/ffmpeg-master-latest-linux64-gpl/bin/ffprobe "$DEST/ffprobe-$TRIPLE"

echo
echo "Terminé. Fichiers prêts dans : $DEST"
