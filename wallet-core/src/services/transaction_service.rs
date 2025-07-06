use chrono::NaiveDate;
use std::sync::Arc;

use crate::db::connection::Database;
use crate::db::transactions::TransactionRepository;
use crate::errors::Result;
use crate::{Money, Transaction};

#[derive(Debug, Clone)]
pub struct TransactionEntryInput {
    pub account_id: i64,
    pub amount: Money,
    pub entry_type: crate::EntryType,
    pub description: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct TransactionFilters {
    pub account_id: Option<i64>,
    pub from_date: Option<NaiveDate>,
    pub to_date: Option<NaiveDate>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

pub struct TransactionService {
    repository: TransactionRepository,
}

impl TransactionService {
    pub fn new(db: Arc<Database>) -> Self {
        Self {
            repository: TransactionRepository::new(db),
        }
    }

    // Core transaction operations
    pub async fn create_transaction(
        &self,
        description: String,
        transaction_date: NaiveDate,
        entries: Vec<TransactionEntryInput>,
    ) -> Result<Transaction> {
        // Validate transaction balance before creating
        Self::validate_transaction_balance(&entries)?;

        self.repository
            .create_transaction(description, transaction_date, entries)
            .await
    }

    pub async fn get_transaction(&self, id: i64) -> Result<Transaction> {
        self.repository.get_transaction(id).await
    }

    pub async fn get_transactions(&self, filters: TransactionFilters) -> Result<Vec<Transaction>> {
        self.repository
            .get_transactions(
                filters.account_id,
                filters.from_date,
                filters.to_date,
                filters.limit,
                filters.offset,
            )
            .await
    }

    // Transaction validation
    pub fn validate_transaction_balance(entries: &[TransactionEntryInput]) -> Result<()> {
        use crate::errors::WalletError;

        // Must have at least 2 entries
        if entries.len() < 2 {
            return Err(WalletError::ValidationError(
                "Transaction must have at least 2 entries".to_string(),
            ));
        }

        // Validate positive amounts
        if entries.iter().any(|e| e.amount.amount_minor() <= 0) {
            return Err(WalletError::ValidationError(
                "All transaction amounts must be positive".to_string(),
            ));
        }

        // Validate all currencies are the same (MVP limitation)
        let first_currency = &entries[0].amount.currency().code();
        if entries
            .iter()
            .any(|e| e.amount.currency().code() != *first_currency)
        {
            return Err(WalletError::ValidationError(
                "Multi-currency transactions not supported yet".to_string(),
            ));
        }

        // Calculate total debits and credits
        let total_debits: i64 = entries
            .iter()
            .filter(|e| matches!(e.entry_type, crate::EntryType::Debit))
            .map(|e| e.amount.amount_minor())
            .sum();

        let total_credits: i64 = entries
            .iter()
            .filter(|e| matches!(e.entry_type, crate::EntryType::Credit))
            .map(|e| e.amount.amount_minor())
            .sum();

        if total_debits != total_credits {
            return Err(WalletError::ValidationError(format!(
                "Transaction is not balanced: debits={total_debits}, credits={total_credits}"
            )));
        }

        Ok(())
    }

    // Helper for simple 2-entry transactions
    pub async fn create_simple_transaction(
        &self,
        description: String,
        date: NaiveDate,
        amount: Money,
        from_account_id: i64, // Account money comes from (credited)
        to_account_id: i64,   // Account money goes to (debited)
    ) -> Result<Transaction> {
        let entries = vec![
            TransactionEntryInput {
                account_id: from_account_id,
                amount: amount.clone(),
                entry_type: crate::EntryType::Credit, // Money leaves the FROM account
                description: None,
            },
            TransactionEntryInput {
                account_id: to_account_id,
                amount,
                entry_type: crate::EntryType::Debit, // Money enters the TO account
                description: None,
            },
        ];

        self.create_transaction(description, date, entries).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Currency, EntryType, Money};

    #[test]
    fn test_validate_transaction_balance_success() {
        let currency = Currency::new("EUR", 2, "€").unwrap();
        let amount = Money::from_minor_units(1000, currency.clone()); // €10.00

        let entries = vec![
            TransactionEntryInput {
                account_id: 1,
                amount: amount.clone(),
                entry_type: EntryType::Credit,
                description: None,
            },
            TransactionEntryInput {
                account_id: 2,
                amount,
                entry_type: EntryType::Debit,
                description: None,
            },
        ];

        assert!(TransactionService::validate_transaction_balance(&entries).is_ok());
    }

    #[test]
    fn test_validate_transaction_balance_unbalanced() {
        let currency = Currency::new("EUR", 2, "€").unwrap();
        let amount1 = Money::from_minor_units(1000, currency.clone()); // €10.00
        let amount2 = Money::from_minor_units(1500, currency.clone()); // €15.00

        let entries = vec![
            TransactionEntryInput {
                account_id: 1,
                amount: amount1,
                entry_type: EntryType::Credit,
                description: None,
            },
            TransactionEntryInput {
                account_id: 2,
                amount: amount2,
                entry_type: EntryType::Debit,
                description: None,
            },
        ];

        let result = TransactionService::validate_transaction_balance(&entries);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not balanced"));
    }

    #[test]
    fn test_validate_transaction_balance_too_few_entries() {
        let currency = Currency::new("EUR", 2, "€").unwrap();
        let amount = Money::from_minor_units(1000, currency);

        let entries = vec![TransactionEntryInput {
            account_id: 1,
            amount,
            entry_type: EntryType::Credit,
            description: None,
        }];

        let result = TransactionService::validate_transaction_balance(&entries);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("at least 2 entries")
        );
    }

    #[test]
    fn test_validate_transaction_balance_negative_amount() {
        let currency = Currency::new("EUR", 2, "€").unwrap();
        let amount = Money::from_minor_units(-1000, currency.clone()); // Negative amount
        let amount2 = Money::from_minor_units(1000, currency);

        let entries = vec![
            TransactionEntryInput {
                account_id: 1,
                amount,
                entry_type: EntryType::Credit,
                description: None,
            },
            TransactionEntryInput {
                account_id: 2,
                amount: amount2,
                entry_type: EntryType::Debit,
                description: None,
            },
        ];

        let result = TransactionService::validate_transaction_balance(&entries);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("must be positive"));
    }
}
