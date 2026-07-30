// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Per the 2026-04-25 operator-confirmed EWS auth rebase (`CLAUDE.md`'s
//! "Auth runs out-of-process" hard constraint): the access token is
//! pre-acquired externally and passed in via `AZURE_ACCESS_TOKEN`. This
//! daemon does not perform an OAuth `client_credentials` handshake inline —
//! that in-process flow was the documented drift this rebase closes.
//! Token refresh is an upstream concern, not this crate's.

use std::env;

pub fn token_from_env() -> Result<String, String> {
    env::var("AZURE_ACCESS_TOKEN").map_err(|_| {
        "AZURE_ACCESS_TOKEN missing — the token must be pre-acquired out-of-process \
         per the EWS auth rebase (service-email/CLAUDE.md); this daemon does not run \
         an OAuth handshake inline"
            .to_string()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    // AZURE_ACCESS_TOKEN is a shared process-global env var; serialize this
    // module's tests against each other so they can't race across threads.
    static ENV_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn token_from_env_reads_present_value_and_errors_clearly_when_missing() {
        let _guard = ENV_LOCK.lock().unwrap();

        unsafe {
            env::remove_var("AZURE_ACCESS_TOKEN");
        }
        let missing = token_from_env();
        assert!(missing.is_err());
        assert!(missing.unwrap_err().contains("AZURE_ACCESS_TOKEN"));

        unsafe {
            env::set_var("AZURE_ACCESS_TOKEN", "test-token-value");
        }
        let present = token_from_env();
        unsafe {
            env::remove_var("AZURE_ACCESS_TOKEN");
        }
        assert_eq!(present.unwrap(), "test-token-value");
    }
}
