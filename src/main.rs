#![windows_subsystem = "windows"]

mod modules;

use anyhow::Result;
use modules::hook::{HookManager, Command};
use modules::capture::CaptureManager;
use modules::security::SecurityManager;
use modules::config::Config;
use modules::ui;
use std::sync::Arc;
use tokio::sync::mpsc;
use std::fs;

#[tokio::main]
async fn main() -> Result<()> {
    // 0. Singleton Check (simple file lock)
    let lock_path = std::env::temp_dir().join("assas.lock");
    let lock = fs::File::create(&lock_path)?;
    if let Err(_) = fs2::FileExt::try_lock_exclusive(&lock) {
        // App already running, exit silently
        return Ok(());
    }

    // 1. Charge la configuration persistante
    let mut config = Config::load().unwrap_or_default();
    
    // Redirect stdout/stderr to a log file in ProgramData (Windows) or tmp (Linux)
    let log_dir = if cfg!(windows) {
        std::path::PathBuf::from(r"C:\ProgramData\Assas")
    } else {
        std::env::temp_dir().join("assas")
    };
    std::fs::create_dir_all(&log_dir).ok();
    let log_file = log_dir.join("logs.txt");
    if let Ok(_file) = std::fs::OpenOptions::new().create(true).append(true).open(log_file) {
        // Note: For a real production app, use a logging crate like 'log' or 'tracing'
        // For simplicity here, we use a simple redirection if possible, 
        // or just acknowledge we should be silent.
    }

    // Si le sel n'existe pas, on le génère et on enregistre
    if config.salt.is_none() {
        config.salt = Some(SecurityManager::generate_random_salt());
        let _ = config.save();
    }
    let salt = config.salt.clone().unwrap();

    // 2. Setup des canaux de communication
    let (tx, mut rx) = mpsc::channel::<Command>(100);
    
    // 3. Démarre le crochet clavier et la surveillance des fenêtres
    let hook_manager = HookManager::new(config, tx);
    hook_manager.start();
    
    // 4. Gestionnaire de sécurité (Mot de passe par défaut pour le démon)
    let security = Arc::new(SecurityManager::new("default_password", &salt)
        .map_err(|e| anyhow::anyhow!(e))?);

    println!("Le démon ASSAS est lancé en arrière-plan.");

    // 5. Boucle d'événements principale
    while let Some(command) = rx.recv().await {
        match command {
            Command::Capture => {
                let security_clone = security.clone();
                tokio::spawn(async move {
                    // Attendre 500ms pour que l'interface POS s'actualise après la touche Suppr
                    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                    
                    match CaptureManager::capture_and_compress().await {
                        Ok(data) => {
                            if let Err(e) = security_clone.encrypt_and_save(&data) {
                                eprintln!("Échec de la sauvegarde de la capture : {:?}", e);
                            }
                        }
                        Err(e) => eprintln!("Erreur lors de la capture : {:?}", e),
                    }
                });
            }
            Command::ShowVault => {
                // Lance la visionneuse de coffre egui dans un thread séparé pour ne pas bloquer le démon
                std::thread::spawn(|| {
                    ui::run_vault_viewer();
                });
            }
        }
    }
    
    Ok(())
}
