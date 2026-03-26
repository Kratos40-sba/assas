# ASSAS (عساس) — POS Shadow Auditor
> **Status:** Architecture v1.0 (MVP Spec)
> **Language:** Rust (Edition 2021)
> **Target OS:** Windows 7 / Windows 10 (x86_64)
> **Hardware Constraint:** Intel Celeron, <2GB RAM. Must run at <1% CPU in the background.

## 1. Executive Summary
ASSAS is an ultra-lightweight, invisible background daemon designed for the Algerian retail market. Its sole purpose is to detect internal cashier theft (voiding items after receiving cash). It monitors specific POS window activities, intercepts targeted keystrokes (e.g., "Delete" or "F12"), takes a silent screenshot, and stores it in an encrypted local vault.

## 2. Core Tech Stack (Recommended Crates)
* **GUI / Vault Viewer:** `eframe` / `egui` (Immediate mode GUI, compiles to a tiny binary, zero web-view overhead).
* **OS Hooks (Win32):** `rdev` (for global keyboard hooks) OR `windows` crate for direct Win32 API calls (checking foreground window title).
* **Screen Capture:** `xcap` or `scrap` (fast, native buffer capture).
* **Compression:** `image` crate (Convert raw bitmaps to highly compressed WebP or JPEG to save disk space).
* **Cryptography:** `chacha20poly1305` (Fast, software-optimized symmetric encryption) + `argon2` (Password hashing).

## 3. System Architecture & Flow
1. **The Stealth Daemon:** - Binary compiled with `#![windows_subsystem = "windows"]` to hide the console.
   - Listens globally for the "Trigger Key" (configurable, e.g., `Suppr`).
2. **Context Verification:** - Before capturing, the app checks if the currently active window title matches the target POS software (e.g., contains "Caisse" or "BatiPOS"). 
   - *Logic:* If the cashier presses 'Delete' while browsing YouTube, ignore it.
3. **Capture & Encrypt Pipeline:**
   - Wait `500ms` (allows the POS UI to show the "Item Deleted" popup).
   - Capture screen $\rightarrow$ Compress to WebP $\rightarrow$ Encrypt byte array in memory.
   - Save to `C:\ProgramData\AssasLogs\{timestamp}.aes`.
4. **The Owner's Vault:**
   - Triggered via a secret global hotkey (e.g., `Ctrl+Alt+Shift+A`).
   - Prompts for the Master Password.
   - Decrypts images into RAM (never touching disk as raw images) and displays them in a simple `egui` gallery sorted by date.

## 4. Critical Engineering Notes
* **Antivirus Evasion / False Positives:** Because this app hooks the keyboard and takes screenshots, Windows Defender *will* likely flag it as spyware/malware. 
* **Requirement:** The installer must automatically add an exclusion path to Windows Defender, or the binary must be digitally signed.
* **Offline First:** This app must require ZERO internet connection to function.