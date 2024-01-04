use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::Postgres;
use utoipa::ToSchema;
use uuid::Uuid;
use o008_common::{AsyncFrom, TenantRequest};
use o008_dal::{DalError, DaoCommand, DaoQuery};
use o008_dal::pg::{ PgPool};
use crate::{DestroyEntity, Entity, EntityError, PersistEntity, QueryEntity};

pub type TenantDao = o008_dal::pg::Tenant;


#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, ToSchema)]
pub struct Tenant {
    #[serde(rename(serialize = "_id", deserialize = "id"))]
    id: Uuid,
    name: String,
    coexisting: bool,
}

impl Tenant {
    pub fn new(name: &str, coexisting: bool) -> Self {
        Self {
            id: Uuid::nil(),
            name: String::from(name),
            coexisting
        }
    }

    pub fn load(id: Uuid, name: &str, coexisting: bool) -> Self {
        Self {
            id,
            name: String::from(name),
            coexisting
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn coexisting(&self) -> bool {
        self.coexisting
    }
}

impl Entity<TenantDao> for Tenant {
    fn dao(&self) -> Box<TenantDao> {
        if self.id.is_nil() {
            Box::new(TenantDao::new(Uuid::new_v4(), &self.name, self.coexisting))
        } else {
            Box::new(TenantDao::new(self.id, &self.name, self.coexisting))
        }
    }
}

#[async_trait]
impl QueryEntity<TenantDao, PgPool, Postgres> for Tenant {
    async fn read(qry: Value) -> Result<Box<Self>, EntityError> {
        match TenantDao::read(qry).await {
            Ok(bt) => Ok(Box::new(From::<TenantDao>::from(*bt))),
            Err(e) => match e {
                DalError::InvalidKey(_) => Err(EntityError::WrongQuery(e.to_string())),
                _ => Err(EntityError::NotFound(e.to_string())),
            }
        }
    }
}

#[async_trait]
impl PersistEntity<TenantDao, PgPool, Postgres> for Tenant {
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
                    coexisting: self.coexisting,
                }))
            },
            Err(e) => Err(EntityError::Persist(e))
        }
    }
}

#[async_trait]
impl DestroyEntity<TenantDao, PgPool, Postgres> for Tenant {
    async fn destroy(&self) -> Result<(), EntityError> {
        if self.id.is_nil() {
            Err(EntityError::UnPersisted(String::from("tenant")))
        } else {
            match self.dao().delete().await {
                Ok(_) => Ok(()),
                Err(e) => Err(EntityError::Destroy(e))
            }
        }
    }
}

impl From<TenantDao> for Tenant {
    fn from(value: TenantDao) -> Self {
        Self::load(value.id(), value.name(), value.coexisting())
    }
}

#[async_trait]
impl AsyncFrom<TenantRequest> for Tenant {
    async fn from(value: TenantRequest) -> Self {
        *Tenant::read(serde_json::to_value(value).unwrap()).await.unwrap()
    }
}
