use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::Postgres;
use uuid::Uuid;
use o008_dal::{DalError, DaoCommand, DaoQuery};
use crate::{DestroyEntity, Entity, EntityError, PersistEntity, QueryEntity};
use crate::pg::Application;
use crate::pg::application::ApplicationDao;
use utoipa::ToSchema;
use o008_common::{AsyncFrom, ServiceRequest};
use o008_dal::pg::{PgDao};

type ServiceDao = o008_dal::pg::Service;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct Service {
    #[serde(rename(serialize = "_id", deserialize = "id"))]
    id: Uuid,
    name: String,
    original_name: String,
    application: Application,
    default_repo: String,
}

impl Service {
    pub fn new(name: &str, app: Application, repo: &str) -> Self {
        Self {
            id: Uuid::nil(),
            name: String::from(name).to_lowercase(),
            original_name: String::from(name),
            application: app,
            default_repo: String::from(repo),
        }
    }

    pub fn load(id: Uuid, name: &str, original_name: &str, app: Application, repo: &str) -> Self {
        Self {
            id,
            name: String::from(name),
            original_name: String::from(original_name),
            application: app,
            default_repo: String::from(repo),
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

    pub fn application(&self) -> Application {
        self.application.clone()
    }

    pub fn default_repo(&self) -> &str {
        &self.default_repo
    }

    pub fn update(&mut self, srq: &ServiceRequest, app: Option<Application>) {
        if let Some(name) = srq.name() {
            self.name = name.to_lowercase();
            self.original_name = name;
        }
        if let Some(application) = app {
            self.application = application
        }
        if let Some(default_repo) = srq.default_repo() {
            self.default_repo = default_repo;
        }
    }
}

impl Entity<ServiceDao> for Service {
    fn dao(&self) -> Box<ServiceDao> {
        Box::new(ServiceDao::new(self.id, &self.name, &self.original_name, self.application.id(), &self.default_repo))
    }
}

#[async_trait]
impl QueryEntity<ServiceDao, PgDao, Postgres> for Service {
    async fn read(qry: Value) -> Result<Box<Self>, EntityError> {
        match ServiceDao::read(qry).await {
          Ok(app) => Ok(Box::new(AsyncFrom::<ServiceDao>::from(*app).await)),
          Err(e) => match e {
              DalError::InvalidKey(_) => Err(EntityError::WrongQuery(e.to_string())),
              _ => Err(EntityError::NotFound(e.to_string())),
          }
        }
    }

    async fn persisted(qry: Value) -> bool {
        ServiceDao::exists(qry).await
    }
}

#[async_trait]
impl PersistEntity<ServiceDao, PgDao, Postgres> for Service {
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
                    original_name: String::from(&self.original_name),
                    application: self.application.clone(),
                    default_repo: String::from(&self.default_repo),
                }))
            },
            Err(e) => Err(EntityError::Persist(e))
        }
    }
}

#[async_trait]
impl DestroyEntity<ServiceDao, PgDao, Postgres> for Service {
    async fn destroy(&self) -> Result<(), EntityError> {
        if self.id.is_nil() {
            Err(EntityError::UnPersisted(String::from("service")))
        } else {
            match self.dao().delete().await {
                Ok(_) => Ok(()),
                Err(e) => Err(EntityError::Destroy(e))
            }
        }
    }
}

#[async_trait]
impl AsyncFrom<ServiceDao> for Service {
    async fn from(value: ServiceDao) -> Self {
        let app = ApplicationDao::read(json!({"id": value.application().to_string()})).await.unwrap();
        Self::load(value.id(), value.name(), value.original_name(), AsyncFrom::<ApplicationDao>::from(*app).await, value.default_repo())
    }
}

#[async_trait]
impl AsyncFrom<ServiceRequest> for Service {
    async fn from(value: ServiceRequest) -> Self {
        let srv = ServiceDao::read(serde_json::to_value(value).unwrap()).await.unwrap();
        AsyncFrom::<ServiceDao>::from(*srv).await
    }
}