use std::sync::Arc;

use chrono::Utc;

use crate::db::connection::Database;
use crate::errors::{Result, WalletError};
use crate::{Account, Currency, Money};
use crate::{AccountType, db::accounts::AccountRepository};
use crate::AccountNode;

pub struct AccountService {
    repository: AccountRepository,
}

impl AccountService {
    pub fn new(db: Arc<Database>) -> Self {
        Self {
            repository: AccountRepository::new(db),
        }
    }

    pub async fn create_account(
        &self,
        name: String,
        account_type: AccountType,
        parent_id: Option<i64>,
        currency: Currency,
    ) -> Result<Account> {
        // Validate parent_id is provided (no root accounts allowed)
        let parent_id = parent_id.ok_or_else(|| {
            WalletError::ValidationError("Parent account is required".to_string())
        })?;

        // Validate parent exists and has same account type
        let parent = self.repository.get_by_id(parent_id).await?;
        if parent.account_type != account_type {
            return Err(WalletError::ValidationError(
                format!(
                    "Account type {:?} must be created under a parent of the same type",
                    account_type
                ),
            ));
        }

        // Validate account name is not empty
        let name = name.trim();
        if name.is_empty() {
            return Err(WalletError::ValidationError(
                "Account name cannot be empty".to_string(),
            ));
        }

        // Create account
        let account = Account {
            id: None,
            name: name.to_string(),
            account_type,
            parent_id: Some(parent_id),
            currency,
            description: None,
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        self.repository.create(&account).await
    }
    pub async fn calculate_balance(&self, account_id: i64) -> Result<Money> {
        // Implementation to calculate balance from transaction entries
        // Including child account balances
        todo!()
    }

    pub async fn get_accounts(&self) -> Result<Vec<Account>> {
        self.repository.get_all().await
    }

    pub async fn get_account_tree(&self) -> Result<Vec<AccountNode>> {
        self.repository.get_account_tree().await
    }

    pub async fn get_account(&self, id: i64) -> Result<Account> {
        self.repository.get_by_id(id).await
    }

    pub async fn update_account(&self, account: &Account) -> Result<Account> {
        todo!();
    }
    pub async fn deactivate_account(&self, id: i64) -> Result<()> {
        todo!();
    }
    pub async fn get_children(&self, parent_id: i64) -> Result<Vec<Account>> {
        todo!();
    }
}
