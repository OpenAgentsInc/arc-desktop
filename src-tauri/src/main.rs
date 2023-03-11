#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::str::FromStr;
use std::time::Duration;
use tauri::{Manager, Window};

use nostr_sdk::prelude::*;
use tokio::time;

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
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

    // window.emit("events", events).unwrap();

    tauri::Builder::default()
        .setup(|app| {
            // `main` here is the window label; it is defined on the window creation or under `tauri.conf.json`
            // the default value is `main`. note that it must be unique
            let main_window = app.get_window("main").unwrap();

            // listen to the `event-name` (emitted on the `main` window)
            let id = main_window.listen("event-name", |event| {
                println!("got window event-name with payload {:?}", event.payload());
            });
            // unlisten to the event using the `id` returned on the `listen` function
            // an `once` API is also exposed on the `Window` struct
            main_window.unlisten(id);

            // emit the `event-name` event to the `main` window
            main_window
                .emit(
                    "event-name",
                    Payload {
                        message: "Tauri is cool!!!!!!".into(),
                    },
                )
                .unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
