use log4rs::{
    append::rolling_file::{
        policy::compound::{
            roll::fixed_window::FixedWindowRoller,
            trigger::size::SizeTrigger, CompoundPolicy
        }, RollingFileAppender
    },
    config::{ Appender, Config, Logger, Root },
    init_config, encode::pattern::PatternEncoder,
};

use log::LevelFilter;
use vault::paths::get_local_dir;

const LOG_SIZE_LIMIT: u64 = 1_048_576;
const LOG_PATTERN: &str = "{d(%Y-%m-%d %H:%M:%S%.3f)} {h({({l}):1.1})} {f:>32.128}:{L:4.7} - {m}{n}";
const LOG_LEVEL: LevelFilter = LevelFilter::Info;

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
    let fixed_window_roller = FixedWindowRoller::builder()
        .base(1)
        .build(&archive_file_pattern(), 21)
        .unwrap();

    let rolling_policy = CompoundPolicy::new(
        Box::new(SizeTrigger::new(LOG_SIZE_LIMIT)),
        Box::new(fixed_window_roller),
    );

    let main_appender = RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(LOG_PATTERN)))
        .build(log_path(), Box::new(rolling_policy))
        .unwrap();


    let log_config = Config::builder()
        .appender(Appender::builder().build("main", Box::new(main_appender)))
        .logger(
            Logger::builder()
                .appender("main")
                .build("main", LOG_LEVEL)
        )
        .build(Root::builder().appender("main").build(LOG_LEVEL))
        .unwrap();

    let _ = init_config(log_config).unwrap();
}

