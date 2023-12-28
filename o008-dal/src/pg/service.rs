use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::Postgres;
use uuid::Uuid;
use crate::{CommandContext, DalError, DaoCommand, DaoQuery, QueryContext};
use crate::pg::{PgCommandContext, PgQueryContext};


#[derive(Debug, PartialEq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Service {
    id: Uuid,
    name: String,
    original_name: String,
    application: Uuid,
    default_repo: String,
}

#[async_trait]
impl<'q> DaoQuery<PgQueryContext<'q>, Postgres> for Service {
    async fn read(key: Value) -> Result<Box<Self>, DalError> {
        Self::query_ctx().await.fetch_one(
            sqlx::query_as::<_, Self>("SELECT id, name, original_name, application, default_repo FROM service WHERE id=$1")
                .bind(Uuid::parse_str(key["id"].as_str().unwrap()).unwrap())
        ).await
    }
}

#[async_trait]
impl<'q> DaoCommand<PgCommandContext<'q>, Postgres> for Service {
    async fn create(&self) -> Result<(), DalError> {
        let cx = Self::command_ctx().await;
        cx.execute(
            sqlx::query("INSERT INTO service(id, name, original_name, application, default_repo) VALUES ($1, $2, $3, $4, $5)")
                .bind(self.id)
                .bind(self.name.as_str())
                .bind(self.original_name.as_str())
                .bind(self.application)
                .bind(self.default_repo.as_str())
        ).await
    }

    async fn update(&self) -> Result<(), DalError> {
        let cx = Self::command_ctx().await;
        cx.execute(
            sqlx::query("UPDATE service SET name=$1, original_name=$2, application=$3, default_repo=$4 WHERE id=$5")
                .bind(self.name.as_str())
                .bind(self.original_name.as_str())
                .bind(self.application)
                .bind(self.default_repo.as_str())
                .bind(self.id)
        ).await
    }

    async fn delete(&self) -> Result<(), DalError> {
        let cx = Self::command_ctx().await;
        cx.execute(
            sqlx::query("DELETE FROM service WHERE id = $1")
                .bind(self.id)
        ).await
    }
}

impl Service {
    pub fn new(id: Uuid, name: &str, original_name: &str, application: Uuid, default_repo: &str) -> Self {
        Self {
            id,
            name: String::from(name),
            original_name: String::from(original_name),
            application,
            default_repo: String::from(default_repo)
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn original_name(&self) -> &str {
        &self.original_name
    }

    pub fn application(&self) -> Uuid {
        self.application
    }

    pub fn default_repo(&self) -> &str {
        &self.default_repo
    }

    pub async fn search_name_application(name: &str, app: Uuid)  -> Result<Box<Self>, DalError> {
        let qx = Self::query_ctx().await;
        qx.fetch_one(
            sqlx::query_as::<_, Self>("SELECT id, name, original_name, application, default_repo FROM service WHERE name=$1 AND application=$2")
                .bind(name)
                .bind(app)
        ).await
    }
}