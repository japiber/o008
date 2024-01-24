use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{to_value, Value};
use sqlx::Postgres;
use uuid::Uuid;
use crate::{CommandContext, DalCount, DalError, DaoCommand, DaoQuery, gen_v7_uuid, QueryContext};
use crate::pg::{hard_check_key, PgDao, Service};

#[derive(Debug, PartialEq, Serialize, Deserialize, sqlx::FromRow)]
pub struct ServiceVersion {
    id: Uuid,
    version: String,
    service: Uuid,
    repo_ref: Uuid,
    builder: Uuid,
}

impl ServiceVersion {
    pub fn new(id: Uuid, version: &str, service: Uuid, repo_ref: Uuid, builder: Uuid) -> Self {
        Self {
            id: gen_v7_uuid(id),
            version: String::from(version),
            service,
            repo_ref,
            builder,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn version(&self) -> &str {
        self.version.as_str()
    }

    pub fn service(&self) -> Uuid {
        self.service
    }

    pub fn repo_ref(&self) -> Uuid {
        self.repo_ref
    }

    pub fn builder(&self) -> Uuid {
        self.builder
    }

    pub async fn service_versions(key: Value) -> Result<Vec<Self>, DalError> {
        match hard_check_key(&key, &["service"]) {
            Ok(service_key) => {
                let id = service_key.first().unwrap();
                Self::query_ctx().await.fetch_all(
                    sqlx::query_as::<_, Self>("SELECT id, version, service, repo_ref, builder FROM service_version WHERE service=$1")
                        .bind(Uuid::parse_str(id.as_str().unwrap()).unwrap())
                ).await

            }
            Err(e) => Err(DalError::InvalidKey(format!("service version dao read {}", e)))
        }
    }
}

#[async_trait]
impl DaoQuery<PgDao, Postgres> for ServiceVersion {
    async fn read(key: Value) -> Result<Box<Self>, DalError> {
        match hard_check_key(&key, &["id"]) {
            Ok(id_key) => {
                let id = id_key.first().unwrap();
                Self::query_ctx().await.fetch_one(
                    sqlx::query_as::<_, Self>("SELECT id, version, service, repo_ref, builder FROM service_version WHERE id=$1")
                        .bind(Uuid::parse_str(id.as_str().unwrap()).unwrap())
                ).await
            },
            Err(_) => match hard_check_key(&key, &["version", "service"]) {
                Ok(version_service_key) => {
                    let version = version_service_key.first().unwrap();
                    let service_qry = version_service_key.get(1).unwrap();
                    if let Ok(srv) = Service::read(to_value(service_qry).unwrap()).await {
                        Self::query_ctx().await.fetch_one(
                            sqlx::query_as::<_, Self>("SELECT id, version, service, repo_ref, builder FROM service_version WHERE version=$1 AND service=$2")
                                .bind(version.as_str().unwrap())
                                .bind(srv.id())
                        ).await
                    } else {
                        Err(DalError::DataNotFound(format!("service {}", service_qry)))
                    }
                }
                Err(e) => Err(DalError::InvalidKey(format!("service version dao read {}", e)))
            }
        }
    }

    async fn exists(key: Value) -> bool {
        if let Ok(id_key) = hard_check_key(&key, &["id"]) {
            let id = id_key.first().unwrap();
            let r = Self::query_ctx().await.fetch_one(
                sqlx::query_as::<_, DalCount>("SELECT COUNT(*) as count FROM service_version WHERE id=$1")
                    .bind(Uuid::parse_str(id.as_str().unwrap()).unwrap())
            ).await;
            r.unwrap().count > 0
        } else if let Ok(version_service_key) = hard_check_key(&key, &["version", "service"]) {
            let version = version_service_key.first().unwrap();
            let service_qry = version_service_key.get(1).unwrap();
            if let Ok(srv) = Service::read(to_value(service_qry).unwrap()).await {
                let r = Self::query_ctx().await.fetch_one(
                    sqlx::query_as::<_, DalCount>("SELECT COUNT(*) as count FROM service_version WHERE version=$1 AND service=$2")
                        .bind(version.as_str().unwrap())
                        .bind(srv.id())
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
impl DaoCommand<PgDao, Postgres> for ServiceVersion {
    async fn insert(&self) -> Result<(), DalError> {
        Self::command_ctx().await.execute(
            sqlx::query("INSERT INTO service_version(id, version, service, repo_ref, builder) VALUES ($1, $2, $3, $4, $5)")
                .bind(self.id)
                .bind(self.version.as_str())
                .bind(self.service)
                .bind(self.repo_ref)
                .bind(self.builder)
        ).await
    }

    async fn update(&self) -> Result<(), DalError> {
        Self::command_ctx().await.execute(
            sqlx::query("UPDATE service_version SET version=$1, service=$2, repo_ref=$3, builder=$4 WHERE id=$5)")
                .bind(self.version.as_str())
                .bind(self.service)
                .bind(self.repo_ref)
                .bind(self.builder)
                .bind(self.id)
        ).await
    }

    async fn delete(&self) -> Result<(), DalError> {
        Self::command_ctx().await.execute(
            sqlx::query("DELETE FROM service_version WHERE id=$1")
                .bind(self.id)
        ).await
    }
}
