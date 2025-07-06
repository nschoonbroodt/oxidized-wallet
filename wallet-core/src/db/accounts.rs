use std::sync::Arc;

use crate::errors::Result;
use crate::{Account, db::connection::Database};
use crate::AccountNode;

pub struct AccountRepository {
    db: Arc<Database>,
}

impl AccountRepository {
    pub fn new(db: Arc<Database>) -> Self {
        AccountRepository { db }
    }

    pub async fn create(&self, account: &Account) -> Result<Account> {
        let id = sqlx::query(
            r#"
            INSERT INTO accounts (name, account_type, parent_id, currency, description, is_active)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            "#,
        )
        .bind(&account.name)
        .bind(&account.account_type)
        .bind(&account.parent_id)
        .bind(&account.currency.code())
        .bind(&account.description)
        .bind(&account.is_active)
        .execute(&self.db.pool)
        .await?
        .last_insert_rowid();

        self.get_by_id(id).await
    }

    pub async fn get_all(&self) -> Result<Vec<Account>> {
        let accounts: Vec<Account> = sqlx::query_as(
            r#"
            SELECT id, name, account_type, parent_id, currency, description, is_active, created_at, updated_at
            FROM accounts
            ORDER BY created_at DESC
            "#,
        )
        .fetch_all(&self.db.pool)
        .await?;
        Ok(accounts)
    }

    pub async fn get_account_tree(&self) -> Result<Vec<AccountNode>> {
        let nodes: Vec<AccountNode> = sqlx::query_as(
            r#"
            WITH RECURSIVE account_tree AS (
                -- Base case: root accounts
                SELECT 
                    id, name, account_type, parent_id, currency, description, 
                    is_active, created_at, updated_at,
                    0 as level, 
                    name as path
                FROM accounts 
                WHERE parent_id IS NULL AND is_active = true
                
                UNION ALL
                
                -- Recursive case: children  
                SELECT 
                    a.id, a.name, a.account_type, a.parent_id, a.currency, 
                    a.description, a.is_active, a.created_at, a.updated_at,
                    t.level + 1, 
                    t.path || ' > ' || a.name
                FROM accounts a
                JOIN account_tree t ON a.parent_id = t.id
                WHERE a.is_active = true
            )
            SELECT 
                id, name, account_type, parent_id, currency, description, 
                is_active, created_at, updated_at, level, path
            FROM account_tree 
            ORDER BY 
                CASE account_type 
                    WHEN 'asset' THEN 1 
                    WHEN 'liability' THEN 2 
                    WHEN 'equity' THEN 3 
                    WHEN 'income' THEN 4 
                    WHEN 'expense' THEN 5 
                END,
                path
            "#,
        )
        .fetch_all(&self.db.pool)
        .await?;
        Ok(nodes)
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Account> {
        let account: Account = sqlx::query_as(
            r#"
            SELECT id, name, account_type, parent_id, currency, description, is_active, created_at, updated_at
            FROM accounts
            WHERE id = ?1
            "#).bind(id)
        .fetch_one(&self.db.pool)
        .await?;
        Ok(account)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{account::AccountType, money::Currency};
    use chrono::Utc;
    use std::sync::Arc;

    async fn setup_test_db() -> Arc<Database> {
        let db = Database::new("sqlite::memory:").await.unwrap();
        db.migrate().await.unwrap();
        Arc::new(db)
    }

    fn create_test_account() -> Account {
        Account {
            id: None,
            name: "Test Checking Account".to_string(),
            account_type: AccountType::Asset,
            parent_id: None,
            currency: Currency::eur(),
            description: Some("Test account for unit tests".to_string()),
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[tokio::test]
    async fn test_create_account() {
        let db = setup_test_db().await;
        let repo = AccountRepository::new(db);
        let test_account = create_test_account();

        // Test account creation
        let created_account = repo.create(&test_account).await.unwrap();

        // Verify the account was created correctly
        assert!(created_account.id.is_some());
        assert_eq!(created_account.name, test_account.name);
        assert_eq!(created_account.account_type, test_account.account_type);
        assert_eq!(created_account.parent_id, test_account.parent_id);
        assert_eq!(
            created_account.currency.code(),
            test_account.currency.code()
        );
        assert_eq!(created_account.description, test_account.description);
        assert_eq!(created_account.is_active, test_account.is_active);

        // Verify timestamps are set
        assert!(created_account.created_at <= Utc::now());
        assert!(created_account.updated_at <= Utc::now());
    }

    #[tokio::test]
    async fn test_get_by_id() {
        let db = setup_test_db().await;
        let repo = AccountRepository::new(db);
        let test_account = create_test_account();

        // First create an account
        let created_account = repo.create(&test_account).await.unwrap();
        let account_id = created_account.id.unwrap();

        // Test retrieving the account by ID
        let retrieved_account = repo.get_by_id(account_id).await.unwrap();

        // Verify all fields match
        assert_eq!(retrieved_account.id, Some(account_id));
        assert_eq!(retrieved_account.name, test_account.name);
        assert_eq!(retrieved_account.account_type, test_account.account_type);
        assert_eq!(retrieved_account.parent_id, test_account.parent_id);
        assert_eq!(
            retrieved_account.currency.code(),
            test_account.currency.code()
        );
        assert_eq!(retrieved_account.description, test_account.description);
        assert_eq!(retrieved_account.is_active, test_account.is_active);

        // Verify it matches the originally created account
        assert_eq!(retrieved_account.created_at, created_account.created_at);
        assert_eq!(retrieved_account.updated_at, created_account.updated_at);
    }

    #[tokio::test]
    async fn test_get_by_id_not_found() {
        let db = setup_test_db().await;
        let repo = AccountRepository::new(db);

        // Try to get an account that doesn't exist
        let result = repo.get_by_id(999).await;

        // Should return an error
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_account_with_parent() {
        let db = setup_test_db().await;
        let repo = AccountRepository::new(db);

        // Create parent account
        let parent_account = create_test_account();
        let created_parent = repo.create(&parent_account).await.unwrap();
        let parent_id = created_parent.id.unwrap();

        // Create child account
        let mut child_account = create_test_account();
        child_account.name = "Child Account".to_string();
        child_account.parent_id = Some(parent_id);

        let created_child = repo.create(&child_account).await.unwrap();

        // Verify parent relationship
        assert_eq!(created_child.parent_id, Some(parent_id));

        // Verify we can retrieve both accounts
        let retrieved_parent = repo.get_by_id(parent_id).await.unwrap();
        let retrieved_child = repo.get_by_id(created_child.id.unwrap()).await.unwrap();

        assert_eq!(retrieved_parent.name, "Test Checking Account");
        assert_eq!(retrieved_child.name, "Child Account");
        assert_eq!(retrieved_child.parent_id, Some(parent_id));
    }
}
