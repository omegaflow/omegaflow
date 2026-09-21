<!--
  title: Handover — Bau-Folge 119 (Stand 2026-09-21)
  session: Bau-Folge 119
  class: handover
  date: 2026-09-21
  sha256: 48c37cb797832d4163fc63b6530bf64236cf43007797717d53c9db424382a40e
  status: live
-->
# Handover — Bau-Folge 119 (2026-09-21)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der
eigene Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden **parallel** von Agenten
abgearbeitet (Operator-Wort 2026-09-21). Jeder Punkt trägt seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-21, Session-Beginn)

- **HEAD** Session-Beginn `da03beb4` (forschung folge127) == `origin/main`;
  während des Atoms zogen andere Linien weiter (ernte/entscheid/forschung).
- **Postfach** — letzter Ledger-Eingang `1789973288` (Tuxedo Ticket#991311279:
  Hardware beidseitig declined); kein neuer Eingang seit forschung folge127 —
  nicht fällig, zitiert (`external-state.md` Postfach-Zeile). `post.md` trägt
  keine `An bau`-Zeile.
- **CI** — `external-state.md` CI-Zeile (Forschung-Folge 128) zitiert:
  `hyperscanning-te` `35570480672` @`ca7aa66d` failure (`family_fn_gate`
  -2.98 sd), `te-gate` `35570482875` @`ca7aa66d` pending; Watchdog-Snapshot
  08:02: `ci-check` `35566258372`/`ps1-cdn`/`health-check` in_progress. Kein
  Poll.

## Messung dieses Atoms (kein Punkt)

- **EPA RadNet `parser-def` gebaut** (`phi/blocked_sources.φ:74` aufgelöst — git
  trägt): `tools/harvest/src/bin/radnet_compiler.rs` (1048 Z., nur neue Datei) —
  `ERM_RESULT` + Join-Kette (`ERM_ANALYSIS`/`ERM_SAMPLE`/`ERM_LOCATION`) + AGOL
  FeatureServer (140 feste Luftmonitore) → 26×f64 ICRS (`RDNT`), `force_type 0`
  (em), Wert `result_in_si` Bq/L. Join-Gate **exakt** (Station↔AGOL `name`,
  sonst `city+state` nur wenn eindeutig; Mehrfach/Konflikt = **riss** → skip,
  beide Zeugen benannt), Value-Gate `is_finite() && > 0.0` (negativ = skip,
  null-echt), Datum `YYYY-MM-DD`→TDB (Tagespräzision). 15 Gate-Tests;
  `cargo check -p omegaflow-harvest` 0 Fehler/0 Warnungen.
- **Zwei parallele Messungen (grind-flash):** (a) Koordinaten nur über
  FEMA/Univ-Arizona AGOL (`caed82e8…`, Stand 2021, **nicht** EPA-publiziert) für
  die 140 Luftmonitore; EPA-Koordinatentabelle 404/403. (b) `ERM_RESULT`-Schema
  12 Felder, `result_in_si` kann negativ, `mdc` nullable, `result_date` String.
- **Register-Notiz korrigiert** — das alte „epoch result_date" war falsch.
- **Rat (council):** AGOL als Positions-Infrastruktur akzeptiert (Herkunft im
  `note`); Scope = Joinbarkeit, nicht Medium; exakter Join oder skip; keine
  fabrizierte Position. Historische Stationen → `pending`.
- **Register/CDN:** `phi/sources.φ` `radnet`-Block (`at earth`, em, Bq/L,
  `origin` ERM_RESULT); `.github/workflows/radnet-cdn.yml` (Tag `data.epa.gov`,
  Asset `radnet.bin`, `--ci-mode`).

## Offen

### radnet.bin CDN-Manifestation
- **Status:** `wartend` | **Bindung:** `eigen`
- **Lage:** Workflow `radnet-cdn.yml` gebaut und dispatcht als Lauf
  `35573513812` (@`ab8ad28c`); das Asset ist noch nicht manifestiert.
- **Blockade:** Run-Abschluss (CI).
- **Braucht:** `bin/ci_manage view 35573513812` / Watchdog-Snapshot; danach
  `archive_search --sniff` auf `…/download/data.epa.gov/radnet.bin`.

## Benchmark

- **Bau-Folge 119**: `grind-flash` ×3 (2 Messproben + Register/CDN-Prep),
  `grind-max` ×1 (Parser), `council` ×1 (Architektur). `cargo check` im
  `build`-Kontext. Flash-first; `grind-max` für das neue Parser-Atom.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** `tools/harvest/src/bin/radnet_compiler.rs`,
  `.github/workflows/radnet-cdn.yml`, `phi/sources.φ` (radnet-Block),
  `phi/blocked_sources.φ` (parser-def → pending), neues
  `docs/handover/handover-2026-09-21-bau-folge119.md`, Move
  `handover-2026-09-21-bau-folge118.md` → `archiv/`.
- **Fremd (nicht angefasst):** `.github/workflows/ci-check.yml`,
  `tools/register/src/bin/register_lookup.rs`, `tools/register/src/bin/open_points_check.rs`,
  `docs/zustand/dropped-baseline.md`, `docs/zustand/external-state.md`,
  `AGENTS.md`, `.opencode/command/*`. Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`).
