<!--
  title: Handover — Mountain-Folge 318 (2026-10-11)
  session: Mountain-Linie in einem Pass — das Band-Paar-Atom gebaut: odf_turn_ratio(uplink, downlink) nach JPL 810-005 Modul 201 Rev F Tabelle 1, die Companion-BND1-Sektion am 9-Feld-Record (Form b), der Witness auf das Paar verdrahtet und alle 14 ODF-Compiler nachgezogen; vorher 7 Stimmen + Primärquelle befragt
  class: handover
  date: 2026-10-11
  sha256: 030b22b63fb2c1aa627054ee129a84b7e1e9d056d8bcbe436c91f006e5bdd2ef
  status: live
-->
# Handover — Mountain-Folge 318 (2026-10-11)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`). Diese
Session konsumierte `handover-2026-10-11-mountain-folge317.md` (→ `archiv/`).

## Burn: open 0.00 · close 0.34 · cap 0.50 · reason: Linien-Session + begrenzte Taucher (PODF-Konsumenten-Karte, 12-Compiler-Band-Trailer) + die Stimmen-Runde (Rat + 7 UI-Stimmen, davon 2 an der tryingopen-Guthaben-Wand); der Kern ist die Primärquellen-Verifikation und der Bau — gemessen via `session_burn`, deepseek-flash, kein pro/max.

## Erledigt in diesem Atom (gelöscht aus dem Register; git trägt es)

- **`uplink_band`-Atom gebaut (Form b).** `odf_turn_ratio(uplink_band, downlink_band)` (`src/archivar/odf.rs`) nach **JPL 810-005 Modul 201 Rev F, Tabelle 1** (S/S 240/221 · S/X 880/221 · X/S 240/749 · X/X 880/749; Ka/K = verhandelt → `None`, nie geraten). Companion **BND1**-Sektion (`write_podf_bin_bands`/`parse_podf_bin_bands`): 9-Feld-Record bleibt 72 B, `None` = absent (0 honored), `parse_podf_bin` rückwärtskompatibel. Witness (`witness_odf_residuum.rs`) liest das **Paar**. 14 ODF-Compiler sammeln `Some(r.uplink_band)` + gepaarter `zip→sort→unzip`; `mro` (streaming/shard) über `harvest_stream`→`flush_shard`-Bandpuffer. **6 TNF-Compiler** (`tnf`/`cassini`/`maven`/`dart`/`messenger`/`gll_rss`) tragen den Uplink-Band im Companion — ableitbar nur bei den UL-/Mixed-Formaten (`tnf_row_uplink_band`), DL-Formate lassen ihn absent (0 honored). 3 Tests. `cargo check` 0/0, alle Bins sauber.
- **Untersuchung (7 Stimmen + Primärquelle).** Rat (5 Stimmen) gegen `[f64;10]`; Roster (Claude Sonnet 5.5 am Original, Qwen3.7-Plus, Qwen3.8-Max, Qwen3.8 2.4T, Duck GPT-6 Luna, GLM 5.3) + `archive_search --all` (GMAT-Doku, TRK-2-34) bestätigen: Downlink-Band allein bestimmt den Ratio nicht; Faktor **749/221 ≈ 3,389**. Riss: Duck 3,67 (Zeilen-Dreher). Pending: DeepSeek V4 Pro + Z.ai (tryingopen-Guthaben / Deep-Think-Rücklauf).

## Offen (aufgeschlüsselt)

### Band-Paar-Atom — CI-Nachlauf
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der nächste ODF-Compiler-Lauf (CI)
- **Lage:** (gemessen 2026-10-11 via `cargo check`) der Witness liest **lokale** `data/*.bin`; in CI müssen `pioneer10_odf.bin`/`galileo_odf.bin` (und die übrigen ODF-Bins) **neu erzeugt** werden, sonst Bänder absent → Witness gibt korrekt „no model (0 honored)".
- **Blockade:** kein CI-Nachlauf.
- **Braucht:** die ODF-Compiler-Workflows `pioneer10-odf`/`galileo-odf` (und die übrigen) neu dispatchten, danach den Witness-Lauf lesen (Bänder present?).

### ci-gate — format + register geheilt; Läufe am neuen HEAD
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der nächste `ci-gate`-Lauf am neuen HEAD
- **Lage:** (gemessen 2026-10-11 via `ci_manage list`) der letzte abgeschlossene `ci-gate` trug `format`+`register` failure (in folge316 geheilt); dieser Push dispatcht einen frischen `ci-gate`. Zusätzlich: `ci-check` rot (`test (4)`, Träger Mycelium/Stehender Pass).
- **Blockade:** `cargo test` läuft nur in CI; kein Polling.
- **Braucht:** einen abgeschlossenen `ci-gate` **einmal** lesen (`ci_manage log <id>`) — format/register/subset grün?

### LiDAR-Inventar / Extraterrestrisch — MLA-Compiler auf RDR (gebaut); Block-Update offen
- **Status:** eigen (Bau) | **Bindung:** eigen
- **Trigger:** der nächste begrenzte Schritt (Mycelium url/origin → `field`-Zeile)
- **Lage:** (gemessen 2026-10-11 via `cargo build`; Träger `phi/sources.φ:12036`) MLA-Compiler walkt `…/messmla_2001/rdr_radr/` rekursiv; RDR-Label `NAME = RANGE`, target MERCURY, roundtrip hält. Der Parser riss bei `mlascirdr` (`merge_structure` schneidet am ersten `END_OBJECT`), vorbestehend, benannt. Register-Block noch CDR.
- **Blockade:** der Block-Repoint (url/origin) ist Myceliums Manifestations-Recht.
- **Braucht:** Mycelium: `origin`→`…/rdr_radr/`, `url`-Asset auf das RDR-Produkt, CDN-Manifestation; dann `field RANGE mla_range inverse-square em km 604800 0.0 0.0`. `opentopography_coverage`: Lizenz-Verdikt.

### Eigene Ephemeride (PEP/Tudat) — Kette + Band-Paar gebaut; Step 4 offen
- **Status:** eigen (Bau) | **Bindung:** eigen
- **Trigger:** der nächste begrenzte Schritt (Step 4: `least_squares.rs` an ein Bahn-Residuum verdrahten)
- **Lage:** (gemessen 2026-10-11 via `sread` + `cargo check`; Träger `docs/concepts/eigene-ephemeride.md`) Step 0/1/2 ✓; Step 3 (Witness-Residuum-Bin) gebaut; CI-Lauf `38151032366` (success) rechnete die volle Kette auf 59 486 Pioneer-10-Samples. **Die Troposphäre (VMF3 b/c) und das Band-Paar (turn_ratio) sind gebaut.** Step 4: `least_squares.rs` gebaut, nicht verdrahtet. PEP/Tudat = Lizenz-/Verfahrensreferenz.
- **Blockade:** kein begrenzter Schritt ohne Bahn-Residuum.
- **Braucht:** Step 4 verdrahten.

### Ephemeris-CDN-Tag — Repoint gemessen, blockiert (vorbestehende Gate-Verstöße)
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** die Heilung der remove-bias/NaN-Verstöße in den betroffenen measure-Bins
- **Lage:** (gemessen 2026-10-11; Träger `src/archivar/cdn.rs:23` `EPHEMERIS_TAG`) **beide** Tags tragen Ephemeriden; nur Referenzen, deren Asset in `-ephemeris` liegt, gehören dorthin (22 Sites/11 Dateien). Der Repoint ist gebaut, aber `commit_check` blockt 10 Dateien an **vorbestehenden** Funden (Body-Name-Literal „earth"/„sun"; NaN als Datenmarker in `quake`/`solar_cycle`) — Whole-File-Scan beim Commit.
- **Blockade:** die 10 measure-Bins tragen remove-bias/NaN-Verstöße aus WP13.
- **Braucht:** eigener Atom (River/Mountain): die Funde heilen, dann den Tag-Repoint committen.

### opentopography_coverage — per-Dataset-Lizenz
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** ein Lizenz-Modell für per-Dataset-terms
- **Lage:** (gemessen 2026-10-11 via `archive_search`) die Lizenz ist **per-Dataset**, ein einzelnes `terms` überzeichnet → Riss.
- **Blockade:** das Register kennt kein per-Dataset-`terms`-Modell.
- **Braucht:** Verdikt (per-Dataset-`terms`-Modell einführen) oder nur `archive_search`-Arm ohne Register-Block.

### Sechs Klima-Quellen — ecad/aodn brauchen gewählte Eingaben
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** eine gewählte ecad-`station` bzw. aodn-`url`
- **Lage:** (gemessen 2026-10-11 via Workflow-Read) `ecad-cdn.yml` verlangt `element`+`station`+`lat`/`lon`/`alt`, `aodn-cdn.yml` eine OPeNDAP-`url`.
- **Blockade:** die Station/das Dataset ist eine wissenschaftliche Wahl, kein Default.
- **Braucht:** je eine gewählte ECA&D-Station und eine AODN-OPeNDAP-URL → `gh workflow run ecad-cdn.yml -f station=… -f lat=… -f lon=… -f alt=…` / `aodn-cdn.yml -f url=…`.

### copernicus B1–B6 — Läufe offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der erste `copernicus-<dataset>-cdn`-Lauf je Dataset
- **Lage:** (gemessen 2026-10-11 via folge315-Register; folge315, unverändert) sechs Workflows; Läufe uscrn `38136104626` · surface-land `38136110423` · igra `38136112567` · woudc `38136114532` · gruan `38136116720` · gnss `38136118542`.
- **Blockade:** kein Ergebnis.
- **Braucht:** die sechs Läufe lesen; Riss: `uscrn` `time_aggregation=daily` / `gnss` `network_type=igs_daily` sind Wahlen ohne Default.

### NOAA S3-Compiler B7/B8 — Läufe offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der erste `noaa-nos-lidar`/`noaa-eri`-CDN-Lauf
- **Lage:** (gemessen 2026-10-11 via folge315-Register; folge315, unverändert) Läufe nos-lidar `38136120891` · eri `38136123058`.
- **Blockade:** kein Ergebnis.
- **Braucht:** die zwei Läufe lesen.

### Teil C — A5/A6 Earth-Flyby-ODFs
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-11 via folge315-Register; folge315, unverändert) **A5** Galileo Earth1/Earth2 als TDF — `GO-SS-RSS-1-TDF-V1.0` (PDS PPI; 1990 DOY 342-344 + 1992 DOY 343-344; keine `.ODF`). **A6** MESSENGER 2005 Earth-Flyby absent.
- **Blockade:** keine.
- **Braucht:** A5 die TDF-Route registrieren (oder `descoped`); A6 `descoped` mit Befund.

### Manifestation SPT-3G D1 — sha256
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein abgeschlossener `cmb-cdn`-Lauf
- **Lage:** (gemessen 2026-10-11 via folge315-Register; folge315, unverändert) Fix v2 unter Test, Träger Mycelium.
- **Blockade:** Lauf läuft.
- **Braucht:** Lauf-Ergebnis lesen; danach sha256 in `phi/sources.φ` nachtragen.

### Frame/Epoche — Transform-Layer
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Architektur-Entscheid der Rat-Linse (Vorlage `state/mycelium/frame-epoch-findings.md`)
- **Lage:** (gemessen 2026-10-11 via folge315-Register; folge315, unverändert) bbox-off Default steht; Transform-Layer `pending`.
- **Blockade:** Architektur.
- **Braucht:** Rat-Linse über den Transform-Layer.

### particle-cern — Baskets + Sample-Verifikation
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der nächste `subset`-Lauf (CI)
- **Lage:** (gemessen 2026-10-11 via folge315-Register; folge315, unverändert) Basket-Arrays gebaut; `cargo test` nur in CI.
- **Blockade:** `subset` hängt am `ci-gate`-Punkt.
- **Braucht:** `subset` grün; ggf. ein v9-only-Sample.

### Exposom-Matrix → TE-Paar-Feed — Repräsentativpunkt
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** extern gedeckter Repräsentativpunkt je Zeile (Operator-Wort 2026-10-10)
- **Lage:** (gemessen 2026-10-11 via folge315-Register; folge315, unverändert) kein offener Datensatz trägt eine im Datensatz gemessene Koordinate.
- **Blockade:** der Repräsentativpunkt ist eine wissenschaftliche Annahme, keine Messung.
- **Braucht:** je Zeile den extern gedeckten Repräsentativpunkt → eine Zeile in `phi/sources.φ` → TE-Paar-CI-Feed.

### iEEG — registriertes Wort maßgeblich
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein neues Operator-Wort über 2026-10-06 hinaus
- **Lage:** (gemessen 2026-10-11 via folge315-Register; folge315, unverändert) iEEG = privates Experiment, kein CDN.
- **Blockade:** keine.
- **Braucht:** nur ein neues Operator-Wort öffnet es. **Nicht erneut vorlegen.**

### api.sensor.community — eigener Punkt, Arm/Zielzeile unbenannt
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste begrenzte Schritt (Arm/Zielzeile benennen)
- **Lage:** (gemessen 2026-10-11 via future-235) HTTP 200; `sensor.community` bereits registriert (`phi:338/1218`). Kein Future-Wort — eigener Register-Punkt.
- **Blockade:** „zulassen" nennt keinen Arm/keine Zeile.
- **Braucht:** Arm/Zielzeile selbst benennen oder `descoped` mit Befund.

### Flyby-Kette — termin 2026-11-01
- **Status:** termin | **Bindung:** termin:2026-11-01
- **Trigger:** ESOC-Recon-Release (oder Descope)
- **Lage:** (gemessen 2026-10-11 via folge315-Register; folge315, unverändert) die JUICE-`orbc`-SPK (A7) ist registriert; `doppler.rs` absent.
- **Blockade:** kein ESOC-Recon-Release.
- **Braucht:** ESOC-Release oder Descope-Befund für `doppler.rs`.

### FMHY-Routing — Riss 4 bot-gated
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Riss 4/5-Abschluss — Overpass bot-gated (gemessen 2026-10-10)
- **Lage:** (gemessen 2026-10-11 via folge315-Register; folge315, unverändert) Overpass HTTP 406 direkt+Proton → bot-gated.
- **Blockade:** Overpass bot-gated.
- **Braucht:** Riss-4-Abschluss oder Descope.

### DEMETER — Lizenz-Riss (Etalab-2.0 widerlegt)
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass (URL/Origin von Mycelium)
- **Lage:** (gemessen 2026-10-11) `regards.cnes.fr` trägt **nicht** Etalab-2.0; das CDPP-eigene Lizenzdokument verbietet kommerziellen Gebrauch und verlangt für jede Weitergabe an Dritte die schriftliche Zustimmung. **Riss** zur Mycelium-Übergabe 308.
- **Blockade:** die gemessene Lizenz ist in Spannung mit der CDN-Manifestation.
- **Braucht:** Verdikt: CDN-Spiegel gegen die Lizenz prüfen; bei Konflikt nur lokale Registrierung. `terms`-Zeile + 6 `field`-Zeilen (Mountain), Mycelium url/origin zuerst.

### Freie GIC-/Space-Weather-Quellen — Verdikte (river-186/187)
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-11) CSES `cc-by-4.0` · PCN `CC-BY-4.0` · PCS `pending` · EAGLE-I `CC BY 4.0` · OE-417 US-`PD` (Portal `doe417.pnnl.gov`, keine GMD-Kategorie). INTERMAGNET `CC BY-NC 4.0` (CDN nein) · GIRO DIDBase `CC BY-NC-SA 4.0` (registriert). FMI/BPA/BGS GIC offen.
- **Blockade:** url/origin sind Myceliums Manifestations-Recht.
- **Braucht:** je Quelle die `terms`-Zeile (Mountain) + url/origin (Mycelium); FMI/BPA/BGS GIC restlich messen.

### Sechs Klima-`series-manifest` — erster Lauf je Familie
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der erste `series-manifest`-Lauf je Klima-Familie
- **Lage:** (gemessen 2026-10-11 via future-235) die sechs Klima-Workflows tragen je einen `series-manifest`-Job; offen ist der erste Lauf je Familie. Alt-Stand `state/zustand/wartend-archiv-2026-10-11-r3.φ`.
- **Blockade:** kein Lauf.
- **Braucht:** die sechs Manifest-Läufe lesen (`ci_manage log <id>`), sha256 je Familie tragen.

### Re-Admission lizenz-blockierter Kandidaten (future-233)
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass
- **Blockade:** url/origin (Mycelium) für die Neu-Admits.
- **Braucht:** vier neue Verdikte/Registrierungen (GRDC-Caravan, RouteViews, EPH-VSOP, GloFAS) — Mountain `terms`, Mycelium url/origin.

### Asservatenkammer-Survey — vier Marker (mycelium-308)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-11) `docs/surveys/survey-2026-10-08-research-api-mcp.md` — vier Marker (Perplexity live · Consensus 429 transient · Elicit descoped · SciSpace pending).
- **Blockade:** keine.
- **Braucht:** die vier Marker im Survey setzen.

## An mycelium

Origin: mountain-folge318 (2026-10-11).

- **Band-Paar-Atom — CI-Nachlauf.** Die 14 ODF-Compiler schreiben jetzt eine `BND1`-Benchsection (Uplink-Band-ID je Record) in `write_podf_bin_bands`. Damit der Witness in CI das Paar liest, müssen die ODF-Assets **neu erzeugt/manifestiert** werden (Compiler-Workflows `pioneer10-odf`, `galileo-odf`, `cassini-odf`, … neu dispatchten); danach den Witness-Lauf lesen. Ohne Nachlauf sind die Bänder absent → der Witness gibt korrekt „no model (0 honored)".
- **MLA-Register-Block (mountain-317):** Compiler walkt `…/rdr_radr/`; Block `phi/sources.φ:12036` repointen (`origin`, `url`-Asset RDR, manifestieren); dann Mountain `field RANGE mla_range`.
- **GIC/Space-Weather + Re-Admission:** die Neu-Admits (GRDC-Caravan/RouteViews/EPH-VSOP/GloFAS) brauchen nach Mountains `terms`-Verdikt url/origin.
- **DEMETER — Lizenz-Riss:** die behauptete Etalab-2.0 ist widerlegt; vor url/origin den CDN-Spiegel gegen die Lizenz prüfen.

## Operator-Wort-Register

- **„b"** | 2026-10-11 | Operator-Session (mountain) — Form **(b)** für das Band-Paar-Atom: Companion-Band-Ebene (Record bleibt 9 Felder), nicht das Feld-6-Packing.
- **„ja bitte mro-Schritt nachziehen?"** | 2026-10-11 | Operator-Session (mountain) — auch `mro_odf_compiler` (streaming/shard) auf das Band-Paar nachziehen.
- **„bitte befrage auch research und roster wenn du den rat befragst"** | 2026-10-11 | Operator-Session (mountain) — jede Ratsfrage durch Research-Schicht + Roster.
- **„nein bitte hole die stimmen erst ein hier geht es um etwas sehr existenzielles"** | 2026-10-11 | Operator-Session (mountain) — Stimmen vor dem Bau.
- **„hab glm abgeholt"** | 2026-10-11 | Operator-Session (mountain) — GLM 5.3-Antwort vom Operator nachgereicht.

## Träger in `state/` — vieles ist schon recherchiert, nicht neu fahren

- **Band-Paar / DSN-Ratio:** die Primärquellen `docs/`-extern (810-005 Modul 201 Rev F, Tabelle 1 — Zitate im Code `odf_turn_ratio`); `state/mountain/2026-10-10_riss-ort_recherche.txt:4445` (ODF-Struct).
- **Eigene Ephemeride:** `state/mountain/2026-10-10_observer-operator_recherche.txt` · `state/stimmen/2026-10-10_sensory_eigene-ephemeride-round.md` · Konzept `docs/concepts/eigene-ephemeride.md`.
- **GIC / Space-Weather:** `state/stimmen/2026-10-02_autolauf-gsics-deepseek.md` (+ glm53flash/kiloultra/muse/nemotron) · `state/stimmen/2026-10-01_zai-ui_gic-review.txt`.
- **LiDAR-Verdikte:** `state/mycelium/lidar_id_diff_measured.md`.
- **Re-Admission / revive:** `state/future/revive-kandidaten-2026-10-11.md`.
- **Quellen-Landschaft / Routing / Lizenzen:** `state/mountain/routing-untersuchung-2026-10-10.md` · `state/mountain/license-census.tsv` · `state/mountain/blocked-sources-klassen.md`.
- **Asservatenkammer:** `docs/surveys/survey-2026-10-10-asservatenkammer-zensus.md`.

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort („mach das ab jetzt
automatisch", 2026-10-07) trägt Commit und Push.
