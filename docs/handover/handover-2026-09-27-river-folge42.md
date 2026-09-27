<!--
  title: Handover — River-Folge 42 (2026-09-27)
  session: River-Folge 42
  class: handover
  date: 2026-09-27
  sha256: c2c07ba692305a0fe80afa1863c681234ed29f29ef1696445955188ae10e2421
  status: live
-->
# Handover — River-Folge 42 (2026-09-27)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt, was
gemacht wurde. Nur eigene Arbeit: pfad-begrenzter Commit, fremde uncommittete
Arbeit unangetastet; gepusht wird, sobald `origin/main` Vorfahr von HEAD ist.
Sortierung umsetzbar → nicht umsetzbar; kein Rang. Jeder Punkt trägt **Trigger** /
**Lage** (mit Messstempel) / **Blockade** / **Braucht**.

Diese Session konsumierte `handover-2026-09-27-river-folge41.md` (nach `archiv/`).

## Wort-Register
- gic-Paper-Einreichung: höchste Priorität | 2026-09-27 | Operator-Wort.
- RX100-K-Beschaffung: kein Kauf vor Förderung | 2026-09-27 | Operator-Wort (Träger `mountain-folge176.md:32`).
- Geräte-Zugriff: vor jedem Zugriff fragen (adb/BT) | 2026-09-26 | Operator-Wort.
- Harte-Läufe-LOCK aufgehoben | 2026-09-26 | Operator-Wort.
- HTTPS ja | 2026-09-26 | Operator-Wort folge36.
- vC 945 von Mantis Shrimp getrennt | 2026-09-26 | Operator-Wort folge36.
- Einzelbefehle liefern | 2026-09-26 | Operator-Wort folge36.
- Mantis Shrimp LOCK | 2026-09-26 | Operator-Wort folge36.

## Verweise (Prosa mit offenen Markern)
- `docs/surveys/survey-2026-09-23-geraete-anbindung-radiatoren.md` — Geräte-Inventare, gebaute Atome, offene Messpunkte (Z. 346-352).
- `docs/surveys/survey-2026-09-20-browser-anbindung.md` — vier Browser-Pfade.
- `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md` — presence-only Ladearchitektur.
- `docs/auftrag/auftrag-gic-einreichung.md` — Einreich-Paket (Träger des gic-Punkts).
- `docs/paper/gic-causal-driver.md` — gic-Papier (Träger des gic-Punkts; 2 `pending`-Platzhalter Repo-DOI/Software-URL).
- `docs/auftrag/auftrag-flyby2-kette.md` — Flyby-Path-2-Kette (Trigger/Fill-Run).

## Offen

Der gemessene Rundenzustand: `state/zustand/standing-pass.md` (zitieren, nie kopieren). Operator-Akte dieser Runde liegen in der Future-Operator-Queue; Dritt-Waits in `state/zustand/wartend.φ`.

### Flyby-Path-2 — Füll-Lauf
- **Status:** termin:2026-09-28 | **Bindung:** termin
- **Trigger:** Perigäum 2026-09-28 **11:45 UTC** (gemessen 2026-09-27 via JPL Horizons `-28` + ESA-Kanal).
- **Lage:** (gemessen 2026-09-27 via `cargo build`) Fill-Bin `tools/measure/src/bin/flyby_path2_fill.rs` gebaut (0 Warnungen), Tube ±12 h aus gesiegeltem Arc; Kanäle RTSW/Kp GFZ/Swarm/OMNI2/ACE; Perigäum-Zelle 13, alle Zellen `pending` (ehrlicher Vor-Flyby-Zustand).
- **Blockade:** kein Workflow registriert — der Lauf muss zum Zeitpunkt erfolgen.
- **Braucht:** RTSW-Snapshots ~28.09. 12:00 + 29.09. 00:00 UTC (`curl -sSfL …/rtsw_mag_1m.json|rtsw_wind_1m.json` → `data/services.swpc.noaa.gov/`), dann `cargo run -p omegaflow-measure --bin flyby_path2_fill -- --flyby juice --snapshots data/services.swpc.noaa.gov/`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der session-weite Consent, nie das Commit-Wort.
