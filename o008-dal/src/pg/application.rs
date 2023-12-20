use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::Postgres;
use uuid::Uuid;
use crate::{CommandContext, DalError, DaoCommand, DaoQuery, QueryContext};
use crate::pg::{PgCommandContext, PgQueryContext};

#[derive(Debug, PartialEq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Application {
    pub id: uuid::Uuid,
    pub name: String,
    pub tenant: uuid::Uuid,
    pub class_unit: String,
}

#[async_trait]
impl<'q> DaoQuery<PgQueryContext<'q>, Postgres> for Application {
    async fn read(key: Value) -> Result<Box<Self>, DalError> {
        Self::query_ctx().await.fetch_one(
            sqlx::query_as::<_, Self>("SELECT id, name, tenant, class_unit FROM application WHERE id=$1")
                .bind(sqlx::types::Uuid::parse_str(key["id"].as_str().unwrap()).unwrap())
        ).await
    }
}

#[async_trait]
impl<'q> DaoCommand<PgCommandContext<'q>, Postgres> for Application {
    async fn create(&self) -> Result<(), DalError> {
        let cx = Self::command_ctx().await;
        cx.execute(
            sqlx::query("INSERT INTO application(id, name, tenant, class_unit) VALUES ($1, $2, $3, $4)")
                .bind(self.id)
                .bind(self.name.as_str())
                .bind(self.tenant)
                .bind(self.class_unit.as_str())
        ).await
    }

    async fn update(&self) -> Result<(), DalError> {
        let cx = Self::command_ctx().await;
        cx.execute(
            sqlx::query("UPDATE application SET name=$1, tenant=$2, class_unit=$3 WHERE id=$4")
                .bind(self.name.as_str())
                .bind(self.tenant)
                .bind(self.class_unit.as_str())
                .bind(self.id)
        ).await
    }

    async fn delete(&self) -> Result<(), DalError> {
        let cx = Self::command_ctx().await;
        cx.execute(
            sqlx::query("DELETE FROM application WHERE id = $1")
                .bind(self.id)
        ).await
    }
}

impl Application {
    pub fn new(id: Uuid, name: &str, tenant: Uuid, class: &str) -> Self {
        Self {
            id,
            name: String::from(name),
            tenant,
            class_unit: String::from(class)
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn tenant(&self) -> Uuid {
        self.id
    }

    pub fn class_unit(&self) -> &str {
        &self.class_unit
    }

    pub async fn search_name(name: &str) -> Result<Box<Self>, DalError> {
        let qx = Self::query_ctx().await;
        qx.fetch_one(
            sqlx::query_as::<_, Self>("SELECT id, name, tenant, class_unit FROM application WHERE name=$1")
                .bind(name)
        ).await
    }
}