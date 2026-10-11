<!--
  title: Handover — Mountain-Folge 317 (2026-10-11)
  session: Mountain-Linie in einem Pass — MLA-Compiler auf die RDR-Route umgebaut + gebaut; VMF3 b/c-SH-Terme aus codes/vmf3.f90 portiert und an den Witness gebunden; uplink_band-Atom + ephemeris_sun-Tag-Riss vermessen; ci-gate/subset laufen am neuen HEAD
  class: handover
  date: 2026-10-11
  sha256: 5a9e2cbb2719bfb934e14a97216bae21377f68df79c3492188f2bff546e9bcd7
  status: live
-->
# Handover — Mountain-Folge 317 (2026-10-11)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`). Diese
Session konsumierte `handover-2026-10-11-mountain-folge316.md` (→ `archiv/`).

## Burn: open 0.00 · close 0.36 · cap 0.50 · reason: Linien-Session + vier begrenzte Taucher (MLA-Route, VMF3-Bindung 0.1047, Ephemeris-Tag 0.0697, uplink_band-Scope 0.0495 — gemessen via `session_burn`); die meisten offenen Punkte hängen an externen Triggern (CI-Läufe/Mycelium url/origin); deepseek-flash, kein pro/max.

## Offen (aufgeschlüsselt)

### ci-gate — format + register geheilt; Läufe am neuen HEAD
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der nächste `ci-gate`-Lauf am neuen HEAD
- **Lage:** (gemessen 2026-10-11 via `ci_manage list`) am HEAD `393cc26fe` laufen mehrere `ci-gate` (`38151750211`/`38151695723`/`38151330684`/`38150985884`, alle `in_progress`); der letzte abgeschlossene `ci-gate` `38143265804` trug `format`+`register` failure (in folge316 geheilt: `cargo fmt` der zwei Compiler, `register_sort --write`). Zusätzlich rot: `ci-check` `38150999644` (`test (4)`, Träger Mycelium/Stehender Pass). Post-push `tools-build` `38152726843` dispatcht (MLA-Compiler bin).
- **Blockade:** `cargo test` läuft nur in CI; kein Polling.
- **Braucht:** einen der `in_progress`-Läufe **einmal** lesen (`ci_manage log <id>`), sobald abgeschlossen — format/register/subset grün?

### LiDAR-Inventar / Extraterrestrisch — MLA-Compiler auf RDR umgebaut (gebaut); Block-Update offen
- **Status:** eigen (Bau) | **Bindung:** eigen
- **Trigger:** der nächste begrenzte Schritt (Mycelium url/origin → `field`-Zeile)
- **Lage:** (gemessen 2026-10-11 via `cargo build`; Träger `phi/sources.φ:12036`) Extraterrestrisch registriert: MOLA `field MARS_RADIUS mola_mars_radius` · NLR `field RANGE nlr_range`/`nlr_eros_radius` · OLA `field radius ola_radius` · Hayabusa `field RANGE hay_lidar_range`. **MLA-Compiler in diesem Atom umgebaut und gebaut** (`tools/harvest/src/bin/mla_compiler.rs`, 0/0): `CDR_ROUTE` → `RDR_ROUTE` (`…/messmla_2001/rdr_radr/`), `--route`-Flag, `collect_pairs` walkt jetzt **rekursiv** Jahr/Monat (`rdr_radr/<year>/<month>/`, Dateien `mlaradr<YYMMDDhhmm>.*`/`mlascirdr…`, `.lbl`/`.tab`). Gemessener Lauf `--route …/rdr_radr/2008/jan/`: `mlaradr0801141902.lbl` → `^STRUCTURE = MLASCIRDR.FMT` … (RDR-Label) ✓, `mlaradr0801141902` kompiliert (1548 Zeilen, `RANGE (ASCII_REAL) KM`, target MERCURY, roundtrip hält); `mlascirdr…` void (`INTERCHANGE_FORMAT` None) — **Parser-Grenze** `merge_structure` schneidet am ersten `END_OBJECT` (Header-Object vor `TABLE`), vorbestehend, benannt. **Register-Block noch CDR:** `phi/sources.φ:12036-12042` (`url …mlastacdr0408192000.bin`, `origin …/cdr/2004/aug/…`) — url/origin sind Myceliums Manifestations-Direktiven, der `field RANGE mla_range` folgt nach dem Repoint.
- **Blockade:** der Block-Repoint (url/origin/compiler) ist Myceliums Manifestations-Recht; der RDR-`mlascirdr`-Pfad scheitert an `merge_structure` (Parser-Grenze).
- **Braucht:** Mycelium: Block-`origin`→`…/rdr_radr/`, `url`-Asset auf das RDR-Produkt, CDN-Manifestation; dann `field RANGE mla_range inverse-square em km 604800 0.0 0.0` (Mountain). `opentopography_coverage`: Lizenz-Verdikt (per-Dataset → `terms`-Modell) oder nur `archive_search`-Arm.

### opentopography_coverage — per-Dataset-Lizenz
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** ein Lizenz-Modell für per-Dataset-terms
- **Lage:** (gemessen 2026-10-11 via `archive_search`) `portal.opentopography.org/API/otCatalog?outputFormat=json`, `format coverage_manifest` — die Lizenz ist **per-Dataset**, ein einzelnes `terms` überzeichnet → Riss.
- **Blockade:** das Register kennt kein per-Dataset-`terms`-Modell.
- **Braucht:** Verdikt (per-Dataset-`terms`-Modell einführen) oder nur `archive_search`-Arm ohne Register-Block.

### Sechs Klima-Quellen — ecad/aodn brauchen gewählte Eingaben
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** eine gewählte ecad-`station` bzw. aodn-`url`
- **Lage:** (gemessen 2026-10-11 via Workflow-Read) surfrad `38142701271` · dwd-cdc `38142703857` · worldclim `38142706143` · knmi `38142709395` dispatcht; `ecad-cdn.yml` verlangt `element`+`station`+`lat`/`lon`/`alt` (per-Akt-Input, kein Default), `aodn-cdn.yml` eine OPeNDAP-`url`.
- **Blockade:** die Station/das Dataset ist eine wissenschaftliche Wahl, kein Default.
- **Braucht:** je eine gewählte ECA&D-Station und eine AODN-OPeNDAP-URL → `gh workflow run ecad-cdn.yml -f station=… -f lat=… -f lon=… -f alt=…` / `aodn-cdn.yml -f url=…`.

### copernicus B1–B6 — Läufe offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der erste `copernicus-<dataset>-cdn`-Lauf je Dataset
- **Lage:** (gemessen 2026-10-11 via folge315-Register) (folge315, unverändert) sechs Workflows; Läufe uscrn `38136104626` · surface-land `38136110423` · igra `38136112567` · woudc `38136114532` · gruan `38136116720` · gnss `38136118542`.
- **Blockade:** kein Ergebnis.
- **Braucht:** die sechs Läufe lesen; bei Void Variable/Version korrigieren. Riss: `uscrn` `time_aggregation=daily` / `gnss` `network_type=igs_daily` sind Wahlen ohne Default.

### NOAA S3-Compiler B7/B8 — Läufe offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der erste `noaa-nos-lidar`/`noaa-eri`-CDN-Lauf
- **Lage:** (gemessen 2026-10-11 via folge315-Register) (folge315, unverändert) `noaa-nos-lidar-cdn.yml` + `noaa-eri-cdn.yml`; Bins grün; Läufe nos-lidar `38136120891` · eri `38136123058`.
- **Blockade:** kein Ergebnis.
- **Braucht:** die zwei Läufe lesen.

### Teil C — A5/A6 Earth-Flyby-ODFs
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-11 via folge315-Register) (folge315, unverändert) **A5** Galileo Earth1/Earth2-Tracking gefunden als TDF — `GO-SS-RSS-1-TDF-V1.0` (PDS PPI; 1990 DOY 342-344 + 1992 DOY 343-344; keine `.ODF`). **A6** MESSENGER 2005 Earth-Flyby absent — RS-Rohbündel beginnen 2006/2007.
- **Blockade:** keine.
- **Braucht:** A5 die TDF-Route als Quelle registrieren (oder `descoped`); A6 `descoped` mit Befund.

### Manifestation SPT-3G D1 — sha256
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein abgeschlossener `cmb-cdn`-Lauf
- **Lage:** (gemessen 2026-10-11 via folge315-Register) (folge315, unverändert) `cmb-cdn 38138848760` in_progress im `Compile`; der alte `38129785553` war rot (`NAXIS2 36557509 != 12*NSIDE^2 (8192)`) — Fix unter Test, Träger Mycelium.
- **Blockade:** Lauf läuft.
- **Braucht:** Lauf-Ergebnis lesen; danach sha256 in `phi/sources.φ` nachtragen.

### Frame/Epoche — Transform-Layer
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Architektur-Entscheid der Rat-Linse (Vorlage `state/mycelium/frame-epoch-findings.md`)
- **Lage:** (gemessen 2026-10-11 via folge315-Register) (folge315, unverändert) bbox-off Default steht; Transform-Layer `pending`.
- **Blockade:** Architektur.
- **Braucht:** Rat-Linse über den Transform-Layer.

### Eigene Ephemeride (PEP/Tudat) — Kette gemessen; Witness-Residuum-Bin gebaut (Step 3)
- **Status:** eigen (Bau) | **Bindung:** eigen
- **Trigger:** der nächste begrenzte Schritt (den vollen `two_way_doppler`-Operator an das Witness-Residuum binden)
- **Lage:** (gemessen 2026-10-11 via `sread` + `cargo check`; Träger `docs/concepts/eigene-ephemeride.md`) **Step 0 (Operator/Kalibration): teil** — Zeit `src/archivar/lsk.rs unix_to_tdb` ✓, Rahmen `src/mathematikerin/receiver.rs` (`precession_matrix_fw06`/`nutation_angles_2000a` 1365-Term/`cirs_to_gcrs`/`itrf_to_icrs`) ✓, Lichtzeit+Shapiro `receiver.rs two_way_doppler`/`shapiro_leg_s` ✓ — **aber `two_way_doppler` wird von keinem Bin gerufen** (nur Tests); die Residuen-Tools nutzen den alten Teil-Pfad `odp.rs downlink_rate_core` (downlink only, kein Shapiro/EOP). **Step 0e Medien/Troposphäre: absent** (VMF3/IONEX nur als Harvest-Compiler). **Step 1/2: teil** (Pioneer-Residuen-Tools, je hartkodiert `const SC_BODY`). **Step 3 vorher absent → in diesem Atom gebaut:** `tools/measure/src/bin/witness_odf_residuum.rs` (Registry `witness_source` + `--body`; pioneer10/galileo_e1 registriert; `cargo check` + build 0/0). **Adapter gebaut** (`007396f68`): `receiver.rs two_way_light_time` exponiert `t_tx_tdb`, die Gravitator-Felder in `TwoWayDopplerInputs` werden `Option` (absent ≠ 0), neues `src/mathematikerin/tropo.rs` (VMF3-Slant-Delay + 4 Tests) — `cargo check` 0/0. **Step 4: `least_squares.rs` gebaut, nicht an ein Bahn-Residuum verdrahtet.** Riss: Konzept/Handover nennen `observer.rs` — die Datei heißt seit `55e06c189` `receiver.rs`.
- **Blockade:** der volle Operator passt nicht ohne Adapter — `two_way_doppler` liefert `observable_hz` (nicht range-rate), braucht `turn_ratio` + einen deklarierten Sonnen-Gravitator + `t_tx` aus der Lichtzeit-Iteration (nicht exponiert).
- **Braucht:** (a) das Witness-Bin an `two_way_doppler` binden — **erledigt**: CI-Lauf `38151032366` (success, 2026-10-11) rechnete die volle Kette auf **59 486 echten Pioneer-10-Samples** (1973–1998); Per-Station-Fit `obs = A·ṙ₂w + B_Pass` liest A ≈ +f/c, der downlink-only-Vergleich ≈ 2× → die **Uplink-Strecke wird getragen** (der alte Pfad trug sie nicht); Zweiweg-Scan 63×63 Peak 0,5 mHz bei 24,9× lokalem Grundrauschen (Pioneer-Anomalie-Band); `model` 59 363/59 486 (123 ohne Stationskoordinaten, 0 honored). (b) die VMF3-**Koeffizienten** — **erledigt**: die b/c sind nicht in den TU-Wien-Produkten (die tragen nur `ah aw zhd zwd`); die empirischen Grad-12-SH-Tabellen wurden mechanisch aus `codes/vmf3.f90` (sha256 `ae75c278…`, HTTP 206) portiert (`src/archivar/vmf3_sh.rs` + `vmf3_sh_data.rs`), `grid_product`/`site_product` bauen `Vmf3Coefficients` aus dem geharvesteten `vmf3_grid.bin`, der Witness-Residuen-Lauf rechnet den Zweiweg-Troposphären-Range-Rate (fehlende Koeffizienten → geometrische Rate, benannt, 0 honored), `cargo check` 0/0; (c) `uplink_band` — Scope vermessen (s. eigener Punkt); (d) der `ephemeris_sun`-Tag-Riss — vermessen (s. eigener Punkt). PEP/Tudat bleiben Lizenz-/Verfahrens-Referenz.

### uplink_band-Atom — Scope vermessen, Bau ausstehend
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** ein eigener, größerer Atom (Record-Kontrakt-Änderung)
- **Lage:** (gemessen 2026-10-11 durch Taucher) der `[f64;9]`-PODF-Record wird von 5 Layouts geteilt (PODF/ODFS/P11R/PNAV/TNF; Kopf 4-Byte-Magic + u32 count + N×72 B); 72-Byte-Hardcodes u. a. `odf.rs:103/115/125/156/161/195/213/218/236/252/257/2580/2878` + `doppler.rs:39/55/60`; **14 ODF-Compiler** verwerfen `OdOrbit.uplink_band` beim Schreiben (cassini/dawn/galileo/juno/magellan/mars_express/messenger/mgs/mro/odyssey/pathfinder/pioneer10/pioneer11/vex); PNAV trägt kein Band, TNF kein Uplink-Band. Verbraucher-Union **45** Code-Dateien (die 29 sind nur die literalen `[f64;9]`-Deklarationen; 16 binden per Inferenz). Gelesen `odf.rs:61/:80/:94`; gebraucht als `turn_ratio` in `TwoWayDopplerInputs` (`receiver.rs:152`).
- **Blockade:** eine Record-Kontrakt-Änderung (auf `[f64;10]` oder getrennte Band-Ebene) berührt alle Serializer + 45 Verbraucher; kein begrenzter Schritt.
- **Braucht:** eigener Atom — Kontrakt festlegen, TNF-Absenz (`Option`, kein 0.0) entscheiden, dann alle Serializer/Verbraucher mechanisch; Scope-Karte: dieses Handover.

### Ephemeris-CDN-Tag — Repoint gemessen, blockiert (vorbestehende Gate-Verstöße)
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** die Heilung der remove-bias/NaN-Verstöße in den betroffenen measure-Bins
- **Lage:** (gemessen 2026-10-11 durch Taucher; Träger `src/archivar/cdn.rs:23` `EPHEMERIS_TAG`) **beide** Tags tragen Ephemeriden: `ssd.jpl.nasa.gov` 999 Assets (107 `ephemeris_*`), `ssd.jpl.nasa.gov-ephemeris` 72 (alle `ephemeris_*`); 13 Bins nur in `-ephemeris`, 49 nur in `ssd.jpl.nasa.gov`. Keine Ein-Tag-Vereinheitlichung: nur Referenzen, deren Asset in `-ephemeris` liegt, gehören dorthin. Kandidaten (22 Sites/11 Dateien): `lsst_anomaly_probe` · `signal_cone_audit_probe` · `tdb_coincidence_probe` · `solar_cycle_probe` · `quake_location_probe` · `body_fixed_earth_audit_probe` · `des_y6_weave_probe` · `dispersion_ortung_probe` · `flyby_ephemeris_gate` · `flyby_path2_fill` · `topocentric_coupling_probe`; `ssd`-only-Bins (`pioneer*_daily`, `uranus_c`, `neptune_c`, `de440_*`…) bleiben am `ssd`-Pfad. Der Repoint ist gebaut, aber `commit_check` blockt 10 Dateien an **vorbestehenden** Funden (Body-Name-Literal „earth"/„sun"; NaN als Datenmarker in `quake`/`solar_cycle`) — Whole-File-Scan beim Commit. Revert, kein uncommitteter Rest.
- **Blockade:** die 10 measure-Bins tragen remove-bias/NaN-Verstöße aus WP13; sie müssen geheilt sein, bevor ein Tag-Repoint committet.
- **Braucht:** eigener Atom (River/Mountain): die Funde heilen (Receiver als Record-Daten, `Option` statt NaN), dann den Tag-Repoint committen; Scope-Karte: dieses Handover.

### particle-cern — Baskets + Sample-Verifikation
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der nächste `subset`-Lauf (CI)
- **Lage:** (gemessen 2026-10-11 via folge315-Register) (folge315, unverändert) Basket-Arrays gebaut; `cargo test` läuft nur in CI.
- **Blockade:** `subset` hängt am `ci-gate`-Punkt.
- **Braucht:** `subset` grün; ggf. ein v9-only-Sample.

### Exposom-Matrix → TE-Paar-Feed — Repräsentativpunkt
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** extern gedeckter Repräsentativpunkt je Zeile (Operator-Wort 2026-10-10)
- **Lage:** (gemessen 2026-10-11 via folge315-Register) (folge315, unverändert) kein offener Datensatz trägt eine im Datensatz gemessene Koordinate.
- **Blockade:** der Repräsentativpunkt ist eine wissenschaftliche Annahme, keine Messung.
- **Braucht:** je Zeile den extern gedeckten Repräsentativpunkt → eine Zeile in `phi/sources.φ` → TE-Paar-CI-Feed.

### iEEG — registriertes Wort maßgeblich
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein neues Operator-Wort über 2026-10-06 hinaus
- **Lage:** (gemessen 2026-10-11 via folge315-Register) (folge315, unverändert) iEEG = privates Experiment, kein CDN.
- **Blockade:** keine.
- **Braucht:** nur ein neues Operator-Wort öffnet es. **Nicht erneut vorlegen.**

### api.sensor.community — eigener Punkt, Arm/Zielzeile unbenannt
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste begrenzte Schritt (Arm/Zielzeile benennen)
- **Lage:** (gemessen 2026-10-11 via future-235) HTTP 200; `sensor.community` bereits registriert (`phi:338/1218`). **Kein Future-Wort** — es ist ein eigener Register-Punkt (Ursprung `mycelium-158…162` 403/ip-blocked + `mountain-306`).
- **Blockade:** „zulassen" nennt keinen Arm/keine Zeile.
- **Braucht:** Arm/Zielzeile selbst benennen oder `descoped` mit Befund.

### Flyby-Kette — termin 2026-11-01
- **Status:** termin | **Bindung:** termin:2026-11-01
- **Trigger:** ESOC-Recon-Release (oder Descope)
- **Lage:** (gemessen 2026-10-11 via folge315-Register) (folge315, unverändert) die JUICE-`orbc`-SPK (A7) ist registriert; `doppler.rs` absent.
- **Blockade:** kein ESOC-Recon-Release.
- **Braucht:** ESOC-Release oder Descope-Befund für `doppler.rs`.

### FMHY-Routing — Riss 4 bot-gated
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Riss 4/5-Abschluss — Overpass bot-gated (gemessen 2026-10-10)
- **Lage:** (gemessen 2026-10-11 via folge315-Register) (folge315, unverändert) Overpass HTTP 406 direkt+Proton → bot-gated.
- **Blockade:** Overpass bot-gated (HTTP 406 direkt+Proton).
- **Braucht:** Riss-4-Abschluss oder Descope.

### DEMETER — Lizenz-Riss (Etalab-2.0 widerlegt)
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass (URL/Origin von Mycelium)
- **Lage:** (gemessen 2026-10-11 via `archive_search`/`sfetch` durch Taucher, URL 200) `regards.cnes.fr` trägt **nicht** Etalab-2.0 (kein Treffer im gerenderten Baum, `data.gouv.fr`-API total 0) — das CDPP-eigene Lizenzdokument `https://cdpp.irap.omp.eu/index.php/licence-sipad` („LICENCE FOR DATA CENTRE USERS V1.1") verbietet jeden kommerziellen Gebrauch und verlangt für jede Weitergabe archivierter Produkte an Dritte die vorherige schriftliche Zustimmung. **Riss:** die Mycelium-Übergabe 308:118 nennt „Lizenz (gemessen): Licence Etalab-2.0"; der Baum widerlegt es. Die 6 Feldnamen sind verifiziert (`src/archivar/extract.rs:694-700`: `demeter_isl_{orbit_count,ne_cm3,ni_cm3,te_k,vf_v,vi0_ms}`) — der Handover-Name stimmt.
- **Blockade:** die gemessene Lizenz ist in Spannung mit der CDN-Manifestation (Weitergabe an Dritte).
- **Braucht:** Verdikt: CDN-Spiegel der DEMETER-Serie gegen die Lizenz prüfen; bei Konflikt keine Manifestation, nur lokale Registrierung. `terms`-Zeile + 6 `field`-Zeilen (Mountain) parallel zu Myceliums `url`/`origin` — Mycelium zieht zuerst url/origin.

### Freie GIC-/Space-Weather-Quellen — Verdikte (river-186)
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-11 durch Taucher, `archive_search --zenodo`/`--verdict`) **CSES** Zenodo `13329652`+`8082201` — `cc-by-4.0` (Zenodo-API), ttl 604800; **PCN** `ftp.space.dtu.dk` — `CC-BY-4.0` belegt auf `isgi.unistra.fr/indices_pc.php` (bestehender `phi:2211`-Eintrag trägt `terms CC-BY-4.0` ohne URL), ttl 86400; **PCS** — Lizenz auf der Seite nicht genannt → `pending`; **EAGLE-I** `data.openei.org/submissions/6458` — `CC BY 4.0` (JSON-LD `license`), ttl 604800; **OE-417** `eia.gov` — US-`PD`; **FMI/BPA/BGS GIC** — offen (Taucher-Lauf unvollständig). **INTERMAGNET** — `CC BY-NC 4.0`, aber Bulk-Weitergabe an Dritte schriftlich vorbehalten → lokale Quelle ja, CDN nein; **GIRO DIDBase** — `CC BY-NC-SA 4.0`, bereits registriert (`phi:1066`).
- **Blockade:** url/origin sind Myceliums Manifestations-Recht.
- **Braucht:** je Quelle die Verdikt-`terms`-Zeile (Mountain) + url/origin (Mycelium); FMI/BPA/BGS GIC restlich messen. **Korrektur river-187:** OE-417 `eia.gov` ist tot → echtes Portal `doe417.pnnl.gov` (SPA; Roh-Records auth-gated via Microsoft Entra) und trägt **keine GMD-Kategorie** → nicht als GMD-Quelle führen; OE-417/EAGLE-I brauchen zudem eine neue Feld-Familie (`count` aus `customers_out` / `power` aus `demand_loss_mw`, mycelium-309) — Entscheidung: Familie `count`/`power` freigeben? **THEMIS-ASI** frei (`api.aurorax.space`, `data.phys.ucalgary.ca`) — neuer Aurora-Zeuge; **PCN** 1975–2024 definitive (`PCN_quicklook`/`PCN_provisional` für 2025) bestätigt.

### Sechs Klima-`series-manifest` — erster Lauf je Familie (mycelium-phi-serien-ciplauf)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der erste `series-manifest`-Lauf je Klima-Familie
- **Lage:** (gemessen 2026-10-11 via future-235) die sechs Klima-Workflows tragen je einen `series-manifest`-Job (Shard-Assets → `<familie>.manifest`, `sort -u`, 0 honored); offen ist der erste Lauf je Familie (sha256). Alt-Stand `state/zustand/wartend-archiv-2026-10-11-r3.φ`.
- **Blockade:** kein Lauf.
- **Braucht:** die sechs Manifest-Läufe lesen (`ci_manage log <id>`), sha256 je Familie tragen.

### Re-Admission lizenz-blockierter Kandidaten (future-233)
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass
- **Blockade:** url/origin (Mycelium) für die Neu-Admits.
- **Braucht:** vier neue Verdikte/Registrierungen (GRDC-Caravan, RouteViews, EPH-VSOP, GloFAS) — Mountain setzt `terms`, Mycelium url/origin; NIST/Zenodo-restricted bleiben `pending`.

### Asservatenkammer-Survey — vier Marker (mycelium-308)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-11 via mycelium-308-Register) `docs/surveys/survey-2026-10-08-research-api-mcp.md` — vier Marker auf den gemessenen Stand setzen (Perplexity live · Consensus 429 transient · Elicit descoped · SciSpace pending).
- **Blockade:** keine.
- **Braucht:** die vier Marker im Survey setzen.

## An mycelium

Origin: mountain-folge317 (2026-10-11).

- **MLA-Register-Block:** der Compiler `mla_compiler.rs` walkt seit diesem Atom `…/messmla_2001/rdr_radr/` (RDR, rekursiv) statt `…/cdr/2004/aug/`; das RDR-Label trägt `NAME = RANGE`. Bitte den Block `phi/sources.φ:12036` repointen — `origin` auf `…/rdr_radr/`, `url`-Asset auf das RDR-Produkt (`pds3_fixed_width_mlaradr….bin`) und manifestieren; danach trägt Mountain `field RANGE mla_range inverse-square em km 604800 0.0 0.0` nach.
- **GIC/Space-Weather + Re-Admission:** die vier Neu-Admits (GRDC-Caravan/RouteViews/EPH-VSOP/GloFAS) brauchen nach Mountains `terms`-Verdikt url/origin.
- **DEMETER — Lizenz-Riss:** die behauptete „Licence Etalab-2.0" (`handover-2026-10-11-mycelium-folge308.md:118`) ist am Baum widerlegt (CDPP Data-Centre-Lizenz V1.1, nicht-kommerziell, Weitergabe an Dritte nur mit schriftlicher Zustimmung). Vor url/origin der DEMETER-Serie den CDN-Spiegel gegen die Lizenz prüfen; bei Konflikt nur lokale Registrierung.

## Operator-Wort-Register

- **„hä wir bitte du solltest eine taucherstaffel losschicken und du hast dich nur um copernicus gekümmert du sollst alle offenen punkte die die letzte session aufwändig dokumentiert hat umsetzen"** | 2026-10-11 | Operator-Session (mountain).
- **„die brauchen wir aber, oder hast du irgendeine idee, wie wir sie bekommen können? Nur A4 (ESA PSA) und A7 (ESA SPICE) Lizenz bleiben unmeasured"** | 2026-10-11 | Operator-Session (mountain).
- **„bitte lasse eine taucherstaffel auf ALLE offenen punkte los"** | 2026-10-11 | Operator-Session (mountain).
- **„Mindat Level-1 ist descoped ausser es dient als faden aber dann gibt es alternativen"** | 2026-10-11 | Operator-Session (mountain).
- **„388 … bauen"** | 2026-10-11 | Operator-Session (mountain).
- **„MODVOLC: 2000–2019-Archiv admitieren+bauen"** | 2026-10-11 | Operator-Session (mountain).
- **„schicke eine taucherstaffel los und BAU ENDLICH!!!!"** | 2026-10-11 | Operator-Session (mountain).
- **„die weltlinie kennt kein jetzt"** | 2026-10-11 | Operator-Session (mountain).
- **„wie bitte gehts noch wir wollen unsere eigenen Ephemeriden bauen"** | 2026-10-11 | Operator-Session (mountain) — die eigene Ephemeride (`docs/concepts/eigene-ephemeride.md`) ist Mountains Bau-Arbeit, nicht die Registrierung fremder (JPL-)Assets.
- **„mit einer taucherstaffel: ja bitte … und kümmer dich um deinen teil"** | 2026-10-11 | Operator-Session (mountain) — den `two_way_doppler`-Adapter (Step 3) per Taucherstaffel bauen; die adressierten `## An mountain`-Blöcke falten.

## Träger in `state/` — vieles ist schon recherchiert, nicht neu fahren

- **Eigene Ephemeride / Reduktionskette:** `state/mountain/2026-10-10_observer-operator_recherche.txt` · `state/mountain/2026-10-10_riss-ort_recherche.txt` · `state/stimmen/2026-10-02_ephemeriden-deepseek.md` · `state/stimmen/2026-10-10_sensory_eigene-ephemeride-round.md` · Konzept `docs/concepts/eigene-ephemeride.md`.
- **GIC / Space-Weather:** `state/stimmen/2026-10-02_autolauf-gsics-deepseek.md` (+ `-glm53flash`/`-kiloultra`/`-muse`/`-nemotron*`) · `state/stimmen/2026-09-30_arena-ui_grok-4.6-high_gic-review3.json` · `state/stimmen/2026-10-01_zai-ui_gic-review.txt`.
- **LiDAR-Verdikte:** `state/mycelium/lidar_id_diff_measured.md`.
- **Re-Admission / revive:** `state/future/revive-kandidaten-2026-10-11.md`.
- **Quellen-Landschaft / Routing / Lizenzen:** `state/mountain/routing-untersuchung-2026-10-10.md` · `state/mountain/license-census.tsv` · `state/mountain/blocked-sources-klassen.md`.
- **Asservatenkammer:** `docs/surveys/survey-2026-10-10-asservatenkammer-zensus.md`; **Elektronen-/Vlies-/TE-Runden:** `state/research/` + `state/stimmen/2026-10-10-river-c-skalenspektrum-rat.md`.

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort („mach das ab jetzt
automatisch", 2026-10-07) trägt Commit und Push.
