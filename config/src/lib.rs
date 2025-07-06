//! This crate contains the code to handle configuration files and application
//! logging.

use std::fs::{ write, File };

use log4rs::{
    append::rolling_file::{
        policy::compound::{
            roll::fixed_window::FixedWindowRoller,
            trigger::size::SizeTrigger, CompoundPolicy
        }, RollingFileAppender
    },
    config::{ Appender, Config as LogConfig, Logger, Root },
    init_config, encode::pattern::PatternEncoder,
};

use log::{ info, debug, error, LevelFilter };
use serde::{ Serialize, Deserialize };
use serde_yaml::{ from_reader, to_string };
use dirs_next::data_local_dir;

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
        match data_local_dir() {
            Some(mut path) => {
                path.push("justnote");
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

/// Startup config struct
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct StartupConfig {
    #[serde(
        skip_serializing_if = "String::is_empty",
        default = "String::default"
    )]
    pub default_vault: String,

    /// Minimize explorer on startup
    pub hide_explorer: bool,
}

/// Logging config struct
#[derive(Debug, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub size_limit: u64,
    pub pattern: String,
    pub level: String,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            size_limit: 1_048_576,
            pattern: String::from(
                "{d(%Y-%m-%d %H:%M:%S%.3f)} {h({({l}):1.1})} {f:>32.128}:{L:4.7} - {m}{n}"
            ),
            level: String::from("Info"),
        }
    }
}

impl LoggingConfig {
    pub fn level(&self) -> LevelFilter {
        match self.level.to_uppercase().as_str() {
            "INFO" => LevelFilter::Info,
            "DEBUG" => LevelFilter::Debug,
            "ERROR" => LevelFilter::Error,
            "WARN" => LevelFilter::Warn,
            _ => LevelFilter::Info,
        }
    }
}

/// The local dir of justnotes
fn dir_str() -> String {
    if let Some(mut path) = data_local_dir() {
        path.push("justnote");
        return path.to_str().unwrap().to_owned();
    }

    String::default()
}

fn log_path() -> String { format!("{}/logs/justnote.log", dir_str()) }

fn archive_file_pattern() -> String {
    let mut pattern = dir_str();
    pattern.push_str("/logs/archives/be_{}.gz");

    pattern
}

pub fn init_logging() {
    let config = Config::from_config_file();

    let fixed_window_roller = FixedWindowRoller::builder()
        .base(1)
        .build(&archive_file_pattern(), 21)
        .unwrap();

    let rolling_policy = CompoundPolicy::new(
        Box::new(SizeTrigger::new(config.log.size_limit)),
        Box::new(fixed_window_roller),
    );

    let main_appender = RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(&config.log.pattern)))
        .build(log_path(), Box::new(rolling_policy))
        .unwrap();

    let log_config = LogConfig::builder()
        .appender(Appender::builder().build("main", Box::new(main_appender)))
        .logger(
            Logger::builder()
                .appender("main")
                .build("main", config.log.level())
        )
        .build(Root::builder().appender("main").build(config.log.level()))
        .unwrap();

    let _ = init_config(log_config).unwrap();
}

