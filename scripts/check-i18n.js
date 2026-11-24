"use strict";
/* =============================================================================
   check-i18n.js : vérifie que les traductions sont COMPLÈTES
   =============================================================================
   Lancer avec :   node scripts/check-i18n.js

   Il contrôle 3 choses :
     1. chaque texte français existe aussi en anglais et en mandarin (et inversement) ;
     2. chaque étiquette data-i18n="..." de index.html existe bien en français ;
     3. chaque identifiant $("#...") utilisé par app.js existe dans index.html.
   S'il trouve un problème, il l'affiche et se termine avec une erreur
   (c'est ce qui fait passer la CI en rouge).
   ============================================================================= */
const fs = require("fs");
const path = require("path");
const vm = require("vm");

const root = path.join(__dirname, "..");
const read = (file) => fs.readFileSync(path.join(root, file), "utf8");

// On exécute i18n.js dans une "bulle" pour récupérer le dictionnaire I18N.
const sandbox = {};
vm.createContext(sandbox);
vm.runInContext(read("src/js/i18n.js") + ";this.I18N = I18N;", sandbox);
const I18N = sandbox.I18N;

const problems = [];
const frKeys = Object.keys(I18N.fr);

// 1. Les trois langues ont exactement les mêmes clés.
for (const lang of Object.keys(I18N)) {
  for (const key of frKeys) {
    if (!(key in I18N[lang])) problems.push(`[${lang}] traduction manquante : ${key}`);
    else if (String(I18N[lang][key]).trim() === "") problems.push(`[${lang}] traduction vide : ${key}`);
  }
  for (const key of Object.keys(I18N[lang])) {
    if (!(key in I18N.fr)) problems.push(`[${lang}] clé en trop (absente du français) : ${key}`);
  }
}

// 2. Les étiquettes du HTML existent.
const html = read("src/index.html");
for (const match of html.matchAll(/data-i18n(?:-[a-z]+)?="([^"]+)"/g)) {
  if (!(match[1] in I18N.fr)) problems.push(`index.html utilise une clé inconnue : ${match[1]}`);
}

// 3. Les identifiants utilisés par app.js existent dans la page.
const app = read("src/js/app.js").replace(/\/\*[\s\S]*?\*\//g, "").replace(/\/\/.*$/gm, "");
for (const match of app.matchAll(/\$\("#([A-Za-z0-9_-]+)/g)) {
  if (!html.includes(`id="${match[1]}"`)) problems.push(`app.js cherche #${match[1]} mais il n'existe pas dans index.html`);
}

if (problems.length > 0) {
  console.error("Problèmes trouvés :\n - " + [...new Set(problems)].join("\n - "));
  process.exit(1);
}
console.log(`OK : ${frKeys.length} textes x ${Object.keys(I18N).length} langues, étiquettes HTML et identifiants cohérents.`);
