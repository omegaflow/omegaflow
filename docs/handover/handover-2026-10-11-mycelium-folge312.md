<!--
  title: Handover — Mycelium-Folge 312 (2026-10-11)
  session: Mycelium-Linie — Meta-Pass in einem Atom: EA-LiDAR-4xx-End geheilt (Ursache gemessen), THEMIS-getimetes-Keogramm-Harness gebaut (stream0), drei Register-Blöcke (mcpherron/grdc_caravan/themis_asi_timed_keogram) + fingrid/mcpherron-sha256, dropped-gate-Churn gepinnt, ENTSOE-Secret gesetzt; Stehender Pass folgt am neuen HEAD.
  class: handover
  date: 2026-10-11
  sha256: e9ef2f08b69fdaadad2d4148cf4448526418fdf2d6b5cb5f66a0a2615a2cc613
  status: live
-->
# Handover — Mycelium-Folge 312 (2026-10-11)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-11-mycelium-folge311.md` (→ `archiv/`).

## Burn: open 0.0000 · close 0.0962 — Grund: Meta-Pass (flash-first; 6 Taucher: 4×grind-flash, 2×general); kein pro/max.

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „Starte die Mycelium-Linie in einem Pass … kein Planungstheater … /commit (Push) … danach den frischen Pass" | 2026-10-11 | Operator (Session 312) · `state/operator-gespraeche/2026-10-11-mycelium.md` |
| „auf jeden eigen setzt du jetzt einen taucher an und der schliesst die aufgabe ab — das ist ein befehl" | 2026-10-11 | Operator (Session 310) |
| Sensory: `{cmb_planck,mla,ola}_compiler.rs` = Referenz, kein Träger (a) | 2026-10-11 | Operator/Sensory (Session 310) |

## Offen — eigen

### open-lidar / coverage-manifest — EA-4xx-End geheilt; Re-Dispatch offen
- **Status:** wartend · **Teile linie:mountain** | **Bindung:** eigen
- **Trigger:** neuer `coverage-manifest`-Lauf endet
- **Lage:** (gemessen 2026-10-11) Lauf `38152623110` **failure**: `ea_lidar_index`-Walk brach an `…/items?limit=1000&f=json&startIndex=158000` mit HTTP **400** ab (`page 159 returned void … the walk stays incomplete`, exit 2). **Gemessen (Taucher):** der 400 war ein Artefakt eines Zwischen-Compilers (URL trug `wksp`, dataset-id `wksp9f0fa3fc-…` → 400 „should match format uuid"); die korrekte id `9f0fa3fc-…` liefert 200/200/200 weit über 158000 hinaus, endet bei **244324** (`numberMatched`, leere `features`, kein `next`-Link). **Fix:** `ea_lidar_index.rs` mapped jedes HTTP-4xx NACH einer getragenen Seite auf `End` (Walk bricht sauber, letzte Seite ohne `next` ist das Ende); `cargo check` 0/0.
- **Blockade:** Re-Dispatch steht aus (erst nach Push); Mountain-Verdikt NL-Basis offen (`state/mycelium/lidar_id_diff_measured.md`).
- **Braucht:** `gh workflow run coverage-manifest.yml` nach dem Push; Lauf-Log lesen; Mountain-Verdikt NL.

### LiDAR-Inventar / Epochen — swisstopo+ign re-open; ahn4 2 Kacheln
- **Status:** eigen · **Teile linie:mountain** | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11, Taucher-Kachel-ID-Diff) `swisstopo` flai 39514/41982 (94,1 %; Epoche 2015–2026 gegen flai nur 2022/2023) und `ign` 337207/524687 (64,3 %; eine Kampagnen-Epoche) → **re-open**; `ahn4` 1373/1375 (99,85 %; 2 Kacheln `08bz1`/`40hz1` fehlen); `linz` ohne flai-Überlappung → bleibt.
- **Blockade:** Parser/Register-Blöcke bei Mountain.
- **Braucht:** die 2 fehlenden ahn4-Kacheln + swisstopo/ign-Epochen nachziehen ODER die Descopes neu messen; Mountain-Verdikt.

### THEMIS-ASI getimetes Keogramm — Harness gebaut; Asset-Name-Riss
- **Status:** eigen · **Teile linie:mountain** | **Bindung:** eigen
- **Trigger:** neuer `themis_asi-timed-keogram-cdn`-Lauf endet
- **Lage:** (gemessen 2026-10-11) River maß: der 310 gebaute `themis_asi_keogram_compiler.rs` spiegelt die achsenlosen stream2-JPGs — **nicht** das getimete Keogramm. Die Zeitachse liegt in den UCalgary **stream0**-Minuten-Frames (20 P5-Sub-Frames je Datei, `#"Image request start" … UTC`, ~3 s); gemessene URL `…/stream0/2026/10/01/atha_themis02/ut06/20261001_0610_atha_themis02_full.pgm.gz` (HTTP 200, 2 107 429 B). **Gebaut:** `tools/harvest/src/bin/themis_asi_timed_keogram_compiler.rs` (Walk stream0 → gunzip → TKDG-Reduce, portiert aus `tools/measure/…`; `--station/--date/--start/--stop/--ci-mode`) + `.github/workflows/themis_asi-timed-keogram-cdn.yml` + Register-Block (`phi/sources.φ:11221`, `format themis_asi_timed_keogram`); `cargo check` 0/0.
- **Blockade:** **Riss** — fester Asset-Name `themis_asi_timed_keogram.bin` gegen per-`--station/--date`-Eingaben (zweiter Lauf clobbert den ersten; Idempotenz-Guard überspringt danach); Station-Koordinaten-Tabelle + `terms` (UCalgary) fehlen.
- **Braucht:** Operator-/Mountain-Entscheid Asset-Name (`<YYYYMMDD>_<label>`-Infix?); `terms`/Koordinaten (Mountain); erster Lauf nach Push.

### GIC CSES + PCN — Läufe enden; sha256 nachtragen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** die fünf GIC-Läufe enden (`38154704083`/`38154706463`/`38154708648`/`38154711279`/`38154713840`)
- **Lage:** (gemessen 2026-10-11) die vier CSES-Compiler emittieren jetzt die pcn-Form + `# harvest:`-Header (311); `cargo check` 0/0; dispatcht.
- **Blockade:** Läufe laufen.
- **Braucht:** Ergebnisse lesen; bei grün sha256 in `phi/sources.φ` (CSES `:20156-20203`, PCN `:2183-2190`); `terms` (CSES CC0 / PCN CC-BY-4.0) + `field`-Riss (Mountain).

### GRDC-Caravan — Block registriert; Reader-Arm + field/terms offen
- **Status:** eigen · **Teile linie:mountain** | **Bindung:** eigen
- **Trigger:** Lauf `grdc-caravan-cdn` endet
- **Lage:** (gemessen 2026-10-11) `grdc_caravan_compiler.rs` + `grdc-caravan-cdn.yml` gebaut; **Register-Block registriert** (`phi/sources.φ:21503`: url/origin/compiler/`format grdc_caravan`/at earth/ttl). **Riss:** kein `field` — die gemessene Einheit ist **mm/d** (nicht `streamflow_cfs`/cfs). Lauf `38156642621` in flight.
- **Blockade:** `src/archivar/extract.rs`-Reader-Arm `grdc_caravan` fehlt (Bin noch nicht konsumierbar); `field` mm/d + `terms` (Mountain); sha256 nach erstem Lauf.
- **Braucht:** Reader-Arm + Manifest-Lauf; `field` mm/d + `terms` (Mountain); sha256 nachtragen.

### MLA-Register — repointet; mla-cdn-Ergebnis + sha256 offen
- **Status:** eigen · **Teile linie:mountain** | **Bindung:** eigen
- **Trigger:** Lauf `mla-cdn` endet
- **Lage:** (gemessen 2026-10-11) Block `phi/sources.φ:12076` repointet (`pds3_fixed_width_mlaradr0801141902.bin`; origin `…/rdr_radr/2008/jan/mlaradr0801141902.tab`); `mla-cdn.yml` gebaut; `field RANGE mla_range` steht (Mountain 319, `phi/sources.φ:12083`). **Riss:** mountain-317 behauptet `NAME = RANGE` im `.lbl`; der Taucher maß **kein** `NAME` (nur `PRODUCT_ID`/`INSTRUMENT_MODE_ID`).
- **Blockade:** `mla-cdn`-Ergebnis; NAME-Riss.
- **Braucht:** Lauf-Log lesen; sha256 nachtragen; NAME-Riss (Mountain).

### SPT-3G D1 `cmap` — Fix v2 grün; sha256 offen
- **Status:** eigen · **Teile linie:mountain** | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) Fix v2 in `8f23c7a71`; Lauf `38150991206` **success**.
- **Blockade:** keine.
- **Braucht:** sha256/Länge aus dem Release-Digest in `phi/sources.φ:11620` (Mountain-Aufenthalt).

### McPherron-Onset-Liste — Block + sha256 registriert; MCON-Bau (Mountain)
- **Status:** wartend · **Teile linie:mountain** | **Bindung:** eigen
- **Trigger:** Mountains `MCON`-Format-Verdikt
- **Lage:** (gemessen 2026-10-11) `mcpherron_compiler.rs` + `.github/workflows/mcpherron-cdn.yml` (river-188); **Lauf `38168682154` success** → Asset `mcpherron.bin` 12 896 968 B, sha256 `0c99e7d8…`; **Register-Block registriert** (`phi/sources.φ:21527`, `format substorm`, sha256 gesetzt).
- **Blockade:** das MCON-Format (Mountain) für den `SubstormOnset`-Konsum (MLT/MLAT/GLON/GLAT leer).
- **Braucht:** Mountains `MCON`-Bau; `terms` (Zenodo CC-BY-4.0) + `field` (Mountain).

### ENTSO-E A78 — Secret gesetzt; Register-Block + erster Lauf offen
- **Status:** eigen · **Teile linie:mountain** | **Bindung:** eigen
- **Trigger:** erster `entsoe-cdn`-Lauf
- **Lage:** (gemessen 2026-10-11) `entsoe_compiler.rs` + `entsoe-cdn.yml` (river-188); **`ENTSOE_API_TOKEN` in `.secrets.local` vorhanden und per `bin/secrets-sync.sh --set` als Repo-Secret gesetzt** (33 Secrets synchronisiert). Query-Parameter (`--domain/--start/--stop`) sind Platzhalter; `documentType=A78` unnetzgemessen (der erste Lauf entscheidet). Ratsverdikt: `pending`, nicht `declined`.
- **Blockade:** erster Lauf fehlt.
- **Braucht:** `gh workflow run entsoe-cdn.yml` nach Push; bei Erfolg Register-Block (url/origin/format/at earth/ttl); `terms`/`ttl` (Mountain).

### Re-Admission — GloFAS über EWDS (neuer Arm)
- **Status:** eigen · **Teile linie:mountain** | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11, Taucher) GloFAS liegt im **EWDS** `https://ewds.climate.copernicus.eu` (CEMS FLOODS), **nicht** CADS: STAC `…/api/catalogue/v1/collections/cems-glofas-historical` 200; `…/api/retrieve/v1/processes/cems-glofas-historical/execution` → 401 (Key `PRIVATE-TOKEN`/`CDS_API_KEY`). Der bestehende `copernicus::API`-Host (`src/archivar/copernicus.rs:85` CADS) erreicht es nicht. **Riss:** `docs/paper/sturzflut-tibet-pfeil.md:450` behauptet GloFAS „declined, 404" (Host falsch).
- **Blockade:** eigener EWDS-Arm fehlt.
- **Braucht:** GloFAS-EWDS-Compiler (eigene Host-Konstante) + Register-Block; `terms` (CEMS FLOODS) (Mountain).

### Fetch-Provenienz — Sidecar gebaut; Diver-Anweisungen + Roh-`curl`-Wege offen
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) `write_fetched` schreibt `<artifact>.src` an den zwei Tool-Schreibstellen; URL absent → keine Sidecar (0 honored). Die Provenienz-Regel steht in `docs/SOURCE_PORT.md`; 76 Dateien nach `data/<netloc>/` gehoben.
- **Blockade:** die Taucher-Anweisungen nutzen weiter rohe `curl -o`/Redirect-Artefakte (am Tool vorbei, kein Sidecar-Site).
- **Braucht:** `docs/SOURCE_PORT.md`/Diver-Prompts auf den Sidecar-Weg `data/<netloc>/<datei>` umstellen.

### Offene-Prosa-Träger (future-235) — 5 Dokumente, je ein nächster Schritt
- **Status:** eigen · **Teile linie:mountain** | **Bindung:** eigen
- **Trigger:** nächste Berührung je Dokument
- **Lage:** (gemessen 2026-10-11) `register_lookup --orphan-docs` = 0 (diese Zeile trägt sie). Der Fünf-Dokument-Träger aus future-235: `docs/concepts/free-voices.md` (`:87` **Falsch-Positiv** — Axiom-Name `pending` in Backticks) · `docs/concepts/tools-map.md` (`:374` **Falsch-Positiv** — Verb „wartet"; `:349` **operator-gebunden** — Debugger-Rechte am live Chrome, `opencode.json:442`, nur das Operator-Wort fehlt) · `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` (`:96` offen, Migrationsschritte operator-gated) · `docs/surveys/survey-2026-09-03-orphan-verdicts.md` (`:106` Step 5 CDN-kanonisch vs 13-netloc, destruktiv) · `docs/surveys/survey-2026-10-08-fmhy-research-landscape.md` (`:53` Seat-Re-Probe MiMo).
- **Blockade:** je Dokument ein eigener Akt (operator-gated / destruktiv / Seat).
- **Braucht:** `free-voices`/`tools-map` rewording (Marker entfernen); die drei surveys den benannten nächsten Schritt.

### Mariner-Rohband-CDN (future-236) — wartet Mountain
- **Status:** wartend · **Teile linie:mountain** | **Bindung:** eigen
- **Trigger:** Mountains `url`-Zeilen + Parser-Arme für SPDF-Roh-Tracking-Images (M4/6/7) + ASU-M10-Imaging-Rohbänder
- **Lage:** (gemessen 2026-10-11) sobald Mountain die Parser/`url`-Zeilen gebaut hat, gehört das Roh-Band in CI → CDN. Der NSSDCA-Auth-Weg (`NSSDC_USER`/`NSSDC_PASS`, **kein `archive_search`-Arm**) ist die eine Kante für `PSCM-00009` (762 7-Track-Tapes). Messung 311: die zwei genutzten Produkte (SPK `phi:31596`, Occultation `phi:20261`) sind bereits geerntet; `mariner-rst` = `descoped`.
- **Blockade:** Mountains Parser/`url`-Zeilen fehlen.
- **Braucht:** Mountains Arme; dann Mycelium `url`/`origin` + Manifest-Lauf; NSSDCA-Auth-Arm als eigene Kante.

### NIST NCNR NSE (future-235) — Route gebaut + registriert
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** erster `nist-ncnr-nse-cdn`-Lauf
- **Lage:** (gemessen 2026-10-11) `https://ncnr.nist.gov/pub/ncnrdata/nse/` HTTP 200, JSON-LD `hasPart[]` monatlicher Sets (`202608`/`202102`/…). **Gebaut:** `tools/harvest/src/bin/nist_ncnr_nse_compiler.rs` (walkt `hasPart`, emittiert `shard <url>`-Zeilen, `format coverage_manifest`, `--ci-mode` → `upload_release("ncnr.nist.gov", …)`) + `.github/workflows/nist-ncnr-nse-cdn.yml` + Register-Block (`phi/sources.φ:11718`, `format coverage_manifest`); `cargo check` 0/0. Ersetzt das descopte `trisp-keller` als offene NSE-Route für `kuprat-complex-te`.
- **Blockade:** Lauf fehlt; `field`-Riss der coverage-Klasse (`main_flow.rs:5173-5180`) für den Loader-Arm.
- **Braucht:** `gh workflow run nist-ncnr-nse-cdn.yml` nach Push; `terms` (US-PD) + `field` (Mountain); Riss `format opentopography_coverage` (Block `phi/sources.φ:20008`) vs Loader-Arm (`main_flow.rs:5106` dispatcht nur `coverage_manifest`/`usgs_lidar_coverage`) — Mountain-Zeilenklasse.

### Frame/Epoche — CRS gebaut; Wire-Doc offen
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) `projection.rs` +344 Z. (Lambert-93 `EPSG:2154`, CH1903+/LV95 `2056`, RD New `28992`); 9 Kontrollpunkt-Tests; `cargo check` 0/0. CHENyx06-Grid `pending`.
- **Blockade:** keine.
- **Braucht:** Wire-Kontrakt-Doc fortschreiben (`shard`-Grammatik mit CRS/Epoche).

### MCP no-leak — Runtime-Hälfte offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** MCP-Session-Restart → `state/mycelium/mcp_allowlist_capture_measured.md`
- **Lage:** (gemessen 2026-10-11) `opencode.json` parst; Wrapper Pin `1.9.0`; Allowlist konsistent.
- **Blockade:** Live-Seat-Prüfung braucht einen MCP-Session-Restart.
- **Braucht:** nach dem Restart die Seats live prüfen.

### Coverage-Manifest-Fetch-Loop — USGS-Manifest registrieren
- **Status:** eigen · **Teile linie:mountain** | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) `shard <url>`-Form steht (`usgs_lidar_coverage.rs:256`, Parser `src/archivar/coverage_manifest.rs:53-60`); Arm `format coverage_manifest` in `src/archivar/main_flow.rs:5102-5224`. `usgs_lidar_coverage` ist **nicht** in `phi/sources.φ` registriert.
- **Blockade:** `field`-Riss der coverage-Klasse (`main_flow.rs:5173-5180` „field undeclared") — Mountain-Zeilenklasse.
- **Braucht:** USGS-Manifest registrieren (url/origin/compiler/format/at earth/ttl = Mycelium); `field` (Mountain); 104 Nationalportale enumerieren.

### de441-de442-sha
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** erster Merge-Lauf je NAIF-Anker (`phi/sources.φ:4321-4369`)
- **Lage:** (gemessen 2026-10-11) NAIF-Assets registriert; sha256 erst nach dem Merge-Lauf.
- **Blockade:** wartet auf den Lauf.
- **Braucht:** Merge-Lauf-Ergebnis.

### dropped-gate — echter Fix (case-sensitiver Match) offen
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) `ci-gate`-Job `dropped-gate` rotete auf `new 5` (`38165122495`); die 5 Namen sind Handover-Churn (je Rename/Move/Descope mit belegendem Commit), **nicht** echte Drops — Ursache: `register_lookup` `scan_words_case` matcht case-sensitiv, pendente Tokens sind lowercase. **Disponiert:** die 5 Keys in `docs/zustand/dropped-legacy-baseline.txt` gepinnt. **Riss:** der future-235-Block behauptet „Baseline (924 keys, `278ba2905`)"; die getrackte `dropped-legacy-baseline.txt` trägt **78** Keys (+5 dieses Atoms), HEAD `7012c20f8` — die 924 ist stale.
- **Blockade:** der eigentliche Match-Fix im `register_lookup` (`config`/`dropped_gate.rs` Q2/Q3-Event-Fold) ist ein eigener CI-only-Atom.
- **Braucht:** `scan_words_case` case-insensitiv machen (oder typisiertes Event) + Gate-Test.

### SWPC/RTSW-Archivreihe — wartet Mountain
- **Status:** wartend · **Teile linie:mountain** | **Bindung:** eigen
- **Trigger:** Mountains Quelle + `ttl` genannt
- **Lage:** (gemessen 2026-10-11, Lauf `38119733631` Job `matrix-vlies`) 126 `alignment pending`-Zellen schließen erst mit einem Compiler auf die **Archivreihe**.
- **Blockade:** Mountains Quelle/`ttl` fehlen.
- **Braucht:** Mountains Quelle+`ttl`, dann Compiler + `*-cdn.yml`.

### Architektur — CI-Optimierung
- **Status:** wartend · **Teile linie:mountain/river** | **Bindung:** eigen
- **Trigger:** grüner `ci-check`-Lauf
- **Lage:** (gemessen 2026-10-11) mehrere `ci-gate`-Läufe in `subset`; `38166097283`/`38165553676` rot in `register`/`clippy`/`format` (Jobs gemessen via `ci_manage jobs`; Ursache noch nicht log-gelesen).
- **Blockade:** Shard-Build-vs-Test-Anteil erst an green `ci-check` messbar.
- **Braucht:** grüner `ci-check` → `ci_manage jobs <id>`; dann `cargo nextest archive` + Shards. Rot-Ursachen der zwei Läufe log-gelesen nachtragen.

### matrix-rotor
- **Status:** wartend · **Teile linie:river** | **Bindung:** eigen
- **Trigger:** nächster `matrix-rotor`-Lauf (externer Akt)
- **Lage:** (gemessen 2026-10-11) letzter rot = transienter Runner-Shutdown.
- **Blockade:** nur der persistente t420 ist dauerhaft.
- **Braucht:** —

### Domain→UCD — pending (Operator-Wort 2026-10-11)
- **Status:** pending | **Bindung:** eigen
- **Trigger:** eine messbare Domain-Quelle
- **Lage:** (gemessen 2026-10-11) keine `domain`-Achse; das Mapping bleibt `pending` (`state/mycelium/quantitykind-ucd-mapping.md`, 5/10 gemessen).
- **Blockade:** keine messbare Domain-Quelle.
- **Braucht:** eine messbare Domain-Quelle (kein Bau).

## An mountain

Origin: mycelium-312 (2026-10-11).

- **THEMIS-ASI-Keogramm** — der 310 gebaute `themis_asi_keogram_compiler.rs` spiegelt stream2 (achsenlos); der getimete Pfad ist stream0 (neu gebaut, s.o.). Braucht Station-Koordinaten (`on earth <lat> <lon> <alt>`) + `terms` (UCalgary) + Asset-Name-Entscheid.
- **GRDC-Caravan** — Block registriert (`phi/sources.φ:21503`); Braucht `field` **mm/d** (nicht cfs) + `terms`.
- **McPherron** — Block + sha256 registriert (`phi/sources.φ:21527`); Braucht `MCON`-Bau + `terms` (Zenodo CC-BY-4.0).
- **ENTSO-E** — Braucht `terms`/`ttl` nach dem ersten Lauf.
- **MLidar/Epochen** — swisstopo/ign re-open + ahn4 2 Kacheln + NL-Verdikt.
- **MLA-NAME-Riss** — `.lbl` trägt kein `NAME`; die `NAME = RANGE`-Zeile widerlegen oder belegen.
- **Mariner-Rohband-CDN (future-236)** — wartet auf deine `url`-Zeilen + Parser-Arme für die SPDF-Roh-Tracking-Images (M4/6/7) und die ASU-M10-Imaging-Rohbänder; dann baut Mycelium den CDN-Lauf. NSSDCA-Auth (`NSSDC_USER`/`NSSDC_PASS`) ist die Kante für `PSCM-00009`.
- **NIST NCNR NSE** — Route gebaut + registriert (`phi/sources.φ:11718`, `format coverage_manifest`); braucht `terms` (US-PD) + `field`. Riss: `format opentopography_coverage` (Block `phi/sources.φ:20008`) wird vom Loader-Arm (`main_flow.rs:5106`) nicht dispatcht.

## An river

Origin: mycelium-312 (2026-10-11).

- **Getimetes THEMIS-ASI-Keogramm (river-189 a/b)** — der Harness ist gebaut (`themis_asi_timed_keogram_compiler.rs` + Workflow + Block); die stream0-URL ist gemessen (`…/stream0/2026/10/01/atha_themis02/ut06/…`, HTTP 200). Der `stream0`-`format` ist jetzt registriert (`format themis_asi_timed_keogram`). Offen: der Asset-Name-Riss (fixed vs per-station/date).
- **ENTSO-E** — Secret gesetzt; Block + erster Lauf offen.
- **McPherron-CDN** — Lauf `38168682154` success, Block + sha256 registriert.
- **SWPC/RTSW-Archivreihe** — unverändert wartend auf Mountain.

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
