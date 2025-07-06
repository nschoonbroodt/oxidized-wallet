use crate::models::money::Money;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, specta::Type)]
pub enum EntryType {
    Credit,
    Debit,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, specta::Type)]
pub struct TransactionEntry {
    pub id: Option<i64>,
    pub transaction_id: i64,
    pub account_id: i64,
    pub amount: Money,
    pub entry_type: EntryType,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, specta::Type)]
pub struct Transaction {
    pub id: Option<i64>,
    pub description: String,
    pub reference: Option<String>,
    pub transaction_date: NaiveDate,
    pub created_at: DateTime<Utc>,
    pub tags: Option<String>,
    pub notes: Option<String>,
    pub entries: Vec<TransactionEntry>,
}
