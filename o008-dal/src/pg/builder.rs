use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::Postgres;
use crate::{Dao, QueryContext, error};
use crate::pg::{PgQueryContext};

#[derive(Debug, PartialEq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Builder {
    pub id: uuid::Uuid,
    pub name: String,
    pub active: bool,
    pub command: String,
}


#[async_trait]
impl<'q> Dao<PgQueryContext<'q>, Postgres> for Builder {

    async fn create(&self) -> Result<(), error::Error> {
        let qx = Self::query_ctx().await;
        qx.execute(
            sqlx::query("INSERT INTO builder (id, name, active, command) VALUES ($1, $2, $3, $4)")
                .bind(self.id)
                .bind(self.name.clone())
                .bind(self.active)
                .bind(self.command.clone())
        ).await
    }

    async fn read(key: serde_json::Value) -> Result<Box<Self>, error::Error> {
        let qx = Self::query_ctx().await;
        qx.fetch_one(
            sqlx::query_as::<_, Self>("SELECT id, name, active, command FROM builder WHERE id=$1")
                .bind(uuid::Uuid::parse_str(key["id"].as_str().unwrap()).unwrap())
        ).await
    }

    async fn update(&self) -> Result<(), error::Error> {
        let qx = Self::query_ctx().await;
        qx.execute(
            sqlx::query("UPDATE builder SET name=$1, active=$2, command=$3 WHERE id=$4")
            .bind(self.name.clone())
            .bind(self.active)
            .bind(self.command.clone())
            .bind(self.id)
        ).await
    }

    async fn delete(&self) -> Result<(), error::Error> {
        let qx = Self::query_ctx().await;
        qx.execute(
            sqlx::query("DELETE FROM builder WHERE id = $1")
            .bind(self.id)
        ).await
    }
}

impl Builder {

    pub fn new(id: uuid::Uuid, name: &str, active: bool, command: &str) -> Self {
        Self {
            id,
            name: String::from(name),
            active,
            command: String::from(command)
        }
    }

    pub async fn search_name(name: &str) -> Result<Box<Self>, error::Error> {
        let qx = Self::query_ctx().await;
        qx.fetch_one(
            sqlx::query_as::<_, Self>("SELECT  id, name, active, command FROM builder WHERE name=$1")
            .bind(name)
        ).await
    }
}
