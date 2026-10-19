<!--
  title: Handover — Mountain-Folge 314 (2026-10-11)
  session: Mountain-Linie in einem Pass — Taucherstaffel (10 Taucher) über alle offenen Punkte der folge313: Messungen, Riss-Auflösungen, verdikt-reife Vorlagen
  class: handover
  date: 2026-10-11
  sha256: e0bf0e78562e6ec09994198b3f9962e376d2db1d955440c6c88715864610e624
  status: live
-->
# Handover — Mountain-Folge 314 (2026-10-11)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/standing-pass.md`). Diese
Session konsumierte `handover-2026-10-11-mountain-folge313.md` (→ `archiv/`).

## Burn: open 0.05 · close 0.14 — Grund: Taucherstaffel (8× general, 7× grind-flash) in einem Atom; flash only. Die Breite war das Operator-Wort („Taucherstaffel auf ALLE offenen Punkte"), kein pro/max.

## Offen (aufgeschlüsselt)

### ci-gate — subset (37 FAILED) + dropped-gate
- **Status:** eigen (Messung) | **Bindung:** eigen
- **Trigger:** der nächste `ci-gate`-Lauf am HEAD
- **Lage:** (gemessen 2026-10-11 via `ci_manage log 38131646200` + die `ci-check`-Panics) `subset` ist **cancelled**, nicht von den 37 gefällt: der Live-Netz-Sweep `test_live_sources_extract` (`tests.rs:8234`) hängt und wird am 30-min-Cap gekappt — **bereits behoben** (`7e6e39650`, `#[ignore]`). Die **37 FAILED** sind heterogen: **A** (~21) Register-Fiktion — die `phi/sources.φ`-Umstrukturierung Mountain 306/310 (`94129c942`, `1f2d4e2c4`, `ff6f4315f`, `e87398886`) entfernte `field`/`origin`/`compiler`-Zeilen, die Tests erwarten den alten Stand (`port.rs:1798` `register_field_map`, `parse.rs:2914`); **B** (3) `range`-only-Quellen wurden vom `flush!()`-Gate verworfen — **behoben** (`6014fafc8`, `|| cur_range.is_some()`, `parse.rs:126`); **C** blinde Tests (`inflate::tar_stream` `b0c9227e7`, `llr` `df0579550`, `vlf` `0d12e17d0`, `receiver::nutation` `f95350d25`, `pc`/`mci`/`actuators` river-142). dropped-gate: `current 2 | pinned 100 | new 2`.
- **Blockade:** Gruppe A (Test-Erwartungen vs. neuer Register-Stand) + Gruppe C (nie gegated); `cargo test` läuft nur in CI.
- **Braucht:** A — `port.rs:1798`/`parse.rs:2914` an den neuen Register-Stand anpassen (21 Tests); C — je Test die Zahl aus einem neuen `subset`-Lauf lesen; dropped-gate gegen `docs/zustand/dropped-legacy-baseline.txt` prüfen.

### te-ground-truth — AR(1)-Null-Arm gebaut, Neu-Messung offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein `te-bias-n`-Lauf am neuen HEAD
- **Lage:** (gemessen 2026-10-11 via `cargo build -p omegaflow-measure --bin te_ground_truth`: grün, 0 warnings) `tools/measure/src/bin/te_ground_truth.rs` (+74/−9): `enum NullKind { Phase, Arx }`; neuer `embedded_threshold(..., kind, ...)` mit `ARX_NULL_LAG = 12` → `arx_restricted_surrogate` (`te.rs:1172`); die vier Phasen-Aufrufe bleiben byte-identisch, ein neuer AR(1)-Arm misst die vier Richtungen separat. Der Riss ist im Paper bereits gezeichnet (Abstract:17, §3.5:254-256, §7:867-868).
- **Blockade:** CI-Lauf steht aus.
- **Braucht:** `te-bias-n` dispatchen/laufen lassen; dann entscheiden, ob der KSG-Arm mit der strukturerhaltenden Null beide Gates hält.

### LiDAR-Inventar / Extraterrestrisch — Parser + vier Blöcke gebaut
- **Status:** eigen (Bau) | **Bindung:** eigen
- **Trigger:** der nächste begrenzte Schritt
- **Lage:** (gemessen 2026-10-11 via `cargo build`: je bin grün, 0 warnings) **MOLA + NLR** als Arme in `pds3_fixed_width_compiler.rs`; **MLA** `mla_compiler.rs` (340 Z., 113 num. Spalten), **OLA** `ola_compiler.rs` (419 Z., 21 num. Spalten). Die vier Quellen sind **registriert** (`phi:31707-31917`: MOLA/NLR/OLA je ein Registry-taugliches `em m`/`km`-Range-Feld, MLA ohne Feld — s. Blockade). Die NLR-DOY-Lücke ist **behoben** (`b334bf278`, `unix_of_iso` parst `YYYY-DDDT…`). Hayabusa war bereits gebaut.
- **Blockade:** MLA trägt **kein** Messfeld — das getestete CDR-Label ist reine Housekeeping-Telemetrie (V/degC), kein `RANGE`; das RDR trägt die Bereiche.
- **Braucht:** MLA-RDR-Label als Quelle (statt CDR); optional weitere registry-taugliche Felder (MOLA `MARS_RANGE`, NLR `SCDST`, je `em m`) nachtragen.

### Sechs Klima-Quellen — Serien-Manifest gebaut, terms/ttl gegenprüfen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der erste `series-manifest`-Lauf je Klima-Workflow
- **Lage:** (gemessen 2026-10-11 via Workflow-Diff) die sechs Klima-Workflows tragen je einen `series-manifest`-Job (nach `modis-cdn.yml:110`): `surfrad-cdn.yml`, `ecad-cdn.yml`, `dwd-cdc-cdn.yml`, `worldclim-cdn.yml`, `aodn-cdn.yml`, `knmi-cdn.yml` — faltet die Shard-Assets je Familie in `<familie>.manifest` (`name sha256:<hex>`, `sort -u`, 0 honored). Shard-Namen aus den Compilern gemessen (`surfrad_`/`ecad_`/`dwd_cdc_`/`worldclim_`/`aodn_`/`knmi_`). `terms`/`ttl`: surfrad `604800` · ecad `604800` · dwd_cdc `2592000` · worldclim `315360000` · aodn `1` (Datensatz-Kadenz) · knmi `86400`.
- **Blockade:** kein Lauf (die sechs Jobs existieren, keiner dispatcht).
- **Braucht:** je Workflow den ersten Lauf starten; `terms`/`ttl` gegen den Compiler gegenprüfen.

### Open-LiDAR — Descope-Verdikte je Portal (gemessen)
- **Status:** descoped | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) **NL `ahn4`**: flai ⊇ Origin außer `08BZ1`/`40HZ1` (2/1375) → **Origin behalten**. **CH `swisstopo`**: flai deckt **39515/41982 = 94,1 %** (56843 STAC-IDs → 41982 unique; norm. Diff missing **2468**) → flai **partial** → **Origin behalten**. **FR `ign_lidarhd`**: flai 337207 vs Origin 524687, id-**disjunkt** (Aggregat hält nur `data/FR/IGN/Lidar_2021/`) → **Origin behalten**. **NZ `linz`**: bleibt. **`ea_lidar_index`**: emittiert `tile`-bbox-Zeilen (`ea_lidar_index.rs:262-274`), kein phi-Block.
- **Blockade:** keine.
- **Braucht:** — (kein Portal wird descoped; flai ist nirgends ein Superset).

### Adressierte φ-Vorschläge (mycelium-307/305) — verdiktet
- **Status:** descoped | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) alle vier Portal-Coverage-Blöcke (`phi:11772`/`11105`/`11780`/`11113`) **und** Bayern-DOM (`:31728`)/VNP46A3 (`:2051`)/open-lidar-data (`:11788`) stehen; Bayern-DOM deckungsgleich. Die zwei Werte-Risse sind **entschieden**: **VNP46A3** `origin` bleibt das Verzeichnis `…/prod-lads/VNP46A3/` — der Workflow (`vnp46a3-cdn.yml:29`) löst per CMR den **neuesten** Granule auf, ein festes `.h5` wäre ein veralteter Snapshot. **open-lidar-data** `terms` bleibt `CC-BY-3.0` (Vlaanderen `Gratisopendatalicentie` v1.2, `phi:11789`).
- **Blockade:** keine.
- **Braucht:** —

### PDS/ESA-Missionsrouten `at` — geklärt
- **Status:** descoped | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) der Riss **löste sich**: ODF/ODR/RSR/TNF/RSS = Bodenstation-Empfang → `at earth` korrekt (Compiler hart `at earth`; Rosetta-AGC am Boden). Die Körper-Frame-Geschwister sind eine andere Datenklasse. Der eine echte Rest — `voyager1_merged`/`voyager2_merged` (In-situ-Felder B/Dichte/Temp) — ist **korrigiert** (`at voyager1`/`at voyager2`, `phi:19835`/`:19847`).
- **Blockade:** keine.
- **Braucht:** —

### SuperMAG-Derivate — descoped (River, Redistribution)
- **Status:** descoped | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) River committete `599cfd45b`: die 5 `substorm_{forsyth,frey,liou,newell,ohtani}`-Blöcke sind aus `phi/sources.φ` entfernt — Befund **„cannot be redistributed"** (Lizenz). Die vier Konsumenten (`src/archivar/substorm.rs`, `substorm_compare`, `gic_onset_detector`, `nobel_probe_bz`) verlieren ihre `.bin`-Quelle; `gic_onset_detector --null` ist im selben Commit.
- **Blockade:** keine.
- **Braucht:** — (River-Verdikt maßgeblich; der Konsumenten-Orphan ist mit dem Commit getragen).

### future-232 — MODVOLC/ITS_LIVE/MASIE Verdikt
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** das adressierte Future-Wort
- **Lage:** (gemessen 2026-10-11) **MODVOLC**: `…/data/2024|2025/` = **404**; lebende Abdeckung endet 2019 (`…/data/2019/365/` 200, `MYDVOLC.A…`) → **Riss** zur Future-Evidence (die 200 nannte); nicht registriert. **ITS_LIVE**: `stac.itslive.cloud` **200** (STAC 1.0.0), S3-catalog **200** (sha256 `5d2a6ada…`); nicht registriert. **MASIE**: `phi:1619` **bereits** (`url`/`terms PD`/`at earth`/`ttl 86400`), aber **unvollständig** (kein `format`/`origin`/`field`).
- **Blockade:** MODVOLC jahrgangsaktuelle Route absent.
- **Braucht:** MODVOLC-Verdikt (nur 2000-2019 admitieren oder `descoped` mit Befund); ITS_LIVE-Block setzen; MASIE-Block ergänzen.

### copernicus B1–B6 — Workflows angelegt, Lauf offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der erste `copernicus-<dataset>-cdn`-Lauf je Dataset
- **Lage:** (gemessen 2026-10-11 via `git status`: sechs Workflows neu angelegt; dispatcht via `gh workflow run`) `copernicus-{surface-land,uscrn,igra,woudc,gruan,gnss}-cdn.yml` (je `--year`/`--area`/`--out`/`--lsk kernels/naif0012.tls`/`--ci-mode`; `CDS_API_KEY`, netloc `cds.climate.copernicus.eu`). Läufe dispatcht: uscrn `38136104626`, surface-land `38136110423`, igra `38136112567`, woudc `38136114532`, gruan `38136116720`, gnss `38136118542`.
- **Blockade:** kein Lauf.
- **Braucht:** die sechs Workflows dispatchen; bei Void Variable/Version korrigieren. Riss: `uscrn` `time_aggregation=daily` / `gnss` `network_type=igs_daily` sind Wahlen ohne Default.

### NOAA S3-Compiler B7/B8 — Workflows angelegt, Lauf offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der erste `noaa-nos-lidar`/`noaa-eri`-CDN-Lauf
- **Lage:** (gemessen 2026-10-11 via `git status`: zwei Workflows neu angelegt; dispatcht via `gh workflow run`) `noaa-nos-lidar-cdn.yml` + `noaa-eri-cdn.yml` nach `noaa-nrs-psd-cdn.yml`/`noaa-ocs-hydrodata-cdn.yml`; Bins gebaut/grün. Läufe: nos-lidar `38136120891`, eri `38136123058`.
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

### Exposom-Register — geschlossen (Baum trägt die Felder)
- **Status:** descoped | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) die Wartend-Zeile „`field`-Zeilen je Exposom-Klasse fehlen" ist **stale**: `osm_pbf` `field osm_node` (`phi:2080`) und `eionet_cdr` 276 `field`-Zeilen (`phi:21551-21834`, 92 Pollutant × 3 Medium) stehen (committet Mountain 253 `295c731a8` / 268 `4b01d9166`) und spiegeln die Compiler-Komponenten exakt. Zeile aus `state/wartend.φ` entfernt.
- **Blockade:** keine.
- **Braucht:** —

### Mindat Level-1 — descoped (Operator-Wort)
- **Status:** descoped | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) kein keyless Bulk-Pfad (OpenMindat tokenpflichtig, `MINDAT_TOKEN` Level 0); der Operator descopte es (Alternativen für Mineralkunde existieren). Zeile aus `state/wartend.φ` entfernt.
- **Blockade:** keine.
- **Braucht:** —

### iEEG — registriertes Wort maßgeblich
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein neues Operator-Wort über 2026-10-06 hinaus
- **Lage:** (gemessen 2026-10-11) iEEG = privates Experiment, kein CDN.
- **Blockade:** keine.
- **Braucht:** nur ein neues Operator-Wort öffnet es. **Nicht erneut vorlegen** (Wort registriert).

### api.sensor.community — zulassen, Zielzeile unbenannt
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** das wörtliche Future-Wort (welcher Arm/welche Zeile) — `state/wartend.φ`
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
- **„Mindat Level-1 ist descoped ausser es dient als faden aber dann gibt es alternativen"** | 2026-10-11 | Operator-Session (mountain) — Mindat Level-1 descoped; die Wartend-Zeile entfernt.

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort („mach das ab jetzt
automatisch", 2026-10-07) trägt Commit und Push.
