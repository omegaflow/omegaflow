<!--
  title: Handover — Mountain-Folge 314 (2026-10-11)
  session: Mountain-Linie in einem Pass — Taucherstaffel (10 Taucher) über alle offenen Punkte der folge313: Messungen, Riss-Auflösungen, verdikt-reife Vorlagen
  class: handover
  date: 2026-10-11
  sha256: 2fe5c5586c6716404d3929d234d76cce0f990e9c9a4541e30d27d22cd1903cf5
  status: live
-->
# Handover — Mountain-Folge 314 (2026-10-11)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`). Diese
Session konsumierte `handover-2026-10-11-mountain-folge313.md` (→ `archiv/`).

## Burn: open 0.05 · close 0.14 — Grund: Taucherstaffel (8× general, 7× grind-flash) in einem Atom; flash only. Die Breite war das Operator-Wort („Taucherstaffel auf ALLE offenen Punkte"), kein pro/max.

## Offen (aufgeschlüsselt)

### ci-gate — subset (37 FAILED) + dropped-gate
- **Status:** eigen (Messung) | **Bindung:** eigen
- **Trigger:** der nächste `ci-gate`-Lauf am HEAD
- **Lage:** (gemessen 2026-10-11 via `ci_manage log 38131646200`/`38131433493`) `format`/`register`/`build`/`clippy` success; **dropped-gate failure** (`current 2 | pinned 100 | new 2`: `asservatenkammer-survey-marker prosa-reconciliation`, `omegaflow-measure zwei bins rot river-184`); **subset cancelled** nach **37 FAILED** — u. a. `parse::range_directive_*` ×3, `parse::every_cdn_source_carries_its_origin_and_compiler_or_is_a_reference`, `inflate::tar_stream_emits_data_chunks_then_end`, `port::probe_classify_tests::*` ×10, `archivar::tests::*` ×15, `llr`, `vlf_awesome`, `mathematikerin::actuators/mci/pc/receiver`. Die vier in folge313 genannten sind darunter; 33 weitere kamen hinzu (Funktions-Zeilen verschoben: `parse.rs` 2822/2914 statt 2826/2944, `inflate.rs` 1306 statt 1328, `port.rs` 4167 statt 4169).
- **Blockade:** die 37 Test-Ausfälle + dropped-gate; Kern `src/archivar` (Mountain/River).
- **Braucht:** die 37 Ausfälle auf eine Ursache zurückführen (`ci_manage log 38131646200`), droppten-Gate gegen `docs/zustand/dropped-legacy-baseline.txt` prüfen.

### te-ground-truth — AR(1)-Null-Arm gebaut, Neu-Messung offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein `te-bias-n`-Lauf am neuen HEAD
- **Lage:** (gemessen 2026-10-11 via `cargo build -p omegaflow-measure --bin te_ground_truth`: grün, 0 warnings) `tools/measure/src/bin/te_ground_truth.rs` (+74/−9): `enum NullKind { Phase, Arx }`; neuer `embedded_threshold(..., kind, ...)` mit `ARX_NULL_LAG = 12` → `arx_restricted_surrogate` (`te.rs:1172`); die vier Phasen-Aufrufe bleiben byte-identisch, ein neuer AR(1)-Arm misst die vier Richtungen separat. Der Riss ist im Paper bereits gezeichnet (Abstract:17, §3.5:254-256, §7:867-868).
- **Blockade:** CI-Lauf steht aus.
- **Braucht:** `te-bias-n` dispatchen/laufen lassen; dann entscheiden, ob der KSG-Arm mit der strukturerhaltenden Null beide Gates hält.

### LiDAR-Inventar / Extraterrestrisch — Parser gebaut, Registrierung offen
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-11 via `cargo build`: je bin grün, 0 warnings) **MOLA + NLR** als Arme in `pds3_fixed_width_compiler.rs` (+55/−4; `MOLA_ROUTE` = PRDR-Volume, da die PEDR-Volume **keine** `.tab` trägt — `RAMAPPING.FMT` via `^STRUCTURE`; `NLR_ROUTE` = `NEAR_A_NLR_5_CDR_EROS_ORBIT_V1_0/data/topo/`). **MLA** `mla_compiler.rs` (340 Z., 113 numerische Spalten, Live-Lauf 988 Zeilen) und **OLA** `ola_compiler.rs` (419 Z., 21 numerische Spalten, PDS4 `Table_Binary` nativ dekodiert). Hayabusa war bereits gebaut (`phi:19609-19674`).
- **Blockade:** `phi/sources.φ` fremd-modifiziert → die vier `phi`-Blöcke (`at mars`/`at mercury`/`at eros`/`at bennu`) erst bei sauberer Datei.
- **Braucht:** vier `phi`-Blöcke setzen; **neu gemessene Parser-Lücke:** NLR `UTC` = ISO **day-of-year** (`2000-074T…`), `unix_of_iso` erwartet Kalender-ISO → die Zeitachse würde NLR-Zeilen überspringen — in `pds3_table.rs` schließen (eigener begrenzter Schritt).

### Sechs Klima-Quellen — terms/ttl + Serien-Manifest-Format
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** der Serien-Manifest-Entwurf
- **Lage:** (gemessen 2026-10-11) surfrad `ttl 604800` · ecad `604800` · dwd_cdc `2592000` · worldclim `315360000` · aodn `1` (Datensatz-Kadenz, kein `TTL_S`) · knmi `86400`; je Workflow-Default (`*-cdn.yml`). **Kein** Serien-Manifest-Dokument unter `state/`/`docs/`; Präzedenz im Code: `modis-cdn.yml:110` + `gosat-cdn.yml:105` (`series-manifest`-Job), Schema-Notiz `modis-year-split.yml:11`. CI-Wait je Block: `state/zustand/wartend.φ:38`.
- **Blockade:** ohne Format trägt jeder Block nur einen Default-Tag.
- **Braucht:** Serien-Manifest-Format nach dem modis/gosat-Muster entwerfen; `terms`/`ttl` je Compiler gegenprüfen.

### Open-LiDAR — Descope-Verdikte je Portal
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** das Mountain-Verdikt je Portal
- **Lage:** (gemessen 2026-10-11) **NL `ahn4`**: flai ⊇ Origin außer den 2 Kacheln `08BZ1`/`40HZ1` (norm. Diff `coverage_id_diff --normalize basename --strip-prefix c_`) → **Origin behalten** (2 unique Kacheln = Provenanz). **CH `swisstopo`**: STAC >10 000 Items (100-Seiten-Probe endet mit `next`), Sidecar fehlt → **unread**. **FR `ign_lidarhd`**: unread. **NZ `linz`**: bleibt. **`ea_lidar_index`**: der Kompiler emittiert `tile <filename> <bbox> <pt_count>` (`ea_lidar_index.rs:262-274`) → widerlegt die folge305-Zeile „Metadaten-Index"; kein phi-Block.
- **Blockade:** CH/FR brauchen Live-Fetch.
- **Braucht:** `target/debug/swisstopo_coverage --out /tmp/opencode/ch_origin.manifest --emit-ids /tmp/opencode/ch_origin.ids` → awk-ID-Norm (`swisssurface3d_<jahr>_` strippen) → `coverage_id_diff --origin … --mirror /tmp/opencode/ch_flai.ids --normalize basename`; FR `open_lidar_data_coverage --prefix data/FR/ --emit-ids`; dann je Portal Verdikt.

### Riss — adressierte φ-Vorschläge (mycelium-307/305)
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-11) alle vier Portal-Coverage-Blöcke (`phi:11772`/`11105`/`11780`/`11113`) **und** Bayern-DOM (`:31728`)/VNP46A3 (`:2051`)/open-lidar-data (`:11788`) **stehen** — kein Set-Auftrag. Bayern-DOM **deckungsgleich** mit dem Vorschlag. Risse: **VNP46A3** `origin` = LAADS-Verzeichnis statt `.h5`; **open-lidar-data** `terms` CC-BY-3.0 (Vlaanderen) vs. CC0-1.0.
- **Blockade:** `phi/sources.φ` trägt **fremde uncommittete Hunks** (40 deletions, nicht eigene) → Register-Edit erst bei sauberer Datei.
- **Braucht:** VNP46A3-`origin` und open-lidar-data-`terms` je gegen die Quelle messen, dann Verdikt setzen.

### Riss — PDS/ESA-Missionsrouten `at`
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-11) der Riss **löst sich**: ein ODF/ODR/RSR/TNF/RSS-Observable ist der Bodenstation-Empfang (DSN/ESA) — `at earth` ist physikalisch korrekt (alle Tracking-Compiler hart `at earth`; Rosetta-Felder `carrier_level_dbm`/`polar_angle_cycles` = IFMS-AGC am Boden). Die verglichenen Geschwister (`at galileo`/`at cassini`/…) sind eine **andere Datenklasse** (Ephemeride/in-situ/Gravimetrie), kein Widerspruch. **Ein echter Rest:** `voyager1_merged`/`voyager2_merged` (`phi:19831`/`:19843`) tragen In-situ-Felder (B, speed, density, temp) → `at earth` falsch; Geschwister `wind_waves` nutzt `at wind` → **`at voyager1`/`at voyager2`**.
- **Blockade:** `phi/sources.φ` fremd-modifiziert (s. o.).
- **Braucht:** `at voyager1`/`at voyager2` setzen, sobald das Register sauber ist.

### SuperMAG-Derivate — Keep-Entscheid
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-11) `sgrep -i substorm src tools` = Konsumenten **vorhanden**: `src/archivar/substorm.rs`, `tools/measure/src/bin/substorm_compare.rs`, `gic_onset_detector.rs`, `nobel_probe_bz.rs`. Ein Abstoßen würde lebende Konsumenten orpharen → **Keep**.
- **Blockade:** keine.
- **Braucht:** Verdikt `keep` in `phi/sources.φ` (den Future-Abstoß-Vorschlag zurückweisen); CDN-Release bleibt.

### future-232 — MODVOLC/ITS_LIVE/MASIE Verdikt
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** das adressierte Future-Wort
- **Lage:** (gemessen 2026-10-11) **MODVOLC**: `…/data/2024|2025/` = **404**; lebende Abdeckung endet 2019 (`…/data/2019/365/` 200, `MYDVOLC.A…`) → **Riss** zur Future-Evidence (die 200 nannte); nicht registriert. **ITS_LIVE**: `stac.itslive.cloud` **200** (STAC 1.0.0), S3-catalog **200** (sha256 `5d2a6ada…`); nicht registriert. **MASIE**: `phi:1619` **bereits** (`url`/`terms PD`/`at earth`/`ttl 86400`), aber **unvollständig** (kein `format`/`origin`/`field`).
- **Blockade:** MODVOLC jahrgangsaktuelle Route absent.
- **Braucht:** MODVOLC-Verdikt (nur 2000-2019 admitieren oder `descoped` mit Befund); ITS_LIVE-Block setzen; MASIE-Block ergänzen.

### copernicus B1–B6 — Workflows angelegt, Lauf offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der erste `copernicus-<dataset>-cdn`-Lauf je Dataset
- **Lage:** (gemessen 2026-10-11 via `git status`: sechs Workflows neu angelegt) `copernicus-{surface-land,uscrn,igra,woudc,gruan,gnss}-cdn.yml` (je `--year`/`--area`/`--out`/`--lsk kernels/naif0012.tls`/`--ci-mode`; `CDS_API_KEY`, netloc `cds.climate.copernicus.eu`). Bins gebaut/grün. Kein Lauf dispatcht.
- **Blockade:** kein Lauf.
- **Braucht:** die sechs Workflows dispatchen; bei Void Variable/Version korrigieren. Riss: `uscrn` `time_aggregation=daily` / `gnss` `network_type=igs_daily` sind Wahlen ohne Default.

### NOAA S3-Compiler B7/B8 — Workflows angelegt, Lauf offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der erste `noaa-nos-lidar`/`noaa-eri`-CDN-Lauf
- **Lage:** (gemessen 2026-10-11 via `git status`: zwei Workflows neu angelegt) `noaa-nos-lidar-cdn.yml` + `noaa-eri-cdn.yml` nach `noaa-nrs-psd-cdn.yml`/`noaa-ocs-hydrodata-cdn.yml`; Bins gebaut/grün. Kein Lauf dispatcht.
- **Blockade:** kein Lauf.
- **Braucht:** die zwei Workflows dispatchen.

### Teil C — A5/A6 Earth-Flyby-ODFs
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-11) **A5** Galileo Earth1/Earth2-Tracking **gefunden als TDF** — `GO-SS-RSS-1-TDF-V1.0` (PDS PPI, `/annex/…`; 1990 DOY 342-344 + 1992 DOY 343-344; **keine** `.ODF`). **A6** MESSENGER 2005 Earth-Flyby **absent** — die RS-Rohbündel `urn-nasa-pds-mess-rs-raw` (`data-odf`/`data-rsr`) beginnen 2006/2007.
- **Blockade:** keine.
- **Braucht:** A5 die TDF-Route als Quelle registrieren (oder `descoped` mit Befund); A6 `descoped` mit Befund (andere Quelle erschöpft).

### Manifestation SPT-3G D1 — sha256
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein `cmb-cdn`-Lauf abgeschlossen
- **Lage:** (gemessen 2026-10-11) `cmb-cdn 38129785553` = **completed failure** (HEAD `82ec4f949`); sha256 fehlt in `phi/sources.φ`.
- **Blockade:** Lauf rot.
- **Braucht:** `ci_manage log 38129785553` → Ursache; danach sha256 nachtragen.

### Frame/Epoche — Transform-Layer
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Architektur-Entscheid der Rat-Linse (Vorlage `state/mycelium/frame-epoch-findings.md`)
- **Lage:** (gemessen 2026-10-11) bbox-off Default steht; Transform-Layer `pending`.
- **Blockade:** Architektur.
- **Braucht:** Rat-Linse über den Transform-Layer.

### PEP + Tudat — Schritt 3 (mehrere Bahnen)
- **Status:** eigen (Register + Bau) | **Bindung:** eigen
- **Trigger:** Schritt 3 — weitere Zeugen (Merkur/Venus Radar + VEX-Ranging) durch die Kette
- **Lage:** (gemessen 2026-10-11) vier Häuser als `ephemeris_*` registriert.
- **Blockade:** keine.
- **Braucht:** weitere Zeugen durch die Kette, Kette CI-nah verankern.

### particle-cern — Baskets + Sample-Verifikation
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der nächste `subset`-Lauf (CI)
- **Lage:** (gemessen 2026-10-11) Basket-Arrays gebaut; `cargo test` läuft nur in CI.
- **Blockade:** `subset` hängt (Punkt ci-gate).
- **Braucht:** `subset` grün; ggf. ein v9-only-Sample.

### Exposom-Matrix → TE-Paar-Feed — Repräsentativpunkt
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** extern gedeckter Repräsentativpunkt je Zeile (Operator-Wort 2026-10-10)
- **Lage:** (gemessen 2026-10-11, unverändert) kein offener Datensatz trägt eine im Datensatz gemessene Koordinate.
- **Blockade:** der Repräsentativpunkt ist eine wissenschaftliche Annahme, keine Messung.
- **Braucht:** je Zeile den extern gedeckten Repräsentativpunkt → eine Zeile in `phi/sources.φ` → TE-Paar-CI-Feed.

### iEEG — registriertes Wort maßgeblich
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein neues Operator-Wort über 2026-10-06 hinaus
- **Lage:** (gemessen 2026-10-11) iEEG = privates Experiment, kein CDN.
- **Blockade:** keine.
- **Braucht:** nur ein neues Operator-Wort öffnet es. **Nicht erneut vorlegen** (Wort registriert).

### api.sensor.community — zulassen, Zielzeile unbenannt
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** das wörtliche Future-Wort (welcher Arm/welche Zeile) — `state/zustand/wartend.φ`
- **Lage:** (gemessen 2026-10-11) HTTP 200; `sensor.community` bereits registriert (`phi:338/1218`).
- **Blockade:** „zulassen" nennt keinen Arm/keine Zeile.
- **Braucht:** das wörtliche Future-Wort.

### Flyby-Kette — termin 2026-11-01
- **Status:** termin | **Bindung:** termin:2026-11-01
- **Trigger:** ESOC-Recon-Release (oder Descope)
- **Lage:** (gemessen 2026-10-11) die JUICE-`orbc`-SPK (A7) ist registriert; `doppler.rs` absent.
- **Blockade:** kein ESOC-Recon-Release.
- **Braucht:** ESOC-Release oder Descope-Befund für `doppler.rs`.

### FMHY-Routing — Riss 4 bot-gated
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Riss 4/5-Abschluss — Overpass bot-gated (gemessen 2026-10-10)
- **Lage:** (gemessen 2026-10-10) Overpass HTTP 406 direkt+Proton → bot-gated.
- **Blockade:** Overpass bot-gated (HTTP 406 direkt+Proton).
- **Braucht:** Riss-4-Abschluss oder Descope.

## Operator-Wort-Register

- **„hä wir bitte du solltest eine taucherstaffel losschicken und du hast dich nur um copernicus gekümmert du sollst alle offenen punkte die die letzte session aufwändig dokumentiert hat umsetzen"** | 2026-10-11 | Operator-Session (mountain) — die Taucherstaffel über den gesamten Wartend-Auftrag losschicken, nicht nur einen Punkt.
- **„die brauchen wir aber, oder hast du irgendeine idee, wie wir sie bekommen können? Nur A4 (ESA PSA) und A7 (ESA SPICE) Lizenz bleiben unmeasured"** | 2026-10-11 | Operator-Session (mountain) — A4/A7-Lizenz ist nötig; gemessen: ESA Science Archives = CC BY-NC 3.0 IGO. Lücke geschlossen.
- **„bitte lasse eine taucherstaffel auf ALLE offenen punkte los"** | 2026-10-11 | Operator-Session (mountain) — die Taucherstaffel auf alle offenen Punkte der folge313; diese Session hat 10 Taucher dispatcht und jeden Punkt gemessen/zu seiner Kante gearbeitet.

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort („mach das ab jetzt
automatisch", 2026-10-07) trägt Commit und Push.
