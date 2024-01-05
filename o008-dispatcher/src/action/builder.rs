use serde_json::{to_value, Value};
use o008_common::{BuilderRequest, RequestValidator};
use o008_entity::{Builder, DestroyEntity, persist_json, QueryEntity};
use crate::{DispatcherError, DispatchResult};
use crate::AppCommandError::{Create, Destroy, InvalidRequest, InvalidResponse, NotFound};

pub async fn create(brq: &BuilderRequest) -> DispatchResult<Value> {
    if brq.is_valid_create() {
        let builder : Builder = From::<BuilderRequest>::from(brq.clone());
        let r = persist_json(Box::new(builder)).await;
        r.map_err(|e| DispatcherError::from(Create(e.to_string())))
    } else {
        Err(DispatcherError::from(InvalidRequest(String::from("builder request is not valid for create"))))
    }
}

pub async fn get(brq: &BuilderRequest) -> DispatchResult<Value> {
    if brq.is_valid_get() {
        match Builder::read(to_value(brq).unwrap()).await {
            Ok(b) => {
                match to_value(b) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(DispatcherError::from(InvalidResponse(e.to_string())))
                }
            },
            Err(e) => Err(DispatcherError::from(NotFound(e.to_string())))
        }
    } else {
        Err(DispatcherError::from(InvalidRequest(String::from("builder request is not valid"))))
    }
}

pub async fn delete(brq: &BuilderRequest) -> DispatchResult<serde_json::Value> {
    if brq.is_valid_get() {
        if let Ok(b) = Builder::read(to_value(brq).unwrap()).await {
            match b.destroy().await {
                Ok(_) => Ok(serde_json::Value::Null),
                Err(e) => Err(DispatcherError::from(Destroy(e.to_string())))
            }
        } else {
            Err(DispatcherError::from(NotFound(format!("builder '{:?}'", brq))))
        }
    } else {
        Err(DispatcherError::from(InvalidRequest(String::from("builder request is not valid"))))
    }
}
