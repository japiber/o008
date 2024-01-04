use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::Postgres;
use uuid::Uuid;
use crate::{QueryContext, error, CommandContext, DaoQuery, DaoCommand, DalCount};
use crate::pg::{hard_check_key, PgPool, soft_check_key};


#[derive(Debug, PartialEq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Builder {
    id: Uuid,
    name: String,
    active: bool,
    build_command: String,
}


#[async_trait]
impl DaoQuery<PgPool, Postgres> for Builder {
    async fn read(key: Value) -> Result<Box<Self>, error::DalError> {
        let id_key = soft_check_key(&key, &["id"])?;
        return if let Some(id) = id_key.first().unwrap() {
            Self::query_ctx().await.fetch_one(
                sqlx::query_as::<_, Self>("SELECT id, name, active, build_command FROM builder WHERE id=$1")
                    .bind(Uuid::parse_str(id.as_str().unwrap()).unwrap())
            ).await
        } else {
            let name_key = hard_check_key(&key, &["name"])?;
            let name = name_key.first().unwrap().as_str().unwrap();
            Self::query_ctx().await.fetch_one(
                sqlx::query_as::<_, Self>("SELECT  id, name, active, build_command FROM builder WHERE name=$1")
                    .bind(name)
            ).await
        }
    }

    async fn exists(key: Value) -> bool {
        if let Ok(id_key) = soft_check_key(&key, &["id"]) {
            if let Some(id) = id_key.first().unwrap() {
                let r = Self::query_ctx().await.fetch_one(
                    sqlx::query_as::<_, DalCount>("SELECT COUNT(*) as count FROM builder WHERE id=$1")
                        .bind(Uuid::parse_str(id.as_str().unwrap()).unwrap())
                ).await;
                return r.unwrap().count > 0
            } else if let Ok(name_key) = soft_check_key(&key, &["name"]) {
                if let Some(name) = name_key.first().unwrap() {
                    let r = Self::query_ctx().await.fetch_one(
                        sqlx::query_as::<_, DalCount>("SELECT COUNT(*) as count FROM builder WHERE name=$1")
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
impl DaoCommand<PgPool, Postgres> for Builder {
    async fn insert(&self) -> Result<(), error::DalError> {
        let cx = Self::command_ctx().await;
        cx.execute(
            sqlx::query("INSERT INTO builder (id, name, active, build_command) VALUES ($1, $2, $3, $4)")
                .bind(self.id)
                .bind(self.name.clone())
                .bind(self.active)
                .bind(self.build_command.clone())
        ).await
    }
    async fn update(&self) -> Result<(), error::DalError> {
        let cx = Self::command_ctx().await;
        cx.execute(
            sqlx::query("UPDATE builder SET name=$1, active=$2, build_command=$3 WHERE id=$4")
                .bind(self.name.clone())
                .bind(self.active)
                .bind(self.build_command.clone())
                .bind(self.id)
        ).await
    }

    async fn delete(&self) -> Result<(), error::DalError> {
        let cx = Self::command_ctx().await;
        cx.execute(
            sqlx::query("DELETE FROM builder WHERE id = $1")
                .bind(self.id)
        ).await
    }

}

impl Builder {
    pub fn new(id: Uuid, name: &str, active: bool, build_command: &str) -> Self {
        Self {
            id,
            name: String::from(name),
            active,
            build_command: String::from(build_command)
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn active(&self) -> bool {
        self.active
    }

    pub fn build_command(&self) -> &str {
        &self.build_command
    }

}
