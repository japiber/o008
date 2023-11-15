use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::Postgres;
use tracing::info;
use o008_dal::{Dao, error};
use o008_dal::error::Error;
use o008_dal::pg::PgQueryContext;
use crate::Entity;

type BuilderDao = o008_dal::pg::Builder;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct BuilderInner {
    pub name: String,
    pub active: bool,
    pub command: String,
}

pub struct Builder(BuilderInner, Option<BuilderDao>);

#[async_trait]
impl<'q> Entity<BuilderInner, BuilderDao, PgQueryContext<'q>, Postgres> for Builder {
    async fn persist(&mut self) -> Result<(), error::Error> {
        let dao = self.dao();
        let p = match self.1 {
            None => (&dao).create().await,
            Some(_) => (&dao).update().await
        };
        self.1 = Some(*dao);
        p
    }

    fn dao(&self) -> Box<BuilderDao> {
        match &self.1 {
            None => Box::new(BuilderDao {
                id: uuid::Uuid::new_v4(),
                name: String::from(self.name()),
                active: self.active(),
                command: String::from(self.command())
            }),
            Some(d) => Box::new(BuilderDao {
                id: d.id,
                name: String::from(self.name()),
                active: self.active(),
                command: String::from(self.command())
            })
        }
    }

    fn inner(&self) -> Box<BuilderInner> {
        Box::new(self.0.clone())
    }
}

impl From<BuilderDao> for Builder {
    fn from(b: BuilderDao) -> Self {
        Self (
            BuilderInner::new(&b.name, b.active, &b.command),
            Some(b)
        )
    }
}

impl BuilderInner {
    pub fn new(name: &str, active: bool, command: &str) -> Self {
        Self {
            name: String::from(name),
            active,
            command: String::from(command)
        }
    }
}


impl Builder {
    pub fn new(name: &str, active: bool, command: &str) -> Self {
        Self(
            BuilderInner::new(name, active, command),
            None
        )
    }

    pub fn name(&self) -> &str {
        &self.0.name
    }

    pub fn active(&self) -> bool {
        self.0.active
    }

    pub fn command(&self) -> &str {
        &self.0.command
    }

    #[tracing::instrument]
    pub async fn search_name(name: &str) -> Option<Builder> {
        let dao = BuilderDao::search_name(name).await;
        match dao {
            Ok(b) => {
                let bb = *b;
                Some(From::<BuilderDao>::from(bb))
            },
            Err(e) => {
                info!("builder name {} not found: {}", name, e);
                None
            }
        }
    }

    pub async fn destroy(&mut self) -> Result<(), error::Error> {
        let p = match &self.1 {
            None => Err(Error::DataGenericError(String::from("cannnot destroy not persisted builder"))),
            Some(dao) => dao.delete().await
        };
        self.1 = None;
        p
    }
}
