use anyhow::Context;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

pub struct PostgresConnection;

impl PostgresConnection {
    pub async fn new_pool(
        connection_string: &str,
        run_migrations: bool,
    ) -> anyhow::Result<Pool<Postgres>> {
        let pool = PgPoolOptions::new()
            .max_connections(20)
            .connect(connection_string)
            .await
            .context("Failed to create Postgres connection pool")?;

        if run_migrations {
            sqlx::migrate!()
                .run(&pool)
                .await
                .context("Failed to run database migrations")?;
        }

        Ok(pool)
    }
}
