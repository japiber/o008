use async_trait::async_trait;
use sqlx::{Database, FromRow, Pool};
use sqlx::database::HasArguments;
use sqlx::query::{Query, QueryAs};

pub mod error;
pub mod pg;

#[async_trait]
pub trait QueryContext<DB> where DB: Database {

    async fn new() -> Self;
    fn pool(&self) -> &Pool<DB>;
    async fn fetch_all<'q, T>(&self, query: QueryAs<'q, DB, T, <DB as HasArguments<'q>>::Arguments>) -> Result<Vec<T>, error::Error>
        where T: Send + Unpin + for<'r> FromRow<'r, DB::Row>;
    async fn fetch_one<'q, T>(&self, query: QueryAs<'q, DB, T, <DB as HasArguments<'q>>::Arguments>) -> Result<Box<T>, error::Error>
        where T: Send + Unpin + for<'r> FromRow<'r, DB::Row>;
    async fn execute<'q>(&self, query: Query<'q, DB, <DB as HasArguments<'q>>::Arguments>) -> Result<(), error::Error>;
}

#[async_trait]
pub trait Dao<T, DB> where T: QueryContext<DB>, DB: Database  {
    async fn query_ctx() -> T {
        T::new().await
    }
    async fn create(&self) -> Result<(), error::Error>;
    async fn read(key: serde_json::Value) -> Result<Box<Self>, error::Error>;
    async fn update(&self) -> Result<(), error::Error>;
    async fn delete(&self) -> Result<(), error::Error>;
}
