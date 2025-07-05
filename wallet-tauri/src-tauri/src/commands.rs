use tauri::State;
use wallet_core::Account;

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
