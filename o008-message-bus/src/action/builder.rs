use serde_json::{to_value, Value};
use tracing::info;

use o008_common::{BuilderRequest, RequestValidator, DispatchResult};
use o008_entity::{Builder, DestroyEntity, persist_json, QueryEntity};

use o008_common::error::AppCommandError::{Create, Destroy, InvalidRequest, NotFound};
use o008_common::error::DispatcherError;

pub async fn create(brq: BuilderRequest) -> DispatchResult<Value> {
    info!("create builder {:?}", brq);
    match brq.is_valid_create() {
        Ok(()) => {
            let builder: Builder = From::<BuilderRequest>::from(brq.clone());
            let r = persist_json(Box::new(builder)).await;
            r.map_err(|e| DispatcherError::from(Create(format!("create action: {}", e))))
        }
        Err(e) => Err(DispatcherError::from(InvalidRequest(format!("create action: {}", e))))
    }
}

pub async fn get(brq: BuilderRequest) -> DispatchResult<Value> {
    info!("get builder {:?}", brq);
    match brq.is_valid_get() {
        Ok(()) => match Builder::read(to_value(brq).unwrap()).await {
            Ok(b) => Ok(to_value(*b).unwrap()),
            Err(e) => Err(DispatcherError::from(NotFound(format!("get action: {}", e))))
        },
        Err(e) => Err(DispatcherError::from(InvalidRequest(format!("get action: {}", e))))
    }
}

pub async fn delete(brq: BuilderRequest) -> DispatchResult<serde_json::Value> {
    info!("update builder {:?}", &brq);
    match brq.is_valid_get() {
        Ok(()) => match Builder::read(to_value(&brq).unwrap()).await {
            Ok(b) => match b.destroy().await {
                Ok(_) => Ok(serde_json::Value::Null),
                Err(e) => Err(DispatcherError::from(Destroy(format!("delete action: {}", e))))
            },
            Err(_) => Err(DispatcherError::from(NotFound(format!("builder '{:?}'", &brq))))
        },
        Err(e) => Err(DispatcherError::from(InvalidRequest(format!("delete action: {}", e))))
    }
}
