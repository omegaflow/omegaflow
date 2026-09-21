<!--
  title: Handover — River-Folge 2 (Stand 2026-09-21)
  session: River-Folge 2
  class: handover
  date: 2026-09-21
  sha256: 8548d73c44dce6e63dbafa7185d5575c89aa38c8645363c481f56ed3afa54411
  status: live
-->
# Handover — River-Folge 2 (2026-09-21)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird pfad-begrenzt, fremde
uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden **parallel** von Agenten
abgearbeitet. Jeder Punkt trägt **Lage / Blockade / Braucht** und seinen
Status-Tag (`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-21, River-Folge 2)

- **HEAD** Start `938095ef`; während der Session zog `origin/main` auf
  `1ceb986c` (future folge84, Vorfahr von `938095ef`) — HEAD == `origin/main`
  beim Abschluss.
- **Postfach** — Zustand-Eintrag `docs/zustand/external-state.md` (Postfach
  (extern)): **nicht fällig**, kein neuer Ledger-Eingang; `state/mail/mail_ledger.φ`
  letzte Zeile `1789978555` (117 Zeilen, unverändert). Keine eingehende Zeile an
  River in `post.md` (`register_lookup` 0 post an river).
- **CI** — Zustand-Eintrag (CI-Status) nachgezogen: Läufe @`1ceb986c`
  `ci-check` `35624872327` pending, `tools-build` `35624872460` in_progress;
  frühere `ci-check` `35624560120` cancelled (überholt), `glm-l2-cdn`
  `35623731418` success. Kein Poll.
- **`register_lookup --open`** — 119 docs, 620 offen, **kein `owner=river`**.
- **`git_safety --snapshot`** — `refs/safety/1790007426`.
- **`open_points_check`** — 9 Pfad-Refs, **0 absent** (kein stale Pfad).

## Offen (aufgeschlüsselt)

### 1. AGENTS Atom-9-Satz präzisieren (Riss-Nachlauf)
- **Status:** operator-gebunden | **Bindung:** operator (Regelzeile)
- **Lage:** `AGENTS.md` („Manifestation breathes with the echo"): „the actuators
  radiate the raw field (Σω, no modulation) — the permeability's radiation
  binding is `pending`" widerspricht dem Baum (Σω × aperture, `356fa616`).
  Zwei Bindungen heißen „permeability": die TE-Bindung (gebaut) und die
  HRV-Ton-Bindung (`tone_scale`, gelesen `omega.rs:345`, nirgends geschrieben —
  echt pending).
- **Blockade:** Regelzeile, kein stiller Eingriff.
- **Braucht:** Operator-Wort zur Präzision: „Σω scaled by the TE aperture
  (built); the HRV tone binding pending" (Post `An future` steht).

### 2. Riss 4 — KSG↔KDE, Echo-Verdrahtung
- **Status:** operator-gebunden | **Bindung:** operator (Bau-Wort)
- **Lage:** `forschung-folge138:101-118`; der Echo-Pfad `omega.rs:1563` →
  `te_probe` → WGSL `te_compute` → `te_embedded_kde` trinkt nur GPU-KDE; der
  KSG-Kanon ist nicht verdrahtet.
- **Blockade:** Operator-Entscheid bauen/descopen.
- **Braucht:** Wort „bauen" → kleinste nicht-fabrizierende Verdrahtung
  (`topological_te_phase` je HUD-Tick 1 Hz, m ≤ 256, CPU-KSG neben GPU-KDE,
  beide Zahlen in `te_say`/HUD); bei „nein" → `descoped` mit Befund.

### 3. vC-Permeabilität — Vollzug (hidden run)
- **Status:** termin | **Bindung:** operator (Maschine)
- **Lage:** `future-folge83:182-186`; der versteckte sensor-getriebene Lauf wartet.
- **Blockade:** Operator-Maschine.
- **Braucht:** Wort für den hidden Lauf (`OMEGAFLOW_HIDDEN=1`).

### 4. Browser-Brücke — Chrome DevTools MCP pinnen
- **Status:** operator-gebunden | **Bindung:** operator
- **Lage:** `survey-2026-09-20-browser-anbindung.md:141-144`; die
  Drei-Pfade-Tabelle steht bereits in `docs/concepts/tools-map.md:259-268`
  (Commit `80294dcf9`) — der Plan-Punkt war stale, nichts zu bauen; offen bleibt
  nur das Pinnen.
- **Blockade:** Debugger-Rechte am live Chrome.
- **Braucht:** Operator-Wort; dann npm `1.9.0` + Telemetrie-Flags
  (`--no-usage-statistics`, `--no-performance-crux`).

### 5. Verhaltenshälfte „halten-vor-reichen" der Linien-Umbenennung
- **Status:** operator-gebunden | **Bindung:** operator
- **Lage:** `future-folge83:252-256`; nur die Namenshälfte ist gebaut
  (`8d6553fe` + Gate-Fixture), die Regel steht in keiner Datei (`sgrep` leer).
- **Blockade:** Architektur-Wort.
- **Braucht:** Wort → Rat/Atom.

## Benchmark

- Keine Delegation in diesem Atom — alle fünf Punkte sind `operator-gebunden`
  oder `termin`, kein Agent trägt einen abarbeitbaren Schritt. Kein neuer
  Sieger; die registrierten Sieger (`grind-flash` Routine, `Rat` Architektur)
  stehen unverändert.

## Geteilter Baum — eigener Pfad-Satz

- `docs/handover/handover-2026-09-21-river-folge2.md` (neu)
- `docs/handover/archiv/handover-2026-09-21-river-folge1.md` (verschoben)
- `docs/zustand/external-state.md` (CI-Status-Zeile nachgezogen)

Fremde uncommittete Arbeit im selben Baum wird **nicht** angefasst
(`phi/blocked_sources.φ`, `phi/pipeline/ledger.φ`, `phi/sources.φ`,
`src/archivar/hdf5.rs`, `src/mathematikerin/te.rs`).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
