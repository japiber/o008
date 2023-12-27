use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::Postgres;
use tracing::info;
use uuid::Uuid;
use o008_dal::{DaoCommand, DaoQuery};
use o008_dal::pg::{PgCommandContext, PgQueryContext};
use crate::{AsyncFrom, Entity, EntityError};
use crate::pg::Application;
use crate::pg::application::ApplicationDao;
use crate::pg::tenant::TenantDao;

type ServiceDao = o008_dal::pg::Service;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Service {
    #[serde(rename(serialize = "_id", deserialize = "id"))]
    id: Uuid,
    name: String,
    original_name: String,
    application: Application,
    default_repo: String,
}

impl Service {
    pub fn new(name: &str, app: Application, repo: &str) -> Self {
        Self {
            id: Uuid::nil(),
            name: String::from(name).to_lowercase(),
            original_name: String::from(name),
            application: app,
            default_repo: String::from(repo),
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn original_name(&self) -> &str {
        &self.original_name
    }

    pub fn application(&self) -> Application {
        self.application.clone()
    }

    pub fn default_repo(&self) -> &str {
        &self.default_repo
    }

    pub async fn get_by_name_and_application(name: &str, app_name: &str, tenant_name: &str) -> Option<Self> {
        if let Ok(tenant_dao) = TenantDao::search_name(tenant_name).await {
            if let Ok(app_dao) = ApplicationDao::search_name_tenant(app_name, tenant_dao.id()).await {
                if let Ok(srv_dao) = ServiceDao::search_name_application(name, app_dao.id()).await {
                    Some(AsyncFrom::<ServiceDao, PgQueryContext, PgCommandContext, Postgres>::from(*srv_dao).await)
                } else {
                    info!("service {} not found", name);
                    None
                }
            } else {
                info!("application {} not found", app_name);
                None
            }
        } else {
            info!("tenant {} not found", tenant_name);
            None
        }
    }
}

#[async_trait]
impl<'q> Entity<ServiceDao, PgQueryContext<'q>, PgCommandContext<'q>, Postgres> for Service {
    async fn persist(&self) -> Result<Box<Self>, EntityError> {
        let dao = self.dao();
        let r= if self.id.is_nil() {
            dao.create().await
        } else {
            dao.update().await
        };
        match r {
            Ok(_) => {
                Ok(Box::new(Self {
                    id: dao.id(),
                    name: String::from(&self.name),
                    original_name: String::from(&self.original_name),
                    application: self.application.clone(),
                    default_repo: String::from(&self.default_repo),
                }))
            },
            Err(e) => Err(EntityError::Persist(e))
        }
    }

    async fn destroy(&self) -> Result<(), EntityError> {
        if self.id.is_nil() {
            Err(EntityError::UnPersisted(String::from("cannot destroy not previously persisted service")))
        } else {
            match self.dao().delete().await {
                Ok(_) => Ok(()),
                Err(e) => Err(EntityError::Destroy(e))
            }
        }
    }

    fn dao(&self) -> Box<ServiceDao> {
        if self.id.is_nil() {
            Box::new(ServiceDao::new(Uuid::new_v4(), &self.name, &self.original_name, self.application.id(), &self.default_repo))
        } else {
            Box::new(ServiceDao::new(self.id, &self.name, &self.original_name, self.application.id(), &self.default_repo))
        }
    }
}

#[async_trait::async_trait]
impl<'q> AsyncFrom<ServiceDao, PgQueryContext<'q>, PgCommandContext<'q>, Postgres> for Service {
    async fn from(value: ServiceDao) -> Self {
        let td = ApplicationDao::read(json!({"id": value.application().to_string()})).await.unwrap();
        Self {
            id: value.id(),
            name: String::from(value.name()),
            original_name: String::from(value.original_name()),
            application: AsyncFrom::<crate::pg::application::ApplicationDao, PgQueryContext, PgCommandContext, Postgres>::from(*td).await,
            default_repo: String::from(value.default_repo()),
        }
    }
}