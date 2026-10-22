use maud::{html, Markup};
use crate::components::facts::{render_verdict_card, Facts};

pub fn render_evidence_page(facts: Option<&Facts>) -> Markup {
    html! {
        div class="container" style="max-width: var(--page); margin: 0 auto; padding: clamp(48px, 6vw, 96px) var(--gutter);" {
            header class="section-header" {
                p class="eyebrow" { "CANONICAL TRUST HUB & PHYSICAL SILICON RECEIPTS" }
                h1 { "Canonical Evidence Hub & Verified Claims" }
                p class="section-desc" {
                    "Every claim on this page links to its record, and each record says where it ran: on the GB10, on the host, or in the emulator."
                }
            }

            div class="grid stats-grid" style="margin-top: 36px;" {
                div class="stat-card" {
                    p class="stat-value tabular-nums" { "R1-R15" }
                    p class="stat-unit" { "PASS on GB10" }
                    p class="stat-label" {
                        "Omega runtime qualification ladder on frozen candidate CAND-3; R15 is a conditional pass under a declared exception (Secure Boot was off). "
                        a href="https://github.com/aien-dev/aien-architecture/blob/main/qualification/candidates/CAND-3.gates.md" { "Gate record" }
                    }
                }
                div class="stat-card" {
                    p class="stat-value tabular-nums" { "24 / 24" }
                    p class="stat-unit" { "QEMU gates PASS" }
                    p class="stat-label" {
                        "AIENOS C kernel gates in the emulator, 0 NOT_RUN. Emulator evidence only; the kernel has not booted on the real machine yet. "
                        a href="https://github.com/aien-dev/aienos/blob/main/evidence/ck_gates_29ed03c97454354a18676f768273dd7f7a8e15462c4be1939d9994d277d0d639.json" { "Receipt" }
                    }
                }
                div class="stat-card" {
                    p class="stat-value tabular-nums" { "20.3x" }
                    p class="stat-unit" { "frozen baseline" }
                    p class="stat-label" {
                        "Prime Drag Race native GPU sieve on GB10 at 19,701 passes per second, 20.3x its own frozen starting point of 971. "
                        a href="https://github.com/aien-dev/benchmarks/blob/main/prime-drag-race/CAMPAIGN-2026-10-05.md" { "Campaign record" }
                    }
                }
                (render_verdict_card(facts))
            }

            section style="margin-top: 48px;" {
                h2 style="font-family: var(--heading); margin-bottom: 24px;" { "The Cryptographic Improvement Ledger" }
                div class="prose reading-width" {
                    p {
                        "Every change to an AIEN repository is judged on the Spark itself by my own pipeline. The verdict is a signed evidence bundle tied to the exact commit, and a change merges only when that bundle says PASS. The card above shows the current verdict for the host runtime, read from the record aienos.com publishes."
                    }
                    p {
                        "Speed is reported against the machine's physical ceiling and against my own previous run, never against someone else's system. A figure appears on this site only when it resolves to a receipt."
                    }
                }
            }
        }
    }
}
