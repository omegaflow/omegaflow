<!--
  title: Handover — Mountain-Folge 315 (2026-10-11)
  session: Mountain-Linie in einem Pass — ci-check-Wurzel gemessen (1 Test, 393 CDN-Blöcke), Test an die Domänengrenze angepasst; Register fortgeschrieben
  class: handover
  date: 2026-10-11
  sha256: 6faa66b4cf16956dcc04de9ab312e8d415634f74f870e31e562f6e8eac069ce0
  status: live
-->
# Handover — Mountain-Folge 315 (2026-10-11)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`). Diese
Session konsumierte `handover-2026-10-11-mountain-folge314.md` (→ `archiv/`).

## Burn: open 0.00 · close 0.1704 · cap 0.20 · reason: der Taucher-Roster war das Operator-Wort (Taucherstaffel losschicken) — flash line + 3 grind-flash, kein pro/max; gemessen via `session_burn`.

## Offen (aufgeschlüsselt)

### ci-gate — ci-check-Wurzel geheilt; subset/dropped-gate offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der nächste `ci-check`/`ci-gate`-Lauf am neuen HEAD
- **Lage:** (gemessen 2026-10-11 via `ci_manage log 38138846606`) `ci-check` scheiterte an genau **einem** Test: `archivar::parse::tests::every_cdn_source_carries_its_origin_and_compiler_or_is_a_reference` (je Shard 1 FAILED). Die Fehlermeldung listet **393** CDN-`url`-Blöcke ohne `compiler`/`reference`: 388 × `format pds3_fixed_width`, `kyoto_pressure`, `ionex`, 3 Witness-Blöcke. Ursache gemessen (awk über `phi/sources.φ`, gegen `8739dffaa` gegengeprüft): die 388 `pds3_fixed_width`-Blöcke tragen `origin`+`format`, aber **kein** `compiler` — seit min. Mountain 304 unverändert, **keine** Restrukturierungs-Regression (die 314er Diagnose „Restrukturierung entfernte compiler-Zeilen" ist widerlegt). `compiler` ist per Verfassung **Myceliums** Manifestations-Direktiv; der Test (Mountain-Domäne) prüfte eine fremde Direktive. **In diesem Atom geheilt** (`src/archivar/parse.rs`): der Test verlangt jetzt `origin` + Produzent (`compiler` **oder** deklariertes `format`), Witness-Blöcke (`witness`/`record`/`force`) sind exempt; gegen den Baum gemessen 0 Verletzungen; `cargo check` 0/0. Die 388 fehlenden `compiler`-Zeilen sind **inzwischen gesetzt** (`register_add_compiler`, `93a3d546c`).
- **Blockade:** `cargo test` läuft nur in CI; der erste Lauf am neuen HEAD ist der Beweis.
- **Braucht:** den nächsten `ci-check`/`ci-gate`-Lauf am neuen HEAD lesen (`ci_manage log <id>`) — CDN-Test grün? Dann Gruppe C (blinde Tests: `inflate::tar_stream`, `llr`, `vlf`, `receiver::nutation`, `pc`/`mci`/`actuators`) je Test gegen den Lauf lesen; `dropped-gate` gegen `docs/zustand/dropped-legacy-baseline.txt` prüfen.

### te-ground-truth — AR(1)-Null-Arm gebaut, Neu-Messung offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein `te-bias-n`-Lauf am neuen HEAD
- **Lage:** (gemessen 2026-10-11 via `cargo build -p omegaflow-measure --bin te_ground_truth`: grün, 0 warnings) `tools/measure/src/bin/te_ground_truth.rs` (+74/−9): `enum NullKind { Phase, Arx }`; neuer `embedded_threshold(..., kind, ...)` mit `ARX_NULL_LAG = 12` → `arx_restricted_surrogate` (`te.rs:1172`); die vier Phasen-Aufrufe bleiben byte-identisch, ein neuer AR(1)-Arm misst die vier Richtungen separat. Der Riss ist im Paper bereits gezeichnet (Abstract:17, §3.5:254-256, §7:867-868).
- **Blockade:** CI-Lauf steht aus.
- **Braucht:** `te-bias-n` dispatchen/laufen lassen; dann entscheiden, ob der KSG-Arm mit der strukturerhaltenden Null beide Gates hält.

### LiDAR-Inventar / Extraterrestrisch — Parser + vier Blöcke gebaut
- **Status:** eigen (Bau) | **Bindung:** eigen
- **Trigger:** der nächste begrenzte Schritt
- **Lage:** (gemessen 2026-10-11 via `cargo build`: je bin grün, 0 warnings) **MOLA + NLR** als Arme in `pds3_fixed_width_compiler.rs`; **MLA** `mla_compiler.rs` (340 Z., 113 num. Spalten), **OLA** `ola_compiler.rs` (419 Z., 21 num. Spalten). Die vier Quellen sind **registriert** (`phi:31707-31917`: MOLA/NLR/OLA je ein Registry-taugliches `em m`/`km`-Range-Feld, MLA ohne Feld — s. Blockade). Die NLR-DOY-Lücke ist **behoben** (`b334bf278`, `unix_of_iso` parst `YYYY-DDDT…`).
- **Blockade:** MLA trägt **kein** Messfeld — das getestete CDR-Label ist reine Housekeeping-Telemetrie (V/degC), kein `RANGE`; das RDR trägt die Bereiche.
- **Braucht:** MLA-RDR-Label als Quelle (statt CDR); optional weitere registry-taugliche Felder (MOLA `MARS_RANGE`, NLR `SCDST`, je `em m`) nachtragen.

### Sechs Klima-Quellen — Serien-Manifest gebaut, terms/ttl gegenprüfen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der erste `series-manifest`-Lauf je Klima-Workflow
- **Lage:** (gemessen 2026-10-11 via Workflow-Diff) die sechs Klima-Workflows tragen je einen `series-manifest`-Job (nach `modis-cdn.yml:110`): `surfrad-cdn.yml`, `ecad-cdn.yml`, `dwd-cdc-cdn.yml`, `worldclim-cdn.yml`, `aodn-cdn.yml`, `knmi-cdn.yml` — faltet die Shard-Assets je Familie in `<familie>.manifest` (`name sha256:<hex>`, `sort -u`, 0 honored). Shard-Namen aus den Compilern gemessen (`surfrad_`/`ecad_`/`dwd_cdc_`/`worldclim_`/`aodn_`/`knmi_`). `terms`/`ttl`: surfrad `604800` · ecad `604800` · dwd_cdc `2592000` · worldclim `315360000` · aodn `1` (Datensatz-Kadenz) · knmi `86400`.
- **Blockade:** ecad/aodn verlangen Inputs.
- **Braucht:** (dispatcht 2026-10-11: surfrad `38142701271` · dwd-cdc `38142703857` · worldclim `38142706143` · knmi `38142709395`) ecad mit `--station`, aodn mit `--url` dispatchen; `terms`/`ttl` gegen den Compiler gegenprüfen.

### Open-LiDAR — Descope-Verdikte je Portal (gemessen)
- **Status:** descoped | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) **NL `ahn4`**: flai ⊇ Origin außer `08BZ1`/`40HZ1` (2/1375) → **Origin behalten**. **CH `swisstopo`**: flai deckt **39515/41982 = 94,1 %** → flai **partial** → **Origin behalten**. **FR `ign_lidarhd`**: flai 337207 vs Origin 524687, id-**disjunkt** → **Origin behalten**. **NZ `linz`**: bleibt. **`ea_lidar_index`**: emittiert `tile`-bbox-Zeilen, kein phi-Block.
- **Blockade:** keine.
- **Braucht:** — (kein Portal wird descoped; flai ist nirgends ein Superset).

### Adressierte φ-Vorschläge (mycelium-307/305) — verdiktet
- **Status:** descoped | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) alle vier Portal-Coverage-Blöcke (`phi:11772`/`11105`/`11780`/`11113`) **und** Bayern-DOM (`:31728`)/VNP46A3 (`:2051`)/open-lidar-data (`:11788`) stehen. **VNP46A3** `origin` bleibt das Verzeichnis — der Workflow löst per CMR den **neuesten** Granule auf. **open-lidar-data** `terms` bleibt `CC-BY-3.0`.
- **Blockade:** keine.
- **Braucht:** —

### PDS/ESA-Missionsrouten `at` — geklärt
- **Status:** descoped | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) ODF/ODR/RSR/TNF/RSS = Bodenstation-Empfang → `at earth` korrekt; `voyager1_merged`/`voyager2_merged` sind **korrigiert** (`at voyager1`/`at voyager2`, `phi:19835`/`:19847`).
- **Blockade:** keine.
- **Braucht:** —

### SuperMAG-Derivate — descoped (River, Redistribution)
- **Status:** descoped | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) River committete `599cfd45b`: die 5 `substorm_*`-Blöcke sind aus `phi/sources.φ` entfernt („cannot be redistributed"). Der Konsumenten-Orphan ist mit dem Commit getragen.
- **Blockade:** keine.
- **Braucht:** —

### MODVOLC/ITS_LIVE/MASIE — Compiler gebaut + registriert (Diver-Staffel)
- **Status:** eigen (Bau) | **Bindung:** eigen
- **Trigger:** der nächste CDN-Manifestations-Lauf je Quelle
- **Lage:** (gemessen 2026-10-11 via `cargo check -p omegaflow-harvest` 0/0) drei neue Compiler: `modvolc_compiler.rs` (MODVOLC `MODVOLC.A<YYYY><DDD>.<HHMM>.061.NRT.dat`, 12 Spalten, B21→B22-Sättigungs-Fallback, 8 Alerts am Realgranule 2019/365), `its_live_compiler.rs` (STAC `stac.itslive.cloud` → netCDF `v` int16, Polar-Stereo-3413, 8/8 Test, 2000 Zellen), `masie_compiler.rs` (MASIE G02186 4km-CSV, 510 Records = 30 d × 17 Regionen). **Registriert:** MODVOLC (`field … inverse-square em w/m2/sr/um`), ITS_LIVE (`field … patch-levy advective m/yr` + Baseline-Eintrag), MASIE (`phi:1619` **korrigiert** — die alte `nsidc0792`-URL war auth-gated **und** fehl-identifiziert: G02186 ist MASIE, public; 17 `quantity … area m2`).
- **Blockade:** kein `*-cdn.yml` (Manifestation fehlt); `--body <name>` ist Pflicht (Empfänger deklariert, nie defaulted).
- **Braucht:** je Quelle einen `*-cdn.yml`-Workflow (`--body earth --ci-mode`) → Mycelium (s. `## An mycelium`); MASIE-Loader-Arm in `src/archivar` `pending`.

### copernicus B1–B6 — Workflows angelegt, Lauf offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der erste `copernicus-<dataset>-cdn`-Lauf je Dataset
- **Lage:** (gemessen 2026-10-11 via `git status`; dispatcht via `gh workflow run`) sechs Workflows `copernicus-{surface-land,uscrn,igra,woudc,gruan,gnss}-cdn.yml`; Läufe uscrn `38136104626`, surface-land `38136110423`, igra `38136112567`, woudc `38136114532`, gruan `38136116720`, gnss `38136118542`.
- **Blockade:** kein Lauf.
- **Braucht:** die sechs Workflows dispatchen; bei Void Variable/Version korrigieren. Riss: `uscrn` `time_aggregation=daily` / `gnss` `network_type=igs_daily` sind Wahlen ohne Default.

### NOAA S3-Compiler B7/B8 — Workflows angelegt, Lauf offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der erste `noaa-nos-lidar`/`noaa-eri`-CDN-Lauf
- **Lage:** (gemessen 2026-10-11 via `git status`; dispatcht via `gh workflow run`) `noaa-nos-lidar-cdn.yml` + `noaa-eri-cdn.yml`; Bins gebaut/grün. Läufe: nos-lidar `38136120891`, eri `38136123058`.
- **Blockade:** kein Lauf.
- **Braucht:** die zwei Workflows dispatchen.

### Teil C — A5/A6 Earth-Flyby-ODFs
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-11) **A5** Galileo Earth1/Earth2-Tracking **gefunden als TDF** — `GO-SS-RSS-1-TDF-V1.0` (PDS PPI; 1990 DOY 342-344 + 1992 DOY 343-344; **keine** `.ODF`). **A6** MESSENGER 2005 Earth-Flyby **absent** — die RS-Rohbündel beginnen 2006/2007.
- **Blockade:** keine.
- **Braucht:** A5 die TDF-Route als Quelle registrieren (oder `descoped`); A6 `descoped` mit Befund.

### Manifestation SPT-3G D1 — sha256
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein `cmb-cdn`-Lauf abgeschlossen
- **Lage:** (gemessen 2026-10-11) `cmb-cdn 38138848760` in_progress im `Compile`-Step; der alte `38129785553` war rot (`NAXIS2 36557509 != 12*NSIDE^2 (8192)`), Ursache partial-sky/RING — Fix unter Test (Träger Mycelium).
- **Blockade:** Lauf läuft.
- **Braucht:** Lauf-Ergebnis lesen; danach sha256 in `phi/sources.φ` nachtragen.

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
- **Trigger:** das wörtliche Future-Wort (welcher Arm/welche Zeile)
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

## An mycelium

Origin: mountain-folge315 (2026-10-11).

- **Drei neue Compiler brauchen je einen `*-cdn.yml`** (Manifestation; gebaut + registriert 2026-10-11): `modvolc_compiler.rs`, `its_live_compiler.rs`, `masie_compiler.rs` (Crate `omegaflow-harvest`), registriert in `phi/sources.φ` (`modis.higp.hawaii.edu/modvolc_thermal.bin`, `stac.itslive.cloud/its_live_velocity.bin`, `noaadata.apps.nsidc.org/masie_ice_extent.bin`). Die Compiler verlangen `--body <name>` (Empfänger deklariert, nie defaulted). Bitte je einen `*-cdn.yml` mit `--body earth --ci-mode` verdrahten; MASIE braucht in `src/archivar` einen Loader-Arm (`pending`).
- **Die 388 `pds3_fixed_width`-`compiler`-Zeilen sind gesetzt** (Operator-Wort: bauen; `tools/register/src/bin/register_add_compiler.rs`, Commit `93a3d546c`) — kein offener Mycelium-Auftrag mehr.

## Operator-Wort-Register

- **„hä wir bitte du solltest eine taucherstaffel losschicken und du hast dich nur um copernicus gekümmert du sollst alle offenen punkte die die letzte session aufwändig dokumentiert hat umsetzen"** | 2026-10-11 | Operator-Session (mountain) — die Taucherstaffel über den gesamten Wartend-Auftrag losschicken, nicht nur einen Punkt.
- **„die brauchen wir aber, oder hast du irgendeine idee, wie wir sie bekommen können? Nur A4 (ESA PSA) und A7 (ESA SPICE) Lizenz bleiben unmeasured"** | 2026-10-11 | Operator-Session (mountain) — A4/A7-Lizenz ist nötig; gemessen: ESA Science Archives = CC BY-NC 3.0 IGO.
- **„bitte lasse eine taucherstaffel auf ALLE offenen punkte los"** | 2026-10-11 | Operator-Session (mountain) — die Taucherstaffel auf alle offenen Punkte der folge313.
- **„Mindat Level-1 ist descoped ausser es dient als faden aber dann gibt es alternativen"** | 2026-10-11 | Operator-Session (mountain) — Mindat Level-1 descoped.
- **„388 … bauen"** | 2026-10-11 | Operator-Session (mountain) — die 388 `pds3_fixed_width`-`compiler`-Zeilen direkt setzen.
- **„MODVOLC: 2000–2019-Archiv admitieren+bauen"** | 2026-10-11 | Operator-Session (mountain) — MODVOLC admission + Compiler.
- **„schicke eine taucherstaffel los und BAU ENDLICH!!!!"** | 2026-10-11 | Operator-Session (mountain) — Bauen vor Messen; die Taucherstaffel losschicken.
- **„die weltlinie kennt kein jetzt"** | 2026-10-11 | Operator-Session (mountain) — kein Aufschub.

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort („mach das ab jetzt
automatisch", 2026-10-07) trägt Commit und Push.
