// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

mod maildir;
mod graph_client;
mod auth;
mod fs_client;

use maildir::MaildirVault;
use graph_client::GraphBridge;
use fs_client::FsClient;
use std::env;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("SYSTEM EVENT: Initializing Sovereign Email Bridge daemon (Anti-Throttling Mode).");

    let archive_path = env::var("TOTEBOX_ARCHIVE_PATH")
        .unwrap_or_else(|_| "/assets/personnel-maildir".to_string());
    let target_user = env::var("EXCHANGE_TARGET_USER")
        .expect("FATAL: EXCHANGE_TARGET_USER missing.");
    // Scoped to one folder per BRIEF-os-totebox-platform.md §14 #21 — defaults
    // to "inbox" rather than crawling the whole mailbox.
    let target_folder =
        env::var("EXCHANGE_TARGET_FOLDER").unwrap_or_else(|_| "inbox".to_string());

    let service_module_id =
        env::var("SERVICE_EMAIL_MODULE_ID").expect("FATAL: SERVICE_EMAIL_MODULE_ID missing.");
    let fs_endpoint = env::var("SERVICE_EMAIL_FS_ENDPOINT")
        .unwrap_or_else(|_| "http://127.0.0.1:9100".to_string());
    let fs = FsClient::new(fs_endpoint, service_module_id);

    let vault = MaildirVault::init(&archive_path)?;
    println!("SYSTEM EVENT: Totebox archive verified at {}", archive_path);
    println!(
        "SYSTEM EVENT: Entering persistent polling loop. Target: {} folder: {}",
        target_user, target_folder
    );

    loop {
        match auth::token_from_env() {
            Ok(token) => {
                let bridge = GraphBridge::new(token);

                let mut current_url = graph_client::folder_messages_url(&target_user, &target_folder);

                loop {
                    match bridge.fetch_url(&current_url).await {
                        Ok(messages) => {
                            if let Some(msg_array) = messages["value"].as_array() {
                                if !msg_array.is_empty() {
                                    let mut mutation_count = 0;
                                    for msg in msg_array {
                                        let raw_json = msg.to_string();

                                        if vault.write_payload(&raw_json).is_ok() {
                                            if let Some(msg_id) = msg["id"].as_str() {
                                                // Land the raw, unmodified message through service-fs's
                                                // WORM append surface — structural JSON pass-through only
                                                // (SYS-ADR-07: zero AI in Ring 1), in addition to the
                                                // existing local maildir vault write above.
                                                if let Err(e) = fs.append(msg_id, msg).await {
                                                    eprintln!("SYSTEM ERROR: service-fs append failed for {} - REASON: {}", msg_id, e);
                                                }
                                                match bridge.mutate_state(&target_user, msg_id).await {
                                                    Ok(_) => {
                                                        mutation_count += 1;
                                                    }
                                                    Err(e) => {
                                                        eprintln!("SYSTEM ERROR: Failed to mutate state for {} - REASON: {}", msg_id, e);
                                                    }
                                                }
                                                // ANTI-THROTTLING INJECTION: 50ms physical sleep between mutations
                                                tokio::time::sleep(Duration::from_millis(50)).await;
                                            }
                                        }
                                    }
                                    println!("SYSTEM EVENT: Extracted and mutated {} payloads to local archive.", mutation_count);
                                }
                            }
                            
                            if let Some(next_link) = messages["@odata.nextLink"].as_str() {
                                println!("SYSTEM EVENT: Network pagination detected. Bypassing throttle to recursively extract next batch.");
                                current_url = next_link.to_string();
                            } else {
                                break; 
                            }
                        }
                        Err(e) => {
                            eprintln!("SYSTEM ERROR: Graph Bridge extraction failed: {}", e);
                            break; 
                        }
                    }
                }
            }
            Err(e) => eprintln!("SYSTEM ERROR: Token acquisition failed: {}", e),
        }
        
        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}
