use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::Postgres;
use tracing::info;
use uuid::Uuid;
use o008_dal::DaoCommand;
use o008_dal::pg::{PgCommandContext, PgQueryContext};
use crate::{Entity, EntityError};

type BuilderDao = o008_dal::pg::Builder;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Builder {
    pub id: Uuid,
    pub name: String,
    pub active: bool,
    pub command: String,
}

#[async_trait]
impl<'q> Entity<BuilderDao, PgQueryContext<'q>, PgCommandContext<'q>, Postgres> for Builder {
    async fn persist(&self) -> Result<Box<Self>, EntityError> {
        let dao = self.dao();
        let r = if self.id.is_nil() {
            dao.create().await
        } else {
            dao.update().await
        };
        match r {
            Ok(_) => {
                Ok(Box::new(Self {
                    id: dao.id(),
                    name: String::from(&self.name),
                    active: self.active,
                    command: String::from(&self.command),
                }))
            },
            Err(e) => Err(EntityError::Persist(e))
        }
    }

    async fn destroy(&self) -> Result<(), EntityError> {
        if self.id.is_nil() {
            Err(EntityError::UnPersisted(String::from("cannot destroy not previously persisted builder")))
        } else {
            match self.dao().delete().await {
                Ok(_) => Ok(()),
                Err(e) => Err(EntityError::Destroy(e))
            }
        }
    }

    fn dao(&self) -> Box<BuilderDao> {
        if self.id.is_nil() {
            Box::new(BuilderDao::new(Uuid::new_v4(), &self.name, self.active, &self.command))
        } else {
            Box::new(BuilderDao::new(self.id, &self.name, self.active, &self.command))
        }
    }
}

impl From<BuilderDao> for Builder {
    fn from(b: BuilderDao) -> Self {
        Self {
            id: b.id(),
            name: String::from(b.name()),
            active: b.active(),
            command: String::from(b.command())

        }
    }
}

impl Builder {
    pub fn new(name: &str, active: bool, command: &str) -> Self {
        Self {
            id: Uuid::nil(),
            name: String::from(name),
            active,
            command: String::from(command)
        }
    }

    #[tracing::instrument]
    pub async fn get_by_name(name: &str) -> Option<Builder> {
        let dao = BuilderDao::search_name(name).await;
        match dao {
            Ok(b) => {
                let bb = *b;
                Some(From::<BuilderDao>::from(bb))
            },
            Err(e) => {
                info!("get builder by name '{}': {}", name, e);
                None
            }
        }
    }
}
