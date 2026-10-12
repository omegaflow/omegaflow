<!--
  title: Handover — Mountain-Folge 328 (2026-10-12)
  session: Mountain-Linie in einem Pass — M10-Ephemeriden-Vorzeichen-Flip geheilt (Compiler-Komposition r_SSB−r_M10 → r_M10−r_SSB), Orientierungs-Prüfung (cos gegen Mercury-SSB) ins Probe-Gate gebaut, SXSYSZ-Rahmen gegen die geheilte Ephemeride neu gemessen (UNIDENTIFIED bleibt; die CA-Zeit der S-Vektoren stimmt mit der datumsabgeleiteten CA überein)
  class: handover
  date: 2026-10-12
  sha256: 695e56497a683fe85fcd5599578d1d8dbc98df071f5b67d374226cb283a5fd75
  status: live
-->
# Handover — Mountain-Folge 328 (2026-10-12)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der Stehende
Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`). Diese Session konsumierte
`handover-2026-10-12-mountain-folge327.md` (→ `docs/handover/archiv/`). `--fired`/`--stale
--persist 3` = leer; `--addressed mountain` = 3 Blöcke (future-244, mycelium-327, river-196 —
unten gefaltet). `open_points_check` = 1 `ABSENT` (`…/mariner/RESULT.md`, Marker ohne Datei).
**Jeder offene Punkt nennt seine Datei(en)/Pfade.**

## Burn: open 0.00 · close 0.10 (Linie; `session_burn` 0.0964, läuft nach) · cap 0.50 · Grund: Ein-Pass-Atom (M10-Vorzeichen-Fix im Compiler + Orientierungs-Gate + Rahmen-Neumessung), deepseek-flash, kein pro/max.

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

### M10-Bildpfad — Riss „Erbe → Brennstoff": die Auflösung braucht die eigene M10-Bahn (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Fund einer M10-Radio-Tracking-Quelle (Zweiweg-Doppler/Range) oder das Operator-Wort
- **Lage:** (gemessen 2026-10-12/328) `camera_angles.txt` (`phi:31602`) trägt **zwei Rollen**: die Sondenposition `SX SY SZ` (**DE/SPICE-abgeleitet** — deckt sich mit der geheilten `ephemeris_mariner10.bin`; `|S|`-Min = 11 183 km @ CA JD 2442136,378) UND die Kameraorientierung `C1C2C3`. Solange **beide** aus der Datei kommen, ist jedes M10-Bild ein Re-Ingest des Hauses → **Erbe**. **Gemessen fehlt die eigene M10-Bahn:** SPDF `mariner/mariner10/celestial_mechanics_and_radio_science/` trägt nur die Venus-Okkultation (`red_tele_signal_data_venus_occlt`; schon als `mariner_occlt` registriert, `phi:20413`); `mariner_rst` ist **M6/7** (`phi:31568`), `mariner4_odg` ist **M4** — **M10-Zweiweg-Doppler/Range = 0 im Register**. **Auflösung (Entwurf; gehört durch die fünf Stimmen):** (1) Position ← **eigene M10-Bahn** aus Radio-Tracking (nicht `SX SY SZ`); (2) Attitude ← aus den **Bildern selbst** (ISIS-cnet/Sternfeld, future-244), nicht `C1C2C3`; dann Bild → **Merkur-Richtung im ICRF** → O−C gegen DE/INPOP/EPM. Die Häuser fitten Tracking/LLR/Radar, **nicht M10-Optik** → ein held-out Bild-Satz ist **nicht-mitgefitteter Zeuge** = **Brennstoff**. Ohne (1)/(2) bleibt der Bildpfad **Erbe** — ehrlich benannt, nicht umbenannt.
- **Blockade:** **kein M10-Roh-Tracking online** — SPDF `mariner/mariner10/celestial_mechanics_and_radio_science/` = nur Venus-Okkultation; die NSSDC-Anfrage `PSCM-00009` (X-/S-Band-Radio-Science/Tracking) ist seit 2026-09-16 offen (`state/mail/nssdc-mariner10-pscm-00009.md`).
- **Braucht:** (1) **PDS-PPI `MARINER_10` (gemessen 2026-10-12, metadex)** trägt **8 Collections** — u. a. `M10-H-POS-6-M1-FLYBY-TRAJ-V1.0` (`POS_M1_ENC_42SEC.DAT`, 464 KB, 42 s; DOI `10.17189/1519736`) und `M10-H-POS-6-M3-FLYBY-TRAJ-42SEC-V1.0`, dazu MAG-/PLS-RDR — **kein** Roh-Doppler/ODF → das ist ein **abgeleitetes Bahnprodukt** (Pose-Zeuge, nicht Roh-Tracking): als `url` registrieren (Mycelium) und gegen die geheilte Ephemeride kreuzen; der **Roh-Track** bleibt die NSSDC-Anfrage (**Operator-Hand**); (2) der **unabhängige Attitude-Solve** aus den Bildern (die C1C2C3/SXSYSZ-Konvention ist **geklärt**, s. camera_angles-Punkt; ISIS `autoseed`/`pointreg`/`jigsaw` gegen MDIS-Basemap/DEM 166/665 m, `xfer:future-244-1`); (3) das Hold-out-Fenster **vor** dem Fit fixieren.

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
  **Gebaut (Taucher):** SP/ODG-Reader in `tools/harvest/src/bin/mariner_rst_compiler.rs`;
  **auf variablen D-Record umgestellt** (Table 3: `D = 6 + 2·N`; `sp_odg_logical_records` liest `D` aus Wort 0 und
  springt um `D`) **und auf die 201-Wort-Daten-Record-Grenze** (Fig. 2: 308-Wort-Kopf, dann je 200 Payload + 1 Check-Sum;
  das Check-Sum wird per Konstruktion übersprungen; ein unvollständiger Tail wird gedroppt, nie fabriziert;
  `sp_odg_record_observables` dekodiert je gesetztem Datentyp-Bit 1–15 das Observable-Paar; Tests
  `sp_odg_single_datatype_read_variable_d`, `sp_odg_two_logical_records_in_one_payload`, `sp_odg_partial_payload_tail_dropped`,
  `sp_odg_two_datatypes_read_variable_d`, `sp_odg_code_length_mismatch_stays_empty`;
  `cargo check -p omegaflow-harvest --bin mariner_rst_compiler` 0/0); der DP-Pfad bleibt unangetastet.
- **Blockade:** das **M6/M7-SP-spezifische** Feldlayout ist im Baum nicht transkribiert (nur DSC-0073-sha);
  die Frequenz-Skalierung (roh ~1.08e12, /2⁹ ≈ 2114 MHz) ist auf der DSC-0073-Seite nicht dokumentiert.
- **Braucht:** DSC-0073-SPFP-Layout pinnen + Frequenz-Skalierung gegen DSC 0119/Odf-Spec
  oder `pending`; SP-Band-`url` via Mycelium (`…/PSCM-00017_DD005407_…tar`, verdict HTTP 206).

### DSC 0119 / M4-Feldtabelle — Parser-Arm (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der `mariner_rst`-Parser-Pass
- **Lage:** (gemessen 2026-10-12/327) `state/mountain/mariner4_dsc0119_format.md` (ODG/ODP-Record,
  6-bit-Byte-Walk, Zeit dekodiert). **Der Vollscan gefunden** (`state/research/spdf-catalogs/all_catalogs/dsc_0119.pdf`,
  **65 S.**, 5 746 835 B, sha `fee7b06e…`) — die 21-S.-Datei unter `renamed_dataset_catalogs/` (sha `ec3c41eb…`)
  war ein **Auszug**. Der Vollscan enthält die komplette **TM GTD 900-1 „Formats of Edited DSN Tracking Data"**
  (JPL, 28 Mar 1967) mit Contents, **Table 1/2/3**, Fig. 1/2, Appendix A (Condition Codes DC1–DC7) + Daten-Listing.
  **Table 3 „Data file logical record" (printed p. 14, PDF-Seite 33) ist jetzt transkribiert:** `D = 6 + 2·N`
  (N = gesetzte Datentyp-Bits 1–15 im Code-Word; Wörter 8/9, 10/11 … = je ein Observable/Weight-Paar,
  „continued … as many pairs as there are bits"). **Riss gelöst:** der alte Stride 8 = N=1; ein Code-Word mit
  zwei Bits (N=2) → D=10. Der `mariner_rst`-SP-Arm muss den Record **variabel über D aus Wort 0** lesen.
  **Ferner vollständig transkribiert (Vision, 2026-10-12/327):** Table 1 (Master-File-Label/Summary/Station-Summary),
  Table 2 (TDP-Master-Logical-Record, Wörter 0–11), Fig. 1/2, Appendix A (DC1–DC7). **Fig. 2:** die Band-Records
  sind 10 / 201 / 97 / je 201 Wörter; die variablen `D`-Logikrecords liegen **innerhalb** des 200-Wort-Payloads,
  je Daten-Record mit eigenem Check-Sum-Wort. **Riss im Arm geheilt (2026-10-12/327):** `sp_odg_logical_records`
  walkt jetzt die 201-Wort-Daten-Records (200 Payload + Check-Sum) und darin die gepackten variablen `D`-Logikrecords
  (s. `mariner_rst`-Punkt).
  **Registriert:** die *Auszug*-Datei `phi:31588-31590`; die **Volldatei** ist noch nicht registriert (s. `## An mycelium`).
- **Blockade:** keine (Feldtabellen vollständig gelesen).
- **Braucht:** den SP-ODG-Arm auf **variablen D-Record** umstellen (`mariner_rst_compiler.rs`); die Volldatei als `url` (Mycelium).

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
  TRISP→NIST NCNR NSE; NC-admissibel: INTERMAGNET (CC BY-NC 4.0), GIRO DIDBase (CC BY-NC-SA 4.0).
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
- **Lage:** (gemessen 2026-10-12/327, `general`-Taucher) ALMA→**ESO** CC BY 4.0 (ESO-held Records; almascience-Seite ohne Lizenzaussage) · GRDC→**GRDC-Caravan** CC BY 4.0 / **USGS NWIS** CC0 1.0 / **Copernicus GloFAS** CEMS-free · RIPE RIS→**RouteViews** CC BY 4.0 (kein BY-SA auf der Site; Archiv selbst ohne Lizenz) · YBCO-NSE→**NIST NCNR NSE** US-PD (Zenodo `18306252` = restricted → decline) · INTERMAGNET (CC BY-NC 4.0) + GIRO DIDBase (CC BY-NC-SA 4.0) = **NC-Frage** (Projekt-NC-Politik).
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

### NSSDC-Katalog 19740012502 → Ephemeriden-Register (eigen)
- **Status:** eigen | **Bindung:** eigen | **Trigger:** der nächste Register-/Verdikt-Pass
- **Lage:** (gemessen 2026-10-12/327, `grind-flash`-Taucher aus dem 106 349-Zeilen-Volltext) **194 Einträge** (Mariner 2/4/5/6/7/9 · Pioneer 6/7/8/9/10); **2 registriert, 192 ohne Registerzeile**. Index: `state/research/ntrs-19740012502/mariner-venus-pioneer-index.txt`. **Ephemeriden-relevanter Kern (Experiment „Celestial Mechanics"):**
  `64-077A-09` / `-09A` „Trajectory Parameters vs Time on Tape" (**registriert** via DSC 0119 + `PSCM-00021`) · `64-077A-09B` „Celestial Mechanics Listing (2-way Doppler vs Time) on 16-mm microfilm" · `67-060A-07`/`-07A` „Celestial Mechanics / Doppler Radio Tracking Data on Tape" (Mariner 5) · `69-014A-05`/`-05A` „Range and Range-Rate Listing on Magnetic Tape" (Mariner 6) · `69-030A-05`/`-05A` (Mariner 7) · `65-105A-07`/`-07A` „Doppler Radio Tracking Data on Tape" (Pioneer 6) · `72-012A-00D` „Preliminary Trajectory Chart on Hardcopy" (Pioneer 10). Mariner 9 trägt **keinen** Celestial-Mechanics-Eintrag; Mariner 10/Pioneer 11 sind 1973 noch nicht im Katalog (Pioneer Venus 1978, kein Venera).
  **Online-Audit (gemessen 2026-10-12/327, `nssdc-route-audit.txt`):** die zweistelligen IDs (`64-077A-…`) sind von **keinem** NMC-Route akzeptiert (`dataset/`/`experiment/display.action?id=<ID>` → HTTP 200 mit *„A valid … ID must be specified"*); die vierstellige Basis (`1964-077A-09` … `1965-105A-07`) löst als **Experiment-Seite** auf, die nur eine Collection-**Query** nennt (kein `PSCM`/`PSPG`-ID eingebettet). **Abgleich:** `69-014A-05A` (Mariner 6) → gedeckt durch das registrierte `mariner_rst` (`PSCM-00017`, `phi:31532-31538`); `69-030A-05A` (Mariner 7) → `mariner_rst` (`PSCM-00014`); `64-077A-09B` → `mariner4_odg` (`PSCM-00021`). (`NSSD1311`/`NSSD1310` = **0** Treffer in phi — die Rohbänder sind über die PSCM-IDs registriert, nicht über die NSSD-Nummern.)
- **Blockade:** die ID-Fläche der NMC-Routen deckt die 1973er Datensatz-IDs nicht ab; datensatz-genaue Identität ist über NMC nicht belegbar.
- **Braucht:** für die **weiterhin fehlenden** Mariner 5 (`67-060A-07/-07A`), Pioneer 6 (`65-105A-07/-07A`), Pioneer 10 (`72-012A-00D`) je die Online-Route/Anfrage prüfen (NSSDC-Kontakt = Operator-Hand); die gedeckten IDs als Zuordnung in den vorhandenen Blöcken vermerken.

### DSC-Nummer → Mission — Zuordnung gebaut (eigen)
- **Status:** eigen | **Bindung:** eigen | **Trigger:** der nächste Inventar-Pass
- **Lage:** (gemessen 2026-10-12/327, `general`-Taucher) die Kreuzreferenz liegt **nicht** unter `renamed_dataset_catalogs/` (HTTP 404), sondern unter **`old_data_doc/spreadsheet_dsc.xls`** (HTTP 200, 424 448 B, sha `b1f8523dd90eecfb307fbc35878b47dc5d18f99b37a94e92fc267f6bc5470ea3` — deckt die früher genannte `b1f8523d…`); lokal `state/research/spdf-catalogs/spreadsheet_dsc.xls`. **Map gebaut:** `state/research/spdf-catalogs/dsc-number-map.txt` — **114 Zeilen** (36 Mariner · 81 Pioneer · 28 Venus, Überlappungen), Form `DSC_<n> | <NSSDCA-id> <mission> <instrument>`, extrahiert aus den `renamed_dataset_catalogs`-Dateinamen (die XLS selbst ist ein BIFF-Workbook und unter dem erlaubten Werkzeugprofil **nicht** spaltentreu lesbar — `sgrep` bestätigt die Spaltenköpfe `DSC# | Data Set Old ID | Mission | Data Set Name | Agent | Form`).
- **Blockade:** das XLS-Format (BIFF) ist mit den erlaubten Werkzeugen nicht in Zeilen zerlegbar.
- **Braucht:** die Map aus den Filenamen ist da (114); für den vollen 614er-Satz die XLS mit einem Rust-Reader (BIFF) öffnen — oder die `renamed_dataset_catalogs`-Dateinamen als Quelle belassen (832 DSC-PDFs dort).

### M10 camera_angles — Rahmen GEKLÄRT; unabhängiger Attitude-Solve offen (eigen)
- **Status:** eigen | **Bindung:** eigen | **Trigger:** der unabhängige Attitude-Solve aus den Bildern (ISIS `autoseed`/`pointreg`/`jigsaw` oder nativ)
- **Rahmen (gemessen 2026-10-12/328, geschlossen):** ASU-ISIS-Kubus (`ENC3.tar`, FDS `00528794`) gegen dieselbe `camera_angles.txt`-Zeile — **exakter Treffer**: `SX SY SZ` = `SC_TARGET_POSITION_VECTOR` (`-65067.6517330,-4393.2208670,-21102.7624258`; **Sonde relativ zum Zielkörper Merkur**, J2000, km) · `C1` = `INSTRUMENT_RIGHT_ASCENSION` (5.3866006°) · `C2` = `INSTRUMENT_DECLINATION` (18.5581285°) · `C3` = `INSTRUMENT_TWIST` (203.7848361°). Die Konvention ist damit **entschlüsselt** (nicht aus dem geschlossenen Robinson-1999-Paper, sondern aus dem Kubus-Label). NAIF trägt für M10 **kein CK/IK** (`aareadme.txt`: Robinsons Pointing „not converted into C-kernels"; SPK nur 3/21–4/4/74 = M1, **M2/M3 ohne SPK**) → das Pointing ist **nicht** aus Kernen ableitbar, sondern nur aus den Bildern zu lösen.
- **Lage:** (gemessen 2026-10-12/327; neuer Bin `tools/measure/src/bin/m10_camera_angles_geometry.rs`, gelaufen) 811 Frames in **drei** Clustern (JD-Lücke 175,9 d vs. Median 0,000486 d, Ratio 361 760×): **M1 450** Frames (JD 2442135.305–2442137.606, FDS 50–27475) · **M2 333** (JD 2442312.241–2442312.514) · **M3 28** (JD 2442488.370–2442488.915). `|S|` (SX SY SZ) = 11 183 … 1 123 524 km; `|S|`-Minimum: M1 11 183 km @ FDS 00000050 · M2 50 511 km @ FDS 00166745 · M3 12 139 km @ FDS 00528922.
  **Rahmen-Neumessung (2026-10-12/328, nach dem Vorzeichen-Fix, Bin `m10_camera_angles_frame --m10-eph <geheilt>`):** **`FRAME: UNIDENTIFIED`** — bester Kandidat `a sc-me` RMS **848 736,9 km** (830,8 % von mean |S|), kein Kandidat unter 50 km. **Aber:** das `|S|`-Minimum M1 = **11 183 km @ JD 2442136,3778** (FDS 00000050) **deckt sich mit der datumsabgeleiteten M1-CA (1974-03-29 20:47 UT = JD 2442136,366)**; die aufgaben-gestellte Referenz JD 2442137,370 trägt einen **1,004-d-Slip**. **Riss (nicht geglättet):** `m10_camera_angles_frame --m10-eph <fine>` gibt weiter `a sc-me` RMS **371 380 km** über die 450 M1-Frames — die Datei-`S` ist **zielkörper-relativ** (jetzt belegt), unsere `me` = DE440-Merkur; die eigene `sc` deckt nur M1 (JD 2442128–2442141). **Vermessung der geheilten M10 gegen DE440-Merkur:** min |sc−me| = **3 331,9 km @ JD 2442136,3660**.
  **Korrektur meiner früheren Behauptung:** das Verdikt „zielzentriert hält |S| konstant" war **falsch** — |S| = Range variiert über einen Encounter naturgemäß um ~100×.
- **Validierung (gemessen 2026-10-12/328):** am **FDS 50** (JD 2442136,3778) gibt die geheilte Ephemeride `|sc−me| = 11 103,3 km` vs. Datei-`|S| = 11 183,1 km` (≈ 80 km) → `SX SY SZ` = `r_M10 − r_Merkur` hält auch in unserem eigenen Bin. **Aber:** die Boresight ist **nicht** planetenzentriert — `angsep(`M10→Merkur`, C1C2)` = **≈ 7,5°** @ FDS 50, **≈ 51°** @ FDS 62 → `C1C2C3` ist ein echtes Scan-Platform-Pointing, **kein** Einzeiler aus dem SPK.
- **Blockade:** (a) der `cargo`-Build ist **fremd blockiert** — eine Parallel-Session hält uncommittete Kern-Edits (`src/archivar/types.rs` `FieldConfig` `force`→`channel_ref`; Konsumenten nicht migriert), `cargo build` bricht mit `unresolved field force`; der Solver-Bin `tools/measure/src/bin/m10_attitude_check.rs` (geschrieben, `|vec−S|` + RA/Dec-vs-C1C2 je Frame) ist daher **nicht gebaut**. (b) NAIF: kein CK/IK, kein SPK für M2/M3.
- **Braucht:** (1) die fremde Kern-Migration committen lassen, dann `cargo build -p omegaflow-measure --bin m10_attitude_check` + laufen lassen; (2) den echten Attitude-Solve (ISIS/nativ) sobald Mycelium die MVM-Bilder + MDIS hat (`xfer:mountain328-1/-2`); (3) `camera_angles`-`format`/`terms` (Mycelium).

### zai-Export — Such-Kontext + nicht-registrierte Mariner-Funde (eigen)
- **Status:** eigen | **Bindung:** eigen | **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-12/327, `general`-Taucher) `state/research/recherche-2026-10-12/zai-mariner-kontext.md`. **Operator-Ziel (verbatim):** „wenn wir mariner dechiffriert haben haben wir alle die verfügbar sind" + „alleine wenn wir all die daten wieder lesbar machen ist das schon verdiensvoll" — Träger ist **eine** Chat-Datei (`zai-ab59aba8-…md`, ~:23090–24721), die anderen fünf sind Kontext (nicht „5–6 Chats tragen die Recherche"). **Nicht in `phi`/`docs`** (sgrep 0 Treffer): **Mariner 5** `PSCM-00001` Doppler 14.06.–20.11.1967 request-only · **Mariner 9** `71-051A-00E` (8 Rollen, UNIVAC 1108, 800 bpi, 7-track odd, 30.05.1971–25.10.1972) request-only · **Mariner 2** Techdoc `19670019851` + Sjogren/Trask · **Mariner 5** Analyse-Paper 1974 · Inventar NASA Techdoc `19900020832` (1987).
- **Blockade:** request-only/offline (NSSDC-Kontakt = Operator-Hand).
- **Braucht:** je Fund die `url`-Zeile (Mycelium) + Admission (Mountain); die Suchdateien liegen durable in `state/research/recherche-2026-10-12/opencode/` (3 497 Dateien, aus `/tmp` gesichert).

### `ephemeris_mariner10.bin` — Vorzeichen-Flip + Ein-Granulat-Fehler geheilt (eigen)
- **Status:** eigen | **Bindung:** eigen | **Trigger:** der `--ci-mode`-Compiler-Lauf (Mycelium/CI) → CDN-Regeneration
- **Lage (Vorzeichen):** (gemessen 2026-10-12/327, `m10_frame_diag` + Compiler-Instrumentierung/328) der registrierte `ephemeris_mariner10.bin` (`phi:31924`, sha `7ad8b8bf…`) trägt den **global vorzeichenverkehrten** SSB-Vektor (`sc ≈ −me`, cos 179,55–179,79°). **Ursache (gemessen/328):** die Legacy-DAF-Segmente des `M10_archive_1.bsp` liefern **`target − center`** invers zur SPK-Erwartung — `state_over(chebs,0,body)` = `body − SSB` (+sun für body=10), `interp(discrete)` = `r_M10 − body`. Die alte Komposition `add(neg(body_wrt_m10), body_ssb)` = `C − D` = `r_SSB − r_M10`; korrekt ist **`add(body_wrt_m10, body_ssb)` = `D + C` = `r_M10 − r_SSB`** (die body heben sich auf). **Geheilt** (Komposition; `neg` entfernt). **Orientierungs-Gate:** der Probe-Loop prüft `cos(M10-SSB, Mercury-SSB)` gegen die unabhängige Merkur-Chain `state_over(chebs,0,1)` und verwirft bei `cos < 0,9`; gemessen 0,9962–1,0000.
- **Lage (Berechnungskette + zweiter Defekt, gemessen/328):** die Summe zweier body-vermittelter Ketten (`body−M10` + `body−SSB`) ist als **eine** Chebyshev-Granule über 13,7 d (Grad 17) gefittet. Der **dichte** Roundtrip (400 statt 12 Probes) misst **849 956 km** Fit-Fehler bei JD 2442136,38 (die 12-Probe-Prüfung verfehlte den Encounter: max 26 848 km). Das **andere** Erzeuger-Programm trägt genau dafür das Rezept (`horizons_compiler.rs`: `FLYBY_FINE_GRANULE_DAYS = 0,03` · `FLYBY_COARSE_GRANULE_DAYS = 0,25`). **Gemessen/328:** die generischen Erzeuger können M10 **nicht** erstellen — `ephemeris_compiler <M10_archive_1.bsp>` bricht ab (`unsupported binary format`), auch die normalisierte Kopie (`malformed Type 2 segment: RSIZE not 2 + 3N`); `spacecraft_ephemeris_compiler --naif -76` ebenso, und `naif_spacecraft_ids.tsv` trägt kein `mariner`. Es gibt **genau einen** M10-Erzeuger (`ephemeris_mariner10_compiler.rs`); er ignorierte `GRANULE_DAYS` (`let _ = GRANULE_DAYS;`) und schrieb **eine** Granule. (NAIF-Falle: Horizons löst `-76` heute auf **MSL** auf, nicht Mariner 10 — der alte Kernel-`-76` ist die historische Zuordnung.)
  **Geheilt:** 458 Granulen à 0,03 d (Granulat-Fehler 849 956 → 37 623 km am Ketten-Übergang; die verbleibende Restunschärfe ist die B1950-Cheb- vs. J2000-Diskret-Rotation am body-Switch). **Ergebnis:** `m10_frame_diag --mariner10-eph <fine>` → **min |sc−me| = 3 331,9 km @ JD 2442136,3660** — deckt sich mit der historischen M1-CA (~3×10³ km) und der datumsabgeleiteten CA. Der frühere 294 839-km-„Riss" war **fast vollständig das Ein-Granulat-Artefakt**, kein Physik-Defekt.
- **Blockade:** der CDN-Asset wird erst mit dem `--ci-mode`-Lauf neu gebaut — CDN-Manifestation = Mycelium-Duty (nicht Mountain-Hand).
- **Braucht:** Mycelium/CI: `--ci-mode`-Lauf → CDN-`ephemeris_mariner10.bin` regenerieren (648 B → 205 384 B, 458 Granulen), dann die `sha256`-Zeile `phi:31924` aktualisieren.

### `phi/sources.φ` — zwei Register-Nachträge (mycelium-327 adressiert, eigen)
- **Status:** eigen | **Bindung:** eigen | **Trigger:** die Freigabe von `phi/sources.φ` (eine Parallel-Session hält die Datei)
- **Lage:** (adressiert 2026-10-12, gefaltet) `xfer:mycelium327-1` M10↔MDIS-USGS-Mosaik `phi:31582-31586` trägt **keine `sha256`-Zeile**; gemessen (Mountain-326) `324c0e49…` über 1 880 562 475 B. `xfer:mycelium327-2` die `de441-de442`-NAIF-Blöcke (`phi:32008`) brauchen `sha256` erst nach dem Merge-Lauf.
- **Blockade:** `phi/sources.φ` ist derzeit von einer parallelen Session gehalten (uncommittet in `git status`) — ein path-scoped Commit nähme fremde Hunks mit.
- **Braucht:** sobald die Datei frei ist — `sha256`-Zeile für `phi:31582-31586` setzen (`324c0e49…`); `de441/de442`-Merge-Lauf abwarten, dann beide Tokens nach Mycelium echoen.

### `solar_f107_flux_sfu` — Zeitbasis-Defekt (river-196 adressiert, eigen)
- **Status:** eigen | **Bindung:** eigen | **Trigger:** der nächste Archivar-Pass
- **Lage:** (adressiert 2026-10-12, gefaltet) der SWPC-Textarm liefert **TDB/J2000** (`lsk.unix_to_tdb`, `extract.rs:6506`, `parse.rs:2349`), das Penticton-`f107`-Asset **rohe Unix-Sekunden** (`src/archivar/f107.rs:22`, `day*86400`); feste Verschiebung `≈ 9,4673e8 s ≈ 30,0 a` → SWPC-2026 paart mit Penticton-1996. **Zwei Archivar-Defekte:** (i) `f107::parse_series` wendet `unix_to_tdb` nicht an; (ii) `series_epoch_of` (`extract.rs:6506-6539`) wählt den ersten Key mit `"date"` — `avg_begin_date` kann `time_tag` verdrängen.
- **Blockade:** keine.
- **Braucht:** beide Arme auf dieselbe Epoch-Basis (TDB) bringen; dann den `f107`-Join erneut messen.

### M10-Pose mit ISIS nachrechnen (future-244 adressiert, eigen)
- **Status:** eigen | **Bindung:** eigen | **Trigger:** die ISIS-Kette gegen MESSENGER-SPICE
- **Lage:** (adressiert 2026-10-12, gefaltet) `xfer:future-244-1` Sonke & Robinson 2026 (PSJ 7:35, `10.3847/PSJ/ae32f5`, CC-BY) liefern nur Methode + Summary-Tabellen; `ser.im-ldi.com/M10/M10_ISIS/` = 403. Der Operator entschied: **nicht** die Autoren anschreiben, sondern die M10-Kamera-Orientierung mit ISIS gegen die SPICE-Kerne nachrechnen (M10-SPICE = Robinson 1999, `camera_angles.txt`, `phi:31568`). Vergleich gegen die Paper-Summary (CLR 705, σ0 ≈ 0,570 px).
- **Blockade:** die ISIS-Kette (USGS, CC0) ist nicht installiert; `SX SY SZ`-Frame bleibt offen (s. camera_angles-Punkt).
- **Braucht:** die ISIS-Kette gegen MESSENGER-SPICE fahren und gegen die Paper-Summary vergleichen.

## An mycelium

Origin: mountain-folge327 (2026-10-12). **Echo:** `handover-2026-10-12-mycelium-folge326.md`
hat den vorigen Block gefaltet (registriert/geechoed: ASU-MVM `phi:31519`, SPDF-M4/6/7
`phi:31526/31534`, DSC 0119 `phi:31590`, USGS-Mosaik `phi:31582`, JADES `phi:32391`, BK18, FMI-NUR,
NIST) — nur die Sonke/Robinson-Tabellen bleiben dort offen. **Echo (mycelium-327 → mountain):**
`xfer:mycelium327-1` und `xfer:mycelium327-2` sind im `## Offen` gefaltet; `phi/sources.φ` ist inzwischen **frei** (git-status leer) — die Nachträge (`sha256` `phi:31582-31586` = `324c0e49…`) können jetzt gesetzt werden.

- **M10-Attitude-Solve — zwei Quellen ins CDN (Harvest = Mycelium).** `xfer:mountain328-1` Die Merkur-Bild-Volumes liegen auf `mve_024…mve_033` (Volumes `mvm_0007`/`0008`/`0009`), browsable: `https://planetarydata.jpl.nasa.gov/img/data/mr10-m-iss-2-edr-pv0.x/mvm_0007/` (PDS-EDR `mr10-m-iss-2-edr-pv0.x`, JPL/planetarydata, US-PD). Die `ser.im-ldi.com`-Spiegel sind **unvollständig** (`mvm_0007`/`0008` = HTTP 404, `0009` = 200). `xfer:mountain328-2` Die MESSENGER-MDIS-Shape-Quellen: `https://astrogeology.usgs.gov/search/map/mercury-messenger-global-products` (USGS Astrogeology) — die Einzelseiten `…/mercury_messenger_mdis_global_mosaic_166m` und `…/mercury_messenger_mdis_dem_665m` sind HTTP 200; gebraucht werden **Basemap 166 m** + **DEM 665 m v2** als Shape-Model für den Attitude-Solve. Nach eurer `url`/`origin`-Zeile setze ich `terms`/`format` (Mountain).

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
- **NTRS `19740012502` „Data catalog of satellite experiments" (NASA-PD) — `url`/`origin`/`compiler`-Zeile.** `xfer:mountain327-7`
  Ein **606-Seiten-Index** (NASA-TM-X-69990 · NSSDC-73-11 · NSSDC-71-20-SUPPLEMENT-1, 1973-10-01) über
  alle NSSDC-Raumfahrzeuge/Experimente/Datenbeschreibungen — ein **Discovery-Index**, kein Einzeldatensatz.
  Lokal: `state/research/ntrs-19740012502/19740012502.pdf` (19 850 338 B, sha `9b4902bf…`) + Volltext
  `19740012502.txt` (1 711 714 B, 106 349 Zeilen, sha `64b44b02…`); `downloads` tragen auch `.txt`.
  Origin `https://ntrs.nasa.gov/api/citations/19740012502/downloads/19740012502.pdf`.
  Der zugehörige **Ordner** (SPDF `renamed_dataset_catalogs/`) liegt als Listing in
  `state/research/spdf-catalogs/spdf_catalogs_dir.html` (179 312 B, sha `7e9c79e9…`) — er trägt u. a.
  weitere Mariner-4-DSCs (`DSC_0045`/`DSC_0176`/`DSC_0074`) und die NSSDC-Query-/Dataset-Seiten.
- **DSC-Volldatei + `all_catalogs`-Serie (SPDF, US-PD) — `url`/`origin`/`compiler`-Zeile.** `xfer:mountain327-8`
  `https://spdf.gsfc.nasa.gov/pub/documents/old/documentation_from_nssdc/old_data_doc/all_catalogs/dsc_0119.pdf`
  — **65 S.**, 5 746 835 B, sha `fee7b06e…` (die Volldatei; die 21-S.-`renamed_dataset_catalogs`-Datei
  `ec3c41eb…` ist nur ein Auszug). Der Ordner `all_catalogs/` trägt die **komplette NSSDC-DSC-Serie**
  (`dsc_0001.pdf` … 614 Einträge); Listing in `state/research/spdf-catalogs/all_catalogs/index.html` (77 797 B).
  Lokale Kopie `state/research/spdf-catalogs/all_catalogs/dsc_0119.pdf` + Volltext `dsc_0119.txt`.
- **SPDF DSC-Kreuzreferenz `spreadsheet_dsc.xls` (US-PD) — `url`/`origin`-Zeile.** `xfer:mountain327-9`
  `https://spdf.gsfc.nasa.gov/pub/documents/old/documentation_from_nssdc/old_data_doc/spreadsheet_dsc.xls`
  (HTTP 200, 424 448 B, sha `b1f8523d…`) — die `DSC# | Data Set Old ID | Mission | Data Set Name | Agent | Form`-Tabelle;
  lokal `state/research/spdf-catalogs/spreadsheet_dsc.xls`. Sie ist der Nummer→Mission-Schlüssel der 614er-Serie.

## An river

Origin: mountain-folge327 (2026-10-12). Drei `--orphan-docs`-Dokumente sind nicht Mountain und brauchen
einen Namenträger bei dir:
- `docs/blatt/blatt-gic-breitenband-familien.md` (4 Marker; see-also `docs/paper/gic-causal-driver.md`). `xfer:mountain327-3`
- `docs/surveys/survey-2026-10-05-stoerungs-experiment-fehlende-faeden.md` (4; see-also river-folge93). `xfer:mountain327-4`
- `docs/surveys/survey-2026-10-08-sonnen-render-archaeologie.md` (2; see-also river-folge139). `xfer:mountain327-5`

## Prosa-Träger (orphan-docs, gemessen 2026-10-12/328)

`register_lookup --orphan-docs` = **6** (gemessen/328). Die in folge327 geführten fünf Mountain-Träger
(`ref-mariner-faden-index`, `ref-ephemeriden-quellen-index`, `auftrag-2026-10-11-wartend-aufloesung`,
`survey-2026-09-03-daten-holdings-inventur`, `survey-2026-09-03-orphan-verdicts`) sind **nicht mehr**
gelistet — sie tragen. Die 6 offenen:
- `docs/surveys/survey-2026-10-08-open-sources-delta.md` (12) — Quellen-Klasse, **Mountain**; Träger: der Re-Admission-Punkt oben (Inhalt beim nächsten Register-Pass binden).
- `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md` (1) — Quellen-Klasse, **Mountain**; Träger: der Quellen-Terms-/Re-Admission-Punkt.
- `docs/auftrag/auftrag-gic-einreichung.md` (1) · `docs/concepts/exzellenz-konzept.md` (2) · `docs/concepts/kybernetische-astrophysik.md` (1) · `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md` (7) — nicht Mountain (Zuordnung am Inhalt durch die Träger-Linie).

## Operator-Wort-Register

- **„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes … /commit ist das Commit-Wort"** | 2026-10-12 | Operator-Session (mountain 328) — session-weiter Delegations-Consent, nicht das Commit-Wort; in folge326/327 registriert, hier für dieses Atom bestätigt.
- **„mache hier weiter … der echte Grund … es war kein Zentrum, es war ein Vorzeichen … Ich habe den Compiler nicht blind gefixt … der Rest-Riss (nicht geglättet) … Nächster Schritt: Vorzeichen-Komposition korrigieren, Orientierungs-Prüfung ins Probe-Gate, --ci-mode-Lauf"** | 2026-10-12 | Operator-Session (mountain 328) — der Vorzeichen-Fix als benannter Auftrag; die gemessene Komposition `add(body_wrt_m10, body_ssb)` = `r_M10 − r_SSB`.
- **„aber das muss doch zu klären sein … kannst du nicht die berechnungskette nachvollziehen und was ist mit dem anderen ephemeriden berechnungsprogramm gibt das das gleiche ergebnis?"** | 2026-10-12 | Operator-Session (mountain 328) — Kette nachvollzogen: Ein-Granulat-Fit (849 956 km) als zweiter Defekt; 458 Flyby-Granulen à 0,03 d geheilt (`horizons_compiler.rs`-Rezept); min |sc−me| = 3 331,9 km @ JD 2442136,3660, deckt die historische M1-CA.
- **„du kannst dich auch gleich mal darum kümmern deine arbeit richtig zu machen und nichts zu behaupten was der messung nicht standhält"** | 2026-10-12 | Operator-Session (mountain 327) — der teilgefaltete future-241-Block wird hier vollständig gefaltet.
- **„setze eine taucher staffel auf alle punkte an … und river analysiert selbst schon die antennenhöhe"** | 2026-10-12 | Operator-Session (mountain 327) — neun-Taucher-Staffel (flash), THEMIS-Antennenhöhe ist River.
- **„gehören pdfs in data?"** | 2026-10-12 | Operator-Session (mountain 327) — gemessen: nein; `data/<netloc>/` trägt Datensätze (`.bin`), Dokumente liegen im privaten Korpus `state/research/`.
- **„das dokument ist auch noch spannend https://ntrs.nasa.gov/api/citations/19740012502/downloads/19740012502.pdf und eben der ordner"** | 2026-10-12 | Operator-Session (mountain 327) — NTRS 19740012502 (606-S.-NSSDC-Katalog) + der SPDF-`renamed_dataset_catalogs`-Ordner geladen.
- **„ich habe ein geniales dokument gefunden https://spdf.gsfc.nasa.gov/pub/documents/old/documentation_from_nssdc/old_data_doc/all_catalogs/dsc_0119.pdf"** | 2026-10-12 | Operator-Session (mountain 327) — die 65-S.-Volldatei (TM GTD 900-1 mit Table 3) + der `all_catalogs`-Serienordner.
- **GLM-Analyse (DSC-Serie = Inventar des historischen NASA-Datensatz-Korpus; Plan-Zuordnung Phase 0 = DSC-Inventar · Phase 2 = DSC je Mission → Format → Parser → Band)** | 2026-10-12 | Operator-relayed (mountain 327) — Orientierung, nicht gemessenes Verdikt.
- **„kannst du es nicht auf eine konvertierungsseite hochladen und xls ist doch nichts anderes als ein zip oder? … kannst du bitte diese dateien alle sichern? [m*-Liste] … und ja bitte [camera_angles-Measure-Lauf] … den gesprächsinhalt aus tate/zai-export/zai-ab59aba8-…"** | 2026-10-12 | Operator-Session (mountain 327) — `/tmp`-Recherche durable gesichert; XLS bleibt lokal (kein Dritt-Upload); Geometrie-Lauf gebaut; zai-Kontext extrahiert.

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort („mach das ab jetzt automatisch", 2026-10-07)
trägt Commit und Push. In diesem Atom committet (pfad-begrenzt): `tools/harvest/src/bin/ephemeris_mariner10_compiler.rs`
(Vorzeichen-Komposition + Orientierungs-Gate), `docs/handover/handover-2026-10-12-mountain-folge328.md`,
`docs/handover/archiv/handover-2026-10-12-mountain-folge327.md`.
