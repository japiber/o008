use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::Postgres;
use utoipa::ToSchema;
use uuid::Uuid;
use o008_common::{ApplicationRequest, AsyncFrom};
use o008_dal::{DalError, DaoCommand, DaoQuery};
use o008_dal::pg::{PgPool};
use crate::{DestroyEntity, Entity, EntityError, PersistEntity, QueryEntity, Tenant};
use crate::pg::tenant::TenantDao;

pub(crate) type ApplicationDao = o008_dal::pg::Application;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct Application {
    #[serde(rename(serialize = "_id", deserialize = "id"))]
    id: Uuid,
    name: String,
    tenant: Tenant,
    class_unit: String,
    functional_group: String,
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

    pub fn load(id: Uuid, name: &str, t: Tenant, cu: &str, fg: &str) -> Self {
        Self {
            id,
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
}

impl Entity<ApplicationDao> for Application {
    fn dao(&self) -> Box<ApplicationDao> {
        Box::new(ApplicationDao::new(self.id, &self.name, self.tenant.dao().id(), &self.class_unit, &self.functional_group))
    }
}

#[async_trait]
impl QueryEntity<ApplicationDao, PgPool, Postgres> for Application {
    async fn read(qry: Value) -> Result<Box<Self>, EntityError> {
        match ApplicationDao::read(qry).await {
            Ok(app) => Ok(Box::new(AsyncFrom::<ApplicationDao>::from(*app).await)),
            Err(e) => match e {
                DalError::InvalidKey(_) => Err(EntityError::WrongQuery(e.to_string())),
                _ => Err(EntityError::NotFound(e.to_string())),
            }
        }
    }

    async fn persisted(qry: Value) -> bool {
        ApplicationDao::exists(qry).await
    }
}

#[async_trait]
impl PersistEntity<ApplicationDao, PgPool, Postgres> for Application {
    async fn persist(&self) -> Result<Box<Self>, EntityError> {
        let dao = self.dao();
        let r = if self.id.is_nil() {
            dao.insert().await
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
}

#[async_trait]
impl DestroyEntity<ApplicationDao, PgPool, Postgres> for Application {
    async fn destroy(&self) -> Result<(), EntityError> {
        if self.id.is_nil() {
            Err(EntityError::UnPersisted(String::from("application")))
        } else {
            match self.dao().delete().await {
                Ok(_) => Ok(()),
                Err(e) => Err(EntityError::Destroy(e))
            }
        }
    }
}

#[async_trait::async_trait]
impl AsyncFrom<ApplicationDao> for Application {
    async fn from(value: ApplicationDao) -> Self {
        let td = TenantDao::read(json!({"id": value.tenant().to_string()})).await.unwrap();
        Self::load(value.id(), value.name(), From::<TenantDao>::from(*td), value.class_unit(), value.functional_group())
    }
}

#[async_trait::async_trait]
impl AsyncFrom<ApplicationRequest> for Application {
    async fn from(value: ApplicationRequest) -> Self {
        let app = ApplicationDao::read(serde_json::to_value(value).unwrap()).await.unwrap();
        AsyncFrom::<ApplicationDao>::from(*app).await
    }
}
