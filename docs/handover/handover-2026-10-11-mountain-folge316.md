<!--
  title: Handover — Mountain-Folge 316 (2026-10-11)
  session: Mountain-Linie in einem Pass — register_sort canonical + mla/ola fmt geheilt (ci-gate register/format), te-bias-n dispatcht, DEMETER-Lizenz-Riss + GIC/Re-Admission-Verdikte gemessen; vier Taucher-Dossiers
  class: handover
  date: 2026-10-11
  sha256: 5fd73358b3d6ddd1d1dd512089bcb85e97e684e38d652bf00f1a199dee243c9c
  status: live
-->
# Handover — Mountain-Folge 316 (2026-10-11)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`). Diese
Session konsumierte `handover-2026-10-11-mountain-folge315.md` (→ `archiv/`).

## Burn: open 0.0076 · close 0.37 (Linien-Session 0.131 + Taucher ≈0.24) · cap 0.45 · reason: Mountain-Linien-Session + sieben grind-flash Taucher (CI-Triage, GIC, Re-Admission, DEMETER, MLA-RDR/Coverage, eigene-Ephemeride-Survey, Witness-Bin) + register_sort + te-bias-n; Fenster-Aggregat 1.2101 über 29 Sessions (fremde Linien inkl., Linien-Anteil nicht exakt isolierbar); deepseek-flash, kein pro/max; gemessen via `session_burn`.

## Offen (aufgeschlüsselt)

### ci-gate — format + register geheilt, CDN-Test grün; nächster Lauf
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der nächste `ci-gate`-Lauf am neuen HEAD
- **Lage:** (gemessen 2026-10-11 via `ci_manage log 38143265804`, HEAD `0e742f052`) der jüngste abgeschlossene `ci-gate` `38143265804`: `build`✓ `clippy`✓ `dropped-gate`✓, **`format` failure** (gemessen: unformatierte `mla_compiler.rs`/`ola_compiler.rs` in HEAD), **`register` failure** (gemessen `register_sort`: 1 `ttl`-order — `pds3_fixed_width_ap01578l.bin` nach `ttl 315360000`; 2 `url`-order — `AFRP20-MASK-3D…nc` nach `masie_ice_extent.bin`, `pds3_fixed_width_mlastacdr0408192000.bin` nach `pds3_fixed_width_l00074ng.bin`), **`subset` cancelled** (Folge der zwei failures, kein eigener Fehlschlag). Der CDN-Test `archivar::parse::tests::every_cdn_source_carries_its_origin_and_producer_or_is_a_reference` **passierte** (13:55:28) — die 315er Wurzel ist geheilt. **In diesem Atom geheilt:** `cargo fmt` der zwei eigenen Compiler; `register_sort phi/sources.φ --write` (1869 Blöcke neu geordnet, 0 Duplikate entfernt; danach `exit 0` canonical über 2786 Blöcke).
- **Blockade:** `cargo test` läuft nur in CI.
- **Braucht:** den nächsten `ci-gate`-Lauf am neuen HEAD lesen (`ci_manage log <id>`) — format/register/subset grün?

### te-ground-truth — AR(1)-Null-Arm; `te-bias-n` dispatcht
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `te-bias-n`-Lauf `38147870533` (dispatcht 2026-10-11)
- **Lage:** (gemessen 2026-10-11 via folge315-Register) (folge315, unverändert) `enum NullKind { Phase, Arx }`, `ARX_NULL_LAG = 12` → `arx_restricted_surrogate`; die vier Phasen-Aufrufe byte-identisch, ein AR(1)-Arm misst die vier Richtungen separat. Riss im Paper (Abstract:17, §3.5:254-256, §7:867-868).
- **Blockade:** CI-Lauf.
- **Braucht:** `38147870533` lesen (`ci_manage log 38147870533`); dann entscheiden, ob der KSG-Arm mit der strukturerhaltenden Null beide Gates hält.

### LiDAR-Inventar / Extraterrestrisch — MLA-RDR gemessen (Code-Änderung), Coverage-Blöcke gesetzt
- **Status:** eigen (Bau) | **Bindung:** eigen
- **Trigger:** der nächste begrenzte Schritt (MLA-Route + Coverage-Register)
- **Lage:** (gemessen 2026-10-11 via `archive_search --verdict` + `sread`) Extraterrestrisch registriert: MOLA `field MARS_RADIUS mola_mars_radius` · NLR `field RANGE nlr_range`/`nlr_eros_radius` · OLA `field radius ola_radius` · Hayabusa `field RANGE hay_lidar_range`. **MLA** (`format pds3_fixed_width`, CDR-`origin`): der **RDR ist erreichbar** — `…/messmla_2001/rdr_radr/` HTTP 200 (die Vermutung `…/rdr/` war 404), und `label/mlascirdr.fmt` trägt `NAME = RANGE` (Spalte 11, `F10.4`, Unit KM); Blocker ist der **hartkodierte `CDR_ROUTE`** in `tools/harvest/src/bin/mla_compiler.rs:10` (walkt nur `cdr/2004/aug/`), kein fehlender Zugang. Terrestrische Coverage: ahn4/swisstopo/linz/ign_lidarhd registriert; **drei fehlende Blöcke in diesem Atom gesetzt** — `coverage_tnm` (`tnmaccess.nationalmap.gov`, terms `PD`) · `ea_lidar_index` (`environment.data.gov.uk`, OGL-UK-3.0) · `lola_coverage` (`astrogeo-ard.s3.amazonaws.com`, `at moon`, terms `PD`) — `register_sort` canonical über **2789** Blöcke. **Offen:** `opentopography_coverage` (`portal.opentopography.org/API/otCatalog?outputFormat=json`, `format coverage_manifest`) — Lizenz **per-Dataset**, ein einzelnes `terms` überzeichnet → Riss. Tatsächliche Punktdaten: nur `open-lidar-data` BE (`format las`) + `usgs-lidar-public` BrooksCamp; die terrestrischen Portale sind **coverage-only** (Daten descoped, folge315).
- **Blockade:** MLA ist eine **Code-Änderung** (Route auf `rdr_radr/` + `RANGE`-Mapping), kein Register-Akt; die opentopography-Lizenz ist **per-Dataset** (ein einzelnes `terms` überzeichnet) → Riss.
- **Braucht:** (a) `mla_compiler.rs` `CDR_ROUTE` → RDR-Route (`--route`-Flag/Repoint) + `field RANGE mla_range em km`, `cargo build -p omegaflow-harvest --bin mla_compiler`, Block-`origin` auf `rdr_radr`. (b) `opentopography_coverage`: Lizenz-Verdikt (per-Dataset → `terms`-Modell) oder nur `archive_search`-Arm.

### Sechs Klima-Quellen — ecad/aodn brauchen gewählte Eingaben
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** eine gewählte ecad-`station` bzw. aodn-`url`
- **Lage:** (gemessen 2026-10-11 via Workflow-Read) surfrad `38142701271` · dwd-cdc `38142703857` · worldclim `38142706143` · knmi `38142709395` dispatcht; `ecad-cdn.yml` verlangt `element`+`station`+`lat`/`lon`/`alt` (per-Akt-Input, kein Default), `aodn-cdn.yml` eine OPeNDAP-`url`.
- **Blockade:** die Station/das Dataset ist eine wissenschaftliche Wahl, kein Default.
- **Braucht:** je eine gewählte ECA&D-Station und eine AODN-OPeNDAP-URL → `gh workflow run ecad-cdn.yml -f station=… -f lat=… -f lon=… -f alt=…` / `aodn-cdn.yml -f url=…`.

### MODVOLC/ITS_LIVE/MASIE — cdn-Workflows (Mycelium)
- **Status:** wartend | **Bindung:** linie:mycelium
- **Trigger:** je ein `*-cdn.yml`-Lauf
- **Lage:** (gemessen 2026-10-11 via `git status`) die drei Workflows `modvolc-cdn.yml` · `its_live-cdn.yml` · `masie-cdn.yml` liegen **untracked** im Baum (parallel erzeugt, Formula-Header `# harvest: asset=…`); die Compiler sind registriert (`phi:32152`/`32161`/`1619`).
- **Blockade:** Workflows nicht committet/gelaufen.
- **Braucht:** Mycelium committet + dispatcht sie (s. `## An mycelium`).

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
- **Lage:** (gemessen 2026-10-11 via `sread` + `cargo check`; Träger `docs/concepts/eigene-ephemeride.md`) **Step 0 (Operator/Kalibration): teil** — Zeit `src/archivar/lsk.rs unix_to_tdb` ✓, Rahmen `src/mathematikerin/receiver.rs` (`precession_matrix_fw06`/`nutation_angles_2000a` 1365-Term/`cirs_to_gcrs`/`itrf_to_icrs`) ✓, Lichtzeit+Shapiro `receiver.rs two_way_doppler`/`shapiro_leg_s` ✓ — **aber `two_way_doppler` wird von keinem Bin gerufen** (nur Tests); die Residuen-Tools nutzen den alten Teil-Pfad `odp.rs downlink_rate_core` (downlink only, kein Shapiro/EOP). **Step 0e Medien/Troposphäre: absent** (VMF3/IONEX nur als Harvest-Compiler). **Step 1/2: teil** (Pioneer-Residuen-Tools, je hartkodiert `const SC_BODY`). **Step 3 vorher absent → in diesem Atom gebaut:** `tools/measure/src/bin/witness_odf_residuum.rs` (Registry `witness_source` + `--body`; pioneer10/galileo_e1 registriert; `cargo check` + build 0/0). **Step 4: `least_squares.rs` gebaut, nicht an ein Bahn-Residuum verdrahtet.** Riss: Konzept/Handover nennen `observer.rs` — die Datei heißt seit `55e06c189` `receiver.rs`.
- **Blockade:** der volle Operator passt nicht ohne Adapter — `two_way_doppler` liefert `observable_hz` (nicht range-rate), braucht `turn_ratio` + einen deklarierten Sonnen-Gravitator + `t_tx` aus der Lichtzeit-Iteration (nicht exponiert).
- **Braucht:** `t1` aus dem Lichtzeit-Helper zurückgeben, `turn_ratio` + `ephemeris_sun.bin`-Position/`GM_SUN_M3_S2` deklarieren, dann den Fit auf `observable_hz` umstellen (nächster begrenzter Schritt); parallel Step 0e Medien (VMF3) an die Kette binden. PEP/Tudat bleiben Lizenz-/Verfahrens-Referenz.

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

### api.sensor.community — zulassen, Zielzeile unbenannt
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** das wörtliche Future-Wort (welcher Arm/welche Zeile)
- **Lage:** (gemessen 2026-10-11 via folge315-Register) (folge315, unverändert) HTTP 200; `sensor.community` bereits registriert (`phi:338/1218`).
- **Blockade:** „zulassen" nennt keinen Arm/keine Zeile.
- **Braucht:** das wörtliche Future-Wort.

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
- **Braucht:** je Quelle die Verdikt-`terms`-Zeile (Mountain) + url/origin (Mycelium); FMI/BPA/BGS GIC restlich messen.

### Re-Admission lizenz-blockierter Kandidaten (future-233)
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-11 durch Taucher) **Riss im Survey:** `docs/surveys/survey-2026-10-09-redistribution-alternativen.md:12` zitiert `phi/declined_sources.φ` (41 Blöcke) — die Datei wurde in `d97197b77` entfernt; der Baum gewinnt. Gemessene Routen: **ALMA** → ESO schon registriert (`phi:21867-21948`, CC BY 4.0), JAO bleibt blockiert; **GRDC** → GRDC-Caravan Zenodo `15349031` `CC BY 4.0` (neu) · USGS NWIS `PD` (schon `phi:475`) · Copernicus GloFAS; **RIPE RIS** → RouteViews `CC BY 4.0` (neu, Riss BY-vs-BY-SA benannt); **IMCCE INPOP/NOE-4** → DE440/441 `PD` (schon `phi:4329-4380`) / EPH-VSOP Zenodo `17142232` `CC BY 4.0` (neu); **TRISP/YBCO-NSE** → NIST `PD` (403, pending) / Zenodo `18306252` `CC BY 4.0` (**Record `Restricted`**, pending); **INTERMAGNET** → Bulk-CDN blockiert; **GIRO DIDBase** → schon registriert (`phi:1066`).
- **Blockade:** url/origin (Mycelium) für die Neu-Admits.
- **Braucht:** vier neue Verdikte/Registrierungen (GRDC-Caravan, RouteViews, EPH-VSOP, GloFAS) — Mountain setzt `terms`, Mycelium url/origin; NIST/Zenodo-restricted bleiben `pending`.

### Asservatenkammer-Survey — vier Marker (mycelium-308)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-11 via mycelium-308-Register) `docs/surveys/survey-2026-10-08-research-api-mcp.md` — vier Marker auf den gemessenen Stand setzen (Perplexity live · Consensus 429 transient · Elicit descoped · SciSpace pending).
- **Blockade:** keine.
- **Braucht:** die vier Marker im Survey setzen.

## An mycelium

Origin: mountain-folge316 (2026-10-11).

- **Drei cdn-Workflows für die neuen Compiler** — `modvolc-cdn.yml`, `its_live-cdn.yml`, `masie-cdn.yml` liegen **untracked** im Baum (`git status`), je mit Formula-Header `# harvest: asset=…`; die Compiler sind registriert (`modis.higp.hawaii.edu/modvolc_thermal.bin` `phi:32152` · `stac.itslive.cloud/its_live_velocity.bin` `phi:32161` · `noaadata.apps.nsidc.org/masie_ice_extent.bin` `phi:1619`). Bitte committen + dispatcht (`--body earth --ci-mode`).
- **DEMETER — Lizenz-Riss:** die behauptete „Licence Etalab-2.0" (`handover-2026-10-11-mycelium-folge308.md:118`) ist am Baum widerlegt (CDPP Data-Centre-Lizenz V1.1, nicht-kommerziell, Weitergabe an Dritte nur mit schriftlicher Zustimmung). Vor url/origin der DEMETER-Serie den CDN-Spiegel gegen die Lizenz prüfen; bei Konflikt nur lokale Registrierung.
- **GIC/Space-Weather + Re-Admission:** die vier Neu-Admits (GRDC-Caravan/RouteViews/EPH-VSOP/GloFAS) brauchen nach Mountains `terms`-Verdikt url/origin.

## Operator-Wort-Register

- **„hä wir bitte du solltest eine taucherstaffel losschicken und du hast dich nur um copernicus gekümmert du sollst alle offenen punkte die die letzte session aufwändig dokumentiert hat umsetzen"** | 2026-10-11 | Operator-Session (mountain).
- **„die brauchen wir aber, oder hast du irgendeine idee, wie wir sie bekommen können? Nur A4 (ESA PSA) und A7 (ESA SPICE) Lizenz bleiben unmeasured"** | 2026-10-11 | Operator-Session (mountain).
- **„bitte lasse eine taucherstaffel auf ALLE offenen punkte los"** | 2026-10-11 | Operator-Session (mountain).
- **„Mindat Level-1 ist descoped ausser es dient als faden aber dann gibt es alternativen"** | 2026-10-11 | Operator-Session (mountain).
- **„388 … bauen"** | 2026-10-11 | Operator-Session (mountain).
- **„MODVOLC: 2000–2019-Archiv admitieren+bauen"** | 2026-10-11 | Operator-Session (mountain).
- **„schicke eine taucherstaffel los und BAU ENDLICH!!!!"** | 2026-10-11 | Operator-Session (mountain).
- **„die weltlinie kennt kein jetzt"** | 2026-10-11 | Operator-Session (mountain).

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort („mach das ab jetzt
automatisch", 2026-10-07) trägt Commit und Push.
