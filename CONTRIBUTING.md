# 贡献指南

感谢你有意为 CIRCA 出力！

> [!NOTE]
> 本仓库以简体中文为主要语言，英文介绍见 [README.en.md](README.en.md)。

## 开发环境

```bash
npm install
npm run dev
```

前置要求：[Rust](https://rustup.rs/)（stable）、[Node.js](https://nodejs.org/)（LTS）、带 WebView2 的 Windows 10/11。

## 提交 PR 之前

1. 保持应用轻量、依赖精简——这是做 Tauri 移植版的初衷。
2. 前端改动放在 `dist/`（纯 HTML/CSS/JS，无框架、无打包器）。
3. Rust 改动放在 `src-tauri/src/main.rs`，平台相关代码请用 `#[cfg(...)]` 隔离。
4. 确保在 `src-tauri/` 下 `cargo fmt --check` 与 `cargo clippy` 通过。
5. 发版时需**同时**更新 `src-tauri/tauri.conf.json` 与 `src-tauri/Cargo.toml` 中的版本号。

## 反馈问题

提 issue 时请附上 Windows 版本、摄像头型号（如相关）与复现步骤；如是崩溃，请附上 `npm run dev` 的控制台输出。
