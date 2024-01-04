use o008_common::{BuilderRequest, RequestValidator};
use o008_entity::{Builder, DestroyEntity, persist_json, QueryEntity};
use crate::{DispatcherError, DispatchResult};
use crate::AppCommandError::{Create, Destroy, InvalidRequest, NotFound};

pub async fn create(brq: &BuilderRequest) -> DispatchResult<serde_json::Value> {
    if brq.is_valid_create() {
        let builder : Builder = From::<BuilderRequest>::from(brq.clone());
        let r = persist_json(Box::new(builder)).await;
        r.map_err(|e| DispatcherError::AppCommand(Create(format!("{}", e))))
    } else {
        Err(DispatcherError::AppCommand(InvalidRequest(String::from("builder request is not valid for create"))))
    }
}

pub async fn get(brq: &BuilderRequest) -> DispatchResult<serde_json::Value> {
    if brq.is_valid_get() {
        match Builder::read(serde_json::to_value(brq).unwrap()).await {
            Ok(b) => Ok(serde_json::to_value(b).unwrap()),
            Err(e) => Err(DispatcherError::AppCommand(NotFound(e.to_string())))
        }
    } else {
        Err(DispatcherError::AppCommand(InvalidRequest(String::from("builder request is not valid for get: the name must be specified"))))
    }
}

pub async fn delete(brq: &BuilderRequest) -> DispatchResult<serde_json::Value> {
    if brq.is_valid_get() {
        if let Ok(b) = Builder::read(serde_json::to_value(brq).unwrap()).await {
            match b.destroy().await {
                Ok(_) => Ok(serde_json::Value::Null),
                Err(e) => Err(DispatcherError::AppCommand(Destroy(format!("{}", e))))
            }
        } else {
            Err(DispatcherError::from(NotFound(format!("builder '{:?}'", brq))))
        }
    } else {
        Err(DispatcherError::AppCommand(InvalidRequest(String::from("builder request is not valid for delete: the name must be specified"))))
    }
}
