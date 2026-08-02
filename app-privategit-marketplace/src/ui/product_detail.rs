// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Product detail page — `GET /software/:product_id` (S136, requested by
//! Command 2026-06-30). One page per catalog entry: BETA badge, tier badge,
//! platform table, curl install command, version, SHA256, and an optional
//! GUIDE link.
//!
//! Two fields the catalog does not carry are handled explicitly rather than
//! fabricated:
//!
//! - **Platform table** ships as a single row today (`i.platform` display
//!   label + the existing `linux-x86_64` download-slug convention already
//!   used by the paid download flow, `main.rs`'s `order_download`). Real
//!   per-product multi-platform data would need a schema change
//!   (`platforms: Vec<PlatformArtifact>` on `Installer`) — tracked as a
//!   follow-up in `NEXT.md`, not blocking this page.
//! - **SHA256** is fetched client-side from `app-privategit-source-2`'s
//!   `GET /releases/:product/:version/MANIFEST` endpoint (same source
//!   `ui::catalog`'s doc comment already names as the correct home for this
//!   data). This avoids adding a new server-side HTTP client dependency and a
//!   new marketplace→source-2 network coupling at render time. Degrades to a
//!   visible "verify via MANIFEST" link if the fetch fails or JS is disabled.

use crate::ui::Lang;
use crate::{Installer, LicenseTier};
use maud::{html, Markup, PreEscaped};

/// Duplicated from `ui::catalog` rather than made `pub(crate)` — matches this
/// crate's established preference for small per-page duplication over
/// cross-module coupling (see `checkout.rs`/`order.rs`, which each carry
/// their own `_style()` rather than sharing one).
fn install_command(source_base_url: &str, id: &str) -> String {
    let base = source_base_url.trim_end_matches('/');
    format!("curl -fsSL {base}/{id}/install.sh | bash")
}

fn tier_badge(tier: LicenseTier) -> Markup {
    html! { span."sw-cat-badge sw-cat-badge--tier" { (tier.label()) } }
}

/// Vanilla-JS SHA256 fetch — populates `#sw-pd-sha-value` from the per-version
/// MANIFEST endpoint. The visible "verify via MANIFEST" link stays in the
/// markup regardless, so the page degrades honestly if this fails or JS is
/// disabled, rather than fabricating a value.
fn sha_fetch_script(manifest_url: &str) -> Markup {
    let url_json = serde_json::to_string(manifest_url).unwrap_or_else(|_| "\"\"".to_string());
    let js = format!(
        r#"(function(){{
fetch({url_json}).then(function(r){{return r.json();}}).then(function(m){{
  var el=document.getElementById('sw-pd-sha-value');
  if(el&&m&&m.sha256){{el.textContent=m.sha256;}}
}}).catch(function(){{}});
}})();"#
    );
    html! { script { (PreEscaped(js)) } }
}

fn product_detail_style() -> Markup {
    let css = r#".sw-pd-wrap{max-width:760px;margin:0 auto;padding:40px 24px 64px;box-sizing:border-box;}
.sw-pd-card{border:1px solid #e4e7ec;border-radius:10px;padding:28px;background:#fff;box-shadow:0 1px 2px rgba(16,24,40,.04);}
.sw-pd-id{font-family:ui-monospace,SFMono-Regular,Menlo,monospace;font-size:11px;color:#667085;letter-spacing:.02em;}
.sw-pd-name{font-family:Georgia,"Times New Roman",serif;font-size:28px;line-height:1.2;color:#111827;margin:6px 0 10px;}
.sw-pd-desc{font-size:14px;line-height:1.6;color:#475467;margin:0 0 20px;max-width:64ch;}
.sw-pd-badges{display:flex;flex-wrap:wrap;gap:6px;margin:0 0 28px;}
.sw-pd-h2{font-size:13px;letter-spacing:.1em;text-transform:uppercase;color:#234ed8;margin:0 0 12px;padding-bottom:6px;border-bottom:1px solid #e4e7ec;}
.sw-pd-table{width:100%;border-collapse:collapse;margin:0 0 28px;font-size:13.5px;}
.sw-pd-table th{text-align:left;color:#667085;font-size:11px;letter-spacing:.06em;text-transform:uppercase;padding:0 0 8px;}
.sw-pd-table td{padding:8px 0;border-top:1px solid #e4e7ec;color:#344054;}
.sw-pd-table a{color:#234ed8;font-weight:600;text-decoration:none;}
.sw-pd-table a:hover{color:#173ab1;}
.sw-pd-version{font-family:ui-monospace,SFMono-Regular,Menlo,monospace;font-size:13px;color:#344054;margin:0 0 10px;}
.sw-pd-sha{font-size:12.5px;color:#475467;line-height:1.6;margin:0 0 28px;word-break:break-all;}
.sw-pd-sha__label{font-weight:600;color:#344054;}
.sw-pd-sha__value{font-family:ui-monospace,SFMono-Regular,Menlo,monospace;}
.sw-pd-sha__fallback{color:#234ed8;}
.sw-pd-back{margin-top:28px;font-size:13px;}
.sw-pd-back a{color:#234ed8;text-decoration:none;}
.sw-pd-back a:hover{color:#173ab1;}
.sw-cat-badge{font-size:11px;font-weight:600;letter-spacing:.02em;padding:3px 8px;border-radius:999px;background:#f2f4f7;color:#344054;white-space:nowrap;}
.sw-cat-badge--free{background:#ecfdf3;color:#067647;}
.sw-cat-badge--ver{background:#eef3ff;color:#234ed8;font-family:ui-monospace,SFMono-Regular,Menlo,monospace;}
.sw-cat-badge--tier{background:#eef3ff;color:#234ed8;}
.sw-cat-cmd{display:flex;align-items:stretch;background:#0e1117;border-radius:6px;overflow:hidden;}
.sw-cat-cmd__text{flex:1;min-width:0;color:#e6edf3;font-family:ui-monospace,SFMono-Regular,Menlo,monospace;font-size:12px;line-height:1.4;padding:9px 11px;overflow-x:auto;white-space:nowrap;}
.sw-cat-cmd__copy{border:0;background:#234ed8;color:#fff;font-size:11px;font-weight:600;padding:0 14px;cursor:pointer;letter-spacing:.04em;flex:0 0 auto;}
.sw-cat-cmd__copy:hover{background:#173ab1;}"#;
    html! { style { (PreEscaped(css)) } }
}

/// Static-label translations for this page — installer name/description stay
/// English regardless of `lang` (no translation source exists for catalog data,
/// same convention as `catalog.rs`/`checkout.rs`).
struct ProductDetailLabels {
    platform_h2: &'static str,
    platform_col: &'static str,
    download_col: &'static str,
    install_h2: &'static str,
    copy_label: &'static str,
    copy_aria: &'static str,
    version_h2: &'static str,
    sha_label: &'static str,
    sha_verifying: &'static str,
    sha_fallback: &'static str,
    guide_h2: &'static str,
    guide_link: &'static str,
    back_link: &'static str,
}

impl ProductDetailLabels {
    fn for_lang(lang: Lang) -> Self {
        match lang {
            Lang::En => Self {
                platform_h2: "Platform",
                platform_col: "Platform",
                download_col: "Download",
                install_h2: "Install",
                copy_label: "Copy",
                copy_aria: "Copy install command to clipboard",
                version_h2: "Version & checksum",
                sha_label: "SHA256: ",
                sha_verifying: "verifying\u{2026}",
                sha_fallback: "verify via MANIFEST",
                guide_h2: "Guide",
                guide_link: "Operational guide",
                back_link: "\u{2190} All products",
            },
            Lang::Es => Self {
                platform_h2: "Plataforma",
                platform_col: "Plataforma",
                download_col: "Descarga",
                install_h2: "Instalaci\u{f3}n",
                copy_label: "Copiar",
                copy_aria: "Copiar comando de instalaci\u{f3}n al portapapeles",
                version_h2: "Versi\u{f3}n y suma de verificaci\u{f3}n",
                sha_label: "SHA256: ",
                sha_verifying: "verificando\u{2026}",
                sha_fallback: "verificar via MANIFEST",
                guide_h2: "Gu\u{ed}a",
                guide_link: "Gu\u{ed}a operativa",
                back_link: "\u{2190} Todos los productos",
            },
        }
    }
}

pub fn product_detail_markup(i: &Installer, source_base_url: &str, lang: Lang) -> Markup {
    let base = source_base_url.trim_end_matches('/');
    let manifest_url = format!("{base}/{}/{}/MANIFEST", i.id, i.edition);
    let download_url = format!("{base}/{}/{}/linux-x86_64", i.id, i.edition);
    let command = install_command(source_base_url, &i.id);
    let l = ProductDetailLabels::for_lang(lang);
    let all_products_href = lang.localize("/software");

    html! {
        (product_detail_style())
        div."sw-pd-wrap" {
            article."sw-pd-card" {
                span."sw-pd-id" { (i.id) }
                h1."sw-pd-name" { (i.name) }
                p."sw-pd-desc" { (i.description) }
                div."sw-pd-badges" {
                    @if i.price_usdc == 0 {
                        span."sw-cat-badge sw-cat-badge--free" { "BETA \u{00b7} free" }
                    }
                    (tier_badge(i.license_tier))
                    span."sw-cat-badge sw-cat-badge--ver" { "v" (i.edition) }
                }

                h2."sw-pd-h2" { (l.platform_h2) }
                table."sw-pd-table" {
                    thead { tr { th { (l.platform_col) } th { (l.download_col) } } }
                    tbody {
                        tr {
                            td { (i.platform) }
                            td { a href=(download_url) { "linux-x86_64" } }
                        }
                    }
                }

                h2."sw-pd-h2" { (l.install_h2) }
                div."sw-cat-install" {
                    div."sw-cat-cmd" {
                        code."sw-cat-cmd__text" { (command) }
                        button."sw-cat-cmd__copy" type="button"
                            data-sw-clip=(command) data-sw-label=(l.copy_label)
                            aria-label=(l.copy_aria) { (l.copy_label) }
                    }
                }

                h2."sw-pd-h2" { (l.version_h2) }
                p."sw-pd-version" { "v" (i.edition) }
                p."sw-pd-sha" {
                    span."sw-pd-sha__label" { (l.sha_label) }
                    span."sw-pd-sha__value" #"sw-pd-sha-value" { (l.sha_verifying) }
                    " \u{2014} "
                    a."sw-pd-sha__fallback" href=(manifest_url) { (l.sha_fallback) }
                }

                @if let Some(url) = &i.guide_url {
                    h2."sw-pd-h2" { (l.guide_h2) }
                    p { a href=(url) { (l.guide_link) } }
                }

                p."sw-pd-back" { a href=(all_products_href) { (l.back_link) } }
            }
        }
        (sha_fetch_script(&manifest_url))
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────
//
// Pure rendering function — no server, no filesystem, no network.
#[cfg(test)]
mod tests {
    use super::*;

    const BASE: &str = "https://example.invalid/releases";

    fn fixture(price_usdc: u64, guide_url: Option<String>) -> Installer {
        Installer {
            id: "os-mediakit".into(),
            name: "MediaKit OS".into(),
            description: "Sovereign media workstation image.".into(),
            edition: "1.2.0".into(),
            platform: "linux-x86_64".into(),
            size_mb: 812,
            path: "os-mediakit/1.2.0/installer.run".into(),
            license_tier: LicenseTier::Fsl,
            price_usdc,
            fsl_conversion_date: None,
            guide_url,
        }
    }

    #[test]
    fn renders_badge_platform_install_and_version() {
        let html = product_detail_markup(&fixture(0, None), BASE, Lang::En).into_string();
        assert!(html.contains("os-mediakit"));
        assert!(html.contains("MediaKit OS"));
        assert!(html.contains("BETA \u{00b7} free"));
        assert!(html.contains("FSL"));
        assert!(html.contains("v1.2.0"));
        assert!(html.contains("linux-x86_64"));
        assert!(html
            .contains("curl -fsSL https://example.invalid/releases/os-mediakit/install.sh | bash"));
        assert!(html.contains("https://example.invalid/releases/os-mediakit/1.2.0/MANIFEST"));
        assert!(html.contains("https://example.invalid/releases/os-mediakit/1.2.0/linux-x86_64"));
    }

    #[test]
    fn paid_product_omits_beta_badge() {
        let html = product_detail_markup(&fixture(1_000_000, None), BASE, Lang::En).into_string();
        assert!(!html.contains("BETA \u{00b7} free"));
    }

    #[test]
    fn guide_link_renders_only_when_present() {
        let without = product_detail_markup(&fixture(0, None), BASE, Lang::En).into_string();
        assert!(!without.contains("Operational guide"));

        let with = product_detail_markup(
            &fixture(0, Some("https://docs.example.invalid/guide".into())),
            BASE,
            Lang::En,
        )
        .into_string();
        assert!(with.contains("Operational guide"));
        assert!(with.contains("https://docs.example.invalid/guide"));
    }

    #[test]
    fn sha_placeholder_and_fetch_script_present() {
        let html = product_detail_markup(&fixture(0, None), BASE, Lang::En).into_string();
        assert!(html.contains("sw-pd-sha-value"));
        assert!(html.contains("verify via MANIFEST"));
        assert!(html.contains("fetch("));
    }

    // ── /es/* extension (Spanish localization follow-up, BRIEF-software-spanish-localization.md) ──

    #[test]
    fn spanish_translates_static_labels_not_product_data() {
        let html = product_detail_markup(
            &fixture(0, Some("https://docs.example.invalid/guide".into())),
            BASE,
            Lang::Es,
        )
        .into_string();
        // Static labels translated.
        assert!(html.contains("Plataforma"));
        assert!(html.contains("Descarga"));
        assert!(html.contains("Instalaci\u{f3}n"));
        assert!(html.contains("Versi\u{f3}n y suma de verificaci\u{f3}n"));
        assert!(html.contains("verificando\u{2026}"));
        assert!(html.contains("verificar via MANIFEST"));
        assert!(html.contains("Gu\u{ed}a operativa"));
        assert!(html.contains("Todos los productos"));
        // Product name/description are NOT translated — no translation source exists.
        assert!(html.contains("MediaKit OS"));
        assert!(html.contains("Sovereign media workstation image."));
    }

    #[test]
    fn spanish_back_link_localizes_to_es_software() {
        let html = product_detail_markup(&fixture(0, None), BASE, Lang::Es).into_string();
        assert!(html.contains("href=\"/es/software\""));
    }

    #[test]
    fn english_back_link_is_unprefixed() {
        let html = product_detail_markup(&fixture(0, None), BASE, Lang::En).into_string();
        assert!(html.contains("href=\"/software\""));
    }
}
