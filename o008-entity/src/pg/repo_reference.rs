use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::Postgres;
use tracing::error;
use utoipa::ToSchema;
use uuid::Uuid;
use o008_common::{RepoReferenceKind, RepoReferenceRequest};
use o008_dal::{DalError, DaoCommand, DaoQuery};
use o008_dal::pg::{PgDao};
use crate::{DestroyEntity, Entity, EntityError, PersistEntity, QueryEntity};

pub type RepoReferenceDao = o008_dal::pg::RepoReference;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct RepoReference {
    #[serde(rename(serialize = "_id", deserialize = "id"))]
    id: Uuid,
    repo: String,
    kind: RepoReferenceKind,
    reference: String
}

impl RepoReference {
    pub fn new(repo: &str, kind: RepoReferenceKind, reference: &str) -> Self {
        Self {
            id: Uuid::nil(),
            repo: String::from(repo),
            kind,
            reference: String::from(reference),
        }
    }

    pub fn load(id: Uuid, repo: &str, kind: RepoReferenceKind, reference: &str) -> Self {
        Self {
            id,
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

impl Entity<RepoReferenceDao> for RepoReference {
    fn dao(&self) -> Box<RepoReferenceDao> {
        Box::new(RepoReferenceDao::new(self.id, &self.repo, self.kind, &self.reference))
    }
}

#[async_trait]
impl QueryEntity<RepoReferenceDao, PgDao, Postgres> for RepoReference {
    async fn read(qry: Value) -> Result<Box<Self>, EntityError> {
        match RepoReferenceDao::read(qry).await {
            Ok(rf) => Ok(Box::new(From::from(*rf))),
            Err(e) => match e {
                DalError::InvalidKey(_) => Err(EntityError::WrongQuery(e.to_string())),
                _ => {
                    error!("{}", e);
                    Err(EntityError::NotFound(e.to_string()))
                },
            }
        }
    }

    async fn persisted(qry: Value) -> bool {
        RepoReferenceDao::exists(qry).await
    }
}

#[async_trait]
impl PersistEntity<RepoReferenceDao, PgDao, Postgres> for RepoReference {
    async fn persist(&self) -> Result<Box<Self>, EntityError> {
        let dao = self.dao();
        let r = if self.id.is_nil() {
            dao.insert().await
        } else {
            dao.update().await
        };
        match r {
            Ok(_) => Ok(Box::new(Self::load(dao.id(), dao.repo(), dao.kind(), dao.reference()))),
            Err(e) => Err(EntityError::Persist(e))
        }
    }
}

#[async_trait]
impl DestroyEntity<RepoReferenceDao, PgDao, Postgres> for RepoReference {
    async fn destroy(&self) -> Result<(), EntityError> {
        if self.id.is_nil() {
            Err(EntityError::UnPersisted(String::from("repo reference")))
        } else {
            match self.dao().delete().await {
                Ok(_) => Ok(()),
                Err(e) => Err(EntityError::Destroy(e))
            }
        }
    }
}

impl From<RepoReferenceDao> for RepoReference {
    fn from(value: RepoReferenceDao) -> Self {
        Self::load(value.id(), value.repo(), value.kind(), value.reference())
    }
}

impl From<RepoReferenceRequest> for RepoReference {
    fn from(value: RepoReferenceRequest) -> Self {
        Self::new(
            value.repo().unwrap().as_str(),
            *value.kind().unwrap(),
            value.reference().unwrap().as_str(),
        )
    }
}