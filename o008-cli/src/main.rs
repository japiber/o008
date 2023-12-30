use serde_json::Value;
use tracing::{error, info};
use o008_dispatcher::{cmd_dispatch_channel, DispatchCommand, CommandQueue, DispatcherError};
use o008_setting::{app_args, AppLogLevel, initialize_tracing};
use o008_common::{defer, ScopeCall};

#[tracing::instrument]
#[tokio::main]
async fn main() {
    tracing::subscriber::set_global_default(initialize_tracing()).expect("could not initialize tracing");

    info!("tracing level: {:?}", app_args().log.unwrap_or(AppLogLevel::Off));
    defer!(println!("application terminates"));

    command_dispatcher().await
}

async fn command_dispatcher() {
    if let Some(cmd) = &app_args().command {
        cmd_dispatch_channel().send(Box::new(DispatchCommand::from(cmd.clone())));
    }
    let print_publish = |v: &Value| { println!("publish {}", serde_json::to_string_pretty(v).unwrap()) };
    let log_error= |e: DispatcherError| { error!("{}", e.to_string()) };
    CommandQueue::poll(true, print_publish, log_error).await
}
