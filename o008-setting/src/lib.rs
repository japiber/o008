use tracing_subscriber::fmt::format::{Compact, DefaultFields, Format};
use tracing_subscriber::fmt::Subscriber;
use once_cell::sync::OnceCell;
pub use app_argument::AppArgument;
pub use app_command::AppCommand;
pub use app_argument::LogLevel;
pub use app_config::AppConfig;
pub use app_config::Database;

mod app_argument;
mod app_config;
mod app_command;


static ST_APP_ARGS: OnceCell<AppArgument> = OnceCell::new();
static ST_APP_CONFIG: OnceCell<AppConfig> = OnceCell::new();

pub fn app_args<'a>() -> &'a AppArgument {
    ST_APP_ARGS.get_or_init(|| AppArgument::new())
}

pub fn app_config<'a>() -> &'a AppConfig {
    ST_APP_CONFIG.get_or_init(|| AppConfig::new(app_args().get_config()).unwrap())
}

pub fn initialize_tracing(max: impl Into<tracing::level_filters::LevelFilter>) -> Subscriber<DefaultFields, Format<Compact>> {
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