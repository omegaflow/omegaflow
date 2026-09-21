<!--
  title: Handover — Forschung-Folge 139 (Stand 2026-09-21)
  session: Forschung-Folge 139
  class: handover
  date: 2026-09-21
  sha256: 65ba94c8d43c57f4d9606e44fab69bd2b272992b57ba4c6ac3d6406c2109de97
  status: live
-->
# Handover — Forschung-Folge 139 (Stand 2026-09-21)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der
eigene Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden **parallel** abgearbeitet.
Jeder Punkt trägt **Lage / Blockade / Braucht** und seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-21, Forschung-Folge 139)

- **HEAD** `b2c966c0` == `origin/main`. Der Pass begann bei `b3ec7683`; während
  der Session landeten fremde Commits bis `b2c966c0`. Arbeitsbaum trägt die
  eigene uncommittete Änderung `src/mathematikerin/te.rs` (R̂-Normalisierung);
  fremd uncommittet: `docs/handover/post.md`, `phi/blocked_sources.φ`,
  `phi/pipeline/ledger.φ`, `phi/sources.φ`, `src/archivar/hdf5.rs`,
  `src/archivar/port.rs`, neu `docs/handover/handover-2026-09-21-mountain-folge127.md`
  — nicht angefasst.
- **Postfach** — neuester Ledger-Eingang `1789978555` (Brave Search API „usage
  limit reached", 100 % von $5.00 free credits, September 2026 — informativ);
  unverändert seit folge137/138, kein handlungsbedürftiger Fall.
- **CI** — `hyperscanning-te` `35598791059` @`bdd2cf1e` **pending**,
  `te-gate` `35595896140` @`44e77da0` **in_progress** (`ci_manage view`, einmal
  gelesen, kein Poll). Watchdog-Snapshot `/tmp/opencode/ci_status.md`: `ci-check`
  `35613891252` in_progress; Fehlversuche `ci-check` `35608204623`/`35603922594`/
  `35599318266`, `glm-l2-cdn` `35599198872`/`35599180155`, `esp32-firmware`
  `35615957619` (fremde Linien, nicht eigene Punkte).

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

## Punkt 3 — `--dropped` Delta-Gate: Baseline 2286

- **Status:** wartend | **Bindung:** eigen
- **Lage:** `register_lookup --dropped --count` = **2286** (folge138), Baseline
  in `docs/zustand/dropped-baseline.md` im annehmenden Commit nachgezogen.
- **Blockade:** nächster `ci-check`-Lauf.
- **Braucht:** `ci_manage view <ci-check-id>`; bei neuer Drift erneut messen.

## Punkt 4 — reduced TE: Gate-Schwellen messen (Normalisierung erledigt)

- **Status:** wartend | **Bindung:** eigen
- **Lage:** Die Normalisierung R̂ ∈ [−1,1] (Kirkley Eq. 20) ist portiert
  (`te.rs` `transfer_entropy_reduced_normalized`: `num = s_full+s_y−s_fy−s_yx+corr`,
  `denom = if num>0 {s_y−s_fy+corr} else {−corr}`, `Some(num/denom)`, `None` bei
  `denom ≤ 0`). Rat-Verdikt (2026-09-21): Eq. 20 (Main Text) ist Kanon, der
  Supplemental-Satz S5 (`H_M(X⁻ᵏ|Y⁻ˡ)`) trägt einen gemessenen Tippfehler — die
  Herleitung S41–S44 entscheidet für `H_M(y⁺|Y⁻ˡ)`; Funktion umbenannt
  (`_normalized`, Name = Implementation); `reduced_te_flow` behält Name und
  Vorzeichen-Gate; `|R̂| ≤ 1.0` inklusiv. Neuer Test
  `gate_reduced_te_normalized_bounded`. `cargo check`/`--tests` 0/0. Verbleibend:
  die Gate-Schwellen (FN > 0.5, FP ≤ 8/30) sind aus der Bias-Analyse (S34/S36)
  gesetzt, nicht gemessen.
- **Blockade:** erster CI-Lauf (`cargo test`); die FN-Zahl (erwartet 20/20) wird
  auf die `M ≤ 0`-Ecke gelesen.
- **Braucht:** `ci-check`-Lauf; bei rotem Gate trägt das Log die gemessene Rate.

## Punkt 5 — Frontalkanäle F3/F4; Takens-Wandzeit

- **Status:** wartend | **Bindung:** eigen
- **Lage:** beide hängen am grünen Screen (`35598791059`).
- **Blockade:** Run-Abschluss.
- **Braucht:** nach grünem Lauf Wandzeit lesen; F3/F4 getrennt fahren.

## Punkt 6 — Riss 4 (KSG↔KDE): Confounder in gemessener Form

- **Status:** wartend | **Bindung:** eigen
- **Lage:** B committet (Shader-Kommentar `shaders.rs:483,525`; Sprechort
  `solar.rs:445,449`, `matrix.rs:894,911,1059`). Kirkley löst den Riss **nicht**
  (dritter, diskreter Pfad). Der Confounder ist präzisiert (Rat, folge138):
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
| 4. reduced TE Schwellen | wartend | eigen | R̂-Normalisierung portiert, 0/0 | Schwellen ungemessen | `ci-check`-Lauf |
| 5. F3/F4 + Takens | wartend | eigen | — | grüner Screen (←1) | `ci_manage view 35598791059` |
| 6. Riss 4 KSG↔KDE | wartend | eigen | Confounder präzisiert (Echo-Pfad) | Trigger fehlt | Binding-Atom |
| 6b. topologische FN | wartend | eigen | Gate-Lücke; `topo` print-only | Zahlen in `35595896140` | `topo`-Spalte lesen |
| 8. Flyby-Path-2 | termin:2026-09-28 | termin | Auftrag steht | Datum | Zellen ab Perigäum |
| 9. NSE/Haug | wartend | dritter | Route offen | Dateieingang | Trigger |
| 10. BepiColombo MORE | termin:2027-04 | termin | Anfrage gesendet | Freigabe | Wissenschaftsphase |

## Benchmark

- **R̂-Normalisierung (2026-09-21, folge139):** flash-only, kein pro/max nötig.
  `general` (flash) löste die Eq.-20-Lesestelle eindeutig auf (Main Text vs.
  S5-Prosa; Paper arXiv 2506.16215v3, HTTP 200 direkt) — Burn **$0.0417**.
  `grind-flash` (flash) baute den Port in `te.rs`, `cargo check`/`--tests` 0/0.
  `council` (pro/max) trug das Abschluss-Verdikt (Q1–Q4) — Burn **$0.0268**.
  Der Sieger der Bau-/Recherche-Klasse ist flash (billigster Träger), kein
  pro/max-Eskalationsbedarf.

## Geteilter Baum — eigener Pfad-Satz

- `docs/handover/handover-2026-09-21-forschung-folge139.md` (neu)
- Move `handover-2026-09-21-forschung-folge138.md` → `archiv/` (eigene Linie,
  atomar)
- `src/mathematikerin/te.rs` (R̂-Normalisierung + Umbenennung)

Fremde uncommittete Arbeit im selben Baum wird **nicht** angefasst.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. Diese Session ändert keinen
Workflow — kein Dispatch nötig; `ci-check` läuft push-getriggert. `/consent` ist
der session-weite Consent (Delegation), nie das Commit-Wort.
