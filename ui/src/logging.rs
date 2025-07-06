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

use serde::{ Serialize, Deserialize };

use log::LevelFilter;
use vault::paths::get_local_dir;

use crate::config::Config;

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
    let dir = get_local_dir().unwrap();

    dir.to_str().unwrap().to_owned()
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

