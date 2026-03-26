# ASSAS (POS Shadow Auditor)

ASSAS is a specialized, stealthy background application designed for owner-side auditing of Point-of-Sale (POS) systems. It monitors specific retail interface windows and captures visual evidence of critical operations (e.g., deletions) to ensure accounting integrity.

## 🚀 Key Features

- **Stealth Operation**: Runs as a background system process with no taskbar icon or console window.
- **Context-Aware Capture**: Automatically triggers screenshots when sensitive keywords (e.g., "Caisse", "Suppr") are detected in active window titles.
- **Secure Storage**: All captures are encrypted using **ChaCha20Poly1305** with random 12-byte nonces and persistent **Argon2** salted keys.
- **Hidden Vault Viewer**: Access captured data through a secure, hotkey-triggered UI (**`Ctrl+Alt+Shift+A`**).
- **Hardened for Windows**: Includes built-in support for Windows Defender exclusions and persistence via a professional PowerShell installer.

## 🛠️ Architecture

The project is built in Rust for performance and safety:
- **`rdev`**: Real-time keyboard hooking.
- **`xcap`**: Cross-platform screen capture.
- **`eframe/egui`**: Lightweight, immediate-mode GUI for the Vault.
- **`chacha20poly1305`**: Industry-standard authenticated encryption.

## 📦 Getting Started

For detailed instructions on building, installing, and using ASSAS, please refer to:

👉 **[Setup & Operational Guide (setup.md)](setup.md)**

## ⚠️ Disclaimer

This tool is intended for **authorized auditing and security purposes only**. The authors are not responsible for any misuse or illegal exploitation of this software. Always ensure compliance with local privacy laws and labor regulations.
