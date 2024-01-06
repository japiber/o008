use serde_json::{to_value, Value};

use o008_common::{RequestValidator, ServiceRequest};
use o008_entity::{Application, persist_json, QueryEntity, Service};

use crate::{DispatcherError, DispatchResult};
use crate::AppCommandError::{Create, InvalidRequest, InvalidResponse, NotFound, Update};

pub async fn create(srq: &ServiceRequest) -> DispatchResult<Value> {
    match srq.is_valid_create() {
        Ok(()) => match Application::read(to_value(srq).unwrap()).await {
            Ok(app) => {
                let srv = Service::new(srq.name().unwrap().as_str(), *app, srq.default_repo().unwrap().as_str());
                let r = persist_json(Box::new(srv)).await;
                r.map_err(|e| DispatcherError::from(Create(e.to_string())))
            }
            Err(e) => Err(DispatcherError::from(NotFound(e.to_string())))
        },
        Err(e) => Err(DispatcherError::from(InvalidRequest(e.to_string())))
    }
}

pub async fn update(src: &ServiceRequest, value: &ServiceRequest) -> DispatchResult<Value> {
    match (src.is_valid_get(), value.is_valid_update()) {
        (Ok(()), Ok(())) => match Service::read(to_value(src).unwrap()).await {
            Ok(mut srv) => match value.application() {
                None => {
                    srv.update(value, None);
                    let r = persist_json(srv).await;
                    r.map_err(|e| DispatcherError::from(Update(e.to_string())))
                }
                Some(arq) => match Application::read(to_value(arq).unwrap()).await {
                    Ok(app) => {
                        srv.update(value, Some(*app));
                        let r = persist_json(srv).await;
                        r.map_err(|e| DispatcherError::from(Update(e.to_string())))
                    }
                    Err(e) => Err(DispatcherError::from(NotFound(e.to_string())))
                }
            },
            Err(e) => Err(DispatcherError::from(NotFound(e.to_string())))
        },
        (Err(e), _) => Err(DispatcherError::from(InvalidRequest(e.to_string()))),
        (_, Err(e)) => Err(DispatcherError::from(InvalidRequest(e.to_string()))),
    }
}



pub async fn get(srq: &ServiceRequest) -> DispatchResult<Value> {
    match srq.is_valid_get() {
        Ok(()) => match Service::read(to_value(srq).unwrap()).await {
            Ok(srv) => match to_value(*srv) {
                Ok(v) => Ok(v),
                Err(e) => Err(DispatcherError::from(InvalidResponse(e.to_string())))
            },
            Err(e) => Err(DispatcherError::from(NotFound(e.to_string())))
        },
        Err(e) => Err(DispatcherError::from(InvalidRequest(e.to_string())))
    }
}
