<!--
  title: Handover — Mountain-Folge 321 (2026-10-11)
  session: Mountain-Linie in einem Pass — NSSDC-Arm gebaut (NASA NSSDCA Master Catalog + China NSSDC/VSSO coreMetadata; cargo check 0/0, live verifiziert) + Mariner 4 Rohband-Framing/6-bit-Packing gemessen (Zeit→1964) + Mariner 10 Imaging-EDR end-to-end gegen die realen Bytes verifiziert
  class: handover
  date: 2026-10-11
  sha256: 9f41dea1360f99a4487e43f8ca751a266e6c27ca8a51d5f79ad33011b5782bab
  status: live
-->
# Handover — Mountain-Folge 321 (2026-10-11)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/standing-pass.md`). Diese
Session konsumierte `handover-2026-10-11-mountain-folge320.md` (→ `archiv/`).

## Burn: open 0.00 · close 0.13 · cap 0.50 · reason: Linien-Session + 2 grind-flash-Taucher (M4-Binärformat, M10-EDR-Verifikation), deepseek-flash, kein pro/max.

## Offen (aufgeschlüsselt)

### NSSDC-Arm — gebaut; die Auth-Route ist gemessen (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-11, live) `archive_search --nssdca <query>` (NASA NSSDCA Master Catalog, `nssdc.gsfc.nasa.gov/nmc/dataset/query`, öffentlich) und `archive_search --nssdc <query>` (China NSSDC/VSSO `coreMetadataList`, `www.nssdc.ac.cn`, öffentlich) sind gebaut (`tools/utils/src/bin/archive_search/nssdc.rs`, `cargo build -p omegaflow-utils --bin archive_search` clean, live gegen PSCM-00009/„Mariner 10"/CSES verifiziert). **Beide Kataloge sind öffentlich; kein Login.** Die NASA-NSSDCA-Datenausgabe läuft über CRUSO (E-Mail/Antrag, Operator-Hand); die China-NSSDC-Ausgabe über CAS-SSO **+ Captcha + Antragsformular/Unterschrift** (`nssdc_zh/js/common.js`, `vsso.js`) — **nicht** autonom. Der Arm hob die rohen NSSDCA-Holdings des M10-EDR: `PSPA-00108 SDDPT Mariner 10 Raw EDR Images in VICAR Format` · `PSPG-00230 SDDPT Mariner 10 Master Data Records`.
- **Blockade:** keine für den Arm; die Ausgabe-Route ist kein Maschinenakt.
- **Braucht:** `NSSDC_USER`/`NSSDC_PASS` gehören nach Messung zum **China**-NSSDC (CAS/Captcha-gated, `state/river/gic-sources-broad-2026-10-11.md:57`), **nicht** zum NASA-NSSDCA — der Riss ist benannt; für CSES der Operator-Antrag (Future), sonst `descoped`.

### Mariner 4 `mariner4_odg` — gebaut + lokal verifiziert (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-11) **Parser gebaut:** `src/archivar/mariner4_odg.rs` (MAGIC `M4OD`), `tools/harvest/src/bin/mariner4_odg_compiler.rs` (4-per-3-Zeichen → 36-bit-Wörter), `extract.rs`-Arm; `cargo check` **0/0**. **Lokaler Lauf:** 528 → **10 600** Observablen (424 Daten-Records × 25 logische), 0 Records ohne Code Word, t **1964-12-05…1967-12-07**; 529 → **1025** (41 × 25), t **1964-11-28 15:35:41…1964-12-05** — beide decken die JPL-Tape-Spannen (`A55822`/`A55823`) exakt. Format-Transkription in `state/mountain/mariner4_dsc0119_format.md`.
- **Blockade:** keine (Parserseite); die **Observable-Einheit/Skalierung** (datatype 8 = Two-way-Doppler CPS) ist beschrieben, die a-priori-Gewichtung nicht angewandt.
- **Braucht:** `url`/`origin`/`compiler`/`format mariner4_odg`-Zeilen (Mycelium) + CDN-Lauf; `terms` NASA-PD (Mountain).

### Mariner 4 — Code-Word-Achse + Rest-Riss (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-11 via `vision`-Hochauflösung + NTRS-Volltextsuchen) DSC 0119 II.1.1 Bit-Achse: `b`(1) + **11 Datentyp-Bits** + `n×5` + `o` + `a` + `x×4` (tx-ID) + `r×4` (rx-ID) + `t×8` (Doppler-Mittelungszeit); Datentyp i ↔ LSB-Bit `35-i`; 528/529 tragen durchweg Bit 27 → **Datentyp 8 = Two-way-Doppler (CPS)**. `TM GTB900-1` (Format-Spec) und `TR32-1106` sind **nicht online** — auch unter dem korrigierten Namen `GTB900-1` (NTRS/IA/Serper leer); die NTRS-Mariner-Mars-1964-Project-Reports (`19660019455`, `19660024428`, `19660026657`, `19670009296`, `19660029256`, `19670005092`) tragen **kein** ODP-Format (Volltextsuche `logical record`/`Range Units`/`Master File`/`Weight Code` → 0).
- **Blockade:** keine.
- **Braucht:** nichts — der Riss ist geschlossen; der Eintrag dient als Beleg.

### Mariner 10 Imaging-EDR — EDR→Bild + Stretch gebaut; Detektor offen (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Bau-Pass
- **Lage:** (gemessen 2026-10-11) `mvm_0001.tgz` = 144 349 050 B, sha256 `44fcfd39…4888ef7`; 1367 Member, **677** `IMAGES/`-Dateien à 702 768 B; EBCDIC-Label (cp037) (`ERT YR=74 DAY=036 GMT=17/34/07` = **1974-02-05 17:34:07 UTC**); Records 1–700 = Pixelzeilen. **Gebaut:** `tools/utils/src/bin/m10_edr_image.rs` — echte 832×700-gray8-Raster je Bild → PNG (std-only Encoder) + **2–98%-Kontrast-Stretch + Histogramm-Statistik**; Rohbild-Pfad verifiziert. **Rat (5 Stimmen) + UI-Seats (Duck.ai GPT-6 Luna, Gemini 3.1 Pro, Claude Sonnet 5.5 Extra hoch, Qwen3.7-Plus, Qwen3.8 2.4T via tryingopen; GLM-5.3 Max `pending`) + Research konvergieren:** ein Kontrast-Stretch **ist kein Gültigkeitstest** auf ein Planetenziel; der richtige Leser ist ein **deterministischer Detektor** (Hintergrund-Schätzung → Limbus-/Crescent-Maske → Ellipsen-/Limb-Fit, Zentroid + Residuum), **nicht** ein Vision-LLM; Vorverarbeitung = Dark-Frame/Flat-Field/Reseau (`mar10cal`/`mar10clean` USGS-ISIS), log/gamma nur zur Sichtprüfung. Die ersten drei MVE_001-Frames bleiben bei diesem SNR **ohne auflösbares Ziel** (`absent`, kein Fehlzustand). Qwen3.8 2.4T nennt den wahrscheinlichsten Grund (noch zu prüfen gegen die Ephemeride): ERT 17:34:07 = ~25–30 min **nach** Closest Approach (17:01 UT, 5768 km) → Venus ~**15° Winkeldurchmesser ≫ FOV** (0,42°/0,98°) bzw. Nachtseiten-Crescent — erst den erwarteten Winkeldurchmesser je Frame vs. FOV+Plattform-Offset rechnen, **dann** Limbus suchen. Voller `.tgz`-Lauf über Repo-`gunzip` **lokal zu langsam** (Timeout 300 s auf 144 MB) → CI.
- **Blockade:** kein vermessenes Ziel/Referenzset → der Erkennungspfad ist `pending`, nicht bewertbar.
- **Braucht:** **Subpixel-Limbus-Fit** (PSF/erf-Radialprofil je Azimut, **Radius fest** aus Range+Brennweite, robuster Kreisfit Huber/Tukey, Kovarianz; **kein** Intensitäts-Schwerpunkt), Reihenfolge Dark→Flat→Linearisierung→Reseau(-Gewichtsmaske); **ERT→Aufnahmezeit** (Lichtlaufzeit + Frame/Readout) gegen Doku prüfen; **Roll** — für Merkur über **moderne Oberflächen-Kontrollnetze** statt Telemetrie. **Downloadbar gemessen:** ASU „Mercury Mariner 10 Image Project" `ser.im-ldi.com/M10/M10_ISIS/` — `camera_angles.txt` (194 640 B, sha256 `0a65df59…`, verfeinerte Kamera-Winkel + Sondenvektor je Bild) + ISIS-Tars `ENC1/ENC2/ENC3/ALL_RES.tar` (Robinson et al. 1999: 10 704 Messungen / 2 306 Punkte / 811 Bilder, 3 Encounters, σ 0.00844 mm = 0,6 px; Brennweiten A 1493.6 / B 1500.1 mm) · USGS-Astropedia `mercury_messenger_mdis_nac_global_mosaic_1km` (MDIS+M10, US-PD) + 665 m DEM. **Die exakte M10↔MDIS-Verknüpfung** (Sonke & Robinson; MEXAG-2024-Eposter `6006`, `10.3847/PSJ/ae32f5`) ist nur Paper → `pending` (kein File). **Venus** (5.2.1974) hat keine Oberflächenmerkmale (Wolkendecke); Labeled-Fixture-Set + FOM + Recorded-Winner (`state/benchmark/`, Mycelium); Repo-`gunzip` beschleunigen oder CI-only.

### Bild ↔ Doppler — Optical-Navigation-Kreuz-Constraint (eigen)
- **Status:** eigen | **Bindung:** eigen | **Trigger:** der nächste Bau-Pass
- **Lage:** (gemessen 2026-10-11 via Rat + Research) die Physik ist **valide, aber kein Sample**: Bildwinkel (Line-of-Sight) + Doppler (Range-Rate) binden gemeinsam die deklarierte `Receiver`-Worldline per Least Squares; Mariner 10 wurde **nicht** mit Onboard-OpNav geflogen → **post-hoc**, so benannt. Es ist **keine** Doppler-Kalibrierung (Kategorienfehler): Doppler stammt aus Radio-Tracking, die Bilder sind eine unabhängige **Bearing**-Beobachtung. Literatur: OpNav ab Mariner 6/7 (1969), M9/Viking; kombinierte Radio+Optik ~Ordnung besser (`10.1007/s40295-020-00244-x`); **kein** M10-spezifisches kombiniertes Paper (Research: `absent`).
- **Blockade:** die Bildserie trägt noch kein gemessenes Ziel (M10-Punkt darüber).
- **Braucht:** Eintritt als **abgeleiteter gemeinsamer Term im ω()-Loop** über die geteilte Receiver-Worldline (kein Sample-Slot); `Riss`, wo Bild und Doppler sich nicht überlappen. Erst sinnvoll, wenn ein Frame ein gemessenes Zentroid trägt.

### Mariner-6/7-Tracking (`mariner_rst`) — Zusatz-Tapes + Observable-Skala (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-11 via grind-flash) Parser gebaut + auf M6 `DD005408` (55 900 Observablen 1969-03-01…08-29) / M7 `DD005410` (38 572, 1969-04-08…09-03) verifiziert. Offen: `DD005407`/`DD005409` sind strukturell andere (Summary-)Tapes (1206-B-Records ohne Observablen-Zeitfeld) — eigener Tape-Typ; die **Observable-Skalierung** einzelner Datentypen ist am Original gegenzuprüfen (Frequenz roh 1.08e12, /2⁹ ≈ 2114 MHz).
- **Blockade:** die Zusatz-Tapes brauchen eine eigene Messung; die Observable-Skala ist nicht durchgehend belegt.
- **Braucht:** die Summary-Tapes messen oder `descoped`; Observable-Skalierung gegen DSC 0073/Spec prüfen.

### Quellen-Terms schreiben — `phi/sources.φ` aktiv gehalten (blockiert)
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** die Freigabe von `phi/sources.φ` durch die haltende Session
- **Lage:** (gemessen 2026-10-11 via `git status`) `phi/sources.φ` trägt uncommittete url/origin/compiler-Manifest-Bindungen (Mycelium) plus `src/mathematikerin/*`; aktiver Fremd-Halter. Die 7 Terms/Verdikte (THEMIS-ASI/PCN/CSES admissibel; OE-417/BGS-GIC/NERC-GMD not-admit; McPherron point-event) sind gemessen, aber noch nicht als Verdikt-Zeilen geschrieben.
- **Blockade:** ein pfad-begrenzter Commit über `phi/sources.φ` nähme fremde Hünke mit („Commit als Letzter").
- **Braucht:** nach Freigabe die `terms`/`ttl`-Zeilen setzen (`terms` = Mountain, `url`/`origin`/`compiler` = Mycelium).

### THEMIS-ASI-Keogramm — Compiler-Fix blockiert am aktiven Dateiset (blockiert)
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** die Freigabe von `tools/harvest/src/bin/themis_asi_keogram_compiler.rs`
- **Lage:** (gemessen 2026-10-11 via `git status`) die Datei ist uncommittet modifiziert; Form-Verdikt steht (`on earth …` + `station …`, a=asilib autoritativ).
- **Blockade:** die Datei hält eine parallele Session (uncommitteter Hunk).
- **Braucht:** nach Freigabe `emit` auf `on earth <lat> <lon> <alt>` + `station <CODE>` umstellen; `cargo check` 0/0.

### Orphan-Prosa-Träger — `survey-2026-10-10-worldwide-lidar-quellen.md` (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Register-/Verdikt-Pass
- **Lage:** (gemessen 2026-10-11 via `register_lookup --orphan-docs`) das Dokument trägt **51 offene Marker** und keinen Live-Handover-Träger.
- **Blockade:** die Worldwide-LiDAR-Verdikte sind Mountain-Recht.
- **Braucht:** die 51 Marker sichten, je Klasse eine Verdikt-/Träger-Zeile, `phi/sources.φ`-Einträge setzen.

### Rätsel-Datenlücken (future-235/236) — url/origin (Mycelium) + Parse (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-11 via future-235/236) 4 offen erreichbar: Ⅶ Wurmloch (NAIF Voyager SPK) · Ⅷ Dunkler Fluss (Zenodo UNCOVER `14281664`, JADES `7460116`) · Ⅻ Urknall (BK18 `bicepkeck.org`) · GIC-Mäntsälä (FMI NUR); Ⅺ Placebo kein offenes Einzeldataset.
- **Blockade:** `url`/`origin` (Mycelium).
- **Braucht:** je Lücke die `url`-Zeile (Mycelium) → `terms`/`format` (Mountain) → Parser.

### NIST NCNR NSE — offene Kohärenz-Route (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-11) `https://ncnr.nist.gov/pub/ncnrdata/nse/` US-PD, monatliche Sets; Lauf `nist-ncnr-nse-cdn 38170359508` grün (`eee1e943…`); `trisp-keller` descoped.
- **Blockade:** `url`/`origin` (Mycelium).
- **Braucht:** `url`-Route registrieren (Mycelium) + `terms`/`field` (Mountain).

### Teil C — A5 offen (Galileo TDF), A6 descoped (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-11) A5 `GO-SS-RSS-1-TDF-V1.0` (PDS PPI); A6 descoped.
- **Blockade:** `url`/`origin` (Mycelium).
- **Braucht:** A5 die TDF-Route als `url`-Zeile (Mycelium) + `terms`/`format` (Mountain), oder `descoped` mit Befund.

### Band-Paar-Atom — CI-Idempotenz-Sperre (wartend)
- **Status:** wartend | **Bindung:** eigen | **Trigger:** ein `spdf.gsfc.nasa.gov`-Asset-Refresh + `witness-odf-residuum`-Lauf
- **Lage:** (gemessen 2026-10-11) `pioneer-odf-cdn.yml:21-31` überspringt Compile+Upload, wenn das Release-Asset existiert (zuletzt 2026-09-18) → Witness `model: 0 samples modeled`.
- **Blockade:** Manifestations-Recht (Mycelium).
- **Braucht:** ODF-Workflows mit `--force`/clobber neu dispatchten; danach Witness-Lauf lesen.

### ci-gate — rot am HEAD; format/register fremd (wartend)
- **Status:** wartend | **Bindung:** eigen | **Trigger:** der nächste abgeschlossene `ci-gate`-Lauf
- **Lage:** (gemessen 2026-10-11 via `ci_manage list`) `ci-gate` `38164577203`/`38164341503`/`38164276418` = failure; `odf.rs` clippy (`:179`/`:213`/`:2222`) Träger mountain.
- **Blockade:** `src/mathematikerin/*`-Format-Hünke hält eine andere Session; register/netloc-Bindung Mycelium.
- **Braucht:** `src/archivar/odf.rs` clippy heilen; die haltende Session formatiert `src/mathematikerin/*`; dann einen `ci-gate`-Lauf lesen (`ci_manage log <id>`).

### Eigene Ephemeride (PEP/Tudat) — Step 4 blockiert (wartend)
- **Status:** wartend | **Bindung:** eigen | **Trigger:** Freigabe von `src/mathematikerin/{mod,receiver,tropo}.rs`
- **Lage:** (gemessen 2026-10-11) Rat-Votum C; Research + UI konvergent.
- **Blockade:** fremd-aktiv gehaltenes Dateiset.
- **Braucht:** nach Freigabe die drei Rat-Schritte 1+2 als begrenzten Bau.

### opentopography_coverage — per-Dataset-Lizenz (eigen)
- **Status:** eigen | **Bindung:** eigen | **Trigger:** das Coverage-Manifest auf dem CDN + Block registrieren
- **Lage:** (gemessen 2026-10-11) per-Dataset-Lizenz; Portal-Manifest ohne `terms` (0 honored); `shard`-Emit nicht registriert.
- **Blockade:** Manifest-Asset (Mycelium).
- **Braucht:** Manifest (Mycelium) + Block (`url /origin /compiler /format coverage_manifest`, keine `terms`).

### SWPC/RTSW-Kurzreihen — nicht alignbar (eigen)
- **Status:** eigen | **Bindung:** eigen | **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-11, Lauf `38119733631` Job `matrix-vlies`) 126/210 Matrix-Zellen `alignment pending` — SWPC-Livereihen ~1 Tag, OMNI endet 2026-10-05; Kanäle auf CDAWeb ACE (AC_H2) retargetet.
- **Blockade:** fehlende statische überlappende Archivreihe.
- **Braucht:** Verdikt + `ttl` für eine statische SWPC/RTSW-Archivreihe (URL) **oder** descoped mit Befund.

### `api.sensor.community` — eigener Register-Punkt (eigen)
- **Status:** eigen | **Bindung:** eigen | **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-11 via future-235) kein Future-Wort; Ursprung `mycelium-158…162` (403) + `mountain-306`.
- **Blockade:** Arm/Zielzeile selbst zu benennen.
- **Braucht:** Arm/Zielzeile benennen oder `descoped` mit Befund.

### Sechs Klima-Quellen / series-manifest · copernicus B1–B6 · NOAA S3 B7/B8 · SPT-3G D1 (wartend)
- **Status:** wartend | **Bindung:** eigen | **Trigger:** je der erste CDN-/Manifest-Lauf bzw. gewählte Eingaben
- **Lage:** (gemessen 2026-10-11) ecad/aodn brauchen gewählte Station/URL; die übrigen Läufe laufen/fehlen.
- **Blockade:** kein Lauf / wissenschaftliche Wahl ohne Default.
- **Braucht:** die Läufe lesen (`ci_manage log <id>`) + sha256 je Familie.

### Frame/Epoche · particle-cern · Exposom-Matrix · iEEG · Flyby-Kette · FMHY (eigen/wartend/termin/blockiert)
- **Status:** eigen/wartend/termin/blockiert | **Bindung:** eigen/termin:2026-11-01
- **Trigger:** Frame/Epoche: CRS-VLR-Messung; particle-cern: `subset`; Exposom: extern gedeckter Repräsentativpunkt; iEEG: neues Operator-Wort über 2026-10-06 — **nicht erneut vorlegen**; Flyby: ESOC-Recon-Release; FMHY: Riss 4/5
- **Lage:** (gemessen 2026-10-11 via 319-Register; unverändert)
- **Blockade:** s. je Punkt im 319-Register.
- **Braucht:** s. je Punkt; sonst der benannte Riss/Descope-Befund.

## An mycelium

Origin: mountain-folge321 (2026-10-11).

- **`phi/sources.φ` aktiv gehalten:** uncommittete url/origin/compiler-Zeilen im Baum (plus `src/mathematikerin/*`); bitte committen, damit Mountains Verdikt-Zeilen (7 Terms) ohne Hunk-Kollision geschrieben werden können.
- **`themis_asi_keogram_compiler.rs` aktiv gehalten:** uncommittet modifiziert; bitte committen (Fix `at earth`→`on earth` wartet).
- **Mariner-Rohband-CDN:** die neuen Parser-Arme (`mariner_rst` + `mariner4_odg` gebaut) gehören in CI → CDN; `url`-Zeilen für `spdf.gsfc.nasa.gov/pub/data/mariner/{mariner4,6,7}/…` (NASA-PD) und den M10-Imaging-Spiegel `ser.im-ldi.com/M10/MVM/mvm_0001.tgz` (144 349 050 B, sha256 `44fcfd39…4888ef7`). Register-Block M4: `compiler tools/harvest/src/bin/mariner4_odg_compiler.rs` · `format mariner4_odg` · Asset `mariner4_odg.bin` (lokal sha256 528 `9219108f…39a4e8` / 529 `44f8c630…f365bf`). Das **NASA-NSSDCA** ist kein CDN-Host (request-only via CRUSO).
- **ASU-Mariner-10-Kontrollnetz (neu gemessen, `--all`):** `url`-Zeile für `ser.im-ldi.com/M10/M10_ISIS/camera_angles.txt` (194 640 B, sha256 `0a65df59…`) + die ISIS-Tars `ENC1/ENC2/ENC3/ALL_RES.tar` und die USGS-Astropedia-Mosaike (`mercury_messenger_mdis_nac_global_mosaic_1km`, US-PD) — dann `terms`/`format` durch Mountain. Das ist der „refined camera positions and orientations"-Datensatz (Robinson et al. 1999), der den Roll für die Merkur-Vorbeiflüge löst.
- **Riss im `mariner_rst`-Block (Mycelium):** `phi/sources.φ:31392` führt im `origin` fälschlich die **M4-Tar** `PSCM-00021_DD001528_05-DEC-64.tar` mit — M4 ist ein eigenes Format (`mariner4_odg`). Bitte den M4-Tar aus dem `mariner_rst`-Origin lösen und einen eigenen Block (`url`/`format mariner4_odg`/`origin` beide M4-Tars/`compiler mariner4_odg_compiler.rs`) anlegen; `terms CC0-1.0 …` schreibt Mountain (im `mariner_rst`-Block inzwischen gesetzt).
- **Rätsel-/NSE-Routen (future-235/236):** `url`/`origin` für NAIF Voyager SPK, Zenodo UNCOVER `14281664`/JADES `7460116`, BK18 `bicepkeck.org`, FMI NUR, NIST NCNR NSE — je `url` durch Mycelium, dann `terms`/`format` durch Mountain.

## Operator-Wort-Register

- **„kannst du bitte mit 4 und 10 weitermachen und den nssdc arm bauen"** | 2026-10-11 | Operator-Session (mountain) — Mariner 4 + Mariner 10 fortsetzen, NSSDC-Arm bauen. Angewandt: Arm gebaut (`--nssdc`/`--nssdca`), M4-Framing gemessen, M10-Decode verifiziert.
- **„und ich vermute wir müssen uns noch um eine bessere bilderkennung kümmern"** | 2026-10-11 | Operator-Session (mountain) — Bild-Erkennung als eigener Punkt benannt (M10-EDR-Bildpfad).
- **„das ist wichtig oder? und zwar ein state of the art erkennungspfad"** | 2026-10-11 | Operator-Session (mountain) — der Bild-Erkennungspfad ist wichtig und soll state-of-the-art sein. Angewandt: Extraktor + Stretch gebaut, Rat + UI-Roster zur Auslegung befragt.
- **„kannst du dafür bitte archive search all den rat und den max roster bemühen"** | 2026-10-11 | Operator-Session (mountain) — Research (`archive_search --all`) + Rat + UI-Roster vor dem Bau. Angewandt: `--all` + Fach-Modi (Research), Rat (5 Stimmen), 2 UI-Seats (Duck.ai GPT-6 Luna, Gemini 3.1 Pro).
- **„wären die bilder nicht auch wichtig für unsere ephemeriden kalibrierung und tatsächlich auch die exakte positionsbestimmung des mariners sozusagen eine dopplerkalibrierung"** | 2026-10-11 | Operator-Session (mountain) — Bild-als-Kalibrierquelle-Hypothese. Verdikt (Rat + UI + Research): valide als Optical-Navigation-Kreuz-Constraint (Bearing + Doppler), **keine** Doppler-Kalibrierung (Kategorienfehler); als abgeleiteter ω()-Term, post-hoc benannt.
- **„bitte befrage unbedingt claude und glm 5.3 max mit search und qwen 3.8 max und evtl noch weitere starke tryingopenseats"** | 2026-10-11 | Operator-Session (mountain) — UI-Runde 2. Angewandt: Claude Sonnet 5.5 Extra hoch (Antwort), Qwen3.7-Plus (Antwort), GLM-5.3 Max (`pending`, Deep Think >4 min); weitere tryingopen-Seats **nicht** gefahren (ehrlich benannt).
- **„…GLM/Z.ai und Qwen 3.8 bitte über tryingopen fahren der seat liefert übrigens durchaus 3.8 max wenn umgestellt"** | 2026-10-11 | Operator-Session (mountain) — UI-Runde 3. Angewandt: Qwen3.8 2.4T (Antwort), GLM 5.3 („Thinking…", `pending`); Lock gesetzt + gelöscht.
- **„…haben wir nicht jetzt mit lidar auch oberflächenmerkmale die wir zusammen mit den ephemeriden nutzen können ich meine die planeten dürften doch heute deutlich besser vermessen sein als damals"** | 2026-10-11 | Operator-Session (mountain) — Roll-/3-DOF-Lösung über moderne Oberflächenmerkmale. Gemessen: **ja**, etablierte Kontrollnetz-Methode — Cook & Robinson 2000 (`10.1029/1999je001135`, „refined camera positions and orientations"), MESSENGER-MDIS-basiertes Mariner-10-Kontrollnetz (MEXAG 2024 Eposter `6006`; Sonke & Robinson 2026 `10.3847/PSJ/ae32f5`); die M10-Merkur-Bilder sind **bereits** auf MDIS geometrisch kontrolliert. Gilt für die **Merkur**-Vorbeiflüge (Oberfläche sichtbar), **nicht** für Venus (Wolkendecke). Unser LiDAR ist terrestrisch; planetares Analogon = Altimetrie (MESSENGER MLA/Magellan) + Kontrollnetze.
- **„ja bitte mit archive search all"** | 2026-10-11 | Operator-Session (mountain) — Kontrollnetz-Quelle mit `archive_search --all` messen. Angewandt: 4× `--all` + `--ntrs/--ads/--openalex/--linkup/--github` + `--verdict/--sniff`; ASU-Kontrollnetz + USGS-Mosaike downloadbar gemessen, M10↔MDIS exakt nur Paper (`pending`).
- **„wir können doch mit ziemlich grosser sicherheit sagen wo sich mariner zum aufnahmezeitpunkt jeweils befand … die kameraeinstellungen und richtung zoom etc vermessen … einmal haben ist die rekonstruktion ein kinderspiel … weberinnen negativer fuzzyindex spezialfall vlt brauchen wir sogar die staubmaske"** | 2026-10-11 | Operator-Session (mountain) — Zeit+Bahn→Kamera-Hypothese. Verdikt (Claude, scharf): falsch gerahmt — **kein Zoom** (feste Brennweite), **ERT ≠ Aufnahmezeit**, Bahn+Zeit gibt **nur 2 von 3 DOF** (Roll nicht aus der Scheibe), es ist **Resection/Pointing-Refinement, keine Triangulation**; Rekonstruktion braucht Reseau/Verzeichnung + Radiometrie. Staubmaske = **Gewichtsmaske im Fit**, nicht Retusche. „Negativer Fuzzy-Index" ist kein etablierter Begriff (Definition offen).
- **„b"** | 2026-10-11 | Operator-Session (mountain) — Form **(b)** für das Band-Paar-Atom: Companion-Band-Ebene.
- **„hab glm abgeholt"** | 2026-10-11 | Operator-Session (mountain) — GLM 5.3-Antwort vom Operator nachgereicht.
- **„auth ist ok wenn wir elegible sind und die daten redistributable wobei für die vermessung ist es auch ok sie privat zu halten"** | 2026-10-11 | Future 236 (Zugangs-Regel) — Auth-Routen sind erlaubt, wenn eligibility + Redistributable; für die Vermessung privat ok.
- **„mach das ab jetzt automatisch"** | 2026-10-07 | Operator (Session-weit) — Commit + Push automatisch (trägt diesen Abschluss).

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort („mach das ab jetzt
automatisch", 2026-10-07) trägt Commit und Push. In diesem Atom: die eigenen
Dateien committen — `docs/handover/*`, `tools/utils/src/bin/archive_search.rs`,
`tools/utils/src/bin/archive_search/{net,nssdc}.rs`; `phi/sources.φ`,
`src/mathematikerin/*`, `docs/concepts/archivar-mathematikerin.md`,
`tools/measure/src/bin/themis_asi_timed_keogram.rs` sind fremd-aktiv gehalten und
werden nicht angetastet („Commit als Letzter").
