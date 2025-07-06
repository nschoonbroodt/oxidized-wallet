use tauri::State;
use wallet_core::{Account, AccountType, Currency, Money, Transaction, TransactionService, TransactionFilters, AccountService};
use wallet_core::AccountNode;
use chrono::NaiveDate;

use crate::AppState;

#[tauri::command]
#[specta::specta]
pub async fn get_accounts(state: State<'_, AppState>) -> Result<Vec<Account>, String> {
    let account_service = wallet_core::AccountService::new(state.db.clone());
    match account_service.get_accounts().await {
        Ok(accounts) => Ok(accounts),
        Err(e) => Err(format!("Failed to get accounts: {}", e)),
    }
}

#[tauri::command]
#[specta::specta]
pub async fn create_account(
    state: State<'_, AppState>,
    name: String,
    account_type: String,
    parent_id: Option<i64>,
    description: Option<String>,
    currency: String,
) -> Result<Account, String> {
    // Convert string to AccountType enum
    let account_type = match account_type.as_str() {
        "Asset" => AccountType::Asset,
        "Liability" => AccountType::Liability,
        "Equity" => AccountType::Equity,
        "Income" => AccountType::Income,
        "Expense" => AccountType::Expense,
        _ => return Err("Invalid account type".to_string()),
    };

    // Create currency object (for now, just EUR)
    let currency = Currency::new(&currency, 2, "€")
        .map_err(|e| format!("Invalid currency: {}", e))?;

    let account_service = wallet_core::AccountService::new(state.db.clone());
    
    match account_service.create_account(
        name,
        account_type,
        parent_id,
        currency,
    ).await {
        Ok(account) => Ok(account),
        Err(e) => Err(format!("Failed to create account: {}", e)),
    }
}

#[tauri::command]
#[specta::specta]
pub async fn get_account_tree(state: State<'_, AppState>) -> Result<Vec<AccountNode>, String> {
    let account_service = wallet_core::AccountService::new(state.db.clone());
    match account_service.get_account_tree().await {
        Ok(tree) => Ok(tree),
        Err(e) => Err(format!("Failed to get account tree: {}", e)),
    }
}

#[tauri::command]
#[specta::specta]
pub async fn get_transactions(
    state: State<'_, AppState>,
    filters: TransactionFilters,
) -> Result<Vec<Transaction>, String> {
    let transaction_service = TransactionService::new(state.db.clone());
    match transaction_service.get_transactions(filters).await {
        Ok(transactions) => Ok(transactions),
        Err(e) => Err(format!("Failed to get transactions: {}", e)),
    }
}

#[tauri::command]
#[specta::specta]
pub async fn get_transaction(
    state: State<'_, AppState>,
    id: i64,
) -> Result<Transaction, String> {
    let transaction_service = TransactionService::new(state.db.clone());
    match transaction_service.get_transaction(id).await {
        Ok(transaction) => Ok(transaction),
        Err(e) => Err(format!("Failed to get transaction: {}", e)),
    }
}

#[tauri::command]
#[specta::specta]
pub async fn create_simple_transaction(
    state: State<'_, AppState>,
    description: String,
    date: NaiveDate,
    amount_cents: i64,
    currency_code: String,
    from_account_id: i64,
    to_account_id: i64,
) -> Result<Transaction, String> {
    // Create Money object
    let currency = Currency::new(&currency_code, 2, "€")
        .map_err(|e| format!("Invalid currency: {}", e))?;
    let amount = Money::from_minor_units(amount_cents, currency);
    
    let transaction_service = TransactionService::new(state.db.clone());
    match transaction_service.create_simple_transaction(
        description,
        date,
        amount,
        from_account_id,
        to_account_id,
    ).await {
        Ok(transaction) => Ok(transaction),
        Err(e) => Err(format!("Failed to create transaction: {}", e)),
    }
}

#[tauri::command]
#[specta::specta]
pub async fn get_account_balance(
    state: State<'_, AppState>,
    account_id: i64,
) -> Result<Money, String> {
    let account_service = AccountService::new(state.db.clone());
    match account_service.calculate_balance(account_id).await {
        Ok(balance) => Ok(balance),
        Err(e) => Err(format!("Failed to calculate balance: {}", e)),
    }
}

#[tauri::command]
#[specta::specta]
pub async fn get_account_balance_with_children(
    state: State<'_, AppState>,
    account_id: i64,
) -> Result<Money, String> {
    let account_service = AccountService::new(state.db.clone());
    match account_service.calculate_balance_with_children(account_id).await {
        Ok(balance) => Ok(balance),
        Err(e) => Err(format!("Failed to calculate hierarchical balance: {}", e)),
    }
}
