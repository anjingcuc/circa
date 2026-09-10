<div align="center">

<img src="src-tauri/icons/icon.png" width="120" alt="CIRCA logo" />

# CIRCA

**CIR**cular **CA**mera — a tiny always-on-top circular webcam overlay.

Drop your face on top of slides, screen recordings, and podcasts — the way Loom and mmhmm do it — without hauling in a full video suite.

[![CI](https://github.com/anjingcuc/circa/actions/workflows/ci.yml/badge.svg)](https://github.com/anjingcuc/circa/actions/workflows/ci.yml)
[![Release](https://github.com/anjingcuc/circa/actions/workflows/release.yml/badge.svg)](https://github.com/anjingcuc/circa/actions/workflows/release.yml)
[![GitHub release](https://img.shields.io/github/v/release/anjingcuc/circa?color=blue)](https://github.com/anjingcuc/circa/releases/latest)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows-blue)](https://github.com/anjingcuc/circa/releases/latest)

[简体中文](README.md) | **English**

A Rust + [Tauri 2](https://v2.tauri.app/) port of [raffimd2/camera-floating-panel](https://github.com/raffimd2/camera-floating-panel) (Electron)

</div>

---

## Features

- **Circular webcam overlay** — frameless, transparent, always on top of everything, including fullscreen PowerPoint slideshows
- **Drag to move, scroll to resize** (120–800 px, always a perfect circle)
- **Camera picker** — hot-plug aware, refreshes instantly when you plug in a new device; your choice is remembered
- **Click-through mode** — let clicks fall through to your slides while the overlay keeps floating above
- **Tiny footprint** — native Rust core + system WebView2, ~2 MB installer

> [!NOTE]
> CIRCA only renders video. Use your screen recorder's **full-screen capture mode** so it picks up both your slides and the overlay; the recorder handles mic audio on its own.

## Download

Grab the latest build from the **[Releases](https://github.com/anjingcuc/circa/releases/latest)** page:

- `CIRCA_x.y.z_x64-setup.exe` — NSIS installer (recommended)

> [!IMPORTANT]
> Windows 10/11 (x64) is the supported platform. On first launch, accept the Windows camera permission prompt once so camera labels can populate.

## Usage

Hover over the circle to reveal the control bar.

| Action | How |
|---|---|
| Move | Drag anywhere on the circle |
| Resize | Scroll wheel on the circle (120–800 px) |
| Switch camera | Dropdown in the hover bar (auto-refreshes on device changes) |
| Toggle click-through | 👆 button or **Ctrl+Shift+C** (global hotkey, works even while click-through is on) |
| Minimize / Close | — / ✕ buttons in the hover bar |

> [!TIP]
> While click-through is on, moving the cursor over the bottom control bar automatically makes it clickable again (a Rust-side 30 Hz cursor watcher), so you can always toggle it off without the hotkey.

## Build from source

Prerequisites: [Rust](https://rustup.rs/) (stable), [Node.js](https://nodejs.org/) (LTS), Windows with WebView2.

```bash
git clone https://github.com/anjingcuc/circa.git
cd circa
npm install
npm run dev     # develop
npm run build   # produce the NSIS installer + standalone exe in src-tauri/target/release
```

### Project layout

```
dist/                    Frontend — plain HTML/CSS/JS (camera capture, UI wiring)
src-tauri/src/main.rs    Rust — window management, click-through cursor watcher,
                         global shortcut, resize command
src-tauri/               Tauri 2 app config, capabilities, icons, bundle settings
.github/workflows/       CI checks + tag-triggered release builds
```

## vs. the original

This is a Rust/Tauri port of [raffimd2/camera-floating-panel](https://github.com/raffimd2/camera-floating-panel) (Electron):

| | Original (Electron) | CIRCA (Tauri / Rust) |
|---|---|---|
| Runtime | Bundled Chromium (Electron 32) | System WebView2, native Rust core |
| Installer size | ~80 MB+ | ~2 MB |
| Click-through | Electron mouse-move forwarding | Rust-side 30 Hz OS cursor polling (`Win32 GetCursorPos`) keeps the control bar reachable |
| UI | Vanilla HTML/CSS/JS | Same — reused from the original |
| Platforms | Windows | Windows (NSIS build) |

## Acknowledgments

- **[raffimd2/camera-floating-panel](https://github.com/raffimd2/camera-floating-panel)** — the original Electron app. CIRCA is a Rust/Tauri port of it and reuses its frontend UI and design; all credit for the original idea and UX goes to the original author.
- Built with [Tauri 2](https://v2.tauri.app/). Released under the MIT license, which notes the derivation from the upstream project. If you are the upstream author and would like different attribution or licensing terms, please [open an issue](https://github.com/anjingcuc/circa/issues).

<div align="center">
<sub>Made with 🦀 while recording slides.</sub>
</div>
