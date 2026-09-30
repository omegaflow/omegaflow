<!--
  title: Handover — River-Folge 69 (2026-09-30)
  session: River-Folge 69
  class: handover
  date: 2026-09-30
  sha256: c84d2a4a6a7e3c0befd113866e558a5cbbf4271690844605ca61a1b51f4a9b96
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
- **Trigger:** der `fpr-diagnostic`-Job im `te-gate`-Lauf am neuen HEAD.
- **Lage:** (gemessen 2026-09-30 via `ci_manage jobs/log 36639308643`) `fpr-membrane` FAILED —
  `Zug 5: FPR 9.52% at a=0 D_Z=0 exceeds 8%` (a=0.5 0.00 %, a=0.9 4.76 %); die übrigen 17 Jobs
  success. Die vier Rat-Messungen sind **gebaut**: Test `gate_membrane_fpr_diagnostic`
  (`src/mathematikerin/te.rs`) — None-Quote je Arm, trials 2⁶/2⁷, n_surr {10,100}, frozen-τ
  (Flag in `topological_te_with`) — plus Job `fpr-diagnostic` in `.github/workflows/te-gate.yml`
  (`cargo check` 0/0, `cargo fmt` sauber).
- **Blockade:** keine.
- **Braucht:** nach Commit `gh workflow run te-gate.yml`; dann `ci_manage jobs <id>` +
  `ci_manage log <id> --all` und die `membrane-diag`-Zeilen lesen. Erst wenn der a=0-Exzess nach
  Selektionskontrolle strukturell bleibt, Switch auf `topological_te_arx` + die vier
  Kalibrier-Gates (`te.rs`).

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
- **Lage:** (gemessen 2026-09-30 via Rat) **Rat-Beschluss:** LAIC = Lithosphäre→Atmosphäre→Ionosphäre
  (kein „Longwave"); der Dreikanal = **Wind** (`tao_wnd_zonal.csv`, `phi/sources.φ:774-786`) /
  **Lithosphäre** (USGS-comcat-Monatsrate, Asset fehlt → `pending`) / **Bz** (vorhanden) auf
  **NINO3.4** (`ersstv5_nino34`, gebunden), Monats-Grid, Lag 0…12, Null μ+2σ über 100 Surrogate
  **plus fam** (das 78-Zellen-Runden-Maximum, sonst FDR-Wette im fam-Paper); Verdikte
  arrow/family bound/silent; NINO→Bz still (Strukturkontrolle). Das Katalog-Asset fehlt — die
  Litho-Zelle ist `pending`, nie 0.0.
- **Blockade:** keine.
- **Braucht:** Bau-Atom `tools/measure/src/bin/enso_blatt_probe.rs` (`TAO_CSV_CDN` bei `:13`,
  `load_wind()` bei `:67-76`, Runde `:193-228`, LAIC-`pending`-Zelle `:253-260`) +
  `src/mathematikerin/te.rs:1583` (`surrogate_max`-Variante, durch die vier Kalibrier-Gates) +
  §3.6 in `docs/paper/gic-causal-driver.md:234`; die comcat-Compiler-Pflicht (M ≥ 4.5,
  1973-01-01…2026-08-01, `usgs_comcat_m45.bin`) an Mountain (`## An mountain`).

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

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). Pfad-begrenzte
Commit-Pfade dieses Atoms:

`src/mathematikerin/te.rs` (Diagnose-Batterie + `frozen_tau`-Flag) ·
`.github/workflows/te-gate.yml` (`fpr-diagnostic`-Job) ·
`docs/paper/gic-causal-driver.md` (Review-Auflagen: Null kalibriert, rundenvergleichbar,
Lag-0-Randbin, entschärfte Sätze) ·
`docs/surveys/survey-2026-09-26-membran-ladearchitektur.md` (M1+M2-QUERY-Nachtrag) ·
`docs/handover/handover-2026-09-30-river-folge69.md` (neu) ·
der folge68-Move nach `docs/handover/archiv/`.

Privat/gitignored (nicht committet): `state/operator-gespraeche/2026-09-30-river.md` · `state/river/`.
Fremde uncommittete Arbeit unangetastet: `phi/blocked_sources.φ` (nicht Rivers).
