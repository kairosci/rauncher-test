use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::{Error, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub install_dir: PathBuf,
    pub log_level: String,
}

impl Default for Config {
    fn default() -> Self {
        let project_dirs = ProjectDirs::from("", "", "r-games-launcher")
            .expect("Failed to determine project directories");

        Self {
            install_dir: project_dirs.data_dir().join("games"),
            log_level: "info".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.log_level, "info");
        assert!(config.install_dir.to_string_lossy().contains("games"));
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::default();
        let serialized = toml::to_string(&config).unwrap();
        let deserialized: Config = toml::from_str(&serialized).unwrap();
        assert_eq!(config.log_level, deserialized.log_level);
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;

        if config_path.exists() {
            let contents = fs::read_to_string(&config_path)?;
            Ok(toml::from_str(&contents)?)
        } else {
            let config = Self::default();
            config.save()?;
            Ok(config)
        }
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;

        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let contents = toml::to_string_pretty(self).map_err(|e| Error::Config(e.to_string()))?;
        fs::write(&config_path, contents)?;

        Ok(())
    }

    pub fn config_path() -> Result<PathBuf> {
        let project_dirs = ProjectDirs::from("", "", "r-games-launcher")
            .ok_or_else(|| Error::Config("Failed to determine project directories".to_string()))?;

        Ok(project_dirs.config_dir().join("config.toml"))
    }

    pub fn data_dir() -> Result<PathBuf> {
        let project_dirs = ProjectDirs::from("", "", "r-games-launcher")
            .ok_or_else(|| Error::Config("Failed to determine project directories".to_string()))?;

        Ok(project_dirs.data_dir().to_path_buf())
    }
}
