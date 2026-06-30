//! Phase S3 — fleet watch + WireGuard peer-table + WORM ledger (daemon mode).
//!
//! Required env vars:
//!   WG_IFACE          — WireGuard interface name (default: wg0)
//!   FLEET_URL         — HTTP endpoint returning approved peers as JSONL (optional)
//!   SERVICE_FS_URL    — service-fs base URL for WORM /v1/append (optional)
//!   NODES_JSONL_PATH  — approved-pubkeys file (default: ~/.local/share/ppn/nodes.jsonl)
//!
//! Runtime requirement: CAP_NET_ADMIN (or root) for `wg set`.

use std::collections::HashSet;
use std::fs;
use std::io::{self, BufRead};
use std::process::Command;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const POLL_INTERVAL_SECS: u64 = 30;

pub fn run() -> io::Result<()> {
    let wg_iface = std::env::var("WG_IFACE").unwrap_or_else(|_| "wg0".to_string());
    let fleet_url = std::env::var("FLEET_URL").ok();
    let service_fs_url = std::env::var("SERVICE_FS_URL").ok();
    let nodes_path = nodes_jsonl_path();

    eprintln!("os-network-admin daemon starting (Phase S3)");
    eprintln!("  interface:   {wg_iface}");
    eprintln!("  nodes file:  {nodes_path}");
    if let Some(ref u) = fleet_url {
        eprintln!("  fleet URL:   {u}");
    }
    if let Some(ref u) = service_fs_url {
        eprintln!("  service-fs:  {u}");
    }

    let mut known: HashSet<String> = load_known_pubkeys(&nodes_path);
    eprintln!("  {} known peer(s) loaded", known.len());

    loop {
        match approved_peers(&fleet_url, &nodes_path) {
            Ok(peers) => {
                for (pubkey, wg_ip) in &peers {
                    if known.contains(pubkey.as_str()) {
                        continue;
                    }
                    match add_wg_peer(&wg_iface, pubkey, wg_ip) {
                        Ok(()) => {
                            eprintln!("peer added: {pubkey} ({wg_ip}/32)");
                            append_worm_event(service_fs_url.as_deref(), pubkey, &wg_iface);
                            known.insert(pubkey.clone());
                        }
                        Err(e) => eprintln!("wg set failed for {pubkey}: {e}"),
                    }
                }
            }
            Err(e) => eprintln!("fleet poll error: {e}"),
        }
        std::thread::sleep(Duration::from_secs(POLL_INTERVAL_SECS));
    }
}

fn nodes_jsonl_path() -> String {
    std::env::var("NODES_JSONL_PATH").unwrap_or_else(|_| {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/root".to_string());
        format!("{home}/.local/share/ppn/nodes.jsonl")
    })
}

fn load_known_pubkeys(path: &str) -> HashSet<String> {
    let mut set = HashSet::new();
    let Ok(file) = fs::File::open(path) else {
        return set;
    };
    for line in io::BufReader::new(file).lines().flatten() {
        if let Some(pk) = json_str_field(&line, "public_key") {
            set.insert(pk);
        }
    }
    set
}

// Returns approved (pubkey, wg_ip) pairs.
// Falls back to nodes.jsonl when FLEET_URL is unset or HTTP polling is not yet wired.
fn approved_peers(
    fleet_url: &Option<String>,
    nodes_path: &str,
) -> io::Result<Vec<(String, String)>> {
    if fleet_url.is_some() {
        // TODO: HTTP GET fleet_url, parse JSONL response into Vec<(pubkey, wg_ip)>.
        // Requires reqwest or ureq — wired when fleet HTTP endpoint is live.
        eprintln!("FLEET_URL set but HTTP polling not yet implemented; reading nodes.jsonl");
    }

    let mut peers = Vec::new();
    let Ok(file) = fs::File::open(nodes_path) else {
        return Ok(peers);
    };
    for line in io::BufReader::new(file).lines().flatten() {
        if let (Some(pk), Some(ip)) = (
            json_str_field(&line, "public_key"),
            json_str_field(&line, "wg_ip"),
        ) {
            peers.push((pk, ip));
        }
    }
    Ok(peers)
}

fn add_wg_peer(iface: &str, pubkey: &str, wg_ip: &str) -> io::Result<()> {
    let status = Command::new("wg")
        .args([
            "set",
            iface,
            "peer",
            pubkey,
            "allowed-ips",
            &format!("{wg_ip}/32"),
        ])
        .status()?;
    if !status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("wg set exited with {status}"),
        ));
    }
    Ok(())
}

fn append_worm_event(service_fs_url: Option<&str>, pubkey: &str, iface: &str) {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let payload = format!(
        r#"{{"schema_version":"1","event":"peer_added","ts":"{ts}","pubkey":"{pubkey}","iface":"{iface}"}}"#
    );
    if let Some(_url) = service_fs_url {
        // TODO: HTTP POST to {_url}/v1/append with payload.
        eprintln!("WORM(stub): {payload}");
    } else {
        eprintln!("WORM(local): {payload}");
    }
}

// Minimal JSON string-field extractor for flat JSONL objects.
// Finds `"key":"value"` without pulling in serde_json.
fn json_str_field(line: &str, key: &str) -> Option<String> {
    let needle = format!("\"{key}\":\"");
    let start = line.find(&needle)? + needle.len();
    let rest = &line[start..];
    let end = rest.find('"')?;
    Some(rest[..end].to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_str_field_extracts_value() {
        let line = r#"{"public_key":"abc123","wg_ip":"10.0.0.2"}"#;
        assert_eq!(json_str_field(line, "public_key"), Some("abc123".into()));
        assert_eq!(json_str_field(line, "wg_ip"), Some("10.0.0.2".into()));
    }

    #[test]
    fn json_str_field_missing_key_returns_none() {
        let line = r#"{"public_key":"abc123"}"#;
        assert_eq!(json_str_field(line, "wg_ip"), None);
    }

    #[test]
    fn json_str_field_empty_value() {
        let line = r#"{"public_key":""}"#;
        assert_eq!(json_str_field(line, "public_key"), Some("".into()));
    }
}
