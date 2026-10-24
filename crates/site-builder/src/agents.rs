//! Agent formats are derived from the same rendered body as the HTML page.
use crate::layouts::base::PageMeta;
use maud::{html, Markup, PreEscaped};
use serde_json::json;
use std::{fs, path::Path, process::Command};

pub const ORIGIN: &str = "https://www.drakestapleton.com";

pub fn markdown_path(route: &str) -> String {
    if route == "/" {
        "/index.md".to_string()
    } else {
        format!("{}.md", route.trim_end_matches('/'))
    }
}

/// Source modification time, never the current build time. Full history in CI
/// keeps hourly rebuilds from making unchanged editorial content look new.
pub fn source_modified(route: &str) -> Option<String> {
    let slug = route.trim_start_matches('/').trim_start_matches("works/");
    let page = match slug {
        "" => "home",
        "works" => "works_index",
        "what-i-learned" => "what_i_learned",
        s if s.starts_with("symphony") => "symphony",
        s => s,
    };
    let output = Command::new("git")
        .args([
            "log",
            "-1",
            "--format=%cI",
            "--",
            "site-metadata.json",
            "crates/site-builder/src/main.rs",
            "crates/site-builder/src/agents.rs",
            "crates/site-builder/src/layouts",
            "crates/site-builder/src/components",
            &format!("crates/site-builder/src/pages/{page}.rs"),
            &format!("content/works/{slug}.md"),
            &format!("content/{slug}.md"),
        ])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let value = String::from_utf8(output.stdout).ok()?.trim().to_string();
    if value.is_empty() {
        None
    } else {
        Some(value)
    }
}

pub fn structured_data(meta: &PageMeta) -> String {
    let mut page = json!({
        "@type": "WebPage",
        "@id": format!("{}#webpage", meta.canonical_url),
        "url": meta.canonical_url,
        "name": meta.title,
        "headline": meta.title,
        "description": meta.description,
        "isPartOf": {"@id": format!("{ORIGIN}/#website")},
        "about": {"@id": format!("{ORIGIN}/#person")}
    });
    if let Some(date) = source_modified(meta.current_route) {
        page["dateModified"] = json!(date);
    }
    let mut graph = vec![
        json!({"@type":"Person", "@id":format!("{ORIGIN}/#person"),
            "name":"Drake Stapleton", "url":format!("{ORIGIN}/")}),
        json!({"@type":"WebSite", "@id":format!("{ORIGIN}/#website"),
            "name":"Drake Stapleton", "url":format!("{ORIGIN}/"),
            "publisher":{"@id":format!("{ORIGIN}/#person")}}),
    ];
    let id = format!("{}#breadcrumb", meta.canonical_url);
    let mut items = vec![json!({"@type":"ListItem", "position":1,
        "name":"Home", "item":format!("{ORIGIN}/")})];
    if meta.current_route != "/" {
        items.push(json!({"@type":"ListItem", "position":2,
            "name":meta.title, "item":meta.canonical_url}));
    }
    page["breadcrumb"] = json!({"@id":id});
    graph.push(json!({"@type":"BreadcrumbList", "@id":id, "itemListElement":items}));
    graph.push(page);
    // A script element is raw text: JSON escaping alone doesn't stop </script>.
    json!({"@context":"https://schema.org", "@graph":graph})
        .to_string()
        .replace('&', "\\u0026")
        .replace('<', "\\u003c")
        .replace('>', "\\u003e")
}

fn markdown_document(meta: &PageMeta, body: &str) -> String {
    let date = source_modified(meta.current_route)
        .map(|d| format!("dateModified: {}\nlast_updated: {}\n", json!(d), json!(d)))
        .unwrap_or_default();
    // Fragment links refer back to HTML, whose explicit IDs remain stable.
    let body = body.replace("href=\"#", &format!("href=\"{}#", meta.canonical_url));
    let body = body
        .replace("href=\"/", &format!("href=\"{ORIGIN}/"))
        .replace("src=\"/", &format!("src=\"{ORIGIN}/"));
    let markdown = htmd::HtmlToMarkdown::builder()
        .skip_tags(vec!["script", "style"])
        .build()
        .convert(&body)
        .expect("Convert page body to Markdown");
    format!(
        "---\ntitle: {}\ndescription: {}\nurl: {}\n{date}---\n\n{}\n\n## Sitemap\n\n- [Site map]({ORIGIN}/sitemap.md)\n- [Agent guide]({ORIGIN}/AGENTS.md)\n- [HTML version]({})\n",
        json!(meta.title), json!(meta.description), json!(meta.canonical_url),
        markdown.trim(), meta.canonical_url,
    )
}

pub fn write_markdown(dist: &Path, meta: &PageMeta, body: &Markup) {
    if meta.current_route == "/404" {
        return;
    }
    let dest = dist.join(markdown_path(meta.current_route).trim_start_matches('/'));
    fs::create_dir_all(dest.parent().unwrap()).expect("Create Markdown directory");
    fs::write(dest, markdown_document(meta, &body.0)).expect("Write Markdown mirror");
}

pub fn write_indexes(
    dist: &Path,
    routes: &[(&str, &str)],
    metadata: &std::collections::HashMap<String, crate::SiteMetadataEntry>,
) {
    let mut sitemap = String::from("# Drake Stapleton: site map\n\n## Public pages\n\n");
    let mut full = String::from("# Drake Stapleton: full public text\n\nGenerated from the same page bodies as HTML. Live evidence is a build-time snapshot; follow source links for current status.\n\n");
    for (route, _) in routes {
        let title = metadata
            .get(*route)
            .map(|m| m.title.as_str())
            .unwrap_or(route);
        let md = markdown_path(route);
        sitemap.push_str(&format!(
            "- [{title}]({ORIGIN}{route}): [Markdown]({ORIGIN}{md})\n"
        ));
        let document = fs::read_to_string(dist.join(md.trim_start_matches('/')))
            .unwrap_or_else(|_| panic!("Missing Markdown for sitemap route {route}"));
        full.push_str(&format!(
            "\n---\n\n## {title}\n\nSource: {ORIGIN}{route}\n\n{document}"
        ));
    }
    fs::write(dist.join("sitemap.md"), sitemap).expect("Write sitemap.md");
    fs::write(dist.join("llms-full.txt"), full).expect("Write llms-full.txt");
}

pub fn render_glossary() -> Markup {
    let source = include_str!("../../../content/glossary.md");
    let mut body = String::new();
    pulldown_cmark::html::push_html(&mut body, pulldown_cmark::Parser::new(source));
    html! { article class="reader" { div class="reader-content" { (PreEscaped(body)) } } }
}

pub fn verify_page(dist: &Path, route: &str, html: &str) {
    if route == "/404" {
        return;
    }
    let mirror = markdown_path(route);
    assert!(
        html.contains(&format!(
            "rel=\"alternate\" type=\"text/markdown\" href=\"{mirror}\""
        )),
        "Missing Markdown discovery on {route}"
    );
    assert!(
        html.contains("rel=\"describedby\" href=\"/llms.txt\""),
        "Missing llms.txt discovery on {route}"
    );
    let markdown = fs::read_to_string(dist.join(mirror.trim_start_matches('/')))
        .unwrap_or_else(|_| panic!("Missing Markdown mirror for {route}"));
    assert!(
        markdown.starts_with("---\ntitle:"),
        "Missing frontmatter on {route}"
    );
    assert!(
        markdown.contains("## Sitemap"),
        "Missing content map on {route}"
    );
    let (_, after) = html
        .split_once("<script type=\"application/ld+json\">")
        .unwrap_or_else(|| panic!("Missing JSON-LD on {route}"));
    let json = after
        .split_once("</script>")
        .expect("JSON-LD closing tag")
        .0;
    let data: serde_json::Value = serde_json::from_str(json).expect("Valid JSON-LD");
    assert_eq!(data["@context"], "https://schema.org");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn meta<'a>(title: &'a str) -> PageMeta<'a> {
        PageMeta {
            title,
            description: "A & B",
            canonical_url: "https://www.drakestapleton.com/atlas",
            current_route: "/atlas",
        }
    }

    #[test]
    fn schema_is_valid_json_and_cannot_close_script() {
        let data = structured_data(&meta("</script><script>alert(1)</script>"));
        assert!(!data.contains('<'));
        let value: serde_json::Value = serde_json::from_str(&data).unwrap();
        assert_eq!(value["@graph"][0]["@type"], "Person");
        assert_eq!(
            value["@graph"][3]["name"],
            "</script><script>alert(1)</script>"
        );
    }

    #[test]
    fn markdown_preserves_text_headings_links_and_table_content() {
        let doc = markdown_document(&meta("Atlas"),
            "<h1>Atlas &amp; evidence</h1><h2>Results</h2><p>Measured, not promised.</p><a href=\"/evidence\">Proof</a><a href=\"#detail\">Detail</a><table><tr><th>Check</th><th>Result</th></tr><tr><td>Build</td><td>pass</td></tr></table>");
        for text in [
            "Atlas & evidence",
            "Results",
            "Measured, not promised.",
            "[Proof](https://www.drakestapleton.com/evidence)",
            "[Detail](https://www.drakestapleton.com/atlas#detail)",
            "Build",
            "pass",
            "## Sitemap",
        ] {
            assert!(doc.contains(text), "Missing {text}: {doc}");
        }
        assert!(doc.starts_with("---\ntitle: \"Atlas\"\n"));
        assert_eq!(markdown_path("/"), "/index.md");
        assert_eq!(markdown_path("/symphony/workflow"), "/symphony/workflow.md");
    }
}
