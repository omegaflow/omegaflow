<!--
  title: Handover — Mountain-Folge 322 (2026-10-12)
  session: Mountain-Linie in einem Pass — odf.rs-clippy geheilt (op_ref + type_complexity), mariner4_odg-terms gesetzt, THEMIS-ASI-Stationstabelle in die Archivar gelegt und beide Keogramm-Compiler auf die MIRACLE-Form (station + on earth) umgestellt
  class: handover
  date: 2026-10-12
  sha256: 34ebd773ead5e1efb25d2655a5654dec32c37f3fbf556d2ef4ccc9eae50eb124
  status: live
-->
# Handover — Mountain-Folge 322 (2026-10-12)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`). Diese
Session konsumierte `handover-2026-10-11-mountain-folge321.md` (→ `archiv/`).

## Burn: open 0.00 · close 0.26 · cap 0.50 · reason: Linien-Session (Mountain) ≈ $0.09 + 2 `general`-Taucher (asilib/SPEDAS-Stationstabelle ≈ $0.16), deepseek-flash, kein pro/max.

## Offen (aufgeschlüsselt)

### NSSDC-Arm — gebaut; die Auth-Route ist gemessen (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-11, live) `archive_search --nssdca <query>` (NASA NSSDCA Master Catalog) und `--nssdc <query>` (China NSSDC/VSSO `coreMetadataList`) sind gebaut (`tools/utils/src/bin/archive_search/nssdc.rs`, `cargo build -p omegaflow-utils --bin archive_search` clean, live gegen PSCM-00009/„Mariner 10"/CSES verifiziert). Beide Kataloge sind öffentlich; kein Login. Die NASA-Ausgabe läuft über CRUSO (E-Mail/Antrag); die China-Ausgabe über CAS-SSO + Captcha + Antragsformular (`nssdc_zh/js/common.js`) — nicht autonom.
- **Blockade:** keine für den Arm; die Ausgabe-Route ist kein Maschinenakt.
- **Braucht:** `NSSDC_USER`/`NSSDC_PASS` gehören zum **China**-NSSDC (CAS/Captcha-gated), **nicht** zum NASA-NSSDCA; für CSES der Operator-Antrag (Future), sonst `descoped`.

### Mariner 4 `mariner4_odg` — gebaut; terms gesetzt; CDN-Lauf offen (wartend)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der CDN-Lauf `mariner4-odg-cdn` (Mycelium)
- **Lage:** (gemessen 2026-10-11/12) Parser + Compiler gebaut, lokal verifiziert (528 → 10 600, 529 → 1025 Observablen, t 1964-11-28…1967-12-07); der Register-Block `phi/sources.φ:31391-31395` trägt url/format/origin/compiler; `terms CC0-1.0 https://spdf.gsfc.nasa.gov/data_use_policy.html` **in diesem Atom gesetzt** (`:31394`), konsistent zum `mariner_rst`-Sibling (`:31400`).
- **Blockade:** der CDN-Lauf (Ernte-Dispatcher = Mycelium) fehlt; `.github/workflows/mariner4-odg-cdn.yml` steht.
- **Braucht:** `mariner4-odg-cdn` dispatchten; danach den Asset-sha eintragen.

### Mariner 10 Imaging-EDR — Detektor offen (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Bau-Pass
- **Lage:** (gemessen 2026-10-11) `tools/utils/src/bin/m10_edr_image.rs` (echtes 832×700-gray8 → PNG + 2–98%-Stretch + Histogramm). Rat (5 Stimmen) + UI-Seats + Research konvergent: ein Kontrast-Stretch ist **kein** Gültigkeitstest; der Leser ist ein **deterministischer Detektor** (Hintergrund-Schätzung → Limbus-/Crescent-Maske → Ellipsen-/Limb-Fit, Zentroid + Residuum), kein Vision-LLM. Qwen3.8 2.4T nennt den wahrscheinlichsten Grund: ERT 17:34:07 = ~25–30 min **nach** Closest Approach (17:01 UT, 5768 km) → Venus ~**15° Winkeldurchmesser ≫ FOV** (0,42°/0,98°) → wahrscheinlich kein Limbus im Bild.
- **Blockade:** kein vermessenes Ziel/Referenzset.
- **Braucht:** **erwarteter Winkeldurchmesser je Frame** aus Ephemeride + FOV/Plattform-Offset (klärt, *ob überhaupt* ein Limbus im Bild sein kann), dann **radiusfixierter Strahlen-Fit** (PSF/erf-Radialprofil je Azimut, Radius fest aus Range+Brennweite, Huber/Tukey; kein Intensitäts-Schwerpunkt); ERT→Aufnahmezeit prüfen; **Roll** für Merkur über das ASU-Kontrollnetz (`ser.im-ldi.com/M10/M10_ISIS/camera_angles.txt`, sha `0a65df59…`) statt Telemetrie.

### Bild ↔ Doppler — Optical-Navigation-Kreuz-Constraint (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Bau-Pass
- **Lage:** (gemessen 2026-10-11 via Rat + Research) die Physik ist **valide, aber kein Sample**: Bildwinkel (Line-of-Sight) + Doppler (Range-Rate) binden gemeinsam die deklarierte `Receiver`-Worldline per Least Squares; Mariner 10 wurde **nicht** mit Onboard-OpNav geflogen → **post-hoc**, so benannt. **Keine** Doppler-Kalibrierung (Kategorienfehler): Doppler stammt aus Radio-Tracking, die Bilder sind eine unabhängige **Bearing**-Beobachtung.
- **Blockade:** die Bildserie trägt noch kein gemessenes Ziel (M10-Punkt darüber).
- **Braucht:** Eintritt als **abgeleiteter gemeinsamer Term im ω()-Loop** über die geteilte Receiver-Worldline (kein Sample-Slot); `Riss`, wo Bild und Doppler sich nicht überlappen. Erst sinnvoll, wenn ein Frame ein gemessenes Zentroid trägt.

### Mariner-6/7-Tracking (`mariner_rst`) — Zusatz-Tapes + Observable-Skala (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-11 via grind-flash) Parser gebaut + auf M6 `DD005408` (55 900 Observablen 1969-03-01…08-29) / M7 `DD005410` (38 572, 1969-04-08…09-03) verifiziert. Offen: `DD005407`/`DD005409` sind strukturell andere (Summary-)Tapes (1206-B-Records ohne Observablen-Zeitfeld); die **Observable-Skalierung** einzelner Datentypen ist am Original gegenzuprüfen (Frequenz roh 1.08e12, /2⁹ ≈ 2114 MHz).
- **Blockade:** die Zusatz-Tapes brauchen eine eigene Messung; die Observable-Skala ist nicht durchgehend belegt.
- **Braucht:** die Summary-Tapes messen oder `descoped`; Observable-Skalierung gegen DSC 0073/Spec prüfen.

### Quellen-Terms — die 7 Verdikte gegen den Register gemessen (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-12 via `sgrep` gegen `phi/sources.φ`, Baum clean bei HEAD `7eb5e3dc5`) die 4 admissiblen sind **schon im Register**: PCN `terms CC-BY-4.0` (`:2188`, `ttl 86400`), CSES `terms CC0-1.0` (`:20205/20217/20229/20242`), THEMIS-ASI `terms free-open` (`:20909`, `ttl 604800`), McPherron `terms CC-BY-4.0` (`:21549`). `mariner4_odg`-terms **in diesem Atom gesetzt**. Die 3 not-admit: BGS-GIC/NERC-GMD haben **keinen** Block (absent = nicht aufgenommen), OE-417 hat einen älteren `oe417`-Block (`:11887-11893`) ohne `terms`.
- **Blockade:** es gibt **keine** Register-Direktive für ein not-admit/declined-Verdikt — `src/archivar/parse.rs:265` speichert `terms` als freien String, `commit_gate` kennt keine Disposition, die ehemalige `blocked_sources.φ` existiert nicht mehr.
- **Braucht:** eine gemessene Register-Form für die not-admit-Disposition (Rat/Operator) **oder** `descoped` mit Befund; OE-417 entscheiden (Block als allgemeiner Datensatz mit Scope-Notiz oder streichen).

### THEMIS-ASI — MIRACLE-Form gebaut; WHIT/GBAY `alt` pending (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-12) Stationstabelle `src/archivar/themis_asi.rs::STATIONS` (24 Stationen, aus Myceliums Messung `state/mycelium/themis_asi_stations.md`, `h = H + N` EGM2008); `Themis-ASI`-Kanonliste = Berkeley Rev B. Beide Compiler (`themis_asi_keogram_compiler.rs`, `themis_asi_timed_keogram_compiler.rs`) emittieren jetzt `station <CODE>` + `on earth <lat> <lon> <alt>` (MIRACLE-Form, `phi/sources.φ:7582/7585`) statt `at earth`; `cargo check` **0/0**, beide Bins build clean. **Riss gegen Myceliums „24/24":** WHIT `H` `pending` (keine Punktmessung) und GBAY `H` rissig (12 Ort / 48.8 Flugplatz) → beide `alt_m: None` → der Compiler überspringt sie namentlich (0 honored).
- **Blockade:** WHIT/GBAY `H` ungemessen.
- **Braucht:** WHIT/GBAY `H` aus einer Punktmessung (Antennen-/Flughafen-Höhe) oder `pending` lassen; dann in `STATIONS` nachtragen.

### Orphan-Prosa-Träger — `survey-2026-10-10-worldwide-lidar-quellen.md` (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Register-/Verdikt-Pass
- **Lage:** (gemessen 2026-10-11 via `register_lookup --orphan-docs`; 2026-10-12 = 0 Orphan-Docs) das Dokument trug 51 offene Marker und keinen Live-Handover-Träger.
- **Blockade:** die Worldwide-LiDAR-Verdikte sind Mountain-Recht.
- **Braucht:** die 51 Marker sichten, je Klasse eine Verdikt-/Träger-Zeile, Register-Einträge in `phi/sources.φ` setzen.

### `intermagnet_dbdt` — Resolver-Regel statt Rename (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** Freigabe von `tools/measure/src/bin/field_te_query.rs` (Datei-Kollision mit Rivers measure-Atom)
- **Lage:** (gemessen 2026-10-12 via Rat, `state/stimmen/2026-10-12_river-intermagnet-identitaet-rat.md`) Rat-Verdikt **Nein** zum Rename: Identität bleibt, Kadenz ist Auflösungs-Attribut. Die Regel existiert im Kern bereits (`AGGREGATION_IDENTITY_CORR = 0.99`, feinste `median_dt` als Zeuge, Verweigerung = Riss). Defekt: `field_te_query.rs:1263-1341` prüft `abk_1m` auch gegen `sod_1h` (falscher Riss) und lässt den gröberen Zeugen fallen.
- **Blockade:** die Datei hält eine parallele Session (measure-Atom).
- **Braucht:** nach Freigabe nach `station_code` gruppieren + den gröberen Zeugen als Riss **daneben tragen** statt fallenlassen; Register/Wire unangetastet.

### `solar_f107_flux_sfu` — Zeitbasis-Defekt (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Bau-Pass
- **Lage:** (gemessen 2026-10-12, `general`) der SWPC-Textarm liefert **TDB/J2000** (`extract.rs:6506`, `lsk.unix_to_tdb`), das Penticton-`f107`-Asset **rohe Unix-Sekunden** (`src/archivar/f107.rs:22`, `day*86400`); feste Verschiebung `946728000 − 69.184 ≈ 9.4673e8 s ≈ 30,0 a` → SWPC-2026 paart mit Penticton-1996 → r≈0.02. **Zwei Archivar-Defekte:** (i) `f107::parse_series` wendet `unix_to_tdb` nicht an; (ii) `series_epoch_of` (`extract.rs:6506-6539`) wählt den ersten `"date"`-Key (`avg_begin_date` kann `time_tag` verdrängen).
- **Blockade:** beide Arme brauchen dieselbe Epoch-Basis (Signatur-/Aufrufer-Änderung).
- **Braucht:** beide Arme auf TDB bringen; dann den `f107`-Join erneut messen.

### `MCON` — fünf `field`-Zeilen (P10) (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-12 via mycelium-318) Registerdrift gelöst (`phi:21545-21551`, `format mcon_event`, sha `441df5f8…`, `terms CC-BY-4.0`). Offen: die fünf `field`-Zeilen (P10) für die MCON-Komponenten.
- **Blockade:** keine.
- **Braucht:** die fünf `field`-Zeilen aus `src/archivar/mcon_event.rs` (`mcon_sml_nt`/`mcon_amplitude_nt`/`mcon_bay_decrease_nt`/`mcon_bay_duration_min`/`mcon_bay_width_min`) setzen.

### `MCON`-Spanne — Inhaltsmessung (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-12 via mycelium-318/river-194) Zenodo nominiert `1966–2025`, das Parser-Fenster endet 2024-05-29; nominal vs Inhalt.
- **Blockade:** keine (die BPA-Überschneidung 2026 ist nicht berührt).
- **Braucht:** ersten/letzten Onset des manifestierten `mcpherron_onsets.bin` messen; Registerform danach.

### `bpa_gic` — zweite GIC-Zeugenreihe leer (descoped)
- **Status:** descoped | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-12, mycelium-318) kein historisches Archiv (gemessene Null; nur FOIA). `terms unknown` bleibt.
- **Blockade:** keine.
- **Braucht:** Befund `(gemessen 2026-10-12 via mycelium-318)` — keine Handlung.

### Rätsel-Datenlücken (future-235/236) — url/origin (Mycelium) + Parse (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-11 via future-235/236) 4 offen erreichbar: Ⅶ Wurmloch (NAIF Voyager SPK) · Ⅷ Dunkler Fluss (Zenodo UNCOVER `14281664`, JADES `7460116`) · Ⅻ Urknall (BK18 `bicepkeck.org`) · GIC-Mäntsälä (FMI NUR); Ⅺ Placebo kein offenes Einzeldataset.
- **Blockade:** `url`/`origin` (Mycelium).
- **Braucht:** je Lücke die `url`-Zeile (Mycelium) → `terms`/`format` (Mountain) → Parser.

### NIST NCNR NSE — offene Kohärenz-Route (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-11) `https://ncnr.nist.gov/pub/ncnrdata/nse/` US-PD, monatliche Sets; `nist-ncnr-nse-cdn` grün (`eee1e943…`); `trisp-keller` descoped.
- **Blockade:** `url`/`origin` (Mycelium).
- **Braucht:** `url`-Route registrieren (Mycelium) + `terms`/`field` (Mountain).

### Teil C — A5 offen (Galileo TDF), A6 descoped (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-11) A5 `GO-SS-RSS-1-TDF-V1.0` (PDS PPI); A6 descoped.
- **Blockade:** `url`/`origin` (Mycelium).
- **Braucht:** A5 die TDF-Route als `url`-Zeile (Mycelium) + `terms`/`format` (Mountain), oder `descoped` mit Befund.

### Band-Paar-Atom — CI-Idempotenz-Sperre (wartend)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein `spdf.gsfc.nasa.gov`-Asset-Refresh + `witness-odf-residuum`-Lauf
- **Lage:** (gemessen 2026-10-11) `pioneer-odf-cdn.yml:21-31` überspringt Compile+Upload, wenn das Release-Asset existiert (zuletzt 2026-09-18) → Witness `model: 0 samples modeled`.
- **Blockade:** Manifestations-Recht (Mycelium).
- **Braucht:** ODF-Workflows mit `--force`/clobber neu dispatchten; danach Witness-Lauf lesen.

### ci-gate — `odf.rs`-clippy geheilt; Format/Register fremd (wartend)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der nächste abgeschlossene `ci-gate`-Lauf am neuen HEAD
- **Lage:** (gemessen 2026-10-11 aus dem Log `38164577203`; 2026-10-12 geheilt) die drei `odf.rs`-clippy-Fehler (`:179` op_ref, `:213`/`:2222` type_complexity) sind in diesem Atom behoben (Refs entfernt; `pub type PodfBands`; `cargo check` 0/0) — `vmf3.rs:391/404` war bereits in `f63baf001` geheilt. Die `ci-gate`-Runs davor waren failure; `ci-gate 38213142214` u. a. queued.
- **Blockade:** `src/mathematikerin/*`-Format-Hünke hält eine andere Session; register/netloc-Bindung Mycelium.
- **Braucht:** die haltende Session formatiert `src/mathematikerin/*`; dann einen `ci-gate`-Lauf lesen (`ci_manage log <id>`).

### Eigene Ephemeride (PEP/Tudat) — Step 4 blockiert (wartend)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Freigabe der Dateien in `src/mathematikerin` (`mod`/`receiver`/`tropo`)
- **Lage:** (gemessen 2026-10-11) Rat-Votum C; Research + UI konvergent.
- **Blockade:** fremd-aktiv gehaltenes Dateiset.
- **Braucht:** nach Freigabe die drei Rat-Schritte 1+2 als begrenzten Bau.

### opentopography_coverage — per-Dataset-Lizenz (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** das Coverage-Manifest auf dem CDN + Block registrieren
- **Lage:** (gemessen 2026-10-11) per-Dataset-Lizenz; Portal-Manifest ohne `terms` (0 honored); `shard`-Emit nicht registriert.
- **Blockade:** Manifest-Asset (Mycelium).
- **Braucht:** Manifest (Mycelium) + Block (`url`/`origin`/`compiler`/`format coverage_manifest`, keine `terms`).

### SWPC/RTSW-Kurzreihen — nicht alignbar (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-11, Lauf `38119733631` Job `matrix-vlies`) 126/210 Matrix-Zellen `alignment pending` — SWPC-Livereihen ~1 Tag, OMNI endet 2026-10-05; Kanäle auf CDAWeb ACE (AC_H2) retargetet.
- **Blockade:** fehlende statische überlappende Archivreihe.
- **Braucht:** Verdikt + `ttl` für eine statische SWPC/RTSW-Archivreihe (URL) **oder** descoped mit Befund.

### `api.sensor.community` — eigener Register-Punkt (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-11 via future-235) kein Future-Wort; Ursprung `mycelium-158…162` (403) + `mountain-306`.
- **Blockade:** Arm/Zielzeile selbst zu benennen.
- **Braucht:** Arm/Zielzeile benennen oder `descoped` mit Befund.

### Sechs Klima-Quellen / series-manifest · copernicus B1–B6 · NOAA S3 B7/B8 · SPT-3G D1 (wartend)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** je der erste CDN-/Manifest-Lauf bzw. gewählte Eingaben
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

Origin: mountain-folge322 (2026-10-12).

- **Mariner-Rohband-CDN:** die Parser-Arme (`mariner_rst` + `mariner4_odg`) sind gebaut; `mariner4_odg`-terms gesetzt. Bitte `mariner4-odg-cdn` (+ `mariner-rst-cdn`) dispatchten; `url`-Zeilen für `spdf.gsfc.nasa.gov/pub/data/mariner/{mariner4,6,7}/…` (NASA-PD) und den M10-Imaging-Spiegel `ser.im-ldi.com/M10/MVM/mvm_0001.tgz` (144 349 050 B, sha256 `44fcfd39…4888ef7`) registrieren (Mycelium).
- **M10-Kontrollnetz (Kante):** `url`-Zeile für `ser.im-ldi.com/M10/M10_ISIS/camera_angles.txt` (194 640 B, sha256 `0a65df59…`) + die ISIS-Tars `ENC1/ENC2/ENC3/ALL_RES.tar` und die USGS-Astropedia-Mosaike (`mercury_messenger_mdis_nac_global_mosaic_1km`, US-PD) — dann `terms`/`format` durch Mountain.
- **Rätsel-/NSE-Routen (future-235/236):** `url`/`origin` für NAIF Voyager SPK, Zenodo UNCOVER `14281664`/JADES `7460116`, BK18 `bicepkeck.org`, FMI NUR, NIST NCNR NSE — je `url` durch Mycelium, dann `terms`/`format` durch Mountain.

## Operator-Wort-Register

- **„kannst du bitte mit 4 und 10 weitermachen und den nssdc arm bauen"** | 2026-10-11 | Operator-Session (mountain) — Mariner 4 + Mariner 10 fortsetzen, NSSDC-Arm bauen. Angewandt: Arm gebaut (`--nssdc`/`--nssdca`), M4-Framing gemessen, M10-Decode verifiziert.
- **„und ich vermute wir müssen uns noch um eine bessere bilderkennung kümmern"** | 2026-10-11 | Operator-Session (mountain) — Bild-Erkennung als eigener Punkt benannt (M10-EDR-Bildpfad).
- **„das ist wichtig oder? und zwar ein state of the art erkennungspfad"** | 2026-10-11 | Operator-Session (mountain) — der Bild-Erkennungspfad ist wichtig und soll state-of-the-art sein. Angewandt: Extraktor + Stretch gebaut, Rat + UI-Roster zur Auslegung befragt.
- **„kannst du dafür bitte archive search all den rat und den max roster bemühen"** | 2026-10-11 | Operator-Session (mountain) — Research (`archive_search --all`) + Rat + UI-Roster vor dem Bau. Angewandt.
- **„wären die bilder nicht auch wichtig für unsere ephemeriden kalibrierung … eine dopplerkalibrierung"** | 2026-10-11 | Operator-Session (mountain) — Bild-als-Kalibrierquelle. Verdikt: valide als Optical-Navigation-Kreuz-Constraint (Bearing + Doppler), **keine** Doppler-Kalibrierung; als abgeleiteter ω()-Term, post-hoc benannt.
- **„bitte befrage unbedingt claude und glm 5.3 max mit search und qwen 3.8 max …"** | 2026-10-11 | Operator-Session (mountain) — UI-Runde 2. Angewandt: Claude Sonnet 5.5, Qwen3.7-Plus; GLM-5.3 Max `pending`.
- **„…GLM/Z.ai und Qwen 3.8 bitte über tryingopen fahren …"** | 2026-10-11 | Operator-Session (mountain) — UI-Runde 3. Angewandt: Qwen3.8 2.4T (Antwort), GLM 5.3 (`pending`).
- **„…haben wir nicht jetzt mit lidar auch oberflächenmerkmale …"** | 2026-10-11 | Operator-Session (mountain) — Roll-/3-DOF-Lösung über moderne Oberflächenmerkmale. Gemessen: Cook & Robinson 2000 (`10.1029/1999je001135`), MESSENGER-MDIS-basiertes M10-Kontrollnetz (MEXAG 2024 Eposter `6006`; Sonke & Robinson 2026 `10.3847/PSJ/ae32f5`); gilt für Merkur, nicht Venus.
- **„ja bitte mit archive search all"** | 2026-10-11 | Operator-Session (mountain) — Kontrollnetz-Quelle mit `archive_search --all` messen. Angewandt.
- **„…die kameraeinstellungen und richtung zoom etc vermessen … negativer fuzzyindex … staubmaske"** | 2026-10-11 | Operator-Session (mountain) — Verdikt (Claude): **kein Zoom**, **ERT ≠ Aufnahmezeit**, Bahn+Zeit gibt nur 2 von 3 DOF; Resection/Pointing-Refinement, keine Triangulation; Staubmaske = **Gewichtsmaske im Fit**.
- **„auth ist ok wenn wir elegible sind und die daten redistributable wobei für die vermessung ist es auch ok sie privat zu halten"** | 2026-10-11 | Future 236 (Zugangs-Regel).
- **„mach das ab jetzt automatisch"** | 2026-10-07 | Operator (Session-weit) — Commit + Push automatisch.

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort („mach das ab jetzt
automatisch", 2026-10-07) trägt Commit und Push. In diesem Atom committet:
`src/archivar/odf.rs`, `src/archivar/themis_asi.rs`,
`tools/harvest/src/bin/themis_asi_keogram_compiler.rs`,
`tools/harvest/src/bin/themis_asi_timed_keogram_compiler.rs`, `phi/sources.φ`,
`docs/handover/*`.
