---
title: AIEN
slug: aien
date: October 2026
summary: AIEN is a sovereign computing stack built on one machine I own: its own operating system, its own language with a GPU engine, a host runtime, and a pipeline that judges every change on the hardware itself. Everything is public.
---

AIEN is the project I spend my days on. The goal is persistent machine intelligence on a machine I own, with nothing in the trusted base that depends on an outside service, and with every claim the project makes resolving to a command anyone can rerun and an evidence bundle anyone can read.

The development machine is a single NVIDIA DGX Spark (Grace Blackwell GB10). I lead the project and decide; AI agents write, test and review as day-to-day collaborators. The architecture, the decision records and the execution plan are public and are the authority for what gets built.

## What the stack is

Six parts depend on each other and are the reason the project exists.

1. **AIENOS**, the operating system. Agent-first: the agent is the control plane, and the model is never the kernel. Its kernel passes its gates in the emulator and has not yet booted on the real machine.
2. **Omega**, the language and semantic substrate the stack is being rewritten into, with its own GPU engine for the GB10. No CUDA in the plan.
3. **Sovereign core**, the host runtime in Rust: the command line tool and daemon, the inference ABI, the paged KV cache and scheduler, and the pipeline that judges every repository.
4. **Architecture**, the canonical flows, boundaries and Architecture Decision Records, and the map from design to implementation.
5. **Protocols**, the specifications, wire protocols and schemas.
6. **Physics**, the FORGE machine-realization layer and the boot artefacts that bring the hardware up, with their evidence.

These are being gathered into one source tree, **Aienix**, so they move in step and are judged as one. Components that only consume the core are attachments in their own repositories, pinned at the boundary: among them INTERPLANE, the open protocol between models and agent runtimes, and the benchmarks repository that holds every measurement bundle.

## How a change gets in

Every pull request in every AIEN repository is judged on the Spark itself, by my own pipeline, not by a hosted runner. The judge runs the repository's gates on the real hardware, signs an evidence bundle tied to the exact commit, and publishes the verdict. A change merges only when that verdict is PASS. A green hosted check means the bridge ran; it is not the verdict.

## What I measure

Two rules govern every number the project publishes.

- **Against my own runs and the ceiling.** Speed is reported against the machine's physical limit and against my own previous run. I do not publish comparisons with other people's systems; anyone who wants a comparison can run the published bundle against mine.
- **Receipts, not promises.** A figure appears only when it resolves to a reproducible command and a signed bundle. Figures that predate this standard were withdrawn from this site on 23 September 2026 and live on only in the repository history.

The guiding measure is what I call Turing yield: how many held-out bits the system actually learned, what it cost to learn them, and whether that learning made the next discovery easier. The model's standing question is "how do I get more Turing today?"

## Principles

- Own the hardware, own the memory, own the code. No metered tokens in the loop.
- Build close to the metal. When an outside tool looks tempting, learn how it works and build the native version that fits.
- Never hand-roll cryptography or weaken security to stay native. That exception is said out loud.
- Open source everything, under the AGPL.
- Pre-alpha and honest about it. Nothing on this page is a release or a hardware qualification claim; when a component is qualified on the Spark, the bundle that proves it will be linked.

## Where to look

- The project site, with the live state of every component: [aienos.com](https://www.aienos.com/)
- The core tree and the map of repositories: [Aienix](https://github.com/aien-dev/aienix)
- The organisation on GitHub: [aien-dev](https://github.com/aien-dev)
- The protocol between models and runtimes: [interplane.io](https://interplane.io/)
