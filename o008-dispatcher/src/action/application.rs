use o008_common::{ApplicationRequest, RequestValidator};
use o008_entity::{Application, persist_json, Tenant};
use crate::{DispatcherError, DispatchResult};
use crate::AppCommandError::{Create, InvalidRequest, NotFound};

pub async fn create(arq: &ApplicationRequest) -> DispatchResult<serde_json::Value> {
    if arq.is_valid_create() {
        let tq = arq.tenant();
        if tq.is_valid_get() {
            let tq_name = tq.name();
            let ot = Tenant::get_by_name(tq_name).await;
            match ot {
                None => Err(DispatcherError::from(NotFound(format!("tenant '{}'", tq_name)))),
                Some(t) => {
                    let app = Application::new(arq.name(), t, arq.class_unit(), arq.functional_group());
                    let r = persist_json(Box::new(app)).await;
                    r.map_err(|e| DispatcherError::from(Create(format!("{}", e))))
                }
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
            let app_name = arq.name();
            if let Some(app) = Application::get_by_name_and_tenant(app_name, app_tenant.name()).await {
                Ok(serde_json::to_value(app).unwrap())
            } else {
                Err(DispatcherError::from(NotFound(format!("application '{}'", app_name))))
            }
        } else {
            Err(DispatcherError::AppCommand(InvalidRequest(String::from("application request is not valid for get: tenant name must be specified"))))
        }
    } else {
        Err(DispatcherError::AppCommand(InvalidRequest(String::from("application request is not valid for get: name and tenant must be specified"))))
    }
}
