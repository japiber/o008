use tracing::{info};
use o008_dispatcher::{cmd_dispatch_channel, DispatchCommand, CommandQueue};
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

    CommandQueue::poll(true).await
}
