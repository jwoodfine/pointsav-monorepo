// SPDX-License-Identifier: FSL-1.1-ALv2
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Markdown rendering.
//!
//! Bodies are CommonMark (via comrak) with one platform extension: `[[slug]]`
//! and `[[slug|label]]` wikilinks resolve to internal `/wiki/{slug}` anchors.
//! Section headings (h2/h3) are extracted for the table of contents.

use std::sync::OnceLock;

use comrak::options::Plugins;
use comrak::plugins::syntect::{SyntectAdapter, SyntectAdapterBuilder};
use comrak::{markdown_to_html_with_plugins, Options};
use syntect::highlighting::ThemeSet;
use syntect::html::{css_for_theme_with_class_style, ClassStyle};

/// Syntax highlighter, built once. Class-based output (no inline colours) so the
/// palette can switch with the page theme via `syntax_css()`.
fn highlighter() -> &'static SyntectAdapter {
    static ADAPTER: OnceLock<SyntectAdapter> = OnceLock::new();
    // No theme set → the adapter emits CSS classes (not inline colours),
    // which `syntax_css()` themes for light and dark.
    ADAPTER.get_or_init(|| SyntectAdapterBuilder::new().build())
}

/// Token-colour CSS for code blocks: a light theme by default and a dark theme
/// scoped under `html[data-theme="dark"]`, so code follows the page like every
/// modern docs site. Backgrounds are stripped — the panel background is a token
/// (`--k-code-block-bg`), light or dark per mode. Generated once.
pub fn syntax_css() -> &'static str {
    static CSS: OnceLock<String> = OnceLock::new();
    CSS.get_or_init(|| {
        let ts = ThemeSet::load_defaults();
        let light = ts
            .themes
            .get("InspiredGitHub")
            .and_then(|t| css_for_theme_with_class_style(t, ClassStyle::Spaced).ok())
            .unwrap_or_default();
        let dark = ts
            .themes
            .get("base16-ocean.dark")
            .and_then(|t| css_for_theme_with_class_style(t, ClassStyle::Spaced).ok())
            .unwrap_or_default();
        format!(
            "/* light */\n{}\n/* dark */\n{}\n",
            transform_theme_css(&light, None),
            transform_theme_css(&dark, Some("html[data-theme=\"dark\"] ")),
        )
    })
}

/// Strip background rules and, optionally, prefix every selector with a scope so
/// a theme only applies in that mode. Keeps foreground token colours only.
fn transform_theme_css(css: &str, scope: Option<&str>) -> String {
    // Drop the leading header comment syntect emits.
    let css = match (css.find("/*"), css.find("*/")) {
        (Some(0), Some(end)) => &css[end + 2..],
        _ => css,
    };
    let no_bg: String = css
        .lines()
        .filter(|l| !l.trim_start().starts_with("background-color"))
        .collect::<Vec<_>>()
        .join("\n");
    let Some(prefix) = scope else { return no_bg };
    let mut out = String::new();
    for rule in no_bg.split_inclusive('}') {
        match rule.find('{') {
            Some(b) if !rule[..b].trim().is_empty() => {
                let (sel, body) = rule.split_at(b);
                let scoped = sel
                    .split(',')
                    .map(|s| format!("{prefix}{}", s.trim()))
                    .filter(|s| !s.trim().is_empty())
                    .collect::<Vec<_>>()
                    .join(", ");
                out.push_str(&scoped);
                out.push(' ');
                out.push_str(body);
            }
            _ => out.push_str(rule),
        }
    }
    out
}

/// A rendered document body plus its heading outline.
#[derive(Debug, Clone)]
pub struct Rendered {
    pub html: String,
    pub headings: Vec<Heading>,
}

/// One section heading for the table of contents.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Heading {
    pub level: u8,
    pub id: String,
    pub text: String,
}

/// Render a Markdown body to HTML, resolving wikilinks and collecting headings.
pub fn render(body_md: &str) -> Rendered {
    let with_links = resolve_wikilinks(body_md);

    let mut opts = Options::default();
    opts.extension.strikethrough = true;
    opts.extension.table = true;
    opts.extension.autolink = true;
    opts.extension.tasklist = true;
    opts.extension.footnotes = true;
    // `header_ids` was renamed `header_id_prefix` (comrak 0.52) — the old
    // field is deprecated and, confirmed live, no longer actually emits an
    // `id` attribute at all (every heading came back with `id=""` until
    // this was caught by a test failure). Empty prefix matches the
    // pre-rename behavior (no prefix on generated ids).
    opts.extension.header_id_prefix = Some(String::new());
    opts.render.r#unsafe = true; // content is trusted (Git-authored, reviewed)

    let mut plugins = Plugins::default();
    plugins.render.codefence_syntax_highlighter = Some(highlighter());

    let html = markdown_to_html_with_plugins(&with_links, &opts, &plugins);
    // Extract headings from comrak's own rendered output (not by re-deriving
    // ids from raw markdown) — comrak's Anchorizer disambiguates duplicate
    // headings with numeric suffixes and strips inline markup differently
    // than a naive slugify, so a separately-computed id silently diverges
    // from the id actually present in the HTML the TOC links point at.
    let headings = extract_headings_from_html(&html);
    Rendered { html, headings }
}

/// Render a full document. When the frontmatter carries a `references:` list, it
/// appends synthesized footnote definitions (`[^id]: text <url>`) so the body's
/// `[^id]` markers resolve into a rendered reference list instead of dead text.
pub fn render_doc(doc: &super::frontmatter::ParsedDoc) -> Rendered {
    if doc.frontmatter.references.is_empty() {
        return render(&doc.body_md);
    }
    let mut body = doc.body_md.clone();
    body.push_str("\n\n");
    for r in &doc.frontmatter.references {
        body.push_str(&format!("[^{}]: {}", r.id, r.text));
        if let Some(u) = r.url.as_deref().filter(|u| !u.is_empty()) {
            body.push_str(&format!(" <{u}>"));
        }
        body.push('\n');
    }
    render(&body)
}

/// Replace `[[slug]]` / `[[slug|label]]` with Markdown links to `/wiki/slug`.
/// A leading `#` in the target (e.g. `[[#section]]`) is treated as a same-page
/// anchor. Escaped `\[[` is left untouched.
fn resolve_wikilinks(md: &str) -> String {
    let bytes = md.as_bytes();
    let mut out = String::with_capacity(md.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'[' && i + 1 < bytes.len() && bytes[i + 1] == b'[' {
            // Guard against escaped `\[[`.
            let escaped = i > 0 && bytes[i - 1] == b'\\';
            if !escaped {
                if let Some(close) = md[i + 2..].find("]]") {
                    let inner = &md[i + 2..i + 2 + close];
                    let (target, label) = match inner.split_once('|') {
                        Some((t, l)) => (t.trim(), l.trim()),
                        None => (inner.trim(), inner.trim()),
                    };
                    let href = if let Some(anchor) = target.strip_prefix('#') {
                        format!("#{}", slugify(anchor))
                    } else {
                        format!("/wiki/{}", target)
                    };
                    out.push('[');
                    out.push_str(label);
                    out.push_str("](");
                    // CommonMark link destinations may not contain an
                    // unescaped space unless wrapped in angle brackets — a
                    // human-readable target like `[[Zero Container
                    // Inference]]` would otherwise render as literal text,
                    // not a link.
                    if href.contains(' ') {
                        out.push('<');
                        out.push_str(&href);
                        out.push('>');
                    } else {
                        out.push_str(&href);
                    }
                    out.push(')');
                    i = i + 2 + close + 2;
                    continue;
                }
            }
        }
        // Default: copy this byte through, respecting UTF-8 boundaries.
        let ch_len = utf8_len(bytes[i]);
        out.push_str(&md[i..i + ch_len]);
        i += ch_len;
    }
    out
}

fn utf8_len(b: u8) -> usize {
    match b {
        0x00..=0x7f => 1,
        0xc0..=0xdf => 2,
        0xe0..=0xef => 3,
        _ => 4,
    }
}

/// Extract level-2/3 headings directly from comrak's rendered HTML — the
/// `id` on each `<h2>`/`<h3>` is whatever comrak's `header_ids` Anchorizer
/// actually assigned (including its numeric-suffix disambiguation for
/// duplicate headings), and `text` is the tag's inner content with any
/// nested markup (inline code, links) stripped and HTML entities decoded.
fn extract_headings_from_html(html: &str) -> Vec<Heading> {
    let mut headings = Vec::new();
    let mut rest = html;
    loop {
        let h2 = rest.find("<h2");
        let h3 = rest.find("<h3");
        let (level, pos) = match (h2, h3) {
            (Some(p2), Some(p3)) if p3 < p2 => (3u8, p3),
            (Some(p2), _) => (2u8, p2),
            (None, Some(p3)) => (3u8, p3),
            (None, None) => break,
        };
        let tag_onward = &rest[pos..];
        let Some(tag_end) = tag_onward.find('>') else {
            break;
        };
        let after_open = &tag_onward[tag_end + 1..];
        let close = format!("</h{level}>");
        let Some(close_pos) = after_open.find(&close) else {
            break;
        };
        let inner = &after_open[..close_pos];
        // comrak's header_id_prefix places the id on a nested, empty
        // `<a id="…" class="anchor" …></a>` inside the heading (immediately
        // before the visible text), not as an attribute on `<h2>`/`<h3>`
        // itself — confirmed live: `<h2><a href="#x" ... id="x"></a>Text</h2>`.
        let id = extract_attr(inner, "id").unwrap_or_default();
        let text = decode_entities(&strip_tags(inner));
        if !text.trim().is_empty() {
            headings.push(Heading {
                level,
                id,
                text: text.trim().to_string(),
            });
        }
        rest = &after_open[close_pos + close.len()..];
    }
    headings
}

/// Extract `attr="value"` from a raw opening-tag string (e.g. `<h2 id="x">`
/// without the trailing `>`).
fn extract_attr(tag: &str, attr: &str) -> Option<String> {
    let needle = format!("{attr}=\"");
    let start = tag.find(&needle)? + needle.len();
    let end = start + tag[start..].find('"')?;
    Some(tag[start..end].to_string())
}

/// Strip HTML tags, keeping only text content (used to recover a heading's
/// plain display text from its rendered, possibly inline-formatted, HTML).
fn strip_tags(html: &str) -> String {
    let mut out = String::with_capacity(html.len());
    let mut in_tag = false;
    for ch in html.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => out.push(ch),
            _ => {}
        }
    }
    out
}

/// Decode the small set of HTML entities comrak actually emits in text nodes.
fn decode_entities(s: &str) -> String {
    s.replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
}

/// Lowercase ASCII slug: alphanumerics kept, runs of other chars → single `-`.
pub fn slugify(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut prev_dash = false;
    for ch in s.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
            prev_dash = false;
        } else if !prev_dash {
            out.push('-');
            prev_dash = true;
        }
    }
    out.trim_matches('-').to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_basic_markdown() {
        let r = render("# Title\n\nSome **bold** text.\n");
        assert!(r.html.contains("<strong>bold</strong>"));
    }

    #[test]
    fn render_doc_turns_references_into_footnotes() {
        use super::super::frontmatter::{Frontmatter, ParsedDoc, Reference};
        let doc = ParsedDoc {
            frontmatter: Frontmatter {
                references: vec![Reference {
                    id: "1".into(),
                    text: "Yjs library.".into(),
                    url: Some("https://yjs.dev".into()),
                }],
                ..Default::default()
            },
            body_md: "This cites a source[^1].\n".into(),
        };
        let r = render_doc(&doc);
        assert!(
            r.html.contains("footnotes"),
            "footnotes section: {}",
            r.html
        );
        assert!(r.html.contains("Yjs library."));
        assert!(
            !r.html.contains("[^1]"),
            "marker should be resolved, not literal"
        );
    }

    #[test]
    fn resolves_wikilinks_with_and_without_label() {
        let r = render("See [[zero-container-inference]] and [[yoyo-compute|GPU compute]].\n");
        assert!(r.html.contains(r#"href="/wiki/zero-container-inference""#));
        assert!(r.html.contains(r#"href="/wiki/yoyo-compute""#));
        assert!(r.html.contains(">GPU compute</a>"));
    }

    #[test]
    fn wikilink_target_with_a_space_still_renders_as_a_real_link() {
        // Without angle-bracket wrapping, CommonMark refuses an unescaped
        // space in a link destination and renders the literal source text
        // instead of an <a> tag — this must not regress. comrak percent-
        // encodes the space in the emitted href (correct, valid HTML — a
        // real browser link), so the assertion checks for a genuine anchor
        // tag rather than an exact (unencoded) href string.
        let r = render("See [[Zero Container Inference]].\n");
        assert!(
            r.html
                .contains("<a href=\"/wiki/Zero%20Container%20Inference\""),
            "got: {}",
            r.html
        );
        assert!(r.html.contains(">Zero Container Inference</a>"));
        assert!(!r
            .html
            .contains("[Zero Container Inference](/wiki/Zero Container Inference)"));
    }

    #[test]
    fn extracts_h2_h3_headings_only() {
        let r = render("# H1\n\n## Why no containers\n\n### Detail\n\n#### Too deep\n");
        let ids: Vec<_> = r.headings.iter().map(|h| h.id.as_str()).collect();
        assert_eq!(ids, vec!["why-no-containers", "detail"]);
        assert_eq!(r.headings[0].level, 2);
        assert_eq!(r.headings[1].level, 3);
    }

    #[test]
    fn headings_inside_code_fence_are_ignored() {
        let r = render("## Real\n\n```\n## Not a heading\n```\n");
        assert_eq!(r.headings.len(), 1);
        assert_eq!(r.headings[0].id, "real");
    }

    #[test]
    fn duplicate_headings_get_comraks_actual_disambiguated_ids() {
        // The bug this guards: a naive local slugify produces "overview" for
        // BOTH headings, so a TOC built from it links #overview twice (the
        // second entry jumps to the first section). Extracting from
        // comrak's own output picks up its Anchorizer's real -1 suffix, so
        // the TOC ids the article() template renders always match what's
        // actually clickable in the body.
        let r = render("## Overview\n\ntext\n\n## Overview\n\nmore\n");
        assert_eq!(r.headings.len(), 2);
        assert_ne!(
            r.headings[0].id, r.headings[1].id,
            "duplicate headings must not collide"
        );
        assert!(r.html.contains(&format!(r#"id="{}""#, r.headings[0].id)));
        assert!(r.html.contains(&format!(r#"id="{}""#, r.headings[1].id)));
    }

    #[test]
    fn heading_with_inline_markup_gets_matching_id_and_plain_text() {
        // A naive slugify over the raw markdown line would see the literal
        // `[...](...)`  syntax; the real id must come from the *rendered*
        // text, and the TOC's display text should be plain (no markup).
        let r = render("## See [the spec](/x)\n");
        assert_eq!(r.headings.len(), 1);
        assert_eq!(r.headings[0].text, "See the spec");
        assert!(
            r.html.contains(&format!(r#"id="{}""#, r.headings[0].id)),
            "TOC id must match a real id in the rendered HTML"
        );
    }

    #[test]
    fn anchor_wikilink_stays_same_page() {
        let r = render("Jump to [[#Cold start|cold start]].\n");
        assert!(r.html.contains(r##"href="#cold-start""##));
    }
}
