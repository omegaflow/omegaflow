<!--
  title: Handover — Mountain-Folge 319 (2026-10-11)
  session: Mountain-Linie in einem Pass — Register-Riss MLA geheilt (field RANGE mla_range eingetragen, der `NAME = RANGE`-Befund aus folge318 als Falschmessung korrigiert) + 10 Quellen-Terms gemessen (Re-Admission/GIC/Space-Weather) + THEMIS-ASI-Koordinatentabelle gemessen (Riss: Compiler emittiert `at earth`, nicht `on earth`) + Band-Paar-CI-Messung (0/59486 modelliert)
  class: handover
  date: 2026-10-11
  sha256: f515351ff05767b52d4e017952f66a8c0d73c4922c10045c903ce37fc535d465
  status: live
-->
# Handover — Mountain-Folge 319 (2026-10-11)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`). Diese
Session konsumierte `handover-2026-10-11-mountain-folge318.md` (→ `archiv/`).

## Burn: open 0.00 · close 0.13 · cap 0.50 · reason: Linien-Session + 6 flash-Taucher (MLA-Riss, Re-Admission-/GIC-Terms, THEMIS-ASI-Koordinaten, Rat Q1/Q2 + Rat Step-4-Modellwahl) + ci-gate-Ursachen + vmf3-clippy-Heilung + Rat-Vorbereitung (`archive_search --all` + 4 UI-Seats) — gemessen via `session_burn`, deepseek-flash, kein pro/max.

## Erledigt in diesem Atom (gelöscht aus dem Register; git trägt es)

- **MLA-Register-Riss geheilt.** Block `phi/sources.φ:12055–12061` (nicht 12036) trug `terms PD` und `at mercury`, aber **keine `field`-Zeile**; der Compiler `mla_compiler.rs:107` emittiert bewusst nur url/format/ttl. Die gemessene RDR-Spalte ist `NAME = RANGE`, `UNIT = "KM"` (`MLARADR.FMT:106`); eingetragen ist jetzt `field RANGE mla_range inverse-square em km 604800 0.0 0.0` — exakt das Haus-Muster `field RANGE hay_lidar_range …` (`:20155`) und `field MARS_RADIUS mola_mars_radius …` (`:12053`).
- **Falschmessung aus folge318/317 korrigiert.** Die Behauptung `RDR-Label: NAME = RANGE` bzw. `^STRUCTURE = MLASCIRDR.FMT` ist **widerlegt** (gemessen via `--verdict`+`curl`): das RDR-`.lbl` trägt `PRODUCT_ID = "MLARADR0801141902"`, `INSTRUMENT_MODE_ID = {"SC_RANGE"}`, `^STRUCTURE = "MLARADR.FMT"` — **kein top-level `NAME`**. `RANGE` ist der Spalten-NAME einer `OBJECT = COLUMN` in der FMT. Mycelium-311 war wahr.
- **Asservatenkammer-Survey — vier Marker vorhanden** (gemessen via `sread`): `survey-2026-10-08-research-api-mcp.md:31–38` trägt Perplexity live · Consensus 429 transient · Elicit descoped · SciSpace pending. Kein Edit nötig; der folge318-Punkt war stale.

## Offen (aufgeschlüsselt)

### Band-Paar-Atom — CI-Idempotenz-Sperre blockt das BND1-Companion
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein `spdf.gsfc.nasa.gov`-Asset-Refresh (Release-Upload) + darauffolgender `witness-odf-residuum`-Lauf
- **Lage:** (gemessen 2026-10-11 via `ci_manage log 38162233629` + GitHub-Releases-API) der Witness-Lauf success, aber `model: 0 samples modeled (123 without station coordinates, 59363 without model)`; der Witness liest das **Release-Asset** `spdx…/releases/download/spdf.gsfc.nasa.gov/pioneer10_odf.bin`, das laut API zuletzt **2026-09-18T05:11:45Z** aktualisiert wurde — also **vor** dem Band-Paar-Bau. **Wurzel gemessen:** `pioneer-odf-cdn.yml:21–31` hat eine **Idempotenz-Sperre** („asset pioneer10_odf.bin already present — manifest skipped"); da das Asset existiert, wird Compile+Upload übersprungen, der Lauf meldet success, ohne das Bin zu erneuern.
- **Blockade:** die Sperre liegt im Mycelium-Workflow; der Re-Manifest-Akt ist CDN-Recht.
- **Braucht:** `pioneer-odf-cdn.yml` (und die übrigen ODF-Workflows) mit einem erzwungenen Re-Manifest (`--force`/clobber bzw. Asset-Löschung) neu dispatchten; danach den Witness-Lauf lesen (BND1 present?).

### ci-gate — Ursachen gemessen; clippy geheilt, format/register fremd
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der nächste abgeschlossene `ci-gate`-Lauf am HEAD `d5525435f`
- **Lage:** (gemessen 2026-10-11 via `ci_manage log 38160978717`) letzter abgeschlossener `ci-gate` (head `591a46a`) = failure, Ursachen benannt:
  - **clippy** (`src/archivar/vmf3.rs:391` `manual_is_multiple_of` ×3 + `:404` `needless_range_loop`) — **in diesem Atom geheilt** (`is_leap` → `is_multiple_of`, Schleife → `MONTH_DAYS.iter().take()`; `cargo check` 0/0).
  - **format** (`src/mathematikerin/mod.rs:33`, `src/mathematikerin/receiver.rs:172`, `src/mathematikerin/tropo.rs:22`) — genau die fremd-modifizierten, aktiv gehaltenen Hünke.
  - **register** (`cdn_reconcile --fail`): `fingrid-cdn.yml:48` schreibt netloc `data.fingrid.fi`, in `phi/sources.φ` **nicht gebunden** (`sgrep fingrid phi/sources.φ` = leer).
- **Blockade:** die `format`-Hünke hält eine andere Linie; die `register`-Bindung braucht Myceliums Workflow/netloc.
- **Braucht:** `src/mathematikerin/*` formatieren (haltende Linie) + `data.fingrid.fi` binden (Mycelium/Mountain); dann den nächsten `ci-gate` einmal lesen.

### Eigene Ephemeride (PEP/Tudat) — Step 4: Rat-Entscheid C, Bau blockiert am aktiven Dateiset
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** die Freigabe des aktiv gehaltenen Mathematikerin-Dateisatzes (`src/mathematikerin/receiver.rs`, `src/mathematikerin/tropo.rs`, `src/mathematikerin/mod.rs`) durch die haltende Linie
- **Lage:** (gemessen 2026-10-11 via `glob`/`sread`/Rat) Step 0/1/2 ✓; Step 3 (Witness-Residuum-Bin) gebaut; Troposphäre (VMF3 b/c) + Band-Paar (turn_ratio) gebaut. **Korrektur:** der LSQ liegt in `src/mathematikerin/least_squares.rs` (nicht `src/archivar/`) und ist bereits an **3** Stellen verdrahtet (`src/archivar/ephemeris.rs:143`, `tools/harvest/src/bin/horizons_compiler.rs:91`, `src/mathematikerin/ozzy.rs:121`) — nur nicht an ein Bahn-Residuum. **Rat-Votum C** (weder 1-Parameter durch die 3-RHS-API routen noch ein 3-Komponenten-Modell erfinden): (1) den Eliminationskern hinter `solve_normal_equations` herausziehen + einen ehrlichen n-RHS-Eingang anbieten (3-RHS wird dünner Wrapper); (2) `fixed_effects_1` aus den 5 Residuen-Bins in **eine** Mathematikerin-Funktion ziehen; (3) vor jedem Modell die Partials zählen — keine zweite Observable → `pending`. **Rat-Vorbereitung (Operator-Wort 2026-10-08):** Recherche-Schicht `archive_search --all` (Verma 2022 ATDF→Doppler; Mazarico 2004 Mars-Odyssey-POD; Verma 2013 MESSENGER/GINS-Orbit-Fit; Bos/Montillet/Williams 2019 Trajektorien-Fit; Pioneer-10-Doppler Zenodo `13309156`) + UI-Rat (Claude Sonnet 5.5 · Qwen3.7-Plus → beide **C**, deckungsgleich mit dem Rat; Z.ai/Duck `pending` — Nicht-Antwort gemessen) — `state/stimmen/2026-10-11_mountain_step4-lsq.md`. Kein Riss.
- **Blockade:** `src/mathematikerin/mod.rs`, `src/mathematikerin/receiver.rs`, `src/mathematikerin/tropo.rs` sind fremd-aktiv gehalten (Ephemeride-Hünke); der Bau würde in ein aktiv gehaltenes Dateiset schreiben.
- **Braucht:** den Dateisatz freigeben, dann die Rat-Schritte 1+2 als begrenzten Bau umsetzen (`cargo check` 0/0).

### THEMIS-ASI — Koordinatentabelle gemessen; Compiler-Form-Riss
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** der nächste begrenzte Schritt (Form des Koordinaten-Eintrags entscheiden)
- **Lage:** (gemessen 2026-10-11 via `--verdict`/`curl`) die maschinenlesbare Tabelle `asilib/data/asi_locations.csv` (HTTP 200, 3228 B) trägt 23 THEMIS-Stationen `code|name|lat|lon` (ATHA…YKNF), **ohne `alt`**; das Berkeley-File `thm_asi_stations.pro` (HTTP 200) trägt eine **abweichende** Station-Menge (mit PINA/NAIN, ohne KUUJ/NRSQ/SNAP/TALO; lon 0–360°) — **Riss**. Der Compiler `themis_asi_keogram_compiler.rs:112` emittiert `at earth`, **nicht** `on earth <lat> <lon> <alt>` — die in folge318 geforderte Form ist ungemessen.
- **Blockade:** die Register-Form der Stationstabelle ist offen (der Compiler konsumiert keine `on earth`-Zeile).
- **Braucht:** entscheiden, ob die Tabelle als `record`/`witness`-Quelle registriert wird (url/origin = Mycelium) oder als Compiler-Eingabe; den Riss a (asilib → 23) vs. b (Berkeley → 20) benennen.

### MLA — CDN-Manifestation (Mountain-Anteil erledigt)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der `mla-cdn`-Lauf
- **Lage:** (gemessen 2026-10-11 via `sread`) Block `phi/sources.φ:12055` repointet (url `pds3_fixed_width_mlaradr0801141902.bin`, origin `…/rdr_radr/2008/jan/mlaradr0801141902.tab`), `field RANGE mla_range …` eingetragen; Workflow `mla-cdn.yml` gebaut (Mycelium).
- **Blockade:** Manifestation ist Myceliums Recht.
- **Braucht:** `mla-cdn`-Lauf lesen; sha256 ins Register.

### DEMETER — lokale Registrierung (Lizenz gemessen)
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass (url/origin von Mycelium, sonst lokal-only)
- **Lage:** (gemessen 2026-10-11 via river-187/188 + `archive_search`) CDPP Data-Centre-Lizenz V1.1, **nicht-kommerziell**; die behauptete Etalab-2.0 ist widerlegt. Verdikt: **nur lokale Registrierung**, kein CDN-Spiegel.
- **Blockade:** die gemessene Lizenz ist in Spannung mit CDN-Manifestation.
- **Braucht:** `terms`-Zeile (Mountain) nach Myceliums lokalem `url`; 6 `field`-Zeilen (`regards.cnes.fr` DM*-Dateien).

### Freie GIC-/Space-Weather-Quellen — Terms gemessen (river-186/187/188)
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass (url/origin von Mycelium)
- **Lage:** (gemessen 2026-10-11 via Taucher) EAGLE-I **CC BY 4.0** (Figshare `10.6084/m9.figshare.24237376`, **nicht Zenodo** — Zenodo-ID 24237376 gibt 404) · OEDI `6458` CC BY 4.0 · OE-417 **US-PD** (`doe417.pnnl.gov`, **keine GMD-Kategorie**) · INTERMAGNET **CC BY-NC 4.0** · GIRO DIDBase **CC BY-NC-SA 4.0** · FMI NUR **CC BY 4.0** · BGS-GIC `terms pending` (UKRI-Werkrechte) · BPA-GIC `terms pending` (kein deklarierter Datensatz). CSES scidb trägt `terms CC0-1.0` (`:20158/20170/20182/20195`) · PCN registriert (`:2183`).
- **Blockade:** url/origin sind Myceliums Manifestations-Recht.
- **Braucht:** je Quelle url/origin (Mycelium) → dann `terms`-Zeile (Mountain); BGS/BPA bleiben `terms pending`.

### Re-Admission lizenz-blockierter Kandidaten (future-233) — Terms gemessen
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass (url/origin von Mycelium)
- **Lage:** (gemessen 2026-10-11 via Taucher) GRDC-Caravan **CC-BY-4.0** (Zenodo `15349031`) · RouteViews **CC-BY-4.0** (Lizenzseite; der by-sa-`href` auf der Hauptseite ist ein Site-Fehler, DataCite-DOI `10.7264/1y7v-2d90` bestätigt BY) · EPH-VSOP **CC-BY-4.0** (Zenodo `17142232`) · GloFAS **Copernicus EMS Terms** (kein CC-BY — Regulation (EU) 2021/696; Attributionspflicht). Survey-Riss: `survey-2026-10-09-redistribution-alternativen.md:12` zitiert ein entferntes Register.
- **Blockade:** url/origin (Mycelium) für die Neu-Admits.
- **Braucht:** vier Verdikte/Registrierungen (Mountain `terms`, Mycelium url/origin).

### Teil C — A5 offen; A6 descoped
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-11 via `sread phi/sources.φ` + folge315) **A5** Galileo Earth1/Earth2 als TDF — `GO-SS-RSS-1-TDF-V1.0` (PDS PPI; 1990 DOY 342-344 + 1992 DOY 343-344; keine `.ODF`); **A6 descoped** — die MESSENGER-2005-Earth-Flyby trägt **kein** ODF/TDF-Dataset (die MLA/RSS-RDR-Blöcke `phi/sources.φ:12055`/`:12040` decken den Flyby ohne eigene ODF-Kampagne; kein Register-Eintrag trägt eine MESSENGER-2005-ODF).
- **Blockade:** keine.
- **Braucht:** A5 die TDF-Route registrieren (oder `descoped`).

### Ephemeris-CDN-Tag — Repoint blockiert (vorbestehende Gate-Verstöße)
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** die Heilung der remove-bias/NaN-Verstöße in den betroffenen measure-Bins
- **Lage:** (gemessen 2026-10-11 via folge318-Register; unverändert; Träger `src/archivar/cdn.rs:23` `EPHEMERIS_TAG`) **beide** Tags tragen Ephemeriden; nur Referenzen, deren Asset in `-ephemeris` liegt, gehören dorthin (22 Sites/11 Dateien). Der Repoint ist gebaut, aber `commit_check` blockt 10 Dateien an **vorbestehenden** Funden (Body-Name-Literal „earth"/„sun"; NaN in `quake`/`solar_cycle`) — Whole-File-Scan beim Commit.
- **Blockade:** die 10 measure-Bins tragen remove-bias/NaN-Verstöße aus WP13.
- **Braucht:** eigener Atom (River/Mountain): die Funde heilen, dann den Tag-Repoint committen.

### opentopography_coverage — per-Dataset-Lizenz (Rat-Entscheid: kein neues Modell)
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** der nächste begrenzte Schritt (Coverage-Manifest manifestieren + Block registrieren)
- **Lage:** (gemessen 2026-10-11 via Rat + `sgrep`; Lauf `38150636428` success) die Lizenz ist **per-Dataset**; das **Rat-Votum** verwirft ein per-Dataset-`terms`-Direktiv (die Grammatik trägt **einen** `terms`-Slot pro Block — `parse.rs:224/265–272`; mehrere Zeilen kollabieren still auf die letzte = Fabrikation). Kanonische Form: **Block-pro-Dataset**, das Portal-Manifest steht auf `origin` (Katalog-URL) **ohne `terms`** (0 honored); die Dataset-Lizenz bleibt `pending` am je geernteten Dataset. `opentopography_coverage.rs:90–95` emittiert `shard <href>`; in `phi/sources.φ` nicht registriert.
- **Blockade:** das Manifest-Asset ist noch nicht manifestiert (Mycelium).
- **Braucht:** das Coverage-Manifest auf den CDN (Mycelium) + Block registrieren (`url …/portal.opentopography.org/opentopography_coverage.manifest / format coverage_manifest / origin …/API/otCatalog?outputFormat=json / compiler … / keine terms-Zeile`); per-Dataset-`terms` als eigener `pending`-Punkt.

### Sechs Klima-Quellen — ecad/aodn brauchen gewählte Eingaben
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** eine gewählte ecad-`station` bzw. aodn-`url`
- **Lage:** (gemessen 2026-10-11 via Workflow-Read) `ecad-cdn.yml` verlangt `element`+`station`+`lat`/`lon`/`alt`, `aodn-cdn.yml` eine OPeNDAP-`url`.
- **Blockade:** die Station/das Dataset ist eine wissenschaftliche Wahl, kein Default.
- **Braucht:** je eine gewählte ECA&D-Station und eine AODN-OPeNDAP-URL → `gh workflow run ecad-cdn.yml -f station=… -f lat=… -f lon=… -f alt=…` / `aodn-cdn.yml -f url=…`.

### Sechs Klima-`series-manifest` — erster Lauf je Familie
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der erste `series-manifest`-Lauf je Klima-Familie
- **Lage:** (gemessen 2026-10-11 via future-235) die sechs Klima-Workflows tragen je einen `series-manifest`-Job; offen ist der erste Lauf je Familie. Alt-Stand `state/zustand/wartend-archiv-2026-10-11-r3.φ`.
- **Blockade:** kein Lauf.
- **Braucht:** die sechs Manifest-Läufe lesen (`ci_manage log <id>`), sha256 je Familie tragen.

### copernicus B1–B6 — Läufe offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der erste `copernicus-<dataset>-cdn`-Lauf je Dataset
- **Lage:** (gemessen 2026-10-11 via folge315-Register; unverändert) sechs Workflows; Läufe uscrn `38136104626` · surface-land `38136110423` · igra `38136112567` · woudc `38136114532` · gruan `38136116720` · gnss `38136118542`.
- **Blockade:** kein Ergebnis.
- **Braucht:** die sechs Läufe lesen; Riss: `uscrn` `time_aggregation=daily` / `gnss` `network_type=igs_daily` sind Wahlen ohne Default.

### NOAA S3-Compiler B7/B8 — Läufe offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der erste `noaa-nos-lidar`/`noaa-eri`-CDN-Lauf
- **Lage:** (gemessen 2026-10-11 via folge315-Register; unverändert) Läufe nos-lidar `38136120891` · eri `38136123058`.
- **Blockade:** kein Ergebnis.
- **Braucht:** die zwei Läufe lesen.

### Manifestation SPT-3G D1 — sha256
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein abgeschlossener `cmb-cdn`-Lauf
- **Lage:** (gemessen 2026-10-11 via folge315-Register; unverändert) Fix v2 unter Test, Träger Mycelium.
- **Blockade:** Lauf läuft.
- **Braucht:** Lauf-Ergebnis lesen; danach sha256 in `phi/sources.φ` nachtragen.

### Frame/Epoche — Transform-Layer (Rat-Entscheid: kein Transform jetzt)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste begrenzte Schritt nur wenn eine CRS-VLR-Messung ihn verlangt
- **Lage:** (gemessen 2026-10-11 via Rat + `sgrep`) Rat-Votum: **kein** Transform-Layer jetzt; `--bbox-off` (`ShardPlacement::Pending` → `shard_intersects_hull` = true, `coverage_manifest.rs:78`) ist **verlustfrei**; die Wire ist 26×f64 ICRS/J2000 ohne Frame-Slot. Der Transform bleibt `pending` mit den Vorbedingungen aus `state/mycelium/frame-epoch-findings.md`. Ein per-Source-ICRS-Stempel wäre Fabrikation.
- **Blockade:** keine (Entscheid steht).
- **Braucht:** nichts bauen; falls je nötig, die CRS-VLR-Range in den Hand-Parsern messen (`open_lidar_data_coverage.rs:127–133`, `noaa_nos_coverage.rs:206–212`) — sonst der benannte Riss.

### particle-cern — Baskets + Sample-Verifikation
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der nächste `subset`-Lauf (CI)
- **Lage:** (gemessen 2026-10-11 via folge315-Register; unverändert) Basket-Arrays gebaut; `cargo test` nur in CI.
- **Blockade:** `subset` hängt am `ci-gate`-Punkt.
- **Braucht:** `subset` grün; ggf. ein v9-only-Sample.

### Exposom-Matrix → TE-Paar-Feed — Repräsentativpunkt
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** extern gedeckter Repräsentativpunkt je Zeile (Operator-Wort 2026-10-10)
- **Lage:** (gemessen 2026-10-11 via folge315-Register; unverändert) kein offener Datensatz trägt eine im Datensatz gemessene Koordinate.
- **Blockade:** der Repräsentativpunkt ist eine wissenschaftliche Annahme, keine Messung.
- **Braucht:** je Zeile den extern gedeckten Repräsentativpunkt → eine Zeile in `phi/sources.φ` → TE-Paar-CI-Feed.

### iEEG — registriertes Wort maßgeblich
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein neues Operator-Wort über 2026-10-06 hinaus
- **Lage:** (gemessen 2026-10-11 via folge315-Register; unverändert) iEEG = privates Experiment, kein CDN.
- **Blockade:** keine.
- **Braucht:** nur ein neues Operator-Wort öffnet es. **Nicht erneut vorlegen.**

### Flyby-Kette — termin 2026-11-01
- **Status:** termin | **Bindung:** termin:2026-11-01
- **Trigger:** ESOC-Recon-Release (oder Descope)
- **Lage:** (gemessen 2026-10-11 via folge315-Register; unverändert) die JUICE-`orbc`-SPK (A7) ist registriert; `doppler.rs` absent.
- **Blockade:** kein ESOC-Recon-Release.
- **Braucht:** ESOC-Release oder Descope-Befund für `doppler.rs`.

### FMHY-Routing — Riss 4 bot-gated
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Riss 4/5-Abschluss — Overpass bot-gated (gemessen 2026-10-10)
- **Lage:** (gemessen 2026-10-11 via folge315-Register; unverändert) Overpass HTTP 406 direkt+Proton → bot-gated.
- **Blockade:** Overpass bot-gated.
- **Braucht:** Riss-4-Abschluss oder Descope.

## An mycelium

Origin: mountain-folge319 (2026-10-11).

- **MLA-Register-Block:** Mountain hat `field RANGE mla_range inverse-square em km 604800 0.0 0.0` in `phi/sources.φ:12055` eingetragen; der Repoint (`origin`→`…/rdr_radr/`, url `pds3_fixed_width_mlaradr0801141902.bin`) steht. **Korrektur:** das RDR-`.lbl` trägt **kein** `NAME` (nur `INSTRUMENT_MODE_ID = {"SC_RANGE"}`, `^STRUCTURE = "MLARADR.FMT"`); die folge317/318-Behauptung `NAME = RANGE`/`MLASCIRDR.FMT` ist widerlegt. Braucht nur noch die CDN-Manifestation + sha256.
- **Band-Paar-Atom — CI-Idempotenz-Sperre (Wurzel gemessen):** der Witness liest das Release-Asset `releases/download/spdf.gsfc.nasa.gov/pioneer10_odf.bin` (Release-API: zuletzt aktualisiert **2026-09-18T05:11:45Z** — vor dem Band-Paar-Bau). Ursache: `pioneer-odf-cdn.yml:21–31` überspringt Compile+Upload, wenn das Asset existiert („already present — manifest skipped"), daher 0 Modelle. **Braucht:** `pioneer-odf-cdn.yml` (und die übrigen ODF-Workflows) mit erzwungenem Re-Manifest (`--force`/clobber bzw. Asset-Löschung) nachziehen; danach den Witness-Lauf lesen.
- **`cdn_reconcile` rot:** `fingrid-cdn.yml:48` schreibt netloc `data.fingrid.fi`, der in `phi/sources.φ` nicht gebunden ist (`sgrep fingrid` = leer) → Job `register` failure. Braucht die `url`/`origin`-Bindung (Mycelium/Mountain).
- **Re-Admission/GIC-Terms stehen** (gemessen); es fehlen die `url`/`origin`-Zeilen für EAGLE-I/OE-417/INTERMAGNET/GIRO/GRDC-Caravan/RouteViews/EPH-VSOP/GloFAS/THEMIS-ASI.
- **DEMETER:** nur lokale Registrierung (Lizenz nicht-kommerziell); vor url/origin den CDN-Spiegel gegen die Lizenz prüfen.

## Operator-Wort-Register

- **„b"** | 2026-10-11 | Operator-Session (mountain) — Form **(b)** für das Band-Paar-Atom: Companion-Band-Ebene (Record bleibt 9 Felder), nicht das Feld-6-Packing.
- **„ja bitte mro-Schritt nachziehen?"** | 2026-10-11 | Operator-Session (mountain) — auch `mro_odf_compiler` (streaming/shard) auf das Band-Paar nachziehen.
- **„bitte befrage auch research und roster wenn du den rat befragst"** | 2026-10-11 | Operator-Session (mountain) — jede Ratsfrage durch Research-Schicht + Roster.
- **„nein bitte hole die stimmen erst ein hier geht es um etwas sehr existenzielles"** | 2026-10-11 | Operator-Session (mountain) — Stimmen vor dem Bau.
- **„hab glm abgeholt"** | 2026-10-11 | Operator-Session (mountain) — GLM 5.3-Antwort vom Operator nachgereicht.

## Träger in `state/` — vieles ist schon recherchiert, nicht neu fahren

- **Band-Paar / DSN-Ratio:** Primärquelle JPL 810-005 Modul 201 Rev F, Tabelle 1 (Zitate im Code `odf_turn_ratio`); `state/mountain/2026-10-10_riss-ort_recherche.txt:4445` (ODF-Struct).
- **Eigene Ephemeride:** `state/mountain/2026-10-10_observer-operator_recherche.txt` · `state/stimmen/2026-10-10_sensory_eigene-ephemeride-round.md` · Konzept `docs/concepts/eigene-ephemeride.md`.
- **GIC / Space-Weather:** `state/stimmen/2026-10-02_autolauf-gsics-deepseek.md` (+ glm53flash/kiloultra/muse/nemotron) · `state/stimmen/2026-10-01_zai-ui_gic-review.txt`.
- **LiDAR-Verdikte:** `state/mycelium/lidar_id_diff_measured.md`.
- **Re-Admission / revive:** `state/future/revive-kandidaten-2026-10-11.md`.
- **Quellen-Landschaft / Routing / Lizenzen:** `state/mountain/routing-untersuchung-2026-10-10.md` · `state/mountain/license-census.tsv` · `state/mountain/blocked-sources-klassen.md`.
- **Asservatenkammer:** `docs/surveys/survey-2026-10-10-asservatenkammer-zensus.md`.

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort („mach das ab jetzt
automatisch", 2026-10-07) trägt Commit und Push.
