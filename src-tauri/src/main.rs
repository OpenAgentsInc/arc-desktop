#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use nostr_sdk::prelude::*;

const PRIVATE_KEY: &str = "4540484eedb0bc5ba1209cea76ff7b6a77fee473708b113eac726058580267ad";

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tokio::main]
async fn main() -> Result<()> {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
