---
title: Research
slug: research
date: October 2026
summary: What the AIEN research program measures today, why an earlier study was withdrawn, and where the current results with receipts live.
---

The research question behind AIEN is simple to state: can a machine I own learn something real, keep learning, and show its work? Everything I measure serves that question.

## What I measure

**Turing yield.** The unit is the held-out bit: a fact or capability the system did not have, tested on data it never saw, so a pass means learning rather than memory. I ask three things of every run. Did it learn anything? What did it cost to learn, in time and energy? Did what it learned make the next discovery easier? A run is judged against my own previous run, never against someone else's system.

**Verified understanding.** Weights suggest, programs explain, verification decides, and evidence teaches. A claim enters the record only when it resolves to a reproducible command and a signed evidence bundle produced by the Spark judge on the real hardware.

**Distance to the ceiling.** When I report speed, I report it as a fraction of the machine's physical limit and as a change against my own frozen baseline. That tells me how much software is still standing in the way of the hardware.

## Programs in flight

- **Brownian discovery:** a pre-registered search program whose candidate stays frozen while it is being measured, with a negative control beside every result.
- **Native inference on the Spark:** the host runtime's inference ABI, paged KV cache and scheduler, and Omega's own GPU engine for the GB10, judged gate by gate.
- **INTERPLANE:** the open protocol between models and agent runtimes, with a conformance suite and pre-registered stage gates whose reports state the result whether it passed or failed.

## A withdrawn study

An earlier page here presented "Eliminating software orchestration tax in large language model inference", with throughput, latency and memory figures and comparisons against interpreted runtimes and other inference engines. It was withdrawn on 23 September 2026. Its figures predate the evidence standard above, and its comparisons with other people's systems are not something this project publishes. The text and tables remain in the repository history for the record; nothing from it is a current result.

## Where the results are

- Measurement bundles, each reproducible: [github.com/aien-dev/benchmarks](https://github.com/aien-dev/benchmarks)
- The evidence hub on this site, with the current Spark verdict: [evidence](/evidence)
- The project's live state: [aienos.com](https://www.aienos.com/)
