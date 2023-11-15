use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::Postgres;
use o008_dal::{Dao, error};
use o008_dal::pg::PgQueryContext;
use crate::Entity;

type TenantDao = o008_dal::pg::Tenant;


#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TenantInner {
    pub name: String,
    pub coexisting: bool,
}

pub struct Tenant(TenantInner, Option<TenantDao>);

impl TenantInner {
    pub fn new(name: &str, coexisting: bool) -> Self {
        Self {
            name: String::from(name),
            coexisting
        }
    }
}

impl Tenant {
    pub fn new(name: &str, coexisting: bool) -> Self {
        Self(
            TenantInner::new(name, coexisting),
            None
        )
    }
}

impl From<TenantDao> for Tenant {
    fn from(value: TenantDao) -> Self {
        Self (
            TenantInner::new(&value.name, value.coexisting),
            Some(value)
        )
    }
}

#[async_trait]
impl<'q> Entity<TenantInner, TenantDao, PgQueryContext<'q>, Postgres> for Tenant {
    async fn persist(&mut self) -> Result<(), error::Error> {
        let dao = self.dao();
        let p = match self.1 {
            None => dao.create().await,
            Some(_) => dao.update().await
        };
        self.1 = Some(*dao);
        p
    }

    fn dao(&self) -> Box<TenantDao> {
        match &self.1 {
            None => Box::new(TenantDao {
                id: uuid::Uuid::new_v4(),
                name: self.0.name.clone(),
                coexisting: self.0.coexisting,
            }),
            Some(t) => Box::new(TenantDao {
                id: t.id,
                name: self.0.name.clone(),
                coexisting: self.0.coexisting
            })
        }
    }

    fn inner(&self) -> Box<TenantInner> {
        Box::new(self.0.clone())
    }
}