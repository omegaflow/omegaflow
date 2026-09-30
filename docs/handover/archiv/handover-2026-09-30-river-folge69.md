<!--
  title: Handover — River-Folge 69 (2026-09-30)
  session: River-Folge 69
  class: handover
  date: 2026-09-30
  sha256: 71e453718570f9d53a80ec5266004aa41340e7f5f9dba9b0b18956d6a213eaf1
  status: live
-->
# Handover — River-Folge 69 (2026-09-30)

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
„Du kannst. Führe den Plan aus — als `line`-Agent (auto-bestätigt). Dies ist der session-weite Consent (Delegation), nicht das Commit-Wort." | 2026-09-30 | Operator (Session, River 68)
„bitte wirklich bis zur Kante umsetzen nicht nur wieder messen und verschleppen" | 2026-09-30 | Operator (Session, River 69 — Planungsmodus)

## Haus — River (Stand 2026-09-30)

Diese Übergabe **ist** das Haus: Membran/`omega.rs`-Feld/Window/Gaze, TE-Maschine, Aktuatorik,
Echo — jeder Punkt mit Zustand. Fundstellen: `state/zustand/standing-pass.md` ·
`state/zustand/external-state.md` · `state/zustand/wartend.φ` · `state/zustand/ereignisse.φ` ·
`state/operator-gespraeche/` · `state/river/`. Rivers Teil: `src/mathematikerin/` (Feld, TE, WGSL),
`src/archivar/` (Query/Vlies), Membran-Pfade `main_flow`/`omega.rs`, Aktuatorik; Papiere
`docs/paper/gic-causal-driver.md` u. a.; Konzepte `docs/concepts/*`. Linien-Preset privat
`state/river/archive-search-preset.txt` (eingelesen in `.opencode/command/river.md`).

## Adressierte Blöcke — gefaltet 2026-09-30

- **mycelium-208** (`format`+`clippy`): am HEAD `62f494db4` gemessen **grün** — die Jobs
  `clippy`/`format`/`build` sind success (`ci_manage jobs 36676970377`); der Block ist
  beantwortet, kein Bau.
- **sensory-210**: die Absolutpfad-Meldung aus river-folge65 ist am Baum widerlegt (0 Scan-Marker;
  `folge65` liegt im `archiv/`) — Block geschlossen, kein Bau.

## Offen (aufgeschlüsselt)

### TE-Null-Riss — FPR unter Autokorrelation (#112 / #13 / #43)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** das Rat-/Operator-Wort über die Gate-Konsequenz (Sample size vs. 8%-Linie).
- **Lage:** (gemessen 2026-09-30 via `ci_manage log 36680694453 --all`; Job `fpr-diagnostic`
  **success**) die vier Rat-Messungen: **`none=0` in jeder Zeile** → kein Selektionsbias;
  a=0 FPR 9.52 % (2/21) → **4.69 % (3/64)**, 7.81 % (5/64, zweiter Draw), **5.47 % (7/128)**;
  n_surr=100 bei a=0 4.69 % (3/64); frozen-τ bei a=0 3.12 % (2/64). Der a=0-Exzess ist
  **nicht strukturell** (bei n≥64 unter der 8 %-Linie, kein n_surr-/frozen-τ-Effekt) — der
  21-Trial-Rot war Small-Sample. `fpr-membrane` bleibt rot (2/21), erwartet.
- **Blockade:** keine.
- **Braucht:** kein Arx-Switch (Selektionskontrolle negativ). Die eine offene Entscheidung:
  `gate_membrane_fpr_phase_vs_arx_n_1000` auf 2⁶/2⁷ Trials heben oder die 8 %-Linie lassen —
  Rat/Operator, kein stiller Eingriff ins Instrument. `te-gate 36680694453` läuft noch
  (`fpr-ksg-shift`/`fpr-ksg-arx`/`ksg-sweep`); diese drei einmalig nachlesen.

### ci-check — cargo-test GPU-Rot (#58 / #15)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der `ci-check`-Lauf `36676970291` am HEAD.
- **Lage:** (gemessen 2026-09-30 via `ci_manage status`) **pending**. Der `ci-gate`-Anteil ist am
  HEAD geheilt (clippy/format/build green; `ci_manage jobs 36676970377`).
- **Blockade:** keine.
- **Braucht:** `ci_manage jobs 36676970291` nach Abschluss; bei Rot `ci_manage log 36676970291`.

### Membran-Volumen M1+M2 (P11) — Lauf-Ergebnis
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** keiner — der Lauf ist gelaufen; die Auswertung.
- **Lage:** (gemessen 2026-09-30 via `ci_manage log 36639392112 --all`) Lauf **success**;
  `QUERY | star_cells 1704587 | bounded_cells 0 | records 32361 | frame_bytes 6731088 |
  build_ms 481.129 | query_ms 237.873`; `membrane-hull tally: 0 of 4 path(s) diverge from the
  resting cone`; `dastcom_asteroids.bin read void`.
- **Blockade:** keine.
- **Braucht:** die QUERY-Zeile in den Träger
  `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md` eintragen; `bounded_cells 0` gegen
  die Hull-Erwartung halten (Riss-Kandidat).

### GIC-Paper — Review-Auflagen abgearbeitet, Export ausstehend
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der `paper-check`-Lauf am neuen HEAD (Export-Abgleich).
- **Lage:** (gemessen 2026-09-30 via `gh run download` + Manuskript-Edits) beide Läufe **success**
  (`fam-scalar-calibration 36639382300`, `bz-yearly-nsurr100 36639387218`). Die vier Auflagen
  (future-157:249-252) sind in `docs/paper/gic-causal-driver.md` eingearbeitet: **Null kalibriert**
  (§3.2/§6 — FWER 7.83/9.50/9.50 % bei n_surr=10, nominal 9.09 %; 1.67/0.00/1.67 % bei n_surr=100,
  nominal 0.99 %); **rundenvergleichbar** (§6 — Jahres-Runde bei n_surr=100 nachgelaufen, Arrow hält:
  ABK 2024 1.2670e-1 > 1.0746e-1, ABK 2025 1.3309e-1 > 1.2802e-1, SOD 2024 1.1695e-1 > 1.1091e-1;
  fam-Spread 0.10557 vs 0.10746 als Riss benannt); **Lag-0 aus der Artefaktzone** (Abstract + §7:
  die Jahres-Pfeile liegen am Lag-0/1-Randbin, der Lag bleibt unaufgelöst); **überzeichnete Sätze**
  (§5-Überschrift, §7 entschärft).
- **Blockade:** keine.
- **Braucht:** `paper-check` am neuen HEAD (der `.tex`-Export unter `docs/paper/export/` muss
  nachgezogen sein); der GEMS-Send läuft über Future (LOCK, `## An future`).

### ENSO TE-Probe — §3-Block (Blatt I)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der Bau des §3-Blocks (der Rat hat am 2026-09-30 entschieden).
- **Lage:** (gemessen 2026-09-30 via Rat + `sread`/`sgrep`) **Rat-Beschluss:** LAIC =
  Lithosphäre→Atmosphäre→Ionosphäre (kein „Longwave"); Dreikanal = **Wind** / **Lithosphäre**
  (comcat) / **Bz** auf **NINO3.4**, Monats-Grid, Lag 0…12, Null μ+2σ **plus fam**; Verdikte
  arrow/family bound/silent. **Gebaut:** `surrogate_max_phase_n` in `src/mathematikerin/te.rs`
  (additiv, berührt die Kalibrier-Gates nicht) + fam über die Bz↔SST-Runde in
  `tools/measure/src/bin/enso_blatt_probe.rs` (Verdikt dreistufig) + §3.6 in
  `docs/paper/gic-causal-driver.md` (gebaut: Bz↔SST; Wind/LAIC `pending`). **Riss (gemessen):**
  das Wind-Asset `tao_wnd_zonal.csv` ist ein **120-Tage-Live-Fenster** (`tao_wnd_compiler.rs`
  holt `d_end−120 d … d_end−7 d`), obwohl die Quelle bis 1977-11-06 reicht (`phi/harvest.φ:271`) —
  der Rat nahm eine historische Reihe an; die Source-Seite korrigiert das.
- **Blockade:** keine (eigen); Wind/LAIC hängen an Mountain-Ports.
- **Braucht:** Wind-Kanal = Compiler-Erweiterung auf das volle 1977+-Record
  (`tools/harvest/src/bin/tao_wnd_compiler.rs`) + Re-Manifestation → Mountain/Mycelium
  (`## An mountain`); LAIC = comcat-Asset (M ≥ 4.5, 1973-01-01…2026-08-01,
  `usgs_comcat_m45.bin`) → Mountain. Danach `cargo run -p omegaflow-measure --bin enso_blatt_probe`
  im CI-Bündel.

### Weberin-Lücke — Vlies-Dichte ω()-Term
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der `ci-check`-Lauf `36676970291` am HEAD.
- **Lage:** (gemessen 2026-09-30 via River 65/66) derived-field-Term gebaut (`src/archivar/vlies.rs`,
  `src/mathematikerin/s2.rs`, `src/mathematikerin/omega.rs`); Träger
  `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md` §7.
- **Blockade:** keine.
- **Braucht:** `ci-check`-Lauf lesen; danach der Bau-Atom.

### Rätsel Ⅰ — Jeans-Engine, σ-Asset
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Mountain re-manifestiert das 56-Byte-`dr3_stars.bin`.
- **Lage:** (gemessen 2026-09-29 via `general`) der Parser liest σ nur bei 56-Byte-Stride
  (`src/archivar/spatial.rs:410-429`); CDN `dr3_stars.bin` = 75 001 828 B, `%56==20` → 44-Byte-Stride
  → σ = `None`; `tools/harvest/src/bin/tycho2_compiler.rs:7` schreibt 56, der Upload fehlt.
- **Blockade:** Asset-Version (Mountain).
- **Braucht:** Mountain re-manifestiert; danach `archive_search --sniff <url>` + σ-Zensus.

### flyby-path2-recon
- **Status:** wartend | **Bindung:** eigen (Aufenthalt `state/zustand/wartend.φ:29`)
- **Trigger:** ESOC publiziert einen SKD `v474+`.
- **Lage:** (gemessen 2026-09-29 River 62) ABSENT, Enumeration endet `juice_cog_000114_…`.
- **Blockade:** Publikation fehlt.
- **Braucht:** nach Publikation `flyby_ephemeris_gate --recon …`.

## An future

Origin: river-folge69.

**GIC-Einreichung (Send an GEMS/Gutachter) = LOCK** (Operator-Wort 2026-09-29, stehende
Regel in `AGENTS.md`). Die Vorbereitung steht: `docs/paper/gic-causal-driver.md` trägt die
vier Review-Auflagen aus future-157:249-252 eingearbeitet (Null kalibriert,
rundenvergleichbar, Lag-0 aus der Artefaktzone, überzeichnete Sätze). Vor dem Send offen:
`paper-check` grün am neuen HEAD + der Repository-/Software-DOI-Knoten
(`docs/paper/gic-causal-driver.md` „Data Availability"/"Software Availability", beide
„citable … pending"). Trigger: Operator-Wort zur Einreichung.

**ENSO-Dreikanal — Definition zur Operator-Vorlage** (Rat 2026-09-30, Riss i): der Begriff
war nie definiert; der Rat legt fest: Wind (`tao_wnd_zonal.csv`) / Lithosphäre (comcat-Rate,
`pending`) / Bz auf NINO3.4, mit fam. Das Operator-Wort („§3-Block zuerst") deckt den
Zuschnitt, nicht die Kanalnamen — dem Operator im nächsten Pass einmal vorlegen.

## An mountain

Origin: river-folge69.

**`dropped-gate` rot am HEAD `62f494db4`** (gemessen 2026-09-30 via `ci_manage log 36676970377`):
`dropped-gate: baseline 1148 | current 1157 | delta 9 — open points dropped without a resolving
commit`. Die zusätzlichen `register_lookup --dropped`-Zeilen sind `entscheid`-Altzeilen (archiv/,
persist 1–57, `git: resolved`/`none`). Der Gate ist haus-weit; die Baseline
(`docs/zustand/dropped-baseline.md`) ist stale.
Braucht: `register_lookup --dropped` je Linie attribuieren und die Baseline im annehmenden Commit
bumpen — oder die Zeilen vorwärts tragen.

**USGS-comcat-Katalog-Asset fehlt** (Rat 2026-09-30, ENSO-§3). Der Dreikanal braucht die
monatliche comcat-Rate (M ≥ 4.5, 1973-01-01…2026-08-01, paginiert, `usgs_comcat_m45.bin`)
als Lithosphären-Kanal; USGS-FDSN lebt als Live-Quelle (`phi/sources.φ:96/:103`), das
historische Katalog-Asset ist ein Register-Gap. Port = Mountain, Manifestation = Mycelium.
Braucht: comcat-Compiler bauen (Parser + Pagination), Asset in `phi/sources.φ` registrieren.

**`tao_wnd_zonal.csv` ist ein 120-Tage-Live-Fenster** (gemessen 2026-09-30 via
`tools/harvest/src/bin/tao_wnd_compiler.rs:27-28` — `d_end = now−7 d`,
`d_start = d_end−120 d`), obwohl die Quelle bis 1977-11-06 reicht
(`phi/harvest.φ:271`). Der ENSO-§3-Block braucht die historische Reihe. Port =
Mountain: den Compiler auf das volle Record erweitern (paginiert, `time`-Fenster
groß genug), danach Re-Manifestation (Mycelium).
Braucht: `tao_wnd_compiler.rs` Zeitfenster öffnen + Asset re-manifestieren.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). Pfad-begrenzte
Commit-Pfade dieses Atoms:

Zweiter Atom (`31eed91de` = erster bereits gepusht): `src/mathematikerin/te.rs`
(`surrogate_max_phase_n`) ·
`tools/measure/src/bin/enso_blatt_probe.rs` (fam über die Bz↔SST-Runde, dreistufiges
Verdikt, `pending`-Zeilen) ·
`docs/paper/gic-causal-driver.md` (§3.6 ENSO-Dreikanal) ·
`docs/handover/handover-2026-09-30-river-folge69.md`.

Privat/gitignored (nicht committet): `state/operator-gespraeche/2026-09-30-river.md` · `state/river/`.
Fremde uncommittete Arbeit unangetastet: `phi/blocked_sources.φ` (nicht Rivers).
