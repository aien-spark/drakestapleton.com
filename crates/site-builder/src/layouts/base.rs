use maud::{html, Markup, PreEscaped, DOCTYPE};

pub struct PageMeta<'a> {
    pub title: &'a str,
    pub description: &'a str,
    pub canonical_url: &'a str,
    pub current_route: &'a str,
}

pub fn render_base_layout(meta: &PageMeta, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                meta name="color-scheme" content="dark";
                meta name="theme-color" content="#08090c";
                title { (meta.title) }
                meta name="description" content=(meta.description);
                link rel="canonical" href=(meta.canonical_url);
                @if meta.current_route != "/404" {
                    meta name="robots" content="index, follow, max-snippet:-1, max-image-preview:large";
                    link rel="describedby" href="/llms.txt";
                    link rel="alternate" type="text/markdown" href=(crate::agents::markdown_path(meta.current_route));
                    script type="application/ld+json" { (PreEscaped(crate::agents::structured_data(meta))) }
                } @else {
                    meta name="robots" content="noindex";
                }
                link rel="icon" type="image/svg+xml" href="/favicon.svg";

                // Open Graph / Twitter Meta
                meta property="og:title" content=(meta.title);
                meta property="og:description" content=(meta.description);
                meta property="og:url" content=(meta.canonical_url);
                meta property="og:type" content="website";
                meta property="og:image" content="https://www.drakestapleton.com/og.png";
                meta name="twitter:card" content="summary_large_image";
                meta name="twitter:title" content=(meta.title);
                meta name="twitter:description" content=(meta.description);
                meta name="twitter:image" content="https://www.drakestapleton.com/og.png";

                // CSS Stylesheets (system fonts only; the page makes no request outside this site)
                link rel="stylesheet" href="/assets/style.css";
                link rel="stylesheet" href="/assets/reader.css";
                link rel="stylesheet" href="/assets/engine.css";
                link rel="stylesheet" href="/assets/universe.css";
                link rel="stylesheet" href="/assets/turing.css";

                // HTMX for zero-build hypermedia interactivity
                script defer src="/js/htmx.min.js" {}
                script defer src="/js/universe.js" {}
                script defer src="/js/progress.js" {}
                (PreEscaped(r#"<script>
                    document.addEventListener('DOMContentLoaded', () => {
                        console.log('[AIEN Engine] Pure Rust Maud engine online. Zero hydration delay.');
                        var navToggle = document.querySelector('.nav-toggle');
                        var siteNav = document.querySelector('.site-nav');
                        if (navToggle && siteNav) {
                            var setOpen = function(open) {
                                if (open) { siteNav.classList.add('nav-open'); }
                                else { siteNav.classList.remove('nav-open'); }
                                navToggle.setAttribute('aria-expanded', open ? 'true' : 'false');
                                navToggle.setAttribute('aria-label', open ? 'Close menu' : 'Open menu');
                            };
                            navToggle.addEventListener('click', function() {
                                setOpen(!siteNav.classList.contains('nav-open'));
                            });
                            var links = siteNav.querySelectorAll('.links a');
                            for (var i = 0; i < links.length; i++) {
                                links[i].addEventListener('click', function() { setOpen(false); });
                            }
                            document.addEventListener('keydown', function(e) {
                                if (e.key === 'Escape') { setOpen(false); }
                            });
                        }
                    });
                </script>"#))
            }
            body {
                div id="root" {
                    div class="site-shell" {
                        div class="read-progress" aria-hidden="true" {}
                        (crate::components::nav::render_nav(meta.current_route))
                        main id="main-content" {
                            (content)
                        }
                        (crate::components::footer::render_footer())
                    }
                }
            }
        }
    }
}
