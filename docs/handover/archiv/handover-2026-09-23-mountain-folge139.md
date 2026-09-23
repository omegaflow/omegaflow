<!--
  title: Handover — Mountain-Folge 139 (Stand 2026-09-23)
  session: Mountain-Folge 139
  class: handover
  date: 2026-09-23
  sha256: f9612bd02d8db284bdf7efd3316d8e96638fd52ab592b00a5dd646f18d9698f5
  status: live
-->
# Handover — Mountain-Folge 139 (2026-09-23)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden **parallel** von Agenten
abgearbeitet. Jeder Punkt trägt **Lage / Blockade / Braucht** und seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-23, Ausführungs-Beginn)

- **HEAD** `7e9ae7af1` == `origin/main` (mycelium folge141). Arbeitsbaum zu Beginn
  sauber; `git_safety --snapshot`: Baum == HEAD, nichts zu sichern.
- **Postfach** — kein `state/mail/mail_ledger.φ`, kein `mail_digest`; `post.md` leer
  von Mountain-Zeilen. Kein offener Eingang.
- **CI** — `te-gate 35767848399` @`7ddd75edb` **cancelled** (00:33Z, kein
  Sweep-Output im Log — die KSG-Messung liegt nicht vor); `ci-check 35796139491`
  @`576dddf98` **failure** — `test`-Job rot: `-p omegaflow-utils --bin archive_search`,
  224 passed / 5 failed; `ci-check 35805438445` @`7e9ae7af1` in_progress.

## Offen (aufgeschlüsselt)

### 1. KSG K-Regel — `TE_KSG_K_PROD` flippen
- **Status:** termin (CI-Lauf) | **Bindung:** eigen
- **Lage:** Apparat in `src/mathematikerin/ksg_k.rs` (`#![cfg(test)]`), Step
  `ksg_k_gate_sweep_and_formula` in `.github/workflows/te-gate.yml:40`; `TE_KSG_K_PROD`
  bleibt **0** (`machines/verdict.rs:12`). Der alte Lauf `35767848399` wurde
  **cancelled**; frischer Lauf `te-gate 35806319936` **dispatcht** @`7e9ae7af1`
  (queued 01:26Z) — die KSG-Messung läuft.
- **Blockade:** Messung liegt noch nicht vor (Lauf queued/running).
- **Braucht:** `ci_manage log 35806319936` **einmal**, sobald der Lauf beendet ist →
  stimmen Formel und Sweep überein, flippt der **Operator** `TE_KSG_K_PROD` auf das
  gemessene k; bei `riss`/`void` bleibt K=0, der Log ist der Befund.

### 2. Ox64-Zweitknoten
- **Status:** wartend | **Bindung:** dritter
- **Lage:** PINE64 hat **zwei** Ox64 SBCs versandt (Ledger `1790046330`, Tracking
  `LZ473049629CN`); physisch nicht angekommen.
- **Blockade:** Geräteankunft.
- **Braucht:** Ankunft abwarten → Bring-up + Kopplung messen.

### 3. ci-check `test`-Job — archive_search 5-Test-Red (geheilt, Confirm offen)
- **Status:** termin (CI-Confirm) | **Bindung:** eigen
- **Lage:** Der `test`-Job war rot auf `-p omegaflow-utils --bin archive_search`
  (224 ok / 5 failed). Fünf unabhängige Logik-Defekte, in diesem Atom geheilt:
  `line_matches` (case-insensitiv lowercaste das Nadelwort nicht,
  `archive_search.rs:1482`), `as_scalar_string` (Nicht-Integer-Zahlen fielen weg,
  `json.rs:37`), `parse_cod` (Record ohne Entry-Feld wurde gerendert, `cod.rs:120`),
  `entrez::term` (Term mit Leerzeichen am ersten Token abgeschnitten, `entrez.rs:17`),
  `register_objstm` (ObjStm-Header-Offsets sind relativ zu `First`, `pdf.rs:204`).
  `cargo check` 0/0 und `cargo check -p omegaflow-utils --tests` 0/0.
- **Blockade:** kein lokaler Testlauf (CI-only); der Confirm steht aus.
- **Braucht:** `ci_manage log <ci-check@diesem Commit>` am nächsten Pass — grün
  schließt den Punkt; bleibt rot, den neuen Test-Ausgang lesen.

## Benchmark

- Punkt 3 (5-Test-Heal) ist Routine-Klasse (Logik-Defekte in `archive_search`), per
  `build`/flash direkt geheilt — kein Doppel-Lauf (die Routine-Klasse ist seit
  2026-09-16 geschlossen, flash-Sieger registriert).

## Geteilter Baum — eigener Pfad-Satz

- `docs/handover/handover-2026-09-23-mountain-folge139.md` (neu)
- Move `handover-2026-09-23-mountain-folge138.md` → `archiv/` (eigene Linie, atomar)
- `tools/utils/src/bin/archive_search.rs` — `line_matches` Nadel-Lowercase
- `tools/utils/src/bin/archive_search/json.rs` — `as_scalar_string` Nicht-Integer
- `tools/utils/src/bin/archive_search/cod.rs` — Entry-Feld-Gate
- `tools/utils/src/bin/archive_search/entrez.rs` — `term=` mit Leerzeichen
- `tools/utils/src/bin/archive_search/pdf.rs` — ObjStm-Offset relativ zu `First`
- `docs/zustand/external-state.md` — CI-Status-Zeile (eigene Messung)
- **Fremd:** `phi/footprints.φ` + `phi/sources.φ` (mycelium, uncommittet — nicht
  berührt, nicht committet).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
