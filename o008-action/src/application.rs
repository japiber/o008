use serde_json::{to_value, Value};
use tracing::info;

use o008_common::{ApplicationRequest, RequestValidator, DispatchResult};
use o008_entity::{Application, persist_json, QueryEntity, Tenant};

use o008_common::error::AppCommandError::{Create, InvalidRequest, InvalidResponse, NotFound};
use o008_common::error::DispatcherError;

pub async fn create(arq: &ApplicationRequest) -> DispatchResult<Value> {
    info!("create application {:?}", arq);
    match arq.is_valid_create() {
        Ok(()) => match Tenant::read(to_value(arq.tenant().unwrap()).unwrap()).await {
            Ok(tenant) => {
                let name = arq.name().unwrap().as_str();
                let cu = arq.class_unit().unwrap().as_str();
                let fg = arq.functional_group().unwrap().as_str();
                let app = Application::new(name, *tenant, cu, fg);
                let r = persist_json(Box::new(app)).await;
                r.map_err( | e| DispatcherError::from(Create(e.to_string())))
            },
            Err(e) => Err(DispatcherError::from(NotFound(format!("create action: {}", e))))
        },
        Err(e) => Err(DispatcherError::from(InvalidRequest(format!("create action: {}", e))))
    }
}

pub async fn get(arq: &ApplicationRequest) -> DispatchResult<Value> {
    info!("get application {:?}", arq);
    match arq.is_valid_get() {
        Ok(()) => match Application::read(to_value(arq).unwrap()).await {
            Ok(app) => {
                match to_value(app) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(DispatcherError::from(InvalidResponse(e.to_string())))
                }
            }
            Err(e) => Err(DispatcherError::from(NotFound(format!("get action: {}", e))))
        },
        Err(e) => Err(DispatcherError::AppCommand(InvalidRequest(e.to_string())))
    }
}
