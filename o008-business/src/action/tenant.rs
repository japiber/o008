use serde_json::{to_value, Value};
use tracing::info;
use o008_common::{RequestValidator, TenantRequest, DispatchResult};
use o008_entity::{persist_json, QueryEntity, Tenant};
use o008_common::error::AppCommandError::{Create, InvalidRequest, InvalidResponse, NotFound};
use o008_common::error::DispatcherError;

pub async fn create(trq: TenantRequest) -> DispatchResult<Value> {
    info!("create tenant {:?}", trq);
    match trq.is_valid_create() {
        Ok(()) => {
            let t = Tenant::new(trq.name(), trq.coexisting());
            let r = persist_json(&t).await;
            r.map_err(|e| DispatcherError::from(Create(format!("{}", e))))
        },
        Err(e) => Err(DispatcherError::from(InvalidRequest(format!("create action: {}", e))))
    }
}

pub async fn get(trq: TenantRequest) -> DispatchResult<Value> {
    info!("create tenant {:?}", trq);
    match trq.is_valid_get() {
        Ok(()) => match Tenant::read(to_value(trq).unwrap()).await {
            Ok(b) => {
                match to_value(*b) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(DispatcherError::from(InvalidResponse(e.to_string())))
                }
            },
            Err(e) => Err(DispatcherError::from(NotFound(e.to_string())))
        },
        Err(e) => Err(DispatcherError::from(InvalidRequest(format!("get action: {}", e))))
    }
}
