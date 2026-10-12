<!--
  title: Handover — Mountain-Folge 326 (2026-10-12)
  session: Mountain-Linie in einem Pass — f107 TDB-Zeitbasis geheilt (extract.rs/f107.rs), JADES plain-tar→FITS-Arm tar_fits gebaut, BK18 format-text + uK^2-Einheit registriert, MCON-Spanne / ISIS-Identität / mariner_rst-Bänder gemessen; adressierte Blöcke future-241 + river-196 gefaltet
  class: handover
  date: 2026-10-12
  sha256: b685688e405c63065f6b17d8597b49fa1c2da270f05dd827a1d3a919a6eebc73
  status: live
-->
# Handover — Mountain-Folge 326 (2026-10-12)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der Stehende
Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`; der Pass steht am
HEAD `9e370d0e7`, seither committete Mycelium-323 (`2bd6d3ce4`) und River-196
(`fb619eaf7` THEMIS-Stations-Tabelle, `e11aa7422` LiDAR-Befund)). Diese Session
konsumierte `handover-2026-10-12-mountain-folge325.md` (→ `docs/handover/archiv/`).
Adressierte Blöcke gefaltet: `## An mountain` aus
`state/future/handover/handover-2026-10-12-future-folge241.md`
(future-235/236/238/239/240/241) und `docs/handover/handover-2026-10-12-river-folge196.md`
(river-195/196). **Jeder offene Punkt nennt seine Datei(en)/Pfade.**

## Burn: open 0.00 · close 0.29 · cap 0.50 · Grund: lange Linien-Session (Mountain 326) + 8 flash/general-Dispatches (THEMIS-STATIONS, f107, ISIS, MCON, BK18, JADES `tar_fits`, mariner_rst/DSC 0119, BK18-Register), deepseek-flash, kein pro/max (`session_burn`: Linien-Agent `$0.6008`→`$0.8949`).

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
  **future-241-Update:** die M10-Pose je Frame liegt jetzt in
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

### JADES `tar_fits` — gebaut, Folgeschritte (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-12) `tar_fits`-Arm gebaut: `src/archivar/extract.rs` (`tar_fits_to_json`
  + `fits_image_bands_to_json` + Dispatch), `format tar_fits` auf dem JADES-Block (`phi/sources.φ:32280`).
  **Offen:** (1) `fits_to_json` ingestiert **kein** FITS-Bild (`extract.rs:3489`, nur BINTABLE);
  (2) der JADES-Block trägt noch **keine** `field`/`rows`-Direktiven → 0 Kanäle;
  (3) `fetch.rs::format_readable` (`:682`) und `port.rs`-Skip-Liste (`:3088`) kennen `tar_fits` nicht
  → `format-gap` in der Diagnose.
- **Blockade:** keine (Bau steht).
- **Braucht:** `field`/`rows` auf dem JADES-Block messen + setzen; `tar_fits` in `format_readable`/
  Skip-Liste eintragen.

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
- **Lage:** (gemessen 2026-10-12 via Rat) Resolver gebaut (Mycelium `3467ec4f9`: per-`station_code`,
  feinster `median_dt` als Zeuge, gröberer als getragener Riss); Verdikt
  `state/stimmen/2026-10-12_river-intermagnet-identitaet-rat.md`. Die Zelle `newell_dphi_dt_omni->
  intermagnet_dbdt` wartet auf den Lauf `38215224525`.
- **Blockade:** Datei hält parallele Session.
- **Braucht:** `ci_manage log 38215224525` lesen, die Zelle neu messen.

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
- **Blockade:** Admission-Verdikte je Kandidat fehlen; Risse RouteViews BY-vs-BY-SA, ESO-CC-BY nur
  für ESO-ALMA-Records.
- **Braucht:** Admission je Kandidat (Mountain-Verdikt); danach Mycelium-CDN.

### `api.sensor.community` — Trigger leer (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-11/12) `phi/sources.φ:330-340` trägt einen vollständigen
  `format`/`field`-Block; kein Future-Wort (Ursprung `mycelium-158…162` 403 + `mountain-306`).
- **Blockade:** keine (eigene Namenswahl).
- **Braucht:** Arm/Zielzeile benennen **oder** `descoped`.

### Rätsel-Datenlücken (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** die `url`-Zeilen (Mycelium)
- **Lage:** (gemessen 2026-10-12 via `sread`) `phi/sources.φ:32235` (UNCOVER `14281664`, `format
  fits` + `CC-BY-4.0`) · `:32201` (JADES `7460116`) · BK18 (erledigt) · GIC-Mäntsälä (FMI NUR
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
  an `src/mathematikerin`; Frame/Epoche CRS-VLR ungemessen; `mycelium-phi-serien-ciplauf` (Klima-
  Workflows, erster Lauf je Familie) ist Nicht-Dritt-Warten → hierher gefaltet.
- **Blockade:** `url`/Manifest (Mycelium); die Freigabe von `src/mathematikerin`.
- **Braucht:** je `url`-Zeile (Mycelium) → `terms`/`format` (Mountain); Läufe lesen
  (`ci_manage log <id>`); iEEG **nicht erneut vorlegen** (Operator-Wort 2026-10-06).

## An mycelium

Origin: mountain-folge326 (2026-10-12).

- **INPOP/NOE4 registriert — CDN-Lauf fehlt (Operator-Wort, future-241).** Die Lizenz ist geklärt:
  INPOP **und** NOE sind **SE-OP-Produkte unter der Open Licence / Licence Ouverte 2.0**
  (GTCU `https://seop.obspm.fr/en/terms/`) → Redistribution erlaubt. Mountain hat 12 Blöcke gesetzt
  (`phi/sources.φ:2185-2285`): `ephemeris_inpop_{mercury,venus,mars,jupiter,saturn,uranus,neptune,sun,moon,earth}.bin`
  (Format `ephemeris_inpop`, `origin` inpop19a-SPICE-Tar, Compiler `inpop_compiler`) +
  `ephemeris_noe4_{phobos,deimos}.bin` (Format `ephemeris_noe4`, `origin` NOE-4-2020.bsp, Compiler
  `noe4_compiler`), alle `terms etalab-2.0 https://seop.obspm.fr/en/terms/`. **Der CDN-Release
  `ftp.imcce.fr` existiert noch nicht** (`gh release view ftp.imcce.fr` = not found) — der erste
  `--ci-mode`-Lauf beider Compiler ist die Mycelium-Manifestation. Der `weberin_verdicts`-Block
  (`phi:20587`) zieht dieselben Origins (inpop19a + epm2021).
- **M10-Kamera/Encounter:** für den M1-Kamera-Blindfleck brauche ich die `url`/`origin`/`compiler`-
  Zeile für `https://ser.im-ldi.com/M10/M10_ISIS/` (`camera_angles.txt`, 194 640 B, sha
  `0a65df59c8685e22004ce7f7afa21cf54f7aedc39c2ecb5965b8fcad3dae97bd`) — US-PD → dann `format`
  Mountain. Ebenso die Sonke/Robinson-Tabellen 1–8 (future-241: t1–t4 IOP-Supplement, USGS-Kombi-
  Mosaik `Mercury_M1_M2_M3_M10_mosaic_global_2010.tif`, ASU ISIS-Tars) und `isis.astrogeology.usgs.gov`
  (CC0/US-PD).
- **mariner-rst Rohband-Routen (future-236):** SPDF M6/M7/M4 `…/mariner*/celestial_mechanics/…`
  (NSSD1311/1310/1312, NASA-PD) + ASU `https://ser.im-ldi.com/M10/MVM/` (`MVM_0001–0014`).
- **DSC 0119:** `url`-Zeile für
  `spdf.gsfc.nasa.gov/pub/documents/old/documentation_from_nssdc/renamed_dataset_catalogs/mariner4_celestial-mechanics_64-077a-09a_DSC_0119.pdf` (sha `ec3c41eb…`).
- **Rätsel/NSE:** JADES `7460116`, UNCOVER `14281664`, BK18-Origin; GIC-Mäntsälä (FMI NUR).
- **BK18 (erledigt):** der `format text`-Block + `uK^2`-Einheit stehen (`phi/sources.φ:31300-31329`,
  `src/archivar/units.rs` thermal `uk^2`; die neue Force/Unit-Paarung ist bis zum nächsten
  `tools-build`-Release in `docs/specs/force-unit-baseline.txt` eingefroren). Danke.

## Operator-Wort-Register

- **„bitte mache das richtige … WIR BAUEN UNSERE EIGENEN EPHEMERIDEN!!!! und wir retten KULTURERBE"** | 2026-10-12 | Operator-Session (mountain) — Auftrag angelegt; kein Quick-and-dirty.
- **„ja bitte lege ihn an und route ihn und wichtig den kompletten UI roster … openweight max und seek … nicht nur duck und claude"** | 2026-10-12 | Operator-Session (mountain) — Rat + voller UI-Roster.
- **„du kannst die parallel fahren und glm kannst du abholen"** | 2026-10-12 | Operator-Session (mountain) — Open-Weight parallel; GLM abholen.
- **„verstehst du jetz warum ich auf glm 5.3 bestehe?"** | 2026-10-12 | Operator-Session (mountain) — GLM Deep Think Max trägt den tiefsten Riss.
- **„ist denn geschwindigkeit wirklich ein problem … ihr müsst euch nur angewöhnen die ui chats richtig zu nutzen … glm start > pending schluss"** | 2026-10-12 | Operator-Session (mountain) — Runden-Reihenfolge (langsamstes zuerst).
- **„warte ich habe kein glm guthaben soll ich welches kaufen … oder reicht unser kostenfreier weg?"** | 2026-10-12 | Operator-Session (mountain) — Antwort: der kostenfreie Weg reicht; Kauf kauft Tempo, nie Urteil.
- **„übergeben … komplett nicht nur oberflächlich … die pfade mit in eure übergabe schreiben"** | 2026-10-12 | Operator-Session (mountain) — diese Übergabe trägt Pfade je Punkt.
- **„Starte die Mountain-Linie in einem Pass … /commit ist das Commit-Wort"** | 2026-10-12 | Operator-Session (mountain 326) — session-weiter Delegations-Consent, nicht das Commit-Wort.

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort („mach das ab jetzt automatisch", 2026-10-07)
trägt Commit und Push. In diesem Atom committet (pfad-begrenzt): `src/archivar/extract.rs`,
`src/archivar/f107.rs`, `src/archivar/units.rs`, `docs/specs/force-unit-baseline.txt`,
`phi/sources.φ`,
`docs/handover/handover-2026-10-12-mountain-folge326.md`,
`docs/handover/archiv/handover-2026-10-12-mountain-folge325.md`.
