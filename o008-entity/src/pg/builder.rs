use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::Postgres;
use utoipa::ToSchema;
use uuid::Uuid;
use o008_dal::{DalError, DaoCommand, DaoQuery};
use crate::{DestroyEntity, Entity, EntityError, PersistEntity, QueryEntity};
use o008_common::{AsyncFrom, BuilderRequest};
use o008_dal::pg::{PgDao};

type BuilderDao = o008_dal::pg::Builder;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, ToSchema)]
pub struct Builder {
    #[serde(rename(serialize = "_id", deserialize = "id"))]
    id: Uuid,
    name: String,
    active: bool,
    build_command: String,
}

impl Builder {
    pub fn new(name: &str, active: bool, build_command: &str) -> Self {
        Self {
            id: Uuid::nil(),
            name: String::from(name),
            active,
            build_command: String::from(build_command)
        }
    }

    pub fn load(id: Uuid, name: &str, active: bool, build_command: &str) -> Self {
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
        self.name.as_str()
    }

    pub fn active(&self) -> bool {
        self.active
    }

    pub fn build_command(&self) -> &str {
        self.build_command.as_str()
    }
}

#[async_trait]
impl Entity<BuilderDao> for Builder {
    fn dao(&self) -> Box<BuilderDao> {
        Box::new(BuilderDao::new(self.id, &self.name, self.active, &self.build_command))
    }
}

#[async_trait]
impl QueryEntity<BuilderDao, PgDao, Postgres> for Builder {
    async fn read(qry: Value) -> Result<Box<Self>, EntityError> {
        match BuilderDao::read(qry).await {
            Ok(b) => Ok(Box::new(From::<BuilderDao>::from(*b))),
            Err(e) => match e {
                DalError::InvalidKey(_) => Err(EntityError::WrongQuery(e.to_string())),
                _ => Err(EntityError::NotFound(e.to_string())),
            }
        }
    }

    async fn persisted(qry: Value) -> bool {
        BuilderDao::exists(qry).await
    }
}

#[async_trait]
impl PersistEntity<BuilderDao, PgDao, Postgres> for Builder {
    async fn persist(&self) -> Result<Box<Self>, EntityError> {
        let dao = self.dao();
        let r = if self.id.is_nil() {
            dao.insert().await
        } else {
            dao.update().await
        };
        match r {
            Ok(_) => {
                Ok(Box::new(Self {
                    id: dao.id(),
                    name: String::from(&self.name),
                    active: self.active,
                    build_command: String::from(&self.build_command),
                }))
            },
            Err(e) => Err(EntityError::Persist(e))
        }
    }
}

#[async_trait]
impl DestroyEntity<BuilderDao, PgDao, Postgres> for Builder {
    async fn destroy(&self) -> Result<(), EntityError> {
        if self.id.is_nil() {
            Err(EntityError::UnPersisted(String::from("builder")))
        } else {
            match self.dao().delete().await {
                Ok(_) => Ok(()),
                Err(e) => Err(EntityError::Destroy(e))
            }
        }
    }
}

impl From<BuilderDao> for Builder {
    fn from(value: BuilderDao) -> Self {
        Self::load(value.id(), value.name(), value.active(), value.build_command())
    }
}

impl From<BuilderRequest> for Builder {
    fn from(value: BuilderRequest) -> Self {
        Self::new(value.name(), value.active(), value.build_command())
    }
}

#[async_trait]
impl AsyncFrom<BuilderRequest> for Builder {
    async fn from(value: BuilderRequest) -> Self {
        *Builder::read(serde_json::to_value(value).unwrap()).await.unwrap()
    }
}
