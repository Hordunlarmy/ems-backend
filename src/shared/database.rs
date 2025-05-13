use sqlx::{PgPool, postgres::PgRow, Row, Error, Transaction};
use dotenvy::dotenv;
use std::env;

pub struct Database {
    pub pool: PgPool,
}

impl Database {
    pub async fn new() -> Result<Self, Error> {

        let db_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

        let max_connections = env::var("MAX_CONNECTIONS")
            .ok()
            .and_then(|val| val.parse::<u32>().ok())
            .unwrap_or(5);

        let pool = sqlx::PgPoolOptions::new()
            .max_connections(max_connections)
            .connect(&db_url)
            .await?;

        Ok(Self { pool })
    }

    pub async fn save(&self, query: &str, params: Vec<&(dyn sqlx::Encode<'_> + sqlx::Type<Postgres>)>) -> Result<u64, Error> {
        let mut q = sqlx::query(query);
        for param in params {
            q = q.bind(param);
        }
        let result = q.execute(&self.pool).await?;
        Ok(result.rows_affected())
    }

    pub async fn get(&self, query: &str, params: Vec<&(dyn sqlx::Encode<'_> + sqlx::Type<Postgres>)>) -> Result<PgRow, Error> {
        let mut q = sqlx::query(query);
        for param in params {
            q = q.bind(param);
        }
        let row = q.fetch_one(&self.pool).await?;
        Ok(row)
    }

    pub async fn select(&self, query: &str, params: Vec<&(dyn sqlx::Encode<'_> + sqlx::Type<Postgres>)>) -> Result<Vec<PgRow>, Error> {
        let mut q = sqlx::query(query);
        for param in params {
            q = q.bind(param);
        }
        let rows = q.fetch_all(&self.pool).await?;
        Ok(rows)
    }

    pub async fn transaction<F, T>(&self, f: F) -> Result<T, Error>
    where
        F: FnOnce(&mut Transaction<'_, sqlx::Postgres>) -> Result<T, Error>,
    {
        let mut tx = self.pool.begin().await?;

        let result = f(&mut tx).await;

        match result {
            Ok(val) => {
                tx.commit().await?;
                Ok(val)
            }
            Err(err) => {
                tx.rollback().await?;
                Err(err)
            }
        }
    }
}

