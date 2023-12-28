use async_trait::async_trait;
use sqlx::{Database, FromRow, Pool};
use sqlx::database::HasArguments;
use sqlx::query::{Query, QueryAs};
mod error;
pub mod pg;

pub use error::DalError;

#[async_trait]
pub trait QueryContext<DB> where DB: Database {
    async fn new() -> Self;
    fn pool(&self) -> &Pool<DB>;
    async fn fetch_all<'q, T>(&self, query: QueryAs<'q, DB, T, <DB as HasArguments<'q>>::Arguments>) -> Result<Vec<T>, DalError>
        where T: Send + Unpin + for<'r> FromRow<'r, DB::Row>;
    async fn fetch_one<'q, T>(&self, query: QueryAs<'q, DB, T, <DB as HasArguments<'q>>::Arguments>) -> Result<Box<T>, DalError>
        where T: Send + Unpin + for<'r> FromRow<'r, DB::Row>;
}

#[async_trait]
pub trait CommandContext<DB> where DB: Database {
    async fn new() -> Self;
    fn pool(&self) -> &Pool<DB>;
    async fn execute<'q>(&self, query: Query<'q, DB, <DB as HasArguments<'q>>::Arguments>) -> Result<(), DalError>;
}


#[async_trait]
pub trait DaoQuery<Q, DB>
    where Q: QueryContext<DB>,
          DB: Database  {
    async fn query_ctx() -> Q {
        Q::new().await
    }
    async fn read(key: serde_json::Value) -> Result<Box<Self>, DalError>;
}

#[async_trait]
pub trait DaoCommand<C, DB>
    where C: CommandContext<DB>,
          DB: Database  {
    async fn command_ctx() -> C {
        C::new().await
    }
    async fn create(&self) -> Result<(), DalError>;
    async fn update(&self) -> Result<(), DalError>;
    async fn delete(&self) -> Result<(), DalError>;
}

