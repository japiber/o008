use serde_json::{to_value, Value};

use o008_common::{ApplicationRequest, RequestValidator, DispatchResult};
use o008_entity::{Application, persist_json, QueryEntity, Tenant};

use o008_common::error::AppCommandError::{Create, InvalidRequest, InvalidResponse, NotFound};
use o008_common::error::DispatcherError;

pub async fn create(arq: &ApplicationRequest) -> DispatchResult<Value> {
    match arq.is_valid_create() {
        Ok(()) => match Tenant::read(serde_json::to_value(arq).unwrap()).await {
            Ok(t) => {
                let app = Application::new(arq.name(), *t, arq.class_unit(), arq.functional_group());
                let r = persist_json(Box::new(app)).await;
                r.map_err(|e| DispatcherError::from(Create(e.to_string())))
            }
            Err(e) => Err(DispatcherError::from(NotFound(e.to_string())))
        },
        Err(e) => Err(DispatcherError::AppCommand(InvalidRequest(e.to_string())))
    }
}

pub async fn get(arq: &ApplicationRequest) -> DispatchResult<Value> {
    match arq.is_valid_get() {
        Ok(()) => match Application::read(serde_json::to_value(arq).unwrap()).await {
            Ok(app) => {
                match to_value(app) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(DispatcherError::from(InvalidResponse(e.to_string())))
                }
            }
            Err(e) => Err(DispatcherError::from(NotFound(e.to_string())))
        },
        Err(e) => Err(DispatcherError::AppCommand(InvalidRequest(e.to_string())))
    }
}
