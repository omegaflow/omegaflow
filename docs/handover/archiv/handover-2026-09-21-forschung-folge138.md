<!--
  title: Handover — Forschung-Folge 138 (Stand 2026-09-21)
  session: Forschung-Folge 138
  class: handover
  date: 2026-09-21
  sha256: 37e8b9c8d93d58f942c0240bf6229d05630e66a1a04be20a5b95027770ed211a
  status: live
-->
# Handover — Forschung-Folge 138 (Stand 2026-09-21)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der
eigene Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden **parallel** abgearbeitet.
Jeder Punkt trägt **Lage / Blockade / Braucht** und seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-21, Forschung-Folge 138)

- **HEAD** `379ca31b` == `origin/main` (fremde Commits seither: `bau folge124`
  `379ca31b`/`e964d4f9`, `entscheid folge83` `40c826cc`, `ernte folge133`
  `db708714`; folge137 nannte `096ed904`). Arbeitsbaum trägt die eigene
  uncommittete Änderung `src/mathematikerin/te.rs` (reduced-TE-Port).
- **Postfach** — neuester Ledger-Eingang `1789978555` (Brave Search API „usage
  limit reached", 100 % von $5.00 free credits, September 2026 — informativ);
  unverändert seit folge137, kein handlungsbedürftiger Fall.
- **CI** — `hyperscanning-te` `35598791059` @`bdd2cf1e` **pending**,
  `te-gate` `35595896140` @`44e77da0` **in_progress**; `ci-check` `35599318266`
  in_progress / `35601265876` pending (`ci_manage view`, einmal gelesen, kein Poll).

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
- **Lage:** Fix A committet (`0f8bc1b2`); `35598791059` @`bdd2cf1e` pending
  (`ci_manage view`).
- **Blockade:** Run-Abschluss (funktionaler Lauf nur in CI).
- **Braucht:** `ci_manage view 35598791059` einmal.

## Punkt 2 — `te-gate` n=1000-FPR-Boden

- **Status:** wartend | **Bindung:** eigen
- **Lage:** `35595896140` @`44e77da0` in_progress; der `te_fn_probe`-Step steht
  (`te-gate.yml:58`).
- **Blockade:** Run-Abschluss.
- **Braucht:** `ci_manage view 35595896140`.

## Punkt 3 — `--dropped` Delta-Gate: Baseline 2219 → 2286

- **Status:** wartend | **Bindung:** eigen
- **Lage:** `register_lookup --dropped --count` = **2286** (2026-09-21 folge138,
  Delta 67 über 2219); Baseline in `docs/zustand/dropped-baseline.md` im
  annehmenden Commit nachgezogen.
- **Blockade:** nächster `ci-check`-Lauf.
- **Braucht:** `ci_manage view <ci-check-id>`; bei neuer Drift erneut messen.

## Punkt 4 — reduced TE (Kirkley): Port gebaut, CI-Nachweis + Normalisierung

- **Status:** wartend | **Bindung:** eigen
- **Lage:** diskreter/gebinnter Schätzer portiert (`te.rs:2313`
  `transfer_entropy_reduced`, `2368` `reduced_te_flow` = MDL-Null R ≤ 0 ⇔ kein
  Fluss, `2262` `lgamma_lanczos`, `2286` `log_factorial`, `2290` `log_choose`,
  `2296` `sorted_log_sum`, `2302` `quantile_edges`, `2309` `bin_of`). Fünf Tests:
  `3708` `split_recording_ksg_kde_reduced_te_all_resolve` (umbenannt aus
  `riss_ksg_kde_estimator_split_is_measured`; zeichnet drei Zahlen auf, keine
  Differenz-Assertion), `3752` `gate_fn_reduced_te_mdl_coupled_ar1_flows`,
  `3787` `gate_fp_reduced_te_mdl_independent_ar1_stays_silent`, `3817`
  `gate_n_floor_reduced_te_table_size`, `3841`
  `gate_symmetry_reduced_te_identical_series_measure_equally`. `cargo check` und
  `cargo check --tests` 0 Fehler / 0 Warnungen. Ergänzend, nie ersetzend
  (Rats-Verdikt: eigenes MDL-Null, diskreter n-Floor Cˡ·Cᵏ, kein
  Abgrenzungs-Gate).
- **Blockade:** die Gate-Schwellen (FN > 0.5, FP ≤ 8/30) sind aus der
  Bias-Analyse (S34/S36) gesetzt, nicht gemessen — der erste CI-Lauf ist die
  Messung; die Normalisierung (Eq. 19/20, R̃ ∈ [−1,1]) ist nicht portiert (der
  R>0-Nenner ist an den zwei Lesestellen uneinheitlich).
- **Braucht:** `ci-check` (default `cargo test` führt die neuen Gates); bei rotem
  Gate trägt das Log die gemessene Rate; Normalisierung erst nach eindeutiger
  Lesestelle.

## Punkt 5 — Frontalkanäle F3/F4; Takens-Wandzeit

- **Status:** wartend | **Bindung:** eigen
- **Lage:** beide hängen am grünen Screen (`35598791059`).
- **Blockade:** Run-Abschluss.
- **Braucht:** nach grünem Lauf Wandzeit lesen; F3/F4 getrennt fahren.

## Punkt 6 — Riss 4 (KSG↔KDE): Confounder in gemessener Form

- **Status:** wartend | **Bindung:** eigen
- **Lage:** B committet (Shader-Kommentar `shaders.rs:483,525`; Sprechort
  `solar.rs:445,449`, `matrix.rs:894,911,1059`). Kirkley löst den Riss **nicht**
  (dritter, diskreter Pfad). **Der Confounder ist präzisiert (Rat, folge138):**
  nicht „kein Produktions-Konsument" — KSG (`topological_te_phase`/
  `topological_te_estimate`) speist 14 Produktions-Messbins inkl.
  `hyperscanning_group_te`. Was absent ist: der KSG-Kanon ist aus dem
  **Manifestations-/Echo-Pfad** (`omega.rs:1563` → `te_probe` → WGSL
  `te_compute` → `te_embedded_kde`) — dieser trinkt nur GPU-KDE.
- **Blockade:** Verdrahtung wäre ein toter Hyph: ihr eigener Leser (das
  Permeability→Radiation-Binding) ist `pending`.
- **Braucht:** Trigger = das Atom „Permeability→Radiation-Binding"; fällt er,
  kleinste nicht-fabrizierende Verdrahtung: ein `topological_te_phase`-Aufruf je
  HUD-Tick (1 Hz, m ≤ 256) in `te_probe`, CPU-KSG neben GPU-KDE, beide Zahlen in
  der `te_say`/HUD-Zeile — kein Wire-/Shader-Eingriff, Echo unverändert bis die
  Drift über Fenster gemessen ist.

## Punkt 6b — topologische FN (Gate-Lücke)

- **Status:** wartend | **Bindung:** eigen
- **Lage:** der topologische Pfad hat FP (`gate_fpr_autocorrelation_*_ksg_*`),
  Symmetrie (`3578`) und n-Floor (`3637`), aber **keine FN**; `te_fn_probe`
  druckt die `topo`-Spalte (found/10 bei c=0.9) — print-only, kein Gate.
- **Blockade:** die Zahlen stehen in `35595896140` (in_progress); ein Assert
  ohne diese Zahlen wäre eine Schwelle aus Annahme.
- **Braucht:** nach grünem Lauf die `topo`-Spalte lesen; dann
  `calibration_fn_topological_ksg_finds_true_coupling` spiegeln
  (`te.rs:3538`-Muster, `topological_te_phase`) mit an diesen Zahlen
  kalibrierter Schwelle; als ignorierter Step in `te-gate.yml`.

## Wartend / operator-gebunden / termin

- Flyby-Path-2-Kette — `termin:2026-09-28` (Auftrag steht, Kanäle live; Zellen ab
  Perigäum).
- NSE/Haug — `wartend`/`dritter` (Route offen, Mail 2026-09-17; Trigger
  Dateieingang).
- BepiColombo MORE — `termin:2027-04` (Freigabe-Anfrage 2026-09-18; kein
  Zwischenzug).

## Planungs-Tafel (offene Punkte)

| Punkt | Status | Bindung | Lage | Blockade | Braucht |
|---|---|---|---|---|---|
| 1. `family_fn_gate` Fix A | wartend | eigen | `35598791059` pending @`bdd2cf1e` | Run-Abschluss | `ci_manage view 35598791059` |
| 2. `te-gate` n=1000 | wartend | eigen | `35595896140` in_progress | Run-Abschluss | `ci_manage view 35595896140` |
| 3. `--dropped` Baseline | wartend | eigen | current 2286, Baseline 2286 | nächster ci-check | `ci_manage view <ci-check-id>` |
| 4. reduced TE | wartend | eigen | Port gebaut, 5 Tests, 0/0 | Gate-Schwellen + Normalisierung ungemessen | `ci-check`-Lauf |
| 5. F3/F4 + Takens | wartend | eigen | — | grüner Screen (←1) | `ci_manage view 35598791059` |
| 6. Riss 4 KSG↔KDE | wartend | eigen | Confounder präzisiert (Echo-Pfad) | Trigger fehlt | Binding-Atom |
| 6b. topologische FN | wartend | eigen | Gate-Lücke; `topo` print-only | Zahlen in `35595896140` | `topo`-Spalte lesen |
| 8. Flyby-Path-2 | termin:2026-09-28 | termin | Auftrag steht | Datum | Zellen ab Perigäum |
| 9. NSE/Haug | wartend | dritter | Route offen | Dateieingang | Trigger |
| 10. BepiColombo MORE | termin:2027-04 | termin | Anfrage gesendet | Freigabe | Wissenschaftsphase |

## Benchmark

- **Reduced-TE-Port (2026-09-21, folge138):** `grind-pro` lieferte einen leeren
  Bericht und **keine** Änderung (`te.rs` unverändert) → Eskalation auf
  `grind-max` (hartes Atom, novel estimator construction); `grind-max` lieferte
  230 Zeilen, 5 Gates, `cargo check`/`--tests` 0/0. Die pro-Stufe war für dieses
  Atom unvollständig — der Rat trug das Urteil (Q1–Q3), der Taucher den Bau.
  Burn (`session_burn`): `grind-pro` **$0.1087** (leer), `grind-max` **$0.1239**
  (vollständig, 1,14× teurer), `council` **$0.0370** — für das harte Atom ist
  max nur marginal teurer als pro und liefert; pro bleibt unzuverlässig, max ist
  der Träger.

## Geteilter Baum — eigener Pfad-Satz

- `docs/handover/handover-2026-09-21-forschung-folge138.md` (neu)
- Move `handover-2026-09-21-forschung-folge137.md` → `archiv/` (eigene Linie,
  atomar)
- `src/mathematikerin/te.rs` (reduced-TE-Port)
- `docs/zustand/dropped-baseline.md` (2219 → 2286, gemessen)
- `docs/zustand/external-state.md` (CI-Zeile)

Fremde uncommittete Arbeit im selben Baum wird **nicht** angefasst.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. Diese Session ändert keinen
Workflow — kein Dispatch nötig; `ci-check` läuft push-getriggert. `/consent` ist
der session-weite Consent (Delegation), nie das Commit-Wort.
