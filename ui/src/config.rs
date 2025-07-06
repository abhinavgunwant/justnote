use std::fs::{ write, File };

use log::{ info, debug, error };
use serde::{ Serialize, Deserialize };
use serde_yaml::{ from_reader, to_string };
use vault::paths::get_local_dir;

use crate::{ startup::StartupConfig, logging::LoggingConfig };

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// Editor font
    pub font: String,

    pub startup: StartupConfig,

    pub log: LoggingConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            font: String::from("Inter"),
            startup: StartupConfig::default(),
            log: LoggingConfig::default(),
        }
    }
}

impl Config {
    fn get_config_path() -> Option<String> {
        match get_local_dir() {
            Some(mut path) => {
                path.push("config.yaml");

                if let Some(path_str) = path.to_str() {
                    return Some(path_str.to_owned());
                }

                error!("Error getting localdir path");
                None
            }

            None => {
                error!("Error getting localdir path");
                None
            }
        }
    }

    pub fn from_config_file() -> Self {
        if let Some(path) = Self::get_config_path() {
            match File::open(path) {
                Ok(file) => {
                    debug!("config file found");
                    match from_reader(file) {
                        Ok(config) => {
                            return config;
                        }

                        Err(e) => {
                            error!("Error when deserializing config: {}", e);
                        }
                    }
                }

                Err(e) => {
                    error!("Error when opening config file: {}", e);

                    match e.kind() {
                        std::io::ErrorKind::NotFound => {
                            info!("Create the default config file");
                            let config = Self::default();
                            config.to_config_file();
                            return config;
                        }

                        _ => {}
                    }
                }
            }
        }

        Self::default()
    }

    pub fn to_config_file(&self) {
        if let Some(path) = Self::get_config_path() {
            match to_string(self) {
                Ok(serialized) => {
                    if let Err(e) = write(path, serialized.as_bytes()) {
                        error!("Error when writing config: {}", e);
                    }
                }

                Err(e) => {
                    error!("Error when serializing config: {}", e);
                }
            }
        }
    }
}

