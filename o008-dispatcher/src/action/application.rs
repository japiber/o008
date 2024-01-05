use serde_json::{to_value, Value};
use o008_common::{ApplicationRequest, RequestValidator};
use o008_entity::{Application, persist_json, QueryEntity, Tenant};
use crate::{DispatcherError, DispatchResult};
use crate::AppCommandError::{Create, InvalidRequest, InvalidResponse, NotFound};

pub async fn create(arq: &ApplicationRequest) -> DispatchResult<Value> {
    if arq.is_valid_create() {
        match Tenant::read(serde_json::to_value(arq).unwrap()).await {
            Ok(t) => {
                let app = Application::new(arq.name(), *t, arq.class_unit(), arq.functional_group());
                let r = persist_json(Box::new(app)).await;
                r.map_err(|e| DispatcherError::from(Create(e.to_string())))
            },
            Err(e) => Err(DispatcherError::from(NotFound(e.to_string())))
        }
    } else {
        Err(DispatcherError::AppCommand(InvalidRequest(String::from("application request is not valid"))))
    }
}

pub async fn get(arq: &ApplicationRequest) -> DispatchResult<Value> {
    if arq.is_valid_get() {
       match Application::read(serde_json::to_value(arq).unwrap()).await {
           Ok(app) => {
              match to_value(app) {
                  Ok(v) => Ok(v),
                  Err(e) => Err(DispatcherError::from(InvalidResponse(e.to_string())))
              }
           },
           Err(e) => Err(DispatcherError::from(NotFound(e.to_string())))
       }
    } else {
        Err(DispatcherError::AppCommand(InvalidRequest(String::from("application request is not valid"))))
    }
}
