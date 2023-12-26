use clap::Subcommand;
use crate::{ApplicationRequest, BuilderRequest, TenantRequest};

#[derive(Subcommand, Debug, Clone)]
pub enum AppCommand {
    CreateBuilder {
        #[arg(short, long)]
        value: BuilderRequest,
    },
    GetBuilder {
        #[arg(short, long)]
        value: BuilderRequest,
    },
    DeleteBuilder {
        #[arg(short, long)]
        value: BuilderRequest,
    },
    CreateTenant {
        #[arg(short, long)]
        value: TenantRequest,
    },
    GetTenant {
        #[arg(short, long)]
        value: TenantRequest,
    },
    CreateApplication {
        #[arg(short, long)]
        value: ApplicationRequest,
    },
    GetApplication {
        #[arg(short, long)]
        value: ApplicationRequest,
    }
}
