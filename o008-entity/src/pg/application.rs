use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::Postgres;
use tracing::info;
use uuid::Uuid;
use o008_dal::{DaoCommand, DaoQuery};
use o008_dal::pg::{PgCommandContext, PgQueryContext};
use crate::{AsyncFrom, Entity, EntityError, Tenant};
use crate::pg::tenant::TenantDao;

type ApplicationDao = o008_dal::pg::Application;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Application {
    id: Uuid,
    name: String,
    tenant: Tenant,
    class_unit: String,
}

#[async_trait]
impl<'q> Entity<ApplicationDao, PgQueryContext<'q>, PgCommandContext<'q>, Postgres> for Application {
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
                    tenant: self.tenant.clone(),
                    class_unit: String::from(&self.class_unit),
                }))
            },
            Err(e) => Err(EntityError::Persist(e))
        }
    }

    async fn destroy(&self) -> Result<(), EntityError> {
        if self.id.is_nil() {
            Err(EntityError::UnPersisted(String::from("cannot destroy not previously persisted application")))
        } else {
            match self.dao().delete().await {
                Ok(_) => Ok(()),
                Err(e) => Err(EntityError::Destroy(e))
            }
        }
    }

    fn dao(&self) -> Box<ApplicationDao> {
        Box::new(if self.id.is_nil() {
            ApplicationDao::new(Uuid::new_v4(), &self.name, self.tenant.dao().id(), &self.class_unit)
        } else {
            ApplicationDao::new(self.id, &self.name, self.tenant.dao().id(), &self.class_unit)
        })
    }

}

impl Application {
    pub fn new(name: &str, t: Tenant, cu: &str) -> Self {
        Self {
            id: Uuid::nil(),
            name: String::from(name),
            tenant: t,
            class_unit: String::from(cu),
        }
    }

    pub async fn get_by_name(name: &str) -> Option<Self> {
        let dao = ApplicationDao::search_name(name).await;
        match dao {
            Ok(ba) => {
              let app = *ba;
              Some(AsyncFrom::<ApplicationDao, PgQueryContext, PgCommandContext, Postgres>::from(app).await)
            },
            Err(e) => {
                info!("application {} not found: {}", name, e);
                None
            }
        }
    }
}

#[async_trait::async_trait]
impl<'q> AsyncFrom<ApplicationDao, PgQueryContext<'q>, PgCommandContext<'q>, Postgres> for Application {
    async fn from(value: ApplicationDao) -> Self {
        let td = TenantDao::read(json!({"id": value.tenant.to_string()})).await.unwrap();
        Self {
            id: td.id(),
            name: String::from(&value.name),
            tenant: Tenant::from(*td),
            class_unit: String::from(&value.class_unit),
        }
    }
}