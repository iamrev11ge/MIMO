 <div align="center">

# MIMO

**A little island. A lot of possibilities.**

A compact, interactive island for Windows that brings your music, notifications, clipboard, files, and everyday tools together in one elegant space at the top of your screen.

[![release](https://img.shields.io/github/v/release/iamrev11ge/MIMO?label=release&color=555)](https://github.com/iamrev11ge/MIMO/releases/latest)
[![downloads](https://img.shields.io/github/downloads/iamrev11ge/MIMO/total?label=downloads&color=555)](https://github.com/iamrev11ge/MIMO/releases)
![size](https://img.shields.io/badge/size-~5%20MB-555)
[![license](https://img.shields.io/badge/license-MIT-555)](LICENSE)

[![Windows installer](https://img.shields.io/badge/WINDOWS-INSTALLER-0a7cd6?style=for-the-badge)](https://github.com/iamrev11ge/MIMO/releases/latest/download/MIMO-Setup.exe)
[![Windows zip](https://img.shields.io/badge/WINDOWS-.ZIP-1f2328?style=for-the-badge)](https://github.com/iamrev11ge/MIMO/releases/latest/download/MIMO-Windows.zip)

<img width="1585" height="992" alt="hero" src="https://github.com/user-attachments/assets/71a99301-6c21-4727-8f85-aa4cdd59466b" />


</div>

## Features

- **Media controls.** See what's playing, browse track details, view album artwork, and control playback with a built-in progress bar.
- **Dynamic themes.** Fluid animations, subtle motion, and music-reactive colors. Prefer something quieter? Turn the effects off.
- **Venom theme.** A distinctive dark, liquid-inspired appearance that transforms the island into a living shape.
- **Notifications.** Keep incoming notifications close, preview them in a compact layout, and open conversations with a click.
- **Quick tools.** Access screenshots, text recognition, clipboard history, notes, passwords, timers, notifications, and settings.
- **Smart clipboard.** Recognizes copied links, calculations, dates, and colors, offering useful actions automatically.
- **File shelf.** Drag and drop files onto MIMO to keep up to five items ready for quick access.
- **Floating timer.** Keep track of time without losing sight of your timer, even above fullscreen applications.
- **Integrated settings.** Customize your experience directly inside the island without opening separate windows.

When launched for the first time, MIMO introduces its features through an interactive guide. You can revisit it anytime from **What MIMO can do** in settings.

### More features

- **Quick notes.** Press `Ctrl+Alt+N` anywhere to create a note. MIMO can recognize time expressions and turn them into reminders.
- **Password manager.** Generate and save passwords securely using Windows DPAPI encryption. Copied passwords are automatically cleared from the clipboard after 30 seconds.
- **Screen text recognition.** Press `Ctrl+Alt+T`, select an area, and extract text. Supports English through Windows OCR and Persian through bundled Tesseract.
- **Clipboard history.** Browse previously copied text and images, pin important items, and retrieve them whenever needed.
- **System indicators.** Monitor microphone and camera activity, network and VPN changes, Bluetooth devices, and charging status.
- **Automatic updates.** Check for new versions, verify downloaded updates using the project's signing key, and install them with a single click.

## Installation

Download the latest **[MIMO Setup](https://github.com/iamrev11ge/MIMO/releases/download/v1.0.0/MIMO.exe)** and launch the installer.

MIMO installs for your current Windows user without requiring administrator privileges. Startup behavior can be customized from settings.

### Requirements

- Windows 10 or Windows 11
- Microsoft WebView2 Runtime

WebView2 is already included with Windows 11 and most up-to-date Windows 10 installations.

> The installer is currently unsigned, so Windows SmartScreen may display a warning when launching it.

## Privacy

Privacy is a core part of MIMO.

The application only connects to the internet to check for updates, a feature that can be disabled in settings.

Your notifications, clipboard history, notes, and saved passwords remain stored locally on your device.

## Build from source

Make sure Rust and the required development dependencies are installed, then run:

```bash
cd src-tauri
cargo build --release
```

The release build produces a standalone `mimo.exe`, approximately 5 MB in size.

The Windows installer is built using NSIS from:

```text
installer/island.nsi
```

GitHub Actions automatically builds the project, and pushing a version tag such as `v1.0.0` triggers the release workflow.

## Platform support

MIMO currently targets Windows.

A Linux port is planned. Since much of the backend relies on native Windows APIs for notifications, media sessions, clipboard access, and window behavior, bringing MIMO to Linux requires adapting these components.

## License

Released under the [MIT License](LICENSE).

Includes Tesseract.js and Persian language data licensed under Apache 2.0.

