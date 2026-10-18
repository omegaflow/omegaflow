# Formular — F2: Concurrency-Schlüssel von `register-coverage`

**Datum:** 2026-10-18 · **Linie:** future (Leitstelle) · **Register:** `R-2026-10-18-j`
**Status:** offen — der Operator fährt den Rat (`/rat`).

## Die Frage

Soll `register-coverage.yml` auf **per-SHA** (`group: register-coverage-${{ github.sha }}`,
`cancel-in-progress: true`, wie `ci-gate` seit 2026-10-08), auf **per-ref** (`…-${{ github.ref }}`,
`true`) oder auf **Debounce/Batch** (C) umgestellt werden?

## Der gemessene Befund (2026-10-18)

`.github/workflows/register-coverage.yml` **schreibt gar nicht**. Es baut nur und ruft zwei **lesende**
Prüfungen: `:32 register_lookup --orphans --fail`, `:34 register_lookup --tokens --noecho-fail`.
`cancel-in-progress: false` steht in `:20`. **Kein `--write`.**

Der mid-write-Riss trifft also **nicht dieses Gate**, sondern die echten Register-Writer:
`register_ttl_apply.rs:176`, `register_terms_apply.rs:161`, `register_block_remove.rs:107` — alle
`fs::write` **in place** (truncate, kein temp+rename). Kill-Test (50 MB, `kill -9` nach 150 ms):
`INPLACE bytes=2097152 old_intact=no` · `ATOMIC bytes=12 old_intact=yes`.

**Aber:** Der Befund erklärt die **sieben von acht abgebrochenen Läufen in 10 Minuten nicht** —
bei `cancel-in-progress: false` müsste der Abbrecher ein anderer sein. Solange der nicht benannt
ist, ist jede Aussage über cancel-Flags **unbelegt** (Fugu, Runde 2).

## Die Stimmen

| Quelle | Runde 1 | Runde 2 |
|---|---|---|
| Schwarm (24 Sitze) | **A 14** · B 9 · pending 1 | unverändert |
| DeepSeek `#low` | C | **B** |
| Fugu Max High | B | **A** |
| Lumo 2.0 Max | B | *noch offen* |
| Paid-Vorrunde (3:3) | A: GLM · DeepSeek · Lumo · B: Fugu · Mistral · Kimi | — |

## Die Optionen

- **A** — per-SHA + `cancel-in-progress: true` (jeder SHA bekommt sein Urteil).
- **B** — per-ref + `true` (nur der jüngste Lauf lebt).
- **C** — Debounce/Batch statt Concurrency-Umbau.

## Was der Rat entscheiden soll

1. **A, B oder C** — und warum, wenn das Gate selbst **nicht schreibt**?
2. Muss der **Abbrecher** der sieben Läufe zuerst benannt werden, bevor das Verdikt fällt?
3. Gehört der **mid-write-Schutz der echten Writer** (temp+rename) als eigener Punkt in dieselbe
   Entscheidung — oder ist er getrennt zu bauen?

## Riss, der bleibt

- Das Gate ist read-only: die Concurrency-Frage ist **Semantik** (welcher SHA/Ref wird gemessen),
  nicht Korruption.
- Die 7/8-Cancel-Ursache ist **ungemessen** — sie könnte die ganze Debatte verschieben.
- C ist eine **harte Zeitschwelle** und verstößt gegen „keine harten Schwellen".

## An future

Origin: leitstelle-2026-10-18
- `R-2026-10-18-j` löst F2 aus dem alten Sammelbogen `R-2026-10-17-a` (R3) heraus und legt den
  gemessenen Befund daneben. future baut nach dem Verdikt.

## An mountain · river · mycelium · sensory

Origin: leitstelle-2026-10-18
- Die Writer (`register_*_apply`) schreiben in place — ein Kill mid-write kann die Datei trunkieren.
