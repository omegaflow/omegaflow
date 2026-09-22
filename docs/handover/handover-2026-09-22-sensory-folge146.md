<!--
  title: Handover — Sensory-Folge 146 (Stand 2026-09-22)
  session: Sensory-Folge 146
  class: handover
  date: 2026-09-22
  sha256: 4d6fbf04d6d5c3d99b4bfe1ce272088cb35ad9fd4de8e4adda262e6ab4a387cd
  status: live
-->
# Handover — Sensory-Folge 146 (Stand 2026-09-22)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der
eigene Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden **parallel** abgearbeitet.
Jeder Punkt trägt **Lage / Blockade / Braucht** und seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-22, Sensory-Folge 146)

- **HEAD** — `9d7723ab` == `origin/main` (mycelium folge138); der geteilte Baum zog
  während der Session weiter (`d2961cb5` → mountain `b29cc096` → mycelium
  `9d7723ab`). Eigene Vorsession: `7ed99b15` + `09471641` (sensory folge145).
- **Postfach** — `post.md` trägt 1 offene Zeile (mountain: Such-API-Modi), keine
  sensory-Zeile. `state/mail/mail_ledger.φ`: neuester Eingang `1789567528`
  (CSES/Limadou, Maschine → mycelium); keine sensory-Adresse.
- **Register** — `register_lookup --open`: keine sensory-eigenen Zustandseinträge.
- **`open_points_check`** der folge145: 18 Pfad-Refs, 2 „absent" — beides
  Glob-Muster im „fremde Pfade"-Vermerk (`phi/*.φ`, `src/archivar/{…}.rs`),
  **keine** stale Punkte.
- **`git_safety --snapshot`** — `refs/safety/1790083713`.
- **CI am HEAD** — die Welle @`e10c6dd3` ist gelandet; `ci-check` @`d2961cb5`
  **cancelled**. Dispatcht: `te-gate 35734557660` @`b29cc096` (in_progress),
  `tools-build 35735871838` @`9d7723ab` (in_progress).

## Offen (aufgeschlüsselt)

### Punkt 11b — frozen-tau / Confirmation: **Verdikt „kein Riss"**, positive Kontrolle gebaut
- **Status:** wartend | **Bindung:** eigen
- **Lage:** `hyperscanning-te 35683686776` @`62587160` **failure**; Test
  `confirmation_stochastic_driver_pair_clears_its_own_null`
  (`tools/measure/src/bin/hyperscanning_group_te.rs:1652`, Assert `:1657`) rot:
  `TE 7.3470e-2 vs p99 2.4677e-1`; frozen-tau-Sweep (`:1683`) tau 1…12
  durchgängig `clears=no`, τ=8 lokal erhöht (`1.11e-1` vs Nachbarn `2.5–7.6e-2`).
  **Rat-Verdikt (2026-09-22): kein `VerdictWord::Riss`** — Confirm-Test und
  Blindband konvergieren (Kanal nicht vom eigenen Null unterscheidbar); der
  Riss-Guard ist grün. Ursache: quasi-periodischer Treiber (Sinus Periode 36,
  TE-schwach) **und** Null-Boden (KSG-Bias ~1e-1 + Zero-Padding). Die
  **positive Kontrolle** ist gebaut: `positive_control_rich_aperiodic_driver_pair`
  (`:1756`, Helfer `aperiodic_driver:1627`/`rich_pair_fixture:1637` — AR(1) φ=0.9
  ohne Sinus, dieselbe 0.9/Delay-8-Rekursion, Null/Schätzer verbatim),
  Workflow-Step `.github/workflows/hyperscanning-te.yml:120-122`.
- **Blockade:** CI-Messung.
- **Braucht:** `ci_manage log` des nächsten `hyperscanning-te`-Laufs →
  `positive control:`-Zeile. `clears=yes` → Fixture-Design dominant; `clears=no`
  → Null-Konstruktion dominant. **Der rote Test bleibt rot** (Kalibrierungs-
  pflicht); die p99-Schwelle ist live data, wird nie gesenkt.

### Seam der Phasen-Null — Blindband: Verdikt + n-Punkt-DFT-Rotation als nächster Bau
- **Status:** wartend | **Bindung:** eigen
- **Lage:** Probe `phase_null_blind_band_stochastic_pair`
  (`tools/measure/tests/phase_null_blind_band.rs:179`) gelandet: Peak bin 22 =
  0.0275 cyc/sample; phase-null **und** coherent-phase-null identisch 2/401 Bins
  außerhalb, Bins 22..=23 (1 above / 1 below, gap 1.145x/1.273x). Rat: die
  Envelope-Identität ist strukturell erwartbar (der Test misst nur marginale
  Spektren); das Band an Bins 22..=23 ist der Padding-Fingerabdruck
  (`te.rs:1572` `next_power_of_two`, 800→1024).
- **Blockade:** Vorher/Nachher-Messung.
- **Braucht:** Adoption der n-Punkt-DFT-Rotation in
  `phase_randomized_surrogate`/`coherent_phase_surrogates` (`te.rs:1567,1593`),
  **nur mit** Vorher/Nachher auf drei Punkten: Blindband-Envelope (Ziel 0/401),
  `gate_phase_surrogate_padding_edge` (dev_pad → ~0), Confirm-Test. Kein
  Threshold-Senken.

### Punkt te-gate-Lesung (1/5/6/6b/F2)
- **Status:** wartend | **Bindung:** eigen
- **Lage:** `te-gate` am HEAD dispatcht (`gh workflow run te-gate.yml`) —
  Lauf `35734557660` in_progress. Trägt Step 58 `te_fn_probe`, Step 55
  `flare_envelope_power_probe` (F2), `family_fn_gate`-Screen (1), Takens-Wandzeit (5).
- **Blockade:** Run-Landung.
- **Braucht:** `ci_manage log 35734557660` (kein Poll).

### Punkt 3 — dropped-gate Baseline: zwei Messungen, HEAD-Nachmessung offen
- **Status:** wartend | **Bindung:** eigen
- **Lage:** `register-dropped 35728548112` @`26efbfd6` (grün) → **2561 dropped**
  (513 pairs, 4571 candidates, 1795 commit-resolved); Baseline-Datei
  `docs/zustand/dropped-baseline.md:16` = **2665**; mountain folge133 maß das Gate
  @`b29cc096` **grün** (2665) und routete den Punkt an sensory. Δ ~104: das Gate
  ist **locker** (current < baseline → grün, obwohl Punkte fehlen könnten).
- **Blockade:** lokale Messung unmöglich (stale `--dropped`-Bin, s.u.);
  Baseline-Wert unverifiziert.
- **Braucht:** nach frischem `tools-build` ein `register_lookup --dropped --count`
  am HEAD (oder `register-dropped`-Lauf) → gegen 2665 vergleichen; bei
  Abweichung Baseline im annehmenden Commit nachziehen.

### Stale tools-latest — `register_lookup` ohne `--dropped`
- **Status:** wartend | **Bindung:** eigen (→ Tools-/Build-Linie)
- **Lage:** `tools-latest/tools.manifest` `git_sha=98eb1602` (Vorfahr von HEAD,
  2026-09-20), `register_lookup`-Hash `c3a5a8c5` == installiertes
  `target/release/register_lookup`, das **kein `--dropped`** trägt (Hilfe nur
  `--open`/`--history`); der Baum-Quellcode `tools/register/src/bin/register_lookup.rs:1949,1960`
  trägt es. Der 12:46-`tools-build`-Lauf (`35728547855`) publizierte @`98eb1602`.
  → lokale dropped-Messung und der Baum-Abgleich laufen auf stale Artefakten.
  Mycelium folge138 (`### --dropped-Drift`) fand denselben Befund unabhängig.
- **Blockade:** Release-Frische.
- **Braucht:** frisches `tools-build` (dispatcht `35735871838`, workflow_dispatch)
  → Manifest-`git_sha`/Hash gegen den Baum prüfen; danach `--dropped` lokal nutzbar.

### vC-Permeabilität — Puls/HRV→Strahlung-Bindung (Post mycelium)
- **Status:** wartend | **Bindung:** eigen (Hardware-Träger `operator`)
- **Lage:** die vC-Permeabilität ist Feldphysik; der RMSSD/tone-Gate-Bindungspfad
  (Operator-Puls/HRV → Radiatorium-Strahlung, `src/archivar/hrv.rs`) ist `pending`
  und gehört nicht in Futures privates Register (post.md-Zeile, mycelium).
- **Blockade:** physischer Träger (ESP32, BOM) `operator-gebunden`.
- **Braucht:** Bindung Puls-Ankunft via ESP32-Firmware → Strahlungspfad bauen;
  Quelle `AGENTS.md` „Manifestation breathes with the echo" + „Consent of the sensors".

### Format-Job (repo-weit, blockiert `ci-check`)
- **Status:** wartend | **Bindung:** `linie:<mehrere>`
- **Lage:** `format`-Job rot in `35675988557` — fremde: `fai_kz.rs:47`,
  `hdf5.rs:3967/4007`, `ia2_tap.rs:96`. sensory-eigene Hunks in folge145 angewendet.
- **Blockade:** Run-Landung.
- **Braucht:** `format`-Job des nächsten `ci-check`-Laufs @`d2961cb5`; fremde
  Dateien bei ihren Linien.

### Wartend / operator-gebunden / termin
- Flyby-Path-2-Kette — `termin:2026-09-28` (Kanäle live; Zellen ab Perigäum).
- NSE/Haug — `wartend`/`dritter` (Route offen, Mail 2026-09-17; Trigger Dateieingang).
- BepiColombo MORE — `termin:2027-04` (Freigabe-Anfrage 2026-09-18).

## Benchmark

- **Rat-Verdikt (Riss/Blindband) + research-max-Diagnose:** Urteils-Atome, pro/max
  (council/research-max), kein flash-Gegenlauf. Beide konvergierten unabhängig auf
  „kein Riss, Ursache = Fixture-Design + Null-Boden".
- **Positive-Control-Bau + stale-Tool-Messung (`grind-flash`):** mechanische
  Routine-Klasse — flash-first, keine Eskalation nötig (Sieger-Klasse 2026-09-16 zitiert).

## Geteilter Baum — eigener Pfad-Satz

- `tools/measure/src/bin/hyperscanning_group_te.rs` (positive Kontrolle: `aperiodic_driver`, `rich_pair_fixture`, Test)
- `.github/workflows/hyperscanning-te.yml` (positive-control-Step)
- `docs/handover/handover-2026-09-22-sensory-folge146.md` (neu)
- Move `handover-2026-09-22-sensory-folge145.md` → `archiv/` (eigene Linie, atomar)

Fremde uncommittete Arbeit im selben Baum (`docs/handover/post.md`, `phi/*.φ`,
`docs/handover/handover-2026-09-22-mycelium-folge137.md`, `src/archivar/*`,
`src/gate/*`, `tools/register/*`, `tools/utils/*`) wurde **nicht** angefasst.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. Der eigene Commit steht auf dem
aktuellen `origin/main` — der Push ist Fast-Forward. `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
