use clap::ValueEnum;
use tracing::level_filters::LevelFilter;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum AppLogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Off,
}

impl From<AppLogLevel> for LevelFilter {
    fn from (v: AppLogLevel) -> Self {
        match v {
            AppLogLevel::Trace => LevelFilter::TRACE,
            AppLogLevel::Debug => LevelFilter::DEBUG,
            AppLogLevel::Info => LevelFilter::INFO,
            AppLogLevel::Warn => LevelFilter::WARN,
            AppLogLevel::Error => LevelFilter::ERROR,
            AppLogLevel::Off => LevelFilter::OFF,
        }
    }
}