# Contributing to ALLTube / Contribuer à ALLTube

**English first, français en dessous.** Everyone is welcome, beginners included.

---

## English

### What is a "fork"? (the cookbook story)

Imagine ALLTube is a cookbook placed on a public table.

- **Forking** means photocopying the whole cookbook and taking the copy home. The original stays untouched and public.
- On your copy, you can change anything: add a recipe, fix a typo, try crazy ideas. Nobody is affected.
- If you made something great, you can **propose it** to the original cookbook. That proposal is called a **pull request**. The maintainer reads it and decides whether to add it.

So: **fork = your own copy**, **pull request = "please add my improvement to the original"**.

### Versions: how nothing ever gets overwritten

- Every official version is a **release** with a **tag** (`v1.0.0`, `v1.1.0`, `v2.0.0`...). A release is frozen: it stays online and downloadable forever.
- The `main` branch is where the next version is prepared. Contributions go there through pull requests.
- Version 1.0.0 will never be replaced by version 2.0.0. They sit side by side on the *Releases* page.

### Your first contribution (from the website, no command line)

1. Click **Fork** (top right of the repository page), then **Create fork**.
2. In **your** copy, open the file you want to improve and click the pencil icon (**Edit this file**).
3. Make your change and click **Commit changes**. Choose **Create a new branch for this commit**, give it a short name (for example `fix-typo-french`) and confirm.
4. GitHub shows a green **Compare & pull request** button. Click it.
5. Fill in the short form (what and why) and click **Create pull request**.
6. The automatic checks run (a green tick is good). The maintainer will review and answer.

### Working on your computer

```bash
git clone https://github.com/<your-name>/alltube.git
cd alltube
# Windows only : download the engines (yt-dlp, ffmpeg)
powershell -ExecutionPolicy Bypass -File scripts\fetch-sidecars.ps1
# Linux only :
bash scripts/fetch-sidecars.sh
cargo install tauri-cli --version "^2" --locked
cargo tauri dev
```

Before you open a pull request, run the same checks as the robot:

```bash
node scripts/check-i18n.js
cd src-tauri
cargo clippy --all-targets -- -D warnings
cargo test
```

### Rules of the project

1. **Comment your code in simple words.** This project is also a school: a 14 year old with no programming experience should be able to follow. French or English comments are both welcome.
2. **Security first.** Never start a program through a shell (`cmd`, `sh`...). Never trust what comes from the web page or the Internet: validate it in the Rust code. Never use `innerHTML` with data coming from outside.
3. **No secrets**: never commit passwords, tokens, cookies or personal files.
4. **Add a test** when you add logic in `formats.rs`, `settings.rs` or `input.rs`.
5. **New text in the interface** must exist in the 3 languages (`fr`, `en`, `zh`) in `src/js/i18n.js`. `node scripts/check-i18n.js` checks it.
6. One pull request = one idea. Small is beautiful.
7. By contributing, you agree that your work is published under the project's [MIT license](LICENSE).

### Recognition

GitHub lists every contributor automatically in the **Contributors** section on the right of the repository page. Significant contributions are also thanked in the release notes and in [CONTRIBUTORS.md](CONTRIBUTORS.md).

### Please be kind

Read the [Code of Conduct](CODE_OF_CONDUCT.md). For security problems, see [SECURITY.md](SECURITY.md).

---

## Français

### C'est quoi « forker » ? (l'histoire du cahier de recettes)

Imagine qu'ALLTube est un cahier de recettes posé sur une table publique.

- **Forker**, c'est photocopier tout le cahier et emporter la copie chez toi. L'original ne bouge pas, il reste public.
- Sur ta copie, tu peux tout changer : ajouter une recette, corriger une faute, tester des idées folles. Personne n'est gêné.
- Si tu as fait quelque chose de super, tu peux le **proposer** au cahier original. Cette proposition s'appelle une **pull request**. Le responsable la lit et décide s'il l'ajoute.

Donc : **fork = ta propre copie**, **pull request = « s'il te plaît, ajoute mon amélioration à l'original »**.

### Les versions : comment rien n'est jamais écrasé

- Chaque version officielle est une **release** avec une **étiquette** (`v1.0.0`, `v1.1.0`, `v2.0.0`...). Une release est figée : elle reste en ligne et téléchargeable pour toujours.
- La branche `main` sert à préparer la prochaine version. Les contributions y arrivent par pull request.
- La version 1.0.0 ne sera jamais remplacée par la 2.0.0. Elles restent côte à côte sur la page *Releases*.

### Ta première contribution (depuis le site, sans ligne de commande)

1. Clique sur **Fork** (en haut à droite de la page du dépôt), puis **Create fork**.
2. Dans **ta** copie, ouvre le fichier à améliorer et clique sur le crayon (**Edit this file**).
3. Fais ta modification et clique sur **Commit changes**. Choisis **Create a new branch for this commit**, donne-lui un nom court (par exemple `correction-faute`) et valide.
4. GitHub affiche un bouton vert **Compare & pull request**. Clique dessus.
5. Remplis le petit formulaire (quoi et pourquoi) et clique sur **Create pull request**.
6. Les vérifications automatiques se lancent (une coche verte, c'est bon). Le responsable relit et répond.

### Travailler sur ton ordinateur

Les commandes sont les mêmes que dans la partie anglaise ci-dessus. Avant d'ouvrir une pull request, lance les mêmes vérifications que le robot :

```bash
node scripts/check-i18n.js
cd src-tauri
cargo clippy --all-targets -- -D warnings
cargo test
```

### Les règles du projet

1. **Commente ton code avec des mots simples.** Ce projet est aussi une école : un adolescent de 14 ans qui n'a jamais programmé doit pouvoir suivre. Les commentaires en français ou en anglais sont les bienvenus.
2. **La sécurité d'abord.** Ne lance jamais un programme via un shell (`cmd`, `sh`...). Ne fais jamais confiance à ce qui vient de la page web ou d'Internet : vérifie-le dans le Rust. Pas d'`innerHTML` avec des données venues de l'extérieur.
3. **Aucun secret** : jamais de mot de passe, jeton, cookie ou fichier personnel dans le dépôt.
4. **Ajoute un test** quand tu ajoutes de la logique dans `formats.rs`, `settings.rs` ou `input.rs`.
5. **Tout nouveau texte de l'interface** doit exister dans les 3 langues (`fr`, `en`, `zh`) dans `src/js/i18n.js`. `node scripts/check-i18n.js` le vérifie.
6. Une pull request = une idée. Petit, c'est beau.
7. En contribuant, tu acceptes que ton travail soit publié sous la [licence MIT](LICENSE) du projet.

### Reconnaissance

GitHub affiche automatiquement chaque contributeur dans la section **Contributors**, à droite de la page du dépôt. Les contributions importantes sont aussi remerciées dans les notes de version et dans [CONTRIBUTORS.md](CONTRIBUTORS.md).

### Sois bienveillant(e)

Lis le [Code de conduite](CODE_OF_CONDUCT.md). Pour les problèmes de sécurité, voir [SECURITY.md](SECURITY.md).
