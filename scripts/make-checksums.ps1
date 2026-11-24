# =============================================================================
#  make-checksums.ps1 : fabrique le fichier d'empreintes SHA-256 des installateurs
# =============================================================================
#  Une "empreinte" (SHA-256) est comme la signature unique d'un fichier : si
#  quelqu'un modifie le fichier, même d'une seule lettre, l'empreinte change.
#  En la publiant à côté de l'installateur, tu permets à chacun de vérifier que
#  le fichier téléchargé est EXACTEMENT celui que tu as fabriqué.
#
#  À lancer à la racine du projet, après "cargo tauri build" :
#     powershell -ExecutionPolicy Bypass -File scripts\make-checksums.ps1
#  Il crée  SHA256SUMS.txt  à côté de l'installateur.
# =============================================================================
$ErrorActionPreference = "Stop"
$folder = Join-Path $PSScriptRoot "..\src-tauri\target\release\bundle\nsis"
$installers = Get-ChildItem -Path $folder -Filter "ALLTube_*_x64-setup.exe"
if (-not $installers) { throw "Aucun installateur trouve dans $folder. Lance d'abord : cargo tauri build --bundles nsis" }

$lines = foreach ($f in $installers) {
    $hash = (Get-FileHash $f.FullName -Algorithm SHA256).Hash.ToLower()
    "$hash  $($f.Name)"
}
$out = Join-Path $folder "SHA256SUMS.txt"
# Encodage UTF-8 sans BOM, fin de ligne LF : format standard lisible par sha256sum.
[System.IO.File]::WriteAllText($out, (($lines -join "`n") + "`n"), (New-Object System.Text.UTF8Encoding($false)))
Write-Host ""
Write-Host "Fichier cree : $out"
Get-Content $out
