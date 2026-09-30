<!--
  title: Handover — River-Folge 67 (2026-09-29)
  session: River-Folge 67
  class: handover
  date: 2026-09-29
  sha256: e0360d2237a4f825e75d42ca17dd75199d45aaf9ebaf95a039485668c50ed272
  status: live
-->
# Handover — River-Folge 67 (2026-09-29)

Dieses Register trägt nur Offenes — git trägt, was gemacht wurde. Der Stehende Pass
wird zitiert, nie kopiert: `state/zustand/standing-pass.md`. Nur eigene Arbeit:
pfad-begrenzter Commit; fremde uncommittete Arbeit unangetastet.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„Erste Handlung: `sread docs/concepts/tool-forms.md`" | 2026-09-27 | Operator (Session, River 49)
„die ttl muss die Aktualisierung der Quelle sein" | 2026-09-27 | Operator (Session, River 47)
„#body erzeugt das bias … komplett rückgängig" — kein Körper privilegiert | 2026-09-27 | Operator (Session, River 47)
„frag den rat" / „folge dem rat" | 2026-09-27 | Operator (Session, River 47)
GIC-Paper-Einreichung: höchste Priorität | 2026-09-27 | Operator-Wort
Geräte-Zugriff: vor jedem Zugriff fragen (adb/BT) | 2026-09-26 | Operator-Wort
Harte-Läufe-LOCK aufgehoben | 2026-09-26 | Operator-Wort
HTTPS ja | 2026-09-26 | Operator-Wort folge36
Entscheidungen nie als Liste vorlegen — jede braucht eine Erklärung | 2026-09-27 | Operator (Future-Session)
„die Kante bin ich" — Wert, Wort, Dritt-Akt und Send bleiben seine Hand | 2026-09-27 | Operator (Future-Session)
ein gegebenes Wort steht in den Operator-Wort-Registern aller live Übergaben | 2026-09-27 | Operator (Future-Session)
`/consent` — session-weiter Delegations-Consent, nicht das Commit-Wort | 2026-09-27 | session-weiter Consent (`/consent`)
Commit-Wort (`/commit`) — pfad-begrenzter Commit + Push, das Doppel-Ask | 2026-09-27 | Operator
„hast du alle eigenen punkte bis zur kante geplant?" | 2026-09-29 | Operator (Session, River 63)
„Jede Linie kennt ihr Haus wie ihre Westentasche … state/ ist das Haus … keine privaten Projekte in Übergaben" | 2026-09-29 | Operator (via future-folge155, an River adressiert)
„wichtig ist nur dass die linien die nachrichten bevorzugt behandeln" | 2026-09-29 | Operator (Session, River 65)
„der Empfehlung folgen; Compiler-Standard NINO3.4 (−5…5 lat, 190…240 lon, 1854–2026), §3-Block zuerst" | 2026-09-29 | Operator (via future-folge155, an River adressiert — ENSO-Zuschnitt)
„du hast in der letzten session vergessen etwas zu committen" (Build-Heilung `main_flow.rs`, E0063 — Rivers Commit) | 2026-09-29 | Operator (Session, River 67)
„erst messen" (Membran-Tod = „zu viele Daten geladen") | 2026-09-29 | Operator (Session, River 67)
„Erst CI: M1+M2" / „Beides" (synthetisch + echter Katalog) | 2026-09-29 | Operator (Session, River 67)

## Haus — River (Stand 2026-09-29)

Diese Übergabe **ist** das Haus: Membran/`omega.rs`-Feld/Window/Gaze, TE-Maschine, Aktuatorik,
Echo — jeder Punkt mit Zustand. Fundstellen: `state/zustand/standing-pass.md` ·
`state/zustand/external-state.md` · `state/zustand/wartend.φ` · `state/zustand/ereignisse.φ` ·
`state/operator-gespraeche/` · `state/river/`. Rivers Teil: `src/mathematikerin/` (Feld, TE, WGSL),
`src/archivar/` (Query/Vlies), Membran-Pfade `main_flow`/`omega.rs`, Aktuatorik; Papiere
`docs/paper/gic-causal-driver.md` u. a.; Konzepte `docs/concepts/*`. Linien-Preset privat
`state/river/archive-search-preset.txt` (eingelesen in `.opencode/command/river.md`).

## Offen (aufgeschlüsselt)

### ci-check/ci-gate — Rot am HEAD gemessen, River-Anteil geheilt
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der `ci-check`/`ci-gate`-Lauf am HEAD nach diesem Commit.
- **Lage:** (gemessen 2026-09-30 via `ci_manage jobs 36630771763` + `log 36630771763`) `ci-gate @869416dbb`
  **failure**: Job `clippy` rot an `src/archivar/spatial.rs:359:29` (**Mountain**, type_complexity)
  + `src/mathematikerin/te.rs:3121:16` (**River**, jetzt auf `SurrogateFn` aliast — `cargo check` 0/0);
  Job `format` rot an `te.rs:3269/:6161` + `fam_calibration.rs:1` (in diesem Atom rustfmt-rein);
  `build`/`dropped-gate` **success**. `ci-check 36630771757` in_progress.
- **Blockade:** keine.
- **Braucht:** `ci_manage view <ci-check-id>` am neuen HEAD. Mountain-Anteil `spatial.rs:359` trägt Mountain.

### te-gate — WebGL-Naht / fpr-membrane
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der `te-gate`-Lauf am neuen HEAD.
- **Lage:** (gemessen 2026-09-29 via `ci_manage jobs 36610589166`) `fpr-membrane` FAILED
  (`Zug 5: FPR 9.52% at a=0 D_Z=0 exceeds 8%`); `fpr-ksg-shift`/`ksg-sweep`/`fpr-ksg-arx` in_progress;
  `36622237567` queued (keine Jobs).
- **Blockade:** keine.
- **Braucht:** `ci_manage view 36610589166` / `ci_manage jobs 36610589166`.

### TE-Null-Riss — FPR unter Autokorrelation (#13 / #43)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** die vier Messungen (unten) am `fpr-membrane`-Arm.
- **Lage:** (gemessen 2026-09-30 via `ci_manage log 36610589166`) der a=0-Arm fällt (9.52 % > 8 %;
  a=0.5 0.00 %, a=0.9 4.76 %). **Rat-Verdikt (Rat-Session 2026-09-30):** kein Null-Switch, kein
  a-differenzierter Test (beides passt das Instrument an die Messung an, A ≠ A). Der a=0-Exzess
  ist bei 21 Trials = 2/21 (95 %-KI [1,2 %, 30,4 %], die 8 %-Linie liegt im KI) unterbestimmt;
  ARX/Shift/Block sind bei a=0 asymptotisch derselbe Null wie Phase. Selektionsbias-Kandidat:
  `neg` zählt bedingt auf messbare Zellen (None-Quote bei a=0 = unmessbar mitgezählt).
- **Blockade:** keine (Rat-Wort liegt vor: erst messen).
- **Braucht:** in derselben Batterie: (1) None-Quote je Arm, (2) Trials 21 → 2⁶/2⁷, (3) n_surr-Sweep
  {10,100} bei a=0, (4) Gegenprobe `topological_te_estimate_frozen` (τ eingefroren). Erst wenn der
  Exzess nach Selektionskontrolle strukturell bleibt, Switch auf `topological_te_arx` + die vier
  Kalibrier-Gates (`te.rs`) umschreiben.

### Membran-Volumen-Messung M1+M2 (P11)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der `membrane-volume-probe`-Lauf + der `ci-check`-Lauf (Synthetic-Test).
- **Lage:** (gemessen 2026-09-29 via `grind-flash` + `cargo check --tests` 0/0) `membrane_hull_probe`
  trägt jetzt eine `QUERY`-Sektion (`star_cells`/`bounded_cells`/`records`/`frame_bytes`/`build_ms`/
  `query_ms`); neuer Test `test_star_grid_hull_bounds_synthetic_catalog` (N∈{100,1000,10000}, `records==1`);
  Workflow `membrane-volume-probe.yml` neu. Der Lauf selbst ist **ungemessen**.
- **Blockade:** keine.
- **Braucht:** Lauf `membrane-volume-probe 36639392112` (dispatcht 2026-09-30) lesen (`records` vs.
  `frame_bytes`); `ci-check` liest den Synthetic-Test.

### Rätsel Ⅰ — Jeans-Engine, σ-Asset
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Mountain re-manifestiert das 56-Byte-`dr3_stars.bin`.
- **Lage:** (gemessen 2026-09-29 via `general`) der Parser liest σ nur bei 56-Byte-Stride
  (`spatial.rs:410-429`); CDN-`dr3_stars.bin` = 75 001 828 B, `%44==0`, `%56==20` → legacy
  44-Byte-Stride → σ = `None` in Produktion; der Compiler schreibt bereits 56 (`tycho2_compiler.rs:7`),
  der Upload fehlt. **Der σ-Schätzer-Sockel ist ohne die 56-Byte-Asset-Version nicht baubar.**
- **Blockade:** Asset-Version (Mountain).
- **Braucht:** Mountain re-manifestiert die 56-Byte-Version; danach `archive_search --sniff <url>` + σ-Zensus.

### GIC-Paper — Review-Auflagen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Re-Dispatch der zwei Läufe + Operator-Wort zur Einreichung (LOCK).
- **Lage:** (gemessen 2026-09-29 via `ci_manage`) `fam-scalar-calibration 36621199997` und
  `bz-yearly-nsurr100 36621205592` fielen am E0063 → **void**; nach der Build-Heilung neu zu dispatchen.
- **Blockade:** Build-Heilung (dieses Atom) + Einreichungs-LOCK.
- **Braucht:** Re-Dispatch 2026-09-30: `fam-scalar-calibration 36639382300` + `bz-yearly-nsurr100 36639387218`;
  dann `ci_manage view`. Future-156-Auflagen (Null kalibrieren, rundenvergleichbar, Lag-0, überzeichnete
  Sätze zurücknehmen) bleiben vor dem LOCK-GEMS-Send.

### ENSO TE-Probe — §3-Block (Blatt I)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** das Rat-Wort über die Blatt-I-Frage (§3-Block) — bis dahin trägt `tools/measure/src/bin/enso_blatt_probe.rs` die gemessene Stille.
- **Lage:** (gemessen 2026-09-29 River 66) Riss: der §3-Block trägt kein ENSO/LAIC/NINO; die Probe ist still.
- **Blockade:** die Entscheidung.
- **Braucht:** Rat/Operator.

### flyby-path2-recon
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ESOC publiziert einen SKD `v474+`.
- **Lage:** (gemessen 2026-09-29 River 62) ABSENT, Enumeration endet `juice_cog_000114_…`.
- **Blockade:** Publikation fehlt.
- **Braucht:** nach Publikation `flyby_ephemeris_gate --recon …`.

### Weberin-Lücke — Vlies-Dichte ω()-Term (Träger: `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md`)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der `ci-check`-Lauf am neuen HEAD.
- **Lage:** (gemessen 2026-09-29 River 65/66) derived-field-Term gebaut (`vlies.rs`, `s2.rs`, `omega.rs`);
  das Survey §7 trägt den Bau-Atom (Rat-Verdikt Option a); Träger dieses Handover.
- **Blockade:** keine.
- **Braucht:** Lauf lesen; danach der Bau-Atom.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). Pfad-begrenzte
Commit-Pfade dieses Atoms:

`src/archivar/main_flow.rs` (σ-Felder, Build-Heilung — 6 Zeilen) ·
`tools/measure/src/bin/membrane_hull_probe.rs` (QUERY-Sektion) ·
`src/archivar/tests.rs` (Synthetic-Skalierungs-Test) ·
`.github/workflows/membrane-volume-probe.yml` (neu) ·
`docs/handover/handover-2026-09-29-river-folge67.md` (neu) ·
der folge66-Move nach `docs/handover/archiv/`.

Privat/gitignored (nicht committet): `state/operator-gespraeche/2026-09-29-river.md` · `state/river/`.
Fremde uncommittete Arbeit unangetastet: `src/mathematikerin/te.rs`,
`tools/harvest/src/bin/infrared_anomaly_compiler.rs`,
`tools/measure/src/bin/direction_distance_join.rs`, `tools/measure/src/bin/fam_calibration.rs`,
`tools/measure/src/bin/vlies_density_probe.rs`.
