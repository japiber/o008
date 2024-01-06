use serde_json::{to_value, Value};

use o008_common::{BuilderRequest, RequestValidator, DispatchResult};
use o008_entity::{Builder, DestroyEntity, persist_json, QueryEntity};

use o008_common::error::AppCommandError::{Create, Destroy, InvalidRequest, InvalidResponse, NotFound};
use o008_common::error::DispatcherError;

pub async fn create(brq: &BuilderRequest) -> DispatchResult<Value> {
    match brq.is_valid_create() {
        Ok(()) => {
            let builder: Builder = From::<BuilderRequest>::from(brq.clone());
            let r = persist_json(Box::new(builder)).await;
            r.map_err(|e| DispatcherError::from(Create(e.to_string())))
        }
        Err(e) => Err(DispatcherError::from(InvalidRequest(e.to_string())))
    }
}

pub async fn get(brq: &BuilderRequest) -> DispatchResult<Value> {
    match brq.is_valid_get() {
        Ok(()) => match Builder::read(to_value(brq).unwrap()).await {
            Ok(b) => {
                match to_value(b) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(DispatcherError::from(InvalidResponse(e.to_string())))
                }
            }
            Err(e) => Err(DispatcherError::from(NotFound(e.to_string())))
        },
        Err(e) => Err(DispatcherError::from(InvalidRequest(e.to_string())))
    }
}

pub async fn delete(brq: &BuilderRequest) -> DispatchResult<serde_json::Value> {
    match brq.is_valid_get() {
        Ok(()) => if let Ok(b) = Builder::read(to_value(brq).unwrap()).await {
            match b.destroy().await {
                Ok(_) => Ok(serde_json::Value::Null),
                Err(e) => Err(DispatcherError::from(Destroy(e.to_string())))
            }
        } else {
            Err(DispatcherError::from(NotFound(format!("builder '{:?}'", brq))))
        },
        Err(e) => Err(DispatcherError::from(InvalidRequest(e.to_string())))
    }
}
