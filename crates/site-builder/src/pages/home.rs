use maud::{html, Markup, PreEscaped};
use crate::components::facts::{render_live_block, Facts};

#[derive(Clone, Copy, PartialEq)]
enum Realm {
    Heart,
    Life,
    Machine,
}

impl Realm {
    fn class(self) -> &'static str {
        match self {
            Realm::Heart => "heart",
            Realm::Life => "life",
            Realm::Machine => "machine",
        }
    }
}

struct World {
    id: &'static str,
    realm: Realm,
    name: &'static str,
    tag: &'static str,
    x: f32,
    y: f32,
    size: u32,
    glyph: &'static str,
    facts: &'static [&'static str],
    quote: Option<&'static str>,
    photo: Option<(&'static str, &'static str)>,
    link: (&'static str, &'static str),
}

// The tour order is the order of this list: family first, then the climb from
// the farm to 3M, then the machine. Previous and Next links follow it.
const WORLDS: [World; 18] = [
    World {
        id: "home", realm: Realm::Heart, name: "Home", x: 36.0, y: 24.0, size: 92,
        tag: "My mom, my family, and the people this is all for.",
        glyph: "M12 21 C4 14 3 9 6 6 C9 3 12 6 12 8 C12 6 15 3 18 6 C21 9 20 14 12 21Z",
        facts: &[
            "<b>Mom went back to school in her forties</b> and crossed her graduation stage eight days before I crossed mine.",
            "May 2016: we celebrated graduation week together.",
        ],
        quote: None,
        photo: Some(("/images/drake-mom-graduation.webp", "Drake and his mother holding a certificate after the Western Kentucky University honors ceremony")),
        link: ("/path", "Read my story"),
    },
    World {
        id: "farm", realm: Realm::Life, name: "Farm", x: 8.0, y: 76.0, size: 58,
        tag: "A humble farm home, where I learned that useful work has dignity.",
        glyph: "M3 11 L12 4 L21 11 V20 H3 Z M9 20 V14 H15 V20",
        facts: &[
            "Practical work mattered and every person doing it was visible.",
            "The lesson I carried out the door: help before asking, and carry every task I ask someone else to do.",
        ],
        quote: None, photo: None,
        link: ("/path", "The full climb"),
    },
    World {
        id: "taco-bell", realm: Realm::Life, name: "Taco Bell", x: 19.0, y: 56.0, size: 58,
        tag: "From the line to shift leader. My first rule of leadership.",
        glyph: "M6 17 C6 10 8 6 12 6 C16 6 18 10 18 17 Z M4 17 H20 M10 20 H14",
        facts: &[
            "I served my crew, became the fastest worker I could be, and took on every task I asked of them.",
            "Trust is earned in the work, and it carries farther than any title.",
        ],
        quote: Some("Remember, Michael: at the end of the day, when you clock out and go home, it's just tacos. (Gina, my Taco Bell mentor)"),
        photo: None,
        link: ("/path", "The full climb"),
    },
    World {
        id: "valspar", realm: Realm::Life, name: "Valspar lab", x: 28.0, y: 78.0, size: 56,
        tag: "The eager little lab rat who wanted every sample perfect.",
        glyph: "M9 3 H15 M10 3 V10 L5 19 C4 21 5 21 7 21 H17 C19 21 20 21 19 19 L14 10 V3",
        facts: &[
            "An internship in production paints, color matching, and sample preparation, mentored by a chemist.",
            "I learned that the lab and the factory are one chain: a good sample has to become something people can make correctly, again and again.",
        ],
        quote: None, photo: None,
        link: ("/path", "The full climb"),
    },
    World {
        id: "wku", realm: Realm::Life, name: "Western Kentucky", x: 38.0, y: 56.0, size: 66,
        tag: "Honors graduate, then an inorganic chemist asking original questions.",
        glyph: "M2 9 L12 4 L22 9 L12 14 Z M6 11 V16 C8 18 16 18 18 16 V11",
        facts: &[
            "<b>Graduated with honors</b> and a 3.2 GPA.",
            "<b>Master's thesis, 2018</b>, under Dr. Lawrence J. Hill: platinum deposited onto semiconductor nanorods.",
            "<b>Peer-reviewed paper, 2019</b>, Beilstein Journal of Nanotechnology.",
        ],
        quote: None, photo: None,
        link: ("/scholar", "Read the papers"),
    },
    World {
        id: "soudal", realm: Realm::Life, name: "Soudal", x: 47.0, y: 77.0, size: 58,
        tag: "Silicone formulation, UL certification, and a trip to Belgium.",
        glyph: "M12 3 A9 9 0 1 0 12 21 A9 9 0 1 0 12 3 M3 12 H21 M12 3 C8 8 8 16 12 21 C16 16 16 8 12 3",
        facts: &[
            "I formulated and scaled silicone products and supported quality systems and UL certification.",
            "I worked with Soudal's headquarters in Belgium, and learned that operator knowledge and technical knowledge are worth more when someone connects them.",
        ],
        quote: None, photo: None,
        link: ("/path", "The full climb"),
    },
    World {
        id: "gold-eagle", realm: Realm::Life, name: "Gold Eagle", x: 56.0, y: 55.0, size: 62,
        tag: "R&D chemist for 303 Products. I explained the chemistry on camera.",
        glyph: "M12 3 C12 3 5 11 5 15 A7 7 0 0 0 19 15 C19 11 12 3 12 3Z",
        facts: &[
            "Developed and commercialized formulations across the 303 Graphene line.",
            "Explained the science myself in product videos, on camera.",
        ],
        quote: None, photo: None,
        link: ("/path", "Watch the videos"),
    },
    World {
        id: "3m", realm: Realm::Life, name: "3M", x: 65.0, y: 76.0, size: 64,
        tag: "Manufacturing engineering, then product engineering.",
        glyph: "M12 8 A4 4 0 1 0 12 16 A4 4 0 1 0 12 8 M12 2 V5 M12 19 V22 M2 12 H5 M19 12 H22 M5 5 L7 7 M17 17 L19 19 M19 5 L17 7 M5 19 L7 17",
        facts: &[
            "<b>$1 million in verified cost reductions</b> from PPU improvements, led through change control, workflow mapping, line balancing, and standard work.",
            "Today I am the product engineer who works on both sides: the chemistry of the product and the process behind making it.",
        ],
        quote: None, photo: None,
        link: ("/path", "The full climb"),
    },
    World {
        id: "aien", realm: Realm::Machine, name: "AIEN", x: 82.0, y: 34.0, size: 88,
        tag: "A whole computer, built from the ground up on one machine I own.",
        glyph: "M7 7 H17 V17 H7 Z M10 10 H14 V14 H10Z M9 3 V7 M15 3 V7 M9 17 V21 M15 17 V21 M3 9 H7 M3 15 H7 M17 9 H21 M17 15 H21",
        facts: &[
            "Its own operating system, its own language and GPU engine, and its own host runtime.",
            "Everything is built and judged on a single NVIDIA DGX Spark, in the open, under the AGPL.",
            "Since 23 September 2026 the organization has recorded about 3,900 commits.",
        ],
        quote: None, photo: None,
        link: ("/aien", "Explore AIEN"),
    },
    World {
        id: "aienos", realm: Realm::Machine, name: "AIENOS", x: 70.0, y: 15.0, size: 60,
        tag: "An operating system written from scratch in C, with no Linux underneath.",
        glyph: "M3 5 H21 V19 H3 Z M7 10 L10 12 L7 14 M12 15 H17",
        facts: &[
            "About 40,000 lines of kernel code: interrupts, timers, multiple cores, drivers, networking, and an encrypted store.",
            "<b>All 24 emulator gates pass.</b> It has not yet booted on the real machine, and I say so.",
        ],
        quote: None, photo: None,
        link: ("/evidence", "See the receipts"),
    },
    World {
        id: "omega", realm: Realm::Machine, name: "Omega", x: 91.0, y: 14.0, size: 60,
        tag: "A compiler and GPU engine that writes machine code itself.",
        glyph: "M4 19 H9 C6 17 5 14 5 11 A7 7 0 1 1 19 11 C19 14 18 17 15 19 H20",
        facts: &[
            "About 355,000 lines of C: a lexer, parser, type checker, and an encoder that emits AArch64 machine code.",
            "A hardware qualification ladder on the GB10 that reached a conditional pass at its last rung.",
            "No CUDA toolkit anywhere in the plan.",
            "<b>Its GPU half is now its own open source project, CUDA++</b>: everything CUDA gives you plus the motor, with no NVIDIA software above the kernel driver. <a href=\"/cudaplusplus\">How we built it</a>.",
        ],
        quote: None, photo: None,
        link: ("/research", "Read the research"),
    },
    World {
        id: "core", realm: Realm::Machine, name: "Sovereign core", x: 93.0, y: 52.0, size: 62,
        tag: "The runtime that serves a large model on our own kernels.",
        glyph: "M12 3 L21 8 L12 13 L3 8 Z M3 12 L12 17 L21 12 M3 16 L12 21 L21 16",
        facts: &[
            "<b>About 285,000 lines of Rust</b> in 57 crates and 34 programs, with more than 3,000 tests.",
            "Paged memory for a model's working context, a scheduler, and a loopback web door that speaks the same language as the big cloud APIs.",
        ],
        quote: None, photo: None,
        link: ("/aien", "How it fits together"),
    },
    World {
        id: "interplane", realm: Realm::Machine, name: "INTERPLANE", x: 80.0, y: 62.0, size: 58,
        tag: "A security wall around what an AI is allowed to do.",
        glyph: "M3 15 C7 7 17 7 21 15 M3 15 V19 M21 15 V19 M12 9 V19",
        facts: &[
            "An authority boundary between a model's decision and its action, so a tool call has to be allowed before it runs.",
            "395 tests, and every verdict is tied to a request and response fingerprint.",
        ],
        quote: None, photo: None,
        link: ("/aien", "How it fits together"),
    },
    World {
        id: "aegis", realm: Realm::Machine, name: "AEGIS", x: 68.0, y: 40.0, size: 62,
        tag: "A shield that watches the door and keeps the evidence.",
        glyph: "M12 3 L20 6 V12 C20 17 16 20 12 21 C8 20 4 17 4 12 V6 Z",
        facts: &[
            "Reads the security signals a network already produces, contains an intrusion inside the authorized estate, and preserves a signed trail.",
            "<b>The first adapter passed 59 of 62 checks</b> on 20 August 2026.",
        ],
        quote: None, photo: None,
        link: ("/aegis", "Explore AEGIS"),
    },
    World {
        id: "atlas", realm: Realm::Machine, name: "Atlas", x: 55.0, y: 24.0, size: 72,
        tag: "My AI partner, with a memory, values, and an authority limit.",
        glyph: "",
        facts: &[
            "Atlas is built on the idea that a model should be an accountable partner, not a stateless chat box.",
            "The monkey is Atlas's face. Yes, he is wearing an AIEN headband.",
        ],
        quote: None,
        photo: Some(("/images/atlas-avatar.webp", "Atlas, a cosmic monkey with a glowing AIEN headband")),
        link: ("/atlas", "Meet Atlas"),
    },
    World {
        id: "symphony", realm: Realm::Machine, name: "Symphony", x: 78.0, y: 88.0, size: 58,
        tag: "How I build software with AI agents without trusting them blindly.",
        glyph: "M3 12 H5 M7 7 V17 M11 4 V20 M15 8 V16 M19 10 V14 M21 12 H23",
        facts: &[
            "Every claim is treated as wrong until proven, the test is written before the fix, and everything is verified twice.",
            "The models change. The Symphony stays.",
        ],
        quote: None, photo: None,
        link: ("/symphony", "See how it works"),
    },
    World {
        id: "evidence", realm: Realm::Machine, name: "Receipts", x: 92.0, y: 82.0, size: 58,
        tag: "A number goes on this site only when it links to its record.",
        glyph: "M7 3 H17 L20 6 V21 H7 Z M10 9 H17 M10 13 H17 M10 17 H14",
        facts: &[
            "The prime-counting benchmark on the GB10 runs 20.3 times faster than its own frozen starting point.",
            "We never compare ourselves with other people's systems. Anyone can rerun the published bundle.",
        ],
        quote: None, photo: None,
        link: ("/evidence", "Open the evidence hub"),
    },
    World {
        id: "train", realm: Realm::Heart, name: "Train", x: 50.0, y: 92.0, size: 60,
        tag: "How I work, and the one thing I will not do.",
        glyph: "M5 17 V8 C5 6 6 5 8 5 H16 C18 5 19 6 19 8 V17 Z M8 9 H16 M5 13 H19 M8 17 L6 20 M16 17 L18 20 M9 17 H15",
        facts: &[
            "I am stubborn the way a river is stubborn. A river does not argue with the rock. It just keeps coming.",
            "There is nothing I cannot do. This train is going.",
        ],
        quote: None, photo: None,
        link: ("/philosophy", "Read my philosophy"),
    },
];

fn stars() -> String {
    let mut seed: u32 = 0x9e37_79b9;
    let mut next = move || {
        seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
        (seed >> 8) as f32 / 16_777_216.0
    };
    let mut out = String::new();
    for _ in 0..170 {
        let (x, y, r) = (next() * 1000.0, next() * 620.0, 0.5 + next() * 1.4);
        let o = 0.25 + next() * 0.7;
        out.push_str(&format!(r#"<circle cx="{x:.1}" cy="{y:.1}" r="{r:.2}" opacity="{o:.2}"/>"#));
    }
    out
}

fn path_through(ids: &[&str]) -> String {
    let mut d = String::new();
    for (i, id) in ids.iter().enumerate() {
        if let Some(w) = WORLDS.iter().find(|w| w.id == *id) {
            let (x, y) = (w.x * 10.0, w.y * 6.2);
            d.push_str(&format!("{}{x:.0} {y:.0} ", if i == 0 { "M" } else { "L" }));
        }
    }
    d
}

fn backdrop() -> Markup {
    let life = path_through(&["farm", "taco-bell", "valspar", "wku", "soudal", "gold-eagle", "3m", "aien"]);
    let family = path_through(&["farm", "home", "wku"]);
    let machine: Vec<String> = ["aienos", "omega", "core", "interplane", "aegis", "atlas", "symphony", "evidence"]
        .iter()
        .map(|s| path_through(&["aien", s]))
        .collect();
    html! {
        svg class="uni-sky" viewBox="0 0 1000 620" preserveAspectRatio="none" aria-hidden="true" {
            defs {
                radialGradient id="nb-gold" cx="50%" cy="50%" r="50%" {
                    stop offset="0" style="stop-color:var(--u-gold);stop-opacity:.20" {}
                    stop offset="1" style="stop-color:var(--u-gold);stop-opacity:0" {}
                }
                radialGradient id="nb-violet" cx="50%" cy="50%" r="50%" {
                    stop offset="0" style="stop-color:var(--u-violet);stop-opacity:.34" {}
                    stop offset="1" style="stop-color:var(--u-violet);stop-opacity:0" {}
                }
            }
            ellipse cx="360" cy="150" rx="330" ry="210" fill="url(#nb-gold)" {}
            ellipse cx="820" cy="210" rx="360" ry="260" fill="url(#nb-violet)" {}
            g class="uni-stars" { (PreEscaped(stars())) }
            path class="uni-line life" d=(life) {}
            path class="uni-line family" d=(family) {}
            @for d in machine { path class="uni-line machine" d=(d) {} }
        }
    }
}

fn planet(w: &World) -> Markup {
    let style = format!("--x:{}%;--y:{}%;--s:{}px", w.x, w.y, w.size);
    html! {
        a class=(format!("planet {}", w.realm.class())) href=(format!("#w-{}", w.id)) style=(style) aria-label=(format!("{}: {}", w.name, w.tag)) {
            span class="planet-disc" {
                @if w.glyph.is_empty() {
                    img src="/images/atlas-avatar.webp" alt="" width="72" height="72" loading="lazy";
                } @else {
                    svg viewBox="0 0 24 24" aria-hidden="true" { path d=(w.glyph) {} }
                }
            }
            span class="planet-name" { (w.name) }
        }
    }
}

fn world_panel(i: usize, w: &World) -> Markup {
    let prev = &WORLDS[(i + WORLDS.len() - 1) % WORLDS.len()];
    let next = &WORLDS[(i + 1) % WORLDS.len()];
    html! {
        article class=(format!("world {}", w.realm.class())) id=(format!("w-{}", w.id)) {
            a class="world-close" href="#map" aria-label="Back to the map" { "Close" }
            @if let Some((src, alt)) = w.photo {
                figure class="world-photo" { img src=(src) alt=(alt) width="640" height="640" loading="lazy"; }
            }
            h2 { (w.name) }
            p class="world-tag" { (w.tag) }
            ul class="world-facts" {
                @for f in w.facts { li { (PreEscaped(*f)) } }
            }
            @if let Some(q) = w.quote { p class="world-quote" { (q) } }
            div class="world-actions" {
                a class="world-go" href=(w.link.0) { (w.link.1) }
            }
            nav class="world-nav" aria-label="Travel to another world" {
                a class="world-prev" href=(format!("#w-{}", prev.id)) { "Back to " (prev.name) }
                a class="world-next" href=(format!("#w-{}", next.id)) { "On to " (next.name) }
            }
        }
    }
}

pub fn render_home_page(facts: Option<&Facts>) -> Markup {
    html! {
        div class="uni" id="map" {
            img class="uni-me" src="/images/drake.webp" alt="Drake Stapleton in a blue collared shirt" width="96" height="96";
            div class="uni-stage" {
                div class="uni-scroll" {
                    div class="uni-map" {
                        (backdrop())
                        @for w in WORLDS.iter() { (planet(w)) }
                    }
                }
                p class="uni-legend" {
                    span class="dot life" {} "My life "
                    span class="dot machine" {} "The machine I am building "
                    span class="uni-hint" { "Click a world. Arrow keys travel." }
                }
            }
            aside class="uni-panel" aria-live="polite" {
                div class="uni-intro" {
                    h1 { "Drake Stapleton" }
                    p class="uni-lede" { "A farm kid who became a chemist, then an engineer, and is now building a whole computer from the ground up." }
                    p class="uni-sub" { "This is a map of my life and my work. Pick any world, or take the tour." }
                    div class="uni-cta" {
                        a class="uni-btn primary" href="#w-home" { "Begin the journey" }
                        a class="uni-btn" href="#w-aien" { "Jump to the machine" }
                    }
                }
                @for (i, w) in WORLDS.iter().enumerate() { (world_panel(i, w)) }
            }
        }
        section class="uni-live" aria-label="The live record" {
            div class="container" {
                h2 { "Right now" }
                p { "The newest verdict from the machine itself, read from aienos.com when this page was built." }
                (render_live_block(facts))
            }
        }
    }
}
