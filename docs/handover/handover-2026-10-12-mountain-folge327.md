<!--
  title: Handover — Mountain-Folge 327 (2026-10-12)
  session: Mountain-Linie in einem Pass — JADES tar_fits Member-Key-Kollision geheilt (Verzeichnis bleibt erhalten), tar_fits in format_readable + Port-Skip-Liste registriert, api.sensor.community als parametrischer map-Quelle gemessen (kein Arm nötig), jma-Hunk als point-event/erbq-Familie committet
  class: handover
  date: 2026-10-12
  sha256: e247cfa743a89db14119fd62a9ab422d5d4c06639a7a0dbbf012366fa1a7a63e
  status: live
-->
# Handover — Mountain-Folge 327 (2026-10-12)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der Stehende
Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`; gemessen am HEAD
`934c131f3`, seither committete Mycelium-324 (`34c131f3`→`e55dd1897`…`934c131f3`) und
Mountain-326 (`d55d6c1b8` `6ba1b67e6`)). Diese Session konsumierte
`handover-2026-10-12-mountain-folge326.md` (→ `docs/handover/archiv/`). Adressierte
Blöcke: `register_lookup --addressed mountain` = `## An mountain` aus future-241 — bereits
von folge326 gefaltet, kein neuer Inhalt. `--fired`/`--stale --persist 3` = leer.
**Jeder offene Punkt nennt seine Datei(en)/Pfade.**

## Burn: open 0.00 · close 0.11 · cap 0.50 · Grund: kurze Linien-Session (Mountain 327), deepseek-flash, keine Sub-Dispatch, kein pro/max (session_burn: Linien-Agent `$0.4604` → `$0.5677`).

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
- **Trigger:** die ASU-`url`-Zeile (Mycelium)
- **Lage:** (gemessen 2026-10-12 future-241 + ISIS-Taucher) 2 433 Zeilen = 811 Records × 3 Zeilen:
  `<JD (D-float)> <FDS>` / `<SX> <SY> <SZ>` / `<C1> <C2> <C3>`; keine Einheit im File; FDS
  `00000050`–`00529916`. Kein Reader existiert.
- **Blockade:** die `url`-Zeile (Mycelium).
- **Braucht:** `url` (Mycelium) + `format camera_angles`-Arm im Archivar (Position + Winkel je FDS-Frame).

### `mariner_rst` — SP-Begleitbänder, eigener Parser-Arm (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Parser-Pass
- **Lage:** (gemessen 2026-10-12) **Riss geklärt:** `DD005407`/`DD005409` sind **keine** Summary-Tapes,
  sondern die **Single-Precision-Begleitbänder** derselben M6/M7-Datensätze (DSC 0073, sha
  `02b97558…`, 540 510 B, JP: „one … single precision and the second … double precision").
  DD005407/09 nutzen das **ODG-Data-File-Format** (10-Wort-Label + 201-Wort-Station-Summary +
  201-Wort-Datenrecords, 848/649 Records); DD005408/10 das kompakte ~21-Wort-Format (15 366/11 748
  Records). Der `mariner_rst_compiler` liest nur das DP-Format; die SP-Bänder brauchen einen eigenen
  Arm. `state/research/…/mariner/RESULT.md` reproduziert. Anker: `state/mountain/mariner4_dsc0119_format.md`.
- **Blockade:** die Frequenz-Skalierung (roh ~1.08e12, /2⁹ ≈ 2114 MHz) ist in der erreichbaren
  DSC-0073-Seite **nicht** dokumentiert (`pending`).
- **Braucht:** SP-ODG-Arm bauen (`tools/harvest/src/bin/mariner_rst_compiler.rs`); Frequenz-Skalierung
  gegen DSC 0119/Odf-Spec pinnen oder `pending` benennen; SP-Band-`url` via Mycelium
  (`…/PSCM-00017_DD005407_…tar`, verdict HTTP 206).

### DSC 0119 / M4-Feldtabelle — Parser-Arm (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** die `url`-Zeile (Mycelium)
- **Lage:** (gemessen 2026-10-12) `state/mountain/mariner4_dsc0119_format.md` (ODG/ODP-Record,
  6-bit-Byte-Walk, Zeit dekodiert); der DSC 0119 trägt `SECTION II` (ODG DATEN-File: Master-Label
  9 Wörter + Station-Summary 201 Wörter + Datentypen 1–15) + `APPENDIX I` (ODP Logical Record).
  **Riss:** „Table 3" (Multi-Datentyp-Recordlänge) fehlt im 16-S.-Scan; Nachmessung 2026-10-12
  bestätigt: die Textschicht fehlt (Scan) → `pending` (ein Vision-Pass könnte Table 3 prüfen).
- **Blockade:** `url`/`origin` (Mycelium); Table 3.
- **Braucht:** DSC-0119-`url` (Mycelium) + Table 3 pinnen → `mariner_rst`-Arm.

### JADES `tar_fits` — Member-Key-Kollision geheilt (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Register-/Parser-Pass
- **Lage:** (gemessen 2026-10-12/327 via `tar -tf` des Zenodo-Tars 2 898 432 B, `archive_search --verdict` HTTP 206)
  Das JADES-Tar (`phi/sources.φ:32381` `format tar_fits`) trägt **4 Objektverzeichnisse × 9 Bänder** =
  36 Members: `JADES-GS-z{10,11,12,13}-0/data_f{090w,115w,150w,200w,277w,335m,356w,410m,444w}.fits`.
  Die alte `tar_fits_member_key` verwarf das Verzeichnis (`rsplit('/')`) → alle vier Objekte kollabierten
  auf denselben Key `data_f090w` … → nur **9** statt 36 Members. **Geheilt:** der Key behält den Pfad
  (`src/archivar/extract.rs::tar_fits_member_key`), Test `tar_fits_keys_members_of_distinct_directories_distinctly`.
  **Ferner:** `tar_fits` in `src/archivar/fetch.rs::format_readable` und in die Port-Skip-Liste
  (`src/archivar/port.rs:3088`) aufgenommen (`cargo check` 0/0).
- **Blockade:** keine (Kollision geheilt).
- **Braucht:** (1) `fits_to_json` (`src/archivar/extract.rs:3489`) ingestiert **kein** FITS-Bild, nur
  BINTABLE → der `format fits`-Pfad (UNCOVER `phi:32408`) liefert noch nichts; (2) der JADES-Block trägt
  noch **keine** `field`-Direktiven → 0 Kanäle; die 36 Members brauchen je eine `field`-Zeile
  (Member-Key = `<objekt>/data_<band>`) mit force/unit; (3) den `format fits`-Image-Fall entscheiden.

### THEMIS-ASI Stationstabelle — LiDAR-Restgrenze (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** eine gemessene Antennenhöhe je Station
- **Lage:** (gemessen 2026-10-12 river-196 + `fb619eaf7`/`e11aa7422`) Tabelle korrigiert in HEAD
  (ATHA 549.82 · EKAT lon −110.6703 / h 430.77 · PGEO 756.26 · MCGR 102.57). **Offen:** HRDEM (NRCan
  1 m, orthometrisch CGVD2013) gibt die Geländehöhe, nicht die **Antennenhöhe** (~2–3 m, `pending`);
  die Restunsicherheit dominiert die **Koordinatenlage** des Standorts (PGM-Koordinate = Stadt, nicht
  AUGO-I), nicht das DEM. Anker `state/mycelium/themis_asi_stations.md`,
  `state/river/themis/calibration-2026-10-12.md` §Höhen-Riss/§LiDAR.
- **Blockade:** keine gemessene Antennenhöhe / Standortkoordinate.
- **Braucht:** Antennenhöhe je Station messen **oder** als `pending` führen (kein Code liest
  `Geodetic altitude`/`site_map_altitude`).

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
- **Trigger:** Rat/Operator-Wort
- **Lage:** (gemessen 2026-10-12 via `sread`) `phi/sources.φ:11887-11893` (`oe417`, kein `terms`);
  BGS-GIC/NERC-GMD ohne Block; es gibt **keine** Register-Direktive für ein not-admit-Verdikt
  (`src/archivar/parse.rs:265` = freier String).
- **Blockade:** fehlende Register-Form.
- **Braucht:** gemessene Register-Form **oder** `descoped`; OE-417 entscheiden.

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

## An mycelium

Origin: mountain-folge327 (2026-10-12).

- **INPOP/NOE4 registriert — CDN-Lauf fehlt.** Die 12 Blöcke stehen (`phi/sources.φ:2185-2285`),
  `terms etalab-2.0`; der CDN-Release `ftp.imcce.fr` existiert noch nicht
  (`gh release view ftp.imcce.fr` = not found) — der erste `--ci-mode`-Lauf beider Compiler
  (`inpop_compiler`/`noe4_compiler`) ist die Manifestation.
- **M10-Kamera/Encounter:** `url`/`origin`/`compiler`-Zeile für
  `https://ser.im-ldi.com/M10/M10_ISIS/` (`camera_angles.txt`, 194 640 B, sha
  `0a65df59c8685e22004ce7f7afa21cf54f7aedc39c2ecb5965b8fcad3dae97bd`) — US-PD → dann `format`
  Mountain. Ebenso die Sonke/Robinson-Tabellen 1–8 und `isis.astrogeology.usgs.gov` (CC0/US-PD).
- **mariner-rst Rohband-Routen (future-236):** SPDF M6/M7/M4 `…/mariner*/celestial_mechanics/…`
  (NSSD1311/1310/1312, NASA-PD) + ASU `https://ser.im-ldi.com/M10/MVM/` (`MVM_0001–0014`).
- **DSC 0119:** `url`-Zeile für
  `spdf.gsfc.nasa.gov/pub/documents/old/documentation_from_nssdc/renamed_dataset_catalogs/mariner4_celestial-mechanics_64-077a-09a_DSC_0119.pdf` (sha `ec3c41eb…`).
- **Rätsel/NSE:** JADES `7460116`, UNCOVER `14281664`, BK18-Origin; GIC-Mäntsälä (FMI NUR).

## Operator-Wort-Register

- **„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes … /commit ist das Commit-Wort"** | 2026-10-12 | Operator-Session (mountain 327) — session-weiter Delegations-Consent, nicht das Commit-Wort; bereits in folge326 registriert, hier für dieses Atom bestätigt.

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort („mach das ab jetzt automatisch", 2026-10-07)
trägt Commit und Push. In diesem Atom committet (pfad-begrenzt): `src/archivar/extract.rs`,
`src/archivar/fetch.rs`, `src/archivar/port.rs`, `phi/sources.φ` (jma-Block → point-event/erbq-Familie),
`docs/handover/handover-2026-10-12-mountain-folge327.md`,
`docs/handover/archiv/handover-2026-10-12-mountain-folge326.md`.
