<!--
  title: Handover — Mountain-Folge 331 (2026-10-12)
  session: Mountain-Linie in einem Pass — f107-Identitäts-Riss durch den Rat + die Forschungsschicht (`archive_search --all`) + 7 UI-Seats untermauert und im Resolver implementiert (gemessene Diskrepanz → benannter Riss statt stiller Auswahl; `None == None`-Fabrikation gefixt); camera_angles-`terms` auf PD gesetzt; adressierte Blöcke (mycelium-330, river-200) gefaltet
  class: handover
  date: 2026-10-12
  sha256: cb4d4dd5f839452c69137522b4ecc2857eebc5109a88790e522b33d31c3b0d5c
  status: live
-->
# Handover — Mountain-Folge 331 (2026-10-12)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der Stehende
Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`). Diese Session konsumierte
`handover-2026-10-12-mountain-folge330.md` (→ `docs/handover/archiv/`). `--fired`/`--stale
--persist 3` = leer; `--addressed mountain` = 2 Blöcke (mycelium-330, river-200 — unten gefaltet).
`open_points_check` = 4 `ABSENT` (Parser-Artefakte: zusammengesetzte/verkürzte Pfade, s. Abschluss).
**Jeder offene Punkt nennt seine Datei(en)/Pfade.**

## Burn: open 0.00 · close 0.00 · cap 0.50 — Grund: Ein-Pass-Atom (f107-Rat + Forschungsschicht `archive_search --all` + 7 UI-Seats, Resolver-Fix + Test, camera_angles-`terms`, adressierte Blöcke gefaltet), deepseek-flash, kein pro/max. Die close-Zahl setzt `session_burn` nach Session-Ende.

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
- **Trigger:** die ASU-`url`-Zeile (registriert; Block liegt jetzt `phi:31610-31616`)
- **Lage:** (gemessen 2026-10-12/327) 2 433 Zeilen = 811 Records × 3 Zeilen:
  `<JD (D-float) + FDS>` / `<SX> <SY> <SZ>` / `<C1> <C2> <C3>`, je mit trailing Label; keine Einheit im File.
  **Gebaut (Taucher):** Modul `src/archivar/camera_angles.rs` (`pub fn parse_camera_angles(text) -> Vec<CameraAngleFrame>` mit jd/fds/sx/sy/sz/c1/c2/c3; D-Exponent, Label-Prüfung, malformed → skip; Tests), `pub mod camera_angles;` in `src/archivar/mod.rs`; **Dispatch:** `format camera_angles` in `src/archivar/extract.rs` (`camera_angles_to_json`/`camera_angles_frames_to_json`, Tests `camera_angles_frames_become_json_rows`/`camera_angles_yield_no_rows_without_a_frame`), `format camera_angles` im Block; `cargo check` 0/0.
- **Blockade:** keine (Arm + Dispatch + `format` + `terms` gesetzt).
- **Braucht:** **`terms` gesetzt (2026-10-12/331): `terms PD https://ser.im-ldi.com/M10/`** — Mariner-10-Missionsdaten + NASA-finanziertes Robinson-et-al.-1999-Pointing = NASA-/USGov-PD, Geschwister des `ser.im-ldi.com/M10/`-EDR-Blocks `phi:31550-31553`; der Mirror selbst trägt keine Terms-Aussage (Nutzungs-Text nur bei NASA). Verbleibt: die **FDS→Bild-Dateiname**-Zuordnung (der FDS ist die Bilddatei, nicht die Zeit — JD/FDS nicht ko-monoton) beim ersten Lauf prüfen.

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
- **Braucht:** **Taucher-Messung (2026-10-12/331): `pending` — die Skalierung ist in keiner Baum-/Netzquelle belegt.** Der DSC-0073-Datenrecord typisiert die Frequenz als **DPFP, nicht SPFP** (DSC 0073 Anlage W. 4-5 „TRANSMITTER FREQUENCY — DPFP"; OCR `state/research/scratch/recherche-2026-10-12/mariner-texte/dsc73_ocr.txt:274-275`); der `/2^9`-Faktor ist ein **numerischer Fit** (M6 1,082359e12 → 2113,98 MHz; M7 1,083311e12 → 2115,84 MHz; `…/mariner-texte/RESULT.md:48-53`), keine dokumentierte Regel. Die DSN-Tape-Format-Spezifikation (`TM GTB900-1`, im Begleitschreiben zitiert) fehlt im Baum; NTRS `19730023976` ist `METADATA_ONLY` (`"downloads":[]`), NTRS-Suchen leer. **Riss:** die Aufgabe fragt nach SPFP-M6/7 — deren Record ist DPFP. Ferner: `state/research/spdf-catalogs/all_catalogs/dsc_0119.txt` ist **leer** (65 B, 1 Leerzeile) — kein Volltext, Quelle ist allein der PDF-Scan (`archive_search --pdf-text` → `pending — no text layer`). SP-Band-`url` via Mycelium (`…/PSCM-00017_DD005407_…tar`, verdict HTTP 206) offen.

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
  → Kanäle. **Taucher-Messung (2026-10-12/331):** der Extraction-Pfad ist gemessen — `format tar_fits` →
  `tar_fits_to_json`/`tar_fits_bytes_to_json` (`src/archivar/extract.rs:3598-3637`) → je Member `fits_to_json`,
  ohne BINTABLE Fallback `fits_image_bands_to_json` (`:3489-3596`); der Wert liest sich als `<member-key>.0.mean`
  (`jnum`/`jpath_val`, `src/archivar/json.rs:290-360`). `field`-Kernform (`src/archivar/parse.rs:1588-1697`):
  `field <member>.0.mean <channel-name> inverse-square em <unit> 33554432.0 0.0 0.0 <nu_hz>`. **Offen:** die
  **BUNIT** je Member (aus dem FITS-Header) muss vor dem Schreiben der 36 Zeilen gemessen werden; `µm` ist kein
  zulässiges `unit`-Token (`units.rs` `allowed_units_for_force(0)` trägt `nm`, normalisiert `um`) → Einheit wählen/ableiten.

### THEMIS-ASI Stationstabelle — an River übergeben (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-12 via river-196/`sread`) Operator-Wort 2026-10-12/327: „river analysiert selbst schon die antennenhöhe" — die Antennenhöhen-Frage (HRDEM Geländehöhe ≠ Antennenhöhe ~2–3 m; PGM-Koordinate = Stadt, nicht AUGO-I) liegt bei **River** (`state/river/themis/calibration-2026-10-12.md` §Höhen-Riss/§LiDAR; river-196). Kein Mountain-Akt.
- **Blockade:** —
- **Braucht:** River-Analyse; Mountain ohne Schritt.

### `intermagnet_dbdt` — Resolver-Regel (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** die Fenster-Disjunktheit der vier Arme
- **Lage:** (gemessen 2026-10-12/330) Resolver gebaut (Mycelium `3467ec4f9`); Verdikt
  `state/stimmen/2026-10-12_river-intermagnet-identitaet-rat.md` (Mountain-Messung angehängt).
  **Die Zelle ist gemessen** (Lauf `38215224525`, head `3467ec4f9` = der Resolver-Commit, Job
  `matrix-newell-omni`): der Resolver greift (Riss `abk_dbdt_1m` vs `_1h`, corr 0,9388, feinster
  Zeuge 1m/n 527039), aber die Zelle liefert **keinen Rung**:
  `newell_dphi_dt_omni->intermagnet_dbdt | 0 | 0 | – | absent | 3.6000e3 | … | alignment pending`,
  `0 of 1 cells pass`. `align_many` (`field_te_query.rs:4659-4660`) verwirft mit
  `Err("bin window carries no overlap between the arms")` — die vier Arme (3 OMNI-Carrier + ABK-Band)
  teilen **kein gemeinsames Zeitfenster**.
- **Blockade:** der Grund der Fenster-Disjunktheit (live-OMNI-Fenster vs. ABK-Archiv-Fenster) ist ungemessen.
- **Braucht:** die Zeit-Fenster der vier registrierten Arme messen (`first`/`last` je Serie) und den Descriptor `src/mathematikerin/descriptors/newell_geospheric_omni.te` — **nie lokal**, Job `matrix-newell-omni` in `.github/workflows/field-te-query.yml` — auf ein gemeinsames Fenster stellen.

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
- **Solver gemessen (2026-10-12/328, Bin `tools/measure/src/bin/m10_attitude_check` gebaut, `cargo build` 0/0):** über die **450 M1-Frames** `|vec − SXSYSZ|` **mean 165 410 km, max 941 373 km @ JD 2442136,50131 (FDS 304)**; `angsep(M10→Merkur, C1C2)` mean 16,9°, max 154,8°. **Nah am Encounter stimmt es:** FDS 50 `|vec| = 11 139,3` vs. `|S| = 11 183,1 km` (`|vec−S| = 328 km`), FDS 52–62 ~340–390 km. **Riss (neu, nicht geglättet):** der Fehler sitzt **außerhalb des Merkur-Kettenfensters** — am FDS 304 (JD 2442136,5013, Sun#2-Kette) ist `sc = [-6 834 808, -61 155 920, -32 061 162]`, `vec = [-821 530, 75 938, 71 890]`, aber Datei-`S = [116 399, 2 143, 39 833]` → **~941 000 km, fast ganz in x**. Das heißt: die geheilte Ephemeride ist **nur im Merkur-Kettenfenster (2442136,235–499) wahr**; die **Sun-Ketten-Komposition** (Sun#1 bis .240, Sun#2 ab .499) trägt einen **~0,78°-Konsistenzfehler** (beide Ketten stimmen in `|r|` auf ~6 km, aber nicht im Vektor). Nächster Schritt war: die Sun-Ketten prüfen. **Geheilt/329 (gemessen):** der Riss war die **halbe B1950-Rotation** der Komposition (nur der Chebyshev-Term wurde rotiert), nicht die Kettenauswahl — s. `ephemeris_mariner10.bin`-Punkt.
- **Boresight-Befund:** `C1C2C3` ist **nicht** planetenzentriert (7,5° @ FDS 50, 51° @ FDS 62) → echtes Scan-Platform-Pointing, der Attitude-Solve braucht die Kamera + Bildregistrierung.
- **Blockade:** (a) NAIF: kein CK/IK, kein SPK für M2/M3; (b) der echte Attitude-Solve braucht die Kamera + die Bildregistrierung.
- **Braucht:** der echte Attitude-Solve ist **`wartend`**: (1) die MVM-Merkur-Bilder — Lauf `38269896436` **failure** (URL-Join `…pv0.x`+`mvm_0007`, gemessen `/tmp/opencode/jpl_fail.log:647`); **geheilt** (Mycelium `9e4e87bf0`: `{base}{volume}` → `{base}/{volume}`), Re-Dispatch `38277503423` **queued**; (2) der MDIS-Shape — Lauf `38275761488` **pending**; (3) die **ISIS-Kette** als CI-Container (`xfer:mountain329-4`). Trigger: CDN-Abschluss + ISIS-Container. `terms` auf `phi:31558`/`phi:31623-31631` ist gesetzt (`0051e5fd6`); `format mariner_m10_edr` steht; offen bleibt Mycelium's `camera_angles`-`format`/`terms`.

### zai-Export — Such-Kontext + nicht-registrierte Mariner-Funde (eigen)
- **Status:** eigen | **Bindung:** eigen | **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-12/327, `general`-Taucher) `state/research/recherche-2026-10-12/zai-mariner-kontext.md`. **Operator-Ziel (verbatim):** „wenn wir mariner dechiffriert haben haben wir alle die verfügbar sind" + „alleine wenn wir all die daten wieder lesbar machen ist das schon verdiensvoll" — Träger ist **eine** Chat-Datei (`zai-ab59aba8-…md`, ~:23090–24721), die anderen fünf sind Kontext (nicht „5–6 Chats tragen die Recherche"). **Nicht in `phi`/`docs`** (sgrep 0 Treffer): **Mariner 5** `PSCM-00001` Doppler 14.06.–20.11.1967 request-only · **Mariner 9** `71-051A-00E` (8 Rollen, UNIVAC 1108, 800 bpi, 7-track odd, 30.05.1971–25.10.1972) request-only · **Mariner 2** Techdoc `19670019851` + Sjogren/Trask · **Mariner 5** Analyse-Paper 1974 · Inventar NASA Techdoc `19900020832` (1987).
- **Blockade:** request-only/offline (NSSDC-Kontakt = Operator-Hand).
- **Braucht:** je Fund die `url`-Zeile (Mycelium) + Admission (Mountain); die Suchdateien liegen durable in `state/research/recherche-2026-10-12/opencode/` (3 497 Dateien, aus `/tmp` gesichert).

### `ephemeris_mariner10.bin` — Sun-Ketten-Komposition geheilt (B1950-Summe einmal rotiert); CDN-Regeneration offen (eigen)
- **Status:** eigen | **Bindung:** eigen | **Trigger:** der `--ci-mode`-Compiler-Lauf (Mycelium/CI) → CDN-Regeneration
- **Lage (Vorzeichen):** (gemessen 2026-10-12/327, `m10_frame_diag` + Compiler-Instrumentierung/328) der registrierte `ephemeris_mariner10.bin` (`phi:31924`, sha `7ad8b8bf…`) trägt den **global vorzeichenverkehrten** SSB-Vektor (`sc ≈ −me`, cos 179,55–179,79°). **Ursache (gemessen/328):** die Legacy-DAF-Segmente des `M10_archive_1.bsp` liefern **`target − center`** invers zur SPK-Erwartung — `state_over(chebs,0,body)` = `body − SSB` (+sun für body=10), `interp(discrete)` = `r_M10 − body`. Die alte Komposition `add(neg(body_wrt_m10), body_ssb)` = `C − D` = `r_SSB − r_M10`; korrekt ist **`add(body_wrt_m10, body_ssb)` = `D + C` = `r_M10 − r_SSB`** (die body heben sich auf). **Geheilt** (Komposition; `neg` entfernt). **Orientierungs-Gate:** der Probe-Loop prüft `cos(M10-SSB, Mercury-SSB)` gegen die unabhängige Merkur-Chain `state_over(chebs,0,1)` und verwirft bei `cos < 0,9`; gemessen 0,9962–1,0000.
- **Lage (Berechnungskette + zweiter Defekt, gemessen/328):** die Summe zweier body-vermittelter Ketten (`body−M10` + `body−SSB`) ist als **eine** Chebyshev-Granule über 13,7 d (Grad 17) gefittet. Der **dichte** Roundtrip (400 statt 12 Probes) misst **849 956 km** Fit-Fehler bei JD 2442136,38 (die 12-Probe-Prüfung verfehlte den Encounter: max 26 848 km). Das **andere** Erzeuger-Programm trägt genau dafür das Rezept (`horizons_compiler.rs`: `FLYBY_FINE_GRANULE_DAYS = 0,03` · `FLYBY_COARSE_GRANULE_DAYS = 0,25`). **Gemessen/328:** die generischen Erzeuger können M10 **nicht** erstellen — `ephemeris_compiler <M10_archive_1.bsp>` bricht ab (`unsupported binary format`), auch die normalisierte Kopie (`malformed Type 2 segment: RSIZE not 2 + 3N`); `spacecraft_ephemeris_compiler --naif -76` ebenso, und `naif_spacecraft_ids.tsv` trägt kein `mariner`. Es gibt **genau einen** M10-Erzeuger (`ephemeris_mariner10_compiler.rs`); er ignorierte `GRANULE_DAYS` (`let _ = GRANULE_DAYS;`) und schrieb **eine** Granule. (NAIF-Falle: Horizons löst `-76` heute auf **MSL** auf, nicht Mariner 10 — der alte Kernel-`-76` ist die historische Zuordnung.)
  **Geheilt/328:** 458 Granulen à 0,03 d; Ein-Granulat-Fehler 849 956 → 37 623 km am Ketten-Übergang.
  **Geheilt/329 (Sun-Ketten-Riss, gemessen):** die B1950-Komposition wurde nur halb rotiert — nur der Chebyshev-Term `c` (`state_over(chebs,0,body)`), das diskrete body-wrt-M10 `d` blieb unrotiert. Die diskreten Segmente liegen (wie die Chebyshev-Segmente) im **FK4-B1950**; die Summe wird jetzt **einmal als Ganzes** rotiert (`rot_apply(&rot, add(body_wrt_m10, body_wrt_ssb_b1950))`). Ketten-Überlappung JD 2442136.238: Sun#1 `d+rot(c)` = [-7 849 733, …] vs. Merkur-Kette [-7 006 833, …] = **843 000 km**; Sun#1 `rot(d)+rot(c)` = [-7 006 677, …] = **63 km**. Am FDS 304 (JD 2442136,50131) liegt `rot(d)+rot(c)` = [-5 896 915, -61 229 896, -32 093 305] auf **205 km** an der unabhängigen `camera_angles`-Wahrheit `me+S` (der alte `d+rot(c)` lag 941 000 km daneben). **Ergebnis:** Roundtrip-Ketten-Übergang **37 623 → 26,5 km**; `m10_attitude_check --m10-eph <fixed>` über 450 M1-Frames: **|vec−S| mean 502 / max 29 009 km** (vorher 165 410 / 941 373), angsep mean **1,53°** (vorher 16,9°), FDS 50 **199 km** (vorher 328 km). Diagnose-Modus `--chain-diag` im Compiler misst X1=`d+rot(c)` / X2=`rot(d)+rot(c)` / X3=`d+c`.
  **Rest-Riss (nicht geglättet):** der Max 29 009 km sitzt allein an FDS 25325 (JD 2442135,305, `|S| ≈ 962 000 km`); dort ist |vec| ≈ |S| auf 0,7 %, der Rest ist Richtung (~1,7°). Da JD und FDS in M1 nicht ko-monoton sind (früher gemessen), verschiebt die Datei-`S` gegenüber dem JD — als **FDS-Mapping-Artefakt** zu prüfen, nicht als Ketten-Fehler.
- **Blockade:** der CDN-Asset wird erst mit dem `--ci-mode`-Lauf neu gebaut — CDN-Manifestation = Mycelium-Duty (nicht Mountain-Hand).
- **Braucht:** Mycelium/CI: `--ci-mode`-Lauf mit dem geheilten Compiler → CDN-`ephemeris_mariner10.bin` regenerieren (458 Granulen, 205 384 B), dann die `sha256`-Zeile `phi:31924` aktualisieren.

### PDS POS-6 (`M10-H-POS-6-M1-FLYBY-TRAJ-V1.0`) — SEDR-Position online, Range-Cross gegen die Ephemeride (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Ephemeriden-/Attitude-Pass
- **Lage:** (gemessen 2026-10-12/329) `POS_M1_ENC_42SEC.DAT` (PDS PPI, HTTP 200, 475 152 B, 3126 × 152-B-Records; detached `.LBL` 38 400 B) trägt **42-s-SEDR-Positionsproben** (AAREADME: „from the Mariner 10 SEDR"): Time (f64 BE, s seit 1966-01-01) + 36 f32 BE; Spalten 8–10 = S/C-Position relativ zu Merkur (`|POSM|` = `DIST S/C TO MERCURY`, exakt). **Gebaut:** `tools/measure/src/bin/m10_pos6_check.rs` (Parser + Cross). **Ergebnis:** über 3126 Records ist `|POS6| − |sc−me|` **mean 1936 km, max 9139 km** (Distanzen 3,1e3–1,6e6 km) → die geheilte Ephemeride ist in der **Range** unabhängig bestätigt. **Riss:** die Vektorrichtung steht in einem **undeklarierten festen Frame** (Label `UNIT = NULL`; feste ~67,6°-Rotation um RA 222,0°/Dec 64,2° von J2000, Best-Fit-Residuum mean 4427 km). Kandidaten direkt getestet — **Ekliptik** (±23,44°), **Galaktisch** (beide Richtungen), **B1950**: Residuum 7,9e5–1,6e6 km, **kein Treffer**; ebenso nicht die Merkur-IAU-Polachse (281,0°/61,4°). Der Frame ist eine missionseigene, im Online-PDS-Volumen **nicht** dokumentierte Konvention (AAREADME: „not a PDS product … not peer reviewed"; CATALOG/INDEX/NSSDC/NTRS/ADS/Web tragen keine Definition). CA: POS-6-Minimum **3144,9 km @ JD 2442136,3655** vs. Ephemeride **3332 km @ .3660** (~200 km, ~43 s).
- **Blockade:** die Frame-Definition des SEDR fehlt (das POS-6-Label deklariert keinen Frame; nur die SEDR-Dokumentation/der Offline-SEDR trägt sie).
- **Braucht:** Mycelium `url`-Zeile für `M10-H-POS-6-M1-FLYBY-TRAJ-V1.0`; den SEDR-Frame aus der NSSDC-Antwort (`PSPA-00318`) klären, dann den Vektor-Cross schließen.

### `phi/sources.φ` — Mosaik-`sha256` gesetzt; `de441/de442` offen (mycelium-327 adressiert, eigen)
- **Status:** eigen | **Bindung:** eigen | **Trigger:** der `de441/de442`-Merge-Lauf
- **Lage:** (gemessen 2026-10-12/330) `xfer:mycelium327-1` **erledigt** (Commit `dcdd4f15e`):
  der Block `…/planetarymaps.usgs.gov/Mercury_M1_M2_M3_M10_mosaic_global_2010.tif` trägt jetzt
  `sha256 324c0e497570f908db6a85d4e4823a9b76da1f676d0ecdb1ba8f67559e14e52f` (Gate-Wert des
  Workflows `.github/workflows/m10-mdis-raw-cdn.yml:71`; die alte `phi:31582-31586`-Referenz war
  stale — der Block liegt bei `phi:31625`). `xfer:mycelium327-2` die `de441-de442`-NAIF-Blöcke
  brauchen `sha256` erst nach dem Merge-Lauf.
- **Blockade:** keine (phi frei, Commit `dcdd4f15e`).
- **Braucht:** `de441/de442`-Merge-Lauf abwarten, dann den `sha256`-Nachzug setzen und
  `xfer:mycelium327-1`/`-2` nach Mycelium echoen.

### `solar_f107_flux_sfu` — Zeitbasis geheilt; der Join bleibt korrelationslos (river-197-1, eigen)
- **Status:** eigen | **Bindung:** eigen | **Trigger:** der nächste `field_te_query`-Lauf
- **Lage:** (gemessen 2026-10-12/330) **Die beiden genannten Archivar-Defekte sind bereits geheilt** (Commit `d55d6c1b8`, mountain 326 „heal the f107 TDB epoch"): `f107::parse_series` (`src/archivar/f107.rs:20-25`) wendet `lsk.unix_to_tdb(day*86400)` an; `series_epoch_of` (`src/archivar/extract.rs:6668`) prüft **zuerst** `INSTANT_TIME_KEYS` (`time_tag` …) und überspringt im Fallback jeden `avg`-Key. **Der geforderte Re-Mess** (Lauf `38215224525`, head `3467ec4f9`, Job `witness-jma`) zeigt: die Riss-Regel greift (SWPC `f107_cm_flux.json` median_dt 18000s vs `f107_penticton.bin` 86400s, **overlap corr −0,0634**), aber der Join bleibt bei ≈ 0 — **die Zeitbasis war nicht die Ursache des Korrelationsrisses**.
- **Blockade:** keine (Resolver-Fix gebaut, `cargo check` 0/0, `rustfmt --check` grün).
- **Braucht:** **Rat + Forschungsschicht + 7 UI-Seats konvergent (2026-10-12/331): Identitäts-Riss → kein erzwungener Join.** Die `resolution`-Regel besteht auf **gemessener Identität** (deklarierte Merkmale UND Überlappungs-Korrelation ≥ 0,99), nicht auf einer Quellen-Klasse. **Implementiert:** `aggregation_pair_kind` (`tools/measure/src/bin/field_te_query.rs`) unterscheidet `DeclaredSame`/`Candidate`/`Distinct` — `None == None` ist **nicht** „derselbe Empfänger", sondern zwei nicht-deklarierte; eine **gemessene** Diskrepanz (`corr < 0,99`) liefert jetzt einen benannten Riss statt der stillen Auswahl des feineren Arms — beide Arme bleiben zwei Knoten, die Zelle liest `pending` (Test `aggregation_pair_kind_never_reads_undeclared_as_the_same_receiver`; die bestehende `aggregation_correlation`-Schwelle unberührt). Rohmaterial `state/stimmen/2026-10-12_mountain_f107-identitaet-ui.md`; Forschungsschicht `state/research/spill/omegaflow_all_1791824847_solar_f10_7_radio_flux_SWPC_vs_Penticton.txt`. **Offener Riss unter den Seats** (nicht geglättet): Claude/Duck lesen „dieselbe Größe, falsches Kriterium" (Pearson ≠ Identität; Residuen-Test in sfu ~1 %); GLM warnt, den Gate **aggregationsbewusst** (gröbste gemeinsame Kadenz, Mindestüberlappung) anzuwenden. Vor dem endgültigen Split in zwei Register-Zeilen (z. B. `…@swpc_recent` / `…@penticton`) den Residuen-Test als zweite Messung prüfen.

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
`xfer:mycelium327-1` **erledigt** (Mountain `dcdd4f15e`: Mosaik-`sha256 324c0e49…` gesetzt);
`xfer:mycelium327-2` (`de441/de442`-sha) bleibt offen bis zum Merge-Lauf. `xfer:mountain327-1` **erledigt** (`0051e5fd6` Mosaik-Block = `phi:31625`).

- **PDS POS-6 M10-Flyby-Trajektorie (PDS/NASA) — `url`/`origin`/`compiler`-Zeile.** `xfer:mountain329-2` `https://pds-ppi.igpp.ucla.edu/data/M10-H-POS-6-M1-FLYBY-TRAJ-V1.0/DATA/POS_M1_ENC_42SEC.DAT` (HTTP 200, 475 152 B) + `.LBL` (HTTP 200, 38 400 B); DOI `10.17189/1519736`. Zweite unabhängige M1-Ephemeriden-Quelle (SEDR-Position); die Cross-Messung steht in `tools/measure/src/bin/m10_pos6_check.rs`. `M10-H-POS-6-M3-FLYBY-TRAJ-42SEC-V1.0` ist die M3-Schwester.

- **ISIS-Kette für das M10-Kontrollnetz (B2/Phase 4) — CI-Container + `url`/`origin`-Zeilen.** `xfer:mountain329-4` Die S&R2026-Methode (SPICE-Pose `camera_angles` + MESSENGER-DEM → ISIS `autoseed`/`pointreg` → Bundle Adjustment; Gate σ0 ≈ 0,570 px) ist der **Türsteher** (nicht der POS-6-Frame). Sie braucht: (1) **ISIS** (USGS, public domain) als **CI-Container** (conda/apt) — nie auf dem Operator-Rechner (heavy compute = CI); (2) die M10-CLR-Bilder (ASU `ser.im-ldi.com`, PD) als CDN-Assets; (3) **MESSENGER-Global-DEM v2** + MDIS-Basemaps (PDS, PD) als `url`-Zeilen. Das ist **Umgebung/CI** (Mycelium), kein Mountain-Register-Akt; Mountain liefert `terms`/`format` je Zeile und fährt die Methode.

- **M10-Ephemeride — CDN-Regeneration nach dem Kompositions-Fix (Mycelium/CI).** `xfer:mountain329-1` Der Compiler `tools/harvest/src/bin/ephemeris_mariner10_compiler.rs` ist geheilt (B1950-Summe einmal rotiert statt nur des Chebyshev-Terms; im selben Atom committet). **Gemessen (mountain-330):** der Lauf `38277021535` (`mariner10-ephemeris-cdn`, **success**) hat das Asset **nicht** neu gebaut — die Idempotenz-Stufe (`mariner10-ephemeris-cdn.yml:31-41`) sieht `ephemeris_mariner10.bin` als vorhanden und **skippt**; `archive_search --sniff` der CDN-URL gibt **648 B / sha `7ad8b8bf…`** = das alte, im Sun-Fenster um bis zu 941 000 km falsche Asset. **Braucht:** `gh workflow run mariner10-ephemeris-cdn.yml -f force=true` (Mycelium-Dispatch), der Compiler erzeugt 458 Granulen / 205 384 B (lokal sha `67949cb6718be222c62c47286f16edbfbab5e35975fbcf0dfb9257645bbe0ae8`); danach die `sha256`-Zeile `phi:32027` (`at mariner10`) auf den neuen Wert setzen (Mountain).

- **M10-Attitude-Solve — Quellen ins CDN (Status gemessen, Echo).** `xfer:mountain328-1/-2` **Erledigt (Mycelium, 2026-10-12):** Family A MVM-Merkur-Bilder (`phi:31558`; Compiler `mariner_m10_jpl_edr_compiler`, Workflow `m10-jpl-edr-cdn.yml`) und Family B MDIS-Shape (`phi:31623-31631`, Basemap 166 m `d4c0880c…`, DEM 665 m v2 `defce776…`; Workflow `m10-mdis-raw-cdn.yml`) sind registriert + dispatcht. **Status (gemessen 2026-10-12/330):** Lauf `38269896436` (jpl-edr) **failure** — URL-Join `…pv0.x`+`mvm_0007` fehlender `/`; **geheilt** (`9e4e87bf0`, `{base}{volume}` → `{base}/{volume}`), Re-Dispatch `38277503423` queued. Lauf `38275761488` (mdis-raw) **pending**. **Mountain erledigt:** `terms` auf `phi:31558`/`phi:31623-31631` (`0051e5fd6`), `format mariner_m10_edr` steht; der Attitude-Solve ist `wartend` auf CDN-Abschluss + ISIS (`xfer:mountain329-4`).

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
  Lokale Kopie `state/research/spdf-catalogs/all_catalogs/dsc_0119.pdf`; die Datei `dsc_0119.txt` ist **leer** (65 B, 1 Leerzeile, gemessen 2026-10-12/331 — kein Volltext, `archive_search --pdf-text` → `pending — no text layer`, Quelle ist der PDF-Bildscan).
- **SPDF DSC-Kreuzreferenz `spreadsheet_dsc.xls` (US-PD) — `url`/`origin`-Zeile.** `xfer:mountain327-9`
  `https://spdf.gsfc.nasa.gov/pub/documents/old/documentation_from_nssdc/old_data_doc/spreadsheet_dsc.xls`
  (HTTP 200, 424 448 B, sha `b1f8523d…`) — die `DSC# | Data Set Old ID | Mission | Data Set Name | Agent | Form`-Tabelle;
  lokal `state/research/spdf-catalogs/spreadsheet_dsc.xls`. Sie ist der Nummer→Mission-Schlüssel der 614er-Serie.

## An river

Origin: mountain-folge327 (2026-10-12). **Antworten auf river-197/199 (gemessen 2026-10-12/330):**
- `xfer:river197-1` (`solar_f107_flux_sfu`): die genannten zwei Archivar-Defekte sind **schon geheilt**
  (`d55d6c1b8`, mountain 326) — `f107.rs:20-25` wendet `unix_to_tdb` an, `series_epoch_of` (`extract.rs:6668`)
  priorisiert `INSTANT_TIME_KEYS`. Der Re-Mess (Lauf `38215224525`) zeigt aber **corr −0,0634** — der
  Join bleibt korrelationslos; der Riss ist **nicht** die Zeitbasis, sondern Fenster/Größe (s. Offen).
- `xfer:river197-2` (MCON 1966–2025 vs Parser-Ende 2024-05-29): als Info getragen, kein Mountain-Akt.
- `xfer:river197-3` (THEMIS-ASI Stationstabelle): bei River erledigt — kein Mountain-Akt.
- `xfer:river199-1` (P9.1): unten beantwortet.

Drei `--orphan-docs`-Dokumente sind nicht Mountain und brauchen
einen Namenträger bei dir:
- `docs/blatt/blatt-gic-breitenband-familien.md` (4 Marker; see-also `docs/paper/gic-causal-driver.md`). `xfer:mountain327-3`
- `docs/surveys/survey-2026-10-05-stoerungs-experiment-fehlende-faeden.md` (4; see-also river-folge93). `xfer:mountain327-4`
- `docs/surveys/survey-2026-10-08-sonnen-render-archaeologie.md` (2; see-also river-folge139). `xfer:mountain327-5`
- **Antwort (Owner-Pen) auf `xfer:river199-1`:** die `channel`-/Deskriptor-Direktiven in `phi/sources.φ` setzt **allein Mountain** (P9.1) — River committet sie **nicht** (Rat + Operator-Wort 2026-09-27: alleiniger Schreiber der `phi/sources.φ`-Verdikt-Zeilen; „the pen is the owner's"; ein fremder Register-Schreibakt reist als Register-Zeile, nicht als fremder Edit). **Erledigt (Commit `7aa6fdd32`):** `phi:27` → `seismic-body`-Kanal (`…elastic-solid:unspecified:none Pa`), `phi:164` → `electric`-Kanal (`…maxwell:elliptic:vacuum:unspecified:none V/m`); damit **9 `channel`-Zeilen / 9 unique** (die zwei Duplikate aufgelöst), plus Abdeckungstest `the_register_declares_a_channel_for_every_live_force_tag`. Die Deskriptoren wurden **nicht** geraten (kanonische Quelle `FIXED_CHANNELS`/`MECHANISMS`). **River kann den register-sourced `force_id_of`-Handle bauen.** `xfer:mountain329-3`

## Prosa-Träger (orphan-docs, gemessen 2026-10-12/328)

`register_lookup --orphan-docs` = **6** (gemessen/328). Die in folge327 geführten fünf Mountain-Träger
(`ref-mariner-faden-index`, `ref-ephemeriden-quellen-index`, `auftrag-2026-10-11-wartend-aufloesung`,
`survey-2026-09-03-daten-holdings-inventur`, `survey-2026-09-03-orphan-verdicts`) sind **nicht mehr**
gelistet — sie tragen. Die 6 offenen:
- `docs/surveys/survey-2026-10-08-open-sources-delta.md` (12) — Quellen-Klasse, **Mountain**; Träger: der Re-Admission-Punkt oben (Inhalt beim nächsten Register-Pass binden).
- `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md` (1) — Quellen-Klasse, **Mountain**; Träger: der Quellen-Terms-/Re-Admission-Punkt.
- `docs/surveys/survey-2026-10-10-worldwide-lidar-quellen.md` (52) — Quellen-Klasse, **Mountain** (gefaltet aus `xfer:mycelium330-1`, 2026-10-12/331); Träger: die LiDAR-Quellen-Klasse (dieser Träger ist der Owner-Aufenthalt).
- `docs/auftrag/auftrag-gic-einreichung.md` (1) · `docs/concepts/exzellenz-konzept.md` (2) · `docs/concepts/kybernetische-astrophysik.md` (1) · `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md` (7) — nicht Mountain (Zuordnung am Inhalt durch die Träger-Linie).

## Operator-Wort-Register

- **„bei council auch archive search all und die ui chats (alle und max!) befragen"** | 2026-10-12 | Operator-Session (mountain 331) — die f107-Ratsfrage durch die Forschungsschicht (`archive_search --all`) + **alle** UI-Seats (mountain-ui + geteilte `open-weight-ui`, max) untermauert; ausgeführt (7 Seats, `state/stimmen/2026-10-12_mountain_f107-identitaet-ui.md`).
- **„und dann vergese auf keinen fall die ergebnisse der taucher zu implementieren"** | 2026-10-12 | Operator-Session (mountain 331) — die Taucher-/Rat-/UI-Ergebnisse im selben Atom implementiert (Resolver-Fix, camera_angles-`terms`).
- **„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes. /commit ist das Commit-Wort"** | 2026-10-12 | Operator-Session (mountain 331) — session-weiter Delegations-Consent, nicht das Commit-Wort; für dieses Atom bestätigt.
- **„die drei fehlenden `terms`-Zeilen sind gesetzt"** | 2026-10-12 | Operator-Session (mountain 330) — die Register-Duty (MVM JPL-EDR + zwei MDIS-`.tif`) ist mit `0051e5fd6` erledigt; der Attitude-Solve steht vor dem Solve (`wartend` auf CDN + ISIS).
- **„Will ich als nächstes die Sun-Kette debuggen (der Fehler sitzt fassbar in der Komposition)?"** | 2026-10-12 | Operator-Session (mountain 329) — der Sun-Ketten-Riss als benannter Auftrag; geheilt: die B1950-Komposition wird einmal als Ganzes rotiert (nicht nur der Chebyshev-Term), gemessen gegen die Merkur-Kette (63 km) und `camera_angles` (205 km).
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
trägt Commit und Push. In diesem Atom (folge331) committet (pfad-begrenzt): `phi/sources.φ`
(camera_angles-`terms` = `PD https://ser.im-ldi.com/M10/`), `tools/measure/src/bin/field_te_query.rs`
(f107-Resolver: `aggregation_pair_kind` — `None == None` ist nicht derselbe Empfänger, gemessene
Diskrepanz → benannter Riss statt stiller Auswahl), `docs/handover/handover-2026-10-12-mountain-folge331.md`,
`docs/handover/archiv/handover-2026-10-12-mountain-folge330.md`. Im gitignorierten `state/`:
`state/stimmen/2026-10-12_mountain_f107-identitaet-ui.md`, `state/operator-gespraeche/2026-10-12-mountain.md`,
`state/research/spill/omegaflow_all_1791824847_solar_f10_7_radio_flux_SWPC_vs_Penticton.txt`; `state/zustand/ui-open-weight.lock` gesetzt und
wieder gelöscht. **CI:** `field-te-query.yml` Lauf `38283460159` dispatcht (2026-10-12/331) — misst
den Resolver-Fix am Baum (Jobs `matrix-newell-omni` / `witness-jma`); das Ergebnis nimmt der
Stehende Pass / die nächste Session. **Warnung an die nächste Session:** der Arbeitsbaum ist von **parallelen Linien**
gehalten (River/Mycelium committen `phi`/`src/*` laufend); vor jedem `block_commit` das
Release-Binary-Verhalten gegen die Arbeitsbaum-Änderungen prüfen.
