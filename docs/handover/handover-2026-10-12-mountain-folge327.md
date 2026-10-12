<!--
  title: Handover — Mountain-Folge 327 (2026-10-12)
  session: Mountain-Linie in einem Pass — JADES tar_fits Member-Key-Kollision geheilt (Verzeichnis bleibt erhalten), tar_fits in format_readable + Port-Skip-Liste registriert, api.sensor.community als parametrischer map-Quelle gemessen (kein Arm nötig), jma-Hunk als point-event/erbq-Familie committet
  class: handover
  date: 2026-10-12
  sha256: bb02f3227f2c18689791b96c3685fa0aa0106f90e3d93669bded958d9c56e844
  status: live
-->
# Handover — Mountain-Folge 327 (2026-10-12)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der Stehende
Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`; gemessen am HEAD
`934c131f3`, seither committete Mycelium-324 (`34c131f3`→`e55dd1897`…`934c131f3`) und
Mountain-326 (`d55d6c1b8` `6ba1b67e6`)). Diese Session konsumierte
`handover-2026-10-12-mountain-folge326.md` (→ `docs/handover/archiv/`). **Korrektur (gemessen 2026-10-12, nicht der Kopfzeile geglaubt):** der `## An mountain`-Block aus future-241 war von folge326 nur **teilweise** gefaltet. Registriert/geechoed sind APOLLO (`phi:32388`), die mariner-rst-Rohbänder (ASU-MVM `phi:31519`, SPDF-M4/6/7 `phi:31526/31534`), DSC 0119 (`phi:31590`), das USGS-Kombi-Mosaik (`phi:31582`), JADES (`phi:32391`), BK18, FMI-NUR, NIST-Route. **Nicht gefaltet und ohne Live-Träger:** die **8-Ziele-Einkaufsliste** (BC430 `3558850` · Pioneer 10/11 `13309156`/`13309040` · Venus-Radar `18176179` + PDS-Geosciences · PDS-PPI `PVO-V-ORSE-1-ODR-OPENLOOP--V1.0` · JPL `obs_data.html`) und die zwei public Indizes — jetzt unten gefaltet. `--fired`/`--stale --persist 3` = leer.
**Jeder offene Punkt nennt seine Datei(en)/Pfade.**

## Burn: open 0.00 · close 0.13 (Linie) · cap 0.50 · Grund: kurze Linien-Session (Mountain 327) + Taucher-Staffel (9 flash-Dispatches: 3×grind-flash, 5×general/council, 1×vision ~ 0.78 gesamt), deepseek-flash, kein pro/max (session_burn: Mountain-Linie `$0.1307`).

## Offen (aufgeschlüsselt)

### Auftrag „Eigene Ephemeriden + Kulturerbe" — Phase 0/1 (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** das Operator-Wort zum Start
- **Lage:** (gemessen 2026-10-12) `docs/auftrag/auftrag-eigene-ephemeriden-und-kulturerbe.md`
  (sha `4a9dac1d…`). Rat + UI konvergent: EIN Archivar, zwei Register
  (`obs`/`heritage`/`observable`/`asset`); `house_gate` → Konsistenz-Hülle, ehrliches Gate =
  **Hold-out**; minimaler Operator mit `pending` (nie 0.0); **Riss 1** M10-Bild = Erbe;
  **Riss 2** Anker reisen mit; `raw_level` 0/1/2; **Ordnung** Rahmenschicht → LLR (relativ) →
  ODF als Prüfling → Radar → ΔDOR/VLBI → Tiefraum; **Erbe→Brennstoff** beim Stand der inneren
  Region. Stimmen: `state/stimmen/2026-10-12_ui_eigene-ephemeriden-verdikt.md`.
- **Blockade:** CDDIS/Earthdata (`blocked account`, `state/zustand/wartend.φ`); LLR-Beschaffung
  = Politik (Operator/Future).
- **Braucht:** Phase 0 = `archive_search --all` je Beobachtungsklasse → Text; Phase 1 =
  ITRF2020 (`https://itrf.ign.fr`) + IERS-EOP + LLR (ILRS/CDDIS, historisch POLAC/Z&N) +
  Operator `canObserve`.

### M10-EDR Winkeldurchmesser — Riss: EDR-Korpus ↔ M10-Ephemeride disjunkt (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** eine M10-Ephemeride, die den Venus-Encounter trifft
- **Lage:** (gemessen via `tools/measure/src/bin/m10_expected_diameter.rs` + future-241) EDR-Asset
  `mariner_m10_edr.bin` (`phi/sources.φ:31385-31390`, `mvm_0001.tgz`, `ser.im-ldi.com`) = 651
  Frames, alle **Venus 1974-02-05/06**; `data/ssd.jpl.nasa.gov/ephemeris_mariner10.bin` deckt nur
  **Merkur-1 (JD 2442128–2442141)**. NAIF `…/naif/M10/kernels/spk/` nur `M10_archive_1.bsp`.
  **future-241-Update:** die M10-Pose je Frame liegt in
  `https://ser.im-ldi.com/M10/M10_ISIS/camera_angles.txt` (194 640 B, sha `0a65df59…`; 811 Bilder
  1974-03-29…1975-03-17 = die drei Merkur-Encounters) → der **M1-Kamera/Encounter-Blindfleck ist
  adressierbar**, die breite Ephemeride für die Registrierung wahrscheinlich entbehrlich.
- **Blockade:** keine breite M10-Ephemeride (falls doch nötig); der `camera_angles`-Parser-Arm fehlt.
- **Braucht:** `camera_angles.txt`-Parser-Arm (siehe unten); sonst JPL Horizons `-76`
  (`tools/harvest/src/bin/horizons_compiler.rs`) oder Merkur-Tapes `mvm_0002+` (`ser.im-ldi.com/M10/MVM/`).

### M10-Kontrollnetz (ISIS) + M10↔MDIS — Quelle gemessen (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Register-/Parser-Pass
- **Lage:** (gemessen 2026-10-12) **ISIS-Quelle verifiziert:** `astrogeology.usgs.gov/software/ISIS`
  = **HTTP 404** (tot); live: `isis.astrogeology.usgs.gov` (USGS Astrogeology, Mariner 10 unterstützt);
  Quell-Repo `github.com/DOI-USGS/ISIS3`, `LICENSE.md` = **CC0-1.0 / US-publikumsfrei** (geschrieben
  1 761 B, sha `7162bb20…`). **Nicht** Zenodo/Unlicense. `camera_angles.txt` (194 640 B, sha
  `0a65df59…`) deklariert in der Datei nur die Labels `JULIAN_DATE&FDS`/`SXSYSZ`/`C1C2C3` — **keine
  Einheit**; `C1C2C3` = die drei Kamera-Orientierungswinkel (Robinson et al. 1999: 811 Bilder,
  Brennweiten A 1493,6 / B 1500,1 mm); `SX SY SZ` = Sondenvektor (numerisch gestützt: |S| ≈ 12 139 km
  bei Encounter III). Zukunft: `state/research/m10-mdis-sonke-robinson/MANIFEST.md` (Sonke & Robinson
  2026, PSJ 7:35, `10.3847/PSJ/ae32f5`, CC BY 4.0) + `state/mountain/mariner4_dsc0119_format.md`.
- **Blockade:** `url`/`origin` (Mycelium) für die ASU-/ISIS-Quellen; `SXSY SZ`-Einheit/Frame nicht
  in der Datei selbst belegt.
- **Braucht:** Mycelium `url`-Zeile für `ser.im-ldi.com/M10/M10_ISIS/` (US-PD) → `terms`/`format`
  Mountain → `camera_angles`-Parser-Arm (siehe unten); die Sonke/Robinson-Tabellen 1–8 als
  `url`-Zeilen (Mycelium) — letzte Meile (voller cnet-Satz) ist paper-only: mit ISIS nachrechnen
  (`camera_angles.txt` + MESSENGER-DEM-v2 + MDIS-Basemaps + `autoseed/pointreg`) **oder**
  Sonke/Robinson anschreiben (**Operator-Hand**, kein Maschinen-Send).

### M10 `camera_angles.txt` — Parser-Arm (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** die ASU-`url`-Zeile (Mycelium) — registriert `phi:31570`
- **Lage:** (gemessen 2026-10-12/327) 2 433 Zeilen = 811 Records × 3 Zeilen:
  `<JD (D-float) + FDS>` / `<SX> <SY> <SZ>` / `<C1> <C2> <C3>`, je mit trailing Label; keine Einheit im File.
  **Gebaut (Taucher):** Modul `src/archivar/camera_angles.rs` (`pub fn parse_camera_angles(text) -> Vec<CameraAngleFrame>` mit jd/fds/sx/sy/sz/c1/c2/c3; D-Exponent, Label-Prüfung, malformed → skip; Tests), `pub mod camera_angles;` in `src/archivar/mod.rs`; **Dispatch:** `format camera_angles` in `src/archivar/extract.rs` (`camera_angles_to_json`/`camera_angles_frames_to_json`, Tests `camera_angles_frames_become_json_rows`/`camera_angles_yield_no_rows_without_a_frame`), `format camera_angles` im Block `phi:31570`; `cargo check` 0/0.
- **Blockade:** keine (Arm + Dispatch + `format` gesetzt).
- **Braucht:** `terms` im Block klären (`phi:31570` `terms unknown`, Mycelium); die **FDS→Bild-Dateiname**-Zuordnung (der FDS ist die Bilddatei, nicht die Zeit — JD/FDS nicht ko-monoton) beim ersten Lauf prüfen.

### `mariner_rst` — SP-Begleitbänder, eigener Parser-Arm (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Parser-Pass
- **Lage:** (gemessen 2026-10-12/327) **Riss geklärt:** `DD005407`/`DD005409` sind **keine** Summary-Tapes,
  sondern die **Single-Precision-Begleitbänder** derselben M6/M7-Datensätze (DSC 0073, sha
  `02b97558…`, 540 510 B, JP: „one … single precision and the second … double precision").
  **Korrektur (Taucher, gemessen):** der ODG-Data-File-Kopf ist **drei** Records — 10-Wort-Label +
  201-Wort-Station-Summary + **97-Wort-Summary-Fortsetzung** = **308** Wörter (nicht 211; die alte
  „10+201+Datenrecords"-Annahme verschöbe jeden Datenrecord um 97). Quellen:
  `state/mountain/mariner4_dsc0119_format.md:13-20`, `state/research/…/mariner/RESULT.md:32-34`,
  Rohwalk DD005407/5409 (L = 60/1206/582 B = 10/201/97 Wörter).
  **Gebaut (Taucher):** SP/ODG-Reader in `tools/harvest/src/bin/mariner_rst_compiler.rs`
  (`sp_odg_data_records`, `sp_odg_record_observables`, `--sp`-Zweig; malformed/mehrdeutige Datentypen → skip;
  Test `sp_odg_reader_decodes_one_record_and_skips_malformed`; `cargo check -p omegaflow-harvest --bin mariner_rst_compiler` 0/0);
  der DP-Pfad bleibt unangetastet.
- **Blockade:** das **M6/M7-SP-spezifische** Feldlayout ist im Baum nicht transkribiert (nur DSC-0073-sha);
  die Frequenz-Skalierung (roh ~1.08e12, /2⁹ ≈ 2114 MHz) ist auf der DSC-0073-Seite nicht dokumentiert.
- **Braucht:** DSC-0073-SPFP-Layout pinnen (TM GTB900-1 / Table 3) + Frequenz-Skalierung gegen DSC 0119/Odf-Spec
  oder `pending`; SP-Band-`url` via Mycelium (`…/PSCM-00017_DD005407_…tar`, verdict HTTP 206).

### DSC 0119 / M4-Feldtabelle — Parser-Arm (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der `mariner_rst`-Parser-Pass
- **Lage:** (gemessen 2026-10-12/327) `state/mountain/mariner4_dsc0119_format.md` (ODG/ODP-Record,
  6-bit-Byte-Walk, Zeit dekodiert); der DSC 0119 trägt `SECTION II` (ODG DATEN-File: Master-Label
  9 Wörter + Station-Summary 201 Wörter + Datentypen 1–15) + `APPENDIX I` (ODP Logical Record).
  **Registriert:** `phi:31588-31590` (`url`/`origin`, sha `ec3c41eb…`). **Vision-Pass (Taucher, 2026-10-12, korrigiert):**
  das PDF hat **21 Seiten** (nicht 16 — der erste Read war unvollständig); die Appendix-Footer sind
  **`II-1`/`II-2`/`II-3`** (S. 14/15/16), nicht „II-B-1". **Kein** Abschnitt heißt wörtlich „Table 3" —
  Table 3 ist nur auf **S. 13** referenziert (die Seite ist sonst leer); das Layout selbst ist **S. 14 (II-1)
  „ODP File Logical Record Format"**: Wort 0 = D (Wortzahl), 1-2 = DP-Zeit s past 1950.0, 3 = Frequenz-Inkrement,
  4 = Pass-Nr., 5 = Code-Wort, 6 = Observable Nr. 1, 7 = Weight-Code-Wort; Code-Wort-Bits selektieren die Datentypen
  (1 Range · 2 Range-Rate · 3 EL · 4 AZ · 5 DEC · 6 HA/RA · 7 C1 · 8 C2 · 9 C3 · 10 TR · 11 RU · 12 PRU · 13-15 unused);
  die multi-Datentyp-Recordlänge ist **variabel** (Fig. 2, S. 17–18). Weitere Tabellen: Stationstabelle External/Internal
  (S. 10), ODG-Datentyp-Liste 1–15 (S. 12), Bit/Datatype 5–11 + Weight-Code-Wort (S. 15/II-2), ODP-Summaries (S. 16/II-3).
  Die Transkription steht in `state/mountain/mariner4_dsc0119_format.md` (nachgemessen).
- **Blockade:** keine (Feldtabelle + Record-Layout gelesen).
- **Braucht:** den variablen Multi-Datentyp-Record (Fig. 2) in den `mariner_rst`-Arm (SP-ODG) aufnehmen —
  dann sind auch die 848/649 SP-Records dekodierbar.

### JADES `tar_fits` — Member-Key-Kollision geheilt, Image-Ingest gebaut (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-12/327 via `tar -tf` des Zenodo-Tars 2 898 432 B, `archive_search --verdict` HTTP 206)
  Das JADES-Tar (`phi:32391` `format tar_fits`) trägt **4 Objektverzeichnisse × 9 Bänder** =
  36 Members: `JADES-GS-z{10,11,12,13}-0/data_f{090w,115w,150w,200w,277w,335m,356w,410m,444w}.fits`.
  Die alte `tar_fits_member_key` verwarf das Verzeichnis (`rsplit('/')`) → die vier Objekte kollabierten
  auf **9** statt 36 Members. **Geheilt:** der Key behält den Pfad, Test `tar_fits_keys_members_of_distinct_directories_distinctly`.
  **Image-Ingest (Taucher):** `fits_to_json` fällt ohne BINTABLE auf `fits_image_bands_to_json` zurück,
  Test `fits_to_json_summarises_an_image_only_fits` → der `format fits`-Pfad (UNCOVER `phi:32408`) liefert jetzt
  eine Band-Summary. `tar_fits` in `fetch.rs::format_readable` + Port-Skip-Liste aufgenommen (`cargo check` 0/0).
- **Blockade:** keine.
- **Braucht:** der JADES-Block braucht **36 `field`-Zeilen** (Member-Key = `<objekt>/data_<band>`) mit force/unit
  → Kanäle; der Extraction-JSON-Pfad (`<member>` → `[0].mean`) muss dafür gemessen werden.

### THEMIS-ASI Stationstabelle — an River übergeben (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-12 via river-196/`sread`) Operator-Wort 2026-10-12/327: „river analysiert selbst schon die antennenhöhe" — die Antennenhöhen-Frage (HRDEM Geländehöhe ≠ Antennenhöhe ~2–3 m; PGM-Koordinate = Stadt, nicht AUGO-I) liegt bei **River** (`state/river/themis/calibration-2026-10-12.md` §Höhen-Riss/§LiDAR; river-196). Kein Mountain-Akt.
- **Blockade:** —
- **Braucht:** River-Analyse; Mountain ohne Schritt.

### `intermagnet_dbdt` — Resolver-Regel (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** Freigabe von `tools/measure/src/bin/field_te_query.rs`
- **Lage:** (gemessen 2026-10-12/327 via `ci_manage log`) Resolver gebaut (Mycelium `3467ec4f9`:
  per-`station_code`, feinster `median_dt` als Zeuge, gröberer als getragener Riss); Verdikt
  `state/stimmen/2026-10-12_river-intermagnet-identitaet-rat.md`. Lauf `38215224525` ist **grün**
  (14/14 Jobs) → der Trigger ist gefeuert; die Zelle `newell_dphi_dt_omni->intermagnet_dbdt` ist frei,
  die Datei hält keine parallele Session mehr (nicht in `git status`).
- **Blockade:** keine.
- **Braucht:** die Zelle `newell_dphi_dt_omni->intermagnet_dbdt` **neu messen**
  (`cargo run -p omegaflow-measure --bin field_te_query`), den Wert in das Verdikt schreiben.

### Quellen-Terms — not-admit/declined-Disposition (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-12/327) **Gebaut (Rat-Form):** `decline <reason-token> <source-url>` mit geschlossenem Set `{unphysical|model|registry|commercial|superseded}` — `src/archivar/parse.rs` (`DeclineReason`, `DeclinedSource`, `pub fn parse_declined`; ein unbekannter Token → `report_anomaly`; Tests `decline_lines_parse_into_the_closed_reason_set`, `a_decline_token_outside_the_set_is_refused`, `a_decline_line_never_becomes_a_source`); `cargo check` 0/0. Die `decline`-Zeile erzeugt **nie** einen `SourceConfig` (Catch-all bleibt leer). Anker: `oe417` (`phi:11983-11991`, freier String).
- **Blockade:** die Gate-Fixture `phi-sources-decline-token` und der `register_lookup`-SOURCES-Scan/`collect_disposed_urls` liegen in Dateien, die eine parallele Session hält (`src/gate/commit_gate.rs`, `commit_gate_vocab.json`, `tools/register/src/bin/register_lookup.rs` — uncommittet).
- **Braucht:** die Fixture + den `register_lookup`-Scan im selben Atom nachziehen (sobald die Dateien frei sind); danach die getilgten Verdikte als `decline <klasse> <url>`-Zeilen re-homen.

### Re-Admission der lizenz-blockierten Kandidaten (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Verdikt-Pass
- **Lage:** (gemessen 2026-10-11, `docs/surveys/survey-2026-10-09-redistribution-alternativen.md`
  §Dritter Lauf) ALMA→ESO, GRDC→GRDC-Caravan/USGS NWIS, RIPE RIS→RouteViews,
  **IMCCE INPOP/NOE-4 → erledigt: Open Licence 2.0 (etalab-2.0), Blöcke gesetzt** (siehe `## An mycelium`),
- **Blockade:** Admission-Verdikte je Kandidat fehlen; Risse RouteViews BY-vs-BY-SA, ESO-CC-BY nur
  für ESO-ALMA-Records.
- **Braucht:** Admission je Kandidat (Mountain-Verdikt); danach Mycelium-CDN.

### Rätsel-Datenlücken (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** die `url`-Zeilen (Mycelium)
- **Lage:** (gemessen 2026-10-12 via `sread`) `phi/sources.φ:32408` (UNCOVER `14281664`, `format
  fits` + `CC-BY-4.0`) · `:32381` (JADES `7460116`) · BK18 (erledigt) · GIC-Mäntsälä (FMI NUR
  `space.fmi.fi`).
- **Blockade:** `url`/`origin` (Mycelium).
- **Braucht:** je Lücke die `url`-Zeile → `terms`/`format` → Parser.

### NIST NCNR NSE · Teil C A5 · opentopography · SWPC/RTSW · Sechs Klima-Quellen · eigene Ephemeride · Frame/Epoche-Sammel (eigen/wartend/blockiert)
- **Status:** eigen/wartend/blockiert | **Bindung:** eigen/termin:2026-11-01
- **Trigger:** je der benannte (CDN-/Manifest-Lauf, `url`-Zeile, Freigabe `src/mathematikerin`,
  Termin 2026-11-01)
- **Lage:** (gemessen 2026-10-11/12) NIST NCNR NSE (`https://ncnr.nist.gov/pub/ncnrdata/nse/`, US-PD)
  braucht `url` (Mycelium `4b5fe3fdb` baut den HDF5-NeXus-Arm); Teil C A5 `GO-SS-RSS-1-TDF-V1.0` (PDS
  PPI) braucht `url`; opentopography_coverage braucht Manifest; SWPC/RTSW braucht Archivreihe-Verdikt
  +`ttl`; Sechs Klima-Quellen (`series-manifest`) brauchen den ersten Lauf; eigene Ephemeride blockiert
  an `src/mathematikerin`; Frame/Epoche CRS-VLR ungemessen.
- **Blockade:** `url`/Manifest (Mycelium); die Freigabe von `src/mathematikerin`.
- **Braucht:** je `url`-Zeile (Mycelium) → `terms`/`format` (Mountain); Läufe lesen
  (`ci_manage log <id>`); iEEG **nicht erneut vorlegen** (Operator-Wort 2026-10-06).

## Taucher-Messungen 2026-10-12/327 (offen)

### Auftrag Phase 0 — Landschaft je Beobachtungsklasse (eigen)
- **Status:** eigen | **Bindung:** eigen | **Trigger:** Operator-Wort Phase 1
- **Lage:** (gemessen 2026-10-12/327, `general`-Taucher) fünf Konsolidate unter `state/research/auftrag-ephemeriden-phase0/`: `a-llr.txt` (15 Kandidaten) · `b-odf.txt` (24) · `c-planetary-radar.txt` (15) · `d-delta-dor.txt` (17) · `e-ephemerides.txt` (15). Terms durchweg `unknown` (nicht gemessen in Phase 0).
- **Blockade:** keine.
- **Braucht:** Phase 1 = Terms je Kandidat messen + ITRF2020 (`https://itrf.ign.fr`) + IERS-EOP + LLR (ILRS/CDDIS) + Operator `canObserve`.

### Re-Admission — Admission-Verdikte gemessen (eigen)
- **Status:** eigen | **Bindung:** eigen | **Trigger:** der Register-Schreib-Pass
- **Blockade:** die NC-Admission ist eine Operator/Rat-Frage.
- **Braucht:** je Kandidat die Admission-Zeile in `phi/sources.φ` (Mountain) + CDN (Mycelium); die NC-Kandidaten dem Operator vorlegen.

### Rätsel/NIST — Directive-Kandidaten gemessen (eigen)
- **Status:** eigen | **Bindung:** eigen | **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-12/327, `general`-Taucher) JADES `7460116` (2 898 432 B, cc-by-4.0) · UNCOVER `14281664` (65 905 920 B, cc-by-4.0) · FMI-NUR `?month=YYYYMM&stations=NUR` (200, 921 600 B, CC BY 4.0, Format undetermined) · NIST NCNR NSE `https://ncnr.nist.gov/pub/ncnrdata/nse/` (200, US-PD) · Teil C A5 `GO-SS-RSS-1-TDF-V1.0`. JADES/UNCOVER sind als Zenodo-Records gemessen; die `url`-Zeilen existieren teils schon (`phi:32391`/`:32408`).
- **Blockade:** keine.
- **Braucht:** die `terms`/`format`-Zeilen in den Blöcken setzen; FMI-NUR-`format` klären (GET-Body opaque).

### M10-Encounter + Sonke/Robinson (eigen)
- **Status:** eigen | **Bindung:** eigen | **Trigger:** der `camera_angles`-Dispatch
- **Lage:** (gemessen 2026-10-12/327, `general`-Taucher) M1-Fenster 1974-03-28..30 = **410 Records**, JD 2442135.305…2442136.500, FDS `00000050`…`00027475`; **Riss:** innerhalb M1 sind JD und FDS **nicht ko-monoton** (FDS steigt 50→27475, JD fällt 2442136.378→2442136.350) — der FDS ist die Bild-Datei-Ordnung, nicht die Zeit. Sonke & Robinson 2026 (`10.3847/PSJ/ae32f5`) ist **OA gold, CC-BY** (`--unpaywall`; Crossref/OpenAlex leer). `https://ser.im-ldi.com/M10/M10_ISIS/` = **403** (kein Listing); `/M10/TXT/camera_angles.txt` = **200, 194 640 B**.
- **Blockade:** das Camera-Pose-Wiring.
- **Braucht:** FDS→Bild-Dateiname-Mapping im `camera_angles`-Dispatch dokumentieren; Sonke/Robinson-Annotation der ASU-Datei als `url` (Mycelium).

### Quellen-Terms — Rat-Verdikt zur not-admit-Form (eigen)
- **Status:** eigen | **Bindung:** eigen | **Trigger:** Operator-Wort
- **Lage:** (gemessen 2026-10-12/327, `council`) Konsens: eine `declined`-Disposition ist eine **Direktive** mit **geschlossenem** Grund-Token (nicht der 183-Token-Freitext des getilgten `declined_sources.φ`), trägt die gemessene URL als Schlüssel, erzeugt **nie** einen `SourceConfig`/ttl und bleibt in `phi/sources.φ`. Der Parser-Catch-all (`parse.rs:2307`) verschluckt eine `decline`-Zeile heute **still** → die Direktive muss ein benannter Arm werden; `register_lookup` kennt `declined` bereits als `Released`.
- **Blockade:** der Token-Satz (welche Gründe geschlossen gelten).
- **Braucht:** Operator-Wort zum Grund-Token-Satz + Parser-Arm + Gate-Fixture im selben Atom.

## An mycelium

Origin: mountain-folge327 (2026-10-12). **Echo:** `handover-2026-10-12-mycelium-folge326.md`
hat den vorigen Block gefaltet (registriert/geechoed: ASU-MVM `phi:31519`, SPDF-M4/6/7
`phi:31526/31534`, DSC 0119 `phi:31590`, USGS-Mosaik `phi:31582`, JADES `phi:32391`, BK18, FMI-NUR,
NIST) — nur die Sonke/Robinson-Tabellen bleiben dort offen.

- **8-Ziele-Einkaufsliste (future-241) — nicht gefaltet, je `url`/`origin`/`compiler`-Zeile.** `xfer:mountain327-1` Die vier
  direkt downloadbaren sind **cc-by-4.0** und ohne Registerzeile (nur APOLLO `7818557` steht, `phi:32388`):
  - **BC430** Zenodo `3558850` — `BC430.tar.gz` 59 541 013 B, 300 Asteroiden 1800–2200.
  - **Pioneer 10/11 Doppler** Zenodo `13309156` (`SC_23.asc` 678 784 528 B) / `13309040` (`SC_24.asc` 721 591 221 B).
  - **Venus-Radar** Zenodo `18176179` — `…75m_v1.tif` 80 909 342 816 B · `Processed.zip` 409 371 501 B.
  - **PDS-Geosciences Venus-Radar** `https://pds-geosciences.wustl.edu/missions/venus_radar/index.htm`
    (PDS: kostenlos + TSPA, **keine** explizite Lizenz; Zitation Campbell & Campbell 2022, `10.3847/PSJ/ac4f43`).
  - **PDS-PPI `PVO-V-ORSE-1-ODR-OPENLOOP--V1.0`** `https://pds-ppi.igpp.ucla.edu/data/PVO-V-ORSE-1-ODR-OPENLOOP--V1.0/`.
  - **JPL `ssd.jpl.nasa.gov/planets/obs_data.html`** (Spacecraft-VLBI) — keine Nutzungsaussage, Lizenz `unverified`.
  Gemessene Quelle: `docs/plans/ref-ephemeriden-quellen-index.md` (Mountain).
- **Orphan-Doc `docs/surveys/survey-2026-10-08-fmhy-research-landscape.md`** (2 offene Marker; see-also
  `docs/concepts/ui-seats.md`, `docs/SOURCE_PORT.md`) braucht einen Namenträger bei dir. `xfer:mountain327-2`
- **Vier neue SPDF-DSC-/NASA-SP-Dokumente — je `url`/`origin`/`compiler`-Zeile (US-PD).** `xfer:mountain327-6`
  Durable lokale Kopie liegt im **privaten Recherche-Korpus** `state/research/` (nicht `data/` — dort liegen die Datensätze, z. B. `data/spdf.gsfc.nasa.gov/*.bin`; Operator-Wort 2026-10-12 „gehören pdfs in data?" → nein):
  - `state/research/mariner-dsc/mariner4_cosmic-ray-telescp_64-077a-04abcde_DSC_0045.pdf` — 3 487 691 B, sha `37745a46…`.
  - `state/research/mariner-dsc/mariner4_helium-magnetometer_64-077a-02a_DSC_0176.pdf` — 4 517 956 B, sha `51d288e1…`.
  - `state/research/mariner-dsc/mariner4_mag_64-077a-02b_DSC_0074.pdf` — 443 225 B, sha `30ba3e6b…`.
  - `state/research/mariner-venus-1962/SP-59.pdf` „Mariner-Venus 1962: Final Project Report" (NASA SP-59, 1965) —
    origin `https://www.nasa.gov/wp-content/uploads/static/history/SP-59.pdf` (HTTP 206), 6 093 576 B,
    sha `a46e1712…`; NTRS trägt **keine** SP-59-Citation (HathiTrust `006260826`, PD).
  Der SPDF-Origin-Präfix ist `https://spdf.gsfc.nasa.gov/pub/documents/old/documentation_from_nssdc/renamed_dataset_catalogs/`.
  Die durable Heimat ist das registrierte CDN-Asset (raw_mirror); die drei Alt-PDFs in `data/` (`doe417_instr.pdf`, `pds-ppi…/124A.pdf`, `cjss…/210406045.pdf`) stehen in **keiner** phi-Zeile → Register-Schuld.

## An river

Origin: mountain-folge327 (2026-10-12). Drei `--orphan-docs`-Dokumente sind nicht Mountain und brauchen
einen Namenträger bei dir:
- `docs/blatt/blatt-gic-breitenband-familien.md` (4 Marker; see-also `docs/paper/gic-causal-driver.md`). `xfer:mountain327-3`
- `docs/surveys/survey-2026-10-05-stoerungs-experiment-fehlende-faeden.md` (4; see-also river-folge93). `xfer:mountain327-4`
- `docs/surveys/survey-2026-10-08-sonnen-render-archaeologie.md` (2; see-also river-folge139). `xfer:mountain327-5`

## Prosa-Träger (Mountain-owned orphan-docs, gemessen 2026-10-12/327)

`register_lookup --orphan-docs` = **8**; die vier Mountain-eigenen tragen ihren Namenträger hier:
- `docs/plans/ref-mariner-faden-index.md` (3) — Träger: `mariner_rst`/DSC-0119-Punkte (M4-Parser-Gap).
- `docs/plans/ref-ephemeriden-quellen-index.md` — Träger: der 8-Ziele-Punkt oben.
- `docs/auftrag/auftrag-2026-10-11-wartend-aufloesung.md` (3) — Träger: Teil C „offene Messlücken".
- `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` (1) — Träger: Re-Admission-Punkt.
- `docs/surveys/survey-2026-09-03-orphan-verdicts.md` (1) — Träger: Quellen-Terms-Punkt.

## Operator-Wort-Register

- **„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes … /commit ist das Commit-Wort"** | 2026-10-12 | Operator-Session (mountain 327) — session-weiter Delegations-Consent, nicht das Commit-Wort; bereits in folge326 registriert, hier für dieses Atom bestätigt.
- **„du kannst dich auch gleich mal darum kümmern deine arbeit richtig zu machen und nichts zu behaupten was der messung nicht standhält"** | 2026-10-12 | Operator-Session (mountain 327) — der teilgefaltete future-241-Block wird hier vollständig gefaltet.
- **„setze eine taucher staffel auf alle punkte an … und river analysiert selbst schon die antennenhöhe"** | 2026-10-12 | Operator-Session (mountain 327) — neun-Taucher-Staffel (flash), THEMIS-Antennenhöhe ist River.
- **„gehören pdfs in data?"** | 2026-10-12 | Operator-Session (mountain 327) — gemessen: nein; `data/<netloc>/` trägt Datensätze (`.bin`), Dokumente liegen im privaten Korpus `state/research/`.

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort („mach das ab jetzt automatisch", 2026-10-07)
trägt Commit und Push. In diesem Atom committet (pfad-begrenzt): `src/archivar/extract.rs`,
`src/archivar/mod.rs`, `src/archivar/fetch.rs`, `src/archivar/port.rs`, `src/archivar/camera_angles.rs`,
`tools/harvest/src/bin/mariner_rst_compiler.rs`, `phi/sources.φ` (jma-Block → point-event/erbq-Familie),
`docs/handover/handover-2026-10-12-mountain-folge327.md`,
`docs/handover/archiv/handover-2026-10-12-mountain-folge326.md`.
