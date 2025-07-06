mod commands;

use std::sync::Arc;

#[cfg(debug_assertions)]
use specta_typescript::Typescript;
use tauri::Manager;
use tauri_specta::{collect_commands, Builder};
use wallet_core::db::connection::Database;

pub struct AppState {
    db: Arc<Database>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = Builder::<tauri::Wry>::new().commands(collect_commands![
        commands::get_accounts,
        commands::create_account,
        commands::get_account_tree,
        commands::get_transactions,
        commands::get_transaction,
        commands::create_simple_transaction,
        commands::get_account_balance,
        commands::get_account_balance_with_children,
        commands::get_net_worth,
        commands::get_total_assets,
        commands::get_current_month_income,
        commands::get_current_month_expenses,
        commands::get_recent_transactions,
    ]);
    #[cfg(debug_assertions)]
    {
        builder
            .export(
                Typescript::default()
                    .bigint(specta_typescript::BigIntExportBehavior::BigInt)
                    .header("// @ts-nocheck\n/* eslint-disable */"),
                "../src/bindings.ts",
            )
            .expect("Failed to export typescript bindings");
    }
    tauri::Builder::default()
        .setup(|app| {
            let app_dir = app.path().app_data_dir()?; // Convert Option to Result

            std::fs::create_dir_all(&app_dir)?;
            let db_path = app_dir.join("wallet.db");

            let runtime = tokio::runtime::Runtime::new()?;
            let db = runtime.block_on(async {
                let db = Database::new(db_path.to_str().unwrap()).await?;
                db.migrate().await?;
                Ok::<_, Box<dyn std::error::Error>>(db)
            })?;

            let state = AppState { db: Arc::new(db) };
            app.manage(state);

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(builder.invoke_handler())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
