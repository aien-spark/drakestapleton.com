use maud::{html, Markup, PreEscaped};

// Illustrated figures for the long works pages. Text in each figure is taken
// from the page's own markdown; colors come from CSS classes in style.css
// ("Work page figures") so every figure follows the light and dark themes.

const AEGIS_HERO: &str = r#"<svg class="ph-art" viewBox="0 0 960 340" role="img" aria-labelledby="wa-aegis-t">
<title id="wa-aegis-t">An intruder stopped at the closed gate of a walled estate. Sensors on the wall watch the door, and evidence leaves the house as a sealed receipt.</title>
<rect class="ph-sky" width="960" height="340"/>
<path class="ph-hill-back" d="M0 270 V210 L90 160 L170 215 L250 170 L340 230 L380 270 Z"/>
<rect class="ph-ground" y="270" width="960" height="70"/>
<path class="ph-stroke-soft" d="M0 270 H960"/>
<path class="ph-line-accent ph-dash" d="M400 270 V122 Q400 82 440 82 H780 Q820 82 820 122 V270"/>
<g class="ph-line-soft"><path d="M430 84 L610 104 M520 80 L610 104 M700 80 L610 104 M790 84 L610 104"/></g>
<g class="ph-fill-accent"><circle cx="430" cy="84" r="8"/><circle cx="520" cy="80" r="8"/><circle cx="700" cy="80" r="8"/><circle cx="790" cy="84" r="8"/></g>
<path class="ph-wall" d="M520 168 H700 V270 H520 Z"/>
<path class="ph-line" d="M500 168 L610 104 L720 168"/>
<rect class="ph-fill-accent" x="596" y="218" width="28" height="52"/>
<rect class="ph-sky" x="540" y="190" width="30" height="26"/><rect class="ph-sky" x="650" y="190" width="30" height="26"/>
<path class="ph-line-accent ph-dash" d="M176 262 H384"/>
<g class="ph-line"><circle cx="150" cy="212" r="16"/><path d="M150 228 V256 M150 238 L128 252 M150 238 L172 252 M150 256 L136 270 M150 256 L164 270"/></g>
<rect class="ph-fill-accent" x="392" y="222" width="18" height="48" rx="2"/>
<path class="ph-line-accent" d="M366 236 L388 258 M388 236 L366 258"/>
<path class="ph-line-accent ph-dash" d="M706 236 H842"/>
<path class="ph-wall" d="M850 160 H930 V268 H850 Z"/>
<path class="ph-line-soft" d="M864 184 H916 M864 202 H916 M864 220 H896"/>
<circle class="ph-fill-accent" cx="910" cy="248" r="12"/>
</svg>"#;

const AEGIS_ICONS: [&str; 4] = [
    r#"<svg class="wa-icon" viewBox="0 0 56 56" aria-hidden="true"><path class="ph-line" d="M5 28 C15 10 41 10 51 28 C41 46 15 46 5 28 Z"/><circle class="ph-fill-accent" cx="28" cy="28" r="8"/></svg>"#,
    r#"<svg class="wa-icon" viewBox="0 0 56 56" aria-hidden="true"><circle class="ph-line" cx="24" cy="24" r="15"/><path class="ph-line" d="M35 35 L50 50"/><circle class="ph-fill-accent" cx="24" cy="24" r="5"/></svg>"#,
    r#"<svg class="wa-icon" viewBox="0 0 56 56" aria-hidden="true"><path class="ph-line" d="M28 5 L48 13 V29 C48 41 38 49 28 53 C18 49 8 41 8 29 V13 Z"/><path class="ph-line-accent" d="M20 22 L36 38 M36 22 L20 38"/></svg>"#,
    r#"<svg class="wa-icon" viewBox="0 0 56 56" aria-hidden="true"><path class="ph-line" d="M12 5 H36 L46 15 V51 H12 Z"/><path class="ph-line-soft" d="M20 22 H38 M20 30 H38"/><circle class="ph-fill-accent" cx="34" cy="42" r="7"/></svg>"#,
];

const AIEN_HERO: &str = r#"<svg class="ph-art" viewBox="0 0 960 340" role="img" aria-labelledby="wa-aien-t">
<title id="wa-aien-t">A single chip on one machine, with traces running out to six layers of the stack.</title>
<defs><radialGradient id="wa-glow" cx="50%" cy="50%" r="50%"><stop offset="0" style="stop-color:var(--accent-bright);stop-opacity:.45"/><stop offset="1" style="stop-color:var(--accent-bright);stop-opacity:0"/></radialGradient></defs>
<rect class="ph-sky" width="960" height="340"/>
<circle cx="480" cy="170" r="190" fill="url(#wa-glow)"/>
<g class="ph-line-soft"><path d="M80 60 H300 V120 H400 M80 120 H200 V170 H400 M80 280 H300 V220 H400 M880 60 H660 V120 H560 M880 120 H760 V170 H560 M880 280 H660 V220 H560"/></g>
<g class="ph-fill-accent"><circle cx="80" cy="60" r="7"/><circle cx="80" cy="120" r="7"/><circle cx="80" cy="280" r="7"/><circle cx="880" cy="60" r="7"/><circle cx="880" cy="120" r="7"/><circle cx="880" cy="280" r="7"/></g>
<g class="ph-line-soft"><path d="M430 74 V50 M460 74 V50 M490 74 V50 M520 74 V50 M430 266 V290 M460 266 V290 M490 266 V290 M520 266 V290"/></g>
<rect class="ph-wall" x="400" y="74" width="160" height="192" rx="14"/>
<rect class="ph-line" x="424" y="98" width="112" height="144" rx="6" style="fill:none"/>
<g class="ph-line-soft"><path d="M424 146 H536 M424 194 H536 M480 98 V242"/></g>
<rect class="ph-fill-accent" x="456" y="150" width="48" height="40" rx="5"/>
</svg>"#;

const AIEN_LAYERS: [(&str, &str, &str); 6] = [
    ("AIENOS", "The operating system", "Agent-first: the agent is the control plane and the model is never the kernel."),
    ("Omega", "Language and GPU engine", "The language and semantic substrate the stack is being rewritten into, with its own GPU engine for the GB10."),
    ("Sovereign core", "Host runtime, in Rust", "Command line tool and daemon, inference ABI, paged KV cache and scheduler, and the judging pipeline."),
    ("Architecture", "Flows and decisions", "The canonical flows, boundaries and Architecture Decision Records."),
    ("Protocols", "Specs and schemas", "The specifications, wire protocols and schemas."),
    ("Physics", "Machine realization", "The FORGE layer and the boot artefacts that bring the hardware up, with their evidence."),
];

const FLOW_STEPS: [(&str, &str); 4] = [
    ("Pull request", "Opened in any AIEN repository."),
    ("Judged on the Spark", "My own pipeline runs the repository's gates on the real hardware, not a hosted runner."),
    ("Signed evidence bundle", "Tied to the exact commit and published with the verdict."),
    ("Verdict", "A change merges only when the verdict is PASS."),
];

const RESEARCH_HERO: &str = r#"<svg class="ph-art" viewBox="0 0 960 300" role="img" aria-labelledby="wa-res-t">
<title id="wa-res-t">A learning curve rising in steps toward a dashed ceiling, with the run's own earlier baseline marked below it.</title>
<rect class="ph-sky" width="960" height="300"/>
<g class="ph-line-soft"><path d="M80 250 H880 M80 190 H880 M80 130 H880"/></g>
<path class="ph-line-accent ph-dash" d="M80 60 H880"/>
<path class="ph-line-soft" d="M80 232 H880" style="stroke-dasharray:3 7"/>
<path class="ph-line" d="M80 232 H250 V204 H380 V172 H520 V132 H650 V100 H780 V84 H880" style="fill:none"/>
<g class="ph-fill-accent"><circle cx="250" cy="204" r="8"/><circle cx="380" cy="172" r="8"/><circle cx="520" cy="132" r="8"/><circle cx="650" cy="100" r="8"/><circle cx="780" cy="84" r="8"/></g>
</svg>"#;

const RESEARCH_Q: [(&str, &str); 3] = [
    ("Did it learn anything?", "Held-out bits: facts or capability the system did not have, tested on data it never saw."),
    ("What did it cost?", "The time and energy spent learning them, against my own previous run."),
    ("Did it get easier?", "Whether what it learned made the next discovery cheaper."),
];

fn hero(svg: &'static str) -> Markup {
    html! { div class="wa-hero" { (PreEscaped(svg)) } }
}

pub fn aegis_figures() -> Markup {
    let steps = [
        ("Observe", "Read the IDS, DNS, identity, endpoint and firewall signals already in the security path."),
        ("Recognize", "Correlate behavior across sensors and separate routine noise from a break-in."),
        ("Contain", "Choose one tested action inside the authorized estate and end the session."),
        ("Preserve", "Record the signal, decision, action, time and result for the incident team."),
    ];
    let actions = [
        ("Terminate session", "Cut the intruder's connection."),
        ("Isolate host", "Take the affected machine off the network."),
        ("Block path", "Close the route the intruder used."),
        ("Revoke token", "End the credential in use."),
        ("Preserve trail", "Keep a signed, timestamped receipt."),
    ];
    let slices = [("Tool use", 45u32, 45u32), ("Coding", 8, 10), ("Science", 3, 4), ("Cyber", 3, 3)];
    html! {
        div class="wa-figures" {
            (hero(AEGIS_HERO))
            section class="wa-block" {
                h2 { "The response path" }
                ol class="wa-steps" {
                    @for (i, (name, body)) in steps.iter().enumerate() {
                        li {
                            (PreEscaped(AEGIS_ICONS[i]))
                            h3 { (name) }
                            p { (body) }
                        }
                    }
                }
            }
            section class="wa-block" {
                h2 { "Five named actions, nothing else" }
                p class="wa-lead" { "Every action targets systems, devices, accounts and sessions under the operator's authority. Novel patterns, uncertain attribution or production-impacting cuts go to a human." }
                ul class="wa-tiles" {
                    @for (name, body) in actions {
                        li { h3 { (name) } p { (body) } }
                    }
                }
            }
            section class="wa-block" {
                h2 { "First adapter, frozen 62-case suite" }
                p class="wa-big" { "59 of 62" span { " checks passed, 95.2%, on 20 August 2026" } }
                div class="wa-bars" role="list" {
                    @for (name, got, of) in slices {
                        div class="wa-bar" role="listitem" {
                            span class="wa-bar-name" { (name) }
                            span class="wa-bar-track" { span class="wa-bar-fill" style=(format!("width: {}%", got * 100 / of)) {} }
                            span class="wa-bar-val" { (got) "/" (of) }
                        }
                    }
                }
            }
        }
    }
}

pub fn aien_figures() -> Markup {
    html! {
        div class="wa-figures" {
            (hero(AIEN_HERO))
            section class="wa-block" {
                h2 { "Six parts, one machine" }
                p class="wa-lead" { "Everything runs on a single NVIDIA DGX Spark (Grace Blackwell GB10). The parts are being gathered into one source tree, Aienix, so they move in step and are judged as one." }
                ul class="wa-layers" {
                    @for (name, role, body) in AIEN_LAYERS {
                        li { h3 { (name) } p class="wa-role" { (role) } p { (body) } }
                    }
                }
            }
            section class="wa-block" {
                h2 { "How a change gets in" }
                ol class="wa-flow" {
                    @for (name, body) in FLOW_STEPS {
                        li { h3 { (name) } p { (body) } }
                    }
                }
                p class="wa-note" { "A green hosted check means the bridge ran. It is not the verdict." }
            }
        }
    }
}

pub fn research_figures() -> Markup {
    html! {
        div class="wa-figures" {
            (hero(RESEARCH_HERO))
            section class="wa-block" {
                h2 { "Three questions for every run" }
                ol class="wa-flow wa-flow-3" {
                    @for (name, body) in RESEARCH_Q {
                        li { h3 { (name) } p { (body) } }
                    }
                }
                p class="wa-note" { "Speed is reported against the machine's physical limit and against my own previous run, never against someone else's system." }
            }
        }
    }
}
