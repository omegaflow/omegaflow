<!--
  title: Handover — Mycelium-Folge 310 (2026-10-11)
  session: Mycelium-Linie — Fortsetzungs-Atom auf Operator-Befehl („auf jeden eigen einen Taucher, der abschließt"). Gebaut/geheilt: SPT-3G-D1 FITS-HDU-Offset (Fix v2), Frame/Epoche-CRS 2154/2056/28992, ea_lidar-Walk-Ende, coverage-CKAN-Arm, usgs-Manifest-Dispatch, OE-417+EAGLE-I-Compiler (+cdn.yml), DemoTER-Lizenz gemessen (nicht-kommerziell), MCP-Config verifiziert, OpenTopography dispatcht, dwd/worldclim-Testbrüche geheilt.
  class: handover
  date: 2026-10-11
  sha256: 3913fe093e2674ccd50f493b6c71c016076d9c0ac114a09c97359dd44b8d2596
  status: live
-->
# Handover — Mycelium-Folge 310 (2026-10-11)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-11-mycelium-folge309.md` (→ `archiv/`).

## Burn: open 0.0040 · close 0.165 · cap 0.25 — Grund: Operator-Befehl („auf jeden eigen einen Taucher"), 11 Taucher in einem Atom (SPT-FITS-Fix + CRS + Coverage + GIC-Compiler + DEMETER + Test-Heilungen).

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „Starte die Mycelium-Linie in einem Pass …" + „Die Übergabe IST der Stand … sie schließt als erste." | 2026-10-11 | Operator (Session 309) · `state/operator-gespraeche/2026-10-11-mycelium.md` |
| **„auf jeden eigen setzt du jetzt einen taucher an und der schliesst die aufgabe ab — das ist ein befehl"** | 2026-10-11 | Operator (Session 310) |
| Sensory: `{cmb_planck,mla,ola}_compiler.rs` = Referenz, kein Träger (a) | 2026-10-11 | Operator/Sensory (Session 310) |
| Faltbündel aus folge309 §Operator-Wort-Register (via `git show <sha>:docs/handover/archiv/…`) | 2026-10-11 | gefaltet, nicht kopiert |

## Offen — eigen

### SPT-3G D1 `cmap` (cmb-cdn) — Fix v2 gebaut (FITS-HDU-Offset); re-dispatch nötig
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) Lauf `38146848045` (Fix `a30522141`) **failure**: nach `full_220ghz.fits` schweigt der Compiler (kein stderr) und exit 1. **Ursache gemessen** (`ci_manage log --all` + Code): `FitsTable::parse` bekam den HDU-**DATA**-Start statt den HDU-Header-Start → `read_map` liest Float-Bytes als Header → `None` → bare `exit(1)`. **Fix v2** in `cmb_planck_compiler.rs` (2 Stellen: `read_table_partial` + `read_map` geben `off` statt data-start an `FitsTable::parse`); `pix2ang_ring` war korrekt. `cargo check` + `cargo build --bin cmb_planck_compiler` 0/0.
- **Blockade:** keine
- **Braucht:** (a) `gh workflow run cmb-cdn.yml` (re-dispatch); (b) bei grün sha256/Länge aus dem Release-Digest in `phi/sources.φ:11620` (Mountain-Aufenthalt).

### fingrid-cdn — weiter rot trotz gesetztem Secret
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) `FINGRID_API_KEY` per `bin/secrets-sync.sh --set` ins Repo gesetzt (Werte nie gelesen); re-dispatch `38149706994` dennoch **failure** (Ursache `unread`).
- **Blockade:** Ursache nicht gelesen.
- **Braucht:** `ci_manage log 38149706994` — die Ursache messen (Secret-Reachability im Materialize-Schritt? `read_secret`-Pfad?); dann fix + re-dispatch.

### Open-LiDAR — ea_lidar-Walk-Ende gefixt; swisstopo-Ursache + Mountain-Verdikt offen
- **Status:** eigen · **Teile linie:mountain** | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) `ea_lidar_index.rs` behandelt 404 nach ≥1 gewalkter Seite als Katalogende (`FetchOutcome::NotFound`, `walk_catalogue`, 4 Tests); transienter Void/404-Seite-1 bleibt Fehler. `cargo check` 0/0. `swisstopo`-Ursache in `38139624730` noch `unread`.
- **Blockade:** keine (Code), swisstopo unread.
- **Braucht:** (a) `ci_manage log 38139624730` für `swisstopo`; (b) Mountain-Verdikt NL-Basis (`state/mycelium/lidar_id_diff_measured.md`).

### coverage-manifest — CKAN-Arm gebaut; weitere Arme offen
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) `tools/harvest/src/bin/ckan_coverage.rs` walkt CNRA `package_search?q=title:lidar` (keyless HTTP 200; 4 Shards gemessen), `--emit-ids`-Sidecar + `--ci-mode`; Matrix-Zeile in `coverage-manifest.yml` (`data.cnra.ca.gov`). `cargo check` 0/0.
- **Blockade:** keine
- **Braucht:** weitere Arme bei Bedarf (gemessen: `wfs` = `ign_lidarhd`; `arcgis`/`s3-copc` ungeeignet für die genannten Portale).

### Coverage-Manifest-Fetch-Loop — Dispatch erweitert
- **Status:** eigen · **Teile linie:mountain** | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) `main_flow.rs:5103` dispatcht jetzt `coverage_manifest` **und** `usgs_lidar_coverage`; `cargo check` 0/0. Der Riss ist geschlossen.
- **Blockade:** das USGS-Manifest ist noch nicht in `phi/sources.φ` registriert (Datei fremd-gehalten).
- **Braucht:** USGS-Manifest mit passendem `format` registrieren (Mountain/Mycelium gemeinsam), dann Lauf.

### Frame/Epoche — CRS 2154/2056/28992 gebaut; Wire-Doc offen
- **Status:** eigen · **Teile linie:mountain** | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) `projection.rs` +344 Z. — Lambert-93 (`EPSG:2154`, GRS80), CH1903+/LV95 (`2056`, Bessel, 3-Param-Shift), RD New (`28992`, Bessel, 7-Param-Shift); 9 Kontrollpunkt-Tests (Quellen im Test); `cargo check` 0/0. `to_geodetic` liefert CRS-eigene Geodäsie (wie der UTM-Arm, kein Helmert); CHENyx06-Grid-Anwendung `pending`.
- **Blockade:** keine
- **Braucht:** Wire-Kontrakt-Doc fortschreiben (`shard`-Grammatik mit CRS/Epoche).

### GIC EAGLE-I/OE-417 — Compiler gebaut+gelaufen; Register + Workflows offen
- **Status:** eigen · **Teile linie:mountain** | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) **OE-417:** `oe417_compiler.rs` + `src/archivar/oe417.rs` (ORNL-Semicolon-CSV → 518 positionless Records, 2 Kanäle demand_loss_mw/customers_affected); **EAGLE-I:** `eaglei_compiler.rs` (figshare-CSV + Census-Gazetteer-Join 3222 Counties → `EGL1`-GeoRec, Kanal customers_out). Beide laufen live gegen die gemessenen Endpunkte, emittieren die Register-Direktiven, `--ci-mode`-Upload. `oe417-cdn.yml` + `eaglei-cdn.yml` gebaut. Wire in `extract.rs`/`geo.rs`/`main_flow.rs` verdrahtet; `cargo check` 0/0.
- **Blockade:** die Register-Direktiven (url/origin/compiler/format/quantity) gehören in `phi/sources.φ` — von fremder Linie gehalten; Mountain muss `terms`/`field` setzen.
- **Braucht:** (a) Register-Block OE-417 + EAGLE-I (Mountain terms + Mycelium url/origin); (b) die beiden Workflows dispatcht messen.

### DEMETER — Lizenz V1.1 nicht-kommerziell; **CDN-Spiegel entfernt, CI-Upload gestoppt**
- **Status:** wartend · **Teile linie:mountain** | **Bindung:** eigen
- **Trigger:** Mountains `terms`-Zeile für `regards.cnes.fr`
- **Lage:** (gemessen 2026-10-11) **Riss gelöst:** Lizenz = **CDPP Data-Centre-Lizenz V1.1** (`cdpp.irap.omp.eu/index.php/licence-sipad`, HTTP 200): nicht-kommerziell, Weitergabe an Dritte nur mit schriftlicher vorheriger Zustimmung. „Etalab-2.0" war eine Cross-Source-Fehlzuordnung (`phi/sources.φ:11142` = IGN LiDAR HD). **CDN-Befund:** der Release `regards.cnes.fr` (`omegaflow/sources`) trug **77 `demeter_isl_*.bin`** (2026-08-04 … 2026-09-12), und `demeter-cdn`-Lauf `38135966376` stand im Upload-Schritt (`--ci-mode`). **Gehandelt:** Lauf `38135966376` **cancelled**; Release `regards.cnes.fr` **gelöscht** (`gh release delete --cleanup-tag`, öffentlicher Spiegel damit weg); die beiden DEMETER-CDN-Workflows **entfernt** (kein CI-Upload-Pfad mehr). Der `--ci-mode`-Upload im `demeter_compiler.rs` bleibt als Code, hat aber keinen Lauf mehr.
- **Blockade:** keine (Lizenz erzwingt lokal-only).
- **Braucht:** lokale Registrierung (kein CDN-Asset); `terms` = CDPP V1.1 (Mountain).

### MCP no-leak — Config verifiziert (Runtime-Hälfte offen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** Config-Restart
- **Lage:** (gemessen 2026-10-11) `opencode.json` parst (`jaq -e OK`); Wrapper `bin/chrome-devtools-mcp` Pin `1.9.0`; `chrome-devtools-voices` trägt `--no-javascript-evaluation`; Allowlist-Hosts (`fe-static.deepseek.com`/`challenges.cloudflare.com`/`cdn.deepseek.com`) konsistent mit `state/mycelium/mcp_allowlist_capture_measured.md`.
- **Blockade:** die Live-Seat-Prüfung braucht einen MCP-Session-Restart.
- **Braucht:** nach dem Config-Restart die Seats live prüfen.

### OpenTopography — dispatcht
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf `38150636428` endet
- **Lage:** (gemessen 2026-10-11) Run `38150636428` in_progress (self-hosted, head `22f125ea`); Secret `OPENTOPOGRAPHY_API_KEY` im Repo gesetzt.
- **Blockade:** keine
- **Braucht:** Ergebnis lesen.

### LiDAR-Inventar / extraterr. LiDAR / Bathymetrie — Teile mountain
- **Status:** eigen · **Teile linie:mountain** | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) TNM/TNM + LOLA + CKAN verdrahtet; `bluetopo`/`emodnet_bathymetry`-Compiler stehen.
- **Blockade:** die Parser/Register-Blöcke liegen bei Mountain.
- **Braucht:** (a) je weiterer Bucket ein `<x>_coverage.rs`; (b) Parser für MOLA/MLA/NLR/OLA/Hayabusa (Mountain); (c) BlueTopo-Gestalt + Register-Blöcke (Mountain, `phi` fremd-gehalten).

### modvolc/its_live/masie/fingrid — Manifestationsstand
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `38148104363` (modvolc-cdn) endet
- **Lage:** (gemessen 2026-10-11) `masie-cdn` **success**; `its_live-cdn 38148180126` **success**; `modvolc-cdn` queued/in_progress; `fingrid-cdn` rot (s. o.).
- **Blockade:** wartet auf die Läufe.
- **Braucht:** `modvolc-cdn`-Ergebnis lesen.

### Architektur — CI-Optimierung
- **Status:** wartend | **Bindung:** eigen · **Teile linie:mountain/river**
- **Trigger:** grüner `ci-check`-Lauf
- **Lage:** (gemessen 2026-10-11) mehrere `ci-gate`-Läufe in `subset → Run cargo test --lib`.
- **Blockade:** Shard-Build-vs-Test-Anteil erst an green ci-check messbar.
- **Braucht:** grüner `ci-check` → `ci_manage jobs <id>`; dann `cargo nextest archive` + Shards.

### matrix-rotor
- **Status:** wartend | **Bindung:** eigen (Workflow) · **Teile linie:river**
- **Trigger:** nächster Lauf
- **Lage:** (gemessen 2026-10-11) letzter rot = transienter Runner-Shutdown (kein Assertion-Rot); Doc/Code-Divergenz geschlossen.
- **Blockade:** nur der persistente t420 ist dauerhaft.
- **Braucht:** —

### fingrid-cdn — Secret angelegt; re-dispatcht
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf `38151131599` endet
- **Lage:** (gemessen 2026-10-11) Ursache gemessen (river-folge187): der Repo-Secret-Liste fehlte `FINGRID_API_KEY` (Sync-Mechanismus legt Einzel-Secrets nur für zur Sync-Zeit in `.github` gelesene `secrets.<NAME>` an; Workflow kam nach dem letzten Bulk-Sync). `bin/secrets-sync.sh --set FINGRID_API_KEY` → Secret existiert jetzt (Repo-Secret-Zahl 51→52, angelegt 15:23:54Z, Werte nie gelesen); re-dispatcht `38151131599`.
- **Blockade:** keine
- **Braucht:** Ergebnis lesen.

### Future-235 §An mycelium gefaltet — `de441-de442-sha` (resolved) + `dropped-gate-move-disposition` (Riss)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11, `de44-cdn`-Lauf `37634706117` success + Release `ssd.jpl.nasa.gov-de`) `de441-de442-sha` ist **geschlossen** — die neun Register-`sha256` (`phi/sources.φ:4334-4406`) stimmen 9/9 mit dem CDN-Release-Digest (Commit `b85c0e7b72`, Mycelium-288). **Riss:** `dropped-gate-move-disposition` — die Baseline `docs/zustand/dropped-legacy-baseline.txt` ist **neu gesetzt 2026-10-11 auf 54 Keys** (`7e6e39650`), nicht mehr „924, `278ba2905`" (future-235-Zitat korrekt-historisch, aber stale); der `ci-gate`-Job `dropped-gate` ist am HEAD **rot**: `current 9 | pinned 54 | new 9` (9 neue Drop-Keys ohne Disposition, `--changed-since 0833f05ad`).
- **Blockade:** die 9 neuen Keys brauchen eine Disposition oder die Baseline eine Re-Seed.
- **Braucht:** die 9 Keys witnessen/disponieren oder die Baseline nach Messung re-seeden (`docs/zustand/dropped-legacy-baseline.txt`).

### Orphan-Prosa-Träger (future-235) — Träger in dieser Übergabe
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11, future-235) trägerlose Prosa-Dokumente mit offenem Marker: `docs/concepts/free-voices.md` (1) · `docs/concepts/tools-map.md` (2) · `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` (1) · `docs/surveys/survey-2026-09-03-orphan-verdicts.md` (1) · `docs/surveys/survey-2026-10-08-fmhy-research-landscape.md` (2).
- **Blockade:** keine
- **Braucht:** je Dokument den nächsten Schritt abarbeiten (frei/offen); Carry ist diese Zeile.

### THEMIS-ASI-Keogramm-Compiler (gefaltet aus river-187)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11, river-187) river-187 bittet um einen keylosen Compiler in `tools/harvest`, der je Nacht/Station die `stream2`-Keogramme (`.pgm.jpg`) am UCalgary-Archiv holt und per `--ci-mode` aufs CDN legt (reproduzierbare gelabelte Nächte).
- **Blockade:** keine
- **Braucht:** Bau (Taucher) + `*-cdn.yml`.

### SWPC/RTSW-Archivreihe-Harvest (gefaltet aus river-187)
- **Status:** wartend · **Teile linie:mountain** | **Bindung:** eigen
- **Trigger:** Mountains Quelle + `ttl` genannt
- **Lage:** (gemessen 2026-10-11, Lauf `38119733631` Job `matrix-vlies`) die 126 `alignment pending`-Zellen schließen erst mit einem Compiler auf die **Archivreihe** (statt ~1-Tag-Live). Vorher ein Quellen-/Gitter-Thema.
- **Blockade:** Mountains Quelle/`ttl` noch nicht genannt.
- **Braucht:** Mountains Quelle+`ttl`, dann Compiler + `*-cdn.yml`.

## Registrierte Korrekturen & Vorfälle

- **SPT-3G-D1 Wurzel gefunden (gemessen 2026-10-11):** nicht das partial-sky/RING-Parsing, sondern ein FITS-HDU-Offset (`FitsTable::parse` bekam den DATA-Start); Fix v2 gebaut.
- **Zwei Testbrüche auf HEAD geheilt (riss):** `dwd_cdc_compiler.rs` (`Station` ohne `Debug`/`PartialEq`), `worldclim_compiler.rs` (`Variable` ohne `Debug`) — `cargo check -p omegaflow-harvest --tests` jetzt 0/0.
- **DEMETER-Lizenz-Riss aufgelöst (gemessen):** „Etalab-2.0" widerlegt (Cross-Source-Fehlzuordnung); CDPP V1.1 = nicht-kommerziell → **kein CDN**.
- **fingrid weiter rot trotz Repo-Secret** — Ursache unread (`38149706994`).
- **DEMETER war auf dem öffentlichen CDN (gemessen 2026-10-11):** Release `regards.cnes.fr` trug 77 `demeter_isl_*.bin`; `demeter-cdn`-Lauf `38135966376` im Upload-Schritt. Lizenz V1.1 verbietet Weitergabe → Lauf cancelled, Release gelöscht, die zwei `demeter-*-cdn.yml` entfernt.
- **`dropped-gate`-Baseline stale (gemessen):** `docs/zustand/dropped-legacy-baseline.txt` = 54 Keys (`7e6e39650`), nicht „924, `278ba2905`"; `ci-gate`-Job `dropped-gate` rot (`current 9 | pinned 54 | new 9`).
- **Drei adressierte Blöcke gefaltet:** future-235, mountain-316, river-187.

## An mountain

Origin: mycelium-310 (2026-10-11).

- **OE-417 + EAGLE-I Register-Blöcke:** die Compiler emittieren die Direktiven verbatim — OE-417: `format oe417`, `at earth`, `ttl 604800`, `quantity oe417_demand_loss_mw … MW` + `quantity oe417_customers_affected … count`; EAGLE-I: `format eaglei_outage`, `ttl 604800`, `quantity eaglei_customers_out … count`. Braucht Mountains `terms` (OE-417 Public Domain / EAGLE-I CC-BY-4.0) + `field`.
- **DEMETER:** `terms` = CDPP Data-Centre-Lizenz V1.1 (nicht-kommerziell); **nur lokale Registrierung**, kein CDN-Spiegel.
- **descope Open-LiDAR NL-Basis** unverändert offen (`state/mycelium/lidar_id_diff_measured.md`).
- **Re-Admission-CDNs:** GRDC-Caravan/RouteViews/EPH-VSOP/GloFAS brauchen nach Mountains `terms` je url/origin (Mycelium).

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
