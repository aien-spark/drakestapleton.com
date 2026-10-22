mod components;
mod layouts;
mod pages;

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::Instant;

use layouts::base::{render_base_layout, PageMeta};
use pages::atlas::render_atlas_page;
use pages::donate::render_donate_page;
use pages::evidence::render_evidence_page;
use pages::home::render_home_page;
use pages::interest::render_interest_page;
use pages::not_found::render_not_found_page;
use pages::path::render_path_page;
use pages::philosophy::render_philosophy_page;
use pages::scholar::render_scholar_page;
use pages::reader::{parse_work_markdown, render_reader_page, WorkFrontmatter};
use pages::software::render_software_page;
use pages::symphony::render_symphony_page;
use pages::what_i_learned::render_what_i_learned_page;
use pages::works_index::render_works_index;

#[derive(serde::Deserialize)]
struct SiteMetadataEntry {
    title: String,
    description: String,
}

fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    if !src.exists() {
        return Ok(());
    }
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if ty.is_dir() {
            copy_dir_all(&from, &to)?;
        } else {
            fs::copy(&from, &to)?;
        }
    }
    Ok(())
}

fn generate_sitemap_and_robots(dist: &Path, routes: &[(&str, &str)]) {
    let mut sitemap = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n");
    for (route, priority) in routes {
        sitemap.push_str(&format!(
            "  <url><loc>https://www.drakestapleton.com{}</loc><priority>{}</priority></url>\n",
            route, priority
        ));
    }
    sitemap.push_str("</urlset>\n");

    let sitemap_path = dist.join("sitemap.xml");
    fs::write(&sitemap_path, sitemap).expect("Failed to write sitemap.xml");

    let robots = "User-agent: *\nAllow: /\nSitemap: https://www.drakestapleton.com/sitemap.xml\nLLMs: https://www.drakestapleton.com/llms.txt\n";
    let robots_path = dist.join("robots.txt");
    fs::write(&robots_path, robots).expect("Failed to write robots.txt");

    println!("  [GEN] Emitted native sitemap.xml and robots.txt in: {}", dist.display());
}

fn render_and_write_page(
    dist: &Path,
    route: &str,
    rel_path: &str,
    metadata: &HashMap<String, SiteMetadataEntry>,
    markup_fn: impl FnOnce() -> maud::Markup,
) {
    let meta = metadata.get(route);
    let title = meta.map(|m| m.title.as_str()).unwrap_or("Drake Stapleton");
    let desc = meta.map(|m| m.description.as_str()).unwrap_or("");
    let canonical = format!("https://www.drakestapleton.com{}", route);

    let doc_meta = PageMeta {
        title,
        description: desc,
        canonical_url: &canonical,
        current_route: route,
    };

    let page_content = markup_fn();
    let full_doc = render_base_layout(&doc_meta, page_content);
    let file_dest = dist.join(rel_path);
    if let Some(parent) = file_dest.parent() {
        fs::create_dir_all(parent).expect("Failed to create parent directory");
    }
    fs::write(&file_dest, full_doc.into_string()).expect("Failed to write page");
    println!("  [PAGE] Emitted {} ({})", route, rel_path);
}

fn build_pure_rust_site(dist: &Path) {
    println!("  [BUILD] Compiling pure Rust Maud static site into: {}", dist.display());
    // The live AIEN record from aienos.com; None renders the pages with a plain "could not be read" line.
    let facts = components::facts::fetch_facts();
    // Create directories
    fs::create_dir_all(dist.join("assets")).expect("Failed to create assets dir");
    fs::create_dir_all(dist.join("js")).expect("Failed to create js dir");
    fs::create_dir_all(dist.join("images")).expect("Failed to create images dir");

    // Copy public static assets
    let public_dir = Path::new("public");
    if public_dir.exists() {
        for entry in fs::read_dir(public_dir).expect("Failed to read public dir") {
            let entry = entry.expect("Valid entry");
            let path = entry.path();
            let name = entry.file_name();
            let dest = dist.join(&name);
            if path.is_dir() {
                copy_dir_all(&path, &dest).expect("Failed to copy public subfolder");
            } else {
                fs::copy(&path, &dest).expect("Failed to copy public file");
            }
        }
        println!("  [ASSETS] Synced public/ assets into {}", dist.display());
    }

    // Load metadata mappings
    let metadata_str = fs::read_to_string("site-metadata.json")
        .expect("Failed to read site-metadata.json");
    let metadata: HashMap<String, SiteMetadataEntry> = serde_json::from_str(&metadata_str)
        .expect("Failed to parse site-metadata.json");

    // 1. Build Standard Static Portfolio Pages
    render_and_write_page(dist, "/", "index.html", &metadata, || render_home_page(facts.as_ref()));
    render_and_write_page(dist, "/404", "404.html", &metadata, render_not_found_page);
    render_and_write_page(dist, "/atlas", "atlas/index.html", &metadata, render_atlas_page);
    render_and_write_page(dist, "/evidence", "evidence/index.html", &metadata, || render_evidence_page(facts.as_ref()));
    render_and_write_page(dist, "/software", "software/index.html", &metadata, render_software_page);
    render_and_write_page(dist, "/path", "path/index.html", &metadata, render_path_page);
    render_and_write_page(dist, "/what-i-learned", "what-i-learned/index.html", &metadata, render_what_i_learned_page);
    render_and_write_page(dist, "/philosophy", "philosophy/index.html", &metadata, render_philosophy_page);
    render_and_write_page(dist, "/scholar", "scholar/index.html", &metadata, render_scholar_page);
    render_and_write_page(dist, "/interest", "interest/index.html", &metadata, render_interest_page);
    render_and_write_page(dist, "/donate", "donate/index.html", &metadata, render_donate_page);
    render_and_write_page(dist, "/symphony", "symphony/index.html", &metadata, || render_symphony_page(None));
    render_and_write_page(dist, "/symphony/first", "symphony/first/index.html", &metadata, || render_symphony_page(Some("first")));
    render_and_write_page(dist, "/symphony/workflow", "symphony/workflow/index.html", &metadata, || render_symphony_page(Some("workflow")));
    render_and_write_page(dist, "/symphony/map", "symphony/map/index.html", &metadata, || render_symphony_page(Some("map")));

    // 2. Scan & Build Markdown Works (/content/works/*.md)
    let works_dir = Path::new("content/works");
    let mut collected_works: Vec<WorkFrontmatter> = Vec::new();
    let public_routes: Vec<(String, &'static str)> = vec![
        ("/".to_string(), "1.0"),
        ("/works".to_string(), "0.9"),
        ("/research".to_string(), "0.9"),
        ("/aien".to_string(), "0.8"),
        ("/aegis".to_string(), "0.8"),
        ("/atlas".to_string(), "0.8"),
        ("/software".to_string(), "0.8"),
        ("/evidence".to_string(), "0.8"),
        ("/scholar".to_string(), "0.7"),
        ("/path".to_string(), "0.7"),
        ("/what-i-learned".to_string(), "0.7"),
        ("/philosophy".to_string(), "0.7"),
        ("/symphony".to_string(), "0.7"),
        ("/interest".to_string(), "0.7"),
        ("/donate".to_string(), "0.7"),
    ];

    if works_dir.exists() {
        for entry in fs::read_dir(works_dir).expect("Failed to read content/works") {
            let entry = entry.expect("Valid entry");
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("md") {
                let raw_content = fs::read_to_string(&path)
                    .unwrap_or_else(|_| panic!("Failed to read {}", path.display()));
                let (opt_meta, html_body) = parse_work_markdown(&raw_content);
                if let Some(work_meta) = opt_meta {
                    // The AIEN page ends with the live record read from aienos.com.
                    let html_body = if work_meta.slug == "aien" {
                        format!("{html_body}{}", components::facts::render_live_block(facts.as_ref()).into_string())
                    } else {
                        html_body
                    };
                    let route = format!("/{}", work_meta.slug);
                    let site_meta = metadata.get(&route);
                    let title = site_meta.map(|m| m.title.as_str()).unwrap_or(&work_meta.title);
                    let desc = site_meta.map(|m| m.description.as_str()).unwrap_or(&work_meta.summary);
                    let canonical = format!("https://www.drakestapleton.com/{}", work_meta.slug);

                    let doc_meta = PageMeta {
                        title,
                        description: desc,
                        canonical_url: &canonical,
                        current_route: &route,
                    };

                    let page_html = render_reader_page(&work_meta, &html_body);
                    let markup = render_base_layout(&doc_meta, page_html);
                    let markup_string = markup.into_string();

                    // Emit to /{slug}/index.html
                    let slug_dir = dist.join(&work_meta.slug);
                    fs::create_dir_all(&slug_dir).expect("Failed to create slug directory");
                    fs::write(slug_dir.join("index.html"), &markup_string)
                        .expect("Failed to write slug index.html");

                    // Emit to /works/{slug}/index.html with works metadata
                    let works_route = format!("/works/{}", work_meta.slug);
                    let works_site_meta = metadata.get(&works_route);
                    let works_title = works_site_meta.map(|m| m.title.as_str()).unwrap_or(title);
                    let works_desc = works_site_meta.map(|m| m.description.as_str()).unwrap_or(desc);
                    let works_canonical = format!("https://www.drakestapleton.com/works/{}", work_meta.slug);

                    let works_doc_meta = PageMeta {
                        title: works_title,
                        description: works_desc,
                        canonical_url: &works_canonical,
                        current_route: &works_route,
                    };
                    let works_page_html = render_reader_page(&work_meta, &html_body);
                    let works_markup = render_base_layout(&works_doc_meta, works_page_html);
                    let works_slug_dir = dist.join("works").join(&work_meta.slug);
                    fs::create_dir_all(&works_slug_dir).expect("Failed to create works slug directory");
                    fs::write(works_slug_dir.join("index.html"), works_markup.into_string())
                        .expect("Failed to write works slug index.html");

                    println!("  [WORK] Emitted {} -> /{}/index.html & /works/{}/index.html", 
                        path.file_name().unwrap().to_string_lossy(), 
                        work_meta.slug, 
                        work_meta.slug
                    );
                    collected_works.push(work_meta);
                }
            }
        }
    }

    // 3. Build Works Index (/works)
    let works_dir_out = dist.join("works");
    fs::create_dir_all(&works_dir_out).expect("Failed to create dist/works");
    let works_meta = metadata.get("/works");
    let works_title = works_meta.map(|m| m.title.as_str()).unwrap_or("Works | Drake Stapleton");
    let works_desc = works_meta.map(|m| m.description.as_str()).unwrap_or("A public catalog of Drake Stapleton's shipped systems.");

    let works_doc_meta = PageMeta {
        title: works_title,
        description: works_desc,
        canonical_url: "https://www.drakestapleton.com/works",
        current_route: "/works",
    };
    let works_markup = render_base_layout(&works_doc_meta, render_works_index(&collected_works));
    fs::write(works_dir_out.join("index.html"), works_markup.into_string())
        .expect("Failed to write works/index.html");
    println!("  [PAGE] Emitted /works (works/index.html)");

    // Convert routes for sitemap generator
    let sitemap_routes: Vec<(&str, &str)> = public_routes
        .iter()
        .map(|(r, p)| (r.as_str(), *p))
        .collect();
    generate_sitemap_and_robots(dist, &sitemap_routes);
}

fn verify_site(dist: &Path) {
    let metadata_str = fs::read_to_string("site-metadata.json")
        .expect("Failed to read site-metadata.json");
    let metadata: HashMap<String, SiteMetadataEntry> = serde_json::from_str(&metadata_str)
        .expect("Failed to parse site-metadata.json");

    let required_routes = vec![
        ("index.html", "/"),
        ("aien/index.html", "/aien"),
        ("research/index.html", "/research"),
        ("atlas/index.html", "/atlas"),
        ("aegis/index.html", "/aegis"),
        ("software/index.html", "/software"),
        ("path/index.html", "/path"),
        ("evidence/index.html", "/evidence"),
        ("what-i-learned/index.html", "/what-i-learned"),
        ("philosophy/index.html", "/philosophy"),
        ("scholar/index.html", "/scholar"),
        ("interest/index.html", "/interest"),
        ("donate/index.html", "/donate"),
        ("symphony/index.html", "/symphony"),
        ("symphony/first/index.html", "/symphony/first"),
        ("symphony/workflow/index.html", "/symphony/workflow"),
        ("symphony/map/index.html", "/symphony/map"),
        ("works/index.html", "/works"),
        ("works/atlas-symphony/index.html", "/works/atlas-symphony"),
        ("works/atlas-harness/index.html", "/works/atlas-harness"),
        ("works/mobdev-panel-arena/index.html", "/works/mobdev-panel-arena"),
        ("works/defiantmob/index.html", "/works/defiantmob"),
        ("works/beltbrain/index.html", "/works/beltbrain"),
        ("works/dhg-hive/index.html", "/works/dhg-hive"),
        ("works/foundations-2025/index.html", "/works/foundations-2025"),
        ("works/aien/index.html", "/works/aien"),
        ("cudaplusplus/index.html", "/cudaplusplus"),
        ("works/cudaplusplus/index.html", "/works/cudaplusplus"),
        ("works/aegis/index.html", "/works/aegis"),
        ("works/research/index.html", "/works/research"),
        ("404.html", "/404"),
    ];

    let mut total_bytes = 0usize;
    let mut verified_pages = 0;

    for (rel_path, route) in &required_routes {
        let file_path = dist.join(rel_path);
        assert!(
            file_path.exists(),
            "Required static route missing: {}",
            file_path.display()
        );

        let content = fs::read_to_string(&file_path)
            .unwrap_or_else(|_| panic!("Failed to read {}", file_path.display()));

        total_bytes += content.len();

        // Check for pre-rendered root content (eliminates SPA clunkiness)
        assert!(
            content.contains("<div id=\"root\">") && content.contains("<div class=\"site-shell\">"),
            "Route {} missing pre-rendered HTML DOM in #root",
            route
        );

        // Check metadata matches site-metadata.json if available
        if let Some(meta) = metadata.get(*route) {
            let escaped_title = meta.title.replace('&', "&amp;");
            assert!(
                content.contains(&escaped_title),
                "Route {} missing required title: {}",
                route,
                escaped_title
            );
        }

        // Verify unslop standards: zero em dashes and zero en dashes
        assert!(
            !content.contains('\u{2014}'),
            "Forbidden em dash detected in {}",
            rel_path
        );
        assert!(
            !content.contains('\u{2013}'),
            "Forbidden en dash detected in {}",
            rel_path
        );

        verified_pages += 1;
        println!("  [OK] {} ({} bytes) -> {}", rel_path, content.len(), route);
    }

    // Verify sitemap and robots
    let sitemap = dist.join("sitemap.xml");
    assert!(sitemap.exists(), "Missing sitemap.xml");
    let robots = dist.join("robots.txt");
    assert!(robots.exists(), "Missing robots.txt");

    println!("------------------------------------------------------------");
    println!("  Verified {} pre-rendered static routes ({} KB total)", verified_pages, total_bytes / 1024);
    println!("  Integrity & Unslop Invariants: PASS");
}

fn main() {
    let start = Instant::now();
    let first_arg = std::env::args().nth(1).unwrap_or_else(|| "build".to_string());
    let (mode, target_dir) = if first_arg == "build" || first_arg == "verify" {
        (first_arg, std::env::args().nth(2).unwrap_or_else(|| "dist-rust".to_string()))
    } else {
        ("verify".to_string(), first_arg)
    };

    let dist = Path::new(&target_dir);

    println!("============================================================");
    println!("  AIEN Native Static Site Builder & Integrity Verifier      ");
    println!("  Engine: Pure Rust & Maud | Target: {}", dist.display());
    println!("  Mode: {}", mode);
    println!("============================================================");

    if mode == "build" {
        build_pure_rust_site(dist);
        verify_site(dist);
    } else {
        if !dist.exists() {
            eprintln!("Error: Target directory '{}' does not exist.", dist.display());
            std::process::exit(1);
        }
        verify_site(dist);
    }

    let elapsed = start.elapsed();
    println!("  Build and Verification Completed in: {:.2?}", elapsed);
    println!("============================================================");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unslop_punctuation_invariants() {
        let sample = "AIEN: Sovereign Cognitive Architecture on dedicated hardware.";
        assert!(!sample.contains('\u{2014}'), "Must not contain em dash");
        assert!(!sample.contains('\u{2013}'), "Must not contain en dash");
    }

    #[test]
    fn test_metadata_deserialization() {
        let sample_json = r#"{"/aien":{"title":"AIEN Sovereign","description":"Sovereign cognitive runtime."}}"#;
        let map: HashMap<String, SiteMetadataEntry> = serde_json::from_str(sample_json).unwrap();
        assert_eq!(map.get("/aien").unwrap().title, "AIEN Sovereign");
    }

    #[test]
    fn test_markdown_reader_parsing() {
        let sample_md = "---\ntitle: \"Test Title\"\nslug: \"test\"\ndate: \"September 2026\"\nsummary: \"Test summary\"\n---\n\n## Subhead\n\nContent here.\n";
        let (meta, html) = parse_work_markdown(sample_md);
        assert!(meta.is_some());
        let m = meta.unwrap();
        assert_eq!(m.title, "Test Title");
        assert_eq!(m.slug, "test");
        assert!(html.contains("<h2>Subhead</h2>"));
    }
}
