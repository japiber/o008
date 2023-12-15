mod app_argument;
mod app_config;
mod app_log_level;
mod app_command;

use tracing_subscriber::fmt::format::{Compact, DefaultFields, Format};
use tracing_subscriber::fmt::Subscriber;
use once_cell::sync::OnceCell;
use tracing::level_filters::LevelFilter;
pub use app_log_level::AppLogLevel;
pub use app_argument::AppArgument;
pub use app_command::AppCommand;
pub use app_config::AppConfig;
pub use app_config::Database;


static ST_APP_CONFIG: OnceCell<AppConfig> = OnceCell::new();

static ST_APP_ARGS: OnceCell<AppArgument> = OnceCell::new();

pub fn app_args<'a>() -> &'a AppArgument {
    ST_APP_ARGS.get_or_init(|| AppArgument::new())
}

pub fn app_config<'a>() -> &'a AppConfig {
    ST_APP_CONFIG.get_or_init(|| AppConfig::new(app_args().get_config()).expect("could not load configuration file"))
}

pub fn initialize_tracing() -> Subscriber<DefaultFields, Format<Compact>> {
    let max = match app_args().log {
        None => LevelFilter::OFF,
        Some(l) => Into::<LevelFilter>::into(l)
    };

    // Start configuring a `fmt` subscriber
    tracing_subscriber::fmt()
        // Use a more compact, abbreviated log format
        .compact()
        // Display source code file paths
        .with_file(false)
        // Display source code line numbers
        .with_line_number(false)
        // Display the thread ID an event was recorded on
        .with_thread_ids(false)
        // Don't display the event's target (module path)
        .with_target(true)
        .with_max_level(max)
        // Build the subscriber
        .finish()
}