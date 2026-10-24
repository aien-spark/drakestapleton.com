# Drake Stapleton: Freedom Fighter, AI Architect & Operator

The personal project record of Drake Stapleton: production orchestration, GPU systems, model training, research, and Atlas.

Open to good conversation and research partnerships that advance what my generation can build with AI.

Live: [https://www.drakestapleton.com/](https://www.drakestapleton.com/)
Conversation: [https://www.drakestapleton.com/interest](https://www.drakestapleton.com/interest)

## Local development and release

The site is a static site rendered by the Rust program in `crates/site-builder` (Maud templates and a Markdown reader). There is no Node build, no `package.json` and no dev server. Run these from the repository root:

```bash
cargo test --manifest-path crates/site-builder/Cargo.toml
cargo run --release --manifest-path crates/site-builder/Cargo.toml -- build dist
cargo run --release --manifest-path crates/site-builder/Cargo.toml -- verify dist
```

`build dist` copies `public/` into `dist/`, renders the pages and writes `sitemap.xml` and `robots.txt` (which allows all crawlers), then verifies the result. `verify dist` re-checks an existing output directory. Page titles and descriptions come from `site-metadata.json`, and the works pages come from the Markdown files in `content/works/`.

## Agent-readable content

Every generated content page has a Markdown mirror built from the same rendered body: `/index.md` for the homepage, `/atlas.md` for `/atlas`, and `/symphony/workflow.md` for a nested route. HTML advertises the mirror with `rel="alternate"` and the resource index with `rel="describedby"`. Page-specific JSON-LD connects the page, website, and Drake's identity. The 404 page is excluded from these discovery outputs.

The build also writes `/sitemap.md` and `/llms-full.txt` for the pages in the public sitemap. `/AGENTS.md` explains public site usage; `/glossary` defines the project's terms. Edit templates and content rather than generated text to keep HTML and Markdown in sync.

Sitemap `lastmod`, Markdown `dateModified`, and JSON-LD `dateModified` come from the latest Git commit touching the page's sources, shared templates, or metadata. They describe editorial source changes, not live facts or hourly rebuild times. CI checks out full history. Source archives without Git history omit dates rather than inventing them.

GitHub Pages cannot negotiate `Accept: text/markdown` or set custom `Link` response headers. Use the explicit Markdown URLs and HTML discovery links on this host. Header negotiation would require a different serving layer.

The build renders the home page, `/path`, `/what-i-learned`, `/philosophy`, `/software`, `/evidence`, `/atlas`, `/symphony` (with `/first`, `/workflow` and `/map`), `/donate`, `/interest`, the works index with one page per work, a 404 page, and the `/aien`, `/aegis` and `/research` pages.

## Checks

`cargo test --manifest-path crates/site-builder/Cargo.toml` runs the unit tests. `verify dist` fails the build when a required route is missing or lacks its pre-rendered page, when a page title does not match `site-metadata.json`, when `sitemap.xml` or `robots.txt` is missing, or when any page contains an em dash or an en dash. `.github/workflows/pages.yml` runs the tests and the build on every pull request and push to `main`.

## Live AIEN record

The home page, the evidence page and the AIEN page carry the Spark judge's verdict for the current sovereign-core commit and one row per core component. The builder reads them at build time from `https://www.aienos.com/facts.json`, the file aienos.com writes from GitHub every hour (`crates/site-builder/src/components/facts.rs`). When the file cannot be read the pages say so instead of showing stale figures. Set `AIEN_FACTS_FILE=<path>` to build from a local copy. `.github/workflows/pages.yml` rebuilds the site every hour so the record stays current.

## Telemetry

The built site loads no outside font, script or tracker: every request a visitor's browser makes goes to this site. The build has no `GOOGLE_ADS_ID` setting. [ops/GOOGLE_ADS_ACQUISITION.md](ops/GOOGLE_ADS_ACQUISITION.md) is a campaign playbook written for an earlier Vite build. The conversion events and Content Security Policy it describes are not in the current code, so read it as a plan. The only live feed is the strip at the top of the home page, which connects to `/api/live/stream` on the same origin and updates only when the page is served by `crates/site-server`.

## Hosting

GitHub Pages deploys `dist/` from `main` (`.github/workflows/pages.yml`, custom domain in `CNAME`).

The `/interest` form posts to formsubmit.co, which forwards the message by email. The site stores no signup data. The unit files in `ops/` (`ops/drake-waitlist-api.service` and `ops/renew-waitlist-cert.sh`) belong to an earlier waitlist API whose Python source is no longer in this repository.

`crates/site-server` is a second, optional program: an Axum server that serves a built `dist/` from memory on port 18090 (`PORT` and `DIST_DIR` set the port and directory), with `/api/health`, `/api/chat` (forwards to a local model server at `127.0.0.1:18006`), a live telemetry stream at `/api/live/stream` and `/api/live/telemetry`. `ops/systemd/drakestapleton-site.service` runs it on the author's machine. GitHub Pages does not run it.

## License

GNU Affero General Public License v3.0 or later (AGPL-3.0-or-later). See [LICENSE](LICENSE).

## Credit

Atlas Symphony was designed, named, and first operated by Drake Stapleton on 30 July 2026.
