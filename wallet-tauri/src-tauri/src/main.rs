// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(debug_assertions)]
use specta_typescript::Typescript;
#[cfg(debug_assertions)]
use tauri_specta::Builder;

fn main() {
    #[cfg(debug_assertions)]
    {
        Builder::<tauri::Wry>::new()
            .typ::<wallet_core::models::account::Account>()
            .export(
                Typescript::default().bigint(specta_typescript::BigIntExportBehavior::BigInt),
                "../src/bindings.ts",
            )
            .expect("Failed to export typescript bindings");
    }
    wallet_tauri_lib::run()
}
