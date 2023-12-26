use o008_common::{ApplicationRequest, BuilderRequest, RequestValidator, TenantRequest};
use o008_entity::{Builder, Entity, persist_json, Tenant};
use o008_entity::pg::Application;
use crate::DispatchResult;
use crate::AppCommandError::InvalidRequest;
use crate::error::AppCommandError::{Create, Destroy, NotFound};
use crate::error::DispatcherError;


pub async fn create_builder_action(brq: &BuilderRequest) -> DispatchResult<serde_json::Value> {
    if brq.is_valid_create() {
        let builder = Builder::from(brq);
        let r = persist_json(Box::new(builder)).await;
        r.map_err(|e| DispatcherError::AppCommand(Create(format!("{}", e))))
    } else {
        Err(DispatcherError::AppCommand(InvalidRequest(String::from("builder request is not valid for create"))))
    }
}

pub async fn get_builder_action(brq: &BuilderRequest) -> DispatchResult<serde_json::Value> {
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

pub async fn delete_builder_action(brq: &BuilderRequest) -> DispatchResult<serde_json::Value> {
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

pub async fn create_tenant_action(trq: &TenantRequest) -> DispatchResult<serde_json::Value> {
    if trq.is_valid_create() {
        let t = Tenant::new(trq.name(), trq.coexisting());
        let r = persist_json(Box::new(t)).await;
        r.map_err(|e| DispatcherError::from(Create(format!("{}", e))))
    } else {
        Err(DispatcherError::AppCommand(InvalidRequest(String::from("tenant request is not valid for create"))))
    }
}

pub async fn get_tenant_action(trq: &TenantRequest) -> DispatchResult<serde_json::Value> {
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

pub async fn create_application_action(arq: &ApplicationRequest) -> DispatchResult<serde_json::Value> {
    if arq.is_valid_create() {
        let tq = arq.tenant();
        if tq.is_valid_get() {
            let tq_name = tq.name();
            let ot = Tenant::get_by_name(tq_name).await;
            match ot {
                None => Err(DispatcherError::from(NotFound(format!("tenant '{}'", tq_name)))),
                Some(t) => {
                    let app = Application::new(arq.name(), t, arq.class_unit());
                    let r = persist_json(Box::new(app)).await;
                    r.map_err(|e| DispatcherError::from(Create(format!("{}", e))))
                }
            }
        } else {
            Err(DispatcherError::AppCommand(InvalidRequest(String::from("application request is not valid for create: tenant name must be specified"))))
        }
    } else {
        Err(DispatcherError::AppCommand(InvalidRequest(String::from("application request is not valid for create"))))
    }
}

pub async fn get_application_action(arq: &ApplicationRequest) -> DispatchResult<serde_json::Value> {
    if arq.is_valid_get() {
        let name = arq.name();
        if let Some(app) = Application::get_by_name(name).await {
            Ok(serde_json::to_value(app).unwrap())
        } else {
            Err(DispatcherError::from(NotFound(format!("application '{}'", name))))
        }
    } else {
        Err(DispatcherError::AppCommand(InvalidRequest(String::from("application request is not valid for get: the name must be specified"))))
    }
}
