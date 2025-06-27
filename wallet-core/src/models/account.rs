use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row, Type};

use crate::models::money::Currency;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text", rename_all = "lowercase")]
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

impl FromRow<'_, sqlx::sqlite::SqliteRow> for Account {
    fn from_row(row: &sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Account {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            account_type: row.try_get("account_type")?,
            parent_id: row.try_get("parent_id")?,
            currency: Currency::from_code(row.try_get("currency")?)
                .map_err(|e| sqlx::Error::Decode(sqlx::error::BoxDynError::from(e)))?,
            description: row.try_get("description")?,
            is_active: row.try_get("is_active")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }
}
