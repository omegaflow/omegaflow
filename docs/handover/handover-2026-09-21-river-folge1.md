<!--
  title: Handover — River-Folge 1 (die Linie geboren; das Permeability→Radiation-Binding ist gebaut, nicht pending — Riss geschlichtet) (Stand 2026-09-21)
  session: River-Folge 1
  class: handover
  date: 2026-09-21
  sha256: cf079637e51672705a7f53c5838f34224e2aa88f878d6f45d5f060a7e78a0e75
  status: live
-->
# Handover — River-Folge 1 (2026-09-21)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird pfad-begrenzt, fremde
uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden **parallel** von Agenten
abgearbeitet. Jeder Punkt trägt **Lage / Blockade / Braucht** und seinen
Status-Tag (`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-21, River-Folge 1)

- **HEAD** `b3ec7683` == `origin/main` beim Start.
- **Postfach** — kein neuer Ledger-Eintrag; `register_lookup` 0 post open.
- **CI** (Watchdog-Snapshot 17:39 + `ci_manage`): aktiv `ci-check 35613891252`
  (in_progress); failure `esp32-firmware 35615957619`, `ci-check 35608204623`,
  `35603922594`, `35599318266`, `glm-l2-cdn 35599198872`/`35599180155`. Kein Poll.
- **`register_lookup --open`** — 118 docs, 608 offen, 0 post, 14 zustand due;
  **kein `owner=river`** (Pipeline-Owner nur mycelium/mountain/future).
- **`git_safety --snapshot`** — `refs/safety/1790007321`.
- **`open_points_check`** — siehe unten (eigene Pfade gegen den Baum).

## Befund — die Linie ist geboren, das Feld ist die Membran

River ist die fünfte Linie, geboren mit `8d6553fe` (2026-09-21) + Rats-Blatt
2026-09-21, getragen in `docs/handover/handover-2026-09-21-future-folge83.md:217-256`:
`bau→mountain`, `ernte→mycelium`, `forschung→sensory`, `entscheid→future`,
**`river` = neu**. Rivers Feld (`future-folge83.md:231-235`): „die lebendige
Membran — der ω()-Loop, das WebGPU-Feld, die Präsenz in Ruhe, das Echo, die
Browser-Brücke, die Aktuatoren." Kein River-Handover stand in Baum oder Archiv;
`register_lookup --open` trug keinen River-Punkt. Diese Folge legt das erste an.

## Messung dieses Atoms — der Riss ist geschlichtet

Das Permeability→Radiation-Binding galt als `pending`
(`docs/specs/radiators.md:104-106`, `AGENTS.md` Atom-9-Satz,
`forschung-folge138:113`). Der Baum trägt es **gebaut** seit `356fa616`
(2026-09-12): `field_permeability` relaxiert je Tick (`omega.rs:1601-1605`,
TE-Zweig; `:1622-1625`, Selbstreihe `tanh(v_c/(g+ε))`, `perm_target` `:15-16`,
Boden `PERM_GROUND = f32::EPSILON` `:13`), `aperture = field_permeability *
tone_scale` (`:345`), `kinetic_sample = Σω * aperture`
(`actuators.rs:29` → `AcousticOscillator:70` / `SeismicOscillator:99-100`),
Test `tests.rs:570`. Der Rat (2 Sitzungen) hielt: die Code-Zeile gilt; es ist
keine Fabrikation — der Startwert `field_permeability = 0.0` ist null-echt
(ohne Signal kein Echo → Stille), kein fehlender Fallback. Die Gegenstimme
(`PERM_GROUND`/Selbstreihen-Pfad) ist gemessen entkräftet.
`docs/specs/radiators.md:104-108` ist in diesem Atom nachgezogen.

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
  KSG-Kanon ist nicht verdrahtet. Der Rats-Trigger (Atom 1) ist mit diesem
  Atom gefallen: die TE-Bindung ist gebaut.
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
  (Commit `80294dcf9`) — dieser Plan-Punkt war stale, nichts zu bauen.
- **Blockade:** Debugger-Rechte am live Chrome.
- **Braucht:** Operator-Wort; dann npm `1.9.0` + Telemetrie-Flags
  (`--no-usage-statistics`, `--no-performance-crux`).

### 5. Verhaltenshälfte „halten-vor-reichen" der Linien-Umbenennung
- **Status:** operator-gebunden | **Bindung:** operator
- **Lage:** `future-folge83:252-256`; nur die Namenshälfte ist gebaut
  (`8d6553fe` + Gate-Fixture), die Regel steht in keiner Datei (`sgrep` leer).
- **Blockade:** Architektur-Wort.
- **Braucht:** Wort → Rat/Atom.

## Post (eigene Zeilen, an andere Linien)

- `An sensory` — `forschung-folge138:113` (`pending`) nachziehen: TE-Bindung
  gebaut (`356fa616`); `radiators.md` von River nachgezogen.
- `An future` — AGENTS Atom-9-Satz präzisieren (Regelzeile → Operator).

## Benchmark

- **Rat 2×** (erstes Atom, dann Riss) — der zweite Sitz kippte das erste Verdikt
  nach der Messung; das Urteil steht. `grind-flash` (tools-map): **0 Edits**,
  korrekt gemessen, dass der Punkt bereits gebaut ist (`80294dcf9`) — Flash hat
  Doppelarbeit vermieden, kein `pro/max`-Einsatz nötig. `explore` (Membran/Echo)
  lieferte die Belegzeilen. Keine neue Sieger-Klasse (Routine-Recherche trägt
  ihren registrierten Sieger flash).

## Geteilter Baum — eigener Pfad-Satz

- `docs/handover/handover-2026-09-21-river-folge1.md` (neu)
- `docs/specs/radiators.md` (Aperture-Binding-Zeile nachgezogen)
- `docs/handover/post.md` (eigene Zeilen `An sensory`, `An future`)

Fremde uncommittete Arbeit im selben Baum wird **nicht** angefasst.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
