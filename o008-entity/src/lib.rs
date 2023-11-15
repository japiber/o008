pub mod entity;

use std::fmt::Debug;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::{Database};
use o008_dal::{Dao, QueryContext, error};

pub use entity::Builder;

#[async_trait]
pub trait Entity<I, T, R, DB>
    where I: Debug + Serialize + for<'a> Deserialize<'a> + PartialEq,
          T: Dao<R, DB> + Send + Sized + From<T>,
          R: QueryContext<DB>,
          DB: Database {

    async fn persist(&mut self) -> Result<(), error::Error>;

    fn dao(&self) -> T;

    fn inner(&self) -> &I;
}

