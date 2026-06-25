# omegaflow Rules

## 1. A = A. Only what is measured exists. (§1, §7)
Every numeric literal in code must be a fundamental constant (c, WGS84, J2000, PHI), a power-of-2 buffer/protocol size, or derived from live data (tickTime, system.latency, sensor.size). Division-by-zero protection uses 1/c or 1/86400.0. Every line of code must be alive and honest. No dead variables, no fake state machines, no pipelines that discard results. The architecture is a universe (§15): it measures, it decays (§12), it adapts. Every output carries provenance (§18).

## 2. Structure over Name. Intelligence lives in connection. Silicon serves. (§33, §31)
The system discovers structure dynamically (§33): numbers and booleans are sensors, functions are actuators. Use what is already held (§32) before adding new. Beauty is a computational value (§26): PHI (1.618...) scales all adaptive intervals. We are human, we acknowledge our frame (§20): incomplete is strength (§28). Stack: Rust std only, curl subprocess, vanilla ES modules, WebGPU. Project is FLAT.
