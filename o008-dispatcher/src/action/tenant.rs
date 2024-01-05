use serde_json::{to_value, Value};
use o008_common::{RequestValidator, TenantRequest};
use o008_entity::{persist_json, QueryEntity, Tenant};
use crate::{DispatcherError, DispatchResult};
use crate::AppCommandError::{Create, InvalidRequest, InvalidResponse, NotFound};

pub async fn create(trq: &TenantRequest) -> DispatchResult<Value> {
    if trq.is_valid_create() {
        let t = Tenant::new(trq.name(), trq.coexisting());
        let r = persist_json(Box::new(t)).await;
        r.map_err(|e| DispatcherError::from(Create(format!("{}", e))))
    } else {
        Err(DispatcherError::from(InvalidRequest(String::from("tenant request is not valid"))))
    }
}

pub async fn get(trq: &TenantRequest) -> DispatchResult<Value> {
    if trq.is_valid_get() {
        match Tenant::read(to_value(trq).unwrap()).await {
            Ok(b) => {
                match to_value(*b) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(DispatcherError::from(InvalidResponse(e.to_string())))
                }
            },
            Err(e) => Err(DispatcherError::from(NotFound(e.to_string())))
        }
    } else {
        Err(DispatcherError::from(InvalidRequest(String::from("tenant request is not valid"))))
    }
}
