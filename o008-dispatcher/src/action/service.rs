use o008_common::{RequestValidator, ServiceRequest};
use o008_entity::{Application, persist_json, Service};
use crate::{DispatcherError, DispatchResult};
use crate::AppCommandError::{Create, InvalidRequest, NotFound};

pub async fn create(srq: &ServiceRequest) -> DispatchResult<serde_json::Value> {
    if srq.is_valid_create() {
        let arq = srq.application();
        if arq.is_valid_get() && arq.tenant().is_valid_get() {
            let app_name = arq.name();
            let oa = Application::get_by_name_and_tenant(app_name, arq.tenant().name()).await;
            match oa {
                None => Err(DispatcherError::from(NotFound(format!("application '{}'", app_name)))),
                Some(app) => {
                    let srv = Service::new(srq.name(), app, srq.default_repo());
                    let r = persist_json(Box::new(srv)).await;
                    r.map_err(|e| DispatcherError::from(Create(format!("{}", e))))
                }
            }
        } else {
            Err(DispatcherError::AppCommand(InvalidRequest(String::from("service request is not valid for create: tenant name must be specified"))))
        }
    } else {
        Err(DispatcherError::AppCommand(InvalidRequest(String::from("service request is not valid for create"))))
    }
}

pub async fn get(srq: &ServiceRequest) -> DispatchResult<serde_json::Value> {
    if srq.is_valid_get() {
        let name = srq.name();
        let arq = srq.application();
        if arq.is_valid_get() {
            let trq = arq.tenant();
            if trq.is_valid_get() {
                if let Some(srv) = Service::get_by_name_and_application(name, arq.name(), trq.name()).await {
                    Ok(serde_json::to_value(srv).unwrap())
                } else {
                    Err(DispatcherError::from(NotFound(format!("service '{}'", name))))
                }
            } else {
                Err(DispatcherError::AppCommand(InvalidRequest(String::from("service request is not valid for get: application tenant name must be specified"))))
            }
        } else {
            Err(DispatcherError::AppCommand(InvalidRequest(String::from("service request is not valid for get: application name and tenant must be specified"))))
        }
    } else {
        Err(DispatcherError::AppCommand(InvalidRequest(String::from("service request is not valid for get: the name must be specified"))))
    }
}
