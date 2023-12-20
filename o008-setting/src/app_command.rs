use clap::Subcommand;

#[derive(Subcommand, Debug, Clone)]
pub enum AppCommand {
    CreateBuilder {
        #[arg(short, long)]
        name: String,
        #[arg(short, long, default_value = "false")]
        active: bool,
        #[arg(short, long)]
        cmd: String,
    },
    GetBuilder {
        #[arg(short, long)]
        name: String,
    },
    DeleteBuilder {
        #[arg(short, long)]
        name: String,
    },
    CreateTenant {
        #[arg(short, long)]
        name: String,
        #[arg(long, default_value = "false")]
        coexisting: bool,
    },
    CreateApplication {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        tenant: String,
        #[arg(short, long)]
        class_unit: String,
    },
    GetApplication {
        #[arg(short, long)]
        name: String,
    }
}
