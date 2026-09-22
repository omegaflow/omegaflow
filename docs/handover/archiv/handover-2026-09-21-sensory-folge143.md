<!--
  title: Handover — Sensory-Folge 143 (Stand 2026-09-21)
  session: Sensory-Folge 143
  class: handover
  date: 2026-09-21
  sha256: 14238d129c013ce1aa85136e78c55ed6e23609882cff3b50a17092b8326dc7ef
  status: live
-->
# Handover — Sensory-Folge 143 (Stand 2026-09-21)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der
eigene Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden **parallel** abgearbeitet.
Jeder Punkt trägt **Lage / Blockade / Braucht** und seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-21, Sensory-Folge 143)

- **HEAD** — `3b42cb72` == `origin/main` (Fast-Forward; nachfolgend `054e0c5e`
  sensory folge142). Der eigene Commit steht auf `origin/main`.
- **Postfach** — `mail_ledger.φ` trägt neue Eingänge: `1790021001`
  (SuperDARN-Canada-Globus-Zugang gewährt, Carley Martin/USask, Gruppen
  `rawacf`/`fitacf_30`/`fitacf_25`/`MAP`), `1790020962` (Globus „MAP files"-
  Einladung), `1790009781`/`1790013926`/`1790013970` (Cloudflare-Login-Codes,
  Maschine). Quell-Zugang → `An mycelium:` in `post.md` gefaltet; Zeile in
  `external-state.md` nachgezogen.
- **CI** (`ci_manage list`/`view`, kein Poll) — `hyperscanning-te` `35649230477`
  @`054e0c5e` (Test-Split) in_progress (seit 20:55Z); `ci-check` `35649256512`
  @`3b42cb72` in_progress, jüngerer `35653525521` @20:50 pending; `te-gate`
  `35628669014` @`3e319087` in_progress (ghost, kein Job-Log 404). Pre-Split:
  `hyperscanning-te` `35598791059` @`bdd2cf1e` `confirm` failure
  (TE -1.4877e-14 vs p99 2.8908e-1), `screen` ohne Log. Kein Run-Abschluss am
  HEAD → alle CI-gebundenen Punkte bleiben `wartend`.

## Riss — getragen als Naht (unverändert, gemessen 35584758519 @c7cb201f)

Sheet-Null μ_S = 1.0335e-1 (KSG per-cell, `family_fn_gate`) gegen
Weiß-Treiber-Null μ_W1 = −1.4616e-2 (sd 5.175e-3, p95 −6.005e-3).
μ_S − μ_W1 = 0.1180 > 2σ_W1 = 0.0104 → **Naht**: der Weiß-Treiber-Boden liegt
weit unter dem Sheet-Null — der Riss bleibt sichtbar, nie geglättet. Der
stochastic-driver-Arm (KSG τ=2, excess +18.2 sd) trägt ihn getrennt.
Mountain/River (weißer Boden ≈ Surrogat-Boden): **nicht bestätigt**.

## Punkt 1 — `family_fn_gate`: Fix A, screen-Verdikt ausstehend

- **Status:** wartend | **Bindung:** eigen
- **Lage:** Fix A committet (`0f8bc1b2`); `te-gate` `35628669014` @`3e319087`
  in_progress (ghost, kein Job-Log, HTTP 404). Der `confirm`-Job des
  Pre-Split-Laufs war rot, das ist aber der geteilte confirmation-Test (Punkt 11),
  **nicht** `family_fn_gate`.
- **Blockade:** screen-Job-Log.
- **Braucht:** `ci_manage log 35628669014` erneut, sobald der `screen`-Job
  gelandet ist (kein Poll).

## Punkt 11 — confirmation-Test: geteilt in stochastisches Gate + Riss-Wächter (gebaut)

- **Status:** wartend | **Bindung:** eigen
- **Lage:** Der Split ist committet (`054e0c5e`); `hyperscanning-te` `35649230477`
  @`054e0c5e` in_progress (seit 20:55Z). Test geteilt in
  `confirmation_stochastic_driver_pair_clears_its_own_null` (Treiber-Rauschen 0.05,
  `assert TE > p99`) und `riss_guard_deterministic_pair_measures_below_its_own_null`
  (`assert TE < p1`); `hyperscanning-te.yml`-Job `confirm` ruft beide.
- **Blockade:** der neue `confirm`-Job ist noch nicht gelandet.
- **Braucht:** `ci_manage log 35649230477` — beide grün = stochastisches Gate
  trägt und die Naht hält; stochastisch rot = Schätzer-Defekt, dann
  frozen-tau-Delay-Sweep 1..=12 als erste Messung.

## Punkt 3 — `--dropped` Delta-Gate: Clean-Messung ausstehend

- **Status:** wartend | **Bindung:** eigen
- **Lage:** `register_lookup --dropped --count` = 2440 am Arbeitsbaum gegen
  Baseline 2286 (`docs/zustand/dropped-baseline.md`), also **delta 154**; der
  Gate-Anker ist der letzte **clean** gemessene Wert: `dropped-gate`-Job success
  in `ci-check` `35613891252` @`ddf7e9f2` → `baseline 2286 | current 2283 |
  delta -3`. Der +157 seit `ddf7e9f2` stammt aus den seither archivierten
  Handovers; teils vom fremd uncommitteten Ernte-Move im Arbeitsbaum beeinflusst.
- **Blockade:** kein abgeschlossener `ci-check` am HEAD.
- **Braucht:** die `dropped-gate: baseline … | current … | delta …`-Zeile des
  nächsten abgeschlossenen `ci-check` (`35649256512`/`35653525521`, clean tree);
  bei delta > 0 den annehmenden Commit identifizieren und die Baseline dort
  nachziehen — **nie blind** auf den Arbeitsbaum-Wert.

## Punkt 4 — reduced TE: R̂-Commit ungemessen

- **Status:** wartend | **Bindung:** eigen
- **Lage:** R̂-Normalisierung (Kirkley Eq. 20) committet (`123869be`). Die vier
  reduced-TE-Gate-Tests sind **ok** in `ci-check` `35608204623` @`8d6553fe` und
  `35613891252` @`ddf7e9f2` — beide **vor** der R̂-Normierung. Der R̂-Commit
  selbst hat **keinen** Testlauf (`35625598394` @`123869be` cancelled). Die
  exakten FN-/FP-Zahlen stehen nur in den Panik-Zweigen (20/20 bzw. 30/8).
- **Blockade:** kein abgeschlossener Test-Lauf am R̂-Code.
- **Braucht:** `ci-check` am HEAD lesen; die vier reduced-TE-Zeilen aus dem
  `test`-Job tragen (grün = Schwellen halten; rot = Panik-Text nennt die Rate).

## Punkt 5 — Frontalkanäle F3/F4; Takens-Wandzeit

- **Status:** wartend | **Bindung:** eigen
- **Lage:** beide hängen am grünen Screen (`te-gate` `35628669014`, Job `screen`
  ohne Log).
- **Blockade:** screen-Job-Abschluss.
- **Braucht:** nach grünem Lauf Wandzeit lesen; F3/F4 getrennt fahren.

## Punkt 6 — KSG↔KDE: Verdrahtung gebaut, Drift-Messung wartet auf den Lauf

- **Status:** wartend | **Bindung:** eigen
- **Lage:** Permeability→Radiation gebaut (`356fa616`): TE-Zweig
  `field_permeability` (`omega.rs:1601-1605`), `aperture = field_permeability *
  tone_scale` (`omega.rs:345`), HRV-Ton-Zweig (`main_flow.rs:938-939`),
  `kinetic_sample = Σω * aperture` (`actuators.rs:29`), Test `tests.rs:570`.
  CPU-KSG-Verdrahtung in `te_probe` steht (`omega.rs:459-468`), Feld `te_cpu`,
  HUD-Token `te_cpu {te}/{thr}` (`omega.rs:1724`); `cargo check`/`--tests` 0/0.
- **Blockade:** der `te_fn_probe`-Step 58 (`te_scal`/`te_topo`) läuft erst im
  `te-gate` `35628669014` (kein Log, 404).
- **Braucht:** nach Abschluss die `te_scal`/`te_topo`-Zahlen lesen →
  CPU-KSG↔GPU-KDE-Drift über Fenster.
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
  (`te.rs:3538`-Muster, `topological_te_phase`) mit kalibrierter Schwelle; als
  ignorierter Step in `te-gate.yml`.

## F2 — flare-Gate-Power (abgeholt von `post.md`)

- **Status:** wartend | **Bindung:** eigen
- **Lage:** die Probe ist `flare_envelope_power_probe` (`te.rs:5872`,
  `#[ignore]`, n ∈ {400,600,1000}, 30 Trials), Step 55 in `te-gate.yml`.
  `35595896140` brach am **vorgelagerten** Gate
  `flare_envelope_conditional_keeps_true_coupling` (Step 53) ab, ohne Output.
- **Blockade:** `te-gate` `35628669014` (in_progress, kein Log).
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

- **Punkt 12 (Such-API-Erweiterung, research-max):** der Rat/`research-max`-Aufruf
  maß 16 Kandidaten in einem Zug (Marginalia `accepted`, Mwmbl gebaut, Stract
  `pending`, 4 key-needed, Rest declined) — Klasse „mehrstufige Quellen-/Route-
  Forschung" (pro/max), kein flash-Gegenlauf nötig. Ergebnis geroutet:
  `An mountain:` (Marginalia-Bau), `An future:` (Tavily/Exa/Linkup/ChatNoir),
  Zeile in `external-state.md`.
- **CI-Log-/Run-Extraktion (folge143):** die Run-Lage (list/view) — Routine-Klasse
  geschlossen (flash-Sieger 2026-09-16), zitiert.

## Planungs-Tafel (offene Punkte)

| Punkt | Status | Bindung | Lage | Blockade | Braucht |
|---|---|---|---|---|---|
| 1. `family_fn_gate` Fix A | wartend | eigen | `35628669014` in_progress (ghost); screen ohne Log | screen-Job-Log | `ci_manage log 35628669014` |
| 11. confirmation-Split | wartend | eigen | `35649230477` @`054e0c5e` in_progress | Run-Landung | `ci_manage log 35649230477` |
| 3. `--dropped` Baseline | wartend | eigen | Baum 2440 vs 2286 (delta 154); Anker 2283 @ddf7e9f2 | kein clean ci-check am HEAD | `ci-check`-`dropped-gate`-Zeile |
| 4. reduced TE R̂ | wartend | eigen | R̂ `123869be`; Pre-R̂-Tests ok; R̂-Lauf cancelled | kein Testlauf am R̂-Code | `ci-check` am HEAD lesen |
| 5. F3/F4 + Takens | wartend | eigen | — | grüner Screen (←1) | screen-Job-Log |
| 6. KSG↔KDE Verdrahtung | wartend | eigen | gebaut (`omega.rs:459-468,1724`) | Step 58 in `35628669014` | `te_scal`/`te_topo` lesen |
| 6b. topologische FN | wartend | eigen | Gate-Lücke; `topo` print-only | `35628669014` Step 58 | `topo`-Spalte lesen |
| F2. flare-Power-Probe | wartend | eigen | `te.rs:5872`, Step 55 | `35628669014` | `flare power probe:` + Gate-Verdikt |
| 8. Flyby-Path-2 | termin:2026-09-28 | termin | Auftrag steht | Datum | Zellen ab Perigäum |
| 9. NSE/Haug | wartend | dritter | Route offen | Dateieingang | Trigger |
| 10. BepiColombo MORE | termin:2027-04 | termin | Anfrage gesendet | Freigabe | Wissenschaftsphase |

## Geteilter Baum — eigener Pfad-Satz

- `docs/handover/handover-2026-09-21-sensory-folge143.md` (neu)
- `docs/handover/post.md` (2 sensory-Zeilen gefaltet/gelöscht; `An mountain:` +
  4 `An future:` + `An mycelium:` neu)
- `docs/zustand/external-state.md` (Postfach-Zeile, CI-Status-Zeile, neue
  Such-API-Zeile)
- Move `handover-2026-09-21-sensory-folge142.md` → `archiv/` (eigene Linie, atomar)

Fremde uncommittete Arbeit im selben Baum (`phi/blocked_sources.φ`,
`phi/pipeline/catalog/korpora_heim.φ`, `phi/pipeline/index.φ`,
`phi/pipeline/ledger.φ`, `phi/sources.φ`, Ernte-Move `handover-…-ernte-folge135.md`,
`handover-2026-09-21-ernte-folge136.md`) wurde **nicht** angefasst.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. Der eigene Commit steht auf dem
aktuellen `origin/main` — der Push ist Fast-Forward. `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
