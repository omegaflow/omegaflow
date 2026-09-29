<!--
  title: Handover — River-Folge 66 (2026-09-29)
  session: River-Folge 66
  class: handover
  date: 2026-09-29
  sha256: 0968d0ee171c7247c1c47ad16705d2345461ac659eb06c8d7133278d9be1caf9
  status: live
-->
# Handover — River-Folge 66 (2026-09-29)

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
„Einmalige Modell-Messung am GIC-Review-Atom. Arbeite die Auflagen aus ## An river (aus future-folge156) zuerst mit grind-flash. Erreicht flash das Ziel nachweislich nicht (die falsche/unvollständige Stelle benennen), schalte über das pro/max-ask auf grind-max. Trage Sieger + Burn (session_burn, beide Arme) als eine Handover-Zeile ein. Kein synthetischer Benchmark, keine Regel in AGENTS.md — nur diese Session misst." | 2026-09-29 | Operator (Session, River 66)
„Antwort: ‚Die 3 verbliebenen Auflagen' (Option 1) — nicht die breite Neu-Auditierung … der Modellvergleich wird am Kalibrierungs-Arm gemessen (flash zuerst, bei nachweislichem Fehlschlag über das ask auf grind-max; Sieger + Burn als eine Handover-Zeile)." | 2026-09-29 | Operator (Session, River 66)

## Haus — River (Stand 2026-09-29)

Diese Übergabe **ist** das Haus: Membran/`omega.rs`-Feld/Window/Gaze, TE-Maschine, Aktuatorik,
Echo — jeder Punkt mit Zustand. Ein Punkt, der nur in `state/` lebt und hier fehlt, ist ein
verlorener Punkt. Vier Orte: `omegaflow` (`~/projects/omegaflow` + privates Schwester-Repo `state/`),
`omegaflow-legacy` (`archive-root/omegaflow-legacy` + Backup), `temp` (`/tmp/opencode`),
`archive` (`archive-root`; `~/backup/{archive,provenance}/*`). Fundstellen:
`state/zustand/standing-pass.md` · `state/zustand/external-state.md` · `state/zustand/wartend.φ` ·
`state/zustand/ereignisse.φ` · `state/operator-gespraeche/` ·
`state/river/`. Rivers Teil: `src/mathematikerin/` (Feld, TE, WGSL), `src/archivar/` (Query/Vlies),
Membran-Pfade `main_flow`/`omega.rs`, Aktuatorik; Papiere `docs/paper/gic-causal-driver.md` u. a.;
Konzepte `docs/concepts/*`. Linien-Preset privat `state/river/archive-search-preset.txt`
(eingelesen in `.opencode/command/river.md`).

**Modell-Messung GIC-Review-Atom (2026-09-29): Sieger `grind-max` am Kalibrierungs-Arm — flash $0.1030 (`grind-flash`) baute `tools/measure/src/bin/fam_calibration.rs` + `bz_retro_probe --yearly-round`, scheiterte aber am lauffähigen Lauf (CI-Job ~8 h > Timeout, aus dem eigenen 13-min/8-Trial-Lauf gemessen); max $0.1381 (`grind-max`) verifizierte flashs Design (Nominal `1/(n_surr+1)`, lag-0≡lag-1, kanonischer Null), machte die Batterie deterministisch-parallel (`--threads`, byte-identisch) und den Jahreslauf 4-threaded (80,8 min statt ~5,4 h), baute den 6-Shard-CI-Job innerhalb des Timeouts (184,5 min bzw. 169,4 min, gemessen) und korrigierte das Paper — beide Betriebs-n-Zahlen bleiben `pending Dispatch` (kein Commit im Arm).**

## Offen (aufgeschlüsselt)

### GIC-Paper — Review-Auflagen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Operator-Wort zur Einreichung (LOCK).
- **Lage:** (gemessen 2026-09-29 River 66 via `grind-flash`/`grind-max` + `cargo check`) die sechs Befunde sind im Paper revidiert (Header-sha `720bd0f9…`). Von den drei verbliebenen Auflagen: **(a) fam-FWER-Kalibrierung** — Skalar-Runden-Max-Batterie gebaut (`tools/measure/src/bin/fam_calibration.rs`), Design verifiziert (Nominal `C/(C+C·n_surr)=1/(n_surr+1)`; `phase_randomized_surrogate` kanonisch); lokale bound Runs 8–10 Trials (FWER 0/10 a=0, 1/10 a=0.9; FPR 0/120 bzw. 2/120), **Betriebs-n-Zahl pending Dispatch**; CI-Job `fam-scalar-calibration.yml` (6 Shards, nsurr10 600 Trials / nsurr100 60 Trials, je <300 min) — ungemessen. **(b) Jahres-Runde @ n_surr=100** — `bz_retro_probe.rs --yearly-round` (6 Paare lag 0/1, per-Lag TE/threshold/fam) + `bz-yearly-nsurr100.yml` (ABK 2024/2025, SOD 2024), deterministisch-parallel; Datenpfad `--verdict` HTTP 206; **Lauf pending Dispatch**. **(c) Jahres-lag-1-Zeilen** — resolved: `transfer_entropy_lag(x,y,0)` (`te.rs:96-99`) dispatcht auf `transfer_entropy` (`x[t+1]`, `te.rs:33`) ≡ `transfer_entropy_lag(x,y,1)`; lag-0≡lag-1 exakt gemessen (`0.21722066662653602`), die Jahres-Familie ist **6 distinkte** Paare, die tabellierten `0 h`-Zeilen **sind** die lag-1-Zeilen. n=2200 (oberes Betriebs-n) bleibt eigener, noch nicht gebauter Job.
- **Blockade:** die Betriebs-n-Zahlen brauchen den CI-Dispatch (Workflows uncommittet → `workflow_dispatch` 404).
- **Braucht:** die zwei Workflows dispatchen (`gh workflow run fam-scalar-calibration.yml` · `gh workflow run bz-yearly-nsurr100.yml`), dann die Ergebnisse in §4.2–4.4/§6 lesen und eintragen; dann Operator-Wort zur Einreichung.

### CI-Verifikation — `ci-check` am HEAD
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der `ci-check`-Lauf auf dem HEAD nach diesem Commit schließt ab.
- **Lage:** (gemessen 2026-09-29 River 66 via `ci_manage`/`general`) am HEAD `250d76056` war `ci-check 36610572567` **pending**; inzwischen ist HEAD `396fd2598` (mountain 206). Der frühere absolute-Pfad-Scan ist geheilt (`river-65` + Mountain/Sensory).
- **Blockade:** keine (nur CI).
- **Braucht:** `ci_manage status`/`view` am aktuellen HEAD lesen; bei rot den gemessenen Grund prüfen.

### ci-gate — clippy + dropped-gate
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der `ci-gate`-Lauf auf dem HEAD nach diesem Commit schließt ab.
- **Lage:** (gemessen 2026-09-29 River 66 via `ci_manage log 36610572360`) `ci-gate 36610572360` = failure: Job `clippy` (`-D warnings`) → `manual_unwrap_or_default` `te.rs:3277` und `type_complexity` `te.rs:6168` (beide River-Code, **in diesem Atom geheilt**: `unwrap_or_default()` + Alias `MembraneVerdictFn`); Job `dropped-gate` → `baseline 1134 | current 1135 | delta 1` (Issue #81).
- **Blockade:** der `dropped-gate`-Delta 1 braucht eine Ursachenmessung (welcher Punkt ohne auflösenden Commit fiel).
- **Braucht:** `ci-gate`-Lauf lesen; bei `dropped-gate` delta 1 `register_lookup --dropped` bzw. die Baseline-Diff prüfen.

### WebGL-Naht — `te_compute` Horizont
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der `te-gate`-Lauf auf dem HEAD nach diesem Commit schließt ab.
- **Lage:** (gemessen 2026-09-29 River 66 via `ci_manage jobs`) `te-gate 36610589166` am HEAD `250d76056` = **in_progress**; Horizon-Parity-Jobs (`lag-sweep`, `mi-lag`, `probe`, `ksg-k-gate`, `flare`) success; `fpr-membrane` failure (s. u.); weitere FPR-Jobs noch offen.
- **Blockade:** keine (nur CI).
- **Braucht:** den `te-gate`-Lauf abschließen lassen und lesen (`ci_manage view`).

### TE-Null-Riss — FPR unter Autokorrelation (#13 / #43)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** das Rat-Wort über den Null-Switch nach der gemessenen Batterie.
- **Lage:** (gemessen 2026-09-29 River 66 via `ci_manage log 36610589166`) die Batterie `gate_membrane_fpr_phase_vs_arx_n_1000` (Job `fpr-membrane`) ist **gelaufen und FAILED**: `Zug 5: FPR 9.52% at a=0 D_Z=0 exceeds 8%` — (a=0 9.52 %, a=0.5 0.00 %, a=0.9 4.76 %). Der **a=0**-Arm fällt, nicht a=0.9; der folge65-Zettel („Phase marginal 8.57 %") ist damit präzisiert. Option 2 (Membran→Arx) ist conditional on the battery — der a=0-Ausfall stützt den Switch an der Wurzel.
- **Blockade:** Rat-Wort über den Switch (kein Null-Wechsel vor dem Wort).
- **Braucht:** Rat hören mit dem gemessenen a=0-Ausfall; bei Switch die vier Kalibrier-Gates auf den neuen Null umschreiben.

### ENSO TE-Probe — §3-Block (Blatt I)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** das Rat-Wort über die Blatt-I-Frage (§3-Block ENSO) — bis dahin trägt `tools/measure/src/bin/enso_blatt_probe.rs` die gemessene Stille.
- **Lage:** (gemessen 2026-09-29 River 66 via `general`) **Riss:** der Operator-Zuschnitt verlangt den „§3-Block (ENSO/Bz/LAIC) zuerst", aber `docs/paper/gic-causal-driver.md` §3 (`:106`) trägt **kein** ENSO/LAIC/NINO — nur `docs/paper/laic-arrow-direction.md`. Die ENSO-Probe (`tools/measure/src/bin/enso_blatt_probe.rs`, n_surr=100) ist still (Bz→SST lag 8 = 2.07e-1 < 2.27e-1; SST→Bz lag 9 = 2.27e-1 < 2.39e-1); der NINO3.4-Zuschnitt ist im Compiler (`ersstv5_compiler.rs:9,14-17`).
- **Blockade:** die Blatt-I-Frage (Wind↔SST) vs. das registrierte Paar (Bz×SST) ist eine Entscheidung.
- **Braucht:** Rat/Operator-Entscheidung; dann §3-Block mit der gemessenen Stille bauen.

### Rätsel Ⅰ — Jeans-Engine, Zensus gesetzt
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Mountain liefert den erweiterten `dr3_stars`-Record samt neuer Asset-Version (CDN).
- **Lage:** (gemessen 2026-09-29 River 66 via `git log`) Mountain hat den Record erweitert (`323b85a2f` „extend the star record with sigma"); ob `StarRec` (`src/archivar/spatial.rs:53`) σ_ϖ/σ_pm trägt und die neue Asset-Version manifestiert ist, ist **neu zu messen**. Der Schätzer-Sockel bleibt ohne die σ-Spalten nicht baubar.
- **Blockade:** Asset-Version/σ-Spalten im Record.
- **Braucht:** den aktuellen `StarRec` + die CDN-Asset-Version messen; dann `--estimator` auf der plx>5-Subprobe (symmetrisierte |z|-Bins, Fehlerkorrektur Pflicht).

### flyby-path2-recon
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ESOC publiziert einen SKD `v474+` mit `juice_cog_000115_…` oder einer `*recon*`-SPK.
- **Lage:** (gemessen 2026-09-29 River 62) ABSENT, Enumeration endet `juice_cog_000114_230416_261003_v01.bsp`.
- **Blockade:** Publikation fehlt.
- **Braucht:** nach Publikation `flyby_ephemeris_gate --recon …`.

### Weberin-Lücke — Prosa-Träger `membran-ladearchitektur`
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der `ci-check`/`te-gate`-Lauf auf dem HEAD nach diesem Commit schließt ab.
- **Lage:** (gemessen 2026-09-29 River 65/66) derived-field-Term gebaut (`vlies.rs`, `s2.rs`, `omega.rs`), `cargo check`/`--tests` 0/0; Survey-Marker `:234-235` nachgezogen.
- **Blockade:** keine.
- **Braucht:** Lauf lesen.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). Pfad-begrenzte
Commit-Pfade dieser Session:

`docs/handover/handover-2026-09-29-river-folge66.md` (neu; Faltung future-156, mountain-206, mycelium-207, sensory-208) ·
der folge65-Move nach `docs/handover/archiv/` ·
`tools/measure/src/bin/fam_calibration.rs` (neu) ·
`tools/measure/src/bin/bz_retro_probe.rs` (`--yearly-round`, `--threads`) ·
`.github/workflows/fam-scalar-calibration.yml` (neu) ·
`.github/workflows/bz-yearly-nsurr100.yml` (neu) ·
`docs/paper/gic-causal-driver.md` (fam-Batterie, Jahres-nsurr100, lag-0≡lag-1; Header-sha `720bd0f9…`) ·
`src/mathematikerin/te.rs` (clippy: `unwrap_or_default`, `MembraneVerdictFn`-Alias).

Nicht committet (fremde Hunks im geteilten Baum, unangetastet): `AGENTS.md`, `opencode.json`,
`.opencode/command/*`, die Register in `phi/`, `src/archivar/hips.rs`, `src/archivar/main_flow.rs`,
`tools/harvest/*`, `tools/measure/src/bin/direction_distance_join.rs`, `tools/measure/src/bin/vlies_density_probe.rs`,
`tools/register/src/bin/open_points_check.rs`, `.github/workflows/first14-cdn.yml`, `.github/workflows/nvss-cdn.yml`, `.github/workflows/paper-check.yml`, `.github/workflows/hips-png-cdn.yml`,
der mycelium-folge206-Archiv-Move, `handover-2026-09-29-mycelium-folge207.md`, die Streudatei `0`.

Privat/gitignored (nicht committet): `state/operator-gespraeche/2026-09-29-river.md` · `state/river/`.
