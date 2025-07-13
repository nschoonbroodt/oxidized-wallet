use chrono::NaiveDate;
use tauri::State;
use wallet_core::AccountNode;
use wallet_core::{
    Account, AccountService, AccountType, Currency, Money, ReportService, Transaction,
    TransactionFilters, TransactionService,
};

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
    _description: Option<String>,
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
    let currency =
        Currency::new(&currency, 2, "€").map_err(|e| format!("Invalid currency: {}", e))?;

    let account_service = wallet_core::AccountService::new(state.db.clone());

    match account_service
        .create_account(name, account_type, parent_id, currency)
        .await
    {
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
pub async fn get_account_tree_filtered(
    state: State<'_, AppState>,
    include_inactive: bool,
) -> Result<Vec<AccountNode>, String> {
    let account_service = wallet_core::AccountService::new(state.db.clone());
    match account_service.get_account_tree_filtered(include_inactive).await {
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
pub async fn get_transaction(state: State<'_, AppState>, id: i64) -> Result<Transaction, String> {
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
    let currency =
        Currency::new(&currency_code, 2, "€").map_err(|e| format!("Invalid currency: {}", e))?;
    let amount = Money::from_minor_units(amount_cents, currency);

    let transaction_service = TransactionService::new(state.db.clone());
    match transaction_service
        .create_simple_transaction(description, date, amount, from_account_id, to_account_id)
        .await
    {
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
    match account_service
        .calculate_balance_with_children(account_id)
        .await
    {
        Ok(balance) => Ok(balance),
        Err(e) => Err(format!("Failed to calculate hierarchical balance: {}", e)),
    }
}

// Dashboard metric commands

#[tauri::command]
#[specta::specta]
pub async fn get_net_worth(state: State<'_, AppState>) -> Result<Money, String> {
    let report_service = ReportService::new(state.db.clone());
    match report_service.get_net_worth().await {
        Ok(net_worth) => Ok(net_worth),
        Err(e) => Err(format!("Failed to calculate net worth: {}", e)),
    }
}

#[tauri::command]
#[specta::specta]
pub async fn get_total_assets(state: State<'_, AppState>) -> Result<Money, String> {
    let report_service = ReportService::new(state.db.clone());
    match report_service.get_total_assets().await {
        Ok(total_assets) => Ok(total_assets),
        Err(e) => Err(format!("Failed to calculate total assets: {}", e)),
    }
}

#[tauri::command]
#[specta::specta]
pub async fn get_current_month_income(state: State<'_, AppState>) -> Result<Money, String> {
    let report_service = ReportService::new(state.db.clone());
    match report_service.get_current_month_income().await {
        Ok(income) => Ok(income),
        Err(e) => Err(format!("Failed to calculate monthly income: {}", e)),
    }
}

#[tauri::command]
#[specta::specta]
pub async fn get_current_month_expenses(state: State<'_, AppState>) -> Result<Money, String> {
    let report_service = ReportService::new(state.db.clone());
    match report_service.get_current_month_expenses().await {
        Ok(expenses) => Ok(expenses),
        Err(e) => Err(format!("Failed to calculate monthly expenses: {}", e)),
    }
}

#[tauri::command]
#[specta::specta]
pub async fn get_recent_transactions(
    state: State<'_, AppState>,
    limit: Option<u32>,
) -> Result<Vec<Transaction>, String> {
    let report_service = ReportService::new(state.db.clone());
    let transaction_limit = limit.unwrap_or(10);

    match report_service
        .get_recent_transactions(transaction_limit)
        .await
    {
        Ok(transactions) => Ok(transactions),
        Err(e) => Err(format!("Failed to get recent transactions: {}", e)),
    }
}

#[tauri::command]
#[specta::specta]
pub async fn update_account(
    state: State<'_, AppState>,
    account_id: i64,
    name: String,
    description: Option<String>,
) -> Result<Account, String> {
    let account_service = AccountService::new(state.db.clone());
    
    // Get the current account
    let mut account = match account_service.get_account(account_id).await {
        Ok(account) => account,
        Err(e) => return Err(format!("Failed to get account: {}", e)),
    };
    
    // Update the fields
    account.name = name.trim().to_string();
    account.description = description;
    account.updated_at = chrono::Utc::now();
    
    // Validate name is not empty
    if account.name.is_empty() {
        return Err("Account name cannot be empty".to_string());
    }
    
    // Update the account
    match account_service.update_account(&account).await {
        Ok(updated_account) => Ok(updated_account),
        Err(e) => Err(format!("Failed to update account: {}", e)),
    }
}

#[tauri::command]
#[specta::specta]
pub async fn deactivate_account(
    state: State<'_, AppState>,
    account_id: i64,
) -> Result<(), String> {
    let account_service = AccountService::new(state.db.clone());
    
    match account_service.deactivate_account(account_id).await {
        Ok(()) => Ok(()),
        Err(e) => Err(format!("Failed to deactivate account: {}", e)),
    }
}
