use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::Postgres;
use uuid::Uuid;
use crate::{CommandContext, DalCount, DalError, DaoCommand, DaoQuery, QueryContext};
use crate::pg::{Application, hard_check_key, PgPool, soft_check_key};


#[derive(Debug, PartialEq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Service {
    id: Uuid,
    name: String,
    original_name: String,
    application: Uuid,
    default_repo: String,
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
}

#[async_trait]
impl DaoQuery<PgPool, Postgres> for Service {
    async fn read(key: Value) -> Result<Box<Self>, DalError> {
        let id_key= soft_check_key(&key, &["id"])?;
        return if let Some(id) = id_key.first().unwrap() {
            Self::query_ctx().await.fetch_one(
                sqlx::query_as::<_, Self>("SELECT id, name, original_name, application, default_repo FROM service WHERE id=$1")
                    .bind(Uuid::parse_str(id.as_str().unwrap()).unwrap())
            ).await
        } else {
            let name_app_key = hard_check_key(&key, &["name", "application"])?;
            let name = name_app_key.get(0).unwrap().as_str().unwrap();
            let app_qry = name_app_key.get(1).unwrap();
            if let Ok(app) = Application::read(app_qry.clone()).await {
                Self::query_ctx().await.fetch_one(
                    sqlx::query_as::<_, Self>("SELECT id, name, original_name, application, default_repo FROM service WHERE name=$1 AND application=$2")
                        .bind(name)
                        .bind(app.id())
                ).await
            } else {
                Err(DalError::DataNotFound(format!("application {}", app_qry)))
            }
        }
    }

    async fn exists(key: Value) -> bool {
        if let Ok(id_key) = soft_check_key(&key, &["id"]) {
            if let Some(id) = id_key.first().unwrap() {
                let r = Self::query_ctx().await.fetch_one(
                    sqlx::query_as::<_, DalCount>("SELECT COUNT(*) AS count FROM service WHERE id=$1")
                        .bind(Uuid::parse_str(id.as_str().unwrap()).unwrap())
                ).await;
                return r.unwrap().count > 0
            } else if let Ok(name_app_key) = soft_check_key(&key, &["name", "application"]) {
                if let (Some(name), Some(app_qry)) = (name_app_key.get(0).unwrap(), name_app_key.get(1).unwrap()) {
                    if let Ok(app) = Application::read(app_qry.clone()).await {
                        let r = Self::query_ctx().await.fetch_one(
                            sqlx::query_as::<_, DalCount>("SELECT COUNT(*) AS count FROM service WHERE name=$1 AND application=$2")
                                .bind(name.as_str().unwrap())
                                .bind(app.id())
                        ).await;
                        return r.unwrap().count > 0
                    }
                }
            }
        }
        false
    }
}

#[async_trait]
impl DaoCommand<PgPool, Postgres> for Service {
    async fn insert(&self) -> Result<(), DalError> {
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
