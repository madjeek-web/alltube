"use strict";
/* =============================================================================
   app.js : le COMPORTEMENT de la fenêtre d'ALLTube
   =============================================================================
   Ce fichier fait vivre la page : il réagit aux clics, parle au "cerveau" Rust,
   et met à jour ce qu'on voit.

   Les mots à connaître :
     - une "fonction" est une petite recette (function maFonction() { ... }) ;
     - une "variable" est une boîte qui contient une valeur (const x = 5) ;
     - un "événement" est quelque chose qui arrive (un clic, une frappe...) ;
     - "invoke" = demander quelque chose au cerveau Rust ;
     - "listen" = écouter ce que le cerveau Rust nous envoie.

   RÈGLE DE SÉCURITÉ N°1 : les titres de vidéos viennent d'Internet, donc ils
   peuvent contenir n'importe quoi (même du code piégé). On les écrit TOUJOURS
   avec textContent (du simple texte) et JAMAIS avec innerHTML (qui exécuterait
   le code). C'est pour ça qu'on construit les éléments avec la fonction h().
   ============================================================================= */

/* -----------------------------------------------------------------------------
   0. PETITS OUTILS
   ----------------------------------------------------------------------------- */

// Parler au cerveau Rust.
const invoke = (command, args) => window.__TAURI__.core.invoke(command, args);
// Écouter les messages du cerveau Rust.
const listen = (name, callback) => window.__TAURI__.event.listen(name, callback);
// Raccourci pour trouver un élément de la page : $("#mon-id").
const $ = (selector) => document.querySelector(selector);

/* h("div", { class: "x", onclick: fn }, [enfants]) fabrique un élément HTML SANS
   JAMAIS interpréter du HTML : c'est la méthode sûre.
     - une valeur false / null est ignorée (pratique pour "disabled")
     - une clé qui commence par "on" ajoute un écouteur d'événement
     - les enfants de type texte sont ajoutés comme du TEXTE simple */
function h(tag, props = {}, children = []) {
  const node = document.createElement(tag);
  for (const [key, value] of Object.entries(props)) {
    if (value === false || value == null) continue;
    if (key.startsWith("on")) node.addEventListener(key.slice(2), value);
    else if (key === "class") node.className = value;
    else node.setAttribute(key, value === true ? "" : String(value));
  }
  for (const child of [].concat(children)) {
    if (child != null) node.append(child);
  }
  return node;
}

// Fabrique une icône (voir le CSS : l'icône vient d'un fichier .svg).
function icon(name, size = "") {
  return h("span", { class: ("icon " + size).trim(), "data-icon": name });
}

// Le code de langue pour les nombres et les dates ("zh" devient "zh-CN").
const locale = () => (currentLang === "zh" ? "zh-CN" : currentLang);

// 263 secondes -> "00:04:23"
function formatClock(seconds) {
  const total = Math.max(0, Math.round(seconds));
  const hh = Math.floor(total / 3600);
  const mm = Math.floor((total % 3600) / 60);
  const ss = total % 60;
  return [hh, mm, ss].map((n) => String(n).padStart(2, "0")).join(":");
}

// 263.46 secondes -> "00:04:23.5" (avec un dixième de seconde, pour les ciseaux)
function formatPrecise(seconds) {
  const tenthsTotal = Math.round(Math.max(0, seconds) * 10);
  const tenths = tenthsTotal % 10;
  return formatClock((tenthsTotal - tenths) / 10) + "." + tenths;
}

// "1:30", "01:02:03.5" ou "90" -> nombre de secondes. Renvoie NaN si c'est invalide.
function parseTime(text) {
  const parts = text.trim().replace(",", ".").split(":");
  if (parts.length > 3 || parts.some((p) => !/^\d+(\.\d+)?$/.test(p))) return NaN;
  return parts.reduce((total, p) => total * 60 + parseFloat(p), 0);
}

// 42471424 octets -> "40,50 MB" (selon la langue)
function formatSize(bytes) {
  const mb = bytes / (1024 * 1024);
  return mb.toLocaleString(locale(), { minimumFractionDigits: 2, maximumFractionDigits: 2 }) + " MB";
}

/* Le Rust renvoie les erreurs sous la forme "code" ou "code:détail".
   On transforme ça en phrase dans la langue de l'utilisateur. */
function errorText(error) {
  const raw = typeof error === "string" ? error : (error && error.message) || "";
  const cut = raw.indexOf(":");
  const code = cut === -1 ? raw : raw.slice(0, cut);
  const detail = cut === -1 ? "" : raw.slice(cut + 1).trim();
  const base = I18N.fr["err." + code] ? t("err." + code) : t("err.unknown");
  return detail ? base + " (" + detail + ")" : base;
}

/* -----------------------------------------------------------------------------
   1. L'ÉTAT : ce dont la page a besoin de se souvenir
   ----------------------------------------------------------------------------- */
const state = {
  settings: null,        // les réglages (reçus du Rust)
  defaultDir: "",        // dossier de téléchargement par défaut
  version: "",           // numéro de version de l'application
  formats: [],           // la liste des formats (reçue du Rust)
  formatId: "mp4_fhd",   // le format actuellement choisi
  custom: null,          // les choix de la fenêtre "Codecs custom"
  downloads: [],         // la liste affichée dans "Téléchargements récents"
  cut: { path: null, duration: 0, start: 0, end: 0, busy: false },
};
const cards = new Map();     // numéro de téléchargement -> les éléments de sa carte
const removedIds = new Set(); // numéros retirés (pour ignorer les messages en retard)

/* -----------------------------------------------------------------------------
   2. MESSAGE TEMPORAIRE (toast)
   ----------------------------------------------------------------------------- */
let toastTimer = null;
function toast(message, isError = false) {
  const el = $("#toast");
  el.textContent = message;
  el.className = "toast" + (isError ? " error" : "");
  el.hidden = false;
  clearTimeout(toastTimer);
  toastTimer = setTimeout(() => { el.hidden = true; }, 5500);
}

/* -----------------------------------------------------------------------------
   3. NAVIGATION : maison, ciseaux, paramètres
   -----------------------------------------------------------------------------
   Un seul écran est visible à la fois. Cliquer sur la maison ferme les
   paramètres, cliquer sur l'engrenage ferme l'accueil, etc. */
function showView(name) {
  for (const view of document.querySelectorAll(".view")) {
    view.classList.toggle("active", view.id === "view-" + name);
  }
  for (const button of document.querySelectorAll(".nav-btn")) {
    button.classList.toggle("active", button.dataset.view === name);
  }
  closePopups();
}

function closePopups() {
  toggleDropdown(false);
  closeMenu();
  $("#lang-pop").hidden = true;
}

/* -----------------------------------------------------------------------------
   4. LE MENU DES FORMATS (MP4 FHD ⌄) ET LA FENÊTRE "CODECS CUSTOM"
   ----------------------------------------------------------------------------- */
function formatLabel(id) {
  if (id === "custom") {
    return state.custom ? (state.custom.container + " " + state.custom.video).toUpperCase() : t("format.custom");
  }
  const found = state.formats.find((f) => f.id === id);
  return found ? found.label : id;
}

function updateFormatButton() {
  $("#format-label").textContent = formatLabel(state.formatId);
}

function renderFormatList(filter = "") {
  const box = $("#format-list");
  box.replaceChildren();
  const needle = filter.trim().toLowerCase();

  for (const group of ["video", "audio"]) {
    const items = state.formats.filter((f) => {
      if (f.group !== group) return false;
      const name = f.id === "custom" ? t("format.custom") : f.label;
      return !needle || (name + " " + f.badge).toLowerCase().includes(needle);
    });
    if (items.length === 0) continue;

    box.append(h("div", { class: "group-title" }, t(group === "video" ? "format.video" : "format.audio")));
    for (const f of items) {
      const selected = f.id === state.formatId;
      box.append(
        h("button", {
          class: "format-item" + (selected ? " selected" : ""),
          type: "button", role: "option", "aria-selected": String(selected),
          onclick: () => chooseFormat(f.id),
        }, [
          h("span", { class: "name" }, f.id === "custom" ? t("format.custom") : f.label),
          f.badge ? h("span", { class: "badge" }, f.badge) : null,
        ])
      );
    }
  }
}

function toggleDropdown(open) {
  const dropdown = $("#format-dropdown");
  const willOpen = open === undefined ? dropdown.hidden : open;
  dropdown.hidden = !willOpen;
  $("#format-btn").setAttribute("aria-expanded", String(willOpen));
  if (willOpen) {
    $("#format-search").value = "";
    renderFormatList();
    $("#format-search").focus();
  }
}

function chooseFormat(id) {
  toggleDropdown(false);
  if (id === "custom") {
    openCustomModal();
    return;
  }
  state.formatId = id;
  updateFormatButton();
  state.settings.last_format = id; // on se souvient du choix pour la prochaine fois
  saveSettingsSoon();
}

/* Règles : quels codecs vont dans quel conteneur.
   (Elles doivent rester identiques à celles de src-tauri/src/formats.rs.) */
const CODEC_RULES = {
  mp4:  { video: ["h264", "h265", "av1"],         audio: ["aac", "mp3"] },
  mkv:  { video: ["h264", "h265", "vp9", "av1"],  audio: ["aac", "mp3", "opus"] },
  webm: { video: ["vp9", "av1"],                  audio: ["opus"] },
};
const CODEC_NAMES = { h264: "H.264", h265: "H.265 (HEVC)", vp9: "VP9", av1: "AV1", aac: "AAC", mp3: "MP3", opus: "Opus" };
const HEIGHTS = [0, 2160, 1440, 1080, 720, 480]; // 0 = la qualité maximale

// Remplit une liste déroulante avec des choix.
function fillSelect(select, values, labelOf, selected) {
  select.replaceChildren();
  for (const v of values) {
    select.append(h("option", { value: String(v), selected: String(v) === String(selected) }, labelOf(v)));
  }
}

// Quand on change de conteneur, on met à jour les codecs proposés.
function refreshCodecChoices() {
  const rules = CODEC_RULES[$("#cc-container").value];
  fillSelect($("#cc-video"), rules.video, (v) => CODEC_NAMES[v], $("#cc-video").value);
  fillSelect($("#cc-audio"), rules.audio, (v) => CODEC_NAMES[v], $("#cc-audio").value);
}

function openCustomModal() {
  const previous = state.custom || { container: "mp4", video: "h264", audio: "aac", max_height: 1080 };
  fillSelect($("#cc-container"), Object.keys(CODEC_RULES), (v) => v.toUpperCase(), previous.container);
  fillSelect($("#cc-video"), CODEC_RULES[previous.container].video, (v) => CODEC_NAMES[v], previous.video);
  fillSelect($("#cc-audio"), CODEC_RULES[previous.container].audio, (v) => CODEC_NAMES[v], previous.audio);
  fillSelect($("#cc-height"), HEIGHTS, (v) => (v === 0 ? t("custom.best") : v + "p"), previous.max_height);
  $("#custom-modal").hidden = false;
  $("#cc-ok").focus();
}

function closeCustomModal() {
  $("#custom-modal").hidden = true;
}

function confirmCustomModal() {
  state.custom = {
    container: $("#cc-container").value,
    video: $("#cc-video").value,
    audio: $("#cc-audio").value,
    max_height: Number($("#cc-height").value),
  };
  state.formatId = "custom";
  updateFormatButton();
  closeCustomModal();
}

/* -----------------------------------------------------------------------------
   5. LA LISTE "TÉLÉCHARGEMENTS RÉCENTS"
   -----------------------------------------------------------------------------
   Chaque téléchargement a une "carte". On la construit UNE fois (buildCard),
   puis on la met à jour (fillCard) à chaque nouvelle information : comme ça
   la liste ne clignote pas et un menu ouvert ne se ferme pas tout seul. */
function buildCard(id) {
  const thumb = h("div", { class: "thumb" });
  const title = h("div", { class: "title" });
  const meta = h("div", { class: "meta" });
  const bar = h("div", { class: "bar" });
  const progress = h("div", { class: "progress" }, [bar]);
  const pill = h("span", { class: "pill" });
  const format = h("span", { class: "fmt" });
  const error = h("div", { class: "err-text" });
  const info = h("div", { class: "info" }, [title, meta, progress, h("div", { class: "badges" }, [pill, format]), error]);
  const dots = h("button", { class: "dots", type: "button", "aria-label": "Menu" }, [icon("dots", "sm")]);
  dots.addEventListener("click", (event) => {
    event.stopPropagation();
    openMenu(id, dots);
  });
  const root = h("div", { class: "card", "data-id": String(id) }, [thumb, info, dots]);
  return { root, thumb, title, meta, bar, progress, pill, format, error, img: null };
}

function fillCard(card, d) {
  card.title.textContent = d.title;
  card.title.setAttribute("title", d.title);

  // Miniature : seulement une adresse https (le Rust l'a déjà vérifié, on revérifie).
  if (d.thumbnail && d.thumbnail.startsWith("https://")) {
    if (!card.img) {
      card.img = h("img", { alt: "", draggable: "false", referrerpolicy: "no-referrer" });
      card.thumb.append(card.img);
    }
    if (card.img.getAttribute("src") !== d.thumbnail) card.img.setAttribute("src", d.thumbnail);
  }

  // Ligne d'infos : auteur, durée, taille.
  const parts = [];
  if (d.uploader) parts.push(h("span", {}, d.uploader));
  if (d.duration) parts.push(h("span", { class: "clock" }, [icon("clock", "xs"), formatClock(d.duration)]));
  if (d.size) {
    parts.push(h("span", { class: "sep" }));
    parts.push(h("span", {}, formatSize(d.size)));
  }
  card.meta.replaceChildren(...parts);

  // Étiquette d'état (TÉLÉCHARGEMENT, TERMINÉ...), barre de progression, format.
  card.pill.className = "pill " + d.status;
  card.pill.textContent = t("status." + d.status);
  card.format.replaceChildren(icon("play", "xs"), d.format_label);
  const active = d.status === "downloading" || d.status === "converting";
  card.progress.hidden = !active;
  card.bar.style.width = Math.round(d.progress * 100) + "%";
  card.error.hidden = d.status !== "error";
  card.error.textContent = d.status === "error" ? errorText(d.error || "") : "";
}

// Ajoute ou met à jour une ligne.
function upsert(d) {
  if (removedIds.has(d.id)) return; // une ligne déjà retirée ne doit pas revenir
  const index = state.downloads.findIndex((x) => x.id === d.id);
  if (index === -1) state.downloads.unshift(d);
  else state.downloads[index] = d;

  let card = cards.get(d.id);
  if (!card) {
    card = buildCard(d.id);
    cards.set(d.id, card);
    $("#downloads").prepend(card.root); // les plus récents en haut
  }
  fillCard(card, d);
  refreshEmpty();
}

function removeLocal(id) {
  removedIds.add(id);
  state.downloads = state.downloads.filter((d) => d.id !== id);
  const card = cards.get(id);
  if (card) card.root.remove();
  cards.delete(id);
  refreshEmpty();
}

// Affiche le message "Glissez-déposez..." quand la liste est vide.
function refreshEmpty() {
  const isEmpty = state.downloads.length === 0;
  $("#empty").hidden = !isEmpty;
  $("#clear-btn").hidden = isEmpty;
}

// Bouton "Tout effacer".
async function clearAll() {
  try {
    await invoke("clear_downloads");
    for (const d of state.downloads) removedIds.add(d.id);
    state.downloads = [];
    cards.clear();
    $("#downloads").replaceChildren();
    refreshEmpty();
  } catch (e) {
    toast(errorText(e), true);
  }
}

/* ----- Le petit menu "..." d'une carte ----- */
let menuEl = null;

function closeMenu() {
  if (menuEl) {
    menuEl.remove();
    menuEl = null;
  }
}

function openMenu(id, anchor) {
  closeMenu();
  const d = state.downloads.find((x) => x.id === id);
  if (!d) return;
  const done = d.status === "done";

  // Fabrique une ligne du menu. "run" est exécutée au clic.
  const entry = (iconName, key, run, options = {}) =>
    h("button", {
      type: "button",
      class: options.danger ? "danger" : "",
      disabled: options.disabled === true,
      onclick: async () => {
        closeMenu();
        try { await run(); } catch (e) { toast(errorText(e), true); }
      },
    }, [icon(iconName, "sm"), t(key)]);

  menuEl = h("div", { class: "menu", role: "menu" }, [
    entry("open", "menu.open", () => invoke("open_download", { id }), { disabled: !done }),
    entry("folder", "menu.folder", () => invoke("reveal_download", { id }), { disabled: !done }),
    entry("cut", "menu.cut", async () => {
      const path = await invoke("download_media_path", { id });
      await loadCutFile(path);
    }, { disabled: !done }),
    entry("trash", "menu.remove", async () => {
      await invoke("remove_download", { id });
      removeLocal(id);
    }, { danger: true }),
  ]);

  // On place le menu sous le bouton "...", aligné à droite (ou au-dessus s'il manque de place).
  document.body.append(menuEl);
  const rect = anchor.getBoundingClientRect();
  const height = menuEl.offsetHeight;
  const below = rect.bottom + 4 + height < window.innerHeight;
  menuEl.style.top = (below ? rect.bottom + 4 : Math.max(8, rect.top - height - 4)) + "px";
  menuEl.style.left = Math.max(8, rect.right - menuEl.offsetWidth) + "px";
}

/* -----------------------------------------------------------------------------
   6. LANCER UN TÉLÉCHARGEMENT (bouton OK, touche Entrée, CTRL + V)
   ----------------------------------------------------------------------------- */
async function startDownload(text) {
  const input = text.trim();
  if (!input) return;
  if (state.formatId === "custom" && !state.custom) {
    openCustomModal(); // il faut d'abord choisir les codecs
    return;
  }
  try {
    const item = await invoke("start_download", {
      input,
      formatId: state.formatId,
      custom: state.formatId === "custom" ? state.custom : null,
    });
    if (!cards.has(item.id)) upsert(item); // (l'événement du Rust a pu arriver avant)
    $("#search-input").value = "";
    showView("home");
  } catch (e) {
    toast(errorText(e), true);
  }
}

/* -----------------------------------------------------------------------------
   7. LES PARAMÈTRES
   ----------------------------------------------------------------------------- */
// Les langues de sous-titres qu'on propose d'ajouter.
const SUB_LANGS = ["fr", "en", "zh", "es", "de", "it", "pt", "ja", "ko", "ar", "ru", "hi", "tr", "nl", "pl", "sv", "uk", "vi", "id", "th"];

// "fr" -> "français" (ou "French", ou "法语", selon la langue de l'application)
function langName(code) {
  try {
    const name = new Intl.DisplayNames([locale()], { type: "language" }).of(code) || code;
    return name.charAt(0).toLocaleUpperCase(locale()) + name.slice(1);
  } catch {
    return code;
  }
}

function renderSubChips() {
  const box = $("#sub-chips");
  box.replaceChildren();
  for (const code of state.settings.subtitle_langs) {
    const remove = h("button", { class: "chip-x", type: "button", "aria-label": "×" }, [icon("close", "xs")]);
    remove.addEventListener("click", () => {
      state.settings.subtitle_langs = state.settings.subtitle_langs.filter((c) => c !== code);
      renderSubChips();
      saveSettingsSoon();
    });
    box.append(h("span", { class: "chip" }, [langName(code), remove]));
  }
  const add = h("button", { class: "add-lang", type: "button" }, [icon("plus", "xs"), t("set.addlang")]);
  add.addEventListener("click", (event) => {
    event.stopPropagation();
    toggleLangPopover();
  });
  box.append(add);
}

function toggleLangPopover() {
  const pop = $("#lang-pop");
  if (!pop.hidden) {
    pop.hidden = true;
    return;
  }
  pop.replaceChildren();
  const available = SUB_LANGS.filter((c) => !state.settings.subtitle_langs.includes(c));
  for (const code of available) {
    pop.append(h("button", {
      type: "button",
      onclick: () => {
        if (state.settings.subtitle_langs.length < 10) state.settings.subtitle_langs.push(code);
        pop.hidden = true;
        renderSubChips();
        saveSettingsSoon();
      },
    }, langName(code)));
  }
  pop.hidden = available.length === 0;
}

// Recopie les réglages dans les cases de la page "Paramètres".
function applySettingsToUI() {
  const s = state.settings;
  $("#set-folder-path").textContent = s.download_dir || state.defaultDir;
  $("#set-subs").checked = s.subtitles;
  $("#set-extra-audio").checked = s.extra_audio;
  $("#set-notify").checked = s.notify_done;
  $("#set-parallel").value = s.parallel;
  $("#set-delay").value = s.delay_secs;
  // Le délai n'a de sens que si on télécharge UN fichier à la fois : sinon on le grise.
  $("#row-delay").classList.toggle("disabled", s.parallel > 1);
  $("#set-delay").disabled = s.parallel > 1;
  $("#set-language").value = s.language;
  $("#set-theme").value = s.theme;
  $("#set-cookies").checked = s.use_cookies;
  $("#cookies-browser-row").hidden = !s.use_cookies;
  $("#set-cookies-browser").value = s.cookies_browser;
  document.documentElement.dataset.theme = s.theme;
  renderSubChips();
  $("#app-version").textContent = t("set.version", { v: state.version });
}

// On attend un tout petit peu avant d'enregistrer (pour ne pas écrire à chaque touche).
let saveTimer = null;
function saveSettingsSoon() {
  clearTimeout(saveTimer);
  saveTimer = setTimeout(saveSettingsNow, 300);
}

async function saveSettingsNow() {
  try {
    // Le Rust nettoie les réglages et nous renvoie la version propre.
    state.settings = await invoke("save_settings", { settings: state.settings });
  } catch (e) {
    toast(errorText(e), true);
  }
}

// Un nombre saisi : on le garde entre min et max.
function clampNumber(value, min, max, fallback) {
  const n = parseInt(value, 10);
  return Number.isNaN(n) ? fallback : Math.min(max, Math.max(min, n));
}

// Remet tous les textes "dynamiques" dans la nouvelle langue.
function repaintAll() {
  updateFormatButton();
  renderFormatList();
  for (const d of state.downloads) {
    const card = cards.get(d.id);
    if (card) fillCard(card, d);
  }
  applySettingsToUI();
}

function bindSettings() {
  const s = () => state.settings;

  $("#set-subs").addEventListener("change", (e) => { s().subtitles = e.target.checked; saveSettingsSoon(); });
  $("#set-extra-audio").addEventListener("change", (e) => { s().extra_audio = e.target.checked; saveSettingsSoon(); });
  $("#set-notify").addEventListener("change", (e) => { s().notify_done = e.target.checked; saveSettingsSoon(); });

  $("#set-parallel").addEventListener("change", (e) => {
    s().parallel = clampNumber(e.target.value, 1, 8, 2);
    applySettingsToUI();
    saveSettingsSoon();
  });
  $("#set-delay").addEventListener("change", (e) => {
    s().delay_secs = clampNumber(e.target.value, 0, 600, 0);
    applySettingsToUI();
    saveSettingsSoon();
  });

  $("#set-language").addEventListener("change", (e) => {
    s().language = e.target.value;
    setLanguage(s().language); // change tous les textes fixes
    repaintAll();              // et les textes dynamiques
    saveSettingsSoon();
  });
  $("#set-theme").addEventListener("change", (e) => {
    s().theme = e.target.value;
    document.documentElement.dataset.theme = s().theme;
    saveSettingsSoon();
  });

  $("#set-cookies").addEventListener("change", (e) => {
    s().use_cookies = e.target.checked;
    $("#cookies-browser-row").hidden = !s().use_cookies;
    saveSettingsSoon();
  });
  $("#set-cookies-browser").addEventListener("change", (e) => { s().cookies_browser = e.target.value; saveSettingsSoon(); });

  $("#set-folder-browse").addEventListener("click", async () => {
    const folder = await invoke("choose_folder");
    if (folder) {
      s().download_dir = folder;
      applySettingsToUI();
      saveSettingsSoon();
    }
  });
  $("#set-folder-default").addEventListener("click", () => {
    s().download_dir = ""; // vide = dossier par défaut
    applySettingsToUI();
    saveSettingsSoon();
  });
}

/* -----------------------------------------------------------------------------
   8. LES CISEAUX : découper une vidéo
   ----------------------------------------------------------------------------- */
// Charge un fichier dans l'éditeur de découpe.
async function loadCutFile(path) {
  try {
    const duration = await invoke("probe_media", { path }); // durée en secondes
    state.cut.path = path;
    state.cut.duration = duration;
    $("#cut-name").textContent = path.split(/[\\/]/).pop();
    $("#cut-start-range").max = String(duration);
    $("#cut-end-range").max = String(duration);
    setCutRange(0, duration);
    $("#cut-drop").hidden = true;
    $("#cut-editor").hidden = false;
    $("#cut-progress").hidden = true;
    showView("cut");
  } catch (e) {
    toast(errorText(e), true);
  }
}

// Met à jour les curseurs, les cases de texte et la durée de l'extrait.
function setCutRange(start, end) {
  const max = state.cut.duration;
  const minGap = Math.min(0.1, max);
  start = Math.min(Math.max(0, start), Math.max(0, max - minGap));
  end = Math.max(Math.min(max, end), start + minGap);
  state.cut.start = start;
  state.cut.end = end;
  $("#cut-start-range").value = String(start);
  $("#cut-end-range").value = String(end);
  $("#cut-start").value = formatPrecise(start);
  $("#cut-end").value = formatPrecise(end);
  $("#cut-length").textContent = formatPrecise(end - start);
}

async function pickMedia() {
  const path = await invoke("choose_media");
  if (path) await loadCutFile(path);
}

async function runCut() {
  if (state.cut.busy || !state.cut.path) return;
  state.cut.busy = true;
  $("#cut-go").disabled = true;
  $("#cut-bar").style.width = "0%";
  $("#cut-progress").hidden = false;
  try {
    const item = await invoke("cut_media", {
      path: state.cut.path,
      start: state.cut.start,
      end: state.cut.end,
      precise: $("#cut-precise").checked,
    });
    if (!cards.has(item.id)) upsert(item);
    toast(t("cut.done"));
  } catch (e) {
    toast(errorText(e), true);
  } finally {
    state.cut.busy = false;
    $("#cut-go").disabled = false;
    $("#cut-progress").hidden = true;
  }
}

function bindCut() {
  $("#cut-open").addEventListener("click", pickMedia);
  $("#cut-change").addEventListener("click", pickMedia);
  $("#cut-go").addEventListener("click", runCut);

  // Les curseurs.
  $("#cut-start-range").addEventListener("input", (e) => setCutRange(parseFloat(e.target.value), state.cut.end));
  $("#cut-end-range").addEventListener("input", (e) => setCutRange(state.cut.start, parseFloat(e.target.value)));

  // Les cases de texte (on accepte 90, 1:30 ou 01:02:03.5). Si c'est invalide, on remet l'ancienne valeur.
  $("#cut-start").addEventListener("change", (e) => {
    const v = parseTime(e.target.value);
    setCutRange(Number.isNaN(v) ? state.cut.start : v, state.cut.end);
  });
  $("#cut-end").addEventListener("change", (e) => {
    const v = parseTime(e.target.value);
    setCutRange(state.cut.start, Number.isNaN(v) ? state.cut.end : v);
  });
}

/* -----------------------------------------------------------------------------
   9. DÉMARRAGE : on branche tout
   ----------------------------------------------------------------------------- */
function bindGeneral() {
  // Boutons de la barre latérale.
  for (const button of document.querySelectorAll(".nav-btn")) {
    button.addEventListener("click", () => showView(button.dataset.view));
  }

  // Formulaire de recherche : OK ou Entrée.
  $("#search-form").addEventListener("submit", (event) => {
    event.preventDefault();
    startDownload($("#search-input").value);
  });

  // Menu des formats.
  $("#format-btn").addEventListener("click", (event) => {
    event.stopPropagation();
    toggleDropdown();
  });
  $("#format-search").addEventListener("input", (e) => renderFormatList(e.target.value));
  $("#format-search").addEventListener("keydown", (e) => {
    if (e.key === "Enter") {
      e.preventDefault();
      const first = $("#format-list .format-item");
      if (first) first.click();
    }
  });

  // Fenêtre "Codecs custom".
  $("#cc-container").addEventListener("change", refreshCodecChoices);
  $("#cc-ok").addEventListener("click", confirmCustomModal);
  $("#cc-cancel").addEventListener("click", closeCustomModal);

  $("#clear-btn").addEventListener("click", clearAll);

  // Un clic ailleurs ferme les menus ouverts.
  document.addEventListener("click", (event) => {
    if (!event.target.closest("#format-dropdown")) toggleDropdown(false);
    if (!event.target.closest(".menu")) closeMenu();
    if (!event.target.closest("#lang-pop")) $("#lang-pop").hidden = true;
  });

  // Échap ferme tout.
  document.addEventListener("keydown", (event) => {
    if (event.key === "Escape") {
      closePopups();
      closeCustomModal();
    }
  });

  // CTRL + V en dehors d'un champ de saisie : si c'est un lien, on télécharge tout de suite.
  document.addEventListener("paste", (event) => {
    const target = event.target;
    if (target instanceof HTMLInputElement || target instanceof HTMLTextAreaElement) return;
    const text = (event.clipboardData && event.clipboardData.getData("text")) || "";
    if (/^\s*(https?:\/\/|www\.)\S+\s*$/i.test(text)) startDownload(text);
  });
}

async function init() {
  setLanguage("fr"); // le français d'abord, le temps de lire les réglages
  bindGeneral();
  bindSettings();
  bindCut();

  try {
    // On demande tout au cerveau Rust en une seule fois.
    const [settings, formats, downloads, defaultDir, missing, version] = await Promise.all([
      invoke("get_settings"),
      invoke("get_formats"),
      invoke("get_downloads"),
      invoke("get_default_download_dir"),
      invoke("engine_status"),
      invoke("app_version"),
    ]);
    state.settings = settings;
    state.formats = formats;
    state.defaultDir = defaultDir;
    state.version = version;
    state.formatId = formats.some((f) => f.id === settings.last_format && f.id !== "custom") ? settings.last_format : "mp4_fhd";

    setLanguage(settings.language);
    applySettingsToUI();
    updateFormatButton();
    // La liste arrive "du plus récent au plus ancien" : on l'ajoute à l'envers
    // car upsert() place chaque nouvelle ligne tout en haut.
    for (const d of [...downloads].reverse()) upsert(d);
    refreshEmpty();
    $("#engine-banner").hidden = missing.length === 0;
  } catch (e) {
    toast(errorText(e), true);
  }

  // Messages envoyés par le cerveau Rust.
  await listen("download-update", (event) => upsert(event.payload));
  await listen("cut-progress", (event) => {
    $("#cut-bar").style.width = Math.round(event.payload * 100) + "%";
  });
  // Un fichier a été déposé sur la fenêtre : on l'ouvre dans les ciseaux.
  await listen("files-dropped", (event) => {
    const first = event.payload && event.payload[0];
    if (first) loadCutFile(first);
  });

  $("#search-input").focus();
}

init();
