pub mod pg;
mod error;

use std::ops::Deref;
use async_trait::async_trait;
use serde::Serialize;
use serde_json::Value;
use sqlx::{Database};
use o008_dal::{CommandContext, DaoCommand, DaoQuery, QueryContext};

pub use error::EntityError;
pub use pg::Application;
pub use pg::Builder;
pub use pg::Service;
pub use pg::Tenant;


pub trait Entity<T>
    where T: Send + Unpin + Sized {
    fn dao(&self) -> Box<T>;
}

#[async_trait]
pub trait QueryEntity<T, Q, DB>: Entity<T>
    where T: DaoQuery<Q, DB> + Send + Unpin + Sized,
          Q: QueryContext<DB>,
          DB: Database {
    async fn read(qry: Value) -> Result<Box<Self>, EntityError>;
}

#[async_trait]
pub trait PersistEntity<T, C, DB>: Entity<T>
    where T: DaoCommand<C, DB> + Send + Unpin + Sized,
          C: CommandContext<DB>,
          DB: Database {

    async fn persist(&self) -> Result<Box<Self>, EntityError>;
}

#[async_trait]
pub trait DestroyEntity<T, C, DB>: Entity<T>
    where T: DaoCommand<C, DB> + Send + Unpin + Sized,
          C: CommandContext<DB>,
          DB: Database {

    async fn destroy(&self) -> Result<(), EntityError>;
}

pub async fn persist_json<E, T, C, DB>(entity: Box<E>) -> Result<Value, EntityError>
    where E: PersistEntity<T, C, DB> + Serialize,
          T: DaoCommand<C, DB> + Send + Unpin + Sized,
          C: CommandContext<DB>,
          DB: Database {
    let r = entity.persist().await;
    match r  {
        Ok(me) => Ok(serde_json::to_value(me.deref()).unwrap_or(serde_json::Value::Null)),
        Err(e) => Err(e)
    }
}