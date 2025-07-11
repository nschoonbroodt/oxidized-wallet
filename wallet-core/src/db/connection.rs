use std::path::Path;

use crate::errors::Result;
use sqlx::{
    SqlitePool,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};

pub struct Database {
    pub pool: SqlitePool,
}

impl Database {
    pub async fn new(database_file: &str) -> Result<Self> {
        // Create the file if it does not exists
        let db_path = database_file
            .trim_start_matches("sqlite://")
            .trim_start_matches("sqlite:");
        if let Some(parent) = Path::new(db_path).parent() {
            tokio::fs::create_dir_all(parent).await.ok();
        }

        let options = SqliteConnectOptions::new()
            .filename(db_path)
            .create_if_missing(true);

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(options)
            .await?;
        Ok(Database { pool })
    }

    pub async fn migrate(&self) -> Result<()> {
        sqlx::migrate!("./migrations")
            .run(&self.pool)
            .await
            .map_err(|e| e.into())
    }
}

#[cfg(test)]
mod tests {

    #[sqlx::test]
    async fn test_database_creation_and_migration(pool: sqlx::SqlitePool) {
        // Test that migration has already run (done automatically by sqlx::test)
        // Verify tables were created by querying one
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM accounts")
            .fetch_one(&pool)
            .await
            .expect("Failed to query accounts table");

        assert_eq!(count.0, 5); // Should have 5 root accounts from migration
    }
}
