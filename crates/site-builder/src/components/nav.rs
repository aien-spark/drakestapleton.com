use maud::{html, Markup};

const LINKS: &[(&str, &str, bool)] = &[
    ("/", "Identity", true),
    ("/path", "Career", true),
    ("/philosophy", "Philosophy", true),
    ("/software", "Orchestration", true),
    ("/works", "Works", false),
    ("/evidence", "Audit", true),
    ("/atlas", "Atlas", true),
    ("/aegis", "AEGIS", true),
    ("/aien", "AIEN", true),
    ("/research", "Research", true),
    ("/scholar", "Scholar", false),
    ("/interest", "Conversation", true),
    ("/donate", "Donate", true),
];

/// GitHub's mark (octicon `mark-github`, 16x16), drawn inline so the header
/// makes no request outside this site.
const GITHUB_MARK: &str = "M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27s1.36.09 2 .27c1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.01 8.01 0 0 0 16 8c0-4.42-3.58-8-8-8z";

pub fn render_nav(current_route: &str) -> Markup {
    html! {
        header class="site-nav" {
            div class="site-nav-inner" {
                a href="/" class="brand" {
                    span class="brand-name" { "Drake Stapleton" }
                    span class="brand-line" { "Freedom Fighter · AI Architect & Operator · AIENOS.com" }
                }
                a class="nav-github" href="https://github.com/drakestapleton" rel="me noopener" target="_blank" aria-label="Drake Stapleton on GitHub (opens in a new tab)" {
                    svg viewBox="0 0 16 16" width="18" height="18" fill="currentColor" aria-hidden="true" focusable="false" {
                        path d=(GITHUB_MARK) {}
                    }
                    span class="nav-github-label" { "GitHub" }
                }
                button type="button" class="nav-toggle" aria-expanded="false" aria-controls="primary-nav" aria-label="Open menu" {
                    span class="nav-toggle-bar" aria-hidden="true" {}
                    span class="nav-toggle-bar" aria-hidden="true" {}
                    span class="nav-toggle-bar" aria-hidden="true" {}
                }
                nav class="links" id="primary-nav" aria-label="Primary" {
                    @for (path, label, exact) in LINKS {
                        @let is_active = if *exact {
                            current_route == *path
                        } else {
                            current_route.starts_with(path)
                        };

                        a href=(path) class=(if is_active { "active" } else { "" }) {
                            (label)
                        }
                    }
                }
            }
        }
    }
}
