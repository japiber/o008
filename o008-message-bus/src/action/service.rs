use serde_json::{to_value, Value};
use tracing::info;

use o008_common::{RequestValidator, ServiceRequest, DispatchResult};
use o008_entity::{Application, persist_json, QueryEntity, Service};

use o008_common::error::AppCommandError::{Create, InvalidRequest, InvalidResponse, NotFound, Update};
use o008_common::error::DispatcherError;

pub async fn create(srq: ServiceRequest) -> DispatchResult<Value> {
    info!("create service {:?}", srq);
    match srq.is_valid_create() {
        Ok(()) => match Application::read(to_value(srq.application()).unwrap()).await {
            Ok(app) => {
                let srv = Service::new(srq.name().unwrap().as_str(), *app, srq.default_repo().unwrap().as_str());
                let r = persist_json(Box::new(srv)).await;
                r.map_err(|e| DispatcherError::from(Create(format!("create action: {}", e))))
            }
            Err(e) => Err(DispatcherError::from(NotFound(format!("create action: {}", e))))
        },
        Err(e) => Err(DispatcherError::from(InvalidRequest(format!("create action: {}", e))))
    }
}

pub async fn update(src: ServiceRequest, value: ServiceRequest) -> DispatchResult<Value> {
    info!("update service from {:?} to {:?}", src, value);
    match (src.is_valid_get(), value.is_valid_update()) {
        (Ok(()), Ok(())) => match Service::read(to_value(src).unwrap()).await {
            Ok(mut srv) => match value.application() {
                None => {
                    srv.update(&value, None);
                    let r = persist_json(srv).await;
                    r.map_err(|e| DispatcherError::from(Update(format!("update action: {}", e))))
                }
                Some(arq) => match Application::read(to_value(arq).unwrap()).await {
                    Ok(app) => {
                        srv.update(&value, Some(*app));
                        let r = persist_json(srv).await;
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



pub async fn get(srq: ServiceRequest) -> DispatchResult<Value> {
    info!("get service {:?}", srq);
    match srq.is_valid_get() {
        Ok(()) => match Service::read(to_value(srq).unwrap()).await {
            Ok(srv) => match to_value(*srv) {
                Ok(v) => Ok(v),
                Err(e) => Err(DispatcherError::from(InvalidResponse(format!("get action: {}", e))))
            },
            Err(e) => Err(DispatcherError::from(NotFound(format!("get action: {}", e))))
        },
        Err(e) => Err(DispatcherError::from(InvalidRequest(format!("get action: {}", e))))
    }
}
