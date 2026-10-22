---
title: CUDA++
slug: cudaplusplus
date: October 2026
summary: CUDA++ is the GPU half of Omega, now its own open source project under the MIT licence. It gives you everything CUDA gives you plus the motor, on the NVIDIA GPUs people already own, with no NVIDIA user-space software. This is how we built it.
---

CUDA++ is the part of AIEN that drives the GPU. It started inside Omega, and on 21 October 2026 it became its own project, with its full history, under the MIT licence so anyone can use it. It does everything CUDA does for a program, and it adds the motor: the part that decides how a whole model's bytes reach the math cores in time, and lets many conversations share one read of the weights.

It runs on NVIDIA GPUs, the ones people already paid for. Nothing NVIDIA ships above the kernel driver is used: no CUDA toolkit, no nvcc, no PTX, no libcuda, no cuBLAS. On 21 October 2026 the 35 billion parameter model I serve on the Spark was running on this code with zero CUDA libraries loaded.

## What it is for

Less computation, out of the big data centers. The goal is not a faster way to rent a data center. It is to need less of one: the same work with fewer bytes moved, fewer operations and fewer joules per answer, so it runs on a machine a person owns. Every result is reported as work per answer against the machine's own ceiling.

## How we built it

One person and a team of AI coding agents, one DGX Spark, four weeks. NVIDIA publishes the source of its Linux kernel module, but not its compiler, its driver library or the GPU's machine code. So we skipped every closed layer.

1. **The driver client, from the open kernel module's headers.** Everything libcuda does to the hardware goes through calls the open kernel module defines in its public headers. We copied 67 of those headers unchanged and wrote our own client against them.
2. **The machine code, reverse engineered.** NVIDIA does not publish the GPU's instruction encoding. We compiled small probe kernels of our own once with NVIDIA's compiler, read the machine code back with NVIDIA's disassembler, and compared kernels that differ in one detail to learn which bits mean what. We cross-checked against Mesa's open NVIDIA compiler and believed nothing until the chip ran it and gave the right answer.
3. **Launches in the order NVIDIA's open Vulkan driver uses**, from Mesa's NVK, on our own command lists.
4. **Built up, checking everything.** Vector add on the GB10 on 26 September, numerics closed on the chip on 2 October, the tensor cores measured at 115.14 TFLOPS by our own probes on 13 October, and the 35B model served on 21 October.

We used no NVIDIA binary, and no NVIDIA source other than those open headers. We did not take apart NVIDIA's programs, and we did not touch the GPU's firmware, which stays NVIDIA's because the chip checks its signature.

## What is still being built

- A page of CUDA's pain points, how CUDA++ fixes each one, and the command that shows it.
- One command that swaps CUDA out for CUDA++ on a machine and one that swaps it back, all the way down to the kernel driver.
- Our own GB10 kernel driver in place of NVIDIA's. Its simulator gates pass; it has not run on the chip yet.

Each of these is shown as in progress on the project site until it has a receipt.

## Where to look

- The project page, every step with its receipt: [aienos.com/cudaplusplus](https://www.aienos.com/cudaplusplus/)
- The repository: [aien-dev/CUDAplusplus](https://github.com/aien-dev/CUDAplusplus) (private until its audit is done, then public)
- The whole method, step by step: [How we did it](https://github.com/aien-dev/CUDAplusplus/blob/62ea5a214d46f36e88cb7b892cecebf1f3e8a172/docs/HOW_WE_DID_IT.md)
- The rest of the stack: [AIEN](/aien)
