mod builder;
mod tenant;
mod application;
mod service;

use std::sync::Arc;
use async_trait::async_trait;
use sqlx::{FromRow, Pool, Postgres};
use sqlx::postgres::{PgArguments, PgRow, PgPoolOptions};
use sqlx::query::{Query, QueryAs};
use crate::{QueryContext, error, CommandContext, DBPool, DaoQuery, DaoCommand, DalError};
use async_once::AsyncOnce;
use serde_json::Value;
use o008_setting::app_config;

pub use builder::Builder;
pub use tenant::Tenant;
pub use application::Application;
pub use service::Service;

pub type PgQueryContext = dyn QueryContext<Postgres>;
pub type PgCommandContext = dyn CommandContext<Postgres>;
pub type PgDaoQuery = dyn DaoQuery<PgQueryContext, Postgres>;
pub type PgDaoCommand = dyn DaoCommand<PgCommandContext, Postgres>;

#[derive(Debug, Clone)]
pub struct PgPool(Arc<Pool<Postgres>>);

#[async_trait]
impl DBPool<Postgres> for PgPool {
    async fn new() -> Self {
        PgPool(pg_pool().await)
    }

    fn pool(&self) -> &Pool<Postgres> {
        self.0.as_ref()
    }
}

#[async_trait]
impl QueryContext<Postgres> for PgPool {
    async fn fetch_all<'q, T>(&self, query: QueryAs<'q, Postgres, T, PgArguments>) -> Result<Vec<T>, error::DalError>
        where T: Send + Unpin + for<'r> FromRow<'r, PgRow>
    {
        match query.fetch_all(self.pool()).await {
            Ok(t) => Ok(t),
            Err(e) => match e {
                sqlx::Error::RowNotFound => Err(error::DalError::DataNotFound(e.to_string())),
                _ => Err(error::DalError::DataGenericError(e)),
            },
        }
    }

    async fn fetch_one<'q, T>(&self, query: QueryAs<'q, Postgres, T, PgArguments>) -> Result<Box<T>, error::DalError>
        where T: Send + Unpin + for<'r> FromRow<'r, PgRow>
    {
        match query.fetch_one(self.pool()).await {
            Ok(t) => Ok(Box::new(t)),
            Err(e) => match e {
                sqlx::Error::RowNotFound => Err(error::DalError::DataNotFound(e.to_string())),
                _ => Err(error::DalError::DataGenericError(e)),
            },
        }
    }
}

#[async_trait]
impl CommandContext<Postgres> for PgPool {
    async fn execute<'q>(&self, query: Query<'q, Postgres, PgArguments>) -> Result<(), error::DalError> {
        match query.execute(self.pool()).await {
            Ok(_) => Ok(()),
            Err(e) => Err(error::DalError::DataCreation(e)),
        }
    }
}

lazy_static::lazy_static! {
    static ref ST_O008_PGPOOL: AsyncOnce<Arc<Pool<Postgres>>> = AsyncOnce::new(async {
            Arc::new(create_pool().await)
        });
}

async fn pg_pool() -> Arc<Pool<Postgres>> {
    Arc::clone(ST_O008_PGPOOL.get().await)
}

async fn create_pool() -> Pool<Postgres> {
    let cfg = app_config().database();
    PgPoolOptions::new()
        .max_connections(cfg.max_conn)
        .connect(&cfg.uri())
        .await
        .expect( "could not connect to postgres database at create_pool")
}

fn hard_check_key(key: &Value, attributes: &[&str]) -> Result<Vec<Value>, DalError> {
    if key.is_object() {
        let mut vec = Vec::<Value>::new();
        let map = key.as_object().unwrap();
        for a in attributes.iter() {
            if !map.contains_key(*a) {
                return Err(DalError::InvalidKey(format!("missing '{}' attribute: {}", *a, key )))
            }
            let value = map.get(*a).unwrap().clone();
            vec.push(value);
        }
        return Ok(vec)
    }
    Err(DalError::InvalidKey(format!("key should be an object: {}", key)))
}

fn soft_check_key(key: &Value, attributes: &[&str]) -> Result<Vec<Option<Value>>, DalError> {
    if key.is_object() {
        let mut vec = Vec::<Option<Value>>::new();
        let map = key.as_object().unwrap();
        for a in attributes.iter() {
            if !map.contains_key(*a) {
                vec.push(None);
            } else {
                let value = map.get(*a).unwrap().clone();
                vec.push(Some(value));
            }
        }
        return Ok(vec)
    }
    Err(DalError::InvalidKey(format!("key should be an object: {}", key)))
}
