use std::sync::Arc;

use chrono::Utc;

use crate::db::connection::Database;
use crate::errors::{Result, WalletError};
use crate::{Account, Currency, Money};
use crate::{AccountType, db::accounts::AccountRepository};
use crate::AccountNode;
use chrono::NaiveDate;

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
        use crate::{Money, Currency};
        
        // Get the account to determine its type
        let account = self.repository.get_by_id(account_id).await?;
        
        // Get raw transaction sums from repository
        let transaction_sums = self.repository.get_account_transaction_sums(account_id).await?;
        
        // If no transactions, return zero balance with default currency
        let (debit_sum, credit_sum, currency_code) = match transaction_sums {
            Some((debits, credits, currency)) => (debits, credits, currency),
            None => {
                // Default to EUR for zero balance
                let currency = Currency::new("EUR", 2, "€")?;
                return Ok(Money::from_minor_units(0, currency));
            }
        };
        
        // Calculate balance based on account type (normal balance)
        let balance_minor = match account.account_type {
            // Assets & Expenses: Debit increases balance (Debit - Credit)
            crate::AccountType::Asset | crate::AccountType::Expense => debit_sum - credit_sum,
            // Liabilities, Equity & Income: Credit increases balance (Credit - Debit)  
            crate::AccountType::Liability | crate::AccountType::Equity | crate::AccountType::Income => credit_sum - debit_sum,
        };
        
        // Reconstruct Money object
        let currency = Currency::new(&currency_code, 2, "€")?;
        Ok(Money::from_minor_units(balance_minor, currency))
    }

    /// Calculate balance including all descendant accounts (hierarchical)
    pub async fn calculate_balance_with_children(&self, account_id: i64) -> Result<Money> {
        use crate::{Money, Currency};
        
        // Get the account to determine its type
        let account = self.repository.get_by_id(account_id).await?;
        
        // Get all descendant account IDs (including self)
        let account_ids = self.repository.get_descendant_account_ids(account_id).await?;
        
        // Get combined transaction sums for all accounts in the hierarchy
        let transaction_sums = self.repository.get_multiple_accounts_transaction_sums(&account_ids).await?;
        
        // If no transactions, return zero balance with default currency
        let (debit_sum, credit_sum, currency_code) = match transaction_sums {
            Some((debits, credits, currency)) => (debits, credits, currency),
            None => {
                // Default to EUR for zero balance
                let currency = Currency::new("EUR", 2, "€")?;
                return Ok(Money::from_minor_units(0, currency));
            }
        };
        
        // Calculate balance based on account type (normal balance)
        // Note: All accounts in hierarchy should have same account type for this to make sense
        let balance_minor = match account.account_type {
            // Assets & Expenses: Debit increases balance (Debit - Credit)
            crate::AccountType::Asset | crate::AccountType::Expense => debit_sum - credit_sum,
            // Liabilities, Equity & Income: Credit increases balance (Credit - Debit)  
            crate::AccountType::Liability | crate::AccountType::Equity | crate::AccountType::Income => credit_sum - debit_sum,
        };
        
        // Reconstruct Money object
        let currency = Currency::new(&currency_code, 2, "€")?;
        Ok(Money::from_minor_units(balance_minor, currency))
    }

    pub async fn calculate_account_balance(&self, _account_id: i64, _as_of_date: Option<NaiveDate>) -> Result<Money> {
        todo!("Account balance calculation with date not yet implemented")
    }

    pub async fn get_account_balances(&self, _account_ids: &[i64]) -> Result<Vec<(i64, Money)>> {
        todo!("Multiple account balances not yet implemented")
    }

    pub async fn validate_accounts(&self, _account_ids: &[i64]) -> Result<()> {
        todo!("Account validation not yet implemented")
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

    pub async fn update_account(&self, _account: &Account) -> Result<Account> {
        todo!("Account update not yet implemented")
    }

    pub async fn deactivate_account(&self, _id: i64) -> Result<()> {
        todo!("Account deactivation not yet implemented")
    }

    pub async fn get_children(&self, _parent_id: i64) -> Result<Vec<Account>> {
        todo!("Get children accounts not yet implemented")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Money, Currency, AccountType, TransactionService, EntryType};
    use crate::services::transaction_service::TransactionEntryInput;
    use std::sync::Arc;


    async fn create_test_account(
        service: &AccountService,
        name: &str,
        account_type: AccountType,
        parent_id: Option<i64>,
    ) -> Account {
        let currency = Currency::new("EUR", 2, "€").unwrap();
        
        let parent_id = match parent_id {
            Some(id) => id,
            None => {
                // Find the root account for this account type (created by migration)
                let root_name = match account_type {
                    AccountType::Asset => "Assets",
                    AccountType::Liability => "Liabilities", 
                    AccountType::Equity => "Equity",
                    AccountType::Income => "Income",
                    AccountType::Expense => "Expenses",
                };
                
                // Query the root account by name
                let accounts = service.get_accounts().await.unwrap();
                accounts.iter()
                    .find(|acc| acc.name == root_name && acc.parent_id.is_none())
                    .map(|acc| acc.id.unwrap())
                    .expect(&format!("Root account '{}' not found", root_name))
            }
        };
        
        service.create_account(
            name.to_string(),
            account_type,
            Some(parent_id),
            currency,
        ).await.unwrap()
    }

    #[sqlx::test]
    async fn test_calculate_balance_no_transactions(pool: sqlx::SqlitePool) {
        let db = Arc::new(Database { pool });
        let account_service = AccountService::new(db);
        
        // Create a test account
        let account = create_test_account(&account_service, "Test Asset", AccountType::Asset, None).await;
        let account_id = account.id.unwrap();
        
        // Calculate balance - should be zero
        let balance = account_service.calculate_balance(account_id).await.unwrap();
        
        assert_eq!(balance.amount_minor(), 0);
        assert_eq!(balance.currency().code(), "EUR");
    }

    #[sqlx::test]
    async fn test_calculate_balance_asset_account(pool: sqlx::SqlitePool) {
        let db = Arc::new(Database { pool });
        let account_service = AccountService::new(db.clone());
        let transaction_service = TransactionService::new(db);
        
        // Create accounts
        let asset_account = create_test_account(&account_service, "Bank Account", AccountType::Asset, None).await;
        let income_account = create_test_account(&account_service, "Salary", AccountType::Income, None).await;
        
        let asset_id = asset_account.id.unwrap();
        let income_id = income_account.id.unwrap();
        
        // Create a transaction: 1000 EUR salary (Income -> Asset)
        let currency = Currency::new("EUR", 2, "€").unwrap();
        let amount = Money::from_minor_units(100000, currency); // 1000.00 EUR
        
        let entries = vec![
            TransactionEntryInput {
                account_id: income_id,
                amount: amount.clone(),
                entry_type: EntryType::Credit, // Income increases with credit
                description: None,
            },
            TransactionEntryInput {
                account_id: asset_id,
                amount: amount.clone(),
                entry_type: EntryType::Debit, // Asset increases with debit
                description: None,
            },
        ];
        
        transaction_service.create_transaction(
            "Salary payment".to_string(),
            chrono::NaiveDate::from_ymd_opt(2025, 7, 6).unwrap(),
            entries,
        ).await.unwrap();
        
        // Test Asset account balance (should be positive)
        let asset_balance = account_service.calculate_balance(asset_id).await.unwrap();
        assert_eq!(asset_balance.amount_minor(), 100000); // +1000 EUR for asset
        
        // Test Income account balance (should be positive)
        let income_balance = account_service.calculate_balance(income_id).await.unwrap();
        assert_eq!(income_balance.amount_minor(), 100000); // +1000 EUR for income
    }

    #[sqlx::test]
    async fn test_calculate_balance_expense_transaction(pool: sqlx::SqlitePool) {
        let db = Arc::new(Database { pool });
        let account_service = AccountService::new(db.clone());
        let transaction_service = TransactionService::new(db);
        
        // Create accounts
        let asset_account = create_test_account(&account_service, "Bank Account", AccountType::Asset, None).await;
        let expense_account = create_test_account(&account_service, "Groceries", AccountType::Expense, None).await;
        
        let asset_id = asset_account.id.unwrap();
        let expense_id = expense_account.id.unwrap();
        
        // First add some money to the asset account
        let currency = Currency::new("EUR", 2, "€").unwrap();
        let initial_amount = Money::from_minor_units(100000, currency.clone()); // 1000.00 EUR
        let expense_amount = Money::from_minor_units(5000, currency); // 50.00 EUR
        
        // Income transaction first
        let income_account = create_test_account(&account_service, "Salary", AccountType::Income, None).await;
        let income_id = income_account.id.unwrap();
        
        let income_entries = vec![
            TransactionEntryInput {
                account_id: income_id,
                amount: initial_amount.clone(),
                entry_type: EntryType::Credit,
                description: None,
            },
            TransactionEntryInput {
                account_id: asset_id,
                amount: initial_amount,
                entry_type: EntryType::Debit,
                description: None,
            },
        ];
        
        transaction_service.create_transaction(
            "Initial income".to_string(),
            chrono::NaiveDate::from_ymd_opt(2025, 7, 1).unwrap(),
            income_entries,
        ).await.unwrap();
        
        // Now create expense transaction: Asset -> Expense
        let expense_entries = vec![
            TransactionEntryInput {
                account_id: asset_id,
                amount: expense_amount.clone(),
                entry_type: EntryType::Credit, // Asset decreases with credit
                description: None,
            },
            TransactionEntryInput {
                account_id: expense_id,
                amount: expense_amount,
                entry_type: EntryType::Debit, // Expense increases with debit
                description: None,
            },
        ];
        
        transaction_service.create_transaction(
            "Grocery shopping".to_string(),
            chrono::NaiveDate::from_ymd_opt(2025, 7, 6).unwrap(),
            expense_entries,
        ).await.unwrap();
        
        // Test Asset account balance (initial - expense)
        let asset_balance = account_service.calculate_balance(asset_id).await.unwrap();
        assert_eq!(asset_balance.amount_minor(), 95000); // 1000 - 50 = 950 EUR
        
        // Test Expense account balance (should be positive)
        let expense_balance = account_service.calculate_balance(expense_id).await.unwrap();
        assert_eq!(expense_balance.amount_minor(), 5000); // +50 EUR expense
    }

    #[sqlx::test]
    async fn test_calculate_balance_with_children(pool: sqlx::SqlitePool) {
        let db = Arc::new(Database { pool });
        let account_service = AccountService::new(db.clone());
        let transaction_service = TransactionService::new(db);
        
        // Create account hierarchy: Parent Asset -> Child Asset
        let parent_account = create_test_account(&account_service, "Bank Accounts", AccountType::Asset, None).await;
        let parent_id = parent_account.id.unwrap();
        
        let child_account = create_test_account(&account_service, "Checking Account", AccountType::Asset, Some(parent_id)).await;
        let child_id = child_account.id.unwrap();
        
        let income_account = create_test_account(&account_service, "Salary", AccountType::Income, None).await;
        let income_id = income_account.id.unwrap();
        
        // Add money to child account
        let currency = Currency::new("EUR", 2, "€").unwrap();
        let amount = Money::from_minor_units(50000, currency); // 500.00 EUR
        
        let entries = vec![
            TransactionEntryInput {
                account_id: income_id,
                amount: amount.clone(),
                entry_type: EntryType::Credit,
                description: None,
            },
            TransactionEntryInput {
                account_id: child_id,
                amount: amount,
                entry_type: EntryType::Debit,
                description: None,
            },
        ];
        
        transaction_service.create_transaction(
            "Money to child account".to_string(),
            chrono::NaiveDate::from_ymd_opt(2025, 7, 6).unwrap(),
            entries,
        ).await.unwrap();
        
        // Test individual balances
        let parent_balance = account_service.calculate_balance(parent_id).await.unwrap();
        let child_balance = account_service.calculate_balance(child_id).await.unwrap();
        
        assert_eq!(parent_balance.amount_minor(), 0); // Parent has no direct transactions
        assert_eq!(child_balance.amount_minor(), 50000); // Child has 500 EUR
        
        // Test hierarchical balance
        let hierarchical_balance = account_service.calculate_balance_with_children(parent_id).await.unwrap();
        assert_eq!(hierarchical_balance.amount_minor(), 50000); // Parent + child = 500 EUR
        
        // Child hierarchical balance should equal its own balance
        let child_hierarchical = account_service.calculate_balance_with_children(child_id).await.unwrap();
        assert_eq!(child_hierarchical.amount_minor(), 50000); // Same as direct balance
    }
}
