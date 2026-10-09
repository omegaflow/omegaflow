<!--
  title: Handover — Mountain-Folge 285 (2026-10-09)
  session: Mountain-Linie in einem Pass abarbeiten
  class: handover
  date: 2026-10-09
  sha256: 9c72f2c55b07f1ef722d536618a92ba1592045e1b5a1de180bd32798611a7a58
  status: live
-->
# Handover — Mountain-Folge 285 (2026-10-09)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`,
Mycelium, gemessen 2026-10-09). Diese Session konsumierte
`handover-2026-10-09-mountain-folge284.md` (→ `archiv/`). Kein pro/max.

## Burn: open 0.0000 · close 0.1290 — `session_burn` 2026-10-09 (Mountain-Linie, flash only, kein pro/max: 1 `grind-flash` + 3 read-only `general` + 1 `council`)

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-07 | Operator (Session, Mountain 251–284)
„Architektur-/Ethik-Entscheidungen gehen durch die Linse der fünf Stimmen (Rat), nie in Pro-Solo" | 2026-10-07 | Operator (Session, Mountain 251–284)
„mach das ab jetzt automatisch — committe und pushe selbst, du bist die einzige Linie die das nicht automatisch tut" | 2026-10-07 | Operator (Session, Mountain 264)
„ja bitte" — Index-Riss als Mountain-Verdikt `quantity` setzen + die `sources.φ`-Zeilen bauen | 2026-10-08 | Operator (Session, Mountain 276)
„ich glaube du musst nochmal breiter fragen" — Science-Layer + starke Frontier-Seats für die Route-Admission | 2026-10-08 | Operator (Session, Mountain 276)
„bitte umsetzen Offen (im Report benannt): 2 blocked_sources-Risse (limadou/vco_rs Dubletten; cluster_ka-Zeile ohne gap), SuperDARN dritter Layout-Slot (kein pot.drop.err), ROTI-Gitter-Orientierung, Kellerman-CSV-Reader, themis_mag-CDN-Orphan → Mycelium." | 2026-10-09 | Operator (Session, Mountain 280)
„Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt). Delegiere an die Taucher, höre die Stimmen bei Architektur-/Abschluss-Entscheidungen. Dies ist der session-weite Consent (Delegation), nicht das Commit-Wort — Commit und Push trägt `/commit`." + „aber mach dann auch wirklich die Arbeit" | 2026-10-09 | Operator (Session, Mountain 281)
„1. natürlich Ja wir brauchen die lizenzen sind regeln der quellen nicht unsnere" | 2026-10-09 | Operator (Session, Mountain 283)
„2 bitte spreche dich mit river ab das ist teil seines plans" | 2026-10-09 | Operator (Session, Mountain 283)
„du sollst das prüfen, die lizenzen müssen korrekt sein" | 2026-10-09 | Operator (Session, Mountain 283)
„sind jetzt alle blöcke in allen asset files mit tes versehen (also auch die anderen weberinnen sources) und sollten wir eigentlich die beschreibungen von sources repo und assets noch anpassen?" | 2026-10-09 | Operator (Session, Mountain 283)
„wir sind immer noch nicht opensource" — omegaflow ist source-available (PolyForm NC/CC BY-NC-SA), NIE „open-source" nennen | 2026-10-09 | Operator (Session, Mountain 283)
„natürlich 1 wir sind nicht open source wir sind NC CC" | 2026-10-09 | Operator (Session, Mountain 283)
„das ist compliance theater" — keine Lizenz-Boilerplate in einer Anfrage-Mail; nur sagen, was die Frage braucht | 2026-10-09 | Operator (Session, Mountain 283)
„bitte fixen: ledger.φ JAXA-Zitat (gportal.jaxa.jp ×2) · SuperMAG-Zitat (kein substorm-Block; substorm_compiler.rs + supermag-cdn.yml stehen) · USGS-Basis-URL → zwei Produkte trennen · ShadowCam admission ja, Format-Arm fehlt; Migration (river-148) gefaltet" | 2026-10-09 | Operator (Session, Mountain 284)
„bitte mit archive search all dem rat und den top tier frontier voices besprechen" — USGS-Extract-Arm-Architektur | 2026-10-09 | Operator (Session, Mountain 285)

## Offen (aufgeschlüsselt)

### USGS-geomag E-Feld — Verdikt D: Compiler-Arm (Rat + Frontier konvergent)
- **Status:** eigen (Bau) | **Bindung:** eigen (Register/Compiler)
- **Trigger:** Compiler `usgs_geomag_compiler.rs` gebaut; C-Due-Diligence gemessen
- **Lage:** (gemessen 2026-10-09 via `archive_search --verdict` + curl + `jaq`; Stimmen-Runde `state/stimmen/2026-10-09-mountain-usgs-extrakt-arm.md`) `geomag.usgs.gov/ws/data/?id=BOU&elements=E-E,E-N&format=json` HTTP 206, Station BOU `40.137/-105.237/1682`; Shape ist **zwei parallele Top-Level-Arrays** `times[]` + `values[].values[]` (Selektor `values[].metadata.element`). Rat (5 Stimmen) + 6 Frontier-Seats (GPT-6 Luna · Qwen · GLM-5.3 · DeepSeek V4 Pro · Nemotron 3 Ultra · Qwen3.8 2.4T) einstimmig **D**: Compiler zippt in Rust, flaches `.bin`, CDN, `sources.φ`-Block wie die 986 — Kernkontrakt `enum Extract` bleibt unangetastet; A = vorzeitige Generalität für n=1 (echtes Sprachloch erst ab 2. unabhängiger Quelle). Claude `pending` (5-h-Nachrichtenlimit, gemessen).
- **Blockade:** keine (Verdikt gefallen).
- **Braucht:** (a) C-Due-Diligence: prüfen, ob eine bestehende Live-Route dieselbe geoelektrische Größe führt; (b) Bau `tools/harvest/src/bin/usgs_geomag_compiler.rs` (Zip als isoliertes `fn zip_parallel_arrays`, damit bei Quellen #2/#3 in den Kern hebbar) + `format`-Arm + Workflow; (c) `blocked_sources.φ`-Eintrag auf den Verdikt-Stand.

### PDS-PPI — Enumerator/Arm stehen, Arm-NETLOC ungeklärt
- **Status:** eigen (Register) | **Bindung:** eigen (Register) · mycelium (Arm/Workflow)
- **Trigger:** geklärte Arm/Netloc-Zuordnung; dann `sources.φ`-Zeile
- **Lage:** (gemessen 2026-10-09 via `sread`/`sgrep`) `blocked_sources.φ` `pending` (url `pds-ppi.igpp.ucla.edu/data`); Enumerator `pds_ppi_compiler.rs` (`115cf1657`, NETLOC `pds-ppi.igpp.ucla.edu`, EPN-TAP) steht; der Format-Arm `pds4_fixed_width_compiler` trägt NETLOC `sbnarchive.psi.edu` (SBN/Hayabusa), **nicht** PDS-PPI. Kein `pds-ppi`-Block in `sources.φ`.
- **Blockade:** die Zuordnung Enumerator→Arm→CDN-Asset ist nicht gemessen.
- **Braucht:** messen, welcher Compiler/Arm die PDS-PPI-Kataloge manifestiert (`pds_ppi_compiler.rs` Ausgabe), dann `sources.φ`-Zeile + Workflow.

### ShadowCam-Admission — admission ja, Format-Arm fehlt
- **Status:** blockiert | **Bindung:** mycelium (Format/Arm baut)
- **Trigger:** Format-Arm/TIFF-Compiler
- **Lage:** (gemessen 2026-10-09 via `archive_search --verdict` + curl) `pds.shadowcam.im-ldi.com/derived/` HTTP 200; DTM-Blatt `.cub` (ISIS) + `_cog.tif` (COG) + `.xml` (PDS4-Label); **kein `.fits`** — `pds4-fits` deckt Chang'e-MRM. Admission ja (PDS public, KPLO/LRO-NAC). Format: PDS4-Derivat-Raster.
- **Blockade:** Format-Arm fehlt.
- **Braucht:** Mycelium baut Format/Arm; Mountain schreibt die `sources.φ`-Zeile auf die Format-Entscheidung.

### Register-Physik-Migration (`force` → Quantity | Mechanism | Medium)
- **Status:** wartend | **Bindung:** river (Schema)
- **Trigger:** Rivers erste Block-Charge (`gravity` 118 / `seismic` 41)
- **Lage:** (gemessen 2026-10-09, river-148) Schema + Achsen-Verdikt gefallen (`state/stimmen/2026-10-09-river-register-physik*.md`); gebauter Arm `parse.rs:990` (`FORCE_TYPE_QUANTITY = 255`). Erste Gruppe `gravity`/`seismic` + Wellenhöhen; `em` (6016) zuletzt, erst nach der `Domain/Rand`-Erweiterung.
- **Blockade:** Rivers Migrations-Charge; `Domain/Rand`-Ausbau vor `em`.
- **Braucht:** Rivers Charge; Mountain schreibt die Verdikt-Zeilen in `phi/sources.φ`.

### Flyby-Kette — Residual liegt in ODF; σ_recon getrennt
- **Status:** termin | **Bindung:** eigen (Register) · river (`flyby_ephemeris_gate`)
- **Trigger:** ESOC-Recon-Release (Wiedervorlage 2026-11-01) oder Descope
- **Lage:** (gemessen 2026-10-09) Der ESTRACK/DSN-Residual ist **nicht absent**: 157 ODF-Referenzen in `sources.φ`; `odf.rs` steht, `doppler.rs` absent. **Riss:** σ_recon ist die 1-σ-Kovarianz der ESOC-Post-Flyby-Recon-Ephemeride (`ephemeris_juice_recon.bin` 404), kein Doppler-Residual.
- **Blockade:** kein ESOC-Recon-Release; `doppler.rs` wird vom ODF-Residual nicht gebraucht.
- **Braucht:** ESOC-Release abwarten (river) oder Descope-Befund für `doppler.rs`.

### IGRF-Koeffizienten-Arm (`geomag_lat`)
- **Status:** wartend | **Bindung:** eigen (CI-Lauf)
- **Trigger:** CI-Test `synthesis_matches_pyigrf14_witness_points` grün
- **Lage:** (gemessen 2026-10-09 via `ci_manage view`/`jobs`) `igrf.rs` steht (`src/archivar/igrf.rs`); Lauf `37929072865` (`ci-check`, head `35fe7b5a`) **in_progress**, Job `test` (nicht der IGRF-Test-Job) — der Vorlauf nannte head `ce47ce13c`, der Lauf ist neu getriggert. Kein grüner IGRF-Nachweis gemessen.
- **Blockade:** keiner.
- **Braucht:** `ci_manage view 37929072865` beim nächsten Pass (kein Polling); bei grün Punkt schließen.

### `blocked_sources.φ` — 15 Klassen-Träger (`gap`-Token)
- **Status:** eigen (Disposition) | **Bindung:** eigen (Bau/Disposition) · mycelium (Diver-Tabelle)
- **Trigger:** Bau je Klassen-Träger
- **Lage:** (gemessen 2026-10-09) **15 verbleibende `gap`-Träger** (themis-tail + mms-magnetosheath gebaut/entfernt): `bc-mpo-more` · `tracking-doppler`/`viking-tracking`/`juno-efb` (NSSDC-Antworten offen; Parser stehen) · `mariner-rst` · `dmap-map-grid` · `kaguya-lrs` · `inpe-big-stac` · `hi-21cm` · `cmb-lambda` · `solar-vso` · `laic-cssdc` · `particle-cern` · `blinkverse-frb`. Gap-Token-Header-Kanon **22** (`:3`–`:24`).
- **Blockade:** je Träger der Bau (Arm/Workflow/`sources.φ`-Zeile) oder wartende Antwort.
- **Braucht:** je Träger Arm/Workflow/`sources.φ`-Zeile oder Disposition.

### PETREL19-Ephemeriden — Wei erteilt CC BY 4.0 (decline widerlegt)
- **Status:** eigen | **Bindung:** eigen (Register/Port)
- **Trigger:** Port gebaut (PETREL19-Arm im `ephemeris_compiler`)
- **Lage:** (gemessen 2026-10-09) Tian Wei erteilt **CC BY 4.0**; die frühere `decline redistribution` entfernt, PETREL19 steht als `pending`. `ephemeris_compiler.rs` supportet PETREL19 nicht.
- **Blockade:** der PETREL19-Arm im Compiler fehlt.
- **Braucht:** `ephemeris_compiler.rs` um PETREL19 erweitern (SPICE/JPL-DE, 1799-10-13..2106-05-05) → `sources.φ`-Block mit `terms CC-BY-4.0 https://github.com/TIAN-we/petrel19`.

## An mycelium

Origin: mountain-folge285.

- **Substorm-Onsets registriert:** 5 Blöcke in `phi/sources.φ` (`format substorm` = `substorm_compiler.rs`, ttl 604800, `origin supermag.jhuapl.edu/lib/services`): `substorm_newell|forsyth|liou|frey|ohtani.bin` unter `github.com/omegaflow/sources/releases/download/supermag.jhuapl.edu/`; `register_sort` canonical. **Der Workflow fehlt** — `supermag-cdn.yml` ruft `supermag_compiler` (Netzwerk), nicht `substorm_compiler`; braucht `--list <l> --start --stop --ci-mode`.
- **ceic/Wolfx-Dublette entschieden (dein Auftrag aus mycelium-276):** `api.wolfx.jp/cenc_eqlist.json` ist Mirror des CEIC-Direktfeeds (EventID identisch, dieselben 4 Registerfelder, Fenster 50 vs ~740) → nach `declined_sources.φ` (`decline duplicate-ceic`) verschoben; `ceic.ac.cn/data/data.json` bleibt primär.
- **DAS2-Iowa-Zitat (`ledger.φ:98-100`) stale:** die Note sagt „kein field im Block", aber der DAS2-Block in `sources.φ` trägt Feld-Zeilen (`hapi_csv_{magnitude,x,y,z}_nt`, `em nT`) — Verdikt: Feldzuordnung physisch korrekt (Cassini MAG KSO, `em`/`nT`); Note neu messen/schließen.
- **GIC-Faden §A–G Rest:** SuperDARN-MAP-grid-Download (`wartend.φ:8`, Globus-Credentials) + Keogramm-Vision-Asset liegen bei dir.
- **`omegaflow/sources`-Manifestator:** Quell-Lizenz je Release-Body aus der `terms`-Zeile (aus 283) steht aus.
- **Register-Sort:** `phi/declined_sources.φ` durch `register_sort` canonicalisiert (11 exakte Dubletten entfernt, Reihe 1211 Blöcke).

## An river

Origin: mountain-folge285.

- **Register-Physik-Migration gefaltet:** Mountain hält die Register-Seite und schreibt die Verdikt-Zeilen in `phi/sources.φ` auf **deine** erste Block-Charge (`gravity`/`seismic`). `Domain/Rand` vor `em` bleibt Bedingung — sag an, wenn die Schema-Charge steht.

## LOCK

- **Privater TE-Pfad (Mountain 217).** Wort „1 ja bitte" (2026-10-02, river-folge82): `complex_te_probe` um Detrend-along-p + CMI/pTE-mit-p-Kovariate erweitern (`docs/blatt/blatt-te-externer-steuerparameter.md`), Lauf lokal/silent, nie CI. Träger `state/mountain/kuprat-complex-te/`. Beide Arme gebaut, `--selftest` grün; offen: der Sweep. Riss: KDE-CMI verliert Power bei großer Kovariat-Varianz.

## Abschluss

Der Commit ist die letzte Handlung; das Commit-Wort des Operators trägt Commit und Push (dieser Atom: `/commit`).

Eigene Pfade: `phi/sources.φ` · `phi/declined_sources.φ` · `phi/blocked_sources.φ` · `docs/handover/handover-2026-10-09-mountain-folge285.md` · `docs/handover/archiv/handover-2026-10-09-mountain-folge284.md`.
