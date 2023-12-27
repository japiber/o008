pub mod pg;
mod error;

use std::ops::Deref;
use async_trait::async_trait;
use serde::Serialize;
use sqlx::{Database};
use o008_dal::{CommandContext, DaoCommand, DaoQuery, QueryContext};

pub use error::EntityError;
pub use pg::Application;
pub use pg::Builder;
pub use pg::Service;
pub use pg::Tenant;


pub trait QueryEntity<T, Q, DB>
    where T: DaoQuery<Q, DB>  + Send + Sized,
          Q: QueryContext<DB>,
          DB: Database {

    fn dao(&self) -> Box<T>;
}


#[async_trait]
pub trait Entity<T, Q, C, DB>
    where T: DaoQuery<Q, DB> + DaoCommand<C, DB> + Send + Sized,
          Q: QueryContext<DB>,
          C: CommandContext<DB>,
          DB: Database {

    async fn persist(&self) -> Result<Box<Self>, EntityError>;

    async fn destroy(&self) -> Result<(), EntityError>;

    fn dao(&self) -> Box<T>;
}

#[async_trait::async_trait]
pub trait AsyncFrom<T, Q, C, DB>: Entity<T, Q, C, DB>
    where T: DaoQuery<Q, DB> + DaoCommand<C, DB> + Send + Sized,
          Q: QueryContext<DB>,
          C: CommandContext<DB>,
          DB: Database {
    async fn from(value: T) -> Self;
}

pub async fn persist_json<E, T, Q, C, DB>(entity: Box<E>) -> Result<serde_json::Value, EntityError>
    where E: Entity<T, Q, C, DB> + Serialize,
          T: DaoQuery<Q, DB> + DaoCommand<C, DB> + Send + Sized,
          Q: QueryContext<DB>,
          C: CommandContext<DB>,
          DB: Database {
    let r = entity.persist().await;
    match r  {
        Ok(me) => Ok(serde_json::to_value(me.deref()).unwrap_or(serde_json::Value::Null)),
        Err(e) => Err(e)
    }
}