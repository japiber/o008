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

pub(crate) type ApplicationDao = o008_dal::pg::Application;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Application {
    #[serde(rename(serialize = "_id", deserialize = "id"))]
    id: Uuid,
    name: String,
    tenant: Tenant,
    class_unit: String,
    functional_group: String,
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
                    functional_group: String::from(&self.functional_group),
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
            ApplicationDao::new(Uuid::new_v4(), &self.name, self.tenant.dao().id(), &self.class_unit, &self.functional_group)
        } else {
            ApplicationDao::new(self.id, &self.name, self.tenant.dao().id(), &self.class_unit, &self.functional_group)
        })
    }

}

impl Application {
    pub fn new(name: &str, t: Tenant, cu: &str, fg: &str) -> Self {
        Self {
            id: Uuid::nil(),
            name: String::from(name),
            tenant: t,
            class_unit: String::from(cu),
            functional_group: String::from(fg),
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn tenant(&self) -> Tenant {
        self.tenant.clone()
    }

    pub fn class_unit(&self) -> &str {
        &self.class_unit
    }

    pub fn functional_group(&self) -> &str {
        &self.functional_group
    }

    pub async fn get_by_name_and_tenant(name: &str, tenant: &str) -> Option<Self> {
        if let Ok(tenant_dao) = TenantDao::search_name(tenant).await {
            if let Ok(app_dao) = ApplicationDao::search_name_tenant(name, tenant_dao.id()).await {
                Some(AsyncFrom::<ApplicationDao, PgQueryContext, PgCommandContext, Postgres>::from(*app_dao).await)
            } else {
                info!("application {} not found", name);
                None
            }
        } else {
            info!("tenant {} not found", tenant);
            None
        }
    }
}

#[async_trait::async_trait]
impl<'q> AsyncFrom<ApplicationDao, PgQueryContext<'q>, PgCommandContext<'q>, Postgres> for Application {
    async fn from(value: ApplicationDao) -> Self {
        let td = TenantDao::read(json!({"id": value.tenant().to_string()})).await.unwrap();
        Self {
            id: value.id(),
            name: String::from(value.name()),
            tenant: Tenant::from(*td),
            class_unit: String::from(value.class_unit()),
            functional_group: String::from(value.functional_group()),
        }
    }
}