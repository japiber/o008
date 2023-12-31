use std::path::PathBuf;
use clap::{Parser};
use o008_common::AppCommand;
use crate::AppLogLevel;


const DEFAULT_CONFIG_FILE: &str = "./config.toml";



#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct AppArgument {
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[arg(short, long, value_name = "LOG_LEVEL")]
    pub log: Option<AppLogLevel>,

    #[command(subcommand)]
    pub command: Option<AppCommand>,
}

impl AppArgument {
    pub fn get_config(&self) -> String {
        match &self.config {
            None => String::from(DEFAULT_CONFIG_FILE),
            Some(pb) => String::from(pb.to_str().expect("specified file is not valid"))
        }
    }
}

impl Default for AppArgument {
    fn default() -> Self {
        AppArgument::parse()
    }
}
