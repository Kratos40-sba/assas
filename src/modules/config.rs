use serde::{Serialize, Deserialize};
use std::fs;
use std::path::PathBuf;
use anyhow::{Result, anyhow};
use directories::ProjectDirs;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub target_titles: Vec<String>,
    pub trigger_key: String,
    pub salt: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            target_titles: vec![
                "Caisse".to_string(),
                "BatiPOS".to_string(),
                "POS".to_string()
            ],
            trigger_key: "Suppr".to_string(),
            salt: None,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let path = Self::get_config_path()?;
        if !path.exists() {
            let default_config = Self::default();
            default_config.save()?;
            return Ok(default_config);
        }

        let content = fs::read_to_string(path)?;
        let config = serde_json::from_str(&content)?;
        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::get_config_path()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let _content = serde_json::from_str::<serde_json::Value>(&serde_json::to_string(self)?)?.to_string(); // Pretty print or just stringify
        let pretty_content = serde_json::to_string_pretty(self)?;
        fs::write(path, pretty_content)?;
        Ok(())
    }

    fn get_config_path() -> Result<PathBuf> {
        if let Some(proj_dirs) = ProjectDirs::from("com", "assas", "auditor") {
            Ok(proj_dirs.config_dir().join("config.json"))
        } else {
            Err(anyhow!("Could not find config directory"))
        }
    }
}
