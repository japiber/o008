use tracing::{error, info};
use o008_common::{defer, ScopeCall, DispatchCommand, InternalCommand};
use o008_setting::{app_args, AppLogLevel, initialize_tracing};
use o008_message_bus::{launch_request_poll, launch_response_poll, RequestMessage, send_request};

#[tracing::instrument]
#[tokio::main]
async fn main() {
    tracing::subscriber::set_global_default(initialize_tracing()).expect("could not initialize tracing");

    info!("tracing level: {:?}", app_args().log.unwrap_or(AppLogLevel::Off));
    defer!(println!("application terminates"));

    let tr = launch_request_poll();

    command_dispatcher().await;

    tr.await.unwrap()
}

async fn command_dispatcher() {
    if let Some(cmd) = &app_args().command {
        let msg = RequestMessage::new(DispatchCommand::from(cmd.clone()));
        let tr = launch_response_poll(msg.id());
        if send_request(msg) {
            match tr.await.unwrap() {
                None => println!("could not get response for command: {:?}", cmd),
                Some(msg) => match msg.response() {
                    Ok(v) => println!(">> {}", serde_json::to_string_pretty(&v).unwrap()),
                    Err(e) => error!("{}", e.to_string())
                }
            }
        }
        send_request(RequestMessage::new(DispatchCommand::from(InternalCommand::Quit)));
    }
}
