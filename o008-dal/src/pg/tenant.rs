use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::Postgres;
use uuid::Uuid;
use crate::{QueryContext, error, CommandContext, DaoCommand, DaoQuery, DalCount};
use crate::pg::{hard_check_key, PgPool, soft_check_key};


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
}

#[async_trait]
impl DaoQuery<PgPool, Postgres> for Tenant {
    async fn read(key: Value) -> Result<Box<Self>, error::DalError> {
        if let Ok(id_key) = hard_check_key(&key, &["id"]) {
            let id = id_key.first().unwrap();
            Self::query_ctx().await.fetch_one(
                sqlx::query_as::<_, Self>("SELECT id, name, coexisting FROM tenant WHERE id=$1")
                    .bind(Uuid::parse_str(id.as_str().unwrap()).unwrap())
            ).await
        } else {
            let name_key = hard_check_key(&key, &["name"])?;
            let name = name_key.first().unwrap().as_str().unwrap();
            Self::query_ctx().await.fetch_one(
                sqlx::query_as::<_, Self>("SELECT id, name, coexisting FROM tenant WHERE name=$1")
                    .bind(name)
            ).await
        }
    }

    async fn exists(key: Value) -> bool {
        if let Ok(id_key) = soft_check_key(&key, &["id"]) {
            if let Some(id) = id_key.first().unwrap() {
                let r = Self::query_ctx().await.fetch_one(
                    sqlx::query_as::<_, DalCount>("SELECT COUNT(*) AS count FROM tenant WHERE id=$1")
                        .bind(Uuid::parse_str(id.as_str().unwrap()).unwrap())
                ).await;
                return r.unwrap().count > 0
            } else if let Ok(name_key) = soft_check_key(&key, &["name"]) {
                if let Some(name) = name_key.first().unwrap() {
                    let r = Self::query_ctx().await.fetch_one(
                        sqlx::query_as::<_, DalCount>("SELECT COUNT(*) AS count FROM tenant WHERE name=$1")
                            .bind(name.as_str().unwrap())
                    ).await;
                    return r.unwrap().count > 0
                }
            }
        }
        false
    }
}

#[async_trait]
impl DaoCommand<PgPool, Postgres> for Tenant {

    async fn insert(&self) -> Result<(), error::DalError> {
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
