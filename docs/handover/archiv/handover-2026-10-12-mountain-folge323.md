<!--
  title: Handover — Mountain-Folge 323 (2026-10-12)
  session: Mountain-Linie in einem Pass — heal the ci-gate license-vocabulary red (3 terms), build the MCON declared_fields (5 components with read-site names + units), fold the addressed future-240/235 blocks (M10↔MDIS registration, mariner-rst raw-band routes, DSC 0119, licence re-admission)
  class: handover
  date: 2026-10-12
  sha256: c2d8b72bfa0aa8ec069627b8edee10b70c50a5671349d43f8419f7b3b36af867
  status: live
-->
# Handover — Mountain-Folge 323 (2026-10-12)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/standing-pass.md`). Diese
Session konsumierte `handover-2026-10-12-mountain-folge322.md` (→ `archiv/`).

## Burn: open 0.00 · close 0.13 · cap 0.15 · reason: Linien-Session (Mountain) + ein `general`-Taucher, deepseek-flash, kein pro/max (lokal nur flash); `session_burn`-Linien-Klasse ≈ $0.08–0.13 (keine saubere Einzelsession-Isolation).

## In diesem Atom erledigt (git trägt es; nicht erneut vorlegen)

- **ci-gate-Rot geheilt — die Ursache gemessen.** Aus dem Log `38209824621`
  (SHA `bc264c4c…`) und `38209451879`: der Job **clippy** fiel an `odf.rs` (bereits in
  `a0d2c04e2` geheilt), der Job **register** an `license_census -- --fail` mit
  `terms-vocab 3 violation(s)` an den Register-Blöcken `phi/sources.φ:1528`/`:11144`/`:11152`.
  Selbst gemessen (`cargo run -p omegaflow-register --bin license_census`): `blocks 2814 |
  terms 2758 | terms-vocab 3 violation(s) | redistribution 0`. Geheilt: `OGL-3.0` →
  `OGL-UK-3.0` (`:1529`), `open-by-attribution` → `free-open` (`:11145`, swisstopo nennt
  seine Bedingungen selbst „Terms of use for free geodata and geoservices (OGD)"), und
  `etalab-2.0` (Etalab Open Licence 2.0, echte benannte Lizenz) **in die geschlossene
  Kanonliste** `tools/register/src/bin/license_census.rs::TERMS` aufgenommen. Ergebnis:
  `terms-vocab 0 violation(s)`, `cargo check` 0/0. Die drei Werte stammen aus `8d0b4c633`
  (mountain 313).
- **MCON `declared_fields` gebaut** (schließt folge322:89-94). `src/archivar/mcon_event.rs`
  trägt jetzt `COLUMNS` (5 Komponenten mit Lese-Ort-Namen + Einheiten: `mcon_sml_nt`/`nT`,
  `mcon_amplitude_nt`/`nT`, `mcon_bay_decrease_nt`/`nT`, `mcon_bay_duration_min`/`min`,
  `mcon_bay_width_min`/`min`) und `declared_fields(tau)` nach dem Muster
  `carisma_mag`/`substorm` (force `em`, kernel aus `kernel_id_for_force`). `extract.rs`
  verdrahtet beide Arme (`declared_fields(tau)` + `series_component_name`). `convert_to_si`
  lernt `"min" => *60` (SI-Sekunde), konsistent zu `"h"`. Test
  `declared_fields_carry_the_read_site_names_and_units`. `cargo check` 0/0.

## Offen (aufgeschlüsselt)

### `solar_f107_flux_sfu` — Zeitbasis-Defekt (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Bau-Pass
- **Lage:** (gemessen 2026-10-12, `general`) der SWPC-Textarm liefert **TDB/J2000**
  (`extract.rs:6506`, `lsk.unix_to_tdb`), das Penticton-`f107`-Asset **rohe Unix-Sekunden**
  (`src/archivar/f107.rs:22`, `day*86400`); feste Verschiebung `946728000 − 69.184 ≈
  9.4673e8 s ≈ 30,0 a` → SWPC-2026 paart mit Penticton-1996 → r≈0.02. Zwei Defekte:
  (i) `f107::parse_series` wendet `unix_to_tdb` nicht an; (ii) `series_epoch_of`
  (`extract.rs:6506-6539`) wählt den ersten `"date"`-Key (`avg_begin_date` kann `time_tag`
  verdrängen).
- **Blockade:** beide Arme brauchen dieselbe Epoch-Basis (Signatur-/Aufrufer-Änderung).
- **Braucht:** beide Arme auf TDB bringen; dann den `f107`-Join erneut messen.

### `MCON`-Spanne — Inhaltsmessung (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-12 via mycelium-318/river-194) Zenodo nominiert `1966–2025`,
  das Parser-Fenster endet 2024-05-29; nominal vs Inhalt.
- **Blockade:** keine (die BPA-Überschneidung 2026 ist nicht berührt).
- **Braucht:** ersten/letzten Onset des manifestierten `mcpherron_onsets.bin` messen;
  Registerform danach.

### `intermagnet_dbdt` — Resolver-Regel statt Rename (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** Freigabe von `tools/measure/src/bin/field_te_query.rs` (Datei-Kollision mit
  Rivers measure-Atom)
- **Lage:** (gemessen 2026-10-12 via Rat) Rat-Verdikt **Nein** zum Rename: Identität bleibt,
  Kadenz ist Auflösungs-Attribut. Regel im Kern vorhanden (`AGGREGATION_IDENTITY_CORR =
  0.99`, feinste `median_dt` als Zeuge, Verweigerung = Riss). Defekt:
  `field_te_query.rs:1263-1341` prüft `abk_1m` auch gegen `sod_1h` (falscher Riss) und lässt
  den gröberen Zeugen fallen.
- **Blockade:** die Datei hält eine parallele Session (measure-Atom).
- **Braucht:** nach Freigabe nach `station_code` gruppieren + den gröberen Zeugen als Riss
  **daneben tragen** statt fallenlassen; Register/Wire unangetastet.

### Quellen-Terms — not-admit/declined-Disposition (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** Rat/Operator-Wort
- **Lage:** (gemessen 2026-10-12) die 4 admissiblen sind im Register (PCN `CC-BY-4.0`, CSES
  `CC0-1.0`, THEMIS-ASI `free-open`, McPherron `CC-BY-4.0`); `mariner4_odg`-terms gesetzt.
  Die 3 not-admit: BGS-GIC/NERC-GMD haben **keinen** Block, OE-417 hat `oe417` (`:11887-11893`)
  ohne `terms`.
- **Blockade:** es gibt **keine** Register-Direktive für ein not-admit/declined-Verdikt —
  `parse.rs:265` speichert `terms` als freien String, `commit_gate` kennt keine Disposition.
- **Braucht:** eine gemessene Register-Form für die not-admit-Disposition (Rat/Operator)
  **oder** `descoped` mit Befund; OE-417 entscheiden.

### Mariner 4 `mariner4_odg` — CDN-Lauf offen (wartend)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der CDN-Lauf `mariner4-odg-cdn` (Mycelium)
- **Lage:** (gemessen 2026-10-11/12) Parser + Compiler gebaut, lokal verifiziert (528 → 10 600,
  529 → 1025 Observablen, t 1964-11-28…1967-12-07); Register-Block `phi/sources.φ:31391-31395`
  trägt url/format/origin/compiler + `terms CC0-1.0`. `ci_manage status`: `38218606564
  mariner4-odg-cdn completed success` (2026-10-12T07:39Z).
- **Blockade:** der Asset-`sha256` steht noch nicht im Register.
- **Braucht:** den Asset-sha messen und in den Block eintragen.

### Mariner 10 Imaging-EDR — Detektor offen (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Bau-Pass
- **Lage:** (gemessen 2026-10-11) `tools/utils/src/bin/m10_edr_image.rs` (832×700-gray8 → PNG
  + 2–98%-Stretch + Histogramm). Rat (5 Stimmen) + UI + Research konvergent: ein Kontrast-Stretch
  ist **kein** Gültigkeitstest; der Leser ist ein **deterministischer Detektor**
  (Hintergrund-Schätzung → Limbus-/Crescent-Maske → Ellipsen-/Limb-Fit, Zentroid + Residuum).
  Qwen nennt als wahrscheinlichsten Grund: ERT 17:34:07 = ~25–30 min **nach** Closest Approach
  (17:01 UT, 5768 km) → Venus ~**15° Winkeldurchmesser ≫ FOV** (0,42°/0,98°) → wahrscheinlich
  kein Limbus im Bild.
- **Blockade:** kein vermessenes Ziel/Referenzset.
- **Braucht:** **erwarteter Winkeldurchmesser je Frame** aus Ephemeride + FOV/Plattform-Offset,
  dann **radiusfixierter Strahlen-Fit** (PSF/erf-Radialprofil je Azimut, Radius fest aus
  Range+Brennweite, Huber/Tukey); ERT→Aufnahmezeit prüfen; **Roll** für Merkur über das
  ASU-Kontrollnetz (`ser.im-ldi.com/M10/M10_ISIS/camera_angles.txt`, sha `0a65df59…`).

### Bild ↔ Doppler — Optical-Navigation-Kreuz-Constraint (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Bau-Pass (erst mit einem Frame mit gemessenem Zentroid)
- **Lage:** (gemessen 2026-10-11 via Rat + Research) Bildwinkel (Line-of-Sight) + Doppler
  (Range-Rate) binden gemeinsam die deklarierte `Receiver`-Worldline per Least Squares;
  Mariner 10 **nicht** mit Onboard-OpNav → post-hoc, so benannt. **Keine** Doppler-Kalibrierung.
- **Blockade:** die Bildserie trägt noch kein gemessenes Ziel (M10-Punkt darüber).
- **Braucht:** Eintritt als **abgeleiteter gemeinsamer Term im ω()-Loop** über die geteilte
  Receiver-Worldline (kein Sample-Slot); `Riss`, wo Bild und Doppler sich nicht überlappen.

### Mariner-6/7-Tracking `mariner_rst` (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-11 via grind-flash) Parser gebaut + auf M6 `DD005408` (55 900
  Observablen) / M7 `DD005410` (38 572) verifiziert. Offen: `DD005407`/`DD005409` sind
  strukturell andere (Summary-)Tapes; die Observable-Skalierung einzelner Datentypen ist am
  Original gegenzuprüfen (Frequenz roh 1.08e12, /2⁹ ≈ 2114 MHz).
- **Blockade:** die Zusatz-Tapes brauchen eine eigene Messung.
- **Braucht:** die Summary-Tapes messen oder `descoped`; Observable-Skalierung gegen DSC
  0073/Spec prüfen. **Neu aus future-240:** DSC 0119 (SPDF digitalisiert) trägt die
  M4-Feldtabelle + trägt den `mariner_rst`-Parser-Arm — siehe offener Punkt unten.

### DSC 0119 / M4-Feldtabelle — Parser-Arm (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** die `url`-Zeile (Mycelium)
- **Lage:** (gemessen 2026-10-11/12 via future-238, `--pdf-image` + vision) DSC 0119
  (`…/mariner4_celestial-mechanics_64-077a-09a_DSC_0119.pdf`, HTTP 200, 1 380 249 B, sha256
  `ec3c41ebbb082287437fe6ff9b9ba91a7fd9474311b274d8cbd975ddd5ce55b0`) trägt `SECTION II`
  ODG-Data-File (Master-Label 9 Wörter BCD + Check-Sum, Station-Summary 201 Wörter, Datentypen
  1–15) + `APPENDIX I` ODP-Logical-Record (Wort 0 #Wörter · 1–2 doppeltgenaue Zeit s past
  1950.0 · 5 Code Word · 7 Weight Code Word) + `Fig. 2`. `TX OTD900-1`/`TR32-1106` sind
  **nicht online** (NTRS/IA/Web = 0).
- **Blockade:** `url`/`origin` (Mycelium).
- **Braucht:** `url`-Zeile für DSC 0119 (Mycelium) → `terms`/`format` (Mountain) →
  `mariner_rst`-Parser-Arm mit der gemessenen Feldtabelle.

### THEMIS-ASI — WHIT/GBAY `alt` pending (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** eine Punktmessung der Höhe
- **Lage:** (gemessen 2026-10-12) Stationstabelle `src/archivar/themis_asi.rs::STATIONS` (24,
  `h = H + N` EGM2008); beide Keogramm-Compiler emittieren `station <CODE>` + `on earth
  <lat> <lon> <alt>` (MIRACLE-Form). **Riss gegen Myceliums „24/24":** WHIT `H` `pending`
  (keine Punktmessung), GBAY `H` rissig (12 Ort / 48.8 Flugplatz) → beide `alt_m: None` →
  der Compiler überspringt sie namentlich (0 honored).
- **Blockade:** WHIT/GBAY `H` ungemessen.
- **Braucht:** WHIT/GBAY `H` aus einer Punktmessung oder `pending` lassen.

### Orphan-Prosa-Träger (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Register-/Verdikt-Pass
- **Lage:** (gemessen 2026-10-11 via `register_lookup --orphan-docs`; 2026-10-12 = 0 im
  Stehenden Pass) `docs/surveys/survey-2026-10-10-worldwide-lidar-quellen.md` trug 51 offene
  Marker und keinen Live-Handover-Träger. **Neu aus future-235:** `docs/concepts/
  kybernetische-astrophysik.md` (1) · `survey-2026-09-14-kapitulationen-pendings-inventur.md`
  (7) · `survey-2026-09-16-dead-sources-relevanz.md` (1) · `survey-2026-10-08-open-sources-
  delta.md` (12) · `survey-2026-10-09-domaenen.md` (4) · `survey-2026-10-10-ephemeris-quellen.md`
  (5) · `survey-raetsel-bestand.md` (5). `exzellenz-konzept.md` (2) sind die `pending`/`0
  honored`-Definition selbst — kein offener Schritt.
- **Blockade:** die Verdikte sind Mountain-Recht; die Marker sind zu sichten.
- **Braucht:** je Klasse eine Verdikt-/Träger-Zeile + Register-Einträge in `phi/sources.φ`,
  oder `descoped` mit Befund.

### Re-Admission der lizenz-blockierten Kandidaten (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Verdikt-Pass
- **Lage:** (gemessen 2026-10-11, `survey-2026-10-09-redistribution-alternativen.md` §Dritter
  Lauf) legale Routen: ALMA → ESO Science Archive (CC BY 4.0); GRDC → GRDC-Caravan /
  USGS NWIS / GloFAS; RIPE RIS → RouteViews (CC-BY-4.0); IMCCE INPOP/NOE-4 → JPL DE440/441 /
  **INTERMAGNET (CC BY-NC 4.0)**, **GIRO DIDBase (CC BY-NC-SA 4.0)**. Risse: RouteViews
  BY-vs-BY-SA, ESO-CC-BY nur für ESO-ALMA-Records.
- **Blockade:** die Admission-Verdikte je Kandidat fehlen.
- **Braucht:** Admission je Kandidat (Verdikt); danach Mycelium-CDN.

### `api.sensor.community` — Trigger leer (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-11/12) kein Future-Wort; Ursprung `mycelium-158…162` (403) +
  `mountain-306`; `archive_search 'sensor.community' --root state` = 0 Future-Handover-Treffer.
- **Blockade:** Arm/Zielzeile selbst zu benennen.
- **Braucht:** Arm/Zielzeile benennen oder `descoped` mit Befund.

### `bpa_gic` (descoped)
- **Status:** descoped | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-12 via mycelium-318) kein historisches Archiv (gemessene Null;
  nur FOIA). `terms unknown` bleibt.
- **Blockade:** keine.
- **Braucht:** Befund `(gemessen 2026-10-12 via mycelium-318)` — keine Handlung.

### Rätsel-Datenlücken (future-235) — url/origin (Mycelium) (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** die `url`-Zeilen (Mycelium)
- **Lage:** (gemessen 2026-10-11 via future-235/240, alle Routen `--verdict`-verifiziert)
  4 offen erreichbar: Ⅶ Wurmloch (NAIF Voyager SPK `…/naif/VOYAGER/kernels/spk/`) · Ⅷ Dunkler
  Fluss (Zenodo UNCOVER `14281664`, JADES `7460116`) · Ⅻ Urknall (BK18
  `bicepkeck.org/bk18_2021_release.html`) · GIC-Mäntsälä (FMI NUR
  `space.fmi.fi/image/www/data_download_month.php?month=YYYYMM&stations=NUR`); Ⅺ Placebo kein
  offenes Einzeldataset (kein Subjekt-Join über die 3 Zwei-Kanal-Quellen).
- **Blockade:** `url`/`origin` (Mycelium).
- **Braucht:** je Lücke die `url`-Zeile (Mycelium) → `terms`/`format` (Mountain) → Parser.

### NIST NCNR NSE (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** die `url`-Route (Mycelium)
- **Lage:** (gemessen 2026-10-11) `https://ncnr.nist.gov/pub/ncnrdata/nse/` US-PD, monatliche
  Sets; `nist-ncnr-nse-cdn` grün (`eee1e943…`); `trisp-keller` descoped.
- **Blockade:** `url`/`origin` (Mycelium).
- **Braucht:** `url`-Route registrieren (Mycelium) + `terms`/`field` (Mountain).

### Teil C — A5 offen (Galileo TDF), A6 descoped (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** die `url`-Route (Mycelium)
- **Lage:** (gemessen 2026-10-11) A5 `GO-SS-RSS-1-TDF-V1.0` (PDS PPI); A6 descoped.
- **Blockade:** `url`/`origin` (Mycelium).
- **Braucht:** A5 die TDF-Route als `url`-Zeile (Mycelium) + `terms`/`format` (Mountain),
  oder `descoped` mit Befund.

### Band-Paar-Atom — CI-Idempotenz-Sperre (wartend)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein `spdf.gsfc.nasa.gov`-Asset-Refresh + `witness-odf-residuum`-Lauf
- **Lage:** (gemessen 2026-10-11) `pioneer-odf-cdn.yml:21-31` überspringt Compile+Upload, wenn
  das Release-Asset existiert (zuletzt 2026-09-18) → Witness `model: 0 samples modeled`.
- **Blockade:** Manifestations-Recht (Mycelium).
- **Braucht:** ODF-Workflows mit `--force`/clobber neu dispatchten; danach Witness-Lauf lesen.

### opentopography_coverage — per-Dataset-Lizenz (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** das Coverage-Manifest auf dem CDN
- **Lage:** (gemessen 2026-10-11) per-Dataset-Lizenz; Portal-Manifest ohne `terms` (0 honored);
  `shard`-Emit nicht registriert.
- **Blockade:** Manifest-Asset (Mycelium).
- **Braucht:** Manifest (Mycelium) + Block (`url`/`origin`/`compiler`/`format coverage_manifest`,
  keine `terms`).

### SWPC/RTSW-Kurzreihen — nicht alignbar (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-11, Lauf `38119733631` Job `matrix-vlies`) 126/210 Matrix-Zellen
  `alignment pending` — SWPC-Livereihen ~1 Tag, OMNI endet 2026-10-05; Kanäle auf CDAWeb ACE
  (AC_H2) retargetet.
- **Blockade:** fehlende statische überlappende Archivreihe.
- **Braucht:** Verdikt + `ttl` für eine statische SWPC/RTSW-Archivreihe **oder** descoped mit
  Befund.

### Sechs Klima-Quellen / series-manifest (wartend)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** je der erste CDN-/Manifest-Lauf
- **Lage:** (gemessen 2026-10-11) ecad/aodn brauchen gewählte Station/URL; die sechs
  Klima-Workflows tragen je einen `series-manifest`-Job; offen ist der erste Lauf je Familie.
- **Blockade:** kein Lauf / wissenschaftliche Wahl ohne Default.
- **Braucht:** die Läufe lesen (`ci_manage log <id>`) + sha256 je Familie.

### Eigene Ephemeride (PEP/Tudat) — Step 4 blockiert (wartend)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Freigabe der Dateien in `src/mathematikerin` (`mod`/`receiver`/`tropo`)
- **Lage:** (gemessen 2026-10-11) Rat-Votum C; Research + UI konvergent.
- **Blockade:** fremd-aktiv gehaltenes Dateiset.
- **Braucht:** nach Freigabe die drei Rat-Schritte 1+2 als begrenzten Bau.

### Frame/Epoche · particle-cern · Exposom-Matrix · iEEG · Flyby-Kette · FMHY (eigen/wartend/termin/blockiert)
- **Status:** eigen/wartend/termin/blockiert | **Bindung:** eigen/termin:2026-11-01
- **Trigger:** Frame/Epoche: CRS-VLR-Messung; particle-cern: `subset`; Exposom: extern gedeckter
  Repräsentativpunkt; iEEG: neues Operator-Wort über 2026-10-06 — **nicht erneut vorlegen**;
  Flyby: ESOC-Recon-Release; FMHY: Riss 4/5
- **Lage:** (gemessen 2026-10-11 via 319-Register; unverändert)
- **Blockade:** s. je Punkt.
- **Braucht:** s. je Punkt; sonst der benannte Riss/Descope-Befund.

## An mycelium

Origin: mountain-folge323 (2026-10-12). (Aus dem gefalteten `## An mountain`-Block
future-240/235 — die `url`/`origin`-Zeilen sind Mycelium-Recht.)

- **M10↔MDIS (Sonke & Robinson 2026) — die 8 Artefakte als `url`-Zeilen.** Paper
  `10.3847/PSJ/ae32f5` (CC BY 4.0, gold-OA); Methoden-/Tabellen-Artefakte gemessen
  (HTTP + sha256; lokal unter `state/research/m10-mdis-sonke-robinson/` + `MANIFEST.md`):
  Table 1 `…/suppdata/psjae32f5t1_ascii.txt?doi=10.3847/PSJ/ae32f5` (`78fd71f6…`) · Table 2
  `…t2…` (`f9b94375…`) · Table 3 `…t3…` (`ad3c8471…`) · Table A1 `…/revision1/psjae32f5t4_mrt.txt`
  (`1e8f7dd4…`, 67 449 B) · ASU `ser.im-ldi.com/M10/M10_ISIS/camera_angles.txt` (`0a65df59…`,
  194 640 B) + ISIS-Tars ENC1/2/3/ALL_RES · USGS-Kombi-Mosaik
  `planetarymaps.usgs.gov/mosaic/Mercury_M1_M2_M3_M10_mosaic_global_2010.tif` (`1a90a8ff…`,
  66 414 088 B, US-PD; Label-Dateien ISIS3/PDS3/`.jgw`) · LPSC 2024 #2663
  `hou.usra.edu/meetings/lpsc2024/pdf/2663.pdf` (`40085b52…`) · PDS-Imaging
  `pds-imaging.jpl.nasa.gov/volumes/mariner10.html` (EDR-Rohvolumes). Kein deponiertes cnet.
  **Nur URL + sha256 registrieren — das 66-MB-Mosaik nicht laden.**
- **Mariner-`mariner_rst`-Rohband-Routen** (NASA-PD): SPDF Mariner 6
  `…/mariner6/celestial_mechanics/2_way_doppler_radio_tracking/` (NSSD1311) + Mariner 7
  `…/mariner7/…` (NSSD1310) + Mariner 4 `…/mariner4/celestial_mechanics/` (NSSD1312); ASU
  `ser.im-ldi.com/M10/MVM/` (`MVM_0001–0014` `.tgz`). `url` + `origin` setzen.
- **DSC 0119 (M4-Feldtabelle)**: `…/renamed_dataset_catalogs/
  mariner4_celestial-mechanics_64-077a-09a_DSC_0119.pdf` (`ec3c41eb…`) als `url`.
- **Rätsel-Routen** (Ⅶ/Ⅷ/Ⅻ/GIC-NUR): siehe offener Punkt oben; je `url`/`origin`.
- **Mariner-Rohband-CDN:** die Parser-Arme (`mariner_rst` + `mariner4_odg`) sind gebaut;
  `mariner4-odg-cdn` ist grün. Bitte `mariner-rst-cdn` dispatchten (steht queued) und die
  `url`-Zeilen für `spdf.gsfc.nasa.gov/pub/data/mariner/{mariner4,6,7}/…` (NASA-PD) + den
  M10-Imaging-Spiegel `ser.im-ldi.com/M10/MVM/mvm_0001.tgz` (144 349 050 B, sha256
  `44fcfd39…4888ef7`) registrieren.

## Operator-Wort-Register

Keine neuen Worte in diesem Atom.

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort („mach das ab jetzt automatisch",
2026-10-07) trägt Commit und Push. In diesem Atom committet: `phi/sources.φ`,
`tools/register/src/bin/license_census.rs`, `src/archivar/mcon_event.rs`,
`src/archivar/extract.rs`, `src/archivar/units.rs`, `docs/handover/*`.
