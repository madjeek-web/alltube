# Guide utilisateur d'ALLTube (version 1.0)

Ce guide est écrit pour **tout le monde**. Si tu sais ouvrir un dossier et cliquer sur un bouton, tu y arriveras. English version: [user-guide.md](user-guide.md).

**Sommaire** : [1. Installer](#1-installer-alltube) · [2. Télécharger](#2-télécharger-une-vidéo-ou-une-musique) · [3. Retrouver tes fichiers](#3-retrouver-tes-fichiers) · [4. Les ciseaux](#4-découper-une-vidéo-les-ciseaux) · [5. Les paramètres](#5-les-paramètres) · [6. Problèmes](#6-si-quelque-chose-ne-va-pas) · [7. Désinstaller](#7-désinstaller)

---

## 1. Installer ALLTube

> ALLTube fonctionne sous **Windows 11**. Les versions Mac et Linux arriveront plus tard.

**Étape 1 : télécharger.**
1. Ouvre la page de téléchargement : <https://madjeek-web.github.io/alltube/>
2. Clique sur le gros bouton **Windows**.
3. Un fichier nommé `ALLTube_1.0.0_x64-setup.exe` se télécharge. Il arrive dans ton dossier **Téléchargements**.

**Étape 2 : lancer l'installation.**
1. Ouvre l'**Explorateur de fichiers** (le petit dossier jaune en bas de l'écran).
2. Clique sur **Téléchargements** dans la colonne de gauche.
3. Fais un **double-clic** sur `ALLTube_1.0.0_x64-setup.exe`.

**Étape 3 : si Windows affiche un message bleu.**
Tu peux voir *« Windows a protégé votre ordinateur »*. C'est normal pour un logiciel qui n'est pas encore signé numériquement. Si tu as téléchargé le fichier depuis la page officielle d'ALLTube :
1. Clique sur **Informations complémentaires**.
2. Clique sur **Exécuter quand même**.

**Étape 4 : suivre l'installateur.**
1. Choisis ta langue si on te la demande, puis clique sur **Suivant** / **Installer**.
2. Attends la fin, puis clique sur **Terminer**.
3. Un raccourci **ALLTube** apparaît sur ton **Bureau** et dans le **menu Démarrer**.

**Astuce : l'épingler à la barre des tâches.** Ouvre ALLTube, fais un **clic droit** sur son icône dans la barre en bas de l'écran, puis choisis **Épingler à la barre des tâches**.

---

## 2. Télécharger une vidéo ou une musique

1. **Copie le lien** de la vidéo : dans ton navigateur, clique dans la barre d'adresse, fais `CTRL + C`.
2. Ouvre **ALLTube**.
3. Clique dans la grande barre du haut (« URL, mots-clés… ») et fais `CTRL + V` pour coller le lien.
   *(Astuce : si tu fais `CTRL + V` ailleurs dans la fenêtre, le téléchargement démarre tout de suite.)*
4. À droite de la barre, clique sur le **format** (par défaut « MP4 FHD »). Une liste s'ouvre :
   - **Vidéo** : MP4 SD, HD, FHD, 2K, 3GP, FLV, Codecs custom.
   - **Audio** : MP3 (3 qualités), M4A, OGG, FLAC, WAV.
   - Pour de la **musique seulement**, choisis **MP3 HQ** (très bonne qualité, lisible partout).
   - Pour une **vidéo qui marche partout**, choisis **MP4 FHD** ou **MP4 HD**.
5. Clique sur le bouton rouge **OK**.
6. Une ligne apparaît dans **Téléchargements récents**. Tu vois l'avancement. Quand c'est vert avec **TERMINÉ**, c'est prêt !

Tu peux lancer plusieurs téléchargements à la suite : ils se mettent en file d'attente. Il n'y a **aucune limite par jour**.

Tu peux aussi taper des **mots-clés** (par exemple le nom d'une chanson) à la place d'un lien : ALLTube prend le premier résultat de la recherche.

**Le petit menu « ⋮ » de chaque ligne** :
- **Ouvrir** : lance le fichier avec ton lecteur (par exemple VLC).
- **Ouvrir le dossier** : montre le fichier dans l'Explorateur.
- **Découper la vidéo** : ouvre les ciseaux avec ce fichier.
- **Retirer** : enlève la ligne de la liste (le fichier reste sur ton ordinateur).

Le bouton **Tout effacer** vide la liste « Téléchargements récents » (tes fichiers ne sont **pas** supprimés).

---

## 3. Retrouver tes fichiers

C'est la question la plus importante. Voici le chemin, pas à pas :

1. Clique sur le petit **dossier jaune** en bas de ton écran (l'**Explorateur de fichiers**).
2. Dans la colonne de gauche, clique sur **Téléchargements**.
3. Ouvre le dossier **ALLTube**.
4. Tes vidéos et tes musiques sont là.

```
Ce PC  >  Téléchargements  >  ALLTube
```

**Raccourci** : dans ALLTube, clique sur les trois points **⋮** d'une ligne terminée, puis sur **Ouvrir le dossier**. Le dossier s'ouvre directement.

**Pour les garder ailleurs** : fais glisser le fichier avec la souris vers ton Bureau ou vers un autre dossier. Pour lire une vidéo ou une musique, double-clique dessus (ou utilise un lecteur comme **VLC**).

**Changer le dossier** : dans ALLTube, clique sur l'engrenage (**Paramètres**), puis **Parcourir** sous « Dossier des téléchargements ». « Dossier par défaut » remet `Téléchargements > ALLTube`.

---

## 4. Découper une vidéo (les ciseaux)

1. Clique sur l'icône **ciseaux** dans la barre de gauche.
2. Clique sur **Ou cliquez ici pour ouvrir un média** et choisis ta vidéo (ou ta musique). Tu peux aussi **glisser le fichier** dans la fenêtre.
3. Règle le **Début** et la **Fin** avec les curseurs. Tu peux aussi écrire le temps, par exemple `1:30` pour 1 minute 30.
4. Choisis le mode :
   - **Case décochée** (rapide) : la coupe est instantanée, mais peut varier de quelques secondes.
   - **« Découpe précise » cochée** : plus lente, mais coupe exactement où tu l'as choisi.
5. Clique sur **Découper**. Le nouveau fichier (avec `_cut` à la fin du nom) apparaît dans la liste et dans ton dossier ALLTube. Ton fichier d'origine n'est jamais modifié.

---

## 5. Les paramètres

Clique sur l'icône **engrenage**. Pour revenir à l'accueil, clique sur la **maison**.

| Réglage | À quoi ça sert |
|---|---|
| Dossier des téléchargements | Où les fichiers sont rangés |
| Récupérer les sous-titres | Ajoute les sous-titres dans la vidéo, dans les langues choisies |
| Autres pistes audio | Récupère d'autres langues audio si la vidéo en propose |
| Notification de fin | Un message Windows quand un téléchargement est terminé |
| Téléchargements simultanés | Combien de téléchargements en même temps (1 à 8) |
| Délai entre les téléchargements | Pause entre deux téléchargements (seulement avec « 1 à la fois ») |
| Langue | **Français**, **English** ou **中文** |
| Thème | Sombre ou clair |
| Cookies du navigateur | Pour des vidéos privées auxquelles **ton compte** a droit. Choisis ton navigateur (Firefox marche le mieux) |

Tes réglages sont enregistrés automatiquement.

---

## 6. Si quelque chose ne va pas

**Un message rouge « Erreur » apparaît, mais je pense que ça a marché.**
Dans la version 1.0, ça peut arriver même quand tout s'est bien passé. **Vérifie le dossier** `Téléchargements > ALLTube` : le fichier y est souvent. Ouvre-le pour vérifier qu'il se lit bien. Si c'est une vraie erreur, copie le message rouge complet et signale-le (voir plus bas).

**Rien ne se passe quand je clique sur OK.**
Vérifie que le lien est complet (il commence par `https://`) et que tu as de la connexion Internet.

**Le lien ne marche pas.**
Certains liens sont privés, supprimés ou limités à un pays. Les sites changent aussi parfois : une mise à jour d'ALLTube peut être nécessaire.

**Windows ou mon antivirus bloque l'installateur.**
Vérifie que tu l'as téléchargé depuis la page officielle d'ALLTube. Tu peux vérifier le fichier avec son empreinte SHA-256 (voir [SECURITY.md](../SECURITY.md)). Si un antivirus le bloque à tort, tu peux le signaler à l'éditeur comme « faux positif ».

**Je veux signaler un bug ou proposer une idée.**
Sur la page du projet, onglet **Issues**, clique sur **New issue** : <https://github.com/madjeek-web/alltube/issues>. Décris ce qui s'est passé avec tes mots.

---

## 7. Désinstaller

1. Ouvre **Paramètres de Windows** (touche Windows, tape « Paramètres »).
2. Va dans **Applications**, puis **Applications installées**.
3. Cherche **ALLTube**, clique sur les trois points, puis **Désinstaller**.

Tes vidéos et tes musiques dans `Téléchargements > ALLTube` ne sont **pas** supprimées.

---

## Rappel important

ALLTube est un outil. **Tu es responsable de ce que tu télécharges.** Télécharge seulement ce que tu as le droit de copier : tes propres vidéos, les contenus libres (Creative Commons, domaine public) ou ceux dont l'auteur est d'accord. Respecte aussi les règles des sites. Plus d'informations dans le [README](../README.fr.md).
