# ASSAS Setup & Operational Guide

This document explains how to build, install, and use the ASSAS (POS Shadow Auditor) on a Windows system.

## 1. Prerequisites (For Building)
If you are building from source on Linux:
- Rust toolchain (`rustup`)
- Mingw-w64 toolchain: `sudo apt-get install mingw-w64`
- Zip utility: `sudo apt-get install zip`

## 2. Building the Windows Package
Run the provided build script on your Linux machine:
```bash
./scripts/build_windows.sh
```
This will generate `target/x86_64-pc-windows-gnu/assas-windows.zip`.

## 3. Installation on Target Windows Machine
1.  **Extract**: Transfer the `assas-windows.zip` to the target machine and extract it.
2.  **Run Installer**: Right-click `install.ps1` and select **"Run with PowerShell"**.
    - *Note*: If prompted by UAC, grant Administrative privileges.
    - *Note*: If execution policy blocks the script, run `Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser` in a PowerShell terminal first.
3.  **Verification**: The installer will:
    - Create `C:\Program Files\Assas` and copy the exe there.
    - Create `C:\ProgramData\Assas` for logs and captured data.
    - Add Windows Defender exclusions for both folders.
    - Set the app to run automatically on startup.

## 4. Usage
- **Stealth Mode**: The application runs silently in the background with no icon or window.
- **Trigger**: It automatically captures screenshots when "Caisse" windows are detected and "Suppr" is pressed.
- **Vault Access**: Press **`Ctrl+Alt+Shift+A`** at any time to open the Vault viewer.
- **Password**: The default password for this version is `default_password`.

## 5. Troubleshooting
- **Not Starting?**: Check `C:\ProgramData\Assas\logs.txt` for errors.
- **Flagged by Antivirus?**: Ensure the exclusions were added. If not, add `C:\Program Files\Assas` and `C:\ProgramData\Assas` manually to Windows Security -> Virus & threat protection -> Manage settings -> Exclusions.
