use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::Postgres;
use uuid::Uuid;
use o008_common::RepoReferenceKind;
use crate::{CommandContext, DalCount, DalError, DaoCommand, DaoQuery, gen_v7_uuid, QueryContext};
use crate::pg::{hard_check_key, PgDao};


#[derive(Debug, PartialEq, Serialize, Deserialize, sqlx::FromRow)]
pub struct RepoReference {
    id: Uuid,
    repo: String,
    kind: RepoReferenceKind,
    reference: String
}

impl RepoReference {
    pub fn new(id: Uuid, repo: &str, kind: RepoReferenceKind, reference: &str) -> Self {
        Self {
            id: gen_v7_uuid(id),
            repo: String::from(repo),
            kind,
            reference: String::from(reference),
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn repo(&self) -> &str {
        self.repo.as_str()
    }

    pub fn kind(&self) -> RepoReferenceKind {
        self.kind
    }

    pub fn reference(&self) -> &str {
        self.reference.as_str()
    }
}

#[async_trait]
impl DaoQuery<PgDao, Postgres> for RepoReference {
    async fn read(key: Value) -> Result<Box<Self>, DalError> {
        if let Ok(id_key) = hard_check_key(&key, &["id"]) {
            let id = id_key.first().unwrap();
            Self::query_ctx().await.fetch_one(
                sqlx::query_as::<_, Self>("SELECT id, repo, kind, reference FROM repo_reference WHERE id=$1")
                    .bind(Uuid::parse_str(id.as_str().unwrap()).unwrap())
            ).await
        } else if let Ok(rkr_key) = hard_check_key(&key, &["repo", "kind", "reference"]) {
            let (repo, kind, reference) = (rkr_key.get(0).unwrap(), rkr_key.get(1).unwrap(), rkr_key.get(2).unwrap());
            Self::query_ctx().await.fetch_one(
                sqlx::query_as::<_, Self>("SELECT id, repo, kind, reference FROM repo_reference WHERE repo=$1 AND kind::text=$2 AND reference=$3")
                    .bind(repo.as_str().unwrap())
                    .bind(kind.as_str().unwrap())
                    .bind(reference.as_str().unwrap())
            ).await
        } else {
            Err(DalError::InvalidKey("(id) or (repo/kind/reference) keys expected".to_string()))
        }
    }

    async fn exists(key: Value) -> bool {
        return if let Ok(id_key) = hard_check_key(&key, &["id"]) {
            let id = id_key.first().unwrap();
            let qr = Self::query_ctx().await.fetch_one(
                sqlx::query_as::<_, DalCount>("SELECT COUNT(*) as count FROM repo_reference WHERE id=$1")
                    .bind(Uuid::parse_str(id.as_str().unwrap()).unwrap())
            ).await;
            qr.unwrap().count > 0
        } else if let Ok(rkr_key) = hard_check_key(&key, &["repo", "kind", "reference"]) {
            let (repo, kind, reference) = (rkr_key.first().unwrap(), rkr_key.get(1).unwrap(), rkr_key.get(2).unwrap());
            let qr = Self::query_ctx().await.fetch_one(
                sqlx::query_as::<_, DalCount>("SELECT COUNT(*) as count  FROM repo_reference WHERE repo=$1 AND kind::text=$2 AND reference=$3")
                    .bind(repo.as_str().unwrap())
                    .bind(kind.as_str().unwrap())
                    .bind(reference.as_str().unwrap())
            ).await;
            qr.unwrap().count > 0
        } else {
            false
        }
    }
}

#[async_trait]
impl DaoCommand<PgDao, Postgres> for RepoReference {
    async fn insert(&self) -> Result<(), DalError> {
        Self::command_ctx().await.execute(
            sqlx::query("INSERT INTO repo_reference (id, repo, kind, reference) VALUES ($1, $2, $3, $4)")
                .bind(self.id)
                .bind(self.repo.as_str())
                .bind(self.kind)
                .bind(self.reference.as_str())
        ).await
    }

    async fn update(&self) -> Result<(), DalError> {
        Self::command_ctx().await.execute(
            sqlx::query("UPDATE repo_reference repo = $1, kind = $2, reference = $3 WHERE id = $4")
                .bind(self.repo.as_str())
                .bind(self.kind)
                .bind(self.reference.as_str())
                .bind(self.id)
        ).await
    }

    async fn delete(&self) -> Result<(), DalError> {
        Self::command_ctx().await.execute(
            sqlx::query("DELETE FROM repo_reference WHERE id = $1")
                .bind(self.id)
        ).await
    }
}
