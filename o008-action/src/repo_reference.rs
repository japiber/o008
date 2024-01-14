use serde_json::{to_value, Value};
use tracing::info;
use o008_common::{DispatcherError, DispatchResult, RepoReferenceRequest, RequestValidator};
use o008_common::AppCommandError::{Create, InvalidRequest, NotFound};
use o008_entity::{persist_json, QueryEntity};
use o008_entity::pg::RepoReference;

pub async fn create(rrq: &RepoReferenceRequest) -> DispatchResult<Value> {
    info!("create repo reference {:?}", rrq);
    match rrq.is_valid_create() {
        Ok(_) => {
            let repo_ref : RepoReference = From::from(rrq.clone());
            let r = persist_json(Box::new(repo_ref)).await;
            r.map_err(|e| DispatcherError::from(Create(format!("create action: {}", e))))
        },
        Err(e) => Err(DispatcherError::from(InvalidRequest(format!("create action: {}", e))))
    }
}

pub async fn get(rrq: &RepoReferenceRequest) -> DispatchResult<Value> {
    info!("get repo reference {:?}", rrq);
    match rrq.is_valid_get() {
        Ok(_) => match RepoReference::read(to_value(rrq).unwrap()).await {
            Ok (rr) => Ok(to_value(*rr).unwrap()),
            Err(e) => Err(DispatcherError::from(NotFound(format!("get action: {}", e)))),
        },
        Err(e) => Err(DispatcherError::from(InvalidRequest(format!("get action: {}", e))))
    }
}