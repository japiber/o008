use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::{Database, FromRow, Pool};
use sqlx::database::HasArguments;
use sqlx::query::{Query, QueryAs};
use uuid::Uuid;

mod error;
pub mod pg;

pub use error::DalError;

#[derive(Debug, PartialEq, Serialize, Deserialize, sqlx::FromRow)]
pub struct DalCount { pub count: i64 }

#[async_trait]
pub trait DBPool<DB> where DB: Database {
    async fn new() -> Self;
    fn pool(&self) -> &Pool<DB>;
}

#[async_trait]
pub trait QueryContext<DB>: DBPool<DB> + Sized
    where DB: Database {
    async fn fetch_all<'q, T>(&self, query: QueryAs<'q, DB, T, <DB as HasArguments<'q>>::Arguments>) -> Result<Vec<T>, DalError>
        where T: Send + Unpin + Sized + for<'r> FromRow<'r, DB::Row>;
    async fn fetch_one<'q, T>(&self, query: QueryAs<'q, DB, T, <DB as HasArguments<'q>>::Arguments>) -> Result<Box<T>, DalError>
        where T: Send + Unpin + Sized + for<'r> FromRow<'r, DB::Row>;
}

#[async_trait]
pub trait CommandContext<DB>: DBPool<DB> + Sized
    where DB: Database {
    async fn execute<'q>(&self, query: Query<'q, DB, <DB as HasArguments<'q>>::Arguments>) -> Result<(), DalError>;
}

#[async_trait]
pub trait DaoQuery<Q, DB>
    where Q: QueryContext<DB> + Sized,
          DB: Database  {
    async fn query_ctx() -> Q {
        Q::new().await
    }
    async fn read(key: serde_json::Value) -> Result<Box<Self>, DalError>;
    async fn exists(key: serde_json::Value) -> bool;
}

#[async_trait]
pub trait DaoCommand<C, DB>
    where C: CommandContext<DB> + Sized,
          DB: Database  {
    async fn command_ctx() -> C {
        C::new().await
    }
    async fn insert(&self) -> Result<(), DalError>;
    async fn update(&self) -> Result<(), DalError>;
    async fn delete(&self) -> Result<(), DalError>;
}

fn gen_v7_uuid(id: Uuid) -> Uuid {
    if id.is_nil() {
        Uuid::now_v7()
    } else {
        id
    }
}

