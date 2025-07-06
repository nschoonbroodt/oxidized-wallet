use std::sync::Arc;
use chrono::NaiveDate;

use crate::db::connection::Database;
use crate::db::transactions::TransactionRepository;
use crate::errors::Result;
use crate::{Transaction, TransactionEntry, Money};

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
        todo!("Transaction creation not yet implemented")
    }

    pub async fn get_transaction(&self, id: i64) -> Result<Transaction> {
        self.repository.get_transaction(id).await
    }

    pub async fn get_transactions(&self, filters: TransactionFilters) -> Result<Vec<Transaction>> {
        self.repository.get_transactions(
            filters.account_id,
            filters.from_date,
            filters.to_date,
            filters.limit,
            filters.offset,
        ).await
    }

    // Transaction validation
    pub fn validate_transaction_balance(entries: &[TransactionEntryInput]) -> Result<()> {
        todo!("Transaction balance validation not yet implemented")
    }

    // Helper for simple 2-entry transactions
    pub async fn create_simple_transaction(
        &self,
        description: String,
        date: NaiveDate,
        amount: Money,
        from_account_id: i64,  // Account being debited
        to_account_id: i64,    // Account being credited
    ) -> Result<Transaction> {
        let entries = vec![
            TransactionEntryInput {
                account_id: from_account_id,
                amount: amount.clone(),
                entry_type: crate::EntryType::Debit,
                description: None,
            },
            TransactionEntryInput {
                account_id: to_account_id,
                amount,
                entry_type: crate::EntryType::Credit,
                description: None,
            },
        ];

        self.create_transaction(description, date, entries).await
    }
}