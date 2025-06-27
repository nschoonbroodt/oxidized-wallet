use std::path::Path;

use crate::errors::Result;
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

pub struct Database {
    pub pool: SqlitePool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self> {
        // Create the file if it does not exists
        let db_path = database_url
            .trim_start_matches("sqlite://")
            .trim_start_matches("sqlite:");
        if let Some(parent) = Path::new(db_path).parent() {
            tokio::fs::create_dir_all(parent).await.ok();
        }
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(database_url)
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
    use super::*;

    #[tokio::test]
    async fn test_database_creation_and_migration() {
        // test on an in-memory SQLite database
        let database_url = "sqlite::memory:";

        // Test database creation
        let db = Database::new(&database_url)
            .await
            .expect("Failed to create database");

        // Test migration
        db.migrate().await.expect("Failed to run migrations");

        // Verify tables were created by querying one
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM accounts")
            .fetch_one(&db.pool)
            .await
            .expect("Failed to query accounts table");

        assert_eq!(count.0, 0); // Should be empty initially
    }
}
