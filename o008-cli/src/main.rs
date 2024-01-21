use tracing::{error, info};
use o008_business::dispatcher;
use o008_common::{defer, ScopeCall, DispatchCommand};
use o008_setting::{app_args, AppLogLevel, initialize_tracing};
use o008_message_bus::{RequestMessage};
use o008_message_bus::helper::bus_processor;

#[tracing::instrument]
#[tokio::main]
async fn main() {
    tracing::subscriber::set_global_default(initialize_tracing()).expect("could not initialize tracing");
    info!("tracing level: {:?}", app_args().log.unwrap_or(AppLogLevel::Off));
    defer!(println!("Agur!!"));

    command_dispatcher().await
}

async fn command_dispatcher() {
    if let Some(cmd) = &app_args().command {
        let msg = RequestMessage::new(DispatchCommand::from(cmd.clone()));
        match bus_processor(msg.clone(), dispatcher::RequestMessageCommand::from(msg)).await {
            None => println!("could not get response for command: {:?}", cmd),
            Some(result) => match result {
                Ok(v) => println!(">> {}", serde_json::to_string_pretty(&v).unwrap()),
                Err(e) => error!("{}", e.to_string())
            }
        }
    }
}
