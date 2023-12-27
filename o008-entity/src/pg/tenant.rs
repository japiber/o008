use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::Postgres;
use tracing::info;
use uuid::Uuid;
use o008_dal::DaoCommand;
use o008_dal::pg::{PgCommandContext, PgQueryContext};
use crate::{Entity, EntityError};

pub type TenantDao = o008_dal::pg::Tenant;


#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
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

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn coexisting(&self) -> bool {
        self.coexisting
    }

    pub async fn get_by_name(name: &str) -> Option<Self> {
        let dao = TenantDao::search_name(name).await;
        match dao {
            Ok(bt) => {
                let tt = *bt;
                Some(From::<TenantDao>::from(tt))
            },
            Err(e) => {
                info!("tenant {} not found: {}", name, e);
                None
            }
        }
    }
}

impl From<TenantDao> for Tenant {
    fn from(value: TenantDao) -> Self {
       Self {
           id: value.id(),
           name: String::from(value.name()),
           coexisting: value.coexisting(),
       }
    }
}

#[async_trait]
impl<'q> Entity<TenantDao, PgQueryContext<'q>, PgCommandContext<'q>, Postgres> for Tenant {
    async fn persist(&self) -> Result<Box<Self>, EntityError> {
        let dao = self.dao();
        let r = if self.id.is_nil() {
            dao.create().await
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

    async fn destroy(&self) -> Result<(), EntityError> {
        if self.id.is_nil() {
            Err(EntityError::UnPersisted(String::from("cannot destroy not previously persisted tenant")))
        } else {
            match self.dao().delete().await {
                Ok(_) => Ok(()),
                Err(e) => Err(EntityError::Destroy(e))
            }
        }
    }

    fn dao(&self) -> Box<TenantDao> {
        if self.id.is_nil() {
            Box::new(TenantDao::new(Uuid::new_v4(), &self.name, self.coexisting))
        } else {
            Box::new(TenantDao::new(self.id, &self.name, self.coexisting))
        }
    }
}