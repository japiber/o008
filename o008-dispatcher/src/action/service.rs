use serde_json::{to_value, Value};
use o008_common::{RequestValidator, ServiceRequest};
use o008_entity::{Application, persist_json, QueryEntity, Service};
use crate::{DispatcherError, DispatchResult};
use crate::AppCommandError::{Create, InvalidRequest, InvalidResponse, NotFound};

pub async fn create(srq: &ServiceRequest) -> DispatchResult<Value> {
    if srq.is_valid_create() {
        match Application::read(to_value(srq).unwrap()).await {
            Ok(app) => {
                let srv = Service::new(srq.name(), *app, srq.default_repo());
                let r = persist_json(Box::new(srv)).await;
                r.map_err(|e| DispatcherError::from(Create(format!("{}", e))))
            },
            Err(e) => Err(DispatcherError::from(NotFound(e.to_string())))
        }
    } else {
        Err(DispatcherError::from(InvalidRequest(String::from("service request is not valid"))))
    }
}

pub async fn get(srq: &ServiceRequest) -> DispatchResult<Value> {
    if srq.is_valid_get() {
        match Service::read(to_value(srq).unwrap()).await {
            Ok(srv) => {
                match to_value(*srv) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(DispatcherError::from(InvalidResponse(e.to_string())))
                }
            },
            Err(e) => Err(DispatcherError::from(NotFound(e.to_string())))
        }
    } else {
        Err(DispatcherError::from(InvalidRequest(String::from("service request is not valid"))))
    }
}
