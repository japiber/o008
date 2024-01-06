use tracing::{error, info};
use o008_dispatcher::{DispatchCommand, DispatchMessage};
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
        let msg = DispatchMessage::send(DispatchCommand::from(cmd.clone()));
        match msg.poll().await {
            Ok(v) => println!("publish {}", serde_json::to_string_pretty(&v).unwrap()),
            Err(e) => error!("{}", e.to_string())
        }
    }
}
