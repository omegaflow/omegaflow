---
description: Reports the session token/cost burn from the opencode DB — the discipline is the session length, not the model.
---

Zeige den Token-/Kosten-Verbrauch aus der opencode-DB. Der Treiber ist die Session-Länge
(Turns × Kontext → `cache_read`), nicht das Modell.

!`cargo run -q -p omegaflow-register --bin session_burn -- --dir omegaflow --top 10`

$ARGUMENTS
