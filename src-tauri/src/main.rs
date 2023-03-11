#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::str::FromStr;
use std::time::Duration;

use nostr_sdk::prelude::*;
use tokio::time;

const PRIVATE_KEY: &str = "4540484eedb0bc5ba1209cea76ff7b6a77fee473708b113eac726058580267ad";

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tokio::main]
async fn main() -> Result<()> {
    let secret_key = SecretKey::from_str(PRIVATE_KEY)?;
    let my_keys = Keys::new(secret_key);

    let message = format!(
        "Hello from NDK tutorial! My public key is: {}",
        my_keys.public_key()
    );
    println!("{}", message);

    let opts = Options::new().wait_for_send(true);
    let client = Client::new_with_opts(&my_keys, opts);
    client.add_relay("wss://relay.damus.io", None).await?;
    client.add_relay("wss://arc1.arcadelabs.co", None).await?;

    client.connect().await;

    let event_id = client.publish_text_note(message, &[]).await?;
    println!("{}", event_id);

    // Retrieve all the events that we have posted
    let filter = Filter {
        ids: None,
        authors: Some(vec![my_keys.public_key()]),
        kinds: None,
        events: None,
        pubkeys: None,
        hashtags: None,
        references: None,
        search: None,
        since: None,
        until: None,
        limit: None,
    };

    time::sleep(Duration::from_secs(1)).await;

    let events = client.get_events_of(vec![filter], None).await?;
    println!("{:#?}", events);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
