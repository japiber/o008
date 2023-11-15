pub mod pg;

use std::fmt::Debug;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::{Database};
use o008_dal::{Dao, QueryContext, error};

pub use pg::Builder;
pub use pg::Tenant;

#[async_trait]
pub trait Entity<I, T, R, DB>
    where I: Debug + Serialize + for<'a> Deserialize<'a> + PartialEq + Clone,
          T: Dao<R, DB> + Send + Sized + From<T>,
          R: QueryContext<DB>,
          DB: Database {

    async fn persist(&mut self) -> Result<(), error::Error>;

    fn dao(&self) -> Box<T>;

    fn inner(&self) -> Box<I>;
}
