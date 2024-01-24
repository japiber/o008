use clap::Subcommand;
use crate::{ApplicationRequest, BuilderRequest, ServiceRequest, TenantRequest};
use crate::request::service_version::ServiceVersionRequest;

#[derive(Subcommand, Debug, Clone)]
pub enum AppCommand {
    CreateBuilder {
        #[arg(short, long)]
        request: BuilderRequest,
    },
    GetBuilder {
        #[arg(short, long)]
        request: BuilderRequest,
    },
    DeleteBuilder {
        #[arg(short, long)]
        request: BuilderRequest,
    },
    CreateTenant {
        #[arg(short, long)]
        request: TenantRequest,
    },
    GetTenant {
        #[arg(short, long)]
        request: TenantRequest,
    },
    CreateApplication {
        #[arg(short, long)]
        request: ApplicationRequest,
    },
    GetApplication {
        #[arg(short, long)]
        request: ApplicationRequest,
    },
    PersistService {
        #[arg(short, long)]
        source: ServiceRequest,
        request: ServiceRequest,
    },
    GetService {
        #[arg(short, long)]
        request: ServiceRequest,
    },
    GetServiceVersions {
        #[arg(short, long)]
        request: ServiceRequest,
    },
    PersistServiceVersion {
        #[arg(short, long)]
        source: ServiceVersionRequest,
        request: ServiceVersionRequest,
    }
}
