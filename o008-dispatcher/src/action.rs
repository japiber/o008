use o008_entity::{Builder, Entity, persist_json, Tenant};
use o008_entity::pg::Application;
use crate::{DispatchResult};
use crate::error::AppCommandError::{Create, Destroy, NotFound};
use crate::error::DispatcherError;


pub async fn create_builder_action(name: &str, active: bool, cmd: &str) -> DispatchResult<serde_json::Value> {
    let builder = Builder::new(name, active, cmd);
    let r = persist_json(Box::new(builder)).await;
    r.map_err(|e| DispatcherError::AppCommand(Create(Box::new(e))))
}

pub async fn get_builder_action(name: &str) -> DispatchResult<serde_json::Value> {
    if let Some(b) = Builder::get_by_name(&name).await {
        Ok(serde_json::to_value(b).unwrap())
    } else {
        Err(DispatcherError::AppCommand(NotFound(format!("builder '{}'", &name))))
    }
}

pub async fn delete_builder_action(name: &str) -> DispatchResult<serde_json::Value> {
    if let Some(b) = Builder::get_by_name(&name).await {
        match b.destroy().await {
            Ok(_) => Ok(serde_json::to_value(b.name).unwrap()),
            Err(e) => Err(DispatcherError::AppCommand(Destroy(Box::new(e))))
        }
    } else {
        Err(DispatcherError::from(NotFound(format!("builder '{}'", &name))))
    }
}

pub async fn create_tenant_action(name: &str, coexisting: bool) -> DispatchResult<serde_json::Value> {
    let t = Tenant::new(name, coexisting);
    let r = persist_json(Box::new(t)).await;
    r.map_err(|e| DispatcherError::from(Create(Box::new(e))))
}

pub async fn create_application_action(name: &str, tenant: &str, class_unit: &str) -> DispatchResult<serde_json::Value> {
    let ot = Tenant::get_by_name(tenant).await;
    match ot {
        None => Err(DispatcherError::from(NotFound(format!("tenant '{}'", tenant)))),
        Some(t) => {
            let app = Application::new(name, t, class_unit);
            let r = persist_json(Box::new(app)).await;
            r.map_err(|e| DispatcherError::from(Create(Box::new(e))))
        }
    }
}

pub async fn get_application_action(name: &str) -> DispatchResult<serde_json::Value> {
    if let Some(app) = Application::get_by_name(name).await {
        Ok(serde_json::to_value(app).unwrap())
    } else {
        Err(DispatcherError::from(NotFound(format!("application '{}'", name))))
    }
}
