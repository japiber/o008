use serde_json::{json, to_value, Value};
use tracing::info;

use o008_common::{RequestValidator, ServiceRequest, DispatchResult};
use o008_entity::{Application, persist_json, QueryEntity, Service, ServiceVersion};

use o008_common::error::AppCommandError::{Create, InvalidRequest, NotFound, Update};
use o008_common::error::DispatcherError;

pub async fn persist(src: ServiceRequest, req: ServiceRequest) -> DispatchResult<Value> {
    if Service::persisted(to_value(src.clone()).unwrap()).await {
        update(src, req).await
    } else {
        let create_req = ServiceRequest::new(
            src.name(),
            src.application(),
            req.default_repo()
        );
        create(create_req).await
    }
}

pub async fn get(srq: ServiceRequest) -> DispatchResult<Value> {
    info!("get service {:?}", srq);
    match srq.is_valid_get() {
        Ok(()) => match Service::read(to_value(srq).unwrap()).await {
            Ok(srv) => Ok(to_value(*srv).unwrap()),
            Err(e) => Err(DispatcherError::from(NotFound(format!("get action: {}", e))))
        },
        Err(e) => Err(DispatcherError::from(InvalidRequest(format!("get action: {}", e))))
    }
}

pub async fn get_with_versions(srq: ServiceRequest) -> DispatchResult<Value> {
    info!("get service versions {:?}", srq);
    match srq.is_valid_get() {
        Ok(()) => match Service::read(to_value(srq).unwrap()).await {
            Ok(srv) => {
               let mut vsrv = srv.clone();
                if let Ok(versions) = ServiceVersion::service_versions(json!({"service": srv.id()})).await {
                    vsrv.set_versions(versions)
                }
                Ok(to_value(vsrv).unwrap())
            },
            Err(e) => Err(DispatcherError::from(NotFound(format!("get action: {}", e))))
        },
        Err(e) => Err(DispatcherError::from(InvalidRequest(format!("get action: {}", e))))
    }
}

async fn create(srq: ServiceRequest) -> DispatchResult<Value> {
    info!("create service {:?}", srq);
    match srq.is_valid_create() {
        Ok(()) => match Application::read(to_value(srq.application()).unwrap()).await {
            Ok(app) => {
                let srv = Service::new(srq.name().unwrap().as_str(), *app, srq.default_repo().unwrap().as_str());
                let r = persist_json(&srv).await;
                r.map_err(|e| DispatcherError::from(Create(format!("create action: {}", e))))
            }
            Err(e) => Err(DispatcherError::from(NotFound(format!("create action: {}", e))))
        },
        Err(e) => Err(DispatcherError::from(InvalidRequest(format!("create action: {}", e))))
    }
}

async fn update(src: ServiceRequest, req: ServiceRequest) -> DispatchResult<Value> {
    info!("update service from {:?} to {:?}", src, req);
    match (src.is_valid_get(), req.is_valid_update()) {
        (Ok(()), Ok(())) => match Service::read(to_value(src).unwrap()).await {
            Ok(mut srv) => match req.application() {
                None => {
                    srv.update(&req, None);
                    let r = persist_json(srv.as_ref()).await;
                    r.map_err(|e| DispatcherError::from(Update(format!("update action: {}", e))))
                }
                Some(arq) => match Application::read(to_value(arq).unwrap()).await {
                    Ok(app) => {
                        srv.update(&req, Some(*app));
                        let r = persist_json(srv.as_ref()).await;
                        r.map_err(|e| DispatcherError::from(Update(format!("update action: {}", e))))
                    }
                    Err(e) => Err(DispatcherError::from(NotFound(format!("update action: {}", e))))
                }
            },
            Err(e) => Err(DispatcherError::from(NotFound(format!("update action: {}", e))))
        },
        (Err(e), _) => Err(DispatcherError::from(InvalidRequest(format!("update action: {}", e)))),
        (_, Err(e)) => Err(DispatcherError::from(InvalidRequest(format!("update action: {}", e)))),
    }
}
