use sqlx::Row;
use std::sync::Arc;

use crate::AccountNode;
use crate::errors::Result;
use crate::{Account, db::connection::Database};

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
        .bind(account.parent_id)
        .bind(account.currency.code())
        .bind(&account.description)
        .bind(account.is_active)
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

    /// Get raw debit/credit sums for an account from transaction entries
    pub async fn get_account_transaction_sums(
        &self,
        account_id: i64,
    ) -> Result<Option<(i64, i64, String)>> {
        let row = sqlx::query(
            r#"
            SELECT 
                COALESCE(SUM(CASE WHEN entry_type = 'debit' THEN amount_minor ELSE 0 END), 0) as total_debits,
                COALESCE(SUM(CASE WHEN entry_type = 'credit' THEN amount_minor ELSE 0 END), 0) as total_credits,
                currency
            FROM transaction_entries 
            WHERE account_id = ?
            GROUP BY currency
            "#,
        )
        .bind(account_id)
        .fetch_optional(&self.db.pool)
        .await?;

        match row {
            Some(row) => {
                let total_debits: i64 = row.get("total_debits");
                let total_credits: i64 = row.get("total_credits");
                let currency: String = row.get("currency");
                Ok(Some((total_debits, total_credits, currency)))
            }
            None => Ok(None),
        }
    }

    /// Get all descendant account IDs using recursive CTE
    pub async fn get_descendant_account_ids(&self, parent_account_id: i64) -> Result<Vec<i64>> {
        let rows = sqlx::query(
            r#"
            WITH RECURSIVE account_tree AS (
                -- Base case: start with the parent account
                SELECT id FROM accounts WHERE id = ?
                UNION ALL
                -- Recursive case: find all children
                SELECT a.id 
                FROM accounts a
                INNER JOIN account_tree at ON a.parent_id = at.id
                WHERE a.is_active = 1
            )
            SELECT id FROM account_tree
            "#,
        )
        .bind(parent_account_id)
        .fetch_all(&self.db.pool)
        .await?;

        let account_ids = rows.iter().map(|row| row.get::<i64, _>("id")).collect();
        Ok(account_ids)
    }

    /// Get raw debit/credit sums for multiple accounts (for hierarchical balance)
    pub async fn get_multiple_accounts_transaction_sums(
        &self,
        account_ids: &[i64],
    ) -> Result<Option<(i64, i64, String)>> {
        if account_ids.is_empty() {
            return Ok(None);
        }

        // Create placeholders for the IN clause
        let placeholders = vec!["?"; account_ids.len()].join(",");
        let query = format!(
            r#"
            SELECT 
                COALESCE(SUM(CASE WHEN entry_type = 'debit' THEN amount_minor ELSE 0 END), 0) as total_debits,
                COALESCE(SUM(CASE WHEN entry_type = 'credit' THEN amount_minor ELSE 0 END), 0) as total_credits,
                currency
            FROM transaction_entries 
            WHERE account_id IN ({placeholders})
            GROUP BY currency
            "#
        );

        let mut query_builder = sqlx::query(&query);
        for account_id in account_ids {
            query_builder = query_builder.bind(account_id);
        }

        let row = query_builder.fetch_optional(&self.db.pool).await?;

        match row {
            Some(row) => {
                let total_debits: i64 = row.get("total_debits");
                let total_credits: i64 = row.get("total_credits");
                let currency: String = row.get("currency");
                Ok(Some((total_debits, total_credits, currency)))
            }
            None => Ok(None),
        }
    }

    pub async fn get_children(&self, parent_id: i64) -> Result<Vec<Account>> {
        let accounts: Vec<Account> = sqlx::query_as(
            r#"
            SELECT id, name, account_type, parent_id, currency, description, is_active, created_at, updated_at
            FROM accounts
            WHERE parent_id = ?1 AND is_active = TRUE
            ORDER BY name
            "#,
        )
        .bind(parent_id)
        .fetch_all(&self.db.pool)
        .await?;
        Ok(accounts)
    }

    pub async fn deactivate(&self, id: i64) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE accounts 
            SET is_active = FALSE, updated_at = CURRENT_TIMESTAMP
            WHERE id = ?1
            "#,
        )
        .bind(id)
        .execute(&self.db.pool)
        .await?;
        Ok(())
    }

    pub async fn update(&self, account: &Account) -> Result<Account> {
        let id = account.id.ok_or_else(|| {
            crate::errors::WalletError::ValidationError(
                "Account ID is required for update".to_string(),
            )
        })?;

        sqlx::query(
            r#"
            UPDATE accounts 
            SET name = ?2, description = ?3, updated_at = CURRENT_TIMESTAMP
            WHERE id = ?1
            "#,
        )
        .bind(id)
        .bind(&account.name)
        .bind(&account.description)
        .execute(&self.db.pool)
        .await?;

        self.get_by_id(id).await
    }

    pub async fn get_account_transaction_sums_before_date(
        &self,
        account_id: i64,
        before_date: chrono::NaiveDate,
    ) -> Result<Option<(i64, i64, String)>> {
        let row = sqlx::query(
            r#"
            SELECT 
                COALESCE(SUM(CASE WHEN entry_type = 'debit' THEN amount_minor ELSE 0 END), 0) as total_debits,
                COALESCE(SUM(CASE WHEN entry_type = 'credit' THEN amount_minor ELSE 0 END), 0) as total_credits,
                currency
            FROM transaction_entries te
            JOIN transactions t ON te.transaction_id = t.id
            WHERE te.account_id = ?1 AND t.transaction_date < ?2
            GROUP BY currency
            "#,
        )
        .bind(account_id)
        .bind(before_date)
        .fetch_optional(&self.db.pool)
        .await?;

        match row {
            Some(row) => {
                let total_debits: i64 = row.get("total_debits");
                let total_credits: i64 = row.get("total_credits");
                let currency: String = row.get("currency");
                Ok(Some((total_debits, total_credits, currency)))
            }
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{account::AccountType, money::Currency};
    use chrono::Utc;
    use std::sync::Arc;

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

    #[sqlx::test]
    async fn test_create_account(pool: sqlx::SqlitePool) {
        let db = Arc::new(Database { pool });
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

    #[sqlx::test]
    async fn test_get_by_id(pool: sqlx::SqlitePool) {
        let db = Arc::new(Database { pool });
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

    #[sqlx::test]
    async fn test_get_by_id_not_found(pool: sqlx::SqlitePool) {
        let db = Arc::new(Database { pool });
        let repo = AccountRepository::new(db);

        // Try to get an account that doesn't exist
        let result = repo.get_by_id(999).await;

        // Should return an error
        assert!(result.is_err());
    }

    #[sqlx::test]
    async fn test_create_account_with_parent(pool: sqlx::SqlitePool) {
        let db = Arc::new(Database { pool });
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
