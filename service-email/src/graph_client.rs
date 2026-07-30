// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

use reqwest::Client;
use serde_json::{Value, json};
use std::error::Error;

/// Folder-scoped messages endpoint. `folder` is a Microsoft Graph well-known
/// folder name (`inbox`, `sentitems`, `drafts`, ...) or a real mailFolder id.
/// Scoping to one folder — rather than the whole mailbox — is the deliberate
/// minimum-viable shape per `BRIEF-os-totebox-platform.md` §14 #21: a
/// concrete, bounded demonstration of external data flowing into the
/// DataGraph, not a full mailbox crawl.
pub fn folder_messages_url(target_user: &str, folder: &str) -> String {
    format!(
        "https://graph.microsoft.com/v1.0/users/{}/mailFolders/{}/messages?$filter=isRead eq false&$top=500",
        target_user, folder
    )
}

pub struct GraphBridge {
    client: Client,
    token: String,
}

impl GraphBridge {
    pub fn new(token: String) -> Self {
        Self {
            client: Client::new(),
            token,
        }
    }

    // Phase 1: High-Velocity URL Extraction (Accepts initial query or nextLink)
    pub async fn fetch_url(&self, url: &str) -> Result<Value, Box<dyn Error>> {
        let res = self.client.get(url)
            .bearer_auth(&self.token)
            .send()
            .await?;

        if !res.status().is_success() {
            return Err(format!("Graph API extraction failed: {}", res.status()).into());
        }

        let body = res.json::<Value>().await?;
        Ok(body)
    }

    // Phase 2: State Mutation
    pub async fn mutate_state(&self, target_user: &str, message_id: &str) -> Result<(), Box<dyn Error>> {
        let url = format!(
            "https://graph.microsoft.com/v1.0/users/{}/messages/{}",
            target_user, message_id
        );
        let res = self.client.patch(&url)
            .bearer_auth(&self.token)
            .json(&json!({"isRead": true}))
            .send()
            .await?;

        if !res.status().is_success() {
            return Err(format!("State mutation failed: {}", res.status()).into());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn folder_messages_url_scopes_to_the_given_user_and_folder() {
        let url = folder_messages_url("jennifer@pointsav.com", "inbox");
        assert!(url.contains("/users/jennifer@pointsav.com/mailFolders/inbox/messages"));
        assert!(url.contains("isRead eq false"));
    }

    #[test]
    fn folder_messages_url_accepts_a_real_mailfolder_id_too() {
        let url = folder_messages_url("jennifer@pointsav.com", "AAMkAGI1AAAA");
        assert!(url.contains("/mailFolders/AAMkAGI1AAAA/messages"));
    }
}
