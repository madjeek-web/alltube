# Security Policy / Politique de sécurité

**English first, français en dessous.**

---

## English

### Supported versions

| Version | Supported |
|---|---|
| 1.0.x | Yes |
| anything older | No |

### How to report a vulnerability

**Please do not open a public issue for a security problem.**

1. Go to the **Security** tab of this repository.
2. Click **Report a vulnerability** (or open <https://github.com/madjeek-web/alltube/security/advisories/new>).
3. Describe what you found, how to reproduce it, and what could go wrong.

This is a volunteer project. You will get an answer as soon as possible, usually within a week. Once the problem is fixed, a new version is published and you are credited (if you wish) in the release notes.

### What counts as a security problem

- running a command or opening a file the user did not ask for (command injection, path traversal),
- a way to make the interface run untrusted code (for example through a video title),
- a way around the Content Security Policy or the minimal Tauri permissions,
- a bug that writes outside the download folder,
- anything that leaks cookies, links or files off the user's computer.

### Out of scope

- Vulnerabilities inside **yt-dlp**, **FFmpeg** or **Tauri** themselves: please report them to those projects (we update our copy when they publish a fix).
- Websites changing and breaking downloads (that is a normal bug, open a regular issue).
- An antivirus flagging the installer by mistake ("false positive"): open a regular issue and also report it to the antivirus vendor.
- Attacks that need full access to the user's computer.

### How ALLTube protects you (by design)

- **No shell.** The download engines are started directly with a list of arguments, so a malicious link cannot run a command.
- Links are validated (`http` and `https` only) and always passed after `--`, so they can never be read as an option.
- The web page only sends **identifiers** (like `mp4_fhd`); the Rust code builds the real commands.
- Every setting received from the page is checked again by the Rust code.
- The page cannot open an arbitrary path: it sends a download number, and the Rust code finds the file.
- Only known media extensions can be opened or cut (never `.exe`, `.bat`...).
- The engines are only looked up **next to the program**, never in the system `PATH`.
- Strict Content Security Policy, no inline script, minimal Tauri permissions.
- Titles coming from the Internet are shown as plain text, never as HTML.
- **No account, no telemetry, no server.** ALLTube sends nothing anywhere.

### Verify your download

Each release includes a `SHA256SUMS.txt` file. On Windows, open PowerShell in your Downloads folder and run:

```powershell
Get-FileHash .\ALLTube_1.0.0_x64-setup.exe -Algorithm SHA256
```

The result must be identical to the line in `SHA256SUMS.txt`. If it is not, **do not install the file**.

### Antivirus and Windows SmartScreen

Version 1.0.0 is not code-signed yet, so Windows SmartScreen may show "Windows protected your PC". Click **More info**, then **Run anyway**, but only if the file came from this repository's *Releases* page and its checksum matches. Code signing is on the [roadmap](ROADMAP.md).

---

## Français

### Versions prises en charge

| Version | Prise en charge |
|---|---|
| 1.0.x | Oui |
| plus ancienne | Non |

### Comment signaler une faille

**Merci de ne pas ouvrir de ticket public pour un problème de sécurité.**

1. Va dans l'onglet **Security** de ce dépôt.
2. Clique sur **Report a vulnerability** (ou ouvre <https://github.com/madjeek-web/alltube/security/advisories/new>).
3. Explique ce que tu as trouvé, comment le reproduire et ce qui pourrait mal tourner.

C'est un projet bénévole : tu auras une réponse dès que possible, en général sous une semaine. Quand le problème est corrigé, une nouvelle version sort et tu es cité(e) dans les notes de version, si tu le souhaites.

### Ce qui compte comme faille

- lancer une commande ou ouvrir un fichier que l'utilisateur n'a pas demandé,
- faire exécuter du code non fiable par l'interface (par exemple via un titre de vidéo),
- contourner la politique de sécurité (CSP) ou les permissions minimales de Tauri,
- écrire en dehors du dossier de téléchargement,
- faire sortir des cookies, des liens ou des fichiers de l'ordinateur.

### Hors sujet

- Les failles de **yt-dlp**, **FFmpeg** ou **Tauri** eux-mêmes : à signaler à ces projets (nous mettons notre copie à jour dès qu'ils publient un correctif).
- Un site qui change et casse les téléchargements : c'est un bug normal, ouvre un ticket classique.
- Un antivirus qui bloque l'installateur par erreur (« faux positif ») : ouvre un ticket classique et signale-le aussi à l'éditeur de l'antivirus.
- Les attaques qui demandent un accès complet à l'ordinateur de l'utilisateur.

### Vérifier ton téléchargement

Chaque version contient un fichier `SHA256SUMS.txt`. Sous Windows, ouvre PowerShell dans ton dossier Téléchargements et tape :

```powershell
Get-FileHash .\ALLTube_1.0.0_x64-setup.exe -Algorithm SHA256
```

Le résultat doit être identique à la ligne de `SHA256SUMS.txt`. Sinon, **n'installe pas le fichier**.

### Antivirus et SmartScreen de Windows

La version 1.0.0 n'est pas encore signée numériquement : Windows SmartScreen peut afficher « Windows a protégé votre ordinateur ». Clique sur **Informations complémentaires**, puis **Exécuter quand même**, mais seulement si le fichier vient de la page *Releases* de ce dépôt et que son empreinte correspond. La signature du code est prévue dans la [feuille de route](ROADMAP.md).
