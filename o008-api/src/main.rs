use tracing::info;
use o008_setting::{app_args, app_config, AppLogLevel, initialize_tracing};
use crate::router::router_o008_v1;


mod handler;
mod router;


#[tokio::main]
async fn main() {
    tracing::subscriber::set_global_default(initialize_tracing()).expect("could not initialize tracing");
    info!("tracing level: {:?}", app_args().log.unwrap_or(AppLogLevel::Off));

    let app = router_o008_v1();

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(app_config().deployment_api().address()).await.unwrap();
    info!("listening on: {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
