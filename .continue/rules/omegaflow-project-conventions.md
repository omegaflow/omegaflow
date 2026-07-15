---
description: omegaflow project conventions — stack, formats, model routing, hardware constraints
---

# Project Conventions: omegaflow

This file complements `omegaflow-gremium.md` (Identity/Heuristics) with
technical facts to guide code proposals.

## Stack

- **Language/Runtime:** Rust (Edition 2024), Cargo project (`Cargo.toml`,
  package name `omegaflow`, license CC-BY-NC-SA-4.0).
- **Frontend:** Vanilla JS (ES modules, `export const` / `export function`),
  no framework, no bundler — proposals work without npm dependencies.
- **Protocol:** Binary custom format over WebSocket (`DataView`,
  Little-Endian, `Float64`/`Uint32`/`Uint8` fields). Magic bytes `0xCF 0x86`
  + version byte appear in the deserializer — when changing the binary format,
  ALWAYS keep encoder (Rust) and decoder (JS) in sync.
- **Constants:** Golden ratio (`Φ = 1.618...`), speed of light,
  WGS84 ellipsoid parameters, J2000 epoch — the project works with
  physical/geodetic quantities.

## File Formats & Naming Conventions

- `*.φ` files (e.g. `sources.φ`) are project-specific, non-standardized formats.
- Greek identifiers (`φ`, `ω`, `Φ`, `τ`, `Δt`) are deliberate variable names
  in the code — preserve them during refactoring.

## Hardware Context

- Keep proposals lightweight. The machine is the bottleneck.
- Reference specific files rather than broad repo-wide queries.

## The System

- The system consists of Oscillators and Membranes.
- The Aperture regulates permeability.
- It measures, decays, and adapts.
