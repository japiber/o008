use tracing::{info};
use o008_dispatcher::{cmd_dispatch_channel, DispatchCommand, single_dispatcher};
use o008_setting::{app_args, app_config, AppLogLevel, initialize_tracing};
use o008_common::{defer, ScopeCall};

#[tracing::instrument]
#[tokio::main]
async fn main() {
    tracing::subscriber::set_global_default(initialize_tracing()).expect("could not initialize tracing");
    info!("tracing level: {:?}", app_args().log.unwrap_or(AppLogLevel::Off));

    defer!(println!("defer end"));

    info!("deployment api {}", app_config().deployment_api().address());

    if let Some(cmd) = &app_args().command {
        cmd_dispatch_channel().send(Box::new(DispatchCommand::from(cmd.clone())));
    }

    single_dispatcher().await;
}
