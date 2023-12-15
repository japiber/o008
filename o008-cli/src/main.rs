use tracing::{error, info};
use o008_dispatcher::AsyncDispatcher;
use o008_setting::{app_args, app_config, AppLogLevel, initialize_tracing};
use o008_common::{defer, ScopeCall};

#[tracing::instrument]
#[tokio::main]
async fn main() {
    tracing::subscriber::set_global_default(initialize_tracing()).expect("could not initialize tracing");
    info!("tracing level: {:?}", app_args().log.unwrap_or(AppLogLevel::Off));

    defer!(println!("end"));

    info!("deployment api {}", app_config().deployment_api().address());

    if let Some(cmd) = &app_args().command {
        match cmd.execute().await {
            Ok(v) => println!("{}", serde_json::to_string_pretty(&v).unwrap()),
            Err(e) => error!("{}", e)
        }
    }
}
