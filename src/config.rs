use std::{fs, io::ErrorKind::NotFound, path::PathBuf};

use directories::ProjectDirs;
use serde::Deserialize;
use thiserror::Error;

const fn bool_true() -> bool {
    true
}

#[derive(Debug, Deserialize)]
pub struct ConfigBranches {
    #[serde(default = "bool_true")]
    pub hololive: bool,

    #[serde(default = "bool_true")]
    pub holostars: bool,
}

impl Default for ConfigBranches {
    fn default() -> Self {
        Self {
            hololive: true,
            holostars: true,
        }
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct ConfigChannels {
    #[serde(default)]
    pub include: Vec<String>,

    #[serde(default)]
    pub exclude: Vec<String>,
}

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub branches: ConfigBranches,

    #[serde(default)]
    pub channels: ConfigChannels,
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Configuration file not found at {0:?}")]
    NotFound(PathBuf),
}

impl Config {
    pub fn new(path: &Option<PathBuf>) -> Result<Self, ConfigError> {
        let config_path = match path {
            Some(p) => {
                if !p.exists() {
                    return Err(ConfigError::NotFound(p.to_owned()));
                }
                p.to_owned()
            }
            None => {
                let proj_dirs =
                    ProjectDirs::from("", "", "Bibi").expect("Unable to get project dirs");
                proj_dirs.config_dir().join("bibi.toml")
            }
        };

        let config = match fs::read_to_string(config_path) {
            // TODO: Handle invalid file error
            Ok(cfg) => toml::from_str(&cfg).unwrap(),
            Err(e) if e.kind() == NotFound && path.is_none() => Config::default(),
            Err(e) => panic!("{}", e),
        };

        Ok(config)
    }
}
