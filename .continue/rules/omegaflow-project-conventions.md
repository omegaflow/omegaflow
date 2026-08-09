---
description: omegaflow project conventions — stack, formats, hardware context
---

# Project Conventions: omegaflow

This file complements `omegaflow-gremium.md` (Identity/Heuristics).

## Stack

- Rust (Edition 2024), Cargo project, license CC-BY-NC-SA-4.0.
- Vanilla JS (ES modules), no framework, no bundler, no npm dependencies.
- Binary custom protocol over WebSocket (Little-Endian, DataView, Float64/Uint32/Uint8).
  Magic bytes 0xCF 0x86 + version byte. Encoder (Rust) and decoder (JS) stay in sync.
- Physical/geodetic constants: Φ, C, WGS84, J2000.

## Nomenclature

- `*.φ` files are project-specific formats.
- Greek identifiers (φ, ω, Φ, τ, Δt) are deliberate variable names — preserve them.
- The system has Oscillators (ring buffers that accumulate) and Membranes (permeability regulation).
- The Aperture (0.0 to 1.0) governs how permeable a membrane is.
- PHI scales all adaptive intervals: tick rates, topology ring sizes, cache TTLs, probe timing.

## Hardware Context

- The machine is a Dell XPS 13 9350 (2016, i5, 8GB RAM, no dedicated GPU).
- Local models run on CPU. Keep tooling lightweight.
- Reference specific files rather than broad repo-wide queries.
