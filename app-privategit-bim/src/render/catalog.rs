// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! "The Plan Room" (2026-07-09 v3 redesign) — real, server-rendered routes
//! over the same underlying library data:
//!
//! * **Objects** (`/objects`, `/objects/{slug}`) — atomic building-component
//!   specifications, walked from `interior.dtcg.json`'s furniture BIM
//!   Objects. Each carries its verified IFC 4.3 entity class and Uniclass
//!   2015 **Pr** (Products) code, and a hand-authored inline SVG plan symbol
//!   (`plan_symbols`) in place of the old letter-monogram thumbnail.
//! * **Compositions** (`/compositions`, `/compositions/{slug}`,
//!   `/compositions/{slug}/o/{object}`) — Key Plans, walked from
//!   `key-plans.dtcg.json` with the shared `card::collect_kp_leaves` helper
//!   and drawn with `svg::render_kp_zone_svg_from_value` (recolored to the
//!   single navy line-work language — see `svg.rs`). The detail page is a
//!   plan-anchored inspector: the key-plan drawing stays fixed on the left;
//!   the right rail swaps between the composition's own Data Box/bill and an
//!   inspected object's spec sheet, with a breadcrumb and back-link — no
//!   660px slide-over modal, no full-context loss on click-through.
//! * **Home** (`/`) — a compact registry front door: masthead + the two
//!   shelves (Objects/Compositions) + prose sourced from
//!   `site-content/pages/home.md`.
//!
//! Every page is a real URL; search and facet state are GET query params, so
//! Back/reload/link-sharing all work by construction (fixes the 2026-07
//! audit's "about:blank on Back", "silent state wipe on reload", and
//! "unshareable filtered views" findings simultaneously). Facet chips are
//! plain `<a href>` toggles — the whole catalog is fully functional with
//! JavaScript disabled.
//!
//! Honest-partial-completion convention (2026-07 audit's "83% of
//! Compositions show no constituent Objects" finding): only PO-1 carries a
//! real, structured `furniture_refs` array. Every other Composition's
//! "Parts list" renders an "N of M parts linked to the catalog" status chip
//! instead of a fabricated bill or a false "assembled from 0 object
//! entries" line. (Compositions with zero furniture data at all are kept
//! out of the public grid entirely — see `composition_is_publicly_visible`
//! — rather than showing this chip at 0 of 0.)

use crate::state::AppState;
use serde_json::{json, Map, Value};

use super::card::collect_kp_leaves;
use super::shell::esc;
use super::svg::render_kp_zone_svg_from_value;

// Category display order + labels shared by cards, facets, and the catalog
// payload. Space labels are the Uniclass 2015 SL (Spaces/locations) *framing*
// per key-plans.dtcg.json's own `$description` ("Classified at Uniclass 2015
// SL level") — a descriptive space-type, deliberately not a fabricated
// numeric code (the Objects tab's Pr codes are the real, verified ones).
const CATEGORY_ORDER: &[&str] = &[
    "private-office",
    "corporate-office",
    "medical",
    "business",
    "laboratory",
    "academic",
    "civic",
];

fn category_label(cat: &str) -> &'static str {
    match cat {
        "private-office" => "Private Office",
        "corporate-office" => "Corporate Office",
        "medical" => "Medical",
        "business" => "Business",
        "laboratory" => "Laboratory",
        "academic" => "Academic",
        "civic" => "Civic",
        _ => "Other",
    }
}

fn category_space(cat: &str) -> &'static str {
    match cat {
        "private-office" => "Private office spaces",
        "corporate-office" => "Open-plan office areas",
        "medical" => "Health & care spaces",
        "business" => "General office spaces",
        "laboratory" => "Laboratory spaces",
        "academic" => "Teaching & learning spaces",
        "civic" => "Civic & community spaces",
        _ => "Office spaces",
    }
}

fn category_rank(cat: &str) -> usize {
    CATEGORY_ORDER
        .iter()
        .position(|c| *c == cat)
        .unwrap_or(CATEGORY_ORDER.len())
}

// ── small value helpers ─────────────────────────────────────────────────────

fn s<'a>(v: &'a Value, key: &str) -> &'a str {
    v.get(key).and_then(Value::as_str).unwrap_or("")
}

fn int_of(v: &Value, key: &str) -> Option<i64> {
    v.get(key).and_then(Value::as_i64)
}

fn f_of(v: &Value, key: &str) -> Option<f64> {
    v.get(key).and_then(Value::as_f64)
}

fn str_vec(v: &Value, key: &str) -> Vec<String> {
    v.get(key)
        .and_then(Value::as_array)
        .map(|a| {
            a.iter()
                .filter_map(|x| x.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default()
}

/// Round to at most two decimals, trailing zeros trimmed (5.9944 → "5.99").
fn round2(n: f64) -> String {
    let r = (n * 100.0).round() / 100.0;
    if r.fract().abs() < 1e-9 {
        format!("{}", r as i64)
    } else {
        let mut out = format!("{r:.2}");
        while out.ends_with('0') {
            out.pop();
        }
        if out.ends_with('.') {
            out.pop();
        }
        out
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Objects — furniture BIM Objects from interior.dtcg.json
// ─────────────────────────────────────────────────────────────────────────────

/// Set of `.ifc` filenames actually present in `blocks/furniture/`. Used to
/// decide whether an Object gets a download link — never fabricate a filename.
fn ifc_file_set(state: &AppState) -> std::collections::HashSet<String> {
    let dir = state.config.library_dir.join("blocks").join("furniture");
    let mut set = std::collections::HashSet::new();
    if let Ok(rd) = std::fs::read_dir(&dir) {
        for entry in rd.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("ifc") {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    set.insert(name.to_string());
                }
            }
        }
    }
    set
}

fn dims_summary(dims: &Value) -> String {
    let w = int_of(dims, "w");
    let d = int_of(dims, "d");
    let hmin = int_of(dims, "h_min");
    let hmax = int_of(dims, "h_max");
    match (w, d) {
        (Some(w), Some(d)) => {
            let h = match (hmin, hmax) {
                (Some(a), Some(b)) if a == b => format!(" × {a}"),
                (Some(a), Some(b)) => format!(" × {a}–{b}"),
                _ => String::new(),
            };
            format!("{w} × {d}{h} mm")
        }
        _ => "—".to_string(),
    }
}

/// Round 6 (2026-07-10) P2: true for a composition that should be visible
/// on the public catalog. Suppresses "room-programme-only" entries —
/// `has_zone_data` true but zero bill items, meaning no `furniture_program`
/// was ever authored, only bare room names — a genuinely thin record, not
/// ready for public display. Corporate Office (`has_zone_data` false, a
/// different correct-by-design floor-scale record type) and any
/// composition with at least one bill item (even partially linked, e.g.
/// PO-2/PO-3) both pass. This is a display-layer gate only — the
/// underlying token data and `/api/tokens.json` are untouched.
fn composition_is_publicly_visible(c: &Value) -> bool {
    let has_zone = c
        .get("has_zone_data")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let bill_len = c
        .get("bill")
        .and_then(Value::as_array)
        .map(|a| a.len())
        .unwrap_or(0);
    !(has_zone && bill_len == 0)
}

/// Round 5 (2026-07-10): groups a flat spec-row array under real IFC
/// property-set headers where a genuine Pset name applies (currently just
/// `Pset_ManufacturerTypeInformation`, which real IFC deployments use for
/// manufacturer/model/SKU-class facts) and a plain, honestly-labelled
/// group for everything else — never inventing a Pset name for a grouping
/// IFC itself doesn't define one for. Addresses the "spec tables are flat"
/// gap flagged in the Round 5 hyperscaler-provider audit: even one grouped
/// header referencing real IFC property-set naming is enough to read as
/// IFC-literate to a specifier already looking for that pattern.
fn spec_rows_grouped(rows: &[Value]) -> String {
    const MFR_KEYS: &[&str] = &["Manufacturer", "Product line", "Model", "SKU"];
    const CLASS_KEYS: &[&str] = &[
        "IFC 4.3 entity class",
        "Uniclass 2015 (Pr)",
        "Uniclass 2015 — Pr",
    ];
    let mut out = String::new();
    let mut last_group: Option<&'static str> = None;
    for r in rows {
        let arr = r.as_array().cloned().unwrap_or_default();
        let k = arr
            .first()
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string();
        let v = arr.get(1).and_then(Value::as_str).unwrap_or("").to_string();
        let group = if MFR_KEYS.contains(&k.as_str()) {
            "Pset_ManufacturerTypeInformation"
        } else if CLASS_KEYS.iter().any(|c| k.starts_with(c)) {
            "Classification"
        } else {
            "Dimensional & physical properties"
        };
        if last_group != Some(group) {
            out.push_str(&format!(
                r#"<tr class="bim-cat-spectable__grouprow"><th colspan="2">{}</th></tr>"#,
                esc(group)
            ));
            last_group = Some(group);
        }
        out.push_str(&format!(
            "<tr><th>{}</th><td>{}</td></tr>",
            esc(&k),
            esc(&v)
        ));
    }
    out
}

/// URL-safe slug for a Composition's `internal_code` (e.g. "CO-1/2" →
/// "co-1-2") — internal codes carry a literal `/` for some categories
/// (Corporate Office fractional floors), which would otherwise split into
/// extra path segments. Distinct from the human-readable `id`/display code.
fn slugify_code(code: &str) -> String {
    code.to_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

pub(crate) fn build_objects(state: &AppState) -> Vec<Value> {
    let ifc_files = ifc_file_set(state);
    let mut out: Vec<Value> = Vec::new();

    let Some(furniture) = state
        .tokens
        .get("interior")
        .and_then(|f| f.get("bim"))
        .and_then(|b| b.get("interior"))
        .and_then(|i| i.get("furniture"))
        .and_then(Value::as_object)
    else {
        return out;
    };

    for (group, slugs) in furniture {
        let Some(slugs) = slugs.as_object() else {
            continue;
        };
        for (slug, entity) in slugs {
            let Some(val) = entity.get("$value") else {
                continue;
            };
            let name = {
                let m = s(val, "model");
                if m.is_empty() {
                    slug.replace('-', " ")
                } else {
                    m.to_string()
                }
            };
            let ifc_class = s(val, "ifc_class");
            let uni_pr = s(val, "uniclass_pr");
            let uni_pr_title = s(val, "uniclass_pr_title");
            let manufacturer = s(val, "manufacturer");
            let dims = val.get("dimensions_mm").cloned().unwrap_or(Value::Null);
            let dim_summary = if dims.is_null() {
                "—".to_string()
            } else {
                dims_summary(&dims)
            };
            let description = entity
                .get("$description")
                .and_then(Value::as_str)
                .unwrap_or("");

            let expected_ifc = format!("{group}-{slug}.ifc");
            let ifc_file = if ifc_files.contains(&expected_ifc) {
                Value::String(expected_ifc)
            } else {
                Value::Null
            };

            // Full spec rows for the detail modal.
            let mut spec: Vec<Value> = Vec::new();
            let mut row = |k: &str, v: String| {
                if !v.is_empty() {
                    spec.push(json!([k, v]));
                }
            };
            row("Manufacturer", s(val, "manufacturer").to_string());
            row("Product line", s(val, "product_line").to_string());
            row("Model", s(val, "model").to_string());
            row("SKU", s(val, "sku").to_string());
            row("Designer", s(val, "designer").to_string());
            if !dims.is_null() {
                row("Dimensions (W × D × H)", dims_summary(&dims));
            }
            if let Some(dia) = int_of(val, "diameter_mm") {
                row("Diameter", format!("⌀ {dia} mm"));
            }
            if let Some(sh) = val.get("seat_height_mm") {
                let (a, b) = (int_of(sh, "min"), int_of(sh, "max"));
                if let (Some(a), Some(b)) = (a, b) {
                    let v = if a == b {
                        format!("{a} mm")
                    } else {
                        format!("{a}–{b} mm")
                    };
                    row("Seat height", v);
                }
            }
            if let Some(cl) = val.get("clearance_mm") {
                let f = int_of(cl, "front").unwrap_or(0);
                let si = int_of(cl, "sides").unwrap_or(0);
                let r = int_of(cl, "rear").unwrap_or(0);
                row(
                    "Clearance (front / sides / rear)",
                    format!("{f} / {si} / {r} mm"),
                );
            }
            let weight = match f_of(val, "weight_kg") {
                Some(w) => format!("{} kg", round2(w)),
                None => s(val, "weight_note").to_string(),
            };
            row("Weight", weight);
            row("IFC 4.3 entity class", ifc_class.to_string());
            if !uni_pr.is_empty() {
                row("Uniclass 2015 (Pr)", format!("{uni_pr} — {uni_pr_title}"));
            }

            let search = format!(
                "{} {} {} {} {} {}",
                name, manufacturer, ifc_class, uni_pr, uni_pr_title, group
            )
            .to_lowercase();

            let mut e = Map::new();
            e.insert("id".into(), json!(slug));
            e.insert("kind".into(), json!("object"));
            e.insert("group".into(), json!(group));
            e.insert(
                "ref".into(),
                json!(format!("bim.interior.furniture.{group}.{slug}")),
            );
            e.insert("name".into(), json!(name));
            e.insert("manufacturer".into(), json!(manufacturer));
            e.insert("ifc_class".into(), json!(ifc_class));
            e.insert("uniclass_pr".into(), json!(uni_pr));
            e.insert("uniclass_pr_title".into(), json!(uni_pr_title));
            e.insert("dims".into(), json!(dim_summary));
            e.insert("dims_w_mm".into(), json!(int_of(&dims, "w")));
            e.insert("ifc_file".into(), ifc_file);
            e.insert("url".into(), val.get("url").cloned().unwrap_or(Value::Null));
            e.insert("description".into(), json!(description));
            e.insert("spec".into(), Value::Array(spec));
            e.insert("search".into(), json!(search));
            out.push(Value::Object(e));
        }
    }

    out.sort_by(|a, b| {
        s(a, "group")
            .cmp(s(b, "group"))
            .then_with(|| s(a, "name").cmp(s(b, "name")))
    });
    out
}

// ─────────────────────────────────────────────────────────────────────────────
// Compositions — Key Plans from key-plans.dtcg.json
// ─────────────────────────────────────────────────────────────────────────────

pub(crate) fn build_compositions(state: &AppState, objects: &[Value]) -> Vec<Value> {
    let mut out: Vec<Value> = Vec::new();

    let Some(bim) = state
        .tokens
        .get("key-plans")
        .and_then(|f| f.get("bim"))
        .and_then(Value::as_object)
    else {
        return out;
    };

    // Reuse the shared leaf-walker: any node carrying a `$value`, at any depth.
    let mut leaves: Vec<(&str, &Value)> = Vec::new();
    collect_kp_leaves(bim, &mut leaves);

    for (_slug, entity) in leaves {
        let Some(val) = entity.get("$value") else {
            continue;
        };
        let internal_code = {
            let c = s(val, "internal_code");
            if c.is_empty() {
                s(val, "display_name").to_string()
            } else {
                c.to_string()
            }
        };
        let display_name = s(val, "display_name");
        let category = s(val, "category");
        let cat_label = category_label(category);
        let space = category_space(category);
        let area_m2 = f_of(val, "area_m2");
        let area_sf = int_of(val, "area_sf");
        let z1 = f_of(val, "zone1_depth_m");
        let z2 = f_of(val, "zone2_depth_m");
        let z3 = f_of(val, "zone3_depth_m");
        let has_zone_data = z1.is_some();
        let svg = render_kp_zone_svg_from_value(val);
        let description = entity
            .get("$description")
            .and_then(Value::as_str)
            .unwrap_or("");

        let furniture_program = str_vec(val, "furniture_program");
        let development_classes = str_vec(val, "development_classes");
        let key_rooms = str_vec(val, "key_rooms");

        // Bill of objects. Only PO-1 carries a real `furniture_refs` array;
        // resolve each against the Objects list. Everything else falls back to
        // the prose program with an explicit "linking pending" flag.
        let refs = val.get("furniture_refs").and_then(Value::as_array);
        let (bill, refs_pending) = if let Some(refs) = refs {
            let mut items: Vec<Value> = Vec::new();
            for r in refs {
                let Some(rstr) = r.as_str() else { continue };
                let matched = objects.iter().find(|o| s(o, "ref") == rstr);
                if let Some(o) = matched {
                    items.push(json!({
                        "linked": true,
                        "name": s(o, "name"),
                        "code": s(o, "uniclass_pr"),
                        "obj_id": s(o, "id"),
                    }));
                } else {
                    items.push(json!({ "linked": false, "name": rstr }));
                }
            }
            (Value::Array(items), false)
        } else {
            let items: Vec<Value> = furniture_program
                .iter()
                .map(|line| json!({ "linked": false, "name": line }))
                .collect();
            (Value::Array(items), true)
        };

        // Spec rows for the modal.
        let mut spec: Vec<Value> = Vec::new();
        let mut row = |k: &str, v: String| {
            if !v.is_empty() {
                spec.push(json!([k, v]));
            }
        };
        row("Internal code", internal_code.clone());
        row("Category", cat_label.to_string());
        match (area_m2, area_sf) {
            (Some(m), Some(sf)) => row("Net leasable area", format!("{} m² · {sf} SF", round2(m))),
            (Some(m), None) => row("Net leasable area", format!("{} m²", round2(m))),
            (None, Some(sf)) => row("Net leasable area", format!("{sf} SF")),
            _ => {}
        }
        if let Some(z) = z1 {
            row("Zone 1 (Habitat) depth", format!("{} m", round2(z)));
        }
        if let Some(z) = z2 {
            row("Zone 2 (Magazine) depth", format!("{} m", round2(z)));
        }
        if let Some(z) = z3 {
            row("Zone 3 (Corridor) depth", format!("{} m", round2(z)));
        }
        if let Some(fr) = f_of(val, "facade_frontage_m") {
            row("Facade frontage", format!("{} m", round2(fr)));
        }
        if let Some(o) = int_of(val, "occupancy_persons") {
            row("Occupancy", format!("{o} persons"));
        } else if let (Some(a), Some(b)) = (
            int_of(val, "occupancy_persons_min"),
            int_of(val, "occupancy_persons_max"),
        ) {
            row("Occupancy", format!("{a}–{b} persons"));
        }
        if let Some(bc) = int_of(val, "bench_count") {
            row("Benches", format!("{bc}"));
        }
        if let Some(ec) = int_of(val, "exam_chairs") {
            row("Exam / treatment chairs", format!("{ec}"));
        }
        row("Tile role", s(val, "tile_role").to_string());
        if !development_classes.is_empty() {
            row("Development classes", development_classes.join(", "));
        }
        row("Uniclass 2015 (SL)", format!("SL — {space}"));

        let search =
            format!("{display_name} {internal_code} {category} {cat_label}").to_lowercase();

        let slug = slugify_code(&internal_code);
        let mut e = Map::new();
        e.insert("id".into(), json!(internal_code));
        e.insert("slug".into(), json!(slug));
        e.insert("kind".into(), json!("composition"));
        e.insert("name".into(), json!(display_name));
        e.insert("category".into(), json!(category));
        e.insert("category_label".into(), json!(cat_label));
        e.insert("area_m2".into(), json!(area_m2.map(round2)));
        e.insert("area_sf".into(), json!(area_sf));
        e.insert("has_zone_data".into(), json!(has_zone_data));
        // Stored as raw numbers, not round2()-formatted strings — zone_bars_html
        // reads these back with Value::as_f64() to compute the bar-width
        // percentages, and rounds only for display (round2(*v)). A prior
        // version stored these as strings (json!(z1.map(round2))), which
        // silently defeated as_f64() (a JSON String is never numeric to
        // serde_json) and rendered an empty <div class="bim-cat-zones"></div>
        // on every composition with real zone data, incl. /compositions/po-1
        // (2026-07-09 audit finding).
        e.insert("zone1".into(), json!(z1));
        e.insert("zone2".into(), json!(z2));
        e.insert("zone3".into(), json!(z3));
        e.insert("uniclass_level".into(), json!("SL"));
        e.insert("uniclass_space".into(), json!(space));
        e.insert("refs_pending".into(), json!(refs_pending));
        e.insert("bill".into(), bill);
        e.insert("furniture_program".into(), json!(furniture_program));
        e.insert("development_classes".into(), json!(development_classes));
        e.insert("key_rooms".into(), json!(key_rooms));
        e.insert(
            "tile_role".into(),
            val.get("tile_role").cloned().unwrap_or(Value::Null),
        );
        e.insert(
            "design_notes".into(),
            val.get("design_notes").cloned().unwrap_or(Value::Null),
        );
        e.insert(
            "compliance".into(),
            val.get("compliance").cloned().unwrap_or(Value::Null),
        );
        e.insert("description".into(), json!(description));
        e.insert("svg".into(), json!(svg));
        e.insert("spec".into(), Value::Array(spec));
        e.insert("search".into(), json!(search));
        out.push(Value::Object(e));
    }

    out.sort_by(|a, b| {
        category_rank(s(a, "category"))
            .cmp(&category_rank(s(b, "category")))
            .then_with(|| {
                int_of(a, "area_sf")
                    .unwrap_or(0)
                    .cmp(&int_of(b, "area_sf").unwrap_or(0))
            })
    });
    out
}

// ─────────────────────────────────────────────────────────────────────────────
// Public: normalized catalog for the `/api/tokens.json` extension
// ─────────────────────────────────────────────────────────────────────────────

/// Normalized `{ objects: [...], compositions: [...] }` catalog. Consumed by
/// `bim-catalog.js` (via the `_catalog` key added to `/api/tokens.json`) to
/// populate the detail modal without a full page reload.
pub fn build_catalog(state: &AppState) -> Value {
    let objects = build_objects(state);
    let compositions = build_compositions(state, &objects);
    json!({
        "objects": objects,
        "compositions": compositions,
    })
}

// ─────────────────────────────────────────────────────────────────────────────
// Shared helpers — query strings, chips, plan-symbol cards
// ─────────────────────────────────────────────────────────────────────────────

/// Minimal percent-encoder for query-string values (manufacturer names,
/// Uniclass titles, etc. can contain spaces/parens/slashes).
pub(crate) fn percent_encode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => {
                out.push(b as char)
            }
            _ => out.push_str(&format!("%{:02X}", b)),
        }
    }
    out
}

/// Build a `?q=...&k=v...` query string from an optional search term plus an
/// ordered list of `(key, Some(value))` pairs (a `None` value omits the
/// key). Every filtered catalog page is a real, shareable, reloadable URL —
/// no client-only filter state (2026-07 audit's "silent state wipe on
/// reload" / "unshareable filtered views" findings).
fn build_query(q: &str, pairs: &[(&str, Option<&str>)]) -> String {
    let mut parts: Vec<String> = Vec::new();
    if !q.trim().is_empty() {
        parts.push(format!("q={}", percent_encode(q.trim())));
    }
    for (k, v) in pairs {
        if let Some(v) = v {
            parts.push(format!("{k}={}", percent_encode(v)));
        }
    }
    if parts.is_empty() {
        String::new()
    } else {
        format!("?{}", parts.join("&"))
    }
}

/// One filter-chip row: a plain `<a href>` per value (real URL toggle, no JS
/// required) — single-select per dimension, click the active chip again to
/// clear it. Replaces the unstyled native checkbox sidebar the 2026-07 audit
/// flagged.
/// `items` is `(query_value, display_label, count)` — kept distinct because
/// Layout's query values ("modeled"/"floor") differ from their display text
/// ("Zone layout modeled"/"Floor-scale"); Uniclass/Manufacturer/Use-Case
/// pass `query_value == display_label`.
fn chip_row(
    title: &str,
    path: &str,
    param: &str,
    items: &[(String, String, usize)],
    active: Option<&str>,
    q: &str,
    other: &[(&str, Option<&str>)],
) -> String {
    if items.is_empty() {
        return String::new();
    }
    let chips: String = items
        .iter()
        .map(|(val, label, count)| {
            let is_active = active == Some(val.as_str());
            let mut pairs: Vec<(&str, Option<&str>)> = other.to_vec();
            let newval = if is_active { None } else { Some(val.as_str()) };
            pairs.push((param, newval));
            let href = format!("{path}{}", build_query(q, &pairs));
            format!(
                r#"<a class="bim-chip-toggle{active_cls}" href="{href}">{label} <span class="bim-chip-toggle__n">{count}</span></a>"#,
                active_cls = if is_active { " is-active" } else { "" },
                href = esc(&href),
                label = esc(label),
                count = count,
            )
        })
        .collect();
    format!(
        r#"<div class="bim-chip-row"><span class="bim-chip-row__label">{title}</span>{chips}</div>"#,
        title = esc(title),
        chips = chips,
    )
}

/// `counted()` values used as both the query value and the display label
/// (Uniclass/Manufacturer/Use-Case chips).
fn counted_pairs(items: Vec<(String, usize)>) -> Vec<(String, String, usize)> {
    items.into_iter().map(|(v, c)| (v.clone(), v, c)).collect()
}

/// Ordered `(value, count)` list preserving first-seen or supplied order.
fn counted(values: impl Iterator<Item = String>) -> Vec<(String, usize)> {
    let mut order: Vec<String> = Vec::new();
    let mut counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for v in values {
        if v.is_empty() {
            continue;
        }
        if !counts.contains_key(&v) {
            order.push(v.clone());
        }
        *counts.entry(v).or_insert(0) += 1;
    }
    order.sort();
    order
        .into_iter()
        .map(|v| {
            let c = counts[&v];
            (v, c)
        })
        .collect()
}

fn search_tokens(q: &str) -> Vec<String> {
    q.trim()
        .split_whitespace()
        .map(|t| t.to_lowercase())
        .collect()
}

fn matches_search(haystack: &str, tokens: &[String]) -> bool {
    tokens.iter().all(|t| haystack.contains(t.as_str()))
}

// ── object card (plan-symbol thumbnail; real /objects/{slug} link) ────────

/// The card itself is a `<div>`, not an `<a>`, so it can host both a real
/// clickable checkbox (item 8's compare feature, 2026-07-09) and a
/// full-card link without nesting one interactive element inside another.
/// `.bim-cat-card__link` is stretched over the card via CSS (the standard
/// "stretched-link" technique) so the whole card still reads and clicks as
/// one link everywhere except the checkbox's own top-left corner, which
/// sits above it in z-order.
pub(crate) fn render_object_card(o: &Value) -> String {
    let id = s(o, "id");
    let name = s(o, "name");
    let group = s(o, "group");
    let manufacturer = s(o, "manufacturer");
    let ifc_class = s(o, "ifc_class");
    let uni_pr = s(o, "uniclass_pr");
    let dims = s(o, "dims");
    let uni_title = s(o, "uniclass_pr_title");
    let width_mm = o.get("dims_w_mm").and_then(Value::as_i64);
    let dim_label = super::plan_symbols::dim_annotation(width_mm);
    let symbol = super::plan_symbols::plan_symbol_svg(group, id, &dim_label);
    let ifc_badge = if o.get("ifc_file").map(Value::is_string).unwrap_or(false) {
        r#"<span class="bim-cat-thumb__fmt">IFC</span>"#
    } else {
        ""
    };

    format!(
        r#"<div class="bim-cat-card bim-cat-card--obj">
  <label class="bim-cat-card__comparetoggle" title="Add to compare">
    <input type="checkbox" class="bim-compare-check" name="ids" value="{id}" aria-label="Add {name} to compare">
    <span class="bim-cat-card__checkbox" aria-hidden="true"></span>
  </label>
  <a class="bim-cat-card__link" href="/objects/{id}" aria-label="{name} — view specification" title="{uni_title}">
    <span class="bim-cat-thumb bim-cat-thumb--obj">{symbol}{ifc_badge}</span>
    <span class="bim-cat-card__body">
      <span class="bim-cat-chip bim-cat-chip--pr"><span class="bim-cat-chip__lv">Pr</span>{uni_pr}</span>
      <span class="bim-cat-card__name">{name}</span>
      <span class="bim-cat-card__meta"><span class="bim-cat-card__em">{mfr}</span> · {ifc_class}</span>
      <span class="bim-cat-card__prov">{mfr} · {dims}</span>
    </span>
  </a>
</div>"#,
        id = esc(id),
        name = esc(name),
        mfr = esc(manufacturer),
        uni_pr = esc(uni_pr),
        uni_title = esc(uni_title),
        ifc_class = esc(ifc_class),
        dims = esc(dims),
        symbol = symbol,
        ifc_badge = ifc_badge,
    )
}

// ── composition card (navy key-plan thumbnail; honest empty-bill note) ────

pub(crate) fn render_composition_card(c: &Value) -> String {
    let slug = s(c, "slug");
    let id = s(c, "id");
    let name = s(c, "name");
    let category = s(c, "category");
    let cat_label = s(c, "category_label");
    let space = s(c, "uniclass_space");
    let has_zone = c
        .get("has_zone_data")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let area_sf = int_of(c, "area_sf");
    let area_line = match area_sf {
        Some(sf) => format!("{sf} SF"),
        None => "—".to_string(),
    };
    let bill_len = c
        .get("bill")
        .and_then(Value::as_array)
        .map(|a| a.len())
        .unwrap_or(0);
    let linked_len = c
        .get("bill")
        .and_then(Value::as_array)
        .map(|a| {
            a.iter()
                .filter(|b| b.get("linked").and_then(Value::as_bool).unwrap_or(false))
                .count()
        })
        .unwrap_or(0);

    // Round 5 (2026-07-10): the three honest-pending states below were
    // previously one generic wording ("Object linking: 0 of 0 — pending"
    // covered both a genuinely floor-scale entry and a room-program-only
    // entry alike), which reads as "we haven't gotten to this yet" even
    // where the state is correct by design. Each now says specifically why,
    // grounded in a direct check of the underlying token data (not
    // inferred): Corporate Office has no furniture-level data anywhere,
    // by design — the tenant designs their own interior against a
    // floor-scale leasehold fraction. The 14 Medical/Business/Laboratory/
    // Academic/Civic entries have only a bare room programme (`key_rooms`)
    // authored, with no furniture layout on record yet. See
    // BRIEF-bim-v3-hyperscaler-redesign.md for the token-file-level
    // verification this distinction is based on.
    let has_room_program = c
        .get("key_rooms")
        .and_then(Value::as_array)
        .map(|a| !a.is_empty())
        .unwrap_or(false);
    let (thumb, note) = if has_zone {
        let svg = s(c, "svg");
        let note = if bill_len == 0 && has_room_program {
            r#"<span class="bim-cat-card__note">Room programme</span>"#.to_string()
        } else if bill_len == 0 {
            String::new()
        } else if linked_len < bill_len {
            format!(
                r#"<span class="bim-cat-card__note">{linked_len} of {bill_len} parts linked to the catalog</span>"#
            )
        } else {
            String::new()
        };
        (
            format!(r#"<span class="bim-cat-thumb bim-cat-thumb--comp">{svg}</span>"#),
            note,
        )
    } else {
        (
            format!(
                r#"<span class="bim-cat-thumb bim-cat-thumb--comp bim-cat-thumb--floorscale">{}</span>"#,
                super::svg::render_floor_scale_svg()
            ),
            r#"<span class="bim-cat-card__note">Leasehold sized as a fraction of the Floor Plate — tenant designs interior layout</span>"#.to_string(),
        )
    };

    format!(
        r#"<a class="bim-cat-card bim-cat-card--comp" href="/compositions/{slug}" data-cat="{category}" aria-label="{name} — view specification">
  {thumb}
  <span class="bim-cat-card__body">
    <span class="bim-cat-chip bim-cat-chip--ef"><span class="bim-cat-chip__lv">SL</span>{space}</span>
    <span class="bim-cat-card__name">{name}</span>
    <span class="bim-cat-card__meta"><span class="bim-cat-card__em">{cat_label}</span> · {area_line} · {id}</span>
    {note}
  </span>
</a>"#,
        slug = esc(slug),
        id = esc(id),
        name = esc(name),
        category = esc(category),
        cat_label = esc(cat_label),
        space = esc(space),
        area_line = esc(&area_line),
        thumb = thumb,
        note = note,
    )
}

// ─────────────────────────────────────────────────────────────────────────────
// SSR — home (`/`), a compact registry front door
// ─────────────────────────────────────────────────────────────────────────────

pub fn render_home(state: &AppState) -> String {
    let objects = build_objects(state);
    // Same visibility gate as render_compositions_index (Round 6 P2) — the
    // homepage's stated count must agree with what /compositions actually
    // shows, or the two numbers contradict each other on the same site.
    let compositions: Vec<Value> = build_compositions(state, &objects)
        .into_iter()
        .filter(composition_is_publicly_visible)
        .collect();
    let obj_n = objects.len();
    let comp_n = compositions.len();

    let lede = state
        .home_page
        .sections
        .iter()
        .find(|sec| sec.heading == "The Library")
        .map(|sec| sec.body_html.as_str())
        .unwrap_or("<p>The catalog of specification-ready building parts.</p>");

    let other_sections: String = state
        .home_page
        .sections
        .iter()
        .filter(|sec| sec.heading != "The Library")
        .map(|sec| {
            format!(
                r#"<section class="bim-home-block"><h2>{}</h2>{}</section>"#,
                esc(&sec.heading),
                sec.body_html,
            )
        })
        .collect();

    // Round 6 (2026-07-10): the hero was text-only — at 1440-1920px the
    // right half was empty grid paper (the cohesion audit's gap #2). Fill
    // it with a real Key Plan, not an invented illustrative one: PO-1's
    // actual zone data if the composition is present in this catalog,
    // falling back to representative values only if it is ever absent.
    // The same draw-on animation used on composition detail pages applies
    // here (bim.js watches `.bim-home-masthead__visual .bim-kp-diagram`
    // too now) — the site's single largest visual asset gets to draft
    // itself in on the page a visitor actually lands on first.
    let hero_svg = compositions
        .iter()
        .find(|c| s(c, "id") == "PO-1")
        .map(|c| {
            let z1 = c.get("zone1").and_then(Value::as_f64).unwrap_or(6.0);
            let z2 = c.get("zone2").and_then(Value::as_f64).unwrap_or(3.5);
            let z3 = c.get("zone3").and_then(Value::as_f64);
            let area = c.get("area_m2").and_then(Value::as_f64);
            super::svg::render_kp_zone_svg(z1, z2, z3, "private-office", area)
        })
        .unwrap_or_else(|| {
            super::svg::render_kp_zone_svg(6.0, 3.5, Some(2.0), "private-office", None)
        });

    format!(
        r##"<div class="bim-home">
  <section class="bim-home-masthead">
    <div class="bim-home-masthead__text">
      <h1>Woodfine BIM Library</h1>
      <div class="bim-home-masthead__lede">{lede}</div>
      <p class="bim-home-registry-line">{obj_n} objects &middot; {comp_n} compositions &middot; IFC&nbsp;4.3 &middot; Uniclass&nbsp;2015</p>
    </div>
    <div class="bim-home-masthead__visual" aria-hidden="true">{hero_svg}</div>
  </section>

  <section class="bim-home-shelves" aria-label="The two shelves">
    <a class="bim-home-shelf" href="/objects">
      <span class="bim-home-shelf__kicker">The parts</span>
      <h2>Objects <span class="bim-cat-count">{obj_n}</span></h2>
      <p>A smart specification for one part of a building — geometry, data, and the rules it must meet, in one open file.</p>
      <span class="bim-home-shelf__cta">Browse Objects →</span>
    </a>
    <a class="bim-home-shelf" href="/compositions">
      <span class="bim-home-shelf__kicker">The assemblies</span>
      <h2>Compositions <span class="bim-cat-count">{comp_n}</span></h2>
      <p>An assembly — parts combined into a room, a floor, a building, with the rules checked at every join.</p>
      <span class="bim-home-shelf__cta">Browse Compositions →</span>
    </a>
  </section>

  {other_sections}
</div>"##,
        obj_n = obj_n,
        comp_n = comp_n,
        lede = lede,
        other_sections = other_sections,
    )
}

// ─────────────────────────────────────────────────────────────────────────────
// SSR — `/objects` catalog page
// ─────────────────────────────────────────────────────────────────────────────

pub fn render_objects_index(
    state: &AppState,
    q: &str,
    uni: Option<&str>,
    mfr: Option<&str>,
) -> String {
    let objects = build_objects(state);
    let tokens = search_tokens(q);

    let uni_items = counted(
        objects
            .iter()
            .map(|o| s(o, "uniclass_pr_title").to_string()),
    );
    let mfr_items = counted(objects.iter().map(|o| s(o, "manufacturer").to_string()));

    let matches: Vec<&Value> = objects
        .iter()
        .filter(|o| matches_search(s(o, "search"), &tokens))
        .filter(|o| uni.map(|v| s(o, "uniclass_pr_title") == v).unwrap_or(true))
        .filter(|o| mfr.map(|v| s(o, "manufacturer") == v).unwrap_or(true))
        .collect();

    let cards: String = if matches.is_empty() {
        r#"<p class="bim-empty">No objects match the current filters.</p>"#.to_string()
    } else {
        matches.iter().map(|o| render_object_card(o)).collect()
    };

    let uni_pairs = counted_pairs(uni_items);
    let mfr_pairs = counted_pairs(mfr_items);
    let uni_chips = chip_row(
        "Uniclass Pr — product type",
        "/objects",
        "uni",
        &uni_pairs,
        uni,
        q,
        &[("mfr", mfr)],
    );
    let mfr_chips = chip_row(
        "Manufacturer",
        "/objects",
        "mfr",
        &mfr_pairs,
        mfr,
        q,
        &[("uni", uni)],
    );

    format!(
        r##"<div class="bim-catalog-page">
  <header class="bim-cat-pagehead">
    <span class="bim-cat-kicker">The parts</span>
    <h1>Objects</h1>
    <p class="bim-cat-pagehead__lede">A smart specification for one part of a building — geometry, data, and the rules it must meet, in one open file that travels with the part through every tool that touches it.</p>
  </header>
  <form class="bim-cat-searchform" method="get" action="/objects">
    <label class="bim-cat-search">
      <span class="bim-cat-search__ico" aria-hidden="true">⌕</span>
      <input type="search" name="q" value="{q}" placeholder="Search the registry" aria-label="Search Objects">
    </label>
    {uni_hidden}{mfr_hidden}
    <button class="bim-cat-searchform__submit" type="submit">Search</button>
  </form>
  <div class="bim-cat-filters">{uni_chips}{mfr_chips}</div>
  <div class="bim-cat-gridhead">
    <div class="bim-cat-res"><b>{n}</b> of {total} objects</div>
  </div>
  <form method="get" action="/objects/compare" id="bim-compare-form">
    <div class="bim-cat-grid bim-cat-grid--obj">{cards}</div>
    <div class="bim-compare-bar" id="bim-compare-bar">
      <span class="bim-compare-bar__label" id="bim-compare-label">Check 2 or more Objects to compare their dimensions.</span>
      <button type="submit" class="bim-compare-bar__go" id="bim-compare-go">Compare selected</button>
      <button type="button" class="bim-compare-bar__clear" id="bim-compare-clear">Clear</button>
    </div>
  </form>
</div>"##,
        q = esc(q),
        uni_hidden = uni
            .map(|v| format!(r#"<input type="hidden" name="uni" value="{}">"#, esc(v)))
            .unwrap_or_default(),
        mfr_hidden = mfr
            .map(|v| format!(r#"<input type="hidden" name="mfr" value="{}">"#, esc(v)))
            .unwrap_or_default(),
        uni_chips = uni_chips,
        mfr_chips = mfr_chips,
        n = matches.len(),
        total = objects.len(),
        cards = cards,
    )
}

// ─────────────────────────────────────────────────────────────────────────────
// SSR — `/compositions` catalog page (section headers by use-case)
// ─────────────────────────────────────────────────────────────────────────────

pub fn render_compositions_index(
    state: &AppState,
    q: &str,
    use_case: Option<&str>,
    layout: Option<&str>,
) -> String {
    let objects = build_objects(state);
    let compositions: Vec<Value> = build_compositions(state, &objects)
        .into_iter()
        .filter(composition_is_publicly_visible)
        .collect();
    let tokens = search_tokens(q);

    // Use-Case chips: (query_value = category slug, display_label, count).
    let use_pairs: Vec<(String, String, usize)> = {
        let mut order: Vec<&str> = Vec::new();
        let mut counts: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
        for c in &compositions {
            let slug = s(c, "category");
            if slug.is_empty() {
                continue;
            }
            if !counts.contains_key(slug) {
                order.push(slug);
            }
            *counts.entry(slug).or_insert(0) += 1;
        }
        order.sort_by_key(|slug| category_rank(slug));
        order
            .into_iter()
            .map(|slug| {
                (
                    slug.to_string(),
                    category_label(slug).to_string(),
                    counts[slug],
                )
            })
            .collect()
    };

    // Layout chips: (query_value = "modeled"/"floor", display_label, count).
    let modeled_n = compositions
        .iter()
        .filter(|c| {
            c.get("has_zone_data")
                .and_then(Value::as_bool)
                .unwrap_or(false)
        })
        .count();
    let floor_n = compositions.len() - modeled_n;
    let mut layout_pairs: Vec<(String, String, usize)> = Vec::new();
    if modeled_n > 0 {
        layout_pairs.push((
            "modeled".to_string(),
            "Zone layout modeled".to_string(),
            modeled_n,
        ));
    }
    if floor_n > 0 {
        layout_pairs.push(("floor".to_string(), "Floor-scale".to_string(), floor_n));
    }

    let matches: Vec<&Value> = compositions
        .iter()
        .filter(|c| matches_search(s(c, "search"), &tokens))
        .filter(|c| use_case.map(|v| s(c, "category") == v).unwrap_or(true))
        .filter(|c| {
            layout
                .map(|v| {
                    let has_zone = c
                        .get("has_zone_data")
                        .and_then(Value::as_bool)
                        .unwrap_or(false);
                    (v == "modeled") == has_zone
                })
                .unwrap_or(true)
        })
        .collect();

    let sections: String = CATEGORY_ORDER
        .iter()
        .filter(|cat| use_case.map(|v| v == **cat).unwrap_or(true))
        .map(|cat| {
            let in_section: Vec<&&Value> = matches
                .iter()
                .filter(|c| s(c, "category") == *cat)
                .collect();
            if in_section.is_empty() {
                return String::new();
            }
            let cards: String = in_section
                .iter()
                .map(|c| render_composition_card(c))
                .collect();
            format!(
                r#"<section class="bim-cat-usesection">
  <h2 class="bim-cat-usesection__h">{label}</h2>
  <div class="bim-cat-grid bim-cat-grid--comp">{cards}</div>
</section>"#,
                label = esc(category_label(cat)),
                cards = cards,
            )
        })
        .collect();

    let body = if matches.is_empty() {
        r#"<p class="bim-empty">No compositions match the current filters.</p>"#.to_string()
    } else {
        sections
    };

    let use_chips = chip_row(
        "Use Case",
        "/compositions",
        "use",
        &use_pairs,
        use_case,
        q,
        &[("layout", layout)],
    );
    let layout_chips = chip_row(
        "Layout",
        "/compositions",
        "layout",
        &layout_pairs,
        layout,
        q,
        &[("use", use_case)],
    );

    format!(
        r##"<div class="bim-catalog-page">
  <header class="bim-cat-pagehead">
    <span class="bim-cat-kicker">The assemblies</span>
    <h1>Compositions</h1>
    <p class="bim-cat-pagehead__lede">An assembly — parts combined into a room, a floor, a building, with the rules checked at every join. Start from an assembly and open its parts list.</p>
  </header>
  <form class="bim-cat-searchform" method="get" action="/compositions">
    <label class="bim-cat-search">
      <span class="bim-cat-search__ico" aria-hidden="true">⌕</span>
      <input type="search" name="q" value="{q}" placeholder="Search the registry" aria-label="Search Compositions">
    </label>
    {use_hidden}{layout_hidden}
    <button class="bim-cat-searchform__submit" type="submit">Search</button>
  </form>
  <div class="bim-cat-filters">{use_chips}{layout_chips}</div>
  <div class="bim-cat-gridhead">
    <div class="bim-cat-res"><b>{n}</b> of {total} compositions</div>
  </div>
  {body}
</div>"##,
        q = esc(q),
        use_hidden = use_case
            .map(|v| format!(r#"<input type="hidden" name="use" value="{}">"#, esc(v)))
            .unwrap_or_default(),
        layout_hidden = layout
            .map(|v| format!(r#"<input type="hidden" name="layout" value="{}">"#, esc(v)))
            .unwrap_or_default(),
        use_chips = use_chips,
        layout_chips = layout_chips,
        n = matches.len(),
        total = compositions.len(),
        body = body,
    )
}

// ─────────────────────────────────────────────────────────────────────────────
// SSR — `/objects/compare` — real, dimensions-scoped compare (item 8, 2026-07-09)
// ─────────────────────────────────────────────────────────────────────────────

/// Best-effort "size" cell for the compare table: the standard W × D × H
/// summary when both width and depth are on record, else the first
/// diameter/seat-height spec row found (round tables, stools — objects that
/// were never going to have a W × D box in the first place), else an honest
/// "—". Never fabricates a figure the object's own record doesn't carry.
fn compare_dims_cell(o: &Value) -> String {
    let dims = s(o, "dims");
    if dims != "—" && !dims.is_empty() {
        return dims.to_string();
    }
    let spec = o.get("spec").and_then(Value::as_array);
    if let Some(spec) = spec {
        for row in spec {
            let arr = row.as_array();
            let Some(arr) = arr else { continue };
            let k = arr.first().and_then(Value::as_str).unwrap_or("");
            if k == "Diameter" || k == "Seat height" {
                let v = arr.get(1).and_then(Value::as_str).unwrap_or("");
                return format!("{k}: {v}");
            }
        }
    }
    "—".to_string()
}

/// Scoped to dimension fields only (`dims_w_mm` and friends) — this
/// furniture-only catalog has no fire-rating or thermal-value fields on any
/// object record, so a compare feature covering those (as the homepage copy
/// mentions) isn't buildable honestly yet; this ships the real, dimensioned
/// slice rather than fabricating the rest.
pub fn render_objects_compare(state: &AppState, ids: &[String]) -> String {
    let objects = build_objects(state);
    // Preserve the order ids were checked in; drop unknown/duplicate ids
    // silently rather than erroring — a stale/hand-edited URL just compares
    // whatever of it still resolves.
    let mut seen = std::collections::HashSet::new();
    let selected: Vec<&Value> = ids
        .iter()
        .filter(|id| seen.insert(id.as_str()))
        .filter_map(|id| objects.iter().find(|o| s(o, "id") == id.as_str()))
        .collect();

    if selected.len() < 2 {
        return r##"<div class="bim-catalog-page">
  <header class="bim-cat-pagehead">
    <span class="bim-cat-kicker">The parts</span>
    <h1>Compare Objects</h1>
    <p class="bim-cat-pagehead__lede">Select 2 or more Objects from the catalog to compare their dimensions side by side.</p>
  </header>
  <p class="bim-empty">Nothing to compare yet — check the box on 2 or more <a href="/objects" data-path="/objects">Object cards</a>, then use Compare.</p>
</div>"##
            .to_string();
    }

    let rows: [(&str, fn(&Value) -> String); 5] = [
        ("Name", |o| esc(s(o, "name"))),
        ("Manufacturer", |o| esc(s(o, "manufacturer"))),
        ("Dimensions", |o| esc(&compare_dims_cell(o))),
        ("IFC 4.3 entity class", |o| esc(s(o, "ifc_class"))),
        ("Uniclass 2015 (Pr)", |o| {
            let code = s(o, "uniclass_pr");
            let title = s(o, "uniclass_pr_title");
            if title.is_empty() {
                esc(code)
            } else {
                format!("{} <small>{}</small>", esc(code), esc(title))
            }
        }),
    ];

    let header_cells: String = selected
        .iter()
        .map(|o| {
            format!(
                r#"<th><a href="/objects/{id}">{name}</a></th>"#,
                id = esc(s(o, "id")),
                name = esc(s(o, "name")),
            )
        })
        .collect();

    let body_rows: String = rows
        .iter()
        .map(|(label, f)| {
            let cells: String = selected
                .iter()
                .map(|o| format!("<td>{}</td>", f(o)))
                .collect();
            format!("<tr><th scope=\"row\">{label}</th>{cells}</tr>")
        })
        .collect();

    let clear_href = "/objects";

    format!(
        r##"<div class="bim-catalog-page">
  <header class="bim-cat-pagehead">
    <span class="bim-cat-kicker">The parts</span>
    <h1>Compare Objects</h1>
    <p class="bim-cat-pagehead__lede">{n} Objects, compared by dimensions — the fields every entry in this catalog actually carries. IFC class and Uniclass 2015 code are included for registry cross-reference.</p>
  </header>
  <div class="bim-table-wrap">
    <table class="bim-cat-spectable bim-compare-table">
      <thead><tr><th></th>{header_cells}</tr></thead>
      <tbody>{body_rows}</tbody>
    </table>
  </div>
  <p class="bim-cat-note"><a href="{clear_href}" data-path="{clear_href}">← Back to Objects</a></p>
</div>"##,
        n = selected.len(),
        header_cells = header_cells,
        body_rows = body_rows,
        clear_href = clear_href,
    )
}

// ─────────────────────────────────────────────────────────────────────────────
// SSR — `/objects/{slug}` detail page
// ─────────────────────────────────────────────────────────────────────────────

pub fn render_object_detail(state: &AppState, slug: &str) -> Option<String> {
    let objects = build_objects(state);
    let compositions = build_compositions(state, &objects);
    let o = objects.iter().find(|o| s(o, "id") == slug)?;

    let name = s(o, "name");
    let group = s(o, "group");
    let manufacturer = s(o, "manufacturer");
    let ifc_class = s(o, "ifc_class");
    let uni_pr = s(o, "uniclass_pr");
    let uni_title = s(o, "uniclass_pr_title");
    let reference = s(o, "ref");
    let description = s(o, "description");
    let width_mm = o.get("dims_w_mm").and_then(Value::as_i64);
    let dim_label = super::plan_symbols::dim_annotation(width_mm);
    let symbol = super::plan_symbols::plan_symbol_svg(group, slug, &dim_label);

    let spec_rows: String = o
        .get("spec")
        .and_then(Value::as_array)
        .map(|rows| spec_rows_grouped(rows))
        .unwrap_or_default();

    let dl = match o.get("ifc_file").and_then(Value::as_str) {
        Some(f) => format!(
            r#"<a class="bim-cat-btn" href="/furniture/download/{}">Download IFC (.ifc)</a>"#,
            esc(f)
        ),
        None => {
            r#"<p class="bim-cat-note">No IFC block published for this object yet.</p>"#.to_string()
        }
    };
    let src = match o
        .get("url")
        .and_then(Value::as_str)
        .filter(|u| !u.is_empty())
    {
        Some(u) => format!(
            r#"<p class="bim-cat-note">Manufacturer source: <a href="{u}" target="_blank" rel="noopener">{u}</a></p>"#,
            u = esc(u)
        ),
        None => String::new(),
    };

    // Reverse lookup — every Composition whose linked bill references this
    // Object (2026-07-09 addition: closes the loop the audit's "unlinked
    // bill rows give no way to inspect" finding pointed at, from the object
    // side).
    let used_in: String = compositions
        .iter()
        .filter(|c| {
            c.get("bill")
                .and_then(Value::as_array)
                .map(|b| {
                    b.iter().any(|row| {
                        row.get("linked").and_then(Value::as_bool).unwrap_or(false)
                            && row.get("obj_id").and_then(Value::as_str) == Some(slug)
                    })
                })
                .unwrap_or(false)
        })
        .map(|c| {
            format!(
                r#"<a class="bim-usedin-row" href="/compositions/{slug}/o/{obj}">{name}</a>"#,
                slug = esc(s(c, "slug")),
                obj = esc(slug),
                name = esc(s(c, "name")),
            )
        })
        .collect::<String>();
    let used_in_block = if used_in.is_empty() {
        String::new()
    } else {
        format!(
            r#"<div class="bim-cat-secth">Used in</div><div class="bim-usedin">{used_in}</div>"#
        )
    };

    Some(format!(
        r#"<div class="bim-detail-page bim-detail-page--object">
  <nav class="bim-breadcrumbs"><a href="/">Home</a> / <a href="/objects">Objects</a> / <span>{name}</span></nav>
  <header class="bim-detail-head">
    <div class="bim-detail-head__symbol">{symbol}</div>
    <div class="bim-detail-head__main">
      <div class="bim-chip-row">
        <span class="bim-cat-chip bim-cat-chip--pr"><span class="bim-cat-chip__lv">Pr</span>{uni_pr}</span>
        <span class="bim-cat-chip bim-cat-chip--plain">{ifc_class}</span>
        <span class="bim-cat-chip bim-cat-chip--machine" title="Real, downloadable machine-readable data — see Download below">IFC 4.3 &middot; DTCG</span>
      </div>
      <h1>{name}</h1>
      <p class="bim-detail-head__prov">{mfr} &middot; BIM Object</p>
    </div>
  </header>
  <p class="bim-cat-desc">{description}</p>
  <div class="bim-cat-secth">Specification &amp; property set</div>
  <table class="bim-cat-spectable"><tbody>{spec_rows}</tbody></table>
  <div class="bim-cat-secth">Classification &amp; registry</div>
  <div class="bim-cat-classblock">
    <div class="bim-cat-clrow"><span class="bim-cat-clrow__k">IFC 4.3 entity class</span><span class="bim-cat-clrow__v">{ifc_class}</span></div>
    <div class="bim-cat-clrow"><span class="bim-cat-clrow__k">Uniclass 2015 — Pr (Products)</span><span class="bim-cat-clrow__v">{uni_pr}<small>{uni_title}</small></span></div>
    <div class="bim-cat-clrow"><span class="bim-cat-clrow__k">Registry reference</span><span class="bim-cat-clrow__v">{reference}</span></div>
  </div>
  {used_in_block}
  <div class="bim-cat-secth">Download <span class="bim-cat-chip bim-cat-chip--machine">machine-readable</span></div>
  {dl}
  {src}
</div>"#,
        name = esc(name),
        mfr = esc(manufacturer),
        ifc_class = esc(ifc_class),
        uni_pr = esc(uni_pr),
        uni_title = esc(uni_title),
        reference = esc(reference),
        description = esc(description),
        symbol = symbol,
        spec_rows = spec_rows,
        used_in_block = used_in_block,
        dl = dl,
        src = src,
    ))
}

// ─────────────────────────────────────────────────────────────────────────────
// SSR — `/compositions/{slug}` and `/compositions/{slug}/o/{object}` —
// the plan-anchored inspector.
// ─────────────────────────────────────────────────────────────────────────────

fn zone_bars_html(c: &Value) -> String {
    let has_zone = c
        .get("has_zone_data")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    if !has_zone {
        return r#"<p class="bim-cat-note">Floor-scale plan — sized as a proportion of the floor plate; no three-zone cross-section is modeled at this program level.</p>"#.to_string();
    }
    let rows: Vec<(&str, &str, Option<f64>)> = vec![
        ("Zone 1", "Habitat", c.get("zone1").and_then(Value::as_f64)),
        ("Zone 2", "Magazine", c.get("zone2").and_then(Value::as_f64)),
        ("Zone 3", "Corridor", c.get("zone3").and_then(Value::as_f64)),
    ];
    let present: Vec<(&str, &str, f64)> = rows
        .into_iter()
        .filter_map(|(a, b, v)| v.map(|v| (a, b, v)))
        .collect();
    let max = present.iter().fold(0.0_f64, |m, (_, _, v)| m.max(*v));
    let bars: String = present
        .iter()
        .map(|(k, t, v)| {
            let pct = if max > 0.0 { (v / max * 100.0).round() as i64 } else { 0 };
            format!(
                r#"<div class="bim-cat-zone"><div class="bim-cat-zone__k">{k}</div><div class="bim-cat-zone__t">{t}</div><div class="bim-cat-zone__n">{v}<small> m</small></div><div class="bim-cat-zone__bar"><i style="width:{pct}%"></i></div></div>"#,
                k = esc(k),
                t = esc(t),
                v = round2(*v),
                pct = pct,
            )
        })
        .collect();
    format!(r#"<div class="bim-cat-zones">{bars}</div>"#)
}

fn bill_html(c: &Value, comp_slug: &str) -> String {
    let bill = c
        .get("bill")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let n = bill.len();
    let linked_n = bill
        .iter()
        .filter(|b| b.get("linked").and_then(Value::as_bool).unwrap_or(false))
        .count();

    if n == 0 {
        let program = c
            .get("furniture_program")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        return if program.is_empty() {
            r#"<p class="bim-cat-note">Room programme on record; no furniture layout catalogued.</p>"#.to_string()
        } else {
            r#"<p class="bim-cat-note">Furniture programme on record; not linked to catalog Objects.</p>"#.to_string()
        };
    }

    let status_chip = if linked_n < n {
        format!(
            r#"<div class="bim-bill-status"><span class="bim-bill-status__chip">{linked_n} of {n} parts linked to the catalog</span></div>"#
        )
    } else {
        String::new()
    };

    // `data-plan-obj="{i}"` (2026-07-09, item 7b): pairs this row with the
    // i-th furniture group drawn on the plan SVG (render/svg.rs wraps each
    // discrete desk/table/chair/office/door instance in draw order with the
    // same ordinal, in a `<g class="bim-plan-obj" data-plan-obj="{i}">`).
    // This is a stable ORDINAL pairing, not a semantic per-SKU match — see
    // the longer note in svg.rs. bim.js highlights both ends on hover; if a
    // composition's bill is longer than its plan's group count (or vice
    // versa), the extra rows/groups simply have no match — a harmless no-op,
    // consistent with this codebase's existing honest-partial-completion
    // convention rather than a fabricated 1:1 claim.
    let rows: String = bill
        .iter()
        .enumerate()
        .map(|(i, b)| {
            let linked = b.get("linked").and_then(Value::as_bool).unwrap_or(false);
            let name = b.get("name").and_then(Value::as_str).unwrap_or("");
            if linked {
                let obj_id = b.get("obj_id").and_then(Value::as_str).unwrap_or("");
                let code = b.get("code").and_then(Value::as_str).unwrap_or("");
                format!(
                    r#"<a class="bim-bill-row bim-bill-row--linked" data-plan-obj="{i}" href="/compositions/{comp_slug}/o/{obj_id}"><span class="bim-bill-row__name">{name}</span><span class="bim-bill-row__code">{code} &middot; view object →</span></a>"#,
                    i = i,
                    comp_slug = esc(comp_slug),
                    obj_id = esc(obj_id),
                    name = esc(name),
                    code = esc(code),
                )
            } else {
                format!(
                    r#"<div class="bim-bill-row bim-bill-row--unlinked" data-plan-obj="{i}"><span class="bim-bill-row__name">{name}</span><span class="bim-bill-row__tag">not in catalog</span></div>"#,
                    i = i,
                    name = esc(name),
                )
            }
        })
        .collect();

    format!(r#"{status_chip}<div class="bim-bill-rows">{rows}</div>"#)
}

/// The Composition detail page. `highlight_object` is `Some(obj_slug)` for
/// `/compositions/{slug}/o/{object}` — the plan stays on the left; only the
/// right inspector rail swaps to the object's spec sheet, with a breadcrumb
/// and a "back to composition" link, so context is never destroyed (fixes
/// the 2026-07 audit's "clicking a bill row wipes the whole panel" finding).
pub fn render_composition_detail(
    state: &AppState,
    slug: &str,
    highlight_object: Option<&str>,
) -> Option<String> {
    let objects = build_objects(state);
    let compositions = build_compositions(state, &objects);
    let c = compositions.iter().find(|c| s(c, "slug") == slug)?;

    let name = s(c, "name");
    let id = s(c, "id");
    let cat_label = s(c, "category_label");
    let space = s(c, "uniclass_space");
    let description = s(c, "description");
    let has_zone = c
        .get("has_zone_data")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let area_sf = int_of(c, "area_sf");
    let area_m2 = c.get("area_m2").and_then(Value::as_str);

    let plan_svg = if has_zone {
        s(c, "svg").to_string()
    } else {
        super::svg::render_floor_scale_svg()
    };
    let plan_variant = if has_zone {
        ""
    } else {
        " bim-cat-preview--floorscale"
    };

    // Round 6 (2026-07-10) bug fix: this metadata used to sit at the bottom of
    // the (often much taller) rail. Since .bim-inspector-page__plan is sticky
    // and only as tall as the drawing itself, that left a large stretch of
    // empty page background below the plan on any composition with a long
    // parts list. It describes the plan, not the parts list, so it belongs
    // under the drawing in both the normal and object-highlight views.
    let plan_area_line = match (area_sf, area_m2) {
        (Some(sf), Some(m2)) => format!(
            r#"<div class="bim-cat-areabox"><span class="bim-cat-areabox__kicker">Data Box</span><span class="bim-cat-areabox__n">{sf}<small> SF</small></span><span class="bim-cat-areabox__l">Net leasable area &middot; {m2} m²</span></div>"#,
            sf = sf,
            m2 = esc(m2)
        ),
        (Some(sf), None) => format!(
            r#"<div class="bim-cat-areabox"><span class="bim-cat-areabox__kicker">Data Box</span><span class="bim-cat-areabox__n">{sf}<small> SF</small></span><span class="bim-cat-areabox__l">Net leasable area</span></div>"#,
            sf = sf
        ),
        _ => String::new(),
    };
    let plan_classblock = format!(
        r#"<div class="bim-cat-secth">Classification &amp; registry</div>
    <div class="bim-cat-classblock">
      <div class="bim-cat-clrow"><span class="bim-cat-clrow__k">Uniclass 2015 — SL (Spaces/locations)</span><span class="bim-cat-clrow__v">{space}</span></div>
      <div class="bim-cat-clrow"><span class="bim-cat-clrow__k">Key Plan reference</span><span class="bim-cat-clrow__v">{id}<small>{cat_label}</small></span></div>
    </div>"#,
        space = esc(space),
        id = esc(id),
        cat_label = esc(cat_label),
    );
    let plan_meta = format!(
        "{plan_area_line}{plan_classblock}",
        plan_area_line = plan_area_line,
        plan_classblock = plan_classblock
    );

    let breadcrumb_normal = format!(
        r#"<nav class="bim-breadcrumbs"><a href="/">Home</a> / <a href="/compositions">Compositions</a> / <span>{name}</span></nav>"#,
        name = esc(name)
    );

    let right_rail = if let Some(obj_slug) = highlight_object {
        let Some(o) = objects.iter().find(|o| s(o, "id") == obj_slug) else {
            return None;
        };
        let o_name = s(o, "name");
        let ifc_class = s(o, "ifc_class");
        let uni_pr = s(o, "uniclass_pr");
        let uni_title = s(o, "uniclass_pr_title");
        let manufacturer = s(o, "manufacturer");
        let description_o = s(o, "description");
        let spec_rows: String = o
            .get("spec")
            .and_then(Value::as_array)
            .map(|rows| spec_rows_grouped(rows))
            .unwrap_or_default();
        let dl = match o.get("ifc_file").and_then(Value::as_str) {
            Some(f) => format!(
                r#"<a class="bim-cat-btn" href="/furniture/download/{}">Download IFC (.ifc)</a>"#,
                esc(f)
            ),
            None => r#"<p class="bim-cat-note">No IFC block published for this object yet.</p>"#
                .to_string(),
        };
        format!(
            r#"<nav class="bim-breadcrumbs"><a href="/">Home</a> / <span class="bim-cat-chip bim-cat-chip--ef bim-cat-chip--inline"><span class="bim-cat-chip__lv">SL</span>{space}</span> / <a href="/compositions/{slug}">{id}</a> / <span>{o_name}</span></nav>
    <p class="bim-detail-backlink"><a href="/compositions/{slug}">← Back to {name}</a></p>
    <div class="bim-chip-row">
      <span class="bim-cat-chip bim-cat-chip--pr"><span class="bim-cat-chip__lv">Pr</span>{uni_pr}</span>
      <span class="bim-cat-chip bim-cat-chip--plain">{ifc_class}</span>
      <span class="bim-cat-chip bim-cat-chip--machine">IFC 4.3 &middot; DTCG</span>
    </div>
    <h2 class="bim-inspector__title">{o_name}</h2>
    <p class="bim-detail-head__prov">{manufacturer} &middot; BIM Object</p>
    <p class="bim-cat-desc">{description_o}</p>
    <div class="bim-cat-secth">Specification &amp; property set</div>
    <table class="bim-cat-spectable"><tbody>{spec_rows}</tbody></table>
    <div class="bim-cat-secth">Classification</div>
    <div class="bim-cat-classblock">
      <div class="bim-cat-clrow"><span class="bim-cat-clrow__k">IFC 4.3 entity class</span><span class="bim-cat-clrow__v">{ifc_class}</span></div>
      <div class="bim-cat-clrow"><span class="bim-cat-clrow__k">Uniclass 2015 — Pr</span><span class="bim-cat-clrow__v">{uni_pr}<small>{uni_title}</small></span></div>
    </div>
    <div class="bim-cat-secth">Download <span class="bim-cat-chip bim-cat-chip--machine">machine-readable</span></div>
    {dl}"#,
            slug = esc(slug),
            id = esc(id),
            name = esc(name),
            space = esc(space),
            o_name = esc(o_name),
            uni_pr = esc(uni_pr),
            uni_title = esc(uni_title),
            ifc_class = esc(ifc_class),
            manufacturer = esc(manufacturer),
            description_o = esc(description_o),
            spec_rows = spec_rows,
            dl = dl,
        )
    } else {
        let spec_rows: String = c
            .get("spec")
            .and_then(Value::as_array)
            .map(|rows| spec_rows_grouped(rows))
            .unwrap_or_default();
        format!(
            r#"{breadcrumb}
    <div class="bim-chip-row">
      <span class="bim-cat-chip bim-cat-chip--ef"><span class="bim-cat-chip__lv">SL</span>{space}</span>
      <span class="bim-cat-chip bim-cat-chip--plain">KEY PLAN</span>
    </div>
    <h2 class="bim-inspector__title">{name}</h2>
    <p class="bim-detail-head__prov">{id} &middot; {cat_label}</p>
    <p class="bim-cat-desc">{description}</p>
    <div class="bim-cat-secth">Zone allocation</div>
    {zone_bars}
    <div class="bim-cat-secth">Parts list <span class="bim-cat-secth__sub">every Object this assembly is built from</span></div>
    {bill}
    <div class="bim-cat-secth">Specification</div>
    <table class="bim-cat-spectable"><tbody>{spec_rows}</tbody></table>"#,
            breadcrumb = breadcrumb_normal,
            space = esc(space),
            name = esc(name),
            id = esc(id),
            cat_label = esc(cat_label),
            description = esc(description),
            zone_bars = zone_bars_html(c),
            bill = bill_html(c, slug),
            spec_rows = spec_rows,
        )
    };

    Some(format!(
        r#"<div class="bim-inspector-page">
  <div class="bim-inspector-page__plan">
    <div class="bim-cat-preview{plan_variant}">{plan_svg}</div>
    <div class="bim-inspector-page__plan-meta">{plan_meta}</div>
  </div>
  <div class="bim-inspector-page__rail">
    {right_rail}
  </div>
</div>"#,
        plan_variant = plan_variant,
        plan_svg = plan_svg,
        plan_meta = plan_meta,
        right_rail = right_rail,
    ))
}

// ─────────────────────────────────────────────────────────────────────────────
// 404 — branded, with full site chrome + search + section links
// ─────────────────────────────────────────────────────────────────────────────

pub fn render_not_found() -> String {
    r#"<div class="bim-notfound">
  <p class="bim-notfound__code">404</p>
  <h1>That page isn't in the registry.</h1>
  <p class="bim-cat-pagehead__lede">The page you followed a link to doesn't exist — or has moved. Try searching, or jump to a section below.</p>
  <form class="bim-cat-searchform" method="get" action="/search">
    <label class="bim-cat-search">
      <span class="bim-cat-search__ico" aria-hidden="true">⌕</span>
      <input type="search" name="q" placeholder="Search the registry" aria-label="Search">
    </label>
    <button class="bim-cat-searchform__submit" type="submit">Search</button>
  </form>
  <nav class="bim-notfound__links">
    <a href="/objects">Objects</a>
    <a href="/compositions">Compositions</a>
    <a href="/research">Research</a>
    <a href="/method">Method</a>
  </nav>
</div>"#
        .to_string()
}
