<!--
  title: Handover — Forschung-Folge 140 (Stand 2026-09-21)
  session: Forschung-Folge 140
  class: handover
  date: 2026-09-21
  sha256: fea30d1040d5c0c01067185bb6980cdb9adbc674135ddf5ec81041010a7caac2
  status: live
-->
# Handover — Forschung-Folge 140 (Stand 2026-09-21)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der
eigene Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden **parallel** abgearbeitet.
Jeder Punkt trägt **Lage / Blockade / Braucht** und seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-21, Forschung-Folge 140)

- **HEAD** `123869be` == `origin/main`. Arbeitsbaum trägt fremd uncommittet
  (nicht angefasst): `docs/handover/post.md`, `phi/blocked_sources.φ`,
  `phi/pipeline/ledger.φ`, `phi/sources.φ`, `src/archivar/hdf5.rs`,
  `src/archivar/port.rs`, `AGENTS.md`, `.github/workflows/free-model-bench.yml`,
  `tools/measure/src/bin/free_model_bench.rs`, die Mountain-Handover-Moves.
  Eigener Pfad-Satz dieser Session: `src/mathematikerin/omega.rs`,
  `src/mathematikerin/tests.rs`, `docs/zustand/external-state.md`, das eigene
  Handover + der Move folge139.
- **Postfach** — neuester Ledger-Eingang `1789978555` (Brave Search API
  „usage limit reached", 100 % von $5.00 free credits, September 2026 —
  informativ); unverändert seit folge137/138, kein handlungsbedürftiger Fall.
- **CI** — `hyperscanning-te` `35598791059` @`bdd2cf1e` **in_progress** (Job
  `screen` läuft, `confirm` wartet — browser-verifiziert, kein Ghost-Lock);
  `te-gate` `35595896140` @`44e77da0` **in_progress**. Watchdog-Snapshot
  `/tmp/opencode/ci_status.md`: `ci-check` `35625798504` pending,
  `35625268875` in_progress; frühere `ci-check`-Läufe überwiegend cancelled
  (fremde Linien). Kein Poll.

## Riss — getragen als Naht (unverändert, gemessen 35584758519 @c7cb201f)

Der Riss trägt zwei gemessene Böden, die sich weigern zu konvergieren:
Sheet-Null μ_S = 1.0335e-1 (KSG per-cell, `family_fn_gate`) gegen
Weiß-Treiber-Null μ_W1 = −1.4616e-2 (sd 5.175e-3, p95 −6.005e-3).
μ_S − μ_W1 = 0.1180 > 2σ_W1 = 0.0104 → **Naht**: der Weiß-Treiber-Boden
liegt weit unter dem Sheet-Null — der Riss bleibt sichtbar, nie geglättet.
Die Naht trägt keine Schätzer-Aussage: der stochastic-driver-Arm (KSG τ=2,
excess +18.2 sd) trägt ihn getrennt; der rote Assert ist Fixture-/Gate-Frage
(Punkt 1). Mountain/River (weißer Boden ≈ Surrogat-Boden): **nicht bestätigt**.

## Punkt 1 — `family_fn_gate`: Fix A, CI-Verdikt ausstehend

- **Status:** wartend | **Bindung:** eigen
- **Lage:** Fix A committet (`0f8bc1b2`); `35598791059` @`bdd2cf1e` in_progress.
- **Blockade:** Run-Abschluss (funktionaler Lauf nur in CI).
- **Braucht:** `ci_manage view 35598791059` einmal.

## Punkt 2 — `te-gate` n=1000-FPR-Boden

- **Status:** wartend | **Bindung:** eigen
- **Lage:** `35595896140` @`44e77da0` in_progress; der `te_fn_probe`-Step steht
  (`te-gate.yml:58`).
- **Blockade:** Run-Abschluss.
- **Braucht:** `ci_manage view 35595896140`.

## Punkt 3 — `--dropped` Delta-Gate: Baseline 2286

- **Status:** wartend | **Bindung:** eigen
- **Lage:** Gate lebt in `ci-check.yml:72-98` (`dropped-gate`): baseline 2286
  aus `docs/zustand/dropped-baseline.md`, current = `register_lookup --dropped
  --count`; Schlag nur bei `delta > 0`. Aktuell delta 0.
- **Blockade:** passives Gate — wird erst zur Aktion bei Drift durch einen neuen
  Commit.
- **Braucht:** `ci_manage view <ci-check-id>` beim nächsten Lauf; bei Drift neu
  messen.

## Punkt 4 — reduced TE: Gate-Schwellen messen

- **Status:** wartend | **Bindung:** eigen
- **Lage:** R̂-Normalisierung (Kirkley Eq. 20) committet (`123869be`); die
  Gate-Schwellen (FN > 0.5, FP ≤ 8/30) sind aus der Bias-Analyse (S34/S36)
  gesetzt, nicht gemessen.
- **Blockade:** erster CI-Lauf (`cargo test`); die FN-Zahl (erwartet 20/20) wird
  auf die `M ≤ 0`-Ecke gelesen.
- **Braucht:** `ci-check`-Lauf; bei rotem Gate trägt das Log die gemessene Rate.

## Punkt 5 — Frontalkanäle F3/F4; Takens-Wandzeit

- **Status:** wartend | **Bindung:** eigen
- **Lage:** beide hängen am grünen Screen (`35598791059`, Job `screen` läuft).
- **Blockade:** Run-Abschluss.
- **Braucht:** nach grünem Lauf Wandzeit lesen; F3/F4 getrennt fahren.

## Punkt 6 — KSG↔KDE: Verdrahtung gebaut

- **Status:** wartend | **Bindung:** eigen
- **Lage:** Der Trigger ist **gefeuert** — die Permeability→Radiation-Bindung ist
  gebaut (`356fa616`): TE-Zweig `field_permeability` (`omega.rs:1601-1605`),
  `aperture = field_permeability * tone_scale` (`omega.rs:345`), HRV-Ton-Zweig
  (`tone_code` geschrieben `main_flow.rs:938-939`, gelesen `omega.rs:1637`,
  `tone_scale` relaxiert `omega.rs:1645`), `kinetic_sample = Σω * aperture`
  (`actuators.rs:29`), Test `tests.rs:570`; Spec `radiators.md:104-113` (die
  frühere Zeile „`tone_scale` written nowhere … pending" ist stale).
  Daraufhin die kleinste nicht-fabrizierende Verdrahtung gebaut:
  `topological_te_phase(xs, ys, 3, 3, ring_gen+const)` je HUD-Tick am Anfang von
  `te_probe` (`omega.rs:459-468`), Feld `te_cpu`, HUD-Token `te_cpu {te}/{thr}`
  (`omega.rs:1724`), zwei Tests (`tests.rs` `te_probe_keeps_the_cpu_topology_*`).
  Das Echo ist unverändert — `field_permeability`/`aperture` hängen weiter nur
  am GPU-KDE-Verdikt. `cargo check`/`--tests` 0/0.
- **Blockade:** keine.
- **Braucht:** CI-Lauf grün; dann die CPU-KSG↔GPU-KDE-Drift über Fenster messen
  (die Riss-4-Frage wird damit messbar, nicht geglättet).
- **Naht (fremde Zeile, nicht angefasst):** `post.md` `An mountain: Riss 4`
  (Operator-Wort 2026-09-21 „bauen — KSG als WGSL-Spiegel") weist den
  WGSL-KSG-Pfad neben `te_embedded_kde` (`shaders.rs:483`) der Mountain-Linie zu;
  die CPU-KSG-Verdrahtung hier ist der diagnostische Brückenpfad dazu — erst
  beide Zahlen (CPU-KSG, GPU-KDE) nebeneinander machen die Drift messbar.

## Punkt 6b — topologische FN (Gate-Lücke)

- **Status:** wartend | **Bindung:** eigen
- **Lage:** der topologische Pfad hat FP, Symmetrie und n-Floor, aber **keine
  FN**; `te_fn_probe` druckt die `topo`-Spalte (found/10 bei c=0.9) —
  print-only, kein Gate.
- **Blockade:** die Zahlen stehen in `35595896140` (in_progress); ein Assert
  ohne diese Zahlen wäre eine Schwelle aus Annahme.
- **Braucht:** nach grünem Lauf die `topo`-Spalte lesen; dann
  `calibration_fn_topological_ksg_finds_true_coupling` spiegeln
  (`te.rs:3538`-Muster, `topological_te_phase`) mit an diesen Zahlen
  kalibrierter Schwelle; als ignorierter Step in `te-gate.yml`.

## F2 — flare-Gate-Power (abgeholt von `post.md`)

- **Status:** wartend | **Bindung:** eigen
- **Lage:** die Probe ist `flare_envelope_power_probe` (`te.rs:5872`,
  `#[ignore]`, n ∈ {400,600,1000}, 30 Trials), läuft in `te-gate.yml`.
- **Blockade:** Run `35595896140` (in_progress).
- **Braucht:** die `flare power probe:`-Zeilen aus dem `te-gate`-Log lesen und
  als Handover-Zeile tragen.

## Wartend / operator-gebunden / termin

- Flyby-Path-2-Kette — `termin:2026-09-28` (Auftrag steht, Kanäle live; Zellen ab
  Perigäum).
- NSE/Haug — `wartend`/`dritter` (Route offen, Mail 2026-09-17; Trigger
  Dateieingang).
- BepiColombo MORE — `termin:2027-04` (Freigabe-Anfrage 2026-09-18; kein
  Zwischenzug).

## Postfaltung (folge140)

- `post.md` `An sensory: F2 …` (HEAD) — abgeholt: F2 ist
  `flare_envelope_power_probe` in `te-gate.yml`; als F2-Punkt geführt, Zeile
  gelöscht.
- `post.md` `An sensory: forschung-folge138:113 …` (Arbeitsbaum) — abgeholt:
  die Zeile war doppelt stale (beide Bindungen gebaut, nicht nur TE); in
  Punkt 6 gefaltet, Zeile gelöscht. `post.md` trägt fremd uncommittete Arbeit →
  die eigene Löschung bleibt uncommittet, die besitzende Linie committet die
  Datei.

## Planungs-Tafel (offene Punkte)

| Punkt | Status | Bindung | Lage | Blockade | Braucht |
|---|---|---|---|---|---|
| 1. `family_fn_gate` Fix A | wartend | eigen | `35598791059` in_progress | Run-Abschluss | `ci_manage view 35598791059` |
| 2. `te-gate` n=1000 | wartend | eigen | `35595896140` in_progress | Run-Abschluss | `ci_manage view 35595896140` |
| 3. `--dropped` Baseline | wartend | eigen | delta 0, Baseline 2286 | passives Gate | `ci_manage view <ci-check-id>` |
| 4. reduced TE Schwellen | wartend | eigen | R̂ committet `123869be` | Schwellen ungemessen | `ci-check`-Lauf |
| 5. F3/F4 + Takens | wartend | eigen | — | grüner Screen (←1) | `ci_manage view 35598791059` |
| 6. KSG↔KDE Verdrahtung | wartend | eigen | gebaut (`omega.rs:459-468,1724`) | keine | CI grün → Drift über Fenster |
| 6b. topologische FN | wartend | eigen | Gate-Lücke; `topo` print-only | Zahlen in `35595896140` | `topo`-Spalte lesen |
| F2. flare-Power-Probe | wartend | eigen | `te.rs:5872`, läuft `te-gate.yml` | `35595896140` | `flare power probe:`-Log |
| 8. Flyby-Path-2 | termin:2026-09-28 | termin | Auftrag steht | Datum | Zellen ab Perigäum |
| 9. NSE/Haug | wartend | dritter | Route offen | Dateieingang | Trigger |
| 10. BepiColombo MORE | termin:2027-04 | termin | Anfrage gesendet | Freigabe | Wissenschaftsphase |

## Benchmark

- **Punkt 6 (KSG↔KDE-Verdrahtung, folge140):** `grind-flash` (flash) baute die
  Verdrahtung (`omega.rs`/`tests.rs`), `cargo check`/`--tests` 0/0 — flash-only,
  kein pro/max nötig. Der Bau ist mechanisch (Aufruf einer bestehenden Funktion
  + HUD-Token + zwei Tests), kein hartes Atom.

## Geteilter Baum — eigener Pfad-Satz

- `docs/handover/handover-2026-09-21-forschung-folge140.md` (neu)
- Move `handover-2026-09-21-forschung-folge139.md` → `archiv/` (eigene Linie,
  atomar)
- `src/mathematikerin/omega.rs` (CPU-KSG `te_cpu` in `te_probe` + HUD)
- `src/mathematikerin/tests.rs` (zwei Tests)
- `docs/zustand/external-state.md` (CI-Status-Zeile)
- `docs/handover/post.md` — nur die zwei eigenen `An sensory`-Zeilen gelöscht,
  **nicht** committet (fremd uncommittet)

Fremde uncommittete Arbeit im selben Baum wird **nicht** angefasst.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. Diese Session ändert keinen
Workflow — kein Dispatch nötig; `ci-check` läuft push-getriggert. `/consent` ist
der session-weite Consent (Delegation), nie das Commit-Wort.
