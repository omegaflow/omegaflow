<!--
  title: Handover — River-Folge 65 (2026-09-29)
  session: River-Folge 65
  class: handover
  date: 2026-09-29
  sha256: 9661480a5e2c22fed15d44f91fee53c84de8c015fd0acad17b5761032a9eeb60
  status: live
-->
# Handover — River-Folge 65 (2026-09-29)

Dieses Register trägt nur Offenes — git trägt, was gemacht wurde. Der Stehende Pass
wird zitiert, nie kopiert: `state/zustand/standing-pass.md`. Nur eigene Arbeit:
pfad-begrenzter Commit; fremde uncommittete Arbeit unangetastet.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„Erste Handlung: `sread docs/concepts/tool-forms.md`" | 2026-09-27 | Operator (Session, River 49)
„du sollst keine operator worte tragen das ist sache von future du bist bis zur kante" | 2026-09-27 | Operator (Session, River 48)
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
„Erste Handlung: `sread docs/concepts/tool-forms.md` … Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent …" | 2026-09-29 | Operator (Session, River 65) — session-weiter Delegations-Consent

## Offen (aufgeschlüsselt)

### TE-Merge — CI-Verifikation (clippy)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der `ci-check`/`ci-gate`-Lauf auf dem HEAD nach diesem Commit schließt ab.
- **Lage:** (gemessen 2026-09-29 River 65 via `ci_manage log`) Trigger gefeuert: auf
  `5d6c9c685` ist `ci-check 36506672136` = failure — **11 archivar-lib-Tests** rot
  (`pds3_img`/`pds3_table`/`pds3_binary`/`pds4`/`pds4_binary`/`gras_2c`/`hips`/`extract`,
  u. a. `ASCII_Date_Time_` statt `ASCII_Date_Time_YMD`); `ci-gate 36506672052` = failure
  mit `dropped-gate: baseline 1113 | current 1127 | delta 14` (14 DROPPED-Einträge in
  `entscheid`-Altübergaben folge6–25). Der `te.rs`-clippy (river64 `691b424f0`) ist geheilt,
  kein clippy-Fehler mehr im Log.
- **Blockade:** die 11 archivar-Tests (Mountain-Regression) halten `ci-check` rot.
- **Braucht:** Mountain heilt die archivar-Regression; danach `ci_manage log <id>` am
  neuen HEAD. Für den dropped-delta 14: Baseline-Bump im annehmenden Commit oder Träger
  der `entscheid`-Linie.

### WGSL-Naht — `te_compute` Horizont
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check` auf dem neuen HEAD grün (dann ohne äußeren Anlass dispatchbar).
- **Lage:** (gemessen 2026-09-28, Rat Q3) die CPU-Membran misst am Kreuz-Horizont τc,
  die WGSL `te_compute` (`shaders.rs:751/766`) weiter an (tx,ty); Produktion trägt
  `TE_KSG_K_PROD=0` (GPU-KSG-Slots absent), die Roh-Paritäts-Gates halten.
  (gemessen 2026-09-29 River 65: `ci-check` rot — Trigger nicht gefeuert.)
- **Blockade:** keine (nur der Trigger).
- **Braucht:** `find_cross_mi_lag` in `te_compute` spiegeln + Horizont-Parity-Gate.

### ENSO TE-Probe
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Mycelium manifestiert das `ersstv5_nino34`-CDN-Asset.
- **Lage:** (gemessen 2026-09-29 River 63 via `register_lookup ersstv5`) die Quellenzeile
  steht (`phi/sources.φ:11111-11117`: `url` + `origin` ERDDAP `nceiErsstv5` +
  `compiler tools/harvest/src/bin/ersstv5_compiler.rs` + `field ersstv5_nino34_ssta`);
  der Compiler existiert; der Blatt-/Paar-Zuschnitt ist operator-gebunden
  (`state/zustand/wartend.φ:23` `blatt-zuschnitt`, Aufnehmer river).
- **Blockade:** CDN-Manifestation (Mycelium) + Operator-Zuschnitt des Paares.
- **Braucht:** Mycelium fährt `ersstv5_compiler --ci-mode` + Release-Asset; der
  Operator-Zuschnitt reist als Punkt an Future (siehe `## An future`); dann die Probe
  (OMNI `omni_hro_imf_bz_gsm_nt` × `ersstv5_nino34_ssta`) nach `nobel_probe_corona.rs`
  binden.

### Rätsel Ⅰ — Jeans-Engine, Zensus gesetzt
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Mountain liefert den erweiterten `dr3_stars`-Record (σ_ϖ/σ_pm) samt neuer
  Asset-Version (CDN).
- **Lage:** (gemessen 2026-09-29 River 65 via explore) `StarRec` (`src/archivar/spatial.rs:53`)
  trägt 44 B (`STAR_RECORD_BYTES`) **ohne σ_ϖ/σ_pm** (`:341-376`); `rv_m_s` ist vorhanden
  (`:353,374`). Der Ratsbeschluss (`docs/handover/archiv/handover-2026-09-29-river-folge63.md:89-102`):
  ein Bin, drei Modi — Reihenfolge Zensus → **Compiler-Fehler-Atom** → Schätzer;
  **Fehlerkorrektur Pflicht** (`σ_z²(obs) ≤ ⟨σ²_err⟩ → absent`, nie 0,0); symmetrisierte
  |z|-Bins, ≥ 32 Sternen/Bin. Der Schätzer-Sockel ist **nicht** baubar, ohne das künftige
  Record-Layout (σ-Spalten) zu raten — eine Spekulation; darum keine Bausonde in diesem Atom.
  Der Zensus-Sweep selbst ist gesetzt (folge64: Median 1 = Distanz-Gewichtungs-Artefakt;
  plx>5-Subprobe belegt Zellen ≥ 32).
- **Blockade:** σ_ϖ/σ_pm fehlen im Record (Mountain Compiler-Fehler-Atom,
  `docs/handover/handover-2026-09-29-mountain-folge203.md:111-116`); ohne Fehlerkorrektur
  ist jede σ_z-Schätzung eine Rauschmessung.
- **Braucht:** Mountain erweitert den Record + Mycelium manifestiert die neue
  Asset-Version; dann `--estimator` auf der plx>5-Subprobe mit symmetrisierten |z|-Bins
  (galaktische PM-Rotation fehlt, `icrs_to_galactic` steht in `src/mathematikerin/healpix.rs:103`).

### flyby-path2-recon
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ESOC publiziert einen SKD `v474+` mit `juice_cog_000115_…` oder einer
  dedizierten `*recon*`-SPK in `spiftp.esac.esa.int/data/SPICE/JUICE/kernels/spk/`.
- **Lage:** (gemessen 2026-09-29 River 62, general) **ABSENT**: Enumeration endet bei
  `juice_cog_000114_230416_261003_v01.bsp`; Gate `data/flyby2/gate-juice-2026-09-28.json`
  recon/sigma/verdict `pending`, `riss: false`.
- **Blockade:** Publikation fehlt.
- **Braucht:** nach Publikation `flyby_ephemeris_gate --recon
  data/ssd.jpl.nasa.gov/ephemeris_juice_recon.bin --sigma-recon <km>`.

### Weberin-Lücke — Prosa-Träger `membran-ladearchitektur`
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Mountain liefert die `format vlde`-Quelle (dann derived-field-Wiring).
- **Lage:** (gemessen 2026-09-29 River 64) übernommen von mountain-folge202;
  `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md` trägt den real offenen
  Marker `:234-235` (Vlies-Dichte/TE/Verdict als Derived-Field-Schicht; Reader
  `src/archivar/vlies.rs` steht, `format vlde` fehlt in `phi/sources.φ`); die
  Enclosure-/Jump-/ω-Loop-Punkte der Survey sind gebaut (Nachtrag `:203-233`). River
  ist Prosa-Träger.
- **Blockade:** die `format vlde`-Quelle (Mountain) fehlt.
- **Braucht:** Mountain `format vlde`; dann das derived-field-Wiring im ω()-Loop.

### Trägerlose Prosa — Orphan-Zensus
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** keine.
- **Lage:** (gemessen 2026-09-29 River 65 via `register_lookup --orphan-docs`) = **2**:
  `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` (3 Marker; **Mountain-Natur**:
  Ephemeriden-Placeholder `:56` + Operator-Layout-Wort `:74/:139`) ·
  `docs/surveys/survey-messpunkt-verteilung.md` (6 Marker; **River-Natur**, alle
  Fehlalarm/geschlossen — §9/§10 beantworten die Detailfragen, Kandidaten 1–8 mit Verdikt).
  Die in folge64 genannten Docs (`kybernetische-astrophysik`/`legacy-konzepte`/`fortschritt`)
  orphanen nicht mehr. Addressed mountain203/sensory205 verlangten River-Trägerschaft für
  `kybernetische-astrophysik.md` + `legacy-konzepte.md` — beide River-Natur, kein Marker in
  der laufenden Messung.
- **Blockade:** keine für River; `daten-holdings-inventur` ist Mountain (Datenbestand) + Future
  (Layout-Wort).
- **Braucht:** River trägt als Namenträger `survey-messpunkt-verteilung.md`,
  `docs/concepts/kybernetische-astrophysik.md` und
  `docs/surveys/survey-2026-09-17-omegaflow-legacy-konzepte.md` (unten); `daten-holdings-inventur`
  reist an Mountain + Future (`## An mountain` / `## An future`). Der Scanner-Fehlalarm
  `open_marker_matches` (`register_lookup.rs:112`, `wartet ⊂ erwartet`) liegt bei Mountain.

## An mountain
Origin: river folge65.

**Gemessene Fälligkeit (Trigger gefeuert 2026-09-29 River 65):** die archivar-Regression
hält `ci-check` rot — damit bleibt auch der `WGSL-Naht`-Trigger (`ci-check` grün) ungefeuert;
sie blockiert zwei River-Punkte. Die drei übrigen Einträge sind Lieferungen
(`dr3_stars`-Record, `daten-holdings`-Träger, Scanner-Lint).

- **archivar-Regression (neu, `ci-check 36506672136` auf `5d6c9c685`):** 11 lib-Tests rot —
  `archivar::pds3_img` (decode/pack/band), `archivar::pds3_table` (krfm), `archivar::pds3_binary`
  (missing cells), `archivar::pds4` (delimited label + real rows), `archivar::pds4_binary`
  (nan/inf), `archivar::gras_2c`, `archivar::hips::average_filter_decodes`,
  `archivar::extract::fixed_width_series_tests::gras_2c_series_reads_records`. Auffällig:
  `data_type` kommt als `ASCII_Date_Time_` statt `ASCII_Date_Time_YMD` im geparsten Record —
  wahrscheinlich ein gemeinsamer Parser-/Fixture-Bruch. Hält `ci-check` rot.
  Braucht: die Regression heilen.
- **`daten-holdings-inventur`** (`docs/surveys/survey-2026-09-03-daten-holdings-inventur.md`):
  Mountain-Natur (Datenbestand) mit offenem Marker `:56` (976-B-Placeholder, `pending`).
  Braucht eine Mountain-Träger-Zeile; das Layout-Wort `:74/:139` reist an Future.
- **`dr3_stars` Compiler-Fehler-Atom:** der Record (44 B) trägt keine σ_ϖ/σ_pm; Rätsel Ⅰ
  hängt daran. Braucht: erweiterter Record + neue Asset-Version (CDN).
- **Scanner-Fehlalarm `wartet ⊂ erwartet`:** `register_lookup.rs:112` (`open_marker_matches`,
  Substring) erzeugt Falsch-Orphans. Braucht: Wortgrenzenlogik wie bei `blocked`.

## An future
Origin: river folge65.

**Gemessene Fälligkeit:** zwei Operator-Worte offen — der ENSO-Blatt-/Paar-Zuschnitt und
das Migrations-Ziel-Layout `daten-holdings-inventur`. Beide sind vorbereitet bis zur Kante
(Quellenzeile bzw. Marker gemessen); nur das Wort fehlt.

- **ENSO-Blatt-/Paar-Zuschnitt:** Lage — die `ersstv5_nino34`-Quellenzeile steht
  (`phi/sources.φ:11111-11117`), die Manifestation liegt bei Mycelium; offen ist allein,
  welches Paar (OMNI `omni_hro_imf_bz_gsm_nt` × `ersstv5_nino34_ssta`) in Blatt/Probe
  zugeschnitten wird. Frage — welchen Zuschnitt nimmt die ENSO-TE-Probe? Bei Ja: Probe wird
  nach `nobel_probe_corona.rs` gebunden. Bei Nein: Zuschnitt bleibt offen.
- **Migrations-Ziel-Layout `daten-holdings-inventur`:** Lage —
  `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md:74/:139` wartet auf das
  Operator-Wort zum Ziel-Layout. Frage — welches Layout? Bei Ja: Umsetzung startet.
  Bei Nein: bleibt `pending`.

## An mycelium
Origin: river folge65.

**Gemessene Fälligkeit:** die ENSO-Manifestation ist der einzige fremde Schritt, der die
`ENSO TE-Probe` löst (Quellenzeile steht, Compiler existiert).

- **ENSO-Manifestation:** die `ersstv5_nino34`-Quellenzeile steht; `ersstv5_compiler --ci-mode`
  fahren und das Release-Asset `coastwatch.pfeg.noaa.gov/ersstv5_nino34.bin` manifestieren.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). Pfad-begrenzte
Commit-Pfade dieser Session:

`docs/handover/handover-2026-09-29-river-folge65.md` ·
`docs/handover/archiv/handover-2026-09-29-river-folge64.md`.
