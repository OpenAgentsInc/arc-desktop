#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use nostr_sdk::prelude::*;
use serde_json::Value;
use std::str::FromStr;
use std::time::Duration;
use tokio::time;
use urlencoding;

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

fn build_lnurl_pay_url(
    service_url: &str,
    amount: Option<u64>,
    metadata: &[Vec<serde_json::Value>],
    callback: Option<&str>,
) -> String {
    // Create a new builder.
    let mut builder = String::from(service_url);
    builder.push_str("?tag=payRequest");

    // Set the amount, if it's specified.
    if let Some(amount) = amount {
        builder.push_str("&maxSendable=");
        builder.push_str(&amount.to_string());
        builder.push_str("&minSendable=");
        builder.push_str(&amount.to_string());
    }

    // Set the metadata.
    builder.push_str("&metadata=");
    builder.push_str(&urlencoding::encode(
        &serde_json::to_string(metadata).unwrap(),
    ));

    // Set the callback, if it's specified.
    if let Some(callback) = callback {
        builder.push_str("&callback=");
        builder.push_str(callback);
    }

    builder
}

const PRIVATE_KEY: &str = "4540484eedb0bc5ba1209cea76ff7b6a77fee473708b113eac726058580267ad";

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tokio::main]
async fn main() -> Result<()> {
    let secret_key = SecretKey::from_str(PRIVATE_KEY)?;
    let my_keys = Keys::new(secret_key);

    let opts = Options::new().wait_for_send(true);
    let client = Client::new_with_opts(&my_keys, opts);
    // client.add_relay("wss://relay.damus.io", None).await?;
    client.add_relay("wss://arc1.arcadelabs.co", None).await?;
    client.connect().await;

    // Retrieve a maximum of 25 kind-0 events from the relays
    let filter = Filter {
        ids: Some(vec![
            "2837654f9543106a5ea3a74d24308d3317a4bc2d9ebd1fd1bcb7781e2d1c3cd2".to_string(),
        ]),
        authors: None,
        kinds: Some(vec![Kind::Ephemeral(0)]),
        events: None,
        pubkeys: None,
        hashtags: None,
        references: None,
        search: None,
        since: None,
        until: None,
        limit: Some(1),
    };

    time::sleep(Duration::from_secs(1)).await;

    let events = client.get_events_of(vec![filter], None).await?;
    println!("{:#?}", events);

    for event in events.iter() {
        let content: Value = serde_json::from_str(&event.content)?;
        let lud16 = content["lud16"].as_str().unwrap_or("");
        println!("{}", lud16);

        let metadata = vec![vec![
            serde_json::Value::String(String::from("text/plain")),
            serde_json::Value::String(String::from("Test payment")),
        ]];

        let pay_url = build_lnurl_pay_url(
            lud16,
            Some(10000),
            &[vec![
                serde_json::json!("text/plain"),
                serde_json::json!("Test payment"),
            ]],
            Some("https://example.com/payments/callback"),
        );

        println!("Pay URL: {}", pay_url);
    }

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
