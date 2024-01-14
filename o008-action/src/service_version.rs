use serde_json::{to_value, Value};
use tracing::info;
use o008_common::{DispatcherError, DispatchResult, RequestValidator, ServiceVersionRequest};
use o008_common::AppCommandError::{Create, InvalidRequest, NotFound};
use o008_entity::{Builder, EntityError, persist_json, PersistEntity, QueryEntity, Service, ServiceVersion};
use o008_entity::pg::RepoReference;

pub async fn create(svr: &ServiceVersionRequest) -> DispatchResult<Value> {
    info!("create service version {:?}", svr);
    match svr.is_valid_create() {
        Ok(_) => match (Service::read(to_value(svr.service()).unwrap()).await,
                        Builder::read(to_value(svr.builder()).unwrap()).await) {
            (Ok(service), Ok(builder)) => create_service_version_with_repo_reference(&svr, service, builder).await,
            (Err(e), _) => Err(DispatcherError::from(NotFound(format!("create action service: {}", e)))),
            (_, Err(e)) => Err(DispatcherError::from(NotFound(format!("create action builder: {}", e)))),
        }
        Err(e) => Err(DispatcherError::from(InvalidRequest(format!("create action: {}", e))))
    }
}

async fn create_service_version_with_repo_reference(svr: &ServiceVersionRequest, service: Box<Service>, builder: Box<Builder>) -> DispatchResult<Value> {
    match RepoReference::read(to_value(svr.repo_ref()).unwrap()).await {
        Ok(rr) =>
            build_and_persist_service_version(
                svr.version().unwrap().as_str(),
                *service,
                *rr,
                *builder
            ).await,
        Err(EntityError::NotFound(_)) => {
            let rrq =  svr.repo_ref().unwrap();
            let rr: RepoReference = From::from(rrq.clone());
            match rr.persist().await {
                Ok(rr) =>
                    build_and_persist_service_version(
                        svr.version().unwrap().as_str(),
                        *service,
                        *rr,
                        *builder
                    ).await,
                Err(e) => Err(DispatcherError::from(Create(format!("create action: {}", e))))
            }
        },
        Err(e) => Err(DispatcherError::from(Create(format!("create action: {}", e))))
    }
}

async fn build_and_persist_service_version(version: &str, service: Service, rr: RepoReference, builder: Builder) -> DispatchResult<Value> {
    let service_version = ServiceVersion::new(
        version,
        service,
        rr,
        builder);
    let r = persist_json(Box::new(service_version)).await;
    r.map_err(|e| DispatcherError::from(Create(format!("create action: {}", e))))
}