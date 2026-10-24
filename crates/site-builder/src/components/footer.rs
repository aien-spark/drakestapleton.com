use maud::{html, Markup};
use crate::components::voicemark::render_his_words;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn footer_links_both_projects_and_the_current_x() {
        let s = render_footer().into_string();
        assert!(s.contains("href=\"https://www.aienos.com/\""), "AIENOS link missing");
        assert!(s.contains("href=\"https://interplane.io/\""), "Interplane link missing");
        assert!(s.contains("AIENOS, the operating system"), "AIENOS identity missing");
        assert!(s.contains("Interplane, the model and runtime protocol"), "Interplane identity missing");
        assert!(s.contains("https://x.com/mdrakestapleton"), "current X handle missing");
        assert!(!s.contains("aienscore"), "stale X handle present");
    }
}

pub fn render_footer() -> Markup {
    html! {
        footer class="site-footer" {
            div class="site-footer-inner" {
                div class="site-footer-identity" {
                    p class="site-footer-name" { "Drake Stapleton" }
                    p class="site-footer-line" { "Freedom Fighter. AI Architect. Operator." }
                    p class="site-footer-line" { "Springfield, Missouri." }
                    p class="site-footer-line" {
                        (render_his_words("His own words, spoken aloud.", false))
                        " Everything else was drafted with AI and kept only with his approval."
                    }
                }
                nav class="site-footer-links" aria-label="Explore the site" {
                    a href="/" { "Life" }
                    a href="/atlas" { "Atlas" }
                    a href="/aegis" { "AEGIS" }
                    a href="/interest" { "Conversation" }
                    a href="/glossary" { "Glossary" }
                    a href="https://x.com/mdrakestapleton" rel="me" { "X" }
                }
                nav class="site-footer-links" aria-label="Projects" {
                    span class="site-footer-line" { "Projects:" }
                    a href="https://www.aienos.com/" aria-label="AIENOS, the operating system" { "AIENOS" }
                    a href="https://interplane.io/" aria-label="Interplane, the model and runtime protocol" { "Interplane" }
                }
            }
        }
    }
}
