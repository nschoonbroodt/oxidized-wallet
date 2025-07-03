#[cfg(debug_assertions)]
use specta_typescript::Typescript;
use tauri_specta::{collect_commands, Builder};

#[tauri::command]
#[specta::specta]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = Builder::<tauri::Wry>::new().commands(collect_commands![greet,]);
    #[cfg(debug_assertions)]
    {
        builder
            .export(
                Typescript::default().bigint(specta_typescript::BigIntExportBehavior::BigInt),
                "../src/bindings.ts",
            )
            .expect("Failed to export typescript bindings");
    }
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(builder.invoke_handler())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
