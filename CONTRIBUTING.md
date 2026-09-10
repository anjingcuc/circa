# Contributing to CIRCA

Thanks for your interest in improving CIRCA!

## Development setup

```bash
npm install
npm run dev
```

Prerequisites: [Rust](https://rustup.rs/) stable, [Node.js](https://nodejs.org/) LTS, Windows 10/11 with WebView2.

## Before submitting a PR

1. Keep the app small and dependency-light — that is the point of the Tauri port.
2. Frontend changes go in `dist/` (plain HTML/CSS/JS, no framework, no bundler).
3. Rust changes go in `src-tauri/src/main.rs`; keep platform-specific code behind `#[cfg(...)]`.
4. Make sure `cargo fmt --check` and `cargo clippy` pass in `src-tauri/`.
5. Bump the version in **both** `src-tauri/tauri.conf.json` and `src-tauri/Cargo.toml` when shipping a release.

## Reporting issues

Include your Windows version, camera model (if relevant), and steps to reproduce. For crashes, attach the console output from `npm run dev`.
