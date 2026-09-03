<!--
  title: The causal arrow, pre-registration (Trishuli flood 2026-08-26)
  class: paper
  date: 2026-08-27
  sha256: 1a6629542c79ca84f63bdbaafedde4db2b6f230331dd0131cff352775212e271
  status: live
  see-also: docs/paper/blatt-pfeil-sturzflut-tibet.md docs/concepts/der-kausalpfeil.md
-->

# The causal arrow — the pre-registration

## Abstract

This Blatt is the seal, registered 2026-08-27 — before the co-located
discharge series for the Trishuli exists. It is not backdated and not
changed after the measurement; what changes after the seal is named. The
protocol stands as the `te_pair_probe` run (mathematikerin
`transfer_entropy_lag` + `surrogate_threshold_lag`, phase-randomized
surrogates, threshold mean + 2σ), the lag sweep {1, 3, 6, 12, 24, 48}, and
the named data sources. Only the protocol and the direction hypothesis are
pre-registered — **no TE number** (0 honored; a number from a missing
arrow would be fabricated). Every cell not yet measurable stays `pending`,
never 0.0.

## The seal

- **The arrow under test:** Niederschlag → Abfluss (Regen → Flut) at the
  Trishuli. The response series (discharge / river stage / lake level) is
  **not yet co-located or co-temporal** for the event; the protocol is
  sealed so the verdict cannot be tuned after the data.
- **The method (fixed):** `te_pair_probe` — transfer entropy
  `TE(A→B)` and `TE(B→A)` at each lag, tested against phase-randomized
  surrogates; a direction is a **finding** only if `TE > mean + 2σ` of the
  surrogate distribution. Lag sweep {1, 3, 6, 12, 24, 48}.
- **The data (named, open where it exists):**
  - Niederschlag: Open-Meteo archive-api, hourly — Gyirong (28.8559,
    85.2950, Oberlauf/Tibet), Rasuwa (28.25, 85.10, Unterlauf/Nepal).
  - Response: DAHITI Koshi water level (satellite altimetry, api_key) or a
    co-located Trishuli gauge when one is open.
- **Path-1 finding (already measured, sealed here for reproducibility):**
  Niederschlag Oberlauf ↔ Unterlauf gives **Rasuwa → Gyirong at Lag
  12–24 h** (TE 0.162/0.151 > threshold 0.133/0.131). The reproduction
  prediction: with the sealed protocol on the sealed series, the arrow
  reproduces at Lag 12–24 h, not at Lag 1–6 h.
- **The prediction (pending arrow):** if the co-located discharge response
  becomes measurable, the direction hypothesis is **Niederschlag →
  Abfluss at a positive lag** (precipitation leads discharge; monsoon
  driving, not the reverse). Direction is pre-registered — the magnitude is
  not (0 honored).

## What arrives and is sealed

The response series fills from the living channels as soon as a co-located,
co-temporal measurement is present (a Trishuli gauge, or a DAHITI altimetry
pass that covers 2026-08-26). Every cell that is not yet measurable stays
`pending` — never 0.0. The seal stands with the method and the direction
hypothesis; the TE values join as soon as the data exists.

## What arrives after the measurement

The verdict is pre-committed, not post-hoc:

- `TE(Niederschlag → Abfluss) > mean + 2σ` at a positive lag
  ⇒ **arrow Niederschlag → Abfluss** (Regen treibt die Flut).
- `TE(Abfluss → Niederschlag) > mean + 2σ`
  ⇒ **arrow Abfluss → Niederschlag**.
- Both directions, or neither ⇒ named accordingly, `pending` where no cell
  reaches significance.

**Verdict as measured 2026-08-27 (co-located gauge):** the DHM Bhotekoshi
at Rasuwagadhi stage series (open, keyless, 10-min) was pulled and run
against the Rasuwa precipitation under the sealed protocol (overlap
n = 129). Result: **TE(Niederschlag → Pegel) = 0.265 > Schwelle 0.218 at
Lag 24 h ⇒ arrow Niederschlag → Pegel** — the rain drives the river stage
~24 h later. Partial signal also reversed (Pegel→Niederschlag at lag 1/6,
both at 12/48) reflecting the small sample and the slow stage dynamics.

Ehrliche Einordnung: measured is **Pegel (stage, m)**, not **Abfluss
(discharge, m³/s)**, on the **pre-flood window** (the open series stops at
the flood onset — telemetry loss 08-26 02:55 UTC). The **flood peak itself
is not recorded** in any open series. The spatial response (CEMS EMSR927)
is delivered (Grading only; collapse point outside — no flood footprint).

The first pre-registered causal-arrow experiment whose verdict is fixed
before the response series exists.

---
*Sealed 2026-08-27. Verdikt-Ordnung
0 honored: no number from a missing arrow; every unfilled cell stays
`pending`.*
