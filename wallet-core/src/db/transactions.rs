use std::sync::Arc;
use chrono::NaiveDate;
use sqlx::Row;

use crate::db::connection::Database;
use crate::errors::Result;
use crate::{Transaction, TransactionEntry, EntryType, Money, Currency};

pub struct TransactionRepository {
    db: Arc<Database>,
}

impl TransactionRepository {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub async fn get_transactions(
        &self,
        account_id: Option<i64>,
        from_date: Option<NaiveDate>,
        to_date: Option<NaiveDate>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Vec<Transaction>> {
        // Build dynamic WHERE clause based on filters
        let mut where_conditions = Vec::new();
        let mut params: Vec<Box<dyn sqlx::Encode<'_, sqlx::Sqlite> + Send + Sync>> = Vec::new();
        let mut param_count = 0;

        if let Some(_) = account_id {
            where_conditions.push(format!("te.account_id = ?{}", param_count + 1));
            param_count += 1;
        }
        if let Some(_) = from_date {
            where_conditions.push(format!("t.transaction_date >= ?{}", param_count + 1));
            param_count += 1;
        }
        if let Some(_) = to_date {
            where_conditions.push(format!("t.transaction_date <= ?{}", param_count + 1));
            param_count += 1;
        }

        let where_clause = if where_conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", where_conditions.join(" AND "))
        };

        let limit_clause = limit.map(|l| format!("LIMIT {}", l)).unwrap_or_default();
        let offset_clause = offset.map(|o| format!("OFFSET {}", o)).unwrap_or_default();

        let query = format!(
            r#"
            SELECT DISTINCT
                t.id as transaction_id,
                t.description as transaction_description,
                t.reference,
                t.transaction_date,
                t.created_at as transaction_created_at,
                t.tags,
                t.notes
            FROM transactions t
            JOIN transaction_entries te ON t.id = te.transaction_id
            {}
            ORDER BY t.transaction_date DESC, t.id DESC
            {} {}
            "#,
            where_clause, limit_clause, offset_clause
        );

        // Execute query with parameters
        let mut query_builder = sqlx::query(&query);
        
        if let Some(aid) = account_id {
            query_builder = query_builder.bind(aid);
        }
        if let Some(fd) = from_date {
            query_builder = query_builder.bind(fd);
        }
        if let Some(td) = to_date {
            query_builder = query_builder.bind(td);
        }

        let rows = query_builder.fetch_all(&self.db.pool).await?;

        let mut transactions = Vec::new();
        for row in rows {
            let transaction_id: i64 = row.get("transaction_id");
            
            // Get entries for this transaction
            let entries = self.get_entries_for_transaction(transaction_id).await?;

            transactions.push(Transaction {
                id: Some(transaction_id),
                description: row.get("transaction_description"),
                reference: row.get("reference"),
                transaction_date: row.get("transaction_date"),
                created_at: row.get("transaction_created_at"),
                tags: row.get("tags"),
                notes: row.get("notes"),
                entries,
            });
        }

        Ok(transactions)
    }

    pub async fn get_transaction(&self, id: i64) -> Result<Transaction> {
        let row = sqlx::query(
            r#"
            SELECT id, description, reference, transaction_date, created_at, tags, notes
            FROM transactions
            WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_one(&self.db.pool)
        .await?;

        let entries = self.get_entries_for_transaction(id).await?;

        Ok(Transaction {
            id: Some(row.get("id")),
            description: row.get("description"),
            reference: row.get("reference"),
            transaction_date: row.get("transaction_date"),
            created_at: row.get("created_at"),
            tags: row.get("tags"),
            notes: row.get("notes"),
            entries,
        })
    }

    pub async fn create_transaction(
        &self,
        description: String,
        transaction_date: NaiveDate,
        entries: Vec<crate::services::transaction_service::TransactionEntryInput>,
    ) -> Result<Transaction> {
        use chrono::Utc;
        
        // Start transaction
        let mut tx = self.db.pool.begin().await?;
        
        // Insert transaction record
        let transaction_result = sqlx::query(
            r#"
            INSERT INTO transactions (description, transaction_date, created_at)
            VALUES (?, ?, ?)
            "#,
        )
        .bind(&description)
        .bind(transaction_date)
        .bind(Utc::now())
        .execute(&mut *tx)
        .await?;
        
        let transaction_id = transaction_result.last_insert_rowid();
        
        // Insert transaction entries
        let mut created_entries = Vec::new();
        for entry_input in entries {
            let entry_type_str = match entry_input.entry_type {
                crate::EntryType::Debit => "debit",
                crate::EntryType::Credit => "credit",
            };
            
            let entry_result = sqlx::query(
                r#"
                INSERT INTO transaction_entries (
                    transaction_id, account_id, amount_minor, currency, 
                    entry_type, description, created_at
                )
                VALUES (?, ?, ?, ?, ?, ?, ?)
                "#,
            )
            .bind(transaction_id)
            .bind(entry_input.account_id)
            .bind(entry_input.amount.amount_minor())
            .bind(entry_input.amount.currency().code())
            .bind(entry_type_str)
            .bind(&entry_input.description)
            .bind(Utc::now())
            .execute(&mut *tx)
            .await?;
            
            let entry_id = entry_result.last_insert_rowid();
            
            created_entries.push(crate::TransactionEntry {
                id: Some(entry_id),
                transaction_id,
                account_id: entry_input.account_id,
                amount: entry_input.amount,
                entry_type: entry_input.entry_type,
                description: entry_input.description,
                created_at: Utc::now(),
            });
        }
        
        // Commit transaction
        tx.commit().await?;
        
        Ok(crate::Transaction {
            id: Some(transaction_id),
            description,
            reference: None,
            transaction_date,
            created_at: Utc::now(),
            tags: None,
            notes: None,
            entries: created_entries,
        })
    }

    async fn get_entries_for_transaction(&self, transaction_id: i64) -> Result<Vec<TransactionEntry>> {
        let rows = sqlx::query(
            r#"
            SELECT 
                te.id,
                te.transaction_id,
                te.account_id,
                te.amount_minor,
                te.currency,
                te.entry_type,
                te.description,
                te.created_at
            FROM transaction_entries te
            WHERE te.transaction_id = ?
            ORDER BY te.id
            "#,
        )
        .bind(transaction_id)
        .fetch_all(&self.db.pool)
        .await?;

        let mut entries = Vec::new();
        for row in rows {
            // Reconstruct Money from database fields
            let amount_minor: i64 = row.get("amount_minor");
            let currency_code: String = row.get("currency");
            let currency = Currency::from_code(&currency_code)?;
            let money = Money::from_minor_units(amount_minor, currency);

            // Parse entry type
            let entry_type_str: String = row.get("entry_type");
            let entry_type = match entry_type_str.as_str() {
                "debit" => EntryType::Debit,
                "credit" => EntryType::Credit,
                _ => return Err(crate::errors::WalletError::ValidationError(
                    format!("Invalid entry type: {}", entry_type_str)
                ).into()),
            };

            entries.push(TransactionEntry {
                id: Some(row.get("id")),
                transaction_id: row.get("transaction_id"),
                account_id: row.get("account_id"),
                amount: money,
                entry_type,
                description: row.get("description"),
                created_at: row.get("created_at"),
            });
        }

        Ok(entries)
    }
}