use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::Postgres;
use crate::{Dao, QueryContext, error};
use crate::pg::PgQueryContext;

#[derive(Debug, PartialEq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Tenant {
    pub id: uuid::Uuid,
    pub name: String,
    pub coexisting: bool,
}

impl Tenant {
    pub fn new(id: uuid::Uuid, name: &str, coexisting: bool) -> Self {
        Self {
            id,
            name: String::from(name),
            coexisting,
        }
    }
}

#[async_trait]
impl<'q> Dao<PgQueryContext<'q>, Postgres> for Tenant {

    async fn create(&self) -> Result<(), error::Error> {
        let qx = Self::query_ctx().await;
        qx.execute(
            sqlx::query("INSERT INTO tenant (id, name, coexisting) VALUES ($1, $2, $3)")
            .bind(self.id)
            .bind(self.name.clone())
            .bind(self.coexisting)
        ).await
    }

    async fn read(key: serde_json::Value) -> Result<Box<Self>, error::Error> {
        let qx = Self::query_ctx().await;
        qx.fetch_one(
            sqlx::query_as::<_, Self>("SELECT id, name, coexisting FROM tenant WHERE id=$1")
            .bind(uuid::Uuid::parse_str(key["id"].as_str().unwrap()).unwrap())
        ).await
    }

    async fn update(&self) -> Result<(), error::Error> {
        let qx = Self::query_ctx().await;
        qx.execute(
            sqlx::query("UPDATE tenant SET name=$1, coexisting=$2 WHERE id=$3")
            .bind(self.name.clone())
            .bind(self.coexisting)
            .bind(self.id)
        ).await
    }

    async fn delete(&self) -> Result<(), error::Error> {
        let qx = Self::query_ctx().await;
        qx.execute(
            sqlx::query("DELETE FROM tenant WHERE id = $1")
            .bind(self.id)
        ).await
    }
}
