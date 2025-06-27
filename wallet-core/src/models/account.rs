use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::models::money::Currency;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AccountType {
    Asset,
    Liability,
    Equity,
    Income,
    Expense,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Account {
    pub id: Option<i64>,
    pub name: String,
    pub account_type: AccountType,
    pub parent_id: Option<i64>,
    pub currency: Currency,
    pub description: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
