use tauri::State;
use wallet_core::{Account, AccountType, Currency};

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
