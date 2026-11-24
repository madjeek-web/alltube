# ALLTube User Guide (version 1.0)

This guide is written for **everyone**. If you can open a folder and click a button, you can do this. Version française : [guide-utilisateur.md](guide-utilisateur.md).

**Contents**: [1. Install](#1-install-alltube) · [2. Download](#2-download-a-video-or-a-song) · [3. Find your files](#3-find-your-files) · [4. The scissors](#4-cut-a-video-the-scissors) · [5. Settings](#5-settings) · [6. Problems](#6-if-something-goes-wrong) · [7. Uninstall](#7-uninstall)

---

## 1. Install ALLTube

> ALLTube runs on **Windows 11**. Mac and Linux versions will come later.

**Step 1: download.**
1. Open the download page: <https://madjeek-web.github.io/alltube/>
2. Click the big **Windows** button.
3. A file named `ALLTube_1.0.0_x64-setup.exe` is downloaded into your **Downloads** folder.

**Step 2: start the installation.**
1. Open **File Explorer** (the small yellow folder at the bottom of the screen).
2. Click **Downloads** in the left column.
3. **Double-click** `ALLTube_1.0.0_x64-setup.exe`.

**Step 3: if Windows shows a blue message.**
You may see *"Windows protected your PC"*. This is normal for a program that is not code-signed yet. If you downloaded the file from the official ALLTube page:
1. Click **More info**.
2. Click **Run anyway**.

**Step 4: follow the installer.**
1. Choose your language if asked, then click **Next** / **Install**.
2. Wait until it finishes, then click **Finish**.
3. An **ALLTube** shortcut appears on your **Desktop** and in the **Start menu**.

**Tip: pin it to the taskbar.** Open ALLTube, **right-click** its icon in the bar at the bottom of the screen, then choose **Pin to taskbar**.

---

## 2. Download a video or a song

1. **Copy the link** of the video: in your browser, click the address bar and press `CTRL + C`.
2. Open **ALLTube**.
3. Click the big bar at the top ("URL, keywords…") and press `CTRL + V` to paste the link.
   *(Tip: if you press `CTRL + V` elsewhere in the window, the download starts right away.)*
4. On the right of the bar, click the **format** (default "MP4 FHD"). A list opens:
   - **Video**: MP4 SD, HD, FHD, 2K, 3GP, FLV, Custom codecs.
   - **Audio**: MP3 (3 qualities), M4A, OGG, FLAC, WAV.
   - For **music only**, choose **MP3 HQ** (great quality, plays everywhere).
   - For a **video that plays everywhere**, choose **MP4 FHD** or **MP4 HD**.
5. Click the red **OK** button.
6. A row appears in **Recent downloads**. You can follow the progress. When it turns green with **DONE**, it is ready!

You can start several downloads one after another: they wait in a queue. There is **no daily limit**.

You can also type **keywords** (for example the name of a song) instead of a link: ALLTube takes the first search result.

**The small "⋮" menu of each row**:
- **Open**: plays the file with your player (for example VLC).
- **Open folder**: shows the file in File Explorer.
- **Cut the video**: opens the scissors with this file.
- **Remove**: removes the row from the list (the file stays on your computer).

The **Clear all** button empties the "Recent downloads" list (your files are **not** deleted).

---

## 3. Find your files

This is the most important question. Here is the path, step by step:

1. Click the small **yellow folder** at the bottom of your screen (**File Explorer**).
2. In the left column, click **Downloads**.
3. Open the **ALLTube** folder.
4. Your videos and songs are there.

```
This PC  >  Downloads  >  ALLTube
```

**Shortcut**: in ALLTube, click the three dots **⋮** of a finished row, then **Open folder**. The folder opens directly.

**To keep them elsewhere**: drag the file with the mouse to your Desktop or another folder. To play a video or a song, double-click it (or use a player such as **VLC**).

**Change the folder**: in ALLTube, click the gear (**Settings**), then **Browse** under "Downloads folder". "Default folder" puts it back to `Downloads > ALLTube`.

---

## 4. Cut a video (the scissors)

1. Click the **scissors** icon in the left bar.
2. Click **Or click here to open a media file** and choose your video (or song). You can also **drag the file** into the window.
3. Set the **Start** and the **End** with the sliders. You can also type the time, for example `1:30` for 1 minute 30.
4. Choose the mode:
   - **Box unchecked** (fast): the cut is instant but may be off by a few seconds.
   - **"Precise cut" checked**: slower, but cuts exactly where you chose.
5. Click **Cut**. The new file (with `_cut` at the end of its name) appears in the list and in your ALLTube folder. Your original file is never changed.

---

## 5. Settings

Click the **gear** icon. To go back to the start, click the **house**.

| Setting | What it does |
|---|---|
| Downloads folder | Where files are stored |
| Get subtitles | Adds subtitles into the video, in the languages you choose |
| Other audio tracks | Gets other audio languages when the video has them |
| Completion notification | A Windows message when a download is finished |
| Simultaneous downloads | How many downloads at the same time (1 to 8) |
| Delay between downloads | Pause between two downloads (only with "1 at a time") |
| Language | **Français**, **English** or **中文** |
| Theme | Dark or light |
| Browser cookies | For private videos that **your account** may see. Choose your browser (Firefox works best) |

Your settings are saved automatically.

---

## 6. If something goes wrong

**A red "Error" appears, but I think it worked.**
In version 1.0 this can happen even when everything went fine. **Check the folder** `Downloads > ALLTube`: the file is often there. Open it to check that it plays. If it is a real error, copy the full red message and report it (see below).

**Nothing happens when I click OK.**
Check that the link is complete (it starts with `https://`) and that you have an Internet connection.

**The link does not work.**
Some links are private, deleted or limited to a country. Websites also change from time to time: an ALLTube update may be needed.

**Windows or my antivirus blocks the installer.**
Check that you downloaded it from the official ALLTube page. You can verify the file with its SHA-256 checksum (see [SECURITY.md](../SECURITY.md)). If an antivirus blocks it by mistake, you can report it to the vendor as a "false positive".

**I want to report a bug or suggest an idea.**
On the project page, **Issues** tab, click **New issue**: <https://github.com/madjeek-web/alltube/issues>. Describe what happened in your own words.

---

## 7. Uninstall

1. Open **Windows Settings** (Windows key, type "Settings").
2. Go to **Apps**, then **Installed apps**.
3. Find **ALLTube**, click the three dots, then **Uninstall**.

Your videos and songs in `Downloads > ALLTube` are **not** deleted.

---

## Important reminder

ALLTube is a tool. **You are responsible for what you download.** Only download what you have the right to copy: your own videos, free content (Creative Commons, public domain) or content whose author agrees. Respect the rules of websites too. More information in the [README](../README.md).
