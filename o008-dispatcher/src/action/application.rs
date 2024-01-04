use o008_common::{ApplicationRequest, RequestValidator};
use o008_entity::{Application, persist_json, QueryEntity, Tenant};
use crate::{DispatcherError, DispatchResult};
use crate::AppCommandError::{Create, InvalidRequest, NotFound};

pub async fn create(arq: &ApplicationRequest) -> DispatchResult<serde_json::Value> {
    if arq.is_valid_create() {
        let tq = arq.tenant();
        if tq.is_valid_get() {
            match Tenant::read(serde_json::to_value(arq).unwrap()).await {
                Ok(t) => {
                    let app = Application::new(arq.name(), *t, arq.class_unit(), arq.functional_group());
                    let r = persist_json(Box::new(app)).await;
                    r.map_err(|e| DispatcherError::from(Create(format!("{}", e))))
                },
                Err(e) => Err(DispatcherError::from(NotFound(e.to_string())))
            }
        } else {
            Err(DispatcherError::AppCommand(InvalidRequest(String::from("application request is not valid for create: tenant name must be specified"))))
        }
    } else {
        Err(DispatcherError::AppCommand(InvalidRequest(String::from("application request is not valid for create"))))
    }
}

pub async fn get(arq: &ApplicationRequest) -> DispatchResult<serde_json::Value> {
    if arq.is_valid_get() {
        let app_tenant = arq.tenant();
        if app_tenant.is_valid_get() {
           match Application::read(serde_json::to_value(arq).unwrap()).await {
               Ok(app) => Ok(serde_json::to_value(app).unwrap()),
               Err(e) => Err(DispatcherError::from(NotFound(e.to_string())))
           }
        } else {
            Err(DispatcherError::AppCommand(InvalidRequest(String::from("application request is not valid for get: tenant name must be specified"))))
        }
    } else {
        Err(DispatcherError::AppCommand(InvalidRequest(String::from("application request is not valid for get: name and tenant must be specified"))))
    }
}
