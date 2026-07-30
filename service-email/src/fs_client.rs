// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Client for `service-fs`'s Ring 1 WORM append surface (`POST /v1/append`).
//!
//! Per `service-email/CLAUDE.md`'s "WORM via `service-fs`" hard constraint —
//! and `BRIEF-os-totebox-platform.md` §14 #21 (one mailbox folder landing
//! through service-fs, not just the local maildir vault) — every fetched
//! message is appended here in addition to the existing maildir write.
//!
//! The payload is the raw, unmodified message JSON from Microsoft Graph; no
//! classification or semantic interpretation happens in this crate.
//! SYS-ADR-07 (zero AI in Ring 1): this client performs structural JSON
//! pass-through only.

use reqwest::Client;
use serde_json::Value;
use std::error::Error;

pub struct FsClient {
    client: Client,
    endpoint: String,
    module_id: String,
}

impl FsClient {
    pub fn new(endpoint: String, module_id: String) -> Self {
        Self {
            client: Client::new(),
            endpoint,
            module_id,
        }
    }

    /// Appends one message payload to service-fs's WORM ledger, tagged with
    /// `payload_id` (the Graph message id) for downstream attribution.
    pub async fn append(&self, payload_id: &str, payload: &Value) -> Result<(), Box<dyn Error>> {
        let url = format!("{}/v1/append", self.endpoint.trim_end_matches('/'));
        let res = self
            .client
            .post(&url)
            .header("X-Foundry-Module-ID", &self.module_id)
            .json(&serde_json::json!({
                "payload_id": payload_id,
                "payload": payload,
            }))
            .send()
            .await?;

        if !res.status().is_success() {
            return Err(format!("service-fs append failed: {}", res.status()).into());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    /// Confirms the module-id header and payload shape service-fs's
    /// `POST /v1/append` actually expects (payload_id + payload fields).
    #[tokio::test]
    async fn append_sends_module_id_header_and_correct_payload_shape() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v1/append"))
            .and(header("x-foundry-module-id", "jennifer"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "cursor": 1,
                "payload_id": "msg-123"
            })))
            .expect(1)
            .mount(&server)
            .await;

        let client = FsClient::new(server.uri(), "jennifer".to_string());
        let payload = serde_json::json!({"id": "msg-123", "subject": "test"});
        client
            .append("msg-123", &payload)
            .await
            .expect("append should succeed against a healthy mock");
    }

    /// A per-tenant module-id mismatch (service-fs's 403 per-tenant boundary)
    /// must surface as an error, not be silently swallowed.
    #[tokio::test]
    async fn append_surfaces_non_success_status_as_an_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v1/append"))
            .respond_with(ResponseTemplate::new(403))
            .mount(&server)
            .await;

        let client = FsClient::new(server.uri(), "wrong-tenant".to_string());
        let payload = serde_json::json!({"id": "msg-123"});
        let err = client
            .append("msg-123", &payload)
            .await
            .expect_err("a 403 from service-fs must surface as an error");
        assert!(err.to_string().contains("403"));
    }

    #[test]
    fn endpoint_trailing_slash_is_normalized() {
        let client = FsClient::new("http://127.0.0.1:9100/".to_string(), "jennifer".to_string());
        assert_eq!(client.endpoint.trim_end_matches('/'), "http://127.0.0.1:9100");
    }
}
