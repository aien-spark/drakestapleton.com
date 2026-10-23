use maud::{html, Markup};

pub fn render_interest_page() -> Markup {
    html! {
        div class="container reading-width" style="padding: clamp(48px, 6vw, 96px) var(--gutter);" {
            header class="section-header" {
                p class="eyebrow" { "COMMUNICATION & INQUIRY" }
                h1 { "Start a Conversation" }
                p class="section-desc" {
                    "Leave Drake Stapleton a note regarding systems architecture, inference research, or verified engineering collaboration."
                }
            }

            div class="prose" {
                p {
                    "I welcome conversations with operators, architects, and builders working on physical engineering, compiled runtimes, hardware-bound security, or accountable AI orchestration."
                }
                p {
                    "If you came from a demo, architecture discussion, short video or project link, mention what you saw and what you would like to understand or build."
                }
                h2 style="font-size: clamp(1.35rem, 3vw, 1.8rem); line-height: 1.3;" { "What would you like to discuss?" }
                ul class="conversation-topics" {
                    li { "A local inference stack and the operational trust boundaries around it." }
                    li { "A signed verification or receipt pipeline, and what its evidence can actually establish." }
                    li { "An agent runtime that keeps model suggestions outside kernel authority." }
                }
                p id="inquiry-guidance" {
                    strong { "A useful note includes" }
                    " the problem, hardware or runtime constraints, the stage of work, any evidence or links, and the outcome you are hoping for."
                }

                form action="https://formsubmit.co/drake@aienos.com" method="POST" class="interest-form" style="display: flex; flex-direction: column; gap: 16px; margin-top: 32px;" {
                    input type="hidden" name="_next" value="https://www.drakestapleton.com/interest?received=1";
                    input type="hidden" name="_subject" value="New inquiry from drakestapleton.com";
                    
                    div {
                        label for="name" style="display: block; font-family: var(--mono); font-size: 0.85rem; margin-bottom: 6px;" { "YOUR NAME" }
                        input type="text" id="name" name="name" required="" style="width: 100%; padding: 12px; border: 1px solid var(--line-strong); background: var(--surface-raised); border-radius: 4px;" {}
                    }

                    div {
                        label for="email" style="display: block; font-family: var(--mono); font-size: 0.85rem; margin-bottom: 6px;" { "YOUR EMAIL" }
                        input type="email" id="email" name="email" required="" style="width: 100%; padding: 12px; border: 1px solid var(--line-strong); background: var(--surface-raised); border-radius: 4px;" {}
                    }

                    div {
                        label for="message" style="display: block; font-family: var(--mono); font-size: 0.85rem; margin-bottom: 6px;" { "MESSAGE" }
                        textarea id="message" name="message" rows="5" required="" aria-describedby="inquiry-guidance form-privacy" style="width: 100%; padding: 12px; border: 1px solid var(--line-strong); background: var(--surface-raised); border-radius: 4px;" {}
                    }

                    button type="submit" class="btn btn-primary" style="align-self: flex-start; margin-top: 8px;" {
                        "Transmit Message"
                    }
                }
                p id="form-privacy" style="margin-top: 16px;" {
                    "This form sends your name, email and message to FormSubmit for email forwarding. Do not include passwords, private keys or other secrets. You can also "
                    a href="mailto:drake@aienos.com" { "email me directly" }
                    "."
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inquiries_have_bounded_examples_and_useful_context() {
        let page = render_interest_page().into_string();
        assert!(page.contains("What would you like to discuss?"));
        assert_eq!(page.matches("<li>").count(), 3);
        for subject in ["local inference", "signed verification", "model suggestions"] {
            assert!(page.contains(subject), "missing inquiry subject: {subject}");
        }
        assert!(page.contains("A useful note includes"));
        for field in ["problem", "hardware", "stage", "evidence", "outcome"] {
            assert!(page.contains(field), "missing note guidance: {field}");
        }
    }

    #[test]
    fn existing_single_form_provider_and_payload_are_preserved() {
        let page = render_interest_page().into_string();
        assert_eq!(page.matches("<form ").count(), 1);
        assert!(page.contains("action=\"https://formsubmit.co/drake@aienos.com\" method=\"POST\""));
        assert!(page.contains("name=\"_next\" value=\"https://www.drakestapleton.com/interest?received=1\""));
        assert!(page.contains("name=\"_subject\" value=\"New inquiry from drakestapleton.com\""));
        for field in ["name", "email", "message"] {
            assert!(page.contains(&format!("name=\"{field}\"")));
            assert!(page.contains(&format!("for=\"{field}\"")));
        }
    }

    #[test]
    fn external_form_is_disclosed_with_a_native_email_alternative() {
        let page = render_interest_page().into_string();
        assert!(page.contains("FormSubmit"));
        assert!(page.contains("name, email and message"));
        assert!(page.contains("href=\"mailto:drake@aienos.com\""));
        assert!(!page.contains("<script"));
    }
}
