use chrono::{Datelike, Local, NaiveDate};
use std::sync::Arc;

use crate::db::connection::Database;
use crate::errors::Result;
use crate::{AccountService, AccountType, Currency, Money, TransactionFilters, TransactionService};

pub struct ReportService {
    account_service: AccountService,
    transaction_service: TransactionService,
}

impl ReportService {
    pub fn new(db: Arc<Database>) -> Self {
        Self {
            account_service: AccountService::new(db.clone()),
            transaction_service: TransactionService::new(db),
        }
    }

    /// Calculate net worth (Assets - Liabilities)
    pub async fn get_net_worth(&self) -> Result<Money> {
        let total_assets = self.get_total_assets().await?;
        let total_liabilities = self.get_total_liabilities().await?;

        let currency = total_assets.currency().clone();
        let net_worth = total_assets.amount_minor() - total_liabilities.amount_minor();

        Ok(Money::from_minor_units(net_worth, currency))
    }

    /// Get total assets (sum of all asset account balances)
    pub async fn get_total_assets(&self) -> Result<Money> {
        self.get_total_by_account_type(AccountType::Asset).await
    }

    /// Get total liabilities (sum of all liability account balances)
    pub async fn get_total_liabilities(&self) -> Result<Money> {
        self.get_total_by_account_type(AccountType::Liability).await
    }

    /// Get current month income
    pub async fn get_monthly_income(&self, year: i32, month: u32) -> Result<Money> {
        self.get_monthly_total_by_account_type(AccountType::Income, year, month)
            .await
    }

    /// Get current month expenses  
    pub async fn get_monthly_expenses(&self, year: i32, month: u32) -> Result<Money> {
        self.get_monthly_total_by_account_type(AccountType::Expense, year, month)
            .await
    }

    /// Get current month income (convenience method using current date)
    pub async fn get_current_month_income(&self) -> Result<Money> {
        let now = Local::now();
        self.get_monthly_income(now.year(), now.month()).await
    }

    /// Get current month expenses (convenience method using current date)
    pub async fn get_current_month_expenses(&self) -> Result<Money> {
        let now = Local::now();
        self.get_monthly_expenses(now.year(), now.month()).await
    }

    /// Helper method to calculate total balance by account type
    async fn get_total_by_account_type(&self, account_type: AccountType) -> Result<Money> {
        let accounts = self.account_service.get_accounts().await?;
        let currency = Currency::new("EUR", 2, "€")?;

        let mut total = 0i64;

        // Sum balances of root accounts of the specified type
        for account in accounts
            .iter()
            .filter(|a| a.parent_id.is_none() && a.account_type == account_type)
        {
            if let Some(account_id) = account.id {
                match self
                    .account_service
                    .calculate_balance_with_children(account_id)
                    .await
                {
                    Ok(balance) => {
                        total += balance.amount_minor();
                    }
                    Err(e) => {
                        // Log error but continue with other accounts
                        eprintln!(
                            "Failed to calculate balance for account {}: {}",
                            account_id, e
                        );
                    }
                }
            }
        }

        Ok(Money::from_minor_units(total, currency))
    }

    /// Helper method to calculate monthly total balance by account type with date filtering
    async fn get_monthly_total_by_account_type(
        &self,
        account_type: AccountType,
        year: i32,
        month: u32,
    ) -> Result<Money> {
        // Calculate start and end dates for the month
        let start_date = NaiveDate::from_ymd_opt(year, month, 1).ok_or_else(|| {
            crate::errors::WalletError::ValidationError("Invalid date".to_string())
        })?;

        let end_date = if month == 12 {
            NaiveDate::from_ymd_opt(year + 1, 1, 1)
        } else {
            NaiveDate::from_ymd_opt(year, month + 1, 1)
        }
        .ok_or_else(|| crate::errors::WalletError::ValidationError("Invalid date".to_string()))?
        .pred_opt()
        .ok_or_else(|| crate::errors::WalletError::ValidationError("Invalid date".to_string()))?;

        // Get all accounts of the specified type
        let accounts = self.account_service.get_accounts().await?;
        let currency = Currency::new("EUR", 2, "€")?;
        let mut total = 0i64;

        // Sum balances for accounts of the specified type within date range
        for account in accounts
            .iter()
            .filter(|a| a.parent_id.is_none() && a.account_type == account_type)
        {
            if let Some(account_id) = account.id {
                // Use the date-filtered balance calculation
                match self
                    .account_service
                    .calculate_account_balance(account_id, Some(end_date))
                    .await
                {
                    Ok(balance) => {
                        // Subtract balance at start of month to get just this month's activity
                        let start_balance = match self
                            .account_service
                            .calculate_account_balance(
                                account_id,
                                Some(start_date.pred_opt().unwrap_or(start_date)),
                            )
                            .await
                        {
                            Ok(start_bal) => start_bal.amount_minor(),
                            Err(_) => 0, // No transactions before start date
                        };

                        total += balance.amount_minor() - start_balance;
                    }
                    Err(e) => {
                        eprintln!(
                            "Failed to calculate monthly balance for account {}: {}",
                            account_id, e
                        );
                    }
                }
            }
        }

        Ok(Money::from_minor_units(total, currency))
    }

    /// Get recent transactions
    pub async fn get_recent_transactions(&self, limit: u32) -> Result<Vec<crate::Transaction>> {
        let filters = TransactionFilters {
            account_id: None,
            from_date: None,
            to_date: None,
            limit: Some(limit),
            offset: None,
        };

        self.transaction_service.get_transactions(filters).await
    }

    /// Get transactions for a specific month
    pub async fn get_monthly_transactions(
        &self,
        year: i32,
        month: u32,
    ) -> Result<Vec<crate::Transaction>> {
        let start_date = NaiveDate::from_ymd_opt(year, month, 1).ok_or_else(|| {
            crate::errors::WalletError::ValidationError("Invalid date".to_string())
        })?;

        let end_date = if month == 12 {
            NaiveDate::from_ymd_opt(year + 1, 1, 1)
        } else {
            NaiveDate::from_ymd_opt(year, month + 1, 1)
        }
        .ok_or_else(|| crate::errors::WalletError::ValidationError("Invalid date".to_string()))?
        .pred_opt()
        .ok_or_else(|| crate::errors::WalletError::ValidationError("Invalid date".to_string()))?;

        let filters = TransactionFilters {
            account_id: None,
            from_date: Some(start_date),
            to_date: Some(end_date),
            limit: None,
            offset: None,
        };

        self.transaction_service.get_transactions(filters).await
    }
}

#[cfg(test)]
mod tests {
    // TODO: Add tests once we have proper test fixtures
}
