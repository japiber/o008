use o008_common::{RequestValidator, ServiceRequest};
use o008_entity::{Application, persist_json, QueryEntity, Service};
use crate::{DispatcherError, DispatchResult};
use crate::AppCommandError::{Create, InvalidRequest, NotFound};

pub async fn create(srq: &ServiceRequest) -> DispatchResult<serde_json::Value> {
    if srq.is_valid_create() {
        let arq = srq.application();
        if arq.is_valid_get() && arq.tenant().is_valid_get() {
            match Application::read(serde_json::to_value(srq).unwrap()).await {
                Ok(app) => {
                    let srv = Service::new(srq.name(), *app, srq.default_repo());
                    let r = persist_json(Box::new(srv)).await;
                    r.map_err(|e| DispatcherError::from(Create(format!("{}", e))))
                },
                Err(e) => Err(DispatcherError::from(NotFound(e.to_string())))
            }
        } else {
            Err(DispatcherError::AppCommand(InvalidRequest(String::from("service request is not valid for create: application and application/tenant name must be specified"))))
        }
    } else {
        Err(DispatcherError::AppCommand(InvalidRequest(String::from("service request is not valid for create"))))
    }
}

pub async fn get(srq: &ServiceRequest) -> DispatchResult<serde_json::Value> {
    if srq.is_valid_get() {
        let arq = srq.application();
        if arq.is_valid_get() && arq.tenant().is_valid_get() {
            match Service::read(serde_json::to_value(srq).unwrap()).await {
                Ok(srv) => Ok(serde_json::to_value(*srv).unwrap()),
                Err(e) => Err(DispatcherError::from(NotFound(e.to_string())))
            }
        } else {
            Err(DispatcherError::AppCommand(InvalidRequest(String::from("service request is not valid for get: application name and application tenant name must be specified"))))
        }
    } else {
        Err(DispatcherError::AppCommand(InvalidRequest(String::from("service request is not valid for get: service name must be specified"))))
    }
}
