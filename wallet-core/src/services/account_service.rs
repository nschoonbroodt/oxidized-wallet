use chrono::Utc;

use crate::errors::{Result, WalletError};
use crate::{Account, Currency, Money};
use crate::{AccountType, db::accounts::AccountRepository};

pub struct AccountService {
    repository: AccountRepository,
}

impl AccountService {
    pub fn new(repository: AccountRepository) -> Self {
        Self { repository }
    }

    pub async fn create_account(
        &self,
        name: String,
        account_type: AccountType,
        parent_id: Option<i64>,
        currency: Currency,
    ) -> Result<Account> {
        // Validate parent exists if specified
        if let Some(parent_id) = parent_id {
            // TODO: Implement validation to check if parent account exists
        }
        // TODO: validate account name

        // Create account
        let account = Account {
            id: None,
            name,
            account_type,
            parent_id,
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
