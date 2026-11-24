# Roadmap / Feuille de route

These are **ideas, not promises**. Want to help? Read [CONTRIBUTING.md](CONTRIBUTING.md).
Ce sont des **idées, pas des promesses**. Envie d'aider ? Lis [CONTRIBUTING.md](CONTRIBUTING.md).

## Next (1.x)

- [ ] **Fix the false red error**: when the engine finishes oddly, check that the file really exists first, and show the green "Done" state if it does.
      *Corriger le faux message d'erreur rouge : vérifier d'abord que le fichier existe, et afficher le vert « Terminé » si c'est le cas.*
- [ ] **"About" button** under the settings gear: author, MIT license, link to the GitHub profile, creation date, version.
      *Bouton « À propos » sous la roue crantée.*
- [ ] **Linux** installers (`.AppImage`, `.deb`) built by the `Release` workflow and tested on real machines.
- [ ] **macOS** installer (`.dmg`): needs engines for macOS, tests on a real Mac, and Apple signing to avoid warnings.
- [ ] **Code signing** of the Windows installer (free for open source through [SignPath Foundation](https://signpath.org)).
- [ ] Updating the download engine (yt-dlp) from inside the app.
- [ ] Video preview in the cutting tool.

## Later (2.x and beyond)

- [ ] **New visual identity**: own logo and icons with more personality.
      *Nouvelle identité visuelle : logo et icônes propres.*
- [ ] **Browser extension** (Chrome, Brave, Firefox): several downloads at once, unlimited, saved in the same `ALLTube` folder, desktop notification when finished, and **file names identical to the video title** (shortened if too long).
- [ ] **Public web version** with the same features and settings, also open source.
- [ ] More interface languages.
- [ ] Mobile app (not planned for now).
