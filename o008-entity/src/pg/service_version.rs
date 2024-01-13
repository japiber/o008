use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::Postgres;
use utoipa::ToSchema;
use uuid::Uuid;
use o008_common::{AsyncFrom, TypeInfo};
use o008_dal::{DalError, DaoCommand, DaoQuery};
use o008_dal::pg::PgDao;
use crate::{Builder, DestroyEntity, Entity, EntityError, PersistEntity, QueryEntity, Service};
use crate::pg::RepoReference;

type ServiceVersionDao = o008_dal::pg::ServiceVersion;

#[derive(Debug, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct ServiceVersion {
    #[serde(rename(serialize = "_id", deserialize = "id"))]
    id: Uuid,
    version: String,
    service: Service,
    repo_ref: RepoReference,
    builder: Builder,
}

impl ServiceVersion {
    pub fn new(version: &str, service: Service, repo_ref: RepoReference, builder: Builder) -> Self {
        Self {
            id: Uuid::nil(),
            version: String::from(version),
            service,
            repo_ref,
            builder,
        }
    }

    pub fn load(id: Uuid, version: &str, service: Service, repo_ref: RepoReference, builder: Builder) -> Self {
        Self {
            id,
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

    pub fn service(&self) -> &Service {
        &self.service
    }

    pub fn repo_ref(&self) -> &RepoReference {
        &self.repo_ref
    }

    pub fn builder(&self) -> &Builder {
        &self.builder
    }
}

impl Entity<ServiceVersionDao> for ServiceVersion {
    fn dao(&self) -> Box<ServiceVersionDao> {
        Box::new(ServiceVersionDao::new(self.id, self.version.as_str(), self.service.id(), self.repo_ref.id(), self.builder.id()))
    }
}

#[async_trait]
impl QueryEntity<ServiceVersionDao, PgDao, Postgres> for ServiceVersion {
    async fn read(qry: Value) -> Result<Box<Self>, EntityError> {
        match ServiceVersionDao::read(qry).await {
            Ok(sv) => Ok(Box::new(AsyncFrom::<ServiceVersionDao>::from(*sv).await)),
            Err(e) => match e {
                DalError::InvalidKey(_) => Err(EntityError::WrongQuery(format!("{}: {}", Self::type_name(), e))),
                _ => Err(EntityError::NotFound(format!("{}: {}", Self::type_name(), e))),
            }
        }
    }

    async fn persisted(qry: Value) -> bool {
        ServiceVersionDao::exists(qry).await
    }
}

#[async_trait]
impl PersistEntity<ServiceVersionDao, PgDao, Postgres> for ServiceVersion {
    async fn persist(&self) -> Result<Box<Self>, EntityError> {
        let dao = self.dao();
        let r = if self.id.is_nil() {
            dao.insert().await
        } else {
            dao.update().await
        };
        match r {
            Ok(_) => Ok(Box::new(Self {
                id: dao.id(),
                version: String::from(self.version.as_str()),
                service: self.service.clone(),
                repo_ref: self.repo_ref.clone(),
                builder: self.builder.clone(),
            })),
            Err(e) => Err(EntityError::Persist(e))
        }
    }
}

#[async_trait]
impl DestroyEntity<ServiceVersionDao, PgDao, Postgres> for ServiceVersion {
    async fn destroy(&self) -> Result<(), EntityError> {
        if self.id.is_nil() {
            Err(EntityError::UnPersisted(String::from(self.type_of())))
        } else {
            match self.dao().delete().await {
                Ok(_) => Ok(()),
                Err(e) => Err(EntityError::Destroy(e))
            }
        }
    }
}

const SERVICE_VERSION_TYPE_INFO: &str = "ServiceVersion";

impl TypeInfo for ServiceVersion {
    fn type_name() -> &'static str {
        SERVICE_VERSION_TYPE_INFO
    }

    fn type_of(&self) -> &'static str {
        SERVICE_VERSION_TYPE_INFO
    }
}

#[async_trait]
impl AsyncFrom<ServiceVersionDao> for ServiceVersion {
    async fn from(value: ServiceVersionDao) -> Self {
        let service = Service::read(json!({"id": value.service()})).await.unwrap();
        let repo_ref = RepoReference::read(json!({"id": value.repo_ref()})).await.unwrap();
        let builder = Builder::read(json!({"id": value.builder().to_string()})).await.unwrap();
        Self::load(value.id(), value.version(), *service, *repo_ref, *builder)
    }
}