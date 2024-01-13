use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::Postgres;
use uuid::Uuid;
use crate::{CommandContext, DalCount, DalError, DaoCommand, DaoQuery, gen_v7_uuid, QueryContext};
use crate::pg::{Application, hard_check_key, PgDao};


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
            id: gen_v7_uuid(id),
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
impl DaoQuery<PgDao, Postgres> for Service {
    async fn read(key: Value) -> Result<Box<Self>, DalError> {
        match hard_check_key(&key, &["id"]) {
            Ok(id_key) => {
                let id = id_key.first().unwrap();
                Self::query_ctx().await.fetch_one(
                    sqlx::query_as::<_, Self>("SELECT id, name, original_name, application, default_repo FROM service WHERE id=$1")
                        .bind(Uuid::parse_str(id.as_str().unwrap()).unwrap())
                ).await
            },
            Err(_) => match hard_check_key(&key, &["name", "application"]) {
                Ok(name_app_key) => {
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
                Err(e) => Err(DalError::InvalidKey(format!("service dao read {}", e)))
            }
        }
    }

    #[tracing::instrument]
    async fn exists(key: Value) -> bool {
        if let Ok(id_key) = hard_check_key(&key, &["id"]) {
            let id = id_key.first().unwrap();
            let r = Self::query_ctx().await.fetch_one(
                sqlx::query_as::<_, DalCount>("SELECT COUNT(*) AS count FROM service WHERE id=$1")
                    .bind(Uuid::parse_str(id.as_str().unwrap()).unwrap())
            ).await;
            r.unwrap().count > 0
        } else if let Ok(name_app_key) = hard_check_key(&key, &["name", "application"]) {
            let (name, app_qry) = (name_app_key.get(0).unwrap(), name_app_key.get(1).unwrap());
            if let Ok(app) = Application::read(app_qry.clone()).await {
                let r = Self::query_ctx().await.fetch_one(
                    sqlx::query_as::<_, DalCount>("SELECT COUNT(*) AS count FROM service WHERE name=$1 AND application=$2")
                        .bind(name.as_str().unwrap())
                        .bind(app.id())
                ).await;
                r.unwrap().count > 0
            } else {
                false
            }
        } else {
            false
        }
    }
}

#[async_trait]
impl DaoCommand<PgDao, Postgres> for Service {
    async fn insert(&self) -> Result<(), DalError> {
        Self::command_ctx().await.execute(
            sqlx::query("INSERT INTO service(id, name, original_name, application, default_repo) VALUES ($1, $2, $3, $4, $5)")
                .bind(self.id)
                .bind(self.name.as_str())
                .bind(self.original_name.as_str())
                .bind(self.application)
                .bind(self.default_repo.as_str())
        ).await
    }

    async fn update(&self) -> Result<(), DalError> {
        Self::command_ctx().await.execute(
            sqlx::query("UPDATE service SET name=$1, original_name=$2, application=$3, default_repo=$4 WHERE id=$5")
                .bind(self.name.as_str())
                .bind(self.original_name.as_str())
                .bind(self.application)
                .bind(self.default_repo.as_str())
                .bind(self.id)
        ).await
    }

    async fn delete(&self) -> Result<(), DalError> {
        Self::command_ctx().await.execute(
            sqlx::query("DELETE FROM service WHERE id=$1")
                .bind(self.id)
        ).await
    }
}
