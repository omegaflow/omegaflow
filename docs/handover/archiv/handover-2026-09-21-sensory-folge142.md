<!--
  title: Handover — Sensory-Folge 142 (Stand 2026-09-21)
  session: Sensory-Folge 142
  class: handover
  date: 2026-09-21
  sha256: b5f92d2c672935e326083aea7655ec02ea0afb7990ba0ac44650f6621581b305
  status: live
-->
# Handover — Sensory-Folge 142 (Stand 2026-09-21)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der
eigene Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden **parallel** abgearbeitet.
Jeder Punkt trägt **Lage / Blockade / Braucht** und seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-21, Sensory-Folge 142)

- **HEAD** — Session-Beginn `af25d752` (fmt-Atom, fremde Linie); beim Pass
  `bf447c4f` (River-Folge 5), `origin/main` = `bf447c4f`. Der eigene Commit steht
  damit auf `origin/main` (Fast-Forward).
- **Postfach** — kein neuer Ledger-Eingang; `state/mail/mail_ledger.φ` unverändert
  (jüngster `1789978555`, Brave-Limit); `external-state.md`-Eintrag jünger als
  2⁶ min, zitiert, nicht neu gemessen. `post.md` trägt keine Sensory-Zeile.
- **CI** (einmalig via `ci_manage`, kein Poll) — `hyperscanning-te` `35598791059`
  @`bdd2cf1e` **in_progress** (ghost: updated_at vor dem Job-Ende): Job `confirm`
  **failure** (Log vollständig), Job `screen` läuft noch (Log HTTP 404).
  `te-gate` `35628669014` @`3e319087` **in_progress**, kein Job-Log (404).
  `ci-check` `35647323573` @`7c2d78ff` **pending**, 0 Jobs. Kein abgeschlossener
  `ci-check` am HEAD.

## Riss — getragen als Naht (unverändert, gemessen 35584758519 @c7cb201f)

Der Riss trägt zwei gemessene Böden, die sich weigern zu konvergieren:
Sheet-Null μ_S = 1.0335e-1 (KSG per-cell, `family_fn_gate`) gegen
Weiß-Treiber-Null μ_W1 = −1.4616e-2 (sd 5.175e-3, p95 −6.005e-3).
μ_S − μ_W1 = 0.1180 > 2σ_W1 = 0.0104 → **Naht**: der Weiß-Treiber-Boden
liegt weit unter dem Sheet-Null — der Riss bleibt sichtbar, nie geglättet.
Die Naht trägt keine Schätzer-Aussage: der stochastic-driver-Arm (KSG τ=2,
excess +18.2 sd) trägt ihn getrennt. Mountain/River (weißer Boden ≈
Surrogat-Boden): **nicht bestätigt**.

## Punkt 1 — `family_fn_gate`: Fix A, screen-Verdikt ausstehend

- **Status:** wartend | **Bindung:** eigen
- **Lage:** Fix A committet (`0f8bc1b2`); Lauf `35598791059` @`bdd2cf1e`
  in_progress (ghost). Der Job `confirm` ist rot, aber das ist der geteilte
  confirmation-Test (Punkt 11), **nicht** `family_fn_gate`; der Job `screen`
  (der `family_fn_gate`/`fn_gate_sweep`/`self_null_discriminator` fährt) hat
  noch kein Log (HTTP 404) → Verdikt ungemessen.
- **Blockade:** screen-Job-Log.
- **Braucht:** `ci_manage log 35598791059 --all` erneut, sobald der `screen`-Job
  gelandet ist (kein Poll).

## Punkt 11 — confirmation-Test: geteilt in stochastisches Gate + Riss-Wächter (gebaut)

- **Status:** wartend | **Bindung:** eigen
- **Lage:** Der Job `confirm` (`35598791059` @`bdd2cf1e`) war rot:
  `confirmation_confirms_the_strong_pair_against_its_own_null` →
  `the confirmation wiring confirms the strong pair against its own per-cell null:
  TE -1.4877e-14 vs p99 2.8908e-1`. Rat (2026-09-21): **genuine Riss, kein
  Verdrahtungsdefekt** — Richtung (x=Ziel b, y=Trieb a), KSG-Formel und
  frozen-tau-Null-Design im Code geprüft sauber; der deterministische Treiber
  (`ar1_sine` Rauschen 0.0) trägt keinen exklusiven Kanal (das Ziel-vergangen
  bestimmt die Zukunft, `TE ≈ 0` ist die degenerierte deterministische
  Zähl-Regime), p99 0.289 ist die Naht. Entscheid: p99-Phasen-Null nur für die
  stochastische Klasse; die deterministische Naht als **harter Regressions-Wächter**;
  Kirkley-Adoption **vertagt** (R̂ `123869be` ungemessen — A = A). Gebaut: Test
  geteilt in `confirmation_stochastic_driver_pair_clears_its_own_null`
  (Treiber-Rauschen 0.05, `assert TE > p99`) und
  `riss_guard_deterministic_pair_measures_below_its_own_null` (`assert TE < p1`,
  te/p1/p99 im Text); `hyperscanning-te.yml`-Job `confirm` ruft beide.
  `cargo check -p omegaflow-measure --bin hyperscanning_group_te --tests` 0/0.
- **Blockade:** der neue `confirm`-Job ist noch nicht gelaufen.
- **Braucht:** frischer `hyperscanning-te`-Lauf; dann `ci_manage log <id>` —
  beide grün = stochastisches Gate trägt und die Naht hält; stochastisch rot =
  Schätzer-Defekt, dann frozen-tau-Delay-Sweep 1..=12 als erste Messung.

## Punkt 3 — `--dropped` Delta-Gate: Clean-Messung ausstehend

- **Status:** wartend | **Bindung:** eigen
- **Lage:** `register_lookup --dropped --count` = **2440** am Arbeitsbaum gegen
  Baseline 2286 (`docs/zustand/dropped-baseline.md`), also **delta 154**; der
  Gate-Anker ist der letzte **clean** gemessene Wert: `dropped-gate`-Job
  success in `ci-check` `35613891252` @`ddf7e9f2` → `baseline 2286 | current
  2283 | delta -3`. Der +157 seit `ddf7e9f2` stammt aus den seither archivierten
  Handovers (forschung 139→140, mountain 126→127, ernte 133→134, future 84→85)
  und ist teils vom fremd uncommitteten Ernte-Move im Arbeitsbaum beeinflusst —
  der reine Commit-Wert ist ungemessen.
- **Blockade:** kein abgeschlossener `ci-check` am HEAD.
- **Braucht:** die `dropped-gate: baseline … | current … | delta …`-Zeile des
  nächsten abgeschlossenen `ci-check` (clean tree) lesen; bei delta > 0 den
  annehmenden Commit identifizieren und die Baseline dort nachziehen — **nie
  blind** auf den Arbeitsbaum-Wert.

## Punkt 4 — reduced TE: R̂-Commit ungemessen

- **Status:** wartend | **Bindung:** eigen
- **Lage:** R̂-Normalisierung (Kirkley Eq. 20) committet (`123869be`). Die vier
  reduced-TE-Gate-Tests (`gate_fn_reduced_te_mdl_coupled_ar1_flows`,
  `gate_fp_reduced_te_mdl_independent_ar1_stays_silent`,
  `gate_n_floor_reduced_te_table_size`,
  `gate_symmetry_reduced_te_identical_series_measure_equally`) sind **ok** in
  `ci-check` `35608204623` @`8d6553fe` und `35613891252` @`ddf7e9f2` — beide
  **vor** der R̂-Normierung. Der R̂-Commit selbst hat **keinen** Testlauf
  (`35625598394` @`123869be` cancelled, 0 Jobs). Die exakten FN-/FP-Zahlen
  stehen nur in den Panik-Zweigen (20/20 bzw. 30/8) und werden bei `ok` nie
  gedruckt.
- **Blockade:** kein abgeschlossener Test-Lauf am R̂-Code.
- **Braucht:** `ci-check` am HEAD lesen; die vier reduced-TE-Zeilen aus dem
  `test`-Job tragen (grün = Schwellen halten; rot = Panik-Text nennt die
  gemessene Rate). Der Kirkley-Entscheid (Punkt 11) landet auf diesen Zahlen.

## Punkt 5 — Frontalkanäle F3/F4; Takens-Wandzeit

- **Status:** wartend | **Bindung:** eigen
- **Lage:** beide hängen am grünen Screen (`35598791059`, Job `screen` ohne Log).
- **Blockade:** screen-Job-Abschluss.
- **Braucht:** nach grünem Lauf Wandzeit lesen; F3/F4 getrennt fahren.

## Punkt 6 — KSG↔KDE: Verdrahtung gebaut, Drift-Messung wartet auf den Lauf

- **Status:** wartend | **Bindung:** eigen
- **Lage:** Der Trigger ist gefeuert — Permeability→Radiation gebaut (`356fa616`):
  TE-Zweig `field_permeability` (`omega.rs:1601-1605`), `aperture =
  field_permeability * tone_scale` (`omega.rs:345`), HRV-Ton-Zweig
  (`main_flow.rs:938-939`, gelesen `omega.rs:1637`, relaxiert `omega.rs:1645`),
  `kinetic_sample = Σω * aperture` (`actuators.rs:29`), Test `tests.rs:570`.
  Die CPU-KSG-Verdrahtung in `te_probe` steht (`omega.rs:459-468`), Feld
  `te_cpu`, HUD-Token `te_cpu {te}/{thr}` (`omega.rs:1724`), zwei Tests
  (`tests.rs`); `cargo check`/`--tests` 0/0.
- **Blockade:** der `te_fn_probe`-Step 58 (`te_scal`/`te_topo`) läuft erst im
  `te-gate` `35628669014` (kein Log, 404).
- **Braucht:** nach Abschluss die `te_scal`/`te_topo`-Zahlen aus dem Lauf lesen
  → CPU-KSG↔GPU-KDE-Drift über Fenster (die Riss-4-Frage wird messbar).
- **Naht (fremde Zeile):** `post.md` `An mountain: Riss 4` (Operator-Wort
  2026-09-21 „bauen — KSG als WGSL-Spiegel") weist den WGSL-KSG-Pfad neben
  `te_embedded_kde` (`shaders.rs:483`) der Mountain-Linie zu.

## Punkt 6b — topologische FN (Gate-Lücke)

- **Status:** wartend | **Bindung:** eigen
- **Lage:** der topologische Pfad hat FP, Symmetrie und n-Floor, aber **keine
  FN**; `te_fn_probe` druckt die `topo`-Spalte (found/10 bei c=0.9) —
  print-only, kein Gate.
- **Blockade:** die Zahlen stehen erst in `te-gate` `35628669014` (Step 58).
- **Braucht:** nach Abschluss die `topo`-Spalte lesen; dann
  `calibration_fn_topological_ksg_finds_true_coupling` spiegeln
  (`te.rs:3538`-Muster, `topological_te_phase`) mit an diesen Zahlen
  kalibrierter Schwelle; als ignorierter Step in `te-gate.yml`.

## F2 — flare-Gate-Power (abgeholt von `post.md`)

- **Status:** wartend | **Bindung:** eigen
- **Lage:** die Probe ist `flare_envelope_power_probe` (`te.rs:5872`,
  `#[ignore]`, n ∈ {400,600,1000}, 30 Trials), Step 55 in `te-gate.yml`.
  `35595896140` brach am **vorgelagerten** Gate `flare_envelope_conditional_keeps_true_coupling`
  (Step 53) ab, ohne Output — ob das Gate rot war oder der Runner starb, ist
  ungemessen.
- **Blockade:** neuer Lauf `35628669014` (in_progress, kein Log).
- **Braucht:** die `flare power probe:`-Zeilen **und** das Verdikt des
  `flare_envelope_conditional`-Steps aus `35628669014` lesen.

## Wartend / operator-gebunden / termin

- Flyby-Path-2-Kette — `termin:2026-09-28` (Auftrag steht, Kanäle live; Zellen ab
  Perigäum).
- NSE/Haug — `wartend`/`dritter` (Route offen, Mail 2026-09-17; Trigger
  Dateieingang).
- BepiColombo MORE — `termin:2027-04` (Freigabe-Anfrage 2026-09-18; kein
  Zwischenzug).

## Benchmark

- **Punkt 11 (Rat, folge142):** Der Rat (pro/max) entschied die Schätzer-Frage
  (Riss vs Defekt; p99-Null nur stochastisch; Kirkley vertagt) in einem Aufruf —
  Routine-Klasse geschlossen, kein flash-Gegenlauf nötig (die Frage brauchte
  Urteil, nicht Extraktion).
- **Punkt 11 (CI-Log-Extraktion, folge142):** `grind-flash` las die Job-/Log-Lage
  der drei Läufe (confirm-Log vollständig, screen/te-gate/ci-check ohne Log) —
  Routine-Klasse geschlossen (flash-Sieger, 2026-09-16), zitiert.

## Planungs-Tafel (offene Punkte)

| Punkt | Status | Bindung | Lage | Blockade | Braucht |
|---|---|---|---|---|---|
| 1. `family_fn_gate` Fix A | wartend | eigen | `35598791059` in_progress (ghost); screen ohne Log | screen-Job-Log | `ci_manage log 35598791059 --all` |
| 11. confirmation-Split | wartend | eigen | gebaut (2 Tests, Workflow-Job), `cargo check --tests` 0/0 | neuer Lauf | `ci_manage log <hyperscanning-te-id>` |
| 3. `--dropped` Baseline | wartend | eigen | Baum 2440 vs 2286 (delta 154); clean-Anker 2283 @ddf7e9f2 | kein abgeschlossener ci-check am HEAD | `dropped-gate`-Zeile des nächsten ci-check |
| 4. reduced TE R̂ | wartend | eigen | R̂ `123869be`; Pre-R̂-Tests ok; R̂-Lauf cancelled | kein Testlauf am R̂-Code | ci-check am HEAD lesen |
| 5. F3/F4 + Takens | wartend | eigen | — | grüner Screen (←1) | screen-Job-Log |
| 6. KSG↔KDE Verdrahtung | wartend | eigen | gebaut (`omega.rs:459-468,1724`) | Step 58 im Lauf `35628669014` | `te_scal`/`te_topo` lesen |
| 6b. topologische FN | wartend | eigen | Gate-Lücke; `topo` print-only | `35628669014` Step 58 | `topo`-Spalte lesen |
| F2. flare-Power-Probe | wartend | eigen | `te.rs:5872`, Step 55 | `35628669014` | `flare power probe:` + Gate-Verdikt |
| 8. Flyby-Path-2 | termin:2026-09-28 | termin | Auftrag steht | Datum | Zellen ab Perigäum |
| 9. NSE/Haug | wartend | dritter | Route offen | Dateieingang | Trigger |
| 10. BepiColombo MORE | termin:2027-04 | termin | Anfrage gesendet | Freigabe | Wissenschaftsphase |

## Geteilter Baum — eigener Pfad-Satz

- `tools/measure/src/bin/hyperscanning_group_te.rs` (Test-Split)
- `.github/workflows/hyperscanning-te.yml` (Job `confirm` ruft beide Tests)
- `docs/handover/handover-2026-09-21-sensory-folge142.md` (neu)
- Move `handover-2026-09-21-sensory-folge141.md` → `archiv/` (eigene Linie, atomar)
- `docs/zustand/external-state.md` (CI-Status-Zeile)

Fremde uncommittete Arbeit im selben Baum (`.github/workflows/hdf5-real-granule.yml`,
`src/archivar/hdf5.rs`, `src/mathematikerin/shaders.rs`, `src/mathematikerin/te.rs`,
`phi/pipeline/index.φ`, `phi/pipeline/ledger.φ`) wurde **nicht** angefasst.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. Der eigene Commit steht auf dem
aktuellen `origin/main` — der Push ist Fast-Forward. Nach dem Push wird
`hyperscanning-te` dispatcht (`gh workflow run hyperscanning-te.yml`), damit der
neue `confirm`-Job sein Verdikt liefert. `/consent` ist der session-weite Consent
(Delegation), nie das Commit-Wort.
