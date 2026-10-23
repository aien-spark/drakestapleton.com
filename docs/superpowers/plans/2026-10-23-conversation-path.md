# Contextual conversation path implementation plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans. One cohesive native implementation, TDD and one independent final review. Drake opts out of approval questions; execute the bounded source work without step prompts.

**Goal:** Complete drakestapleton.com#71's source/qualification acceptance: an obvious homepage-to-conversation path and useful public inquiry context, preserving the existing form.

**Architecture:** Reuse current Rust/Maud pages and existing visual system. Add a contextual native anchor to the existing homepage introduction, retain journey/machine links, and enrich the existing interest page with at most three inquiry examples, concise note guidance and accurate third-party submission disclosure. No JavaScript is needed for the new flow.

**Tech stack:** Existing Rust/Maud static builder, existing CSS and HTML form. No runtime, new dependency, tracking, provider, form or campaign page.

**Spec:** https://github.com/aien-dev/drakestapleton.com/issues/71. Related #68 supplies concrete inquiry examples, but this task does not claim its third-party delivery/error-path acceptance or close it.

## Global constraints

- Base e55a667463a170e30603cdc428294abdb1f9f716, own branch feat/71-contextual-conversation. Preserve L402 work pages and L422 voice work.
- **NO DEPLOYMENT or auto-merge enabling deployment without explicit approval.** This is source and local qualification only. Record that hold for the publisher; do not add an eligible ordinary READY line that could auto-deploy.
- Preserve issue60's private editorial gate. Do not read/recover/publish private chapters or invent biography. Existing public introduction and story/worlds remain unchanged.
- No unverified specific video, demo, YouTube or campaign URL. Generic visitor context is conditional wording, not a claim that particular media exists.
- Keep existing formsubmit.co action, POST, field names, subject and redirect. One form only. No real submission/email or native mail app launch during tests.
- Disclose the existing external handler; no new trackers or scripts. No formal service, pricing, guaranteed availability or outcome claims.
- Preserve existing routes, canonical/social metadata and mobile navigation. Use the current look; no new illustrations, fake astronomy or unrelated redesign.

## Review focus

- New CTA remains a real keyboard-usable /interest anchor when page JavaScript is disabled.
- Three actions fit a390px viewport and do not hide/obscure existing journey/machine actions.
- Added context makes no new personal/media/service claims and no deployment is authorized.
- Existing one-form payload/provider remains unchanged; third-party privacy disclosure and direct-email fallback are honest.
- Failed/unavailable live facts are still labelled unavailable; do not replace the facts collector or invent status.

### Task 1: Render and qualify the complete bounded path

**Files:** pages/home.rs, pages/interest.rs; public CSS only if rendered layout needs a focused adjustment. Evidence under evidence/CONVERSATION-PATH-L656/.

**Interfaces:** Existing render_home_page(Option<&Facts>) -> Markup and render_interest_page() -> Markup remain unchanged. New navigation target is the existing /interest route; no new route or handler.

- [ ] Baseline full cargo test --locked --workspace; retain raw output.
- [ ] Add rendered-output tests for a contextual introductory /interest anchor and retained existing actions, at most three real inquiry examples plus note guidance, and one unchanged form contract with provider disclosure/direct-email fallback. Run and observe meaningful missing-behavior RED before template edits.
- [ ] Implement only the two public-page changes. Keep original personal story/worlds, form mechanism, navigation and metadata intact. Run full native suite and release build/verify.
- [ ] Exercise actual local HTTP desktop1440x900 and phone390x844, with and without page JavaScript. Follow the CTA by keyboard, inspect focus/overflow, look at screenshots, and test native required-field validation without submitting. Compare form contracts, page canonicals/social metadata and other rendered outputs against the pinned baseline using the same explicit facts input.
- [ ] Publish original RED and actual GREEN, tool/source/asset hashes, browser case inventory/screenshots and limits. Independently review the frozen bounded delta. Preserve the NO-DEPLOY hold and route only a non-auto-deploying source handoff to the existing publisher. No issue closure or live-success claim from local checks.
