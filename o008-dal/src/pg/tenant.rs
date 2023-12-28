use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::Postgres;
use uuid::Uuid;
use crate::{QueryContext, error, CommandContext, DaoCommand, DaoQuery};
use crate::pg::{PgCommandContext, PgQueryContext};

#[derive(Debug, PartialEq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Tenant {
    id: Uuid,
    name: String,
    coexisting: bool,
}

impl Tenant {
    pub fn new(id: Uuid, name: &str, coexisting: bool) -> Self {
        Self {
            id,
            name: String::from(name),
            coexisting,
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

    pub async fn search_name(name: &str) -> Result<Box<Self>, error::DalError> {
        let qx = Self::query_ctx().await;
        qx.fetch_one(
            sqlx::query_as::<_, Self>("SELECT id, name, coexisting FROM tenant WHERE name=$1")
                .bind(name)
        ).await
    }
}

#[async_trait]
impl<'q> DaoQuery<PgQueryContext<'q>, Postgres> for Tenant {
    async fn read(key: serde_json::Value) -> Result<Box<Self>, error::DalError> {
        let qx = Self::query_ctx().await;
        qx.fetch_one(
            sqlx::query_as::<_, Self>("SELECT id, name, coexisting FROM tenant WHERE id=$1")
            .bind(Uuid::parse_str(key["id"].as_str().unwrap()).unwrap())
        ).await
    }
}

#[async_trait]
impl<'q> DaoCommand<PgCommandContext<'q>, Postgres> for Tenant {

    async fn create(&self) -> Result<(), error::DalError> {
        let cx = Self::command_ctx().await;
        cx.execute(
            sqlx::query("INSERT INTO tenant (id, name, coexisting) VALUES ($1, $2, $3)")
                .bind(self.id)
                .bind(self.name.clone())
                .bind(self.coexisting)
        ).await
    }

    async fn update(&self) -> Result<(), error::DalError> {
        let cx = Self::command_ctx().await;
        cx.execute(
            sqlx::query("UPDATE tenant SET name=$1, coexisting=$2 WHERE id=$3")
                .bind(self.name.clone())
                .bind(self.coexisting)
                .bind(self.id)
        ).await
    }

    async fn delete(&self) -> Result<(), error::DalError> {
        let cx = Self::command_ctx().await;
        cx.execute(
            sqlx::query("DELETE FROM tenant WHERE id = $1")
                .bind(self.id)
        ).await
    }
}
