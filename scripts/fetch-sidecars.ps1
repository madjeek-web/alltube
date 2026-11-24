# =============================================================================
#  fetch-sidecars.ps1 : récupère les 3 "moteurs" d'ALLTube pour Windows 64 bits
# =============================================================================
#  À lancer UNE FOIS avant de compiler (et de temps en temps pour mettre à jour
#  yt-dlp, car les sites changent souvent) :
#
#     powershell -ExecutionPolicy Bypass -File scripts\fetch-sidecars.ps1
#
#  Ce que fait le script :
#   1. télécharge yt-dlp.exe depuis le dépôt OFFICIEL (github.com/yt-dlp),
#   2. télécharge ffmpeg + ffprobe depuis les builds BtbN (github.com/BtbN),
#   3. VÉRIFIE l'empreinte SHA-256 de chaque fichier (la "signature" du
#      fichier) avec la liste officielle publiée à côté. Si une seule lettre
#      est différente, le fichier a été modifié : le script s'arrête.
#   4. les range dans src-tauri\binaries avec le nom attendu par Tauri.
# =============================================================================

$ErrorActionPreference = "Stop"

$triple = "x86_64-pc-windows-msvc"                       # Windows 64 bits
$dest   = Join-Path $PSScriptRoot "..\src-tauri\binaries"
$tmp    = Join-Path ([System.IO.Path]::GetTempPath()) ("alltube-" + [guid]::NewGuid().ToString("N"))
New-Item -ItemType Directory -Force -Path $dest | Out-Null
New-Item -ItemType Directory -Force -Path $tmp  | Out-Null

# Vérifie qu'un fichier a bien l'empreinte attendue.
function Assert-Hash($file, $expected, $label) {
    $actual = (Get-FileHash $file -Algorithm SHA256).Hash.ToLower()
    if ($actual -ne $expected.ToLower()) {
        throw "ATTENTION : l'empreinte de $label est differente de celle publiee. Fichier refuse."
    }
    Write-Host "  OK : empreinte verifiee pour $label"
}

# Cherche, dans un fichier de sommes de contrôle, la ligne d'un fichier précis.
function Get-ExpectedHash($sumsFile, $name) {
    $pattern = "^\s*([0-9a-fA-F]{64})\s+\*?" + [regex]::Escape($name) + "\s*$"
    foreach ($line in Get-Content $sumsFile) {
        if ($line -match $pattern) { return $Matches[1] }
    }
    throw "Empreinte introuvable pour $name dans $sumsFile"
}

try {
    # ---------------------------------------------------------------- yt-dlp
    Write-Host "1/2 yt-dlp"
    $ytBase = "https://github.com/yt-dlp/yt-dlp/releases/latest/download"
    Invoke-WebRequest "$ytBase/yt-dlp.exe"    -OutFile "$tmp\yt-dlp.exe"
    Invoke-WebRequest "$ytBase/SHA2-256SUMS"  -OutFile "$tmp\SHA2-256SUMS"
    Assert-Hash "$tmp\yt-dlp.exe" (Get-ExpectedHash "$tmp\SHA2-256SUMS" "yt-dlp.exe") "yt-dlp.exe"
    Copy-Item "$tmp\yt-dlp.exe" (Join-Path $dest "yt-dlp-$triple.exe") -Force

    # ---------------------------------------------------------------- ffmpeg
    Write-Host "2/2 ffmpeg et ffprobe"
    $ffName = "ffmpeg-master-latest-win64-gpl.zip"
    $ffBase = "https://github.com/BtbN/FFmpeg-Builds/releases/latest/download"
    Invoke-WebRequest "$ffBase/$ffName"            -OutFile "$tmp\$ffName"
    Invoke-WebRequest "$ffBase/checksums.sha256"   -OutFile "$tmp\checksums.sha256"
    Assert-Hash "$tmp\$ffName" (Get-ExpectedHash "$tmp\checksums.sha256" $ffName) $ffName

    Expand-Archive -Path "$tmp\$ffName" -DestinationPath "$tmp\ffmpeg" -Force
    $ffmpegExe  = Get-ChildItem -Path "$tmp\ffmpeg" -Recurse -Filter "ffmpeg.exe"  | Select-Object -First 1
    $ffprobeExe = Get-ChildItem -Path "$tmp\ffmpeg" -Recurse -Filter "ffprobe.exe" | Select-Object -First 1
    if (-not $ffmpegExe -or -not $ffprobeExe) { throw "ffmpeg.exe ou ffprobe.exe introuvable dans l'archive" }
    Copy-Item $ffmpegExe.FullName  (Join-Path $dest "ffmpeg-$triple.exe")  -Force
    Copy-Item $ffprobeExe.FullName (Join-Path $dest "ffprobe-$triple.exe") -Force

    Write-Host ""
    Write-Host "Termine. Fichiers prets dans : $dest"
}
finally {
    # On nettoie les fichiers temporaires dans tous les cas.
    Remove-Item -Recurse -Force $tmp -ErrorAction SilentlyContinue
}
