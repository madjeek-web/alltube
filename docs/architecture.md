# Architecture : comment ALLTube fonctionne / How ALLTube works

**Français d'abord, English below.** Ce document est fait pour être lu comme un cours, même si tu n'as jamais programmé.

---

## Français

### 1. L'image d'ensemble : un restaurant

| Dans le restaurant | Dans ALLTube | Où ? |
|---|---|---|
| La salle et le menu (ce que voit le client) | **Le visage** : la fenêtre, les boutons, la liste | `src/` (HTML, CSS, JavaScript) |
| La cuisine (là où tout se décide) | **Le cerveau** : file d'attente, réglages, sécurité | `src-tauri/src/` (Rust) |
| Les fournisseurs spécialisés | **Les moteurs** : yt-dlp (récupère) et ffmpeg (convertit) | `src-tauri/binaries/` |
| Les tickets entre la salle et la cuisine | **Les commandes** (`invoke`) et les **événements** | `main.rs` et `app.js` |

Le client (la page web) ne cuisine jamais lui-même. Il passe une commande, et la cuisine (Rust) **vérifie tout** avant d'agir.

### 2. Le voyage d'un téléchargement

1. **Tu colles un lien et tu cliques sur OK.** `app.js` appelle `invoke("start_download", { input, formatId, custom })`.
2. **`input.rs`** vérifie le texte : lien `http(s)` valide, ou mots-clés (transformés en recherche `ytsearch1:...`). Tout le reste est refusé.
3. **`formats.rs`** transforme l'identifiant du format (`mp4_fhd`) en « recette » : quels réglages donner à yt-dlp, et faut-il convertir ensuite avec ffmpeg.
4. **`downloads.rs`** crée une ligne « En attente » et appelle `pump()`, le chef d'orchestre. Il regarde combien de téléchargements tournent déjà (réglage « simultanés ») et le délai éventuel, puis démarre le suivant.
5. **`engine.rs`** lance yt-dlp **sans shell**, avec une liste d'arguments. Le lien est placé après `--`.
6. yt-dlp écrit des lignes que nous savons lire : `ALLTUBE_META` (titre, auteur, durée, miniature), `ALLTUBE_PROGRESS` (octets reçus) et `ALLTUBE_FILE` (chemin du fichier final).
7. À chaque nouvelle information, Rust envoie l'événement `download-update`. La page met la carte à jour (barre de progression, titre, miniature...).
8. Si besoin, **`media.rs`** lance ffmpeg pour convertir (3GP, FLV, codecs personnalisés) avec une barre de progression.
9. La ligne passe à « Terminé » (ou « Erreur »), une notification s'affiche si elle est activée, et l'historique est sauvegardé dans `history.json`.

### 3. Les fichiers, un par un

| Fichier | Rôle en une phrase |
|---|---|
| `main.rs` | Démarre le programme et liste les commandes que la page a le droit d'appeler |
| `settings.rs` | La fiche de réglages, **revérifiée** à chaque fois (`sanitize`) et sauvegardée sans risque de corruption |
| `formats.rs` | Les 14 formats et leur recette ; rejette tout identifiant inconnu |
| `input.rs` | Première barrière de sécurité : valide ce que l'utilisateur tape |
| `engine.rs` | Trouve et lance yt-dlp / ffmpeg / ffprobe en sécurité, et sait les arrêter (même leurs « enfants » sous Windows) |
| `downloads.rs` | La file d'attente, la progression, l'historique, annuler / retirer / tout effacer |
| `media.rs` | ffmpeg : conversion, découpe (les ciseaux), durée d'un fichier |

Côté page : `index.html` (le squelette), `css/style.css` (la décoration, thèmes sombre et clair), `js/i18n.js` (tous les textes en 3 langues), `js/app.js` (le comportement).

### 4. Le modèle de sécurité

Une règle d'or : **la page web n'est pas digne de confiance**, car elle affiche des titres venus d'Internet. Donc :

- Le Rust **revalide tout** ce que la page envoie (réglages, formats, chemins).
- La page n'envoie que des **identifiants** (`mp4_fhd`) et des **numéros** (`id` de téléchargement), jamais une commande ou un chemin libre.
- **Aucun shell** : `Command::new(programme).args([...])`. Un lien comme `x; del *.*` reste du texte.
- Seules les extensions de médias connues peuvent être ouvertes ou découpées.
- La politique de sécurité (CSP) de `tauri.conf.json` interdit les scripts inline et les connexions réseau depuis la page. Les permissions Tauri (`capabilities/default.json`) sont réduites au minimum.
- Les titres sont écrits avec `textContent`, jamais `innerHTML`.

### 5. Recettes pour contribuer

**Ajouter une langue** : copie le bloc `en` de `src/js/i18n.js`, traduis, ajoute une `<option>` dans `src/index.html`, ajoute le code dans `UI_LANGUAGES` de `src-tauri/src/settings.rs`, puis lance `node scripts/check-i18n.js`.

**Ajouter un format** : ajoute une ligne dans `FORMATS` et un cas dans `build_plan` de `formats.rs`, ajoute un test, c'est tout (la page affiche la liste envoyée par Rust).

**Changer une icône** : remplace le fichier dans `src/assets/icons/` (mêmes nom et taille carrée). Pour les icônes de l'application : `cargo tauri icon`.

**Lancer les tests** : `cd src-tauri && cargo test`.

---

## English

### Big picture: a restaurant

- **The face** (`src/`, HTML/CSS/JavaScript) is the dining room: what the customer sees.
- **The brain** (`src-tauri/src/`, Rust) is the kitchen: queue, settings, security.
- **The engines** (yt-dlp, ffmpeg, ffprobe) are the specialised suppliers, stored in `src-tauri/binaries/`.
- The page talks to Rust with **commands** (`invoke`) and listens to **events**. The page never cooks: Rust re-checks everything.

### The journey of a download

1. You paste a link and click OK: `app.js` calls `start_download`.
2. `input.rs` validates the text (a valid `http(s)` link, or keywords turned into a search).
3. `formats.rs` turns the format id (`mp4_fhd`) into a recipe for yt-dlp (and a ffmpeg conversion when needed).
4. `downloads.rs` queues it and `pump()` starts it according to the "simultaneous downloads" and "delay" settings.
5. `engine.rs` starts yt-dlp **without a shell**, with the link after `--`.
6. yt-dlp prints `ALLTUBE_META`, `ALLTUBE_PROGRESS` and `ALLTUBE_FILE` lines that Rust parses.
7. Rust emits `download-update` events, the page updates the card.
8. `media.rs` runs ffmpeg for conversions (3GP, FLV, custom codecs).
9. The row becomes Done or Error, a notification is shown if enabled, and the history is saved.

### Security model

The web page is **not trusted** (it displays titles from the Internet). Rust re-validates everything; the page only sends identifiers and numbers; no shell; only media extensions can be opened; strict CSP and minimal Tauri permissions; titles are written with `textContent`.

### Recipes

- **Add a language**: copy the `en` block in `src/js/i18n.js`, translate, add an `<option>` in `src/index.html`, add the code to `UI_LANGUAGES` in `settings.rs`, run `node scripts/check-i18n.js`.
- **Add a format**: add a line to `FORMATS` and a case in `build_plan` in `formats.rs`, plus a test.
- **Change an icon**: replace the file in `src/assets/icons/`; for app icons use `cargo tauri icon`.
- **Run the tests**: `cd src-tauri && cargo test`.
