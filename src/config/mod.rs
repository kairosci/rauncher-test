use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::{Error, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub install_dir: PathBuf,
    pub log_level: String,
    // Additional configuration options
    #[serde(default = "default_download_threads")]
    pub download_threads: u32,
    #[serde(default)]
    pub bandwidth_limit: Option<u64>, // bytes per second
    #[serde(default)]
    pub cdn_region: Option<String>,
    #[serde(default)]
    pub auto_update: bool,
    #[serde(default)]
    pub cache_size: u64, // bytes, 0 = unlimited
    #[serde(default = "default_config_version")]
    pub config_version: u32,
}

fn default_download_threads() -> u32 {
    4
}

fn default_config_version() -> u32 {
    1
}

impl Default for Config {
    fn default() -> Self {
        let project_dirs = ProjectDirs::from("", "", "rauncher")
            .expect("Failed to determine project directories");

        Self {
            install_dir: project_dirs.data_dir().join("games"),
            log_level: "info".to_string(),
            download_threads: default_download_threads(),
            bandwidth_limit: None,
            cdn_region: None,
            auto_update: false,
            cache_size: 100 * 1024 * 1024, // 100 MB default cache
            config_version: default_config_version(),
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;

        if config_path.exists() {
            let contents = fs::read_to_string(&config_path)?;
            
            // Try to parse config
            match toml::from_str::<Config>(&contents) {
                Ok(mut config) => {
                    // Check if migration is needed
                    if config.config_version < default_config_version() {
                        log::info!(
                            "Migrating config from version {} to {}",
                            config.config_version,
                            default_config_version()
                        );
                        config = Self::migrate_config(config)?;
                        config.save()?;
                    }
                    config.validate()?;
                    Ok(config)
                }
                Err(e) => {
                    log::warn!("Failed to parse config: {}. Using defaults and merging.", e);
                    // Merge with defaults for missing values
                    let default = Self::default();
                    let mut config = default;
                    
                    // Try to parse as a toml::Value to extract what we can
                    if let Ok(value) = toml::from_str::<toml::Value>(&contents) {
                        if let Some(table) = value.as_table() {
                            // Merge known fields
                            if let Some(install_dir) = table.get("install_dir") {
                                if let Some(dir_str) = install_dir.as_str() {
                                    config.install_dir = PathBuf::from(dir_str);
                                }
                            }
                            if let Some(log_level) = table.get("log_level") {
                                if let Some(level_str) = log_level.as_str() {
                                    config.log_level = level_str.to_string();
                                }
                            }
                        }
                    }
                    
                    config.validate()?;
                    config.save()?;
                    Ok(config)
                }
            }
        } else {
            let config = Self::default();
            config.save()?;
            Ok(config)
        }
    }
    
    /// Migrate config from older versions
    fn migrate_config(mut config: Config) -> Result<Self> {
        let current_version = config.config_version;
        let target_version = default_config_version();
        
        log::info!("Migrating config from v{} to v{}", current_version, target_version);
        
        // Apply migrations sequentially
        if current_version < 1 {
            // Migration to v1: Add new fields with defaults
            let defaults = Self::default();
            if config.download_threads == 0 {
                config.download_threads = defaults.download_threads;
            }
            if config.cache_size == 0 {
                config.cache_size = defaults.cache_size;
            }
        }
        
        // Update version
        config.config_version = target_version;
        
        Ok(config)
    }

    /// Validate configuration values
    fn validate(&self) -> Result<()> {
        // Validate log level
        let valid_log_levels = ["trace", "debug", "info", "warn", "error"];
        if !valid_log_levels.contains(&self.log_level.as_str()) {
            return Err(Error::Config(format!(
                "Invalid log level: '{}'. Must be one of: {}",
                self.log_level,
                valid_log_levels.join(", ")
            )));
        }

        // Validate install directory - ensure parent exists or can be created
        if let Some(parent) = self.install_dir.parent() {
            if !parent.exists() {
                return Err(Error::Config(format!(
                    "Install directory parent does not exist: {}",
                    parent.display()
                )));
            }
        }
        
        // Validate download threads (1-64 reasonable range)
        if self.download_threads == 0 || self.download_threads > 64 {
            return Err(Error::Config(format!(
                "Invalid download_threads: {}. Must be between 1 and 64",
                self.download_threads
            )));
        }
        
        // Validate bandwidth limit if set
        if let Some(limit) = self.bandwidth_limit {
            if limit < 1024 {
                return Err(Error::Config(
                    "Bandwidth limit must be at least 1024 bytes/second (1 KB/s)".to_string()
                ));
            }
        }

        Ok(())
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
        let project_dirs = ProjectDirs::from("", "", "rauncher")
            .ok_or_else(|| Error::Config("Failed to determine project directories".to_string()))?;

        Ok(project_dirs.config_dir().join("config.toml"))
    }

    pub fn data_dir() -> Result<PathBuf> {
        let project_dirs = ProjectDirs::from("", "", "rauncher")
            .ok_or_else(|| Error::Config("Failed to determine project directories".to_string()))?;

        Ok(project_dirs.data_dir().to_path_buf())
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
