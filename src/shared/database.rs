use sqlx::{
    postgres::{PgArguments, PgPoolOptions, PgRow},
    query_with, Error, PgPool, Postgres,
};

pub enum SqlParam<'a> {
    Int(i32),
    Text(&'a str),
    Bool(bool),
    // Add more as needed
}

pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn new(database_url: &str, max_connections: u32) -> Result<Self, Error> {
        let pool = PgPoolOptions::new()
            .max_connections(max_connections)
            .connect(database_url)
            .await?;
        Ok(Self { pool })
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    fn bind_params<'q>(mut query: sqlx::query::Query<'q, Postgres, PgArguments>, params: &[SqlParam<'q>]) -> sqlx::query::Query<'q, Postgres, PgArguments> {
        for param in params {
            query = match param {
                SqlParam::Int(i) => query.bind(*i),
                SqlParam::Text(s) => query.bind(*s),
                SqlParam::Bool(b) => query.bind(*b),
            }
        }
        query
    }

    pub async fn save<'q>(&self, sql: &str, params: &[SqlParam<'q>]) -> Result<u64, Error> {
        let query = query_with(sql, PgArguments::default());
        let query = Self::bind_params(query, params);
        let result = query.execute(&self.pool).await?;
        Ok(result.rows_affected())
    }

    pub async fn get<'q>(&self, sql: &str, params: &[SqlParam<'q>]) -> Result<PgRow, Error> {
        let query = query_with(sql, PgArguments::default());
        let query = Self::bind_params(query, params);
        query.fetch_one(&self.pool).await
    }

    pub async fn select<'q>(&self, sql: &str, params: &[SqlParam<'q>]) -> Result<Vec<PgRow>, Error> {
        let query = query_with(sql, PgArguments::default());
        let query = Self::bind_params(query, params);
        query.fetch_all(&self.pool).await
    }
}

