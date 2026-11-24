"use strict";
/* =============================================================================
   i18n.js : les TRADUCTIONS (français, anglais, mandarin)
   =============================================================================
   "i18n" est l'abréviation de "internationalisation" (i + 18 lettres + n).
   Ici, chaque texte de l'application a une CLÉ (par exemple "home.recent") et
   une traduction dans chaque langue.

   Pour AJOUTER une langue plus tard :
     1. copie le bloc "en" en dessous, change le code (ex : "es" pour l'espagnol)
        et traduis chaque phrase ;
     2. ajoute <option value="es">Español</option> dans index.html ;
     3. ajoute "es" dans UI_LANGUAGES dans src-tauri/src/settings.rs.

   Dans une phrase, {v} est une "case à remplir" (par exemple le numéro de version).
   ============================================================================= */

const I18N = {
  /* ------------------------------ FRANÇAIS (langue par défaut) ------------------------------ */
  fr: {
    "nav.home": "Accueil",
    "nav.cut": "Découper une vidéo",
    "nav.settings": "Paramètres",
    "search.placeholder": "URL, mots-clés…",
    "search.ok": "OK",
    "home.recent": "Téléchargements récents",
    "home.clear": "Tout effacer",
    "home.drop": "Glissez-déposez votre lien ou utilisez CTRL + V",
    "home.compat": "Compatible avec X, YouTube, TikTok et bien plus !",
    "status.queued": "En attente",
    "status.downloading": "Téléchargement",
    "status.converting": "Conversion",
    "status.done": "Terminé",
    "status.error": "Erreur",
    "menu.open": "Ouvrir",
    "menu.folder": "Ouvrir le dossier",
    "menu.cut": "Découper la vidéo",
    "menu.remove": "Retirer",
    "format.search": "Rechercher un format…",
    "format.video": "Vidéo",
    "format.audio": "Audio",
    "format.custom": "Codecs custom",
    "custom.title": "Codecs personnalisés",
    "custom.container": "Conteneur",
    "custom.video": "Codec vidéo",
    "custom.audio": "Codec audio",
    "custom.height": "Résolution maximale",
    "custom.best": "Maximale",
    "custom.ok": "Valider",
    "custom.cancel": "Annuler",
    "cut.title": "Découper une vidéo",
    "cut.drop": "Glissez-déposez votre vidéo ici",
    "cut.open": "Ou cliquez ici pour ouvrir un média",
    "cut.change": "Changer de fichier",
    "cut.start": "Début",
    "cut.end": "Fin",
    "cut.duration": "Durée de l'extrait :",
    "cut.precise": "Découpe précise (plus lente)",
    "cut.precise.hint": "Recompresse le fichier pour couper à l'image près. Sinon la coupe est instantanée mais peut varier de quelques secondes selon la vidéo.",
    "cut.go": "Découper",
    "cut.done": "Découpe terminée. Le fichier est dans la liste des téléchargements.",
    "settings.title": "Paramètres",
    "set.folder": "Dossier des téléchargements",
    "set.folder.desc": "Dossier de destination pour les fichiers téléchargés",
    "set.browse": "Parcourir",
    "set.default": "Dossier par défaut",
    "set.subs": "Récupérer les sous-titres",
    "set.subs.desc": "Télécharge les sous-titres dans les fichiers vidéo",
    "set.addlang": "Ajouter une langue",
    "set.audio": "Autres pistes audio",
    "set.audio.desc": "Récupère d'autres pistes audio si disponibles",
    "set.notify": "Notification de fin",
    "set.notify.desc": "Affiche une notification à la fin des téléchargements",
    "set.parallel": "Téléchargements simultanés",
    "set.parallel.desc": "Maximum de téléchargements en parallèle",
    "set.delay": "Délai entre les téléchargements",
    "set.delay.desc": "Secondes d'attente entre deux téléchargements. Incompatible avec les téléchargements en parallèle",
    "set.language": "Langue",
    "set.language.desc": "Traduction de l'application",
    "set.theme": "Thème",
    "set.theme.desc": "Le thème de l'application",
    "theme.dark": "Sombre",
    "theme.light": "Clair",
    "set.cookies": "Utiliser les cookies du navigateur",
    "set.cookies.desc": "Permet de télécharger les vidéos privées",
    "set.cookies.browser": "Navigateur :",
    "set.version": "Version de l'application : {v}",
    "err.invalid_input": "Lien ou mots-clés invalides.",
    "err.engine_missing": "Les moteurs de téléchargement (yt-dlp et ffmpeg) sont introuvables. Réinstalle ALLTube.",
    "err.engine_start": "Impossible de lancer le moteur de téléchargement.",
    "err.bad_format": "Format inconnu.",
    "err.bad_codecs": "Cette combinaison de codecs n'est pas compatible avec ce conteneur.",
    "err.file_not_found": "Fichier introuvable (déplacé ou supprimé ?).",
    "err.bad_file_type": "Ce type de fichier n'est pas pris en charge.",
    "err.bad_time_range": "La fin doit venir après le début.",
    "err.download_failed": "Échec du téléchargement",
    "err.convert_failed": "Échec de la conversion",
    "err.cut_failed": "Échec de la découpe",
    "err.io": "Erreur d'accès au fichier",
    "err.interrupted": "Téléchargement interrompu",
    "err.unknown": "Erreur inconnue"
  },

  /* ------------------------------ ENGLISH ------------------------------ */
  en: {
    "nav.home": "Home",
    "nav.cut": "Cut a video",
    "nav.settings": "Settings",
    "search.placeholder": "URL, keywords…",
    "search.ok": "OK",
    "home.recent": "Recent downloads",
    "home.clear": "Clear all",
    "home.drop": "Drag and drop your link or press CTRL + V",
    "home.compat": "Works with X, YouTube, TikTok and many more!",
    "status.queued": "Queued",
    "status.downloading": "Downloading",
    "status.converting": "Converting",
    "status.done": "Done",
    "status.error": "Error",
    "menu.open": "Open",
    "menu.folder": "Open folder",
    "menu.cut": "Cut the video",
    "menu.remove": "Remove",
    "format.search": "Search a format…",
    "format.video": "Video",
    "format.audio": "Audio",
    "format.custom": "Custom codecs",
    "custom.title": "Custom codecs",
    "custom.container": "Container",
    "custom.video": "Video codec",
    "custom.audio": "Audio codec",
    "custom.height": "Maximum resolution",
    "custom.best": "Maximum",
    "custom.ok": "Confirm",
    "custom.cancel": "Cancel",
    "cut.title": "Cut a video",
    "cut.drop": "Drag and drop your video here",
    "cut.open": "Or click here to open a media file",
    "cut.change": "Change file",
    "cut.start": "Start",
    "cut.end": "End",
    "cut.duration": "Clip length:",
    "cut.precise": "Precise cut (slower)",
    "cut.precise.hint": "Re-encodes the file to cut to the exact frame. Otherwise the cut is instant but can be off by a few seconds depending on the video.",
    "cut.go": "Cut",
    "cut.done": "Cut finished. The file is in the downloads list.",
    "settings.title": "Settings",
    "set.folder": "Downloads folder",
    "set.folder.desc": "Destination folder for downloaded files",
    "set.browse": "Browse",
    "set.default": "Default folder",
    "set.subs": "Get subtitles",
    "set.subs.desc": "Downloads subtitles into the video files",
    "set.addlang": "Add a language",
    "set.audio": "Other audio tracks",
    "set.audio.desc": "Gets other audio tracks when available",
    "set.notify": "Completion notification",
    "set.notify.desc": "Shows a notification when downloads finish",
    "set.parallel": "Simultaneous downloads",
    "set.parallel.desc": "Maximum number of parallel downloads",
    "set.delay": "Delay between downloads",
    "set.delay.desc": "Seconds to wait between two downloads. Not compatible with parallel downloads",
    "set.language": "Language",
    "set.language.desc": "Translation of the application",
    "set.theme": "Theme",
    "set.theme.desc": "The application theme",
    "theme.dark": "Dark",
    "theme.light": "Light",
    "set.cookies": "Use browser cookies",
    "set.cookies.desc": "Lets you download private videos",
    "set.cookies.browser": "Browser:",
    "set.version": "Application version: {v}",
    "err.invalid_input": "Invalid link or keywords.",
    "err.engine_missing": "The download engines (yt-dlp and ffmpeg) could not be found. Please reinstall ALLTube.",
    "err.engine_start": "Could not start the download engine.",
    "err.bad_format": "Unknown format.",
    "err.bad_codecs": "This codec combination is not compatible with this container.",
    "err.file_not_found": "File not found (moved or deleted?).",
    "err.bad_file_type": "This file type is not supported.",
    "err.bad_time_range": "The end must be after the start.",
    "err.download_failed": "Download failed",
    "err.convert_failed": "Conversion failed",
    "err.cut_failed": "Cut failed",
    "err.io": "File access error",
    "err.interrupted": "Download interrupted",
    "err.unknown": "Unknown error"
  },

  /* ------------------------------ 中文 (普通话, 简体) ------------------------------ */
  zh: {
    "nav.home": "主页",
    "nav.cut": "剪切视频",
    "nav.settings": "设置",
    "search.placeholder": "链接或关键词…",
    "search.ok": "确定",
    "home.recent": "最近下载",
    "home.clear": "全部清除",
    "home.drop": "将链接拖放到此处，或按 CTRL + V",
    "home.compat": "支持 X、YouTube、TikTok 等更多网站！",
    "status.queued": "等待中",
    "status.downloading": "下载中",
    "status.converting": "转换中",
    "status.done": "已完成",
    "status.error": "错误",
    "menu.open": "打开",
    "menu.folder": "打开所在文件夹",
    "menu.cut": "剪切视频",
    "menu.remove": "移除",
    "format.search": "搜索格式…",
    "format.video": "视频",
    "format.audio": "音频",
    "format.custom": "自定义编码",
    "custom.title": "自定义编码",
    "custom.container": "容器格式",
    "custom.video": "视频编码",
    "custom.audio": "音频编码",
    "custom.height": "最大分辨率",
    "custom.best": "最高",
    "custom.ok": "确认",
    "custom.cancel": "取消",
    "cut.title": "剪切视频",
    "cut.drop": "将视频拖放到此处",
    "cut.open": "或点击此处打开媒体文件",
    "cut.change": "更换文件",
    "cut.start": "开始",
    "cut.end": "结束",
    "cut.duration": "片段时长：",
    "cut.precise": "精确剪切（较慢）",
    "cut.precise.hint": "重新编码文件，可精确到帧。否则剪切是即时的，但根据视频不同，可能有几秒的误差。",
    "cut.go": "剪切",
    "cut.done": "剪切完成。文件已添加到下载列表。",
    "settings.title": "设置",
    "set.folder": "下载文件夹",
    "set.folder.desc": "下载文件的保存位置",
    "set.browse": "浏览",
    "set.default": "默认文件夹",
    "set.subs": "获取字幕",
    "set.subs.desc": "将字幕下载到视频文件中",
    "set.addlang": "添加语言",
    "set.audio": "其他音轨",
    "set.audio.desc": "如有可用，获取其他音轨",
    "set.notify": "完成通知",
    "set.notify.desc": "下载完成后显示通知",
    "set.parallel": "同时下载数",
    "set.parallel.desc": "最大并行下载数量",
    "set.delay": "下载间隔",
    "set.delay.desc": "两次下载之间等待的秒数。与并行下载不兼容",
    "set.language": "语言",
    "set.language.desc": "应用界面语言",
    "set.theme": "主题",
    "set.theme.desc": "应用的主题",
    "theme.dark": "深色",
    "theme.light": "浅色",
    "set.cookies": "使用浏览器 Cookie",
    "set.cookies.desc": "可下载私有视频",
    "set.cookies.browser": "浏览器：",
    "set.version": "应用版本：{v}",
    "err.invalid_input": "链接或关键词无效。",
    "err.engine_missing": "找不到下载引擎（yt-dlp 和 ffmpeg）。请重新安装 ALLTube。",
    "err.engine_start": "无法启动下载引擎。",
    "err.bad_format": "未知格式。",
    "err.bad_codecs": "此编码组合与该容器格式不兼容。",
    "err.file_not_found": "找不到文件（已被移动或删除？）。",
    "err.bad_file_type": "不支持此文件类型。",
    "err.bad_time_range": "结束时间必须晚于开始时间。",
    "err.download_failed": "下载失败",
    "err.convert_failed": "转换失败",
    "err.cut_failed": "剪切失败",
    "err.io": "文件访问错误",
    "err.interrupted": "下载已中断",
    "err.unknown": "未知错误"
  }
};

/* La langue actuellement utilisée. Au tout début : le français. */
let currentLang = "fr";

/* t("clé") renvoie le texte traduit. Si la traduction manque, on retombe sur
   le français, puis sur la clé elle-même (pour qu'on voie le problème). */
function t(key, vars) {
  let text = (I18N[currentLang] && I18N[currentLang][key]) || I18N.fr[key] || key;
  if (vars) {
    for (const [name, value] of Object.entries(vars)) {
      text = text.split("{" + name + "}").join(String(value));
    }
  }
  return text;
}

/* Change la langue et met à jour TOUS les textes fixes de la page :
     data-i18n="clé"              -> remplace le texte de l'élément
     data-i18n-placeholder="clé"  -> remplace le texte grisé d'un champ
     data-i18n-title="clé"        -> remplace l'infobulle (texte au survol)
     data-i18n-aria="clé"         -> remplace l'étiquette lue par les lecteurs d'écran */
function setLanguage(code) {
  currentLang = I18N[code] ? code : "fr";
  document.documentElement.lang = currentLang === "zh" ? "zh-CN" : currentLang;
  for (const el of document.querySelectorAll("[data-i18n]")) {
    el.textContent = t(el.dataset.i18n);
  }
  for (const el of document.querySelectorAll("[data-i18n-placeholder]")) {
    el.setAttribute("placeholder", t(el.dataset.i18nPlaceholder));
  }
  for (const el of document.querySelectorAll("[data-i18n-title]")) {
    el.setAttribute("title", t(el.dataset.i18nTitle));
  }
  for (const el of document.querySelectorAll("[data-i18n-aria]")) {
    el.setAttribute("aria-label", t(el.dataset.i18nAria));
  }
}
