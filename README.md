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

*A [Rust](https://www.rust-lang.org/) / [Tauri 2](https://v2.tauri.app/) port of [raffimd2/camera-floating-panel](https://github.com/raffimd2/camera-floating-panel) (Electron).*

</div>

---

## ✨ Features

- 🎥 **Circular webcam overlay** — frameless, transparent, always on top of everything, including fullscreen PowerPoint slideshows
- 🖱️ **Drag to move, scroll to resize** (120–800 px, always a perfect circle)
- 📷 **Camera picker** with hot-plug support — plug in a new camera and the list refreshes instantly; your choice is remembered across launches
- 👻 **Click-through mode** — let clicks fall through to your slides; the overlay keeps floating above while staying out of your way
- 🪶 **Tiny footprint** — a small native binary (Rust + system WebView2) instead of a bundled Chromium

> CIRCA only renders video. Audio is handled by whatever screen recorder you use — record with your tool's **full-screen capture mode** so it picks up both your slides and the overlay together.

## 🎮 Controls

Hover over the circle to reveal the control bar.

| Action | How |
|---|---|
| Move | Drag anywhere on the circle |
| Resize | Scroll wheel on the circle (120–800 px) |
| Switch camera | Dropdown in the hover bar (auto-refreshes on device changes) |
| Toggle click-through | 👆 button or **Ctrl+Shift+C** (global hotkey, works even while click-through is on) |
| Minimize / Close | — / ✕ buttons in the hover bar |

**Tip for click-through:** while it is on, moving the cursor over the bottom control bar automatically makes the bar clickable again (a Rust-side cursor watcher), so you can always toggle it off without the hotkey.

## 📥 Download

Grab the latest installer from the **[Releases](https://github.com/anjingcuc/circa/releases/latest)** page:

- `CIRCA_x.y.z_x64-setup.exe` — NSIS installer (recommended)

Windows 10/11 (x64) is the supported platform. On first launch, accept the Windows camera permission prompt once so camera labels can populate.

## 🛠️ Build from source

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
.github/workflows/       CI checks + automated release builds
```

## 🔀 vs. the original [camera-floating-panel](https://github.com/raffimd2/camera-floating-panel)

| | Original (Electron) | CIRCA (Tauri / Rust) |
|---|---|---|
| Runtime | Bundled Chromium (Electron 32) | System WebView2, native Rust core |
| Installer size | ~80 MB+ | ~3 MB |
| Click-through | Electron mouse-move forwarding | Rust-side 30 Hz OS cursor polling (`Win32 GetCursorPos`) keeps the control bar reachable |
| UI | Vanilla HTML/CSS/JS | Same — reused from the original |
| Platforms | Windows | Windows (NSIS build) |

## 🙏 Acknowledgments

- **[raffimd2/camera-floating-panel](https://github.com/raffimd2/camera-floating-panel)** — the original Electron app. CIRCA is a Rust/Tauri port of it and reuses its frontend UI and design. All credit for the original idea and UX goes to the original author.
- Built with [Tauri 2](https://v2.tauri.app/).

## 📄 License

This port is released under the [MIT License](LICENSE).

The upstream project does not declare a license; it is credited here as the source of the original design and UI. If you are the upstream author and would like different attribution or licensing terms, please [open an issue](https://github.com/anjingcuc/circa/issues).

<div align="center">
<sub>Made with 🦀 while recording slides.</sub>
</div>
