use o008_common::{RequestValidator, TenantRequest};
use o008_entity::{persist_json, Tenant};
use crate::{DispatcherError, DispatchResult};
use crate::AppCommandError::{Create, InvalidRequest, NotFound};

pub async fn create(trq: &TenantRequest) -> DispatchResult<serde_json::Value> {
    if trq.is_valid_create() {
        let t = Tenant::new(trq.name(), trq.coexisting());
        let r = persist_json(Box::new(t)).await;
        r.map_err(|e| DispatcherError::from(Create(format!("{}", e))))
    } else {
        Err(DispatcherError::AppCommand(InvalidRequest(String::from("tenant request is not valid for create"))))
    }
}

pub async fn get(trq: &TenantRequest) -> DispatchResult<serde_json::Value> {
    if trq.is_valid_get() {
        let name = trq.name();
        if let Some(b) = Tenant::get_by_name(name).await {
            Ok(serde_json::to_value(b).unwrap())
        } else {
            Err(DispatcherError::AppCommand(NotFound(format!("tenant '{}'", name))))
        }
    } else {
        Err(DispatcherError::AppCommand(InvalidRequest(String::from("tenant request is not valid for get: the name must be specified"))))
    }
}
