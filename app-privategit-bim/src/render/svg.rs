// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

use serde_json::Value;

pub fn render_kp_zone_svg_from_value(val: &Value) -> String {
    let z1 = val
        .get("zone1_depth_m")
        .and_then(|v| v.as_f64())
        .unwrap_or(6.0);
    let z2 = val
        .get("zone2_depth_m")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);
    let z3 = val
        .get("zone3_depth_m")
        .and_then(|v| v.as_f64())
        .filter(|v| *v > 0.0);
    let category = val
        .get("category")
        .and_then(|v| v.as_str())
        .unwrap_or("private-office");
    let area_m2 = val.get("area_m2").and_then(|v| v.as_f64());
    render_kp_zone_svg(z1, z2, z3, category, area_m2)
}

#[allow(dead_code)]
pub fn render_kp_fraction_svg(display_name: &str) -> String {
    let fraction = if display_name.contains("1/8") {
        0.125
    } else if display_name.contains("1/4") {
        0.25
    } else if display_name.contains("1/3") {
        1.0 / 3.0
    } else if display_name.contains("1/2") {
        0.5
    } else {
        1.0
    };
    let fill_w = (164.0 * fraction) as u32;
    let label = if display_name.contains("1/8") {
        "1/8 Floor"
    } else if display_name.contains("1/4") {
        "1/4 Floor"
    } else if display_name.contains("1/3") {
        "1/3 Floor"
    } else if display_name.contains("1/2") {
        "1/2 Floor"
    } else {
        "Full Floor"
    };
    format!(
        "<svg class=\"bim-kp-diagram\" viewBox=\"0 0 180 112\" xmlns=\"http://www.w3.org/2000/svg\" aria-hidden=\"true\">\
<text x=\"90\" y=\"8\" font-size=\"7\" fill=\"#888\" font-family=\"sans-serif\" text-anchor=\"middle\" letter-spacing=\"1.5\">FLOOR PLATE</text>\
<rect x=\"8\" y=\"12\" width=\"164\" height=\"88\" fill=\"#ebebeb\" stroke=\"#ccc\" stroke-width=\"0.5\"/>\
<rect x=\"8\" y=\"12\" width=\"{fw}\" height=\"88\" fill=\"#c8d8e8\" stroke=\"#a0b8cc\" stroke-width=\"0.5\"/>\
<text x=\"90\" y=\"62\" font-size=\"14\" fill=\"#5a7898\" font-family=\"sans-serif\" text-anchor=\"middle\" font-weight=\"600\">{lbl}</text>\
<text x=\"90\" y=\"80\" font-size=\"8\" fill=\"#888\" font-family=\"sans-serif\" text-anchor=\"middle\">of net leasable area</text>\
<text x=\"90\" y=\"110\" font-size=\"7\" fill=\"#888\" font-family=\"sans-serif\" text-anchor=\"middle\" letter-spacing=\"1.5\">SIZED AGAINST FLOOR PLATE</text>\
</svg>",
        fw = fill_w,
        lbl = label,
    )
}

// obj_idx's final increment in each category branch below is legitimately
// unread (nothing after the match block needs the running count) — not a
// real bug, just how a running counter's last write always looks to this
// lint.
#[allow(unused_assignments)]
pub fn render_kp_zone_svg(
    z1: f64,
    z2: f64,
    z3: Option<f64>,
    category: &str,
    area_m2: Option<f64>,
) -> String {
    let d3 = z3.unwrap_or(0.0);
    let total = z1 + z2 + d3;
    if total <= 0.0 {
        return String::new();
    }

    // "The Plan Room" (2026-07-09 v3 redesign): one drawing language, one
    // navy, for every category — retires the prior per-category hex accent
    // (cream/tan/maroon/olive sub-palette the 2026-07 audit flagged as a
    // second, uncoordinated design system). `accent` now always resolves to
    // the same brand navy CSS variable regardless of `category`; every
    // literal furniture-fill/stroke hex below is likewise replaced with the
    // shared token palette (`--bim-accent` / `--bim-accent-active` /
    // `--bim-accent-subtle`) so the diagram themes with the page for free —
    // see BRIEF-bim-v3-hyperscaler-redesign.md `composition_detail_treatment`.
    let accent = "var(--bim-accent)";
    let accent_active = "var(--bim-accent-active)";
    let accent_subtle = "var(--bim-accent-subtle)";
    let caption = "var(--bim-fg-caption)";

    // Drawing area: x=22, y=10, max_w=153, h=94 within 180×112 viewBox.
    // Left 22px reserved for the Habitat/Magazine/Corridor letter labels.
    let x0: f64 = 22.0;
    let y0: f64 = 10.0;
    let max_dw: f64 = 153.0;
    let dh: f64 = 94.0;

    // Proportional width: frontage = area / depth. Normalise against 6 m reference.
    let frontage = area_m2.map(|a| a / total).unwrap_or(total);
    let plan_w = ((frontage / 6.0) * max_dw).clamp(max_dw * 0.30, max_dw);
    let xr: f64 = x0 + plan_w;

    let size_tier: u8 = match (category, area_m2) {
        ("private-office", Some(a)) => {
            if a < 38.0 {
                0
            } else if a < 55.0 {
                1
            } else {
                2
            }
        }
        ("medical", Some(a)) => {
            if a < 270.0 {
                0
            } else if a < 410.0 {
                1
            } else {
                2
            }
        }
        ("laboratory", Some(a)) => {
            if a < 260.0 {
                0
            } else if a < 370.0 {
                1
            } else {
                2
            }
        }
        ("academic", Some(a)) => {
            if a < 175.0 {
                0
            } else if a < 315.0 {
                1
            } else {
                2
            }
        }
        ("business", Some(a)) => {
            if a < 360.0 {
                0
            } else if a < 545.0 {
                1
            } else {
                2
            }
        }
        ("civic", Some(a)) => {
            if a < 420.0 {
                0
            } else if a < 700.0 {
                1
            } else {
                2
            }
        }
        _ => 1,
    };

    let h1 = (z1 / total) * dh;
    let h2 = (z2 / total) * dh;
    let h3 = dh - h1 - h2;
    let y1 = y0 + h1;
    let y2 = y1 + h2;

    let lz1a = y0 + h1 * 0.38;
    let lz1b = y0 + h1 * 0.64;
    let lz2a = y1 + h2 * 0.38;
    let lz2b = y1 + h2 * 0.64;
    let lz3a = y2 + h3 * 0.38;
    let lz3b = y2 + h3 * 0.68;

    let mut s = String::with_capacity(2400);

    // Explicit width/height (2x viewBox, crisp at typical card sizes) fixes
    // the 2026-07 audit's 2px-collapse bug: inside a flex-column ancestor
    // (`.bim-cat-modal__body`), an SVG with only a `viewBox` has no intrinsic
    // size to resolve `height:auto` against. CSS `aspect-ratio: 180/112` on
    // `.bim-kp-diagram` (bim-planroom.css) is the second, belt-and-suspenders
    // layer — the diagram can no longer collapse under any container.
    s.push_str("<svg class=\"bim-kp-diagram\" viewBox=\"0 0 180 112\" width=\"360\" height=\"224\" xmlns=\"http://www.w3.org/2000/svg\" role=\"img\" aria-label=\"Key Plan footprint\">");
    s.push_str(&format!(
        "<rect width=\"180\" height=\"112\" fill=\"{}\"/>",
        "var(--bim-bg-surface)"
    ));

    // FACADE label centered over the actual plan footprint (not a fixed
    // viewBox x) with a reserved gap in the mullion-tick run underneath it —
    // fixes the 2026-07 audit's label/tick collision by construction rather
    // than by z-order.
    let plan_center_x = x0 + plan_w / 2.0;
    let mull_step = plan_w / 5.0;
    let gap_half = 13.0;
    s.push_str(&format!(
        "<rect x=\"{:.1}\" y=\"3\" width=\"26\" height=\"7\" fill=\"{}\"/>",
        plan_center_x - 13.0,
        "var(--bim-bg-surface)"
    ));
    s.push_str(&format!(
        "<text x=\"{:.1}\" y=\"8.5\" font-size=\"5.5\" fill=\"{}\" class=\"bim-plan-mono bim-plan-mono--dim\" text-anchor=\"middle\" letter-spacing=\"1.2\">FACADE</text>",
        plan_center_x, accent
    ));

    // Mullion ticks (4 evenly spaced along facade edge), skipping any that
    // would fall inside the FACADE label's reserved gap.
    for i in 1u8..=4 {
        let mx = x0 + mull_step * i as f64;
        if (mx - plan_center_x).abs() < gap_half {
            continue;
        }
        s.push_str(&format!(
            "<line x1=\"{:.1}\" y1=\"6\" x2=\"{:.1}\" y2=\"{:.0}\" stroke=\"{}\" stroke-width=\"0.8\"/>",
            mx, mx, y0, accent
        ));
    }

    // Zone fills — three opacity tiers of the same navy selection wash
    // (`--bim-accent-subtle`), not three unrelated hues, so the plan themes
    // with the page.
    s.push_str(&format!(
        "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"{}\" fill-opacity=\"0.9\"/>",
        x0, y0, plan_w, h1, accent_subtle
    ));
    s.push_str(&format!(
        "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"{}\" fill-opacity=\"0.55\"/>",
        x0, y1, plan_w, h2, accent_subtle
    ));
    if h3 >= 1.0 {
        s.push_str(&format!(
            "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"{}\" fill-opacity=\"0.25\"/>",
            x0, y2, plan_w, h3, accent_subtle
        ));
    }

    // Perimeter (primary line weight)
    s.push_str(&format!(
        "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"none\" stroke=\"{}\" stroke-width=\"1.2\"/>",
        x0, y0, plan_w, dh, accent
    ));

    // Zone boundary dashed lines (secondary weight)
    s.push_str(&format!(
        "<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"{}\" stroke-dasharray=\"3.5,2.5\" stroke-width=\"0.75\"/>",
        x0, y1, xr, y1, accent_active
    ));
    if h3 >= 1.0 {
        s.push_str(&format!(
            "<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"{}\" stroke-dasharray=\"3.5,2.5\" stroke-width=\"0.75\"/>",
            x0, y2, xr, y2, accent_active
        ));
    }

    // Habitat/Magazine/Corridor labels (left of plan) — caption weight,
    // uniform across all three. Round 7 (2026-07-10): were "Z1"/"Z2"/"Z3" —
    // dropped the "Zone N" ordinal (real research found no architectural
    // standard, IFC construct, or mainstream BIM tool uses it; Revit's own
    // default zone is literally "Default," IFC's IfcZone means something
    // else entirely) in favour of single-letter initials of the real
    // proprietary names, same footprint as the labels they replace. The
    // letters are the primary identity of each band (kept visible at every
    // viewport, class `--zone`); the depth-in-metres numbers below each
    // letter are tertiary detail (class `--dim`) — real, but the least
    // load-bearing text on the diagram, and the one Round 5's live 360px
    // audit found rendering as small as 4.0px computed. `--dim` nodes are
    // hidden below a render-width threshold in bim-planroom.css rather than
    // shrunk further; the letters stay legible on their own at every size.
    // Full names (Habitat/Magazine/Corridor) render alongside the parallel
    // HTML zone-bar view (`zone_bars_html`, catalog.rs) and in the
    // surrounding prose/figcaption — this diagram isn't the only place a
    // reader sees the real name.
    s.push_str(&format!(
        "<text x=\"21\" y=\"{:.1}\" font-size=\"5\" fill=\"{}\" class=\"bim-plan-mono bim-plan-mono--zone\" text-anchor=\"end\">H</text>",
        lz1a, caption
    ));
    s.push_str(&format!(
        "<text x=\"21\" y=\"{:.1}\" font-size=\"4\" fill=\"{}\" class=\"bim-plan-mono bim-plan-mono--dim\" text-anchor=\"end\">{:.1}m</text>",
        lz1b, caption, z1
    ));
    s.push_str(&format!(
        "<text x=\"21\" y=\"{:.1}\" font-size=\"5\" fill=\"{}\" class=\"bim-plan-mono bim-plan-mono--zone\" text-anchor=\"end\">M</text>",
        lz2a, caption
    ));
    s.push_str(&format!(
        "<text x=\"21\" y=\"{:.1}\" font-size=\"4\" fill=\"{}\" class=\"bim-plan-mono bim-plan-mono--dim\" text-anchor=\"end\">{:.1}m</text>",
        lz2b, caption, z2
    ));
    if h3 >= 8.0 {
        s.push_str(&format!(
            "<text x=\"21\" y=\"{:.1}\" font-size=\"5\" fill=\"{}\" class=\"bim-plan-mono bim-plan-mono--zone\" text-anchor=\"end\">C</text>",
            lz3a, caption
        ));
        if h3 >= 14.0 {
            s.push_str(&format!(
                "<text x=\"21\" y=\"{:.1}\" font-size=\"4\" fill=\"{}\" class=\"bim-plan-mono bim-plan-mono--dim\" text-anchor=\"end\">{:.1}m</text>",
                lz3b, caption, d3
            ));
        }
    }

    // ── Furniture macros ───────────────────────────────────────────────────────
    macro_rules! desk {
        ($s:expr, $dx:expr, $dy:expr) => {
            $s.push_str(&format!(
                "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"15\" height=\"9\" fill=\"var(--bim-accent-subtle)\" stroke=\"var(--bim-accent-active)\" stroke-width=\"0.5\" rx=\"0.5\"/>",
                $dx, $dy
            ));
            $s.push_str(&format!(
                "<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"3\" fill=\"var(--bim-accent-subtle)\" stroke=\"var(--bim-accent-active)\" stroke-width=\"0.4\"/>",
                ($dx as f64) + 7.5,
                ($dy as f64) + 13.0
            ));
        };
    }
    macro_rules! round_table {
        ($s:expr, $cx:expr, $cy:expr, $r:expr, $n:expr) => {{
            let (cx, cy, r) = ($cx as f64, $cy as f64, $r as f64);
            $s.push_str(&format!(
                "<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"{:.1}\" fill=\"var(--bim-accent-subtle)\" stroke=\"var(--bim-accent-active)\" stroke-width=\"0.5\"/>",
                cx, cy, r
            ));
            let offsets: &[(f64, f64)] =
                &[(0.0, -(r + 3.5)), (r + 3.5, 0.0), (0.0, r + 3.5), (-(r + 3.5), 0.0)];
            for &(dx, dy) in offsets.iter().take($n) {
                $s.push_str(&format!(
                    "<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"2.5\" fill=\"var(--bim-accent-subtle)\" stroke=\"var(--bim-accent-active)\" stroke-width=\"0.4\"/>",
                    cx + dx,
                    cy + dy
                ));
            }
        }};
    }
    macro_rules! door {
        ($s:expr, $dx:expr, $dy:expr, $dh:expr) => {{
            let (dx, dy, dh) = ($dx as f64, $dy as f64, $dh as f64);
            $s.push_str(&format!(
                "<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"var(--bim-accent-active)\" stroke-width=\"0.75\"/>",
                dx, dy, dx, dy + dh
            ));
            $s.push_str(&format!(
                "<path d=\"M{:.1},{:.1} A{:.1},{:.1} 0 0,1 {:.1},{:.1}\" stroke=\"var(--bim-accent-active)\" stroke-width=\"0.75\" fill=\"none\" stroke-dasharray=\"2,1.5\"/>",
                dx, dy, dh, dh, dx + dh * 0.87, dy + dh * 0.5
            ));
        }};
    }

    // Plan <-> parts-list hover linkage (item 7b, 2026-07-09): every discrete
    // furniture instance drawn below that plausibly corresponds to one
    // "PARTS LIST" row is wrapped in `<g class="bim-plan-obj"
    // data-plan-obj="{obj_idx}">`, with obj_idx incrementing once per item in
    // the same draw order every time. render/catalog.rs's bill_html tags
    // parts-list rows with the same ordinal (0, 1, 2, ...), capped to
    // whichever list — SVG groups or bill rows — is shorter for that
    // composition, and bim.js highlights both ends of a match on hover.
    // This is a stable ORDINAL pairing, not a semantic per-SKU match: the
    // plan is a procedurally-drawn category template (this function has no
    // knowledge of which specific catalog Object a given desk/table symbol
    // represents), not a literal trace of each linked Object's real
    // footprint — only PO-1 carries a real per-item `furniture_refs` bill
    // at all (see catalog.rs's module doc). Singular one-off fixtures
    // (a single credenza, reception desk, or service riser per plan) are
    // deliberately left unwrapped — a stated scope cut; the repeating
    // desk/workstation/chair/bench/office groups that dominate each
    // category template, and are the ones most analogous to real parts-list
    // rows, are the ones wired up.
    let mut obj_idx: usize = 0;
    macro_rules! obj_open {
        ($s:expr) => {
            $s.push_str(&format!(
                "<g class=\"bim-plan-obj\" data-plan-obj=\"{}\">",
                obj_idx
            ));
        };
    }
    macro_rules! obj_close {
        ($s:expr) => {{
            $s.push_str("</g>");
            obj_idx += 1;
        }};
    }

    match category {
        // ── Private Office ─────────────────────────────────────────────────
        "private-office" => {
            let desk_n = size_tier as usize + 1;
            for i in 0..desk_n {
                obj_open!(s);
                desk!(s, x0 + 3.0 + 19.0 * i as f64, y0 + 3.0);
                obj_close!(s);
            }
            if h1 >= 25.0 {
                let tbl_r = (h1 * 0.18).clamp(7.0, 10.0);
                let tbl_x = (x0 + plan_w * 0.58).min(xr - tbl_r - 12.0);
                obj_open!(s);
                round_table!(s, tbl_x, y0 + h1 * 0.72, tbl_r, 3);
                obj_close!(s);
            }
            let cred_x = (xr - 17.0).max(x0 + 3.0 + 19.0 * desk_n as f64 + 3.0);
            obj_open!(s);
            s.push_str(&format!(
                "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"14\" height=\"5\" fill=\"var(--bim-accent-subtle)\" stroke=\"var(--bim-accent-active)\" stroke-width=\"0.4\" rx=\"0.3\"/>",
                cred_x, y0 + 3.0
            ));
            obj_close!(s);
            if h2 >= 10.0 {
                let cw = (plan_w * 0.65).min(85.0);
                obj_open!(s);
                s.push_str(&format!(
                    "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"5\" fill=\"var(--bim-accent-subtle)\" stroke=\"var(--bim-accent-active)\" stroke-width=\"0.5\" rx=\"0.3\"/>",
                    x0 + 3.0, y1 + 3.0, cw
                ));
                s.push_str(&format!(
                    "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"9\" height=\"9\" fill=\"var(--bim-accent-subtle)\" stroke=\"var(--bim-accent-active)\" stroke-width=\"0.4\" rx=\"0.3\"/>",
                    (x0 + cw + 5.0).min(xr - 12.0), y1 + 2.0
                ));
                obj_close!(s);
            }
            if h3 >= 10.0 {
                obj_open!(s);
                door!(s, x0 + 4.0, y2, (h3 * 0.85).min(13.0));
                obj_close!(s);
            }
        }

        // ── Medical ────────────────────────────────────────────────────────
        "medical" => {
            let doc_n = if size_tier == 2 { 2usize } else { 1 };
            let chair_n = match size_tier {
                0 => 2usize,
                1 => 4,
                _ => 6,
            };
            for i in 0..doc_n {
                let ox = x0 + 1.0 + 21.0 * i as f64;
                obj_open!(s);
                s.push_str(&format!(
                    "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"19\" height=\"{:.1}\" fill=\"var(--bim-accent-subtle)\" stroke=\"{}\" stroke-width=\"0.6\"/>",
                    ox, y0, h1, accent
                ));
                s.push_str(&format!(
                    "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"10\" height=\"6\" fill=\"var(--bim-accent-subtle)\" stroke=\"var(--bim-accent-active)\" stroke-width=\"0.4\" rx=\"0.3\"/>",
                    ox + 2.0, y0 + h1 - 9.0
                ));
                obj_close!(s);
            }
            let ch_x0 = x0 + 2.0 + 21.0 * doc_n as f64;
            let ch_area = (xr - 28.0) - ch_x0;
            if h1 >= 15.0 && ch_area > 0.0 {
                let sp = (ch_area / chair_n as f64).max(11.0);
                let cy = y0 + h1 * 0.35;
                for i in 0..chair_n {
                    let cx = ch_x0 + sp * i as f64;
                    if cx + 10.0 > xr - 28.0 {
                        break;
                    }
                    obj_open!(s);
                    s.push_str(&format!(
                        "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"10\" height=\"7\" fill=\"var(--bim-accent-subtle)\" stroke=\"var(--bim-accent-active)\" stroke-width=\"0.5\" rx=\"1.5\"/>",
                        cx, cy
                    ));
                    s.push_str(&format!(
                        "<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"3.5\" fill=\"var(--bim-accent-subtle)\" stroke=\"var(--bim-accent-active)\" stroke-width=\"0.4\"/>",
                        cx + 5.0, cy - 4.0
                    ));
                    obj_close!(s);
                }
            }
            s.push_str(&format!(
                "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"22\" height=\"7\" fill=\"var(--bim-accent-subtle)\" stroke=\"var(--bim-accent-active)\" stroke-width=\"0.5\" rx=\"0.5\"/>",
                xr - 26.0, y0 + 4.0
            ));
            if h2 >= 10.0 {
                let bw = (plan_w * 0.60).min(100.0);
                s.push_str(&format!(
                    "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"6\" fill=\"var(--bim-accent-subtle)\" stroke=\"var(--bim-accent-active)\" stroke-width=\"0.5\" rx=\"0.5\"/>",
                    x0 + 4.0, y1 + 3.0, bw
                ));
                let sects = (bw / 18.0) as usize;
                for i in 1..sects {
                    let bx = x0 + 4.0 + 18.0 * i as f64;
                    s.push_str(&format!(
                        "<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"var(--bim-accent-active)\" stroke-width=\"0.3\"/>",
                        bx, y1 + 3.0, bx, y1 + 9.0
                    ));
                }
                s.push_str(&format!(
                    "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"11\" height=\"11\" fill=\"var(--bim-accent-subtle)\" stroke=\"var(--bim-accent-active)\" stroke-width=\"0.5\" rx=\"1\"/>",
                    xr - 15.0, y1 + 2.0
                ));
            }
            if h3 >= 8.0 {
                s.push_str(&format!(
                    "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"16\" height=\"{:.1}\" fill=\"var(--bim-accent-subtle)\" stroke=\"var(--bim-accent-active)\" stroke-width=\"0.5\" rx=\"0.5\"/>",
                    xr - 20.0, y2 + 2.0, (h3 - 4.0).max(5.0)
                ));
            }
        }

        // ── Laboratory ─────────────────────────────────────────────────────
        "laboratory" => {
            let office_n = if size_tier == 0 { 1usize } else { 2 };
            let bench_n = match size_tier {
                0 => 3usize,
                1 => 5,
                _ => 7,
            };
            let rec_h = (h1 * 0.55).max(15.0).min(h1);
            s.push_str(&format!(
                "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"16\" height=\"{:.1}\" fill=\"var(--bim-accent-subtle)\" stroke=\"{}\" stroke-width=\"0.5\"/>",
                x0 + 1.0, y0, rec_h, accent
            ));
            for i in 0..office_n {
                let ox = x0 + 19.0 + 20.0 * i as f64;
                let off_h = (h1 * 0.65).min(h1);
                obj_open!(s);
                s.push_str(&format!(
                    "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"18\" height=\"{:.1}\" fill=\"var(--bim-accent-subtle)\" stroke=\"{}\" stroke-width=\"0.5\"/>",
                    ox, y0, off_h, accent
                ));
                if off_h >= 16.0 {
                    desk!(s, ox + 2.0, y0 + off_h - 13.0);
                }
                obj_close!(s);
            }
            let bx0 = x0 + 20.0 + 20.0 * office_n as f64;
            let b_area = xr - 4.0 - bx0;
            if b_area > 0.0 && h1 >= 12.0 {
                let bs = b_area / bench_n as f64;
                for i in 0..bench_n {
                    let bx = bx0 + bs * i as f64;
                    if bx + 11.0 > xr - 2.0 {
                        break;
                    }
                    obj_open!(s);
                    s.push_str(&format!(
                        "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"11\" height=\"6\" fill=\"var(--bim-accent-subtle)\" stroke=\"var(--bim-accent-active)\" stroke-width=\"0.5\" rx=\"0.3\"/>",
                        bx, y0 + 4.0
                    ));
                    s.push_str(&format!(
                        "<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"2.5\" fill=\"var(--bim-accent-subtle)\" stroke=\"var(--bim-accent-active)\" stroke-width=\"0.4\"/>",
                        bx + 5.5, y0 + 14.0
                    ));
                    obj_close!(s);
                }
            }
            if h2 >= 10.0 {
                let sr_w = 30.0f64;
                s.push_str(&format!(
                    "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"var(--bim-accent-subtle)\" stroke=\"{}\" stroke-width=\"0.5\"/>",
                    x0 + 1.0, y1, sr_w, h2 * 0.85, accent
                ));
                round_table!(s, x0 + 1.0 + sr_w / 2.0, y1 + h2 * 0.42, 6.0, 4);
                let sb_w = (plan_w - sr_w - 10.0).clamp(0.0, 100.0);
                if sb_w > 0.0 {
                    s.push_str(&format!(
                        "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"6\" fill=\"var(--bim-accent-subtle)\" stroke=\"var(--bim-accent-active)\" stroke-width=\"0.4\" rx=\"0.3\"/>",
                        x0 + sr_w + 5.0, y1 + 3.0, sb_w
                    ));
                }
            }
            if h3 >= 8.0 {
                s.push_str(&format!(
                    "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"32\" height=\"5\" fill=\"var(--bim-accent-subtle)\" stroke=\"var(--bim-accent-active)\" stroke-width=\"0.5\" rx=\"0.3\"/>",
                    x0 + 4.0, y2 + 2.0
                ));
            }
        }

        // ── Business ───────────────────────────────────────────────────────
        "business" => {
            let office_n: usize = match size_tier {
                0 => 2,
                1 => 3,
                _ => 5,
            };
            let ws_cols: usize = match size_tier {
                0 => 3,
                1 => 4,
                _ => 5,
            };
            let ws_rows: usize = match size_tier {
                0 => 3,
                1 => 4,
                _ => 5,
            };
            let conf_n: usize = if size_tier == 2 { 2 } else { 1 };
            let col_n = if office_n > 3 { 2usize } else { 1 };
            let per_col = office_n.div_ceil(col_n);
            let oh = ((h1 - 6.0) / per_col as f64).min(13.0);
            s.push_str(&format!(
                "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"3\" fill=\"var(--bim-accent-subtle)\" stroke=\"var(--bim-accent-active)\" stroke-width=\"0.3\" rx=\"0.3\"/>",
                x0 + 1.0, y0, 16.0 * col_n as f64 + 2.0 * (col_n - 1) as f64
            ));
            for i in 0..per_col {
                let oy = y0 + 5.0 + oh * i as f64;
                if oy + oh > y0 + h1 - 1.0 {
                    break;
                }
                obj_open!(s);
                s.push_str(&format!(
                    "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"16\" height=\"{:.1}\" fill=\"var(--bim-accent-subtle)\" stroke=\"{}\" stroke-width=\"0.5\"/>",
                    x0 + 1.0, oy, oh - 0.5, accent
                ));
                obj_close!(s);
            }
            if col_n == 2 {
                for i in 0..(office_n - per_col) {
                    let oy = y0 + 5.0 + oh * i as f64;
                    if oy + oh > y0 + h1 - 1.0 {
                        break;
                    }
                    obj_open!(s);
                    s.push_str(&format!(
                        "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"16\" height=\"{:.1}\" fill=\"var(--bim-accent-subtle)\" stroke=\"{}\" stroke-width=\"0.5\"/>",
                        x0 + 19.0, oy, oh - 0.5, accent
                    ));
                    obj_close!(s);
                }
            }
            let ws_x0 = x0 + 2.0 + 18.0 * col_n as f64;
            let ws_aw = xr - ws_x0 - 3.0;
            let ws_sx = ws_aw / ws_cols as f64;
            let ws_sy = (h1 - 2.0) / ws_rows as f64;
            for row in 0..ws_rows {
                for col in 0..ws_cols {
                    let wx = ws_x0 + ws_sx * col as f64;
                    let wy = y0 + 1.0 + ws_sy * row as f64;
                    let ww = (ws_sx - 2.0).clamp(5.0, 16.0);
                    let wh = (ws_sy - 1.5).clamp(3.0, 10.0);
                    if wx + ww > xr - 2.0 {
                        break;
                    }
                    obj_open!(s);
                    s.push_str(&format!(
                        "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"var(--bim-accent-subtle)\" stroke=\"var(--bim-accent-active)\" stroke-width=\"0.4\" rx=\"0.3\"/>",
                        wx, wy, ww, wh
                    ));
                    obj_close!(s);
                }
            }
            if h2 >= 12.0 {
                let cw = (plan_w * 0.38).min(58.0);
                let ch = (h2 * 0.48).clamp(8.0, 16.0);
                for ci in 0..conf_n {
                    let cx_t = x0 + 4.0 + ci as f64 * (cw + 6.0);
                    if cx_t + cw > xr - 26.0 {
                        break;
                    }
                    let cy_t = y1 + (h2 - ch) / 2.0;
                    s.push_str(&format!(
                        "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"var(--bim-accent-subtle)\" stroke=\"var(--bim-accent-active)\" stroke-width=\"0.5\" rx=\"1\"/>",
                        cx_t, cy_t, cw, ch
                    ));
                    let cc = ((cw / 10.0) as usize).max(2);
                    for j in 0..cc {
                        let chair_x = cx_t + (cw / cc as f64) * (j as f64 + 0.5) - 3.0;
                        s.push_str(&format!(
                            "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"6\" height=\"3.5\" fill=\"var(--bim-accent-subtle)\" stroke=\"var(--bim-accent-active)\" stroke-width=\"0.3\" rx=\"0.5\"/>",
                            chair_x, cy_t - 4.5
                        ));
                        s.push_str(&format!(
                            "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"6\" height=\"3.5\" fill=\"var(--bim-accent-subtle)\" stroke=\"var(--bim-accent-active)\" stroke-width=\"0.3\" rx=\"0.5\"/>",
                            chair_x, cy_t + ch + 1.0
                        ));
                    }
                }
                s.push_str(&format!(
                    "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"22\" height=\"{:.1}\" fill=\"var(--bim-accent-subtle)\" stroke=\"{}\" stroke-width=\"0.4\"/>",
                    xr - 24.0, y1, h2 * 0.75, accent
                ));
            }
            if h3 >= 8.0 {
                s.push_str(&format!(
                    "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"16\" height=\"{:.1}\" fill=\"var(--bim-accent-subtle)\" stroke=\"var(--bim-accent-active)\" stroke-width=\"0.5\" rx=\"0.3\"/>",
                    xr - 20.0, y2 + 1.0, (h3 - 3.0).max(4.0)
                ));
            }
        }

        // ── Academic ───────────────────────────────────────────────────────
        "academic" => {
            match size_tier {
                0 => {
                    for row in 0..4usize {
                        for col in 0..2usize {
                            obj_open!(s);
                            s.push_str(&format!(
                                "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"10\" height=\"7\" fill=\"var(--bim-accent-subtle)\" stroke=\"var(--bim-accent-active)\" stroke-width=\"0.4\" rx=\"0.3\"/>",
                                x0 + 3.0 + col as f64 * 12.0, y0 + 5.0 + row as f64 * 12.0
                            ));
                            obj_close!(s);
                        }
                    }
                    let ctw = 42.0f64;
                    let cth = (h1 * 0.45).clamp(14.0, 22.0);
                    let cty = y0 + (h1 - cth) / 2.0;
                    s.push_str(&format!(
                        "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"var(--bim-accent-subtle)\" stroke=\"var(--bim-accent-active)\" stroke-width=\"0.5\" rx=\"1\"/>",
                        x0 + 29.0, cty, ctw, cth
                    ));
                    obj_open!(s);
                    round_table!(s, x0 + 86.0, y0 + h1 * 0.28, 8.0, 4);
                    obj_close!(s);
                    obj_open!(s);
                    round_table!(s, x0 + 86.0, y0 + h1 * 0.72, 8.0, 4);
                    obj_close!(s);
                }
                1 => {
                    for row in 0..4usize {
                        for col in 0..2usize {
                            obj_open!(s);
                            s.push_str(&format!(
                                "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"10\" height=\"7\" fill=\"var(--bim-accent-subtle)\" stroke=\"var(--bim-accent-active)\" stroke-width=\"0.4\" rx=\"0.3\"/>",
                                x0 + 3.0 + col as f64 * 12.0, y0 + 5.0 + row as f64 * 12.0
                            ));
                            obj_close!(s);
                            let rx2 = xr - 25.0 + col as f64 * 12.0;
                            if rx2 + 10.0 < xr - 2.0 {
                                obj_open!(s);
                                s.push_str(&format!(
                                    "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"10\" height=\"7\" fill=\"var(--bim-accent-subtle)\" stroke=\"var(--bim-accent-active)\" stroke-width=\"0.4\" rx=\"0.3\"/>",
                                    rx2, y0 + 5.0 + row as f64 * 12.0
                                ));
                                obj_close!(s);
                            }
                        }
                    }
                    let ctw = 58.0f64;
                    let cth = (h1 * 0.50).clamp(18.0, 26.0);
                    let cty = y0 + (h1 - cth) / 2.0;
                    s.push_str(&format!(
                        "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"var(--bim-accent-subtle)\" stroke=\"var(--bim-accent-active)\" stroke-width=\"0.5\" rx=\"{:.1}\"/>",
                        x0 + 29.0, cty, ctw, cth, cth / 2.0
                    ));
                    obj_open!(s);
                    round_table!(s, x0 + 104.0, y0 + h1 * 0.5, 8.0, 4);
                    obj_close!(s);
                }
                _ => {
                    let t_rows = ((h1 - 8.0) / 8.0) as usize;
                    for row in 0..t_rows.min(6) {
                        for col in 0..5usize {
                            obj_open!(s);
                            s.push_str(&format!(
                                "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"7\" height=\"5\" fill=\"var(--bim-accent-subtle)\" stroke=\"var(--bim-accent-active)\" stroke-width=\"0.3\" rx=\"0.5\"/>",
                                x0 + 3.0 + col as f64 * 9.0, y0 + 5.0 + row as f64 * 8.0
                            ));
                            obj_close!(s);
                        }
                    }
                    for row in 0..4usize {
                        for col in 0..2usize {
                            obj_open!(s);
                            s.push_str(&format!(
                                "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"10\" height=\"7\" fill=\"var(--bim-accent-subtle)\" stroke=\"var(--bim-accent-active)\" stroke-width=\"0.4\" rx=\"0.3\"/>",
                                x0 + 52.0 + col as f64 * 12.0, y0 + 5.0 + row as f64 * 12.0
                            ));
                            obj_close!(s);
                        }
                    }
                    obj_open!(s);
                    round_table!(s, x0 + 98.0, y0 + h1 * 0.28, 9.0, 4);
                    obj_close!(s);
                    obj_open!(s);
                    round_table!(s, x0 + 98.0, y0 + h1 * 0.72, 9.0, 4);
                    obj_close!(s);
                    obj_open!(s);
                    round_table!(s, x0 + 128.0, y0 + h1 * 0.5, 9.0, 4);
                    obj_close!(s);
                }
            }
            if h2 >= 12.0 {
                obj_open!(s);
                desk!(s, x0 + 4.0, y1 + 3.0);
                obj_close!(s);
                if size_tier >= 1 {
                    obj_open!(s);
                    desk!(s, x0 + 24.0, y1 + 3.0);
                    obj_close!(s);
                }
                let sw = (plan_w * 0.32).min(48.0);
                s.push_str(&format!(
                    "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"5\" fill=\"var(--bim-accent-subtle)\" stroke=\"var(--bim-accent-active)\" stroke-width=\"0.4\" rx=\"0.3\"/>",
                    xr - sw - 4.0, y1 + 4.0, sw
                ));
            }
        }

        // ── Civic ──────────────────────────────────────────────────────────
        "civic" => {
            let office_n: usize = match size_tier {
                0 => 2,
                1 => 4,
                _ => 5,
            };
            let conf_n: usize = match size_tier {
                0 => 1,
                _ => 2,
            };
            let ocols = if office_n > 3 { 2usize } else { 1 };
            let oper_col = office_n.div_ceil(ocols);
            let oh = ((h1 - 2.0) / oper_col as f64).min(12.0);
            for i in 0..oper_col {
                let oy = y0 + 1.0 + oh * i as f64;
                if oy + oh > y0 + h1 - 1.0 {
                    break;
                }
                obj_open!(s);
                s.push_str(&format!(
                    "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"13\" height=\"{:.1}\" fill=\"var(--bim-accent-subtle)\" stroke=\"{}\" stroke-width=\"0.4\"/>",
                    x0 + 1.0, oy, oh - 0.5, accent
                ));
                obj_close!(s);
            }
            if ocols == 2 {
                for i in 0..(office_n - oper_col) {
                    let oy = y0 + 1.0 + oh * i as f64;
                    if oy + oh > y0 + h1 - 1.0 {
                        break;
                    }
                    obj_open!(s);
                    s.push_str(&format!(
                        "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"13\" height=\"{:.1}\" fill=\"var(--bim-accent-subtle)\" stroke=\"{}\" stroke-width=\"0.4\"/>",
                        x0 + 16.0, oy, oh - 0.5, accent
                    ));
                    obj_close!(s);
                }
            }
            let court_w = if size_tier == 2 { 36.0f64 } else { 0.0 };
            let conf_zone_x = xr - (conf_n as f64 * 32.0 + court_w + 2.0);
            for ci in 0..conf_n {
                let cx = conf_zone_x + ci as f64 * 32.0;
                if cx < x0 + 34.0 {
                    continue;
                }
                obj_open!(s);
                s.push_str(&format!(
                    "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"28\" height=\"{:.1}\" fill=\"var(--bim-accent-subtle)\" stroke=\"{}\" stroke-width=\"0.5\"/>",
                    cx, y0 + 1.0, h1 - 2.0, accent
                ));
                let cth = ((h1 - 2.0) * 0.48).min(12.0);
                let cty = y0 + 1.0 + ((h1 - 2.0) - cth) / 2.0;
                s.push_str(&format!(
                    "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"18\" height=\"{:.1}\" fill=\"var(--bim-accent-subtle)\" stroke=\"var(--bim-accent-active)\" stroke-width=\"0.4\" rx=\"0.5\"/>",
                    cx + 4.0, cty, cth
                ));
                obj_close!(s);
            }
            if size_tier == 2 {
                let crx = xr - 34.0;
                s.push_str(&format!(
                    "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"30\" height=\"{:.1}\" fill=\"var(--bim-accent-subtle)\" stroke=\"{}\" stroke-width=\"0.6\"/>",
                    crx, y0 + 1.0, h1 - 2.0, accent
                ));
                let cr_rows = ((h1 - 8.0) / 7.0) as usize;
                for row in 0..cr_rows.min(4) {
                    for col in 0..3usize {
                        let sy = y0 + 3.0 + row as f64 * 7.0;
                        if sy + 4.0 > y0 + h1 - 3.0 {
                            break;
                        }
                        s.push_str(&format!(
                            "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"7\" height=\"4\" fill=\"var(--bim-accent-subtle)\" stroke=\"var(--bim-accent-active)\" stroke-width=\"0.3\" rx=\"0.3\"/>",
                            crx + 2.0 + col as f64 * 9.0, sy
                        ));
                    }
                }
            }
            let rec_start = x0 + 2.0 + 14.0 * ocols as f64 + 3.0;
            let rec_end = conf_zone_x - 2.0;
            if rec_end - rec_start >= 8.0 {
                s.push_str(&format!(
                    "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"4\" fill=\"var(--bim-accent-subtle)\" stroke=\"var(--bim-accent-active)\" stroke-width=\"0.4\" rx=\"0.3\"/>",
                    rec_start, y0 + 2.0, (rec_end - rec_start).min(28.0)
                ));
            }
            if h2 >= 12.0 {
                let sr_w = (plan_w * 0.38).min(58.0);
                s.push_str(&format!(
                    "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"var(--bim-accent-subtle)\" stroke=\"{}\" stroke-width=\"0.5\"/>",
                    x0 + 1.0, y1, sr_w, h2 * 0.82, accent
                ));
                round_table!(s, x0 + 1.0 + sr_w / 2.0, y1 + h2 * 0.40, 8.0, 4);
                s.push_str(&format!(
                    "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"16\" height=\"{:.1}\" fill=\"var(--bim-accent-subtle)\" stroke=\"var(--bim-accent-active)\" stroke-width=\"0.5\" rx=\"0.3\"/>",
                    xr - 20.0, y1 + 1.0, (h2 * 0.65).min(h2 - 2.0)
                ));
            }
            if h3 >= 8.0 {
                obj_open!(s);
                door!(s, x0 + 4.0, y2, (h3 * 0.80).min(14.0));
                obj_close!(s);
                s.push_str(&format!(
                    "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"32\" height=\"4\" fill=\"var(--bim-accent-subtle)\" stroke=\"var(--bim-accent-active)\" stroke-width=\"0.3\" rx=\"0.3\"/>",
                    x0 + 22.0, y2 + 2.0
                ));
            }
        }

        _ => {}
    }

    s.push_str(&format!(
        "<text x=\"{:.1}\" y=\"110\" font-size=\"5.5\" fill=\"{}\" class=\"bim-plan-mono bim-plan-mono--dim\" text-anchor=\"middle\" letter-spacing=\"1.2\">CORE</text>",
        plan_center_x, accent
    ));

    s.push_str("</svg>");
    s
}

/// Deliberate "not modeled" drawing-set note for floor-scale Compositions
/// (`has_zone_data == false` — no three-zone cross-section on record) — a
/// small-scale floor outline with a hatched core inside a dashed frame,
/// labeled plainly, so an empty plan reads as an intentional drawing-set
/// convention rather than a broken/blank card. See
/// BRIEF-bim-v3-hyperscaler-redesign.md `composition_detail_treatment`.
pub fn render_floor_scale_svg() -> String {
    let mut s = String::with_capacity(900);
    s.push_str("<svg class=\"bim-kp-diagram bim-kp-diagram--floorscale\" viewBox=\"0 0 180 112\" width=\"360\" height=\"224\" xmlns=\"http://www.w3.org/2000/svg\" role=\"img\" aria-label=\"Floor-scale entry — zone layout not modeled\">");
    s.push_str("<rect width=\"180\" height=\"112\" fill=\"var(--bim-bg-surface)\"/>");
    s.push_str("<rect x=\"16\" y=\"14\" width=\"148\" height=\"78\" fill=\"none\" stroke=\"var(--bim-accent-active)\" stroke-width=\"1\" stroke-dasharray=\"4,3\"/>");
    // Hatched core, centered.
    let (cx0, cy0, cw, ch): (f64, f64, f64, f64) = (74.0, 40.0, 32.0, 26.0);
    s.push_str(&format!(
        "<rect x=\"{cx0}\" y=\"{cy0}\" width=\"{cw}\" height=\"{ch}\" fill=\"none\" stroke=\"var(--bim-accent)\" stroke-width=\"0.9\"/>"
    ));
    let mut hx: f64 = cx0 - ch;
    while hx < cx0 + cw {
        let x1 = hx.max(cx0);
        let x2 = (hx + ch).min(cx0 + cw);
        if x2 > x1 {
            s.push_str(&format!(
                "<line x1=\"{x1:.1}\" y1=\"{y1:.1}\" x2=\"{x2:.1}\" y2=\"{y2:.1}\" stroke=\"var(--bim-accent-active)\" stroke-width=\"0.5\"/>",
                y1 = cy0 + (x1 - hx),
                y2 = cy0 + (x2 - hx),
            ));
        }
        hx += 6.0;
    }
    s.push_str("<text x=\"90\" y=\"9\" font-size=\"5\" fill=\"var(--bim-fg-caption)\" class=\"bim-plan-mono\" text-anchor=\"middle\" letter-spacing=\"1.1\">FLOOR-SCALE — ZONE LAYOUT NOT MODELED</text>");
    s.push_str("<text x=\"90\" y=\"104\" font-size=\"4.5\" fill=\"var(--bim-fg-caption)\" class=\"bim-plan-mono bim-plan-mono--dim\" text-anchor=\"middle\">CORE (illustrative)</text>");
    s.push_str("</svg>");
    s
}

/// Round 6 (2026-07-10) P3: the two-ladder model diagram for the Method
/// page — the page whose entire job is explaining this model rendered zero
/// diagrams while every composition detail page rendered a real one (the
/// cohesion-audit's single highest-leverage visual finding). Two labeled
/// columns — the element ladder (Object -> Composition) and the space
/// ladder (Zone -> Key Plan -> Tile -> Floor Plate -> Building) — each
/// rung connected to the next by a solid aggregation line, with a single
/// dashed containment line bridging Composition to Key Plan. Same visual
/// grammar as the plan diagrams: navy strokes, mono labels, dashed lines
/// reserved for the one relationship that is genuinely different in kind
/// (containment, not aggregation) — never a second decorative dash style.
pub fn render_two_ladder_svg() -> String {
    let accent = "var(--bim-accent)";
    let accent_subtle = "var(--bim-accent-subtle)";
    let caption = "var(--bim-fg-caption)";
    let mut s = String::with_capacity(2400);
    s.push_str("<svg class=\"bim-method-diagram\" viewBox=\"0 0 320 210\" xmlns=\"http://www.w3.org/2000/svg\" role=\"img\" aria-label=\"The two-ladder model: element ladder (Object, Composition) and space ladder (Key Plan, Tile, Floor Plate, Building), joined by containment\">");
    s.push_str("<rect width=\"320\" height=\"210\" fill=\"var(--bim-bg-surface)\"/>");

    // Column headers.
    s.push_str(&format!(
        "<text x=\"64\" y=\"14\" font-size=\"7\" fill=\"{accent}\" class=\"bim-plan-mono\" text-anchor=\"middle\" letter-spacing=\"1.2\" font-weight=\"600\">ELEMENT LADDER</text>"
    ));
    s.push_str(&format!(
        "<text x=\"236\" y=\"14\" font-size=\"7\" fill=\"{accent}\" class=\"bim-plan-mono\" text-anchor=\"middle\" letter-spacing=\"1.2\" font-weight=\"600\">SPACE LADDER</text>"
    ));

    // Element ladder: Composition (upper), Object (lower) — box + label,
    // solid aggregation line between them.
    let elem_rungs: [(&str, f64); 2] = [("Composition", 40.0), ("Object", 168.0)];
    for (label, y) in elem_rungs {
        s.push_str(&format!(
            "<rect x=\"14\" y=\"{y:.1}\" width=\"100\" height=\"26\" rx=\"2\" fill=\"{accent_subtle}\" stroke=\"{accent}\" stroke-width=\"1\"/>\
<text x=\"64\" y=\"{ty:.1}\" font-size=\"7.5\" fill=\"{accent}\" class=\"bim-plan-mono\" text-anchor=\"middle\" font-weight=\"600\">{label}</text>",
            ty = y + 16.5
        ));
    }
    s.push_str(&format!(
        "<line x1=\"64\" y1=\"66\" x2=\"64\" y2=\"168\" stroke=\"{accent}\" stroke-width=\"1\"/>\
<path d=\"M64,66 l-3,7 l6,0 z\" fill=\"{accent}\"/>"
    ));
    s.push_str(&format!(
        "<text x=\"78\" y=\"120\" font-size=\"5\" fill=\"{caption}\" class=\"bim-plan-mono bim-plan-mono--dim\" text-anchor=\"start\">aggregates</text>"
    ));

    // Space ladder: Building, Floor Plate, Tile, Key Plan (top to bottom),
    // four rungs, solid aggregation lines between each. Round 7 (2026-07-10):
    // "Zone" was removed as a fifth rung here — Habitat/Magazine/Corridor
    // are the internal decomposition of a single Key Plan, not a separate
    // composable spatial unit with its own aggregation chain the way Tiles
    // aggregate Key Plans (a real modelling error this diagram introduced;
    // see "Key Plans and Tiles" for where Habitat/Magazine/Corridor are
    // actually explained). Rungs re-centered in the 210-height viewBox at
    // the same 42px pitch (22 rung height + 20 gap) used before.
    let space_rungs: [(&str, f64); 4] = [
        ("Building", 31.0),
        ("Floor Plate", 73.0),
        ("Tile", 115.0),
        ("Key Plan", 157.0),
    ];
    for (label, y) in space_rungs {
        s.push_str(&format!(
            "<rect x=\"186\" y=\"{y:.1}\" width=\"100\" height=\"22\" rx=\"2\" fill=\"var(--bim-bg-surface)\" stroke=\"{accent}\" stroke-width=\"1\"/>\
<text x=\"236\" y=\"{ty:.1}\" font-size=\"7\" fill=\"{accent}\" class=\"bim-plan-mono\" text-anchor=\"middle\" font-weight=\"600\">{label}</text>",
            ty = y + 14.5
        ));
    }
    for pair in space_rungs.windows(2) {
        let (y_upper_bottom, y_lower_top) = (pair[0].1 + 22.0, pair[1].1);
        s.push_str(&format!(
            "<line x1=\"236\" y1=\"{y_upper_bottom:.1}\" x2=\"236\" y2=\"{y_lower_top:.1}\" stroke=\"{accent}\" stroke-width=\"1\"/>\
<path d=\"M236,{y_upper_bottom:.1} l-3,7 l6,0 z\" fill=\"{accent}\"/>"
        ));
    }

    // Containment bridge: Composition (element ladder) -> Key Plan (space
    // ladder), dashed — the one relationship that is genuinely different
    // in kind (an Object/Composition sits INSIDE a Key Plan; it is not
    // aggregated the way a Tile aggregates Key Plans).
    // Round 7 (2026-07-10): endpoint recomputed to land on Key Plan's new
    // vertical center (157 + 22/2 = 168, was 147 when Key Plan sat at y=136
    // with Zone still occupying the rung below it).
    s.push_str(&format!(
        "<line x1=\"114\" y1=\"53\" x2=\"186\" y2=\"168\" stroke=\"{accent}\" stroke-width=\"1\" stroke-dasharray=\"4,3\"/>\
<path d=\"M186,168 l-8,-2 l1,6 z\" fill=\"{accent}\"/>"
    ));
    s.push_str(&format!(
        "<text x=\"150\" y=\"106\" font-size=\"5\" fill=\"{caption}\" class=\"bim-plan-mono bim-plan-mono--dim\" text-anchor=\"middle\" transform=\"rotate(35 150 106)\">contained in</text>"
    ));

    s.push_str("</svg>");
    s
}
