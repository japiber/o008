use serde_json::{to_value, Value};
use tracing::info;
use o008_common::{DispatcherError, DispatchResult, RequestValidator, ServiceVersionRequest};
use o008_common::AppCommandError::{Create, InvalidRequest, NotFound, Update};
use o008_entity::{Builder, EntityError, persist_json, PersistEntity, QueryEntity, Service, ServiceVersion};
use o008_entity::pg::RepoReference;


pub async fn persist(src: ServiceVersionRequest, req: ServiceVersionRequest) -> DispatchResult<Value> {
    if ServiceVersion::persisted(to_value(&src).unwrap()).await {
        update(src, req).await
    } else {
        let create_req = ServiceVersionRequest::new(
            src.version(),
            src.service(),
            req.repo_ref(),
            req.builder()
        );
        create(create_req).await
    }
}

async fn create(svr: ServiceVersionRequest) -> DispatchResult<Value> {
    info!("create service version {:?}", svr);
    match svr.is_valid_create() {
        Ok(_) => match (Service::read(to_value(svr.service()).unwrap()).await,
                        Builder::read(to_value(svr.builder()).unwrap()).await) {
            (Ok(service), Ok(builder)) => create_service_version_with_repo_reference(svr, service, builder).await,
            (Err(e), _) => Err(DispatcherError::from(NotFound(format!("create action service: {}", e)))),
            (_, Err(e)) => Err(DispatcherError::from(NotFound(format!("create action builder: {}", e)))),
        }
        Err(e) => Err(DispatcherError::from(InvalidRequest(format!("create action: {}", e))))
    }
}

async fn update(src: ServiceVersionRequest, svr: ServiceVersionRequest) -> DispatchResult<Value> {
    info!("create service version {:?}", svr);
    match (src.is_valid_get(), svr.is_valid_update()) {
        (Ok(()), Ok(())) => match ServiceVersion::read(to_value(src).unwrap()).await {
            Ok(mut sv) => update_service_version(sv.as_mut(), svr).await,
            Err(e) =>  Err(DispatcherError::from(NotFound(format!("update action: {}", e))))
        }
        (Err(e), _) => Err(DispatcherError::from(InvalidRequest(format!("update action: {}", e)))),
        (_, Err(e)) => Err(DispatcherError::from(InvalidRequest(format!("update action: {}", e)))),
    }
}

async fn update_service_version(sv: &mut ServiceVersion, req: ServiceVersionRequest) -> DispatchResult<Value> {
    let (service, repo_ref, builder) = get_updated_entities(&req).await;
    if let Some(rs) = service {
        match rs {
            Ok(s) => sv.set_service(*s),
            Err(e) => return Err(DispatcherError::from(NotFound(format!("update action: {}", e))))
        }
    }

    if let Some(rr) = repo_ref {
        match rr {
            Ok(r) => sv.set_repo_ref(*r),
            Err(e) => return Err(DispatcherError::from(NotFound(format!("update action: {}", e))))
        }
    }

    if let Some(rb) = builder {
        match rb {
            Ok(b) => sv.set_builder(*b),
            Err(e) => return Err(DispatcherError::from(NotFound(format!("update action: {}", e))))
        }
    }

    if let Some(v) = req.version() {
        sv.set_version(v.as_str())
    }

    let r = persist_json(sv).await;
    r.map_err(|e| DispatcherError::from(Update(format!("update action: {}", e))))
}

async fn get_updated_entities(req: &ServiceVersionRequest) -> (Option<Result<Box<Service>, EntityError>>,
                                                              Option<Result<Box<RepoReference>, EntityError>>,
                                                              Option<Result<Box<Builder>, EntityError>>) {
    let srv = if let Some(s) = req.service() {
        Some(Service::read(to_value(s).unwrap()).await)
    } else {
        None
    };

    let rrf = if let Some(r) = req.repo_ref() {
        Some(RepoReference::read(to_value(r).unwrap()).await)
    } else {
        None
    };

    let bld = if let Some(b) = req.builder() {
        Some(Builder::read(to_value(b).unwrap()).await)
    } else {
        None
    };

    (srv, rrf, bld)
}

async fn create_service_version_with_repo_reference(svr: ServiceVersionRequest, service: Box<Service>, builder: Box<Builder>) -> DispatchResult<Value> {
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
    let r = persist_json(&service_version).await;
    r.map_err(|e| DispatcherError::from(Create(format!("create action: {}", e))))
}