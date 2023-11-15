use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use tracing::level_filters::LevelFilter;
use crate::AppCommand;

const DEFAULT_CONFIG_FILE: &'static str = "./config.toml";

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Off,
}

impl From<LogLevel> for LevelFilter {
    fn from (v: LogLevel) -> Self {
        match v {
            LogLevel::Trace => LevelFilter::TRACE,
            LogLevel::Debug => LevelFilter::DEBUG,
            LogLevel::Info => LevelFilter::INFO,
            LogLevel::Warn => LevelFilter::WARN,
            LogLevel::Error => LevelFilter::ERROR,
            LogLevel::Off => LevelFilter::OFF,
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct AppArgument {
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[arg(short, long, value_name = "LOG_LEVEL")]
    pub log: Option<LogLevel>,

    #[command(subcommand)]
    pub command: Option<AppCommand>,
}

impl AppArgument {

    pub fn new() -> Self {
        AppArgument::parse()
    }

    pub fn get_config(&self) -> String {
        match self.config.clone() {
            None => String::from(DEFAULT_CONFIG_FILE),
            Some(pb) => String::from(pb.to_str().expect("specified file is not valid"))
        }
    }
}
