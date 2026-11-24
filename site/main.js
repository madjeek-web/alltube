"use strict";
/* =============================================================================
   main.js : le petit script de la page de téléchargement
   =============================================================================
   Il fait 3 choses :
     1. choisit la langue (français ou anglais) et mémorise ton choix ;
     2. repère ton système (Windows, Mac, Linux) pour mettre le bon bouton en rouge ;
     3. demande à GitHub quelle est la DERNIÈRE version publiée, et branche chaque
        bouton directement sur le bon fichier. Si GitHub ne répond pas, les boutons
        renvoient simplement vers la page des versions : la page marche toujours.
   Aucun cookie, aucun traceur, aucune donnée envoyée à part la demande à l'API GitHub.
   ============================================================================= */
(function () {
  var REPO = "madjeek-web/alltube";
  var API = "https://api.github.com/repos/" + REPO + "/releases/latest";
  var DOWNLOAD_PREFIX = "https://github.com/" + REPO + "/releases/download/";
  var root = document.documentElement;

  /* ---------- 1. Langue ---------- */
  function setLang(code) {
    root.lang = code === "en" ? "en" : "fr";
    var buttons = document.querySelectorAll("[data-set-lang]");
    for (var i = 0; i < buttons.length; i++) {
      buttons[i].setAttribute("aria-pressed", String(buttons[i].getAttribute("data-set-lang") === root.lang));
    }
    try { localStorage.setItem("alltube-lang", root.lang); } catch (e) { /* stockage bloqué : tant pis */ }
  }
  var saved = null;
  try { saved = localStorage.getItem("alltube-lang"); } catch (e) { saved = null; }
  var guess = (navigator.language || "fr").toLowerCase().indexOf("fr") === 0 ? "fr" : "en";
  setLang(saved === "fr" || saved === "en" ? saved : guess);
  var langButtons = document.querySelectorAll("[data-set-lang]");
  for (var b = 0; b < langButtons.length; b++) {
    langButtons[b].addEventListener("click", function (event) { setLang(event.currentTarget.getAttribute("data-set-lang")); });
  }

  /* ---------- 2. Système de l'utilisateur ---------- */
  var platform = ((navigator.userAgentData && navigator.userAgentData.platform) || navigator.platform || navigator.userAgent || "").toLowerCase();
  var os = platform.indexOf("win") !== -1 ? "windows" : platform.indexOf("mac") !== -1 ? "mac" : platform.indexOf("linux") !== -1 ? "linux" : "windows";

  /* ---------- 3. Boutons et dernière version ---------- */
  function button(name) { return document.getElementById("dl-" + name); }

  // Un bouton qui n'a pas encore de fichier : grisé, il ne mène nulle part.
  document.querySelectorAll(".dl.soon").forEach(function (el) {
    el.addEventListener("click", function (event) { if (el.classList.contains("soon")) event.preventDefault(); });
  });
  var current = button(os);
  if (current && !current.classList.contains("soon")) current.classList.add("primary");

  function enable(name, asset) {
    var el = button(name);
    // On accepte uniquement un lien de téléchargement venant de NOTRE dépôt.
    if (!el || !asset || typeof asset.browser_download_url !== "string" || asset.browser_download_url.indexOf(DOWNLOAD_PREFIX) !== 0) return;
    el.href = asset.browser_download_url;
    el.classList.remove("soon");
    el.removeAttribute("aria-disabled");
    var note = document.getElementById("dl-" + name + "-note");
    if (note) {
      var mb = asset.size ? Math.round(asset.size / (1024 * 1024)) : 0;
      note.textContent = (asset.name || "") + (mb ? " · " + mb + " MB" : "");
    }
    if (name === os) el.classList.add("primary");
  }

  function find(assets, test) {
    for (var i = 0; i < assets.length; i++) {
      if (test(String(assets[i].name || "").toLowerCase())) return assets[i];
    }
    return null;
  }

  fetch(API, { headers: { Accept: "application/vnd.github+json" } })
    .then(function (response) {
      if (!response.ok) throw new Error("HTTP " + response.status);
      return response.json();
    })
    .then(function (release) {
      var assets = Array.isArray(release.assets) ? release.assets : [];
      var version = String(release.tag_name || "").replace(/[^0-9A-Za-z.\-+v]/g, "");
      if (version) document.getElementById("release-version").textContent = version;

      enable("windows", find(assets, function (n) { return n.slice(-4) === ".exe" && n.indexOf("setup") !== -1; }) || find(assets, function (n) { return n.slice(-4) === ".exe"; }));
      enable("mac", find(assets, function (n) { return n.slice(-4) === ".dmg"; }));
      enable("linux", find(assets, function (n) { return n.slice(-9) === ".appimage"; }) || find(assets, function (n) { return n.slice(-4) === ".deb"; }));

      var sums = find(assets, function (n) { return n === "sha256sums.txt"; });
      if (sums && typeof sums.browser_download_url === "string" && sums.browser_download_url.indexOf(DOWNLOAD_PREFIX) === 0) {
        document.getElementById("release-checksums").href = sums.browser_download_url;
      }
    })
    .catch(function () {
      /* GitHub ne répond pas (hors ligne, limite atteinte...) : le bouton Windows renvoie vers la page des versions. */
    });
})();
