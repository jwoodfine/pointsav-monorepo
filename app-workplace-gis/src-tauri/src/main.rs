// Workplace*GIS — sovereign desktop GIS viewer shell
// Copyright © 2026 PointSav Digital Systems
// Licensed under the Apache License, Version 2.0

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::fs;
use tauri::Manager;

/// Default tile-source endpoint used until the operator configures one via
/// the first-run dialog. Matches the CLAUDE.md architecture note: production
/// default is `gis.woodfinegroup.com`, overridable to a PPN (WireGuard)
/// address such as `http://10.8.0.9:9200`-style host for local iteration.
const DEFAULT_ENDPOINT: &str = "https://gis.woodfinegroup.com";
const CONFIG_FILENAME: &str = "gis-config.json";

#[derive(Debug, Serialize, Deserialize)]
struct GisConfig {
    endpoint: String,
}

/// Wave 2 scope names three cluster layers (T1/T2/T3) with no further tile
/// schema documented yet. This is a static placeholder list — once the real
/// tile server's style/layer contract is known, replace with a call that
/// reads the layer list from the endpoint's style document instead.
#[derive(Debug, Serialize, Deserialize, Clone)]
struct LayerInfo {
    id: String,
    label: String,
}

fn default_layers() -> Vec<LayerInfo> {
    vec![
        LayerInfo {
            id: "t1".into(),
            label: "T1 — Tier 1 clusters".into(),
        },
        LayerInfo {
            id: "t2".into(),
            label: "T2 — Tier 2 clusters".into(),
        },
        LayerInfo {
            id: "t3".into(),
            label: "T3 — Tier 3 clusters".into(),
        },
    ]
}

#[derive(Debug, Serialize)]
struct GeoJsonFile {
    path: String,
    contents: String,
}

// `config_path`/`load_config` helpers moved to the shared
// `workplace-shell-chrome` crate (2026-07-14 retrofit) — see that crate's
// README for why. `GisConfig`'s shape stays here since it's app-specific.

#[tauri::command]
fn get_tile_endpoint(app_handle: tauri::AppHandle) -> String {
    app_handle
        .path_resolver()
        .app_data_dir()
        .and_then(|dir| workplace_shell_chrome::load_config::<GisConfig>(&dir, CONFIG_FILENAME))
        .map(|cfg| cfg.endpoint)
        .unwrap_or_else(|| DEFAULT_ENDPOINT.to_string())
}

#[tauri::command]
fn set_tile_endpoint(app_handle: tauri::AppHandle, endpoint: String) -> Result<(), String> {
    let dir = app_handle
        .path_resolver()
        .app_data_dir()
        .ok_or("Cannot resolve app data directory")?;
    workplace_shell_chrome::save_config(&dir, CONFIG_FILENAME, &GisConfig { endpoint })
}

/// True once `gis-config.json` exists — used by the frontend to decide
/// whether to show the first-run endpoint-configuration dialog.
#[tauri::command]
fn has_gis_config(app_handle: tauri::AppHandle) -> bool {
    app_handle
        .path_resolver()
        .app_data_dir()
        .map(|dir| workplace_shell_chrome::has_config(&dir, CONFIG_FILENAME))
        .unwrap_or(false)
}

/// Static Wave-2 layer list (see `default_layers` doc comment). Returned as
/// an IPC command rather than a plain constant so the frontend has one
/// stable call site to swap for a real style-derived layer list later.
#[tauri::command]
fn get_available_layers() -> Vec<LayerInfo> {
    default_layers()
}

/// Opens a native file-picker (via the `dialog-open` allowlist feature) for
/// the operator to load a local GeoJSON overlay (e.g. an exported cluster
/// snapshot) and returns its raw contents for the frontend to add as a
/// MapLibre GeoJSON source. Returns `Ok(None)` if the dialog is cancelled.
#[tauri::command]
async fn load_geojson_file() -> Result<Option<GeoJsonFile>, String> {
    // Blocking dialog must not run on the main thread; spawn_blocking gives us
    // a safe thread and tauri's dialog internally re-dispatches UI work to the
    // main thread as required per platform.
    let picked = tauri::async_runtime::spawn_blocking(|| {
        tauri::api::dialog::blocking::FileDialogBuilder::new()
            .add_filter("GeoJSON", &["json", "geojson"])
            .pick_file()
    })
    .await
    .map_err(|e| format!("File dialog task failed: {}", e))?;

    let Some(path) = picked else {
        return Ok(None);
    };

    let contents = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    Ok(Some(GeoJsonFile {
        path: path.to_string_lossy().to_string(),
        contents,
    }))
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_tile_endpoint,
            set_tile_endpoint,
            has_gis_config,
            get_available_layers,
            load_geojson_file
        ])
        .setup(|app| {
            if let Some(dir) = app.path_resolver().app_data_dir() {
                workplace_shell_chrome::ensure_app_data_dir(&dir).ok();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running workplace-gis");
}
