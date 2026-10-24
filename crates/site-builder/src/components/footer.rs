use maud::{html, Markup};
use crate::components::voicemark::render_his_words;

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
                    a href="https://x.com/aienscore" { "X" }
                }
            }
        }
    }
}
