# Vedmin 🧙‍♂️

[![Maintenance](https://img.shields.io/badge/Maintenance%20Status-Active-brightgreen.svg)](https://github.com/hugofelipe/vedmin)
[![Tauri Version](https://img.shields.io/badge/Tauri-2.0-blue.svg)](https://tauri.app)
[![Rust Version](https://img.shields.io/badge/Rust-1.75+-orange.svg)](https://rust-lang.org)
[![Svelte Version](https://img.shields.io/badge/Svelte-5-ff3e00.svg)](https://svelte.dev)

**Vedmin** is a high-performance desktop application designed for quick video preparation, specifically focused on creating perfect 1-minute clips from long screen recordings.

Built with **Tauri**, **Rust**, and **Svelte 5**, it provides a lightning-fast experience for trimming and accelerating videos without the complexity of a full-blown video editor.

## ✨ Features

- 🏎️ **Smart Acceleration**: Automatically calculates and applies the necessary speed factor to fit any video into exactly 60 seconds.
- ✂️ **Precision Trimming**: Quickly cut 1s or 2s from the end of the video to remove recorder UI interactions.
- 🔇 **Audio Stripping**: Completely removes audio tracks for clean, social-media-ready exports.
- 📽️ **CFR Enforcement**: Converts Variable Framerate (VFR) recordings (common in Apple ReplayKit/iOS) to a constant 60 FPS, ensuring frame-perfect duration.
- 🚀 **Native Performance**: Powered by FFmpeg and Rust for maximum processing speed.
- 🎨 **Minimalist UI**: Clean, intuitive interface with drag-and-drop support.

## 🛠️ Tech Stack

- **Backend**: [Rust](https://www.rust-lang.org/)
- **Framework**: [Tauri v2](https://tauri.app/)
- **Frontend**: [Svelte 5](https://svelte.dev/) + [SvelteKit](https://kit.svelte.dev/)
- **Video Engine**: [FFmpeg](https://ffmpeg.org/)
- **Icons**: [Lucide Svelte](https://lucide.dev/)
- **Package Manager**: [Bun](https://bun.sh/)

## 🚀 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Bun](https://bun.sh/) or Node.js
- [FFmpeg](https://ffmpeg.org/download.html) (must be available in your PATH)

### Development

1. Install dependencies:
   ```bash
   bun install
   ```

2. Run the app in development mode:
   ```bash
   bun run tauri dev
   ```

### Building

To create a production build for your machine:
```bash
bun run tauri build
```
