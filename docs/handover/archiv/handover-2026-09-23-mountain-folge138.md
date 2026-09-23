<!--
  title: Handover — Mountain-Folge 138 (Stand 2026-09-23)
  session: Mountain-Folge 138
  class: handover
  date: 2026-09-23
  sha256: 20a81ac6db1f3e5ca09b3c837d41ea911f1fd53d7cf6fe26344abd44c676965f
  status: live
-->
# Handover — Mountain-Folge 138 (2026-09-23)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden **parallel** von Agenten
abgearbeitet. Jeder Punkt trägt **Lage / Blockade / Braucht** und seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-23, Ausführungs-Beginn)

- **HEAD** `eea5867e1` == `origin/main` (sensory folge150). Arbeitsbaum zu Beginn
  sauber; `git_safety --snapshot`: Baum == HEAD, nichts zu sichern.
- **Postfach** — kein `state/mail/mail_ledger.φ`, kein `mail_digest`. Kein offener
  Eingang. `post.md` trug zwei an mountain geroutete Zeilen
  (`path_reference_scan` rot, `dropped-gate` rot) — beide in diesem Atom geheilt,
  Zeilen abgeholt und gelöscht.
- **CI** — Watchdog-Snapshot 2026-09-23T00:17: `te-gate 35767848399`
  **in_progress** @`7ddd75edb` (seit 18:32Z, seither kein Update); `ci-check
  35767837058` failure @`7ddd75edb` — die beiden mountain-Reds. Nach den Fixes wird
  `ci-check` neu dispatcht (mit dem Commit).

## Offen (aufgeschlüsselt)

### 1. KSG K-Regel — `TE_KSG_K_PROD` flippen
- **Status:** termin (CI-Lauf) | **Bindung:** eigen
- **Lage:** Apparat gebaut in `src/mathematikerin/ksg_k.rs` (`#![cfg(test)]`):
  Formelpfad `formula_k(n,d,τ)`, Sweep `run_ksg_k_sweep` k=1..=12 gegen FPR/FN,
  Parity-Gate bei k=4; Test `ksg_k_gate_sweep_and_formula` als Step in
  `.github/workflows/te-gate.yml`. `TE_KSG_K_PROD` bleibt **0**
  (`machines/verdict.rs:12`) — kein K fabriziert, solange die Messung fehlt.
- **Blockade:** der Lauf `te-gate 35767848399` @`7ddd75edb` ist **in_progress**,
  seit 18:32Z ohne Update (>6 h) — die Messung liegt noch nicht vor.
- **Braucht:** `ci_manage log 35767848399` **einmal**, sobald der Lauf beendet ist
  → stimmen Formel und Sweep überein, flippt der **Operator** `TE_KSG_K_PROD` auf
  das gemessene k; bei `riss`/`void` bleibt K=0, der Log ist der Befund. Hängt der
  Lauf (Ghost-Lock/Queue), entscheidet der Watchdog (2×-Median) oder ein frischer
  `te-gate`-Dispatch am aktuellen HEAD.

### 2. Ox64-Zweitknoten
- **Status:** wartend | **Bindung:** dritter
- **Lage:** PINE64 hat **zwei** Ox64 SBCs versandt (Ledger `1790046330`, Tracking
  `LZ473049629CN`); physisch nicht angekommen.
- **Blockade:** Geräteankunft.
- **Braucht:** Ankunft abwarten → Bring-up + Kopplung messen.

## Benchmark

- M3 (`path_reference_scan`) + M4 (`dropped-gate`) per `grind-flash` geheilt —
  Routine-Klasse (Scanner-Skip + Baseline-Bump), der flash-Sieger ist registriert
  (2026-09-16, identisches Ergebnis 2,4–11× günstiger); kein Doppel-Lauf.

## Geteilter Baum — eigener Pfad-Satz

- `docs/handover/handover-2026-09-23-mountain-folge138.md` (neu)
- Move `handover-2026-09-22-mountain-folge137.md` → `archiv/` (eigene Linie, atomar)
- `docs/handover/post.md` — die zwei abgeholten `An mountain:`-Zeilen entfernt
- `tools/register/src/bin/path_reference_scan.rs` — M3-Skip-Klassen + Tests
- `docs/zustand/dropped-baseline.md` — M4-Baseline am annehmenden Commit
- **Fremd:** nichts uncommittet im Baum (die measure-Reste sind in `eea5867e1`
  committet).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
