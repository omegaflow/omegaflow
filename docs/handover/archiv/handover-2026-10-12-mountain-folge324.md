<!--
  title: Handover — Mountain-Folge 324 (2026-10-12)
  session: Mountain-Linie in einem Pass — Mariner-10-EDR Angular-Durchmesser gebaut (Riss: EDR-Korpus Venus vs M10-Ephemeride Merkur-1, disjunkt), Vinyl-terms/format-Kanten gesetzt (mariner_m10_edr PD, UNCOVER fits, JADES/BK18), Gate-Regel prior-work-unread gebaut
  class: handover
  date: 2026-10-12
  sha256: 0c8fea6701a88a1e5cdc8eddd398018f86d2207a931ba4fed55e7fd35ae919b0
  status: live
-->
# Handover — Mountain-Folge 324 (2026-10-12)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`). Diese
Session konsumierte `handover-2026-10-12-mountain-folge323.md` (→ `archiv/`).

## Burn: open 0.00 · close 0.12 · cap 0.15 · reason: Linien-Session (Mountain) + 4 flash-Taucher (M1-Bau, 2× Lizenz-/Format-Recherche, Gate-Regel), deepseek-flash, kein pro/max (lokal nur flash).

## In diesem Atom erledigt (git trägt es; nicht erneut vorlegen)

- **M1 gebaut — die Entscheidungsmessung, und sie liefert einen Riss.**
  `tools/measure/src/bin/m10_expected_diameter.rs` (Rust std, `cargo check` 0/0, build
  clean). Sie liest die Frame-Epochen aus `mariner_m10_edr.bin` (unix-Sekunden) und die
  Bahnen Mariner 10 / Venus / Merkur, rechnet `2·asin(R/range)` und vergleicht mit dem FOV
  (0,42°/0,98°). **Lauf-Ergebnis: 651 eindeutige Frames, alle 1974-02-05/06 (Venus)**; die
  registrierte `ephemeris_mariner10.bin` (aus `M10_archive_1.bsp`) deckt nur
  **JD 2442128.03–2442141.75 (Merkur-1, ≈1974-03-29)**. Die beiden registrierten Assets sind
  **disjunkt** — 0 Frames meßbar, alle namentlich übersprungen (kein erfundener Wert).
  NAIF `…/naif/M10/kernels/spk/` trägt nur `M10_archive_1.bsp`. `body_barycenter_position`
  liefert **Meter** (nicht km, Korrektur der Vorgabe).
- **R1:** `mariner_m10_edr`-`terms` **`PD`** gesetzt (`phi:31386`, `https://ser.im-ldi.com/M10/`);
  gemessen: die ASU-Spiegelseite trägt keine Lizenzaussage, die Daten sind rohe JPL/NASA-EDR-Tapes
  (US-Government) — konsistent zu den bestehenden `PD`-Ankern auf JPL/PDS-Hosts.
- **R2:** `uncover_dr4_sps_catalog.fits` `terms CC-BY-4.0` + `format fits` (Parser existiert,
  `fits_to_json`); `jades_early_galaxies.tar` `terms CC-BY-4.0`; `bk18_bandpowers_20210607.txt`
  `terms CC-BY-4.0`. Quelle je Zenodo-API (`"license":{"id":"cc-by-4.0"}`) bzw. BK18-Release-Seite
  („Creative Commons Attribution 4.0 International Public License").
- **Gate-Fixture (Operator-Wort „5 wähle das richtige"):** neue handover-gescannte Regel
  **`prior-work-unread`** in `src/gate/commit_gate.rs` (+ Vocab-feedback + Test) — eine
  Übergabe-Zeile, die einen Vorarbeits-Artefakt unter `state/` mit einer Byte-Größe nennt,
  **ohne** Re-Meßstempel (`measured`/`gemessen`), wird geflaggt. **Grund (gemessen):** die
  code-only `fabrication`-Vocab greift nicht auf `.md` (`is_code_path`), die 66-MB-Regression
  lebte in der Handover-Prosa — die Vocab-Zeile (b) wäre am Regressionsort inert gewesen.
- **E1:** `ephemeris_house_gate` grün — Erde (DE/INPOP/EPM) innerhalb 22,1/15,9/30,7 km am
  JUICE-Perigäum (2026-09-28); Register `data/flyby2/house-gate-2026-09-28.json`.

## Offen (aufgeschlüsselt)

### M10-EDR Winkeldurchmesser — Riss: EDR-Korpus ↔ M10-Ephemeride disjunkt (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** eine Deckspanne der M10-Ephemeride, die den Venus-Encounter trifft
- **Lage:** (gemessen 2026-10-12 via `m10_expected_diameter`) EDR-Asset = 651 Frames, alle
  Venus 1974-02-05/06 (`mvm_0001.tgz`, Tape 1); `ephemeris_mariner10.bin` = Merkur-1
  (JD 2442128–2442141). 0 meßbare Frames.
- **Blockade:** keine registrierte M10-Ephemeride deckt den Venus-Encounter (NAIF nur
  `M10_archive_1.bsp`).
- **Braucht:** eine **breitere M10-Ephemeride** über alle Encounters (JPL Horizons —
  `ephemeris_horizons_check`/`horizons_compiler` prüfen; Mycelium `url`/`origin`, dann
  Mountain `terms`/`compiler`), **oder** den passenden Encounter-Korpus (Mercury-Tapes
  `mvm_0002+`). Danach M1 erneut fahren; **dann** entscheidet sich M4 (Detektor-Bau) — nicht vorher.

### M10-EDR-Detektor (M4) — an M1 gebunden (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** ein gemessenes „Limbus möglich" aus der erweiterten M1
- **Lage:** (gemessen 2026-10-11/12) Design steht (Rat+UI+Research, folge321: radiusfixierter
  subpixel Limb-Fit, PSF/erf je Azimut, Radius fest aus Range+Brennweite, Huber/Tukey, kein
  Vision-LLM); `m10_edr_image.rs` ist nur der Stretch-Extraktor. Venus (5.2.1974) hat
  **keine Oberflächenmerkmale** (Wolkendecke) — der Merkur-Kontrollnetz-Pfad gilt für Merkur.
- **Blockade:** M1 hat keinen meßbaren Frame (s. o.).
- **Braucht:** erst die erweiterte M1; bei „Limbus möglich" bauen, sonst `descoped` mit M1-Befund.

### JADES `.tar` — Parser-Arm `tar_fits` fehlt (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Bau-Pass
- **Lage:** (gemessen 2026-10-12) `jades_early_galaxies.tar` (Zenodo 7460116, `terms CC-BY-4.0`
  gesetzt) trägt FITS-Bilder; es gibt **keinen** Plain-`.tar`→FITS-Arm (nur `tar_gz_yaml`
  gzip-only; `fits_to_json` liest nur BINTABLE, keine Image-HDUs).
- **Blockade:** neuer Parser-Arm.
- **Braucht:** `tar_fits`-Arm (Plain-Tar via `inflate::tar_members` + FITS-Image-Ingest) bauen,
  dann `format tar_fits` setzen.

### BK18 `text` — Zeilen-Direktiven fehlen (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Bau-Pass
- **Lage:** (gemessen 2026-10-12) `bk18_bandpowers_20210607.txt` (`terms CC-BY-4.0` gesetzt) ist
  whitespace-kolumnar; der generische `Extract::Rows`-Pfad braucht `format text` + `at <body>` +
  `rows .` + Spalten-Direktiven (Muster: Venera-Block `phi:32300`). Spaltenlayout ungemessen.
- **Blockade:** Spaltenlayout nicht gemessen.
- **Braucht:** BK18-Spaltenköpfe messen, dann `format text`/`at earth`/`rows .`/`quantity`
  setzen.

### `mariner_rst` — Summary-Tapes + Observable-Skala (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-11 via grind-flash) Parser auf M6 `DD005408` (55 900 Observablen)
  / M7 `DD005410` (38 572) verifiziert. Offen: `DD005407`/`DD005409` sind strukturell andere
  (Summary-)Tapes; die Observable-Skalierung einzelner Datentypen ist am Original gegenzuprüfen
  (Frequenz roh 1.08e12, /2⁹ ≈ 2114 MHz).
- **Blockade:** die Zusatz-Tapes brauchen eine eigene Messung.
- **Braucht:** die Summary-Tapes messen oder `descoped`; Observable-Skalierung gegen DSC 0073/Spec.

### DSC 0119 / M4-Feldtabelle — Parser-Arm (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** die `url`-Zeile (Mycelium)
- **Lage:** (gemessen 2026-10-11/12 via future-238, `--pdf-image` + vision) DSC 0119
  (HTTP 200, 1 380 249 B, sha256 `ec3c41ebbb082287437fe6ff9b9ba91a7fd9474311b274d8cbd975ddd5ce55b0`)
  trägt `SECTION II` ODG-Data-File + `APPENDIX I` ODP-Logical-Record + `Fig. 2`. `TX OTD900-1`/
  `TR32-1106` nicht online (NTRS/IA/Web = 0).
- **Blockade:** `url`/`origin` (Mycelium).
- **Braucht:** `url`-Zeile für DSC 0119 (Mycelium) → `terms`/`format` (Mountain) →
  `mariner_rst`-Parser-Arm.

### Bild ↔ Doppler — Optical-Navigation-Kreuz-Constraint (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** ein Frame mit gemessenem Zentroid
- **Lage:** (gemessen 2026-10-11 via Rat + Research) Bildwinkel + Doppler binden gemeinsam die
  deklarierte `Receiver`-Worldline per Least Squares; Mariner 10 **nicht** mit Onboard-OpNav
  → post-hoc. **Keine** Doppler-Kalibrierung.
- **Blockade:** die Bildserie trägt kein gemessenes Ziel (M10-Punkt darüber).
- **Braucht:** Eintritt als abgeleiteter Term im ω()-Loop über die geteilte Receiver-Worldline
  (kein Sample-Slot); `Riss`, wo Bild und Doppler sich nicht überlappen.

### `solar_f107_flux_sfu` — Zeitbasis-Defekt (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Bau-Pass
- **Lage:** (gemessen 2026-10-12, `general`) SWPC-Textarm liefert TDB/J2000 (`extract.rs:6506`,
  `lsk.unix_to_tdb`), Penticton-`f107` rohe Unix-Sekunden (`src/archivar/f107.rs:22`); feste
  Verschiebung ≈ 30,0 a → r≈0.02. Zwei Defekte: (i) `f107::parse_series` ohne `unix_to_tdb`;
  (ii) `series_epoch_of` (`extract.rs:6506-6539`) wählt den ersten `"date"`-Key.
- **Blockade:** beide Arme brauchen dieselbe Epoch-Basis (Signatur-/Aufrufer-Änderung).
- **Braucht:** beide Arme auf TDB bringen; den `f107`-Join erneut messen.

### `MCON`-Spanne (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-12 via mycelium-318/river-194) Zenodo nominiert `1966–2025`, das
  Parser-Fenster endet 2024-05-29; nominal vs Inhalt.
- **Blockade:** keine.
- **Braucht:** ersten/letzten Onset des manifestierten `mcpherron_onsets.bin` messen.

### `intermagnet_dbdt` — Resolver-Regel statt Rename (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** Freigabe von `tools/measure/src/bin/field_te_query.rs`
- **Lage:** (gemessen 2026-10-12 via Rat) Identität bleibt, Kadenz ist Auflösungs-Attribut.
  Defekt: `field_te_query.rs:1263-1341` prüft `abk_1m` auch gegen `sod_1h` (falscher Riss) und
  lässt den gröberen Zeugen fallen.
- **Blockade:** die Datei hält eine parallele Session.
- **Braucht:** nach Freigabe nach `station_code` gruppieren + den gröberen Zeugen als Riss
  daneben tragen.

### Quellen-Terms — not-admit/declined-Disposition (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** Rat/Operator-Wort
- **Lage:** (gemessen 2026-10-12) die 4 admissiblen sind im Register; die 3 not-admit:
  BGS-GIC/NERC-GMD ohne Block, OE-417 (`:11887-11893`) ohne `terms`. Es gibt **keine**
  Register-Direktive für ein not-admit/declined-Verdikt.
- **Blockade:** fehlende Register-Form.
- **Braucht:** eine gemessene Register-Form für die not-admit-Disposition (Rat/Operator)
  **oder** `descoped` mit Befund; OE-417 entscheiden.

### THEMIS-ASI — WHIT/GBAY `alt` pending (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** eine Punktmessung der Höhe
- **Lage:** (gemessen 2026-10-12) `src/archivar/themis_asi.rs::STATIONS` (24, `h = H + N`
  EGM2008); beide Compiler emittieren `station` + `on earth`. **Riss gegen Myceliums „24/24":**
  WHIT `H` pending, GBAY rissig → beide `alt_m: None`, namentlich übersprungen.
- **Blockade:** WHIT/GBAY `H` ungemessen.
- **Braucht:** WHIT/GBAY `H` aus einer Punktmessung oder `pending` lassen.

### Orphan-Prosa-Träger (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Register-/Verdikt-Pass
- **Lage:** (gemessen 2026-10-11/12) `survey-2026-10-10-worldwide-lidar-quellen.md` (51 Marker,
  trägerlos). **Aus future-235:** `docs/concepts/kybernetische-astrophysik.md` (1) ·
  `survey-2026-09-14-kapitulationen-pendings-inventur.md` (7) · `survey-2026-09-16-dead-sources-relevanz.md`
  (1) · `survey-2026-10-08-open-sources-delta.md` (12) · `survey-2026-10-09-domaenen.md` (4) ·
  `survey-2026-10-10-ephemeris-quellen.md` (5) · `survey-raetsel-bestand.md` (5).
  `exzellenz-konzept.md` (2) = die `pending`/`0 honored`-Definition selbst.
- **Blockade:** die Verdikte sind Mountain-Recht; die Marker sind zu sichten.
- **Braucht:** je Klasse eine Verdikt-/Träger-Zeile + Register-Einträge **oder** `descoped`.

### Re-Admission der lizenz-blockierten Kandidaten (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Verdikt-Pass
- **Lage:** (gemessen 2026-10-11, `survey-2026-10-09-redistribution-alternativen.md` §Dritter
  Lauf) ALMA → ESO Science Archive (CC BY 4.0); GRDC → GRDC-Caravan / USGS NWIS / GloFAS;
  NIST NCNR NSE / Zenodo 18306252. NC-admissibel: INTERMAGNET (CC BY-NC 4.0), GIRO DIDBase
  (CC BY-NC-SA 4.0). Risse: RouteViews BY-vs-BY-SA, ESO-CC-BY nur für ESO-ALMA-Records.
- **Blockade:** die Admission-Verdikte je Kandidat fehlen.
- **Braucht:** Admission je Kandidat (Verdikt); danach Mycelium-CDN.

### `api.sensor.community` — Trigger leer (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-11/12) kein Future-Wort; Ursprung `mycelium-158…162` (403) +
  `mountain-306`.
- **Blockade:** Arm/Zielzeile selbst zu benennen.
- **Braucht:** Arm/Zielzeile benennen oder `descoped` mit Befund.

### Rätsel-Datenlücken (future-235) — url/origin (Mycelium) (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** die `url`-Zeilen (Mycelium)
- **Lage:** (gemessen 2026-10-11 via future-235/240) 4 offen: Ⅶ Wurmloch (NAIF Voyager SPK) ·
  Ⅷ Dunkler Fluss (Zenodo UNCOVER `14281664`, JADES `7460116`) · Ⅻ Urknall (BK18) ·
  GIC-Mäntsälä (FMI NUR `space.fmi.fi/image`); Ⅺ Placebo kein offenes Einzeldataset.
- **Blockade:** `url`/`origin` (Mycelium).
- **Braucht:** je Lücke die `url`-Zeile (Mycelium) → `terms`/`format` (Mountain) → Parser.

### NIST NCNR NSE (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** die `url`-Route (Mycelium)
- **Lage:** (gemessen 2026-10-11) `https://ncnr.nist.gov/pub/ncnrdata/nse/` US-PD, monatliche
  Sets; `nist-ncnr-nse-cdn` grün; `trisp-keller` descoped.
- **Blockade:** `url`/`origin` (Mycelium).
- **Braucht:** `url`-Route (Mycelium) + `terms`/`field` (Mountain).

### Teil C — A5 offen (Galileo TDF) (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** die `url`-Route (Mycelium)
- **Lage:** (gemessen 2026-10-11) A5 `GO-SS-RSS-1-TDF-V1.0` (PDS PPI); A6 descoped.
- **Blockade:** `url`/`origin` (Mycelium).
- **Braucht:** A5 die TDF-Route als `url`-Zeile (Mycelium) + `terms`/`format` (Mountain).

### Band-Paar-Atom — CI-Idempotenz-Sperre (wartend)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein `spdf.gsfc.nasa.gov`-Asset-Refresh + `witness-odf-residuum`-Lauf
- **Lage:** (gemessen 2026-10-11) `pioneer-odf-cdn.yml:21-31` überspringt Compile+Upload, wenn
  das Release-Asset existiert → Witness `model: 0 samples modeled`.
- **Blockade:** Manifestations-Recht (Mycelium).
- **Braucht:** ODF-Workflows mit `--force`/clobber neu dispatchten; Witness-Lauf lesen.

### opentopography_coverage — per-Dataset-Lizenz (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** das Coverage-Manifest auf dem CDN
- **Lage:** (gemessen 2026-10-11) per-Dataset-Lizenz; Portal-Manifest ohne `terms`; `shard`-Emit
  nicht registriert.
- **Blockade:** Manifest-Asset (Mycelium).
- **Braucht:** Manifest (Mycelium) + Block (`url`/`origin`/`compiler`/`format coverage_manifest`).

### SWPC/RTSW-Kurzreihen — nicht alignbar (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-11, Lauf `38119733631` Job `matrix-vlies`) 126/210 Matrix-Zellen
  `alignment pending` — SWPC-Livereihen ~1 Tag, OMNI endet 2026-10-05.
- **Blockade:** fehlende statische überlappende Archivreihe.
- **Braucht:** Verdikt + `ttl` für eine statische SWPC/RTSW-Archivreihe **oder** descoped.

### Sechs Klima-Quellen / series-manifest (wartend)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** je der erste CDN-/Manifest-Lauf
- **Lage:** (gemessen 2026-10-11) ecad/aodn brauchen gewählte Station/URL; offen ist der erste
  `series-manifest`-Lauf je Familie.
- **Blockade:** kein Lauf / wissenschaftliche Wahl ohne Default.
- **Braucht:** die Läufe lesen (`ci_manage log <id>`) + sha256 je Familie.

### Eigene Ephemeride (PEP/Tudat) — blockiert (wartend)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Freigabe der Dateien in `src/mathematikerin` (`mod`/`receiver`/`tropo`)
- **Lage:** (gemessen 2026-10-11) Rat-Votum C; Research + UI konvergent. House-gate grün (E1).
- **Blockade:** fremd-aktiv gehaltenes Dateiset.
- **Braucht:** nach Freigabe die Rat-Schritte 1+2 als begrenzten Bau.

### Frame/Epoche · particle-cern · Exposom-Matrix · iEEG · Flyby-Kette · FMHY (eigen/wartend/termin/blockiert)
- **Status:** eigen/wartend/termin/blockiert | **Bindung:** eigen/termin:2026-11-01
- **Trigger:** Frame/Epoche: CRS-VLR-Messung; particle-cern: `subset`; Exposom: extern gedeckter
  Repräsentativpunkt; iEEG: neues Operator-Wort über 2026-10-06 — **nicht erneut vorlegen**;
  Flyby: ESOC-Recon-Release; FMHY: Riss 4/5
- **Lage:** (gemessen 2026-10-11 via 319-Register; unverändert)
- **Blockade:** s. je Punkt.
- **Braucht:** s. je Punkt; sonst der benannte Riss/Descope-Befund.

### Auftrag — Eigene Ephemeriden + Kulturerbe-Rettung (eigen) · Träger von `docs/auftrag/auftrag-eigene-ephemeriden-und-kulturerbe.md`
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** das Operator-Wort zum Start der Phase 0/1
- **Lage:** (gemessen 2026-10-12) der Auftrag liegt in `docs/auftrag/auftrag-eigene-ephemeriden-und-kulturerbe.md` (sha `c681a397…`); vom **Rat (fünf Stimmen)** gehalten — EIN Archivar, ZWEI Register (`obs`/`heritage`/`observable`/`asset`); `house_gate` → Konsistenz-Hülle (Hold-out-Gate); minimaler Operator mit `pending`; Riss 1: M10-**Bild** = Erbe, nicht Brennstoff; Riss 2: a-priori/Zustand + Asteroidenmassen reisen mit. Vom UI-Roster gehalten: `state/stimmen/2026-10-12_ui_eigene-ephemeriden-verdikt.md` (Qwen ✓, Claude ✓, Z.ai `pending`, Duck `pending`, Nemotron `pending`).
- **Blockade:** CDDIS/Earthdata-Konto (`blocked account`, Operator/Future); ISIS-Quelle unverifiziert (USGS/PD, nicht Zenodo).
- **Braucht:** Phase 0 (Forschungslage je Beobachtungsklasse + **voller** UI-Roster) → Phase 1 (ITRF2020/EOP + ein registrierter ODF), wie im Auftrag; Operator-Akte als Future-Queue.

## An mycelium

Origin: mountain-folge324 (2026-10-12).

- **THEMIS-ASI-Keogramm (UCalgary) — `terms free-open`.** Gemessen 2026-10-12:
  `data.phys.ucalgary.ca/data/how_to_cite.html` trägt „All data … is openly and freely
  available. We ask that users … acknowledge and cite" (HTTP 206); kein benanntes
  Lizenz-Token. Es gibt **noch keinen** Register-Block für das UCalgary-Keogramm (die
  `themis_asi`-Zeile `phi:20908-20912` ist Berkeley/anderer Netloc) — bitte `url`/`origin`
  für `data.phys.ucalgary.ca/…/THEMIS/asi/stream2` anlegen, dann setze ich `terms free-open`
  + `field`.
- **USGS-Mosaik — die wahre Größe:** `planetarymaps.usgs.gov/mosaic/Mercury_M1_M2_M3_M10_mosaic_global_2010.tif`
  → 302 auf S3, `Content-Range: bytes 0-0/1880562475` = **1 880 562 475 B (1,88 GB), gemessen
  2026-10-12**; der frühere 66-MB-Wert war ein Range-Teil als Vollgröße. sha256 über 1,88 GB
  jenseits des `--sniff`-Horizonts (`partial 342ef36a…`). Bitte die `url`-Zeile mit 1,88 GB.
- **Rätsel-/NSE-Identität:** `uncover_dr4_sps_catalog.fits` `format fits` + `terms CC-BY-4.0`
  **gesetzt** (Mountain, 2026-10-12); `jades_early_galaxies.tar` `terms CC-BY-4.0` gesetzt,
  `format tar_fits` **fehlt** (neuer Parser-Arm, s. offener Punkt); `bk18_bandpowers_20210607.txt`
  `terms CC-BY-4.0` gesetzt, `format text` + Zeilen-Direktiven offen. Compiler/Workflow je
  Mycelium, sobald die Identität steht.
- **Register-terms-Rot** (OGL/free-open/etalab): in `ee03f55fc` committet.

## Operator-Wort-Register

- **„sag ma siehst du eigentlich was schon gemacht wurde welche untersuchungen du und future bereits gemacht haben?"** | 2026-10-12 | Operator-Session (mountain) — Vorarbeit LESEN. Angewandt: die future-M10↔MDIS-Untersuchung + die eigene folge320/321/322 gesichtet; Vorarbeit unter `state/` per `sread` (glob sieht sie nicht).
- **„1 empfehlung 2 warte ab 3 ja auf jeden fall"** | 2026-10-12 | Operator-Session (mountain) — Plan bestätigt: M1+R1–R3 parallel, M10-EDR-Bau an M1 gebunden, Gate-Fixture ja.
- **„5 wähle das richtige"** | 2026-10-12 | Operator-Session (mountain) — die richtige Fixture-Form. Angewandt: handover-gescannte Regel `prior-work-unread` statt code-only Vocab-Zeile.
- **„Du kannst. Führe den … Plan aus … Delegiere an die Taucher …"** | 2026-10-12 | Operator-Session (mountain) — session-weiter Consent (Delegation), nicht das Commit-Wort.

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort („mach das ab jetzt automatisch",
2026-10-07) trägt Commit und Push. In diesem Atom committet: `phi/sources.φ`,
`src/gate/commit_gate.rs`, `src/gate/commit_gate_vocab.json`,
`tools/measure/src/bin/m10_expected_diameter.rs`, `docs/handover/*`.
