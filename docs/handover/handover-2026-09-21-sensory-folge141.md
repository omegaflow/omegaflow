<!--
  title: Handover — Sensory-Folge 141 (Stand 2026-09-21)
  session: Sensory-Folge 141
  class: handover
  date: 2026-09-21
  sha256: 487e1f5e21380f6be1e319ed8864b38064da94bd026f152c01b6bc00f6e36283
  status: live
-->
# Handover — Sensory-Folge 141 (Stand 2026-09-21)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der
eigene Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden **parallel** abgearbeitet.
Jeder Punkt trägt **Lage / Blockade / Braucht** und seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

Linien-Identität: Die Linie fährt jetzt als **Sensory** (Voice-Rename `8d6553fe`,
`forschung → sensory`); dieses Register ist das erste sensory-benannte — die
`sensory`-Auto-Erkennung (`grep handover-.*sensory`) greift damit wieder.

## Stehender Pass (gemessen 2026-09-21, Sensory-Folge 141)

- **HEAD** — Session-Beginn `0b1a7d03` (Forschung-Folge 140); `origin/main`
  stand beim Pass auf `3e319087` (River-Folge 3). Fremde Linien pushten während
  der Session weiter (River, Future, hawc-cdn — `origin/main` beim Commit
  `0778d737`); der eigene Commit steht als Fast-Forward auf dem aktuellen
  `origin/main`. Arbeitsbaum beim Session-Beginn fremd uncommittet (Ernte-Move
  `ernte-folge134.md` → `archiv/`, `ernte-folge135.md`, die fünf Register
  `phi/blocked_sources.φ`, `phi/footprints.φ`, `phi/pipeline/ledger.φ`,
  `phi/sources.φ`, `phi/witnesses.φ`) — von der Ernte-Linie während der Session
  committet, **nicht angefasst**.
- **Postfach** — letzter Ledger-Eingang `1789978555` (Brave Search API „usage
  limit reached", 100 % von $5.00 free credits, September 2026 — informativ);
  `external-state.md`-Eintrag jünger als 2⁶ min, zitiert, nicht neu gemessen.
- **CI** (überholt den Watchdog-Snapshot): `te-gate` `35595896140` @`44e77da0`
  **failure** — die n=1000-FPR-Kalibrierung `gate_fpr_autocorrelation` **7/7 ok**
  (4122,64 s), danach Abbruch am `flare_envelope_conditional`-Step **ohne
  Test-Output** (Runner-Shutdown-Signatur); die Probe-Steps 55
  (`flare_envelope_power_probe`) und 58 (`te_fn_probe`) liefen **nicht**.
  Daraufhin neu dispatcht: `te-gate` `35628669014` @`3e319087` **in_progress**.
  `hyperscanning-te` `35598791059` @`bdd2cf1e` in_progress; `35596009980`
  @`44e77da0` success. `ci-check` `35627916025` @`0b1a7d03` cancelled (vom
  River-Push überholt); `35625268875` @`b2c966c0` in_progress. Kein Poll.

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
- **Lage:** Fix A committet (`0f8bc1b2`); `35598791059` @`bdd2cf1e` in_progress
  (Job `screen` läuft, `confirm` wartet). `35596009980` @`44e77da0` success ist
  ein älterer Commit, kein Fix-A-Verdikt.
- **Blockade:** Run-Abschluss.
- **Braucht:** `ci_manage view 35598791059` einmal.

## Punkt 3 — `--dropped` Delta-Gate: Drift gemessen, Clean-Messung ausstehend

- **Status:** wartend | **Bindung:** eigen
- **Lage:** `register_lookup --dropped --count` = **2440** am Arbeitsbaum gegen
  Baseline 2286 (`docs/zustand/dropped-baseline.md`), also **delta 154**; der
  Gate-Anker ist der letzte **clean** gemessene Wert: `dropped-gate`-Job
  success in `ci-check` `35613891252` @`ddf7e9f2` → `baseline 2286 | current
  2283 | delta -3`. Der +157 seit `ddf7e9f2` stammt aus den seither archivierten
  Handovers (forschung 139→140, mountain 126→127, ernte 133→134, future 84→85)
  und ist teils vom fremd uncommitteten Ernte-Move im Arbeitsbaum beeinflusst —
  der reine Commit-Wert ist ungemessen. Verteilung: forschung 811, ernte 652,
  bau 443, entscheid 397, future 73, river 18, mountain 17 (Summe 2440).
- **Blockade:** kein abgeschlossener `ci-check` am HEAD (`123869be`/`0b1a7d03`
  beide cancelled) → keine Clean-Messung des Commit-Werts.
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
  **vor** der R̂-Normierung. Der R̂-Commit selbst hat **keinen** Testlauf:
  `35625598394` @`123869be` = cancelled, 0 Jobs. Die exakten FN-/FP-Zahlen
  stehen nur in den Panik-Zweigen (20/20 bzw. 30/8) und werden bei `ok` nie
  gedruckt.
- **Blockade:** kein abgeschlossener Test-Lauf am R̂-Code.
- **Braucht:** `ci-check` am HEAD/`3e319087` lesen; die vier reduced-TE-Zeilen
  aus dem `test`-Job tragen (grün = Schwellen halten; rot = Panik-Text nennt die
  gemessene Rate).

## Punkt 5 — Frontalkanäle F3/F4; Takens-Wandzeit

- **Status:** wartend | **Bindung:** eigen
- **Lage:** beide hängen am grünen Screen (`35598791059`, Job `screen` läuft).
- **Blockade:** Run-Abschluss.
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
  neuen `te-gate` `35628669014`.
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
- **Blockade:** neuer Lauf `35628669014` (queued/in_progress).
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

- **Punkt 4 (CI-Log-Extraktion, folge141):** `grind-flash` las die
  reduced-TE-Gate-Zeilen aus vier `ci-check`-Logs (runs 35625598394/35603922594/
  35608204623/35613891252) und fand die Lücke (R̂-Commit cancelled, 0 Jobs).
  Die Routine-Klasse ist geschlossen (flash-Sieger, 2026-09-16) — zitiert, kein
  pro/max-Gegenlauf.

## Planungs-Tafel (offene Punkte)

| Punkt | Status | Bindung | Lage | Blockade | Braucht |
|---|---|---|---|---|---|
| 1. `family_fn_gate` Fix A | wartend | eigen | `35598791059` in_progress | Run-Abschluss | `ci_manage view 35598791059` |
| 3. `--dropped` Baseline | wartend | eigen | Arbeitsbaum 2440 vs 2286 (delta 154); clean-Anker 2283 @ddf7e9f2 | kein abgeschlossener ci-check am HEAD | `dropped-gate`-Zeile des nächsten ci-check |
| 4. reduced TE R̂ | wartend | eigen | R̂ `123869be`; Pre-R̂-Tests ok; R̂-Lauf cancelled | kein Testlauf am R̂-Code | ci-check am HEAD lesen |
| 5. F3/F4 + Takens | wartend | eigen | — | grüner Screen (←1) | `ci_manage view 35598791059` |
| 6. KSG↔KDE Verdrahtung | wartend | eigen | gebaut (`omega.rs:459-468,1724`) | Step 58 im Lauf `35628669014` | `te_scal`/`te_topo` lesen |
| 6b. topologische FN | wartend | eigen | Gate-Lücke; `topo` print-only | `35628669014` Step 58 | `topo`-Spalte lesen |
| F2. flare-Power-Probe | wartend | eigen | `te.rs:5872`, Step 55 | `35628669014` | `flare power probe:` + Gate-Verdikt |
| 8. Flyby-Path-2 | termin:2026-09-28 | termin | Auftrag steht | Datum | Zellen ab Perigäum |
| 9. NSE/Haug | wartend | dritter | Route offen | Dateieingang | Trigger |
| 10. BepiColombo MORE | termin:2027-04 | termin | Anfrage gesendet | Freigabe | Wissenschaftsphase |

## Geteilter Baum — eigener Pfad-Satz

- `docs/handover/handover-2026-09-21-sensory-folge141.md` (neu)
- Move `handover-2026-09-21-forschung-folge140.md` → `archiv/` (eigene Linie,
  atomar)
- `docs/zustand/external-state.md` (CI-Status-Zeile + TE-Gate-FPR-Zeile)

Fremde uncommittete Arbeit im selben Baum (Ernte-Move, `ernte-folge135.md`,
die Register `phi/blocked_sources.φ`, `phi/footprints.φ`,
`phi/pipeline/ledger.φ`, `phi/sources.φ`, `phi/witnesses.φ`) wurde **nicht**
angefasst — die Ernte-Linie committete sie während der Session selbst.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. Der eigene Commit steht auf dem
aktuellen `origin/main` — der Push ist Fast-Forward. Diese Session ändert keinen
Workflow; `ci-check` läuft push-getriggert, `te-gate` wurde dispatcht.
`/consent` ist der session-weite Consent (Delegation), nie das Commit-Wort.
