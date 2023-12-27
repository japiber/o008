use o008_common::{BuilderRequest, RequestValidator};
use o008_entity::{Builder, Entity, persist_json};
use crate::{DispatcherError, DispatchResult};
use crate::AppCommandError::{Create, Destroy, InvalidRequest, NotFound};

pub async fn create(brq: &BuilderRequest) -> DispatchResult<serde_json::Value> {
    if brq.is_valid_create() {
        let builder = Builder::from(brq);
        let r = persist_json(Box::new(builder)).await;
        r.map_err(|e| DispatcherError::AppCommand(Create(format!("{}", e))))
    } else {
        Err(DispatcherError::AppCommand(InvalidRequest(String::from("builder request is not valid for create"))))
    }
}

pub async fn get(brq: &BuilderRequest) -> DispatchResult<serde_json::Value> {
    if brq.is_valid_get() {
        let name = brq.name();
        if let Some(b) = Builder::get_by_name(name).await {
            Ok(serde_json::to_value(b).unwrap())
        } else {
            Err(DispatcherError::AppCommand(NotFound(format!("builder '{}'", name))))
        }
    } else {
        Err(DispatcherError::AppCommand(InvalidRequest(String::from("builder request is not valid for get: the name must be specified"))))
    }
}

pub async fn delete(brq: &BuilderRequest) -> DispatchResult<serde_json::Value> {
    if brq.is_valid_get() {
        let name = brq.name();
        if let Some(b) = Builder::get_by_name(name).await {
            match b.destroy().await {
                Ok(_) => Ok(serde_json::to_value(b.name).unwrap()),
                Err(e) => Err(DispatcherError::AppCommand(Destroy(format!("{}", e))))
            }
        } else {
            Err(DispatcherError::from(NotFound(format!("builder '{}'", name))))
        }
    } else {
        Err(DispatcherError::AppCommand(InvalidRequest(String::from("builder request is not valid for delete: the name must be specified"))))
    }
}
