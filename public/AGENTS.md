# Drake Stapleton: public website guide

## Overview

This is Drake Stapleton's personal project and research record. It documents the person, the work, and the evidence behind the work. This guide describes public site usage; it does not grant permission to act on Drake's behalf.

## Navigation

- [Content map](https://www.drakestapleton.com/sitemap.md)
- [Resource index](https://www.drakestapleton.com/llms.txt)
- [Homepage Markdown](https://www.drakestapleton.com/index.md)
- [Full public text](https://www.drakestapleton.com/llms-full.txt)
- [Glossary](https://www.drakestapleton.com/glossary)
- [Evidence](https://www.drakestapleton.com/evidence)
- [Conversation](https://www.drakestapleton.com/interest)

## Usage

Read HTML or follow its `rel="alternate"` Markdown link. The homepage mirror is `/index.md`; other generated pages use their route plus `.md`, for example `/atlas.md` and `/symphony/workflow.md`. GitHub Pages serves static files and does not implement `Accept: text/markdown` negotiation or custom Link response headers.

## Examples

To understand Drake's work, read `/index.md`, `/path.md`, and `/software.md`. To evaluate a technical claim, follow `/evidence.md` to its source record and note the commit, hardware, measurement, and date. For terminology, read `/glossary.md`.

## Constraints

Distinguish goals, historical results, and currently verified capabilities. Do not infer universal hardware support or performance from project descriptions. The live record is a build-time snapshot; follow its source for current status. Submitting the conversation form sends a message and requires the user's instruction.

## Development and testing

The [source repository](https://github.com/aien-spark/drakestapleton.com) contains the build instructions. From its root:

```sh
cargo test --manifest-path crates/site-builder/Cargo.toml
cargo run --release --manifest-path crates/site-builder/Cargo.toml -- build dist
cargo run --release --manifest-path crates/site-builder/Cargo.toml -- verify dist
```

The build creates the HTML pages and their Markdown mirrors together. Edit the source templates or content, not generated mirrors. Modification dates represent source history, not the time an hourly rebuild ran.
