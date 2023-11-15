use tracing::level_filters::LevelFilter;
use tracing::{error, info};
use o008_setting::{app_args, app_config, AppArgument, AppCommand, initialize_tracing};
use o008_entity::{Builder, Entity};
use o008_entity::entity::Tenant;


#[tracing::instrument]
#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let args = app_args();
    tracing::subscriber::set_global_default(
        match args.log {
            None => initialize_tracing(LevelFilter::OFF),
            Some(l) => initialize_tracing(Into::<LevelFilter>::into(l))
        }).expect("TODO: panic message");
    info!("tracing level {:?}", args.log);

    println!("dn uri {}", app_config().database().uri());

    do_command(&args).await;
}


#[tracing::instrument]
async fn do_command(args: &AppArgument) {

   if let Some(cmd) = args.command.clone() {
       match cmd {
           AppCommand::CreateBuilder { name, active, cmd } => {
               let mut builder = Builder::new(&name, active, &cmd);
               match builder.persist().await {
                   Ok(_) => info!("builder {:?} created successfully", builder.inner()),
                   Err(_) => error!("could not create builder {:?}", builder.inner())
               }
           },
           AppCommand::GetBuilder { name} => {
               if let Some(b) = Builder::search_name(&name).await {
                  println!("'{}' builder found: {:?}", &name, b.inner())
               } else {
                   error!("'{}' builder not found", &name)
               }
           },
           AppCommand::DeleteBuilder { name } => {
               if let Some(mut b) = Builder::search_name(&name).await {
                   println!("'{}' builder found: {:?}", &name, b.inner());
                   match b.destroy().await {
                       Ok(_) => println!("builder '{}' has benn destroyed", b.name()),
                       Err(e) => println!("could not destroy builder: {}", e)
                   }
               } else {
                   error!("'{}' builder not found", &name)
               }
           },
           AppCommand::CreateTenant { name, coexisting} => {
               let mut t = Tenant::new(&name, coexisting);
               match t.persist().await {
                   Ok(_) => info!("tenant {:?} created successfully", t.inner()),
                   Err(_) => error!("could not create tenant {:?}", t.inner())
               }
           }
       }
   }



}
