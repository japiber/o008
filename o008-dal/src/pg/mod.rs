mod builder;
mod tenant;

use async_trait::async_trait;
use sqlx::{FromRow, Pool, Postgres};
use sqlx::postgres::{PgArguments, PgRow, PgPoolOptions};
use sqlx::query::{Query, QueryAs};
use crate::{QueryContext, error};
use async_once::AsyncOnce;

pub use builder::Builder;
pub use tenant::Tenant;
use o008_setting::app_config;

#[derive(Debug, Clone)]
pub struct PgQueryContext<'p>(&'p Pool<Postgres>);

#[async_trait]
impl<'p> QueryContext<Postgres> for PgQueryContext<'p> {

    async fn new() -> Self {
        PgQueryContext(pg_pool().await)
    }

    fn pool(&self) -> &Pool<Postgres> {
        self.0
    }

    async fn fetch_all<'q, T>(&self, query: QueryAs<'q, Postgres, T, PgArguments>) -> Result<Vec<T>, error::Error>
        where T: Send + Unpin + for<'r> FromRow<'r, PgRow>
    {
        match query.fetch_all(self.pool()).await {
            Ok(t) => Ok(t),
            Err(e) => match e {
                sqlx::Error::RowNotFound => Err(error::Error::DataNotFound(e.to_string())),
                _ => Err(error::Error::DataGenericError(e.to_string())),
            },
        }
    }

    async fn fetch_one<'q, T>(&self, query: QueryAs<'q, Postgres, T, PgArguments>) -> Result<Box<T>, error::Error>
        where T: Send + Unpin + for<'r> FromRow<'r, PgRow>
    {
        match query.fetch_one(self.pool()).await {
            Ok(t) => Ok(Box::new(t)),
            Err(e) => match e {
                sqlx::Error::RowNotFound => Err(error::Error::DataNotFound(e.to_string())),
                _ => Err(error::Error::DataGenericError(e.to_string())),
            },
        }
    }

    async fn execute<'q>(&self, query: Query<'q, Postgres, PgArguments>) -> Result<(), error::Error> {
        match query.execute(self.pool()).await {
            Ok(_) => Ok(()),
            Err(e) => Err(error::Error::DataCreation(e.to_string())),
        }
    }
}

lazy_static::lazy_static! {
    static ref ST_O008_PGPOOL: AsyncOnce<Pool<Postgres>> = AsyncOnce::new(async {
            create_pool().await
        });
}

async fn pg_pool() -> &'static Pool<Postgres> {
    ST_O008_PGPOOL.get().await
}

async fn create_pool() -> Pool<Postgres> {
    let cfg = app_config().database();
    PgPoolOptions::new()
        .max_connections(cfg.max_conn)
        .connect(&cfg.uri())
        .await
        .expect( "could not connect to postgres database at create_pool")
}

