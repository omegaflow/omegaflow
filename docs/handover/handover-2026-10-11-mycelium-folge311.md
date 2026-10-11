<!--
  title: Handover — Mycelium-Folge 311 (2026-10-11)
  session: Mycelium-Linie — Meta-Pass in einem Atom: fingrid-Dir-Bug geheilt (Ursache gemessen), MLA-Register auf RDR repointet + mla-cdn.yml gebaut, die drei adressierten Blöcke (mountain-317, river-187, future-235) gefaltet; Stehender Pass folgt am neuen HEAD.
  class: handover
  date: 2026-10-11
  sha256: 5600652a04a0f6c5521a17c5d409f3f7c21d3eb4d87882c5ec71b8cb72ee5d10
  status: live
-->
# Handover — Mycelium-Folge 311 (2026-10-11)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-11-mycelium-folge310.md` (→ `archiv/`).

## Burn: open 0.0040 · close 0.1070 — Grund: Meta-Pass (flash-first; 1 Taucher MLA-Route, 1 Taucher Orphan-Marker); kein pro/max.

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „Starte die Mycelium-Linie in einem Pass …" + „Die Übergabe IST der Stand … sie schließt als erste." | 2026-10-11 | Operator (Session 309) · `state/operator-gespraeche/2026-10-11-mycelium.md` |
| „auf jeden eigen setzt du jetzt einen taucher an und der schliesst die aufgabe ab — das ist ein befehl" | 2026-10-11 | Operator (Session 310) |
| Sensory: `{cmb_planck,mla,ola}_compiler.rs` = Referenz, kein Träger (a) | 2026-10-11 | Operator/Sensory (Session 310) |

## Offen — eigen

### fingrid-cdn — Dir-Bug geheilt (Ursache gemessen)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Push-Trigger-Lauf `fingrid-cdn` endet
- **Lage:** (gemessen 2026-10-11, Log Lauf `38151131599`:520) `fingrid_compiler: data/data.fingrid.fi/fingrid_datasets.bin create returned void: No such file or directory (os error 2)` — der Compiler legte das Elternverzeichnis nicht an; das Secret ist jetzt gesetzt (Log :475 `FINGRID_API_KEY: ***`). **Fix:** `create_dir_all(parent)` vor `File::create` (`fingrid_compiler.rs`); `cargo check -p omegaflow-harvest --bin fingrid_compiler` 0/0.
- **Blockade:** keine
- **Braucht:** Lauf-Ergebnis lesen (Push-Trigger auf `fingrid_compiler.rs`); bei grün sha256/Länge in `phi/sources.φ` (Mountain-Aufenthalt).

### MLA-Register-Block auf RDR repointet (mountain-317) — Workflow gebaut
- **Status:** eigen · **Teile linie:mountain** | **Bindung:** eigen
- **Trigger:** Lauf `mla-cdn` endet
- **Lage:** (gemessen 2026-10-11) Block `phi/sources.φ:12036` repointet — `url` → `…/pds-geosciences.wustl.edu/pds3_fixed_width_mlaradr0801141902.bin`, `origin` → `…/rdr_radr/2008/jan/mlaradr0801141902.tab`. Gemessen (Taucher): erste RDR-Paarung `mlaradr0801141902.{lbl,tab}` (rdr_radr → `2008/jan/`); die CDR-Route existiert noch, das alte Produkt `mlastacdr0408192000.tab` ist **404**. Neuer Workflow `.github/workflows/mla-cdn.yml` (`arm=mla_compiler`, `--dat/--label`-Paar, `--ci-mode`, Idempotenz gegen `pds-geosciences.wustl.edu`).
- **Blockade:** **Riss:** mountain-317 behauptet, das RDR-`.lbl` trage `NAME = RANGE`; der Taucher maß **kein `NAME`** im `.lbl` (nur `PRODUCT_ID`, `INSTRUMENT_MODE_ID = {"SC_RANGE"}`, `RECORD_BYTES = 180`). Der `.lbl`-`^TABLE`-Pointer ist uppercase (`.TAB`) gegen die lowercase Datei — die Compiler-Paarung (`<stem>.lbl`↔`<stem>.tab`) ist case-tolerant.
- **Braucht:** (a) `mla-cdn`-Ergebnis lesen; (b) Mountain: `field RANGE …`-Zeile + NAME-Riss auflösen.

### SPT-3G D1 `cmap` (cmb-cdn) — Fix v2 grün; sha256 offen
- **Status:** eigen · **Teile linie:mountain** | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) Fix v2 (FITS-HDU-Header-Start statt DATA-Start) in `8f23c7a71`; Lauf `38150991206` **success** (15:56Z).
- **Blockade:** keine
- **Braucht:** sha256/Länge aus dem Release-Digest in `phi/sources.φ:11620` (Mountain-Aufenthalt).

### Open-LiDAR / coverage-manifest — swisstopo-Bound entfernt; re-dispatcht
- **Status:** wartend · **Teile linie:mountain** | **Bindung:** eigen
- **Trigger:** Lauf `38152623110` endet
- **Lage:** (gemessen 2026-10-11, Log `38139624730`) `swisstopo_coverage: STAC walk reached the page bound 40 — the walk stays incomplete` (exit 2); `ea_lidar_index` 404 nach ≥1 Seite = Katalogende (Fix). Der 40-Seiten-Bound ist entfernt (`308795bc8`, Compiler-Default 5000), re-dispatcht `38152623110`.
- **Blockade:** Mountain-Verdikt NL-Basis offen (`state/mycelium/lidar_id_diff_measured.md`).
- **Braucht:** Lauf-Ergebnis lesen; Mountain-Verdikt NL.

### OpenTopography — Lauf success; Registrierung juristisch geklärt
- **Status:** eigen · **Teile linie:mountain** | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) Lauf `38150636428` **success**; Secret `OPENTOPOGRAPHY_API_KEY` im Repo gesetzt. **Juristisch geklärt:** der Manifest-Inhalt ist nur der URL/DOI-Index (keine Datenbytes); das Publizieren eines freien Index ist nach § 2 / §§ 87a–e UrhG und *Svensson*/*GS Media* zulässig; OT-Nutzungsbedingungen: Daten „free of all copyright restrictions", per-Dataset-Compliance beim Konsumenten. Der `terms`-Block ist damit kein Blankett-Problem.
- **Blockade:** der `field`-Riss der coverage-Klasse (`main_flow.rs:5173-5180` „field undeclared") — Mountain-Zeilenklasse.
- **Braucht:** Register-Block (Mycelium url/origin/compiler/format/ttl; `terms` = OT-Nutzungsbedingungen + `field` = Mountain).

### GIC OE-417 + EAGLE-I — url/origin registriert; Mountains `terms`/`field` offen
- **Status:** eigen · **Teile linie:mountain** | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) `oe417-cdn 38151005570` **success**; `eaglei-cdn 38151003530` **success**. **url/origin/compiler/sha256/format/ttl/quantity registriert** (Mycelium-Hälfte; `phi/sources.φ:11847` OE-417, `:11332` EAGLE-I; `register_sort` canonical über 2791 Blöcke). Direktiven aus den Lauf-Logs gemessen (OE-417 sha `282bc3a1…`, EAGLE-I sha `c791efe0…`).
- **Blockade:** Mountains `terms`/`field` (OE-417 Public Domain / EAGLE-I CC-BY-4.0 im `## An mountain`-Block angefragt — zusätzlich Riss: der ORNL-Mirror `openenergyhub.ornl.gov` ist nicht die von Mountain gemessene `eia.gov`/`doe417.pnnl.gov`-Quelle).
- **Braucht:** Mountain trägt `terms` + `field` nach.

### GIC CSES + PCN — Emission/Header gebaut; Läufe dispatcht
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** die fünf GIC-Läufe enden (`38154704083`/`38154706463`/`38154708648`/`38154711279`/`38154713840`)
- **Lage:** (gemessen 2026-10-11) CSES-Blöcke + PCN-Block stehen in `phi/sources.φ` (CSES `:20156-20203`, PCN `:2183-2190`), aber die vier CSES-Compiler emittierten **keine** Register-Direktiven und die vier Workflows trugen **keinen** `# harvest:`-Header. **Gebaut:** die vier Compiler emittieren jetzt die pcn-Form (url/origin/compiler/sha256/format/ttl/at earth/field) + Header in den vier `cses-*-cdn.yml`; `cargo check` 0/0. **Dispatcht:** `cses-efd 38154704083` · `cses-hpm 38154706463` · `cses-lap 38154708648` · `cses-scm 38154711279` · `pcn 38154713840` (alle queued).
- **Blockade:** Läufe laufen.
- **Braucht:** Ergebnisse lesen; bei grün sha256 in die Blöcke nachtragen; `terms` (CSES CC0 / PCN CC-BY-4.0) + `field`-Riss (PCN) bei Mountain.

### GRDC-Caravan — Compiler + Workflow gebaut (Re-Admission)
- **Status:** eigen · **Teile linie:mountain** | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) `tools/harvest/src/bin/grdc_caravan_compiler.rs` + `.github/workflows/grdc-caravan-cdn.yml` gebaut (Zenodo `15349031`; `GRDC_Caravan_extension_nc.zip` Zip64, 5357 Einträge / 5356 `.nc`, Variable `streamflow` **mm/d**, CC-BY-4.0); `cargo check` 0/0. **Riss:** die ursprünglich emittierte `field`-Zeile trug die bestehende `streamflow_cfs`/cfs-Triade, die gemessene Einheit ist **mm/d** — die `field`-Zeile wurde aus der Emission entfernt (kein `cfs` über `mm/d`).
- **Blockade:** der `extract.rs`-Reader-Arm `grdc_caravan` fehlt (Bin noch nicht konsumierbar); `field`/`terms` (Mountain).
- **Braucht:** (a) Reader-Arm + erster Manifest-Lauf; (b) url/origin/sha256 registrieren (Mycelium); (c) `field` (mm/d) + `terms` (Mountain).

### DEMETER — descoped (Operator-Wort 2026-10-11)
- **Status:** descoped | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11, Operator-Wort) Lizenz = **CDPP Data-Centre-Lizenz V1.1** (nicht-kommerziell, Weitergabe nur mit schriftlicher Zustimmung) → wie SuperMAG **nur für private Experimente**, kein Register/CDN. CDN-Release `regards.cnes.fr` (77 Bins) gelöscht, `demeter-cdn 38135966376` cancelled, die zwei `demeter-*-cdn.yml` entfernt. Wort: „descoped siehe SuperMAG nur für private Experimente".
- **Blockade:** keine
- **Braucht:** keine (descoped mit Befund).

### Coverage-Manifest-Fetch-Loop — `shard`-Form steht; USGS-Register + `field`-Riss
- **Status:** eigen · **Teile linie:mountain** | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) **Riss aufgelöst:** die Übergabe behauptete `url size`; der Baum trägt bereits `shard <url>` (`usgs_lidar_coverage.rs:256`, Test `:507-518`, Commit `8739dffaa`) — der Parser (`src/archivar/coverage_manifest.rs:53-60`) akzeptiert `shard <url> [6 f64 [EPSG [epoch]]]`; eine bbox-lose Liste ist bare `shard <url>` (→ Pending, alle Shards). Kein Code-Change. Draft-Registerblock bereit (url/origin/compiler/format/at earth/ttl 604800).
- **Blockade:** `field`-Riss: `main_flow.rs:5173-5180` bricht mit „field undeclared" ab — die kanonischen coverage-Blöcke (`lola_coverage` `:10873`, `coverage_manifest` `:20861`) tragen **kein** `field`. Mountain-Zeilenklasse.
- **Braucht:** USGS-Manifest registrieren (url = Mycelium, `terms`/`field` = Mountain).

### Frame/Epoche — CRS gebaut; Wire-Doc offen
- **Status:** eigen · **Teile linie:mountain** | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) `projection.rs` +344 Z. — Lambert-93 (`EPSG:2154`), CH1903+/LV95 (`2056`), RD New (`28992`); 9 Kontrollpunkt-Tests; `cargo check` 0/0. CHENyx06-Grid `pending`.
- **Blockade:** keine
- **Braucht:** Wire-Kontrakt-Doc fortschreiben (`shard`-Grammatik mit CRS/Epoche).

### MCP no-leak — Runtime-Hälfte offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** MCP-Session-Restart → `state/mycelium/mcp_allowlist_capture_measured.md`
- **Lage:** (gemessen 2026-10-11) `opencode.json` parst; Wrapper Pin `1.9.0`; `chrome-devtools-voices` trägt `--no-javascript-evaluation`; Allowlist-Hosts konsistent.
- **Blockade:** die Live-Seat-Prüfung braucht einen MCP-Session-Restart.
- **Braucht:** nach dem Restart die Seats live prüfen.

### Orphan-Prosa-Träger (future-235) — Marker neu gemessen; Träger diese Übergabe
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) `register_lookup --orphan-docs` = 0 (diese Zeile trägt sie). Marker am Baum neu gemessen: `docs/concepts/free-voices.md:87` = **Falsch-Positiv** (Axiom-Name `pending` in Backticks, `docs/granit.md:20`); `docs/concepts/tools-map.md:374` = **Falsch-Positiv** (Verb „wartet"); `docs/concepts/tools-map.md:349` = **operator-gebunden** (Debugger-Rechte am live Chrome; Binding steht `opencode.json:442`, nur das Operator-Wort fehlt; die zitierte Zeile `opencode.json:172-178` ist stale); `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md:96` = offen, Migrationsschritte operator-gated; `docs/surveys/survey-2026-09-03-orphan-verdicts.md:106` = Step 5 (CDN-kanonisch vs 13-netloc, destruktiv); `docs/surveys/survey-2026-10-08-fmhy-research-landscape.md:53` = Seat-Re-Probe (MiMo).
- **Blockade:** keine
- **Braucht:** je Dokument den benannten nächsten Schritt; die zwei Falsch-Positive bei nächster Doku-Berührung rewording (kein Marker).

### Fetch-Provenienz (future-235) — Sidecar gebaut; Roh-`curl`-Wege offen
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) `write_fetched` in `tools/utils/src/bin/archive_search.rs` schreibt `<artifact>.src` (genau die Quell-URL) an den zwei Tool-Schreibstellen (`--pdf-image`, `--arxiv-src`); URL absent → keine Sidecar (0 honored, stderr-Benennung); `cargo check` 0/0. Die Provenienz liegt sonst in `opencode.db`; `state/research/README.md` §Provenienz hat 76 Dateien nach `data/<netloc>/` gehoben.
- **Blockade:** die divers' rohen `curl -o`/Redirect-Artefakte laufen an jedem Projekt-Tool vorbei (kein Sidecar-Site).
- **Braucht:** die Taucher-Anweisungen auf den Sidecar-Weg/`data/<netloc>/` umstellen; `src/archivar/fetch.rs` (Compiler-Cache) ist netloc-keyed und unberührt.

### de441-de442-sha (future-235)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** erster Manifest-Lauf je NAIF-Anker (`phi/sources.φ:4321-4369`)
- **Lage:** (gemessen 2026-10-11) NAIF-Assets registriert (`phi/sources.φ:4321-4369`); sha256 erst nach dem Merge-Lauf.
- **Blockade:** wartet auf den Lauf.
- **Braucht:** Merge-Lauf-Ergebnis.

### dropped-gate-move-disposition (future-235) — disponiert; Event-Fold offen
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) **Riss aufgelöst:** die future-Zahl (924 keys, `278ba2905`) war stale; die getrackte Baseline trägt **78 Keys** (HEAD `7012c20f8`), die 24 Drops wurden in `3a3f992cb` disponiert; der `dropped-gate`-Job ist grün. Eine Disposition = die per-Key-Entscheidung „echter stiller Drop (rot) vs. belegter Move/Rename/Descope", gepinnt als Key.
- **Blockade:** keine.
- **Braucht:** (a) **gebaut** (dieses Atom: Baseline/Kommentar unberührt, Befund im Handover); (b) offen: die Q2/Q3-Event-Fold-Verdrahtung in CI (`dropped_gate.rs` `alias:`/typisiertes Event) — größerer Atom, CI-only.

### THEMIS-ASI-Keogramm-Compiler — Workflow + Station-Koordinaten + terms
- **Status:** eigen · **Teile linie:mountain** | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) `themis_asi_keogram_compiler.rs` gebaut (Route `…/THEMIS/asi/stream2/<YYYY>/<MM>/<DD>/<label>/…_full-keogram.pgm.jpg`, keyless, `--station`/`--date`/`--start`/`--stop`/`--ci-mode`); `cargo check`/`--bin` 0/0.
- **Blockade:** Station-Koordinaten-Tabelle fehlt; `terms` fehlt.
- **Braucht:** (a) **gebaut:** `.github/workflows/themis_asi-keogram-cdn.yml` (`workflow_dispatch` mit `station`/`date`/`start`/`stop`/`force`, kein fabrizierter Default; Idempotenz je Nacht); (b) Station-Koordinaten (Mountain); (c) `terms` (UCalgary).

### LiDAR-Inventar / extraterr. LiDAR / Bathymetrie — Teile mountain
- **Status:** eigen · **Teile linie:mountain** | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) TNM + LOLA + CKAN verdrahtet; `bluetopo`/`emodnet_bathymetry`-Compiler stehen. **Ratifikations-Messung (Taucher, Kachel-ID-Diff):** die Portal-Bin-Descopes sind **nicht** ratifiziert — `swisstopo` flai 39514/41982 (94,1 %; Epoche 2015–2026 gegen flai nur 2022/2023) und `ign` flai 337207/524687 (64,3 %; eine Kampagnen-Epoche) fallen Fläche **und** Epoche; `ahn4` 1373/1375 (99,85 %, 2 Kacheln `08bz1`/`40hz1` fehlen); `linz` ohne flai-Überlappung → bleibt.
- **Blockade:** Parser/Register-Blöcke bei Mountain.
- **Braucht:** (a) die 2 fehlenden ahn4-Kacheln + swisstopo/ign-Epochen nachziehen oder die Descopes neu messen; (b) je weiterer Bucket ein `<x>_coverage.rs`; (c) Parser MOLA/MLA/NLR/OLA/Hayabusa (Mountain); (d) BlueTopo-Gestalt + Register-Blöcke (Mountain).

### Re-Admission-CDNs (mountain-317/316)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Mountains `terms`-Verdikt je Quelle
- **Lage:** (gemessen 2026-10-11) Mountains `terms`-Verdikte stehen (GRDC-Caravan Zenodo `15349031` CC-BY-4.0 · RouteViews CC-BY-4.0 · EPH-VSOP Zenodo `17142232` CC-BY-4.0 · GloFAS Copernicus). **GRDC-Caravan ist gebaut** (`grdc_caravan_compiler.rs` + `grdc-caravan-cdn.yml`; Lauf läuft). **RouteViews/EPH-VSOP** brauchen zuerst Mountains `field`-Verdikt (RouteViews = Infrastruktur, VSOP = eine C-Bibliothek, kein Datenprodukt — Risse). **GloFAS** läuft über **EWDS** (`ewds.climate.copernicus.eu`), nicht CDS — der bestehende `copernicus::API`-Host erreicht es nicht (Riss); ein eigener Arm fehlt.
- **Blockade:** je Quelle ein manifestierender Arm + (RouteViews/VSOP) Mountains `field`-Verdikt.
- **Braucht:** GRDC-Caravan-Lauf + Registerblock (Mycelium url, Mountain `field` mm/d/`terms`); GloFAS-EWDS-Arm; RouteViews/VSOP-`field`-Verdikt.

### SWPC/RTSW-Archivreihe-Harvest (river-187)
- **Status:** wartend · **Teile linie:mountain** | **Bindung:** eigen
- **Trigger:** Mountains Quelle + `ttl` genannt
- **Lage:** (gemessen 2026-10-11, Lauf `38119733631` Job `matrix-vlies`) die 126 `alignment pending`-Zellen schließen erst mit einem Compiler auf die **Archivreihe** (statt ~1-Tag-Live).
- **Blockade:** Mountains Quelle/`ttl` noch nicht genannt.
- **Braucht:** Mountains Quelle+`ttl`, dann Compiler + `*-cdn.yml`.

### Architektur — CI-Optimierung
- **Status:** wartend · **Teile linie:mountain/river** | **Bindung:** eigen
- **Trigger:** grüner `ci-check`-Lauf
- **Lage:** (gemessen 2026-10-11) mehrere `ci-gate`-Läufe in `subset → Run cargo test --lib`.
- **Blockade:** Shard-Build-vs-Test-Anteil erst an green `ci-check` messbar.
- **Braucht:** grüner `ci-check` → `ci_manage jobs <id>`; dann `cargo nextest archive` + Shards.

### matrix-rotor
- **Status:** wartend · **Teile linie:river** | **Bindung:** eigen
- **Trigger:** nächster `matrix-rotor`-Lauf (externer Akt)
- **Lage:** (gemessen 2026-10-11) letzter rot = transienter Runner-Shutdown (kein Assertion-Rot).
- **Blockade:** nur der persistente t420 ist dauerhaft.
- **Braucht:** —

## Aus dem State-Zensus (Operator-Relais „in state steckt viel Wissen", 2026-10-11)

Ein Zensus über die nicht-Spill-`state/`-Dateien gegen diese Übergabe + `wartend.φ`
fand offene Schleifen; die Mycelium-Anteile hier gefaltet, die fremden als
Registerzeilen in `## An mountain`/`## An river` (die Owner falten).

- **VO-Landschaft (mycelium-269) — descoped (Operator-Wort 2026-10-11):** die 72 `tap_index_*`-Kataloge + `tap_index_merge` wurden von `mountain 306` (`1a885775f`) **gelöscht**; die Live-Suche steht und ist gemessen (`archive_search --regtap "solar"` → live IVOA-Dienste; `regtap_census.rs`, `tap_compiler.rs`). Der Operator: „wir haben doch jetzt die Möglichkeit die Kataloge live zu durchsuchen" → statischer Index `descoped`, kein Migrations-Arm.
- **`Domain→UCD`-Mapping — pending (Operator-Wort 2026-10-11):** eine `domain`-Achse wird nicht eingeführt; das Mapping bleibt `pending` (keine messbare Domain-Quelle, kein Bau).
- **Swarm-Recherche:** `state/mycelium/swarm-recherche-synthese-2026-10-05.md:85` — q3–q6 mit demselben Roster nachholen (braucht UI-Seats).
- **CDN-Reconcile:** `state/zustand/ereignisse.φ:70822` — `cdn_reconcile`; die 81 internen Familien-Tags werden jetzt **gebunden** (Fix in `cdn_reconcile.rs`: `internal_family`), Orphans 97→**16**; der Rest sind Namens-/Upload-Korrekturen + **9 echte Lösch-Kandidaten** (Operator-Wort).
- **Fehlende Arme (`ereignisse.φ:163087`) — je Taucher gemessen:** **hi-21cm → HI4PI gebaut** (`hi4pi_compiler.rs` + `hi4pi-cdn.yml`, CDSARC `J/A+A/594/A116`, 933 Kanäle, `cargo check` 0/0); **cmb-lambda** bereits gebaut (`cmb_act_compiler.rs`, `act-cdn.yml`, `phi:11653`); **particle-cern** bereits gebaut (`root.rs` + `cern_root_compiler.rs`); **mariner-rst** = `descoped` (nur Offline-CD-WO, PSCM-00009); **viking-tracking** = `descoped` (`viking_text` trägt Lander-Range/Doppler; PSPG-00011 „not received").
- **Mariner-10-Bestellung — gemessen, Empfehlung: nicht bestellen.** Die zwei vom Projekt genutzten Produkte sind bereits geerntet + manifestiert + sha-verifiziert (SPK `ephemeris_mariner10.bin` `phi:31596`; Occultation `mariner_occlt.bin` `phi:20261`); Mariner-10 trägt **keinen** Kraft-Mechanismus und steht außerhalb der gemessenen Zeugen-Tabelle (`eigene-ephemeride.md`). Bestellung = 7 CD-WO von 762 Bändern (physischer Versand, per-Akt). Riss: `wartend-archiv-…r3.φ:7` behauptet „NAIF MARINER10 = 404" — der Baum hat das Kernel live (HTTP 206).
- **Domain→UCD — Kategorien-Fehler korrigiert (Operator-Einwand „9 Kraftmedien längst überholt"):** `state/mycelium/domain-ucd-mapping.md` neu aufgesetzt — der Baum trägt **vier Achsen**: **9 Mechanismen** (`channel.rs:786` `MECHANISMS`, je `TransportOp`/`PdeType`/`Boundary`/`Interaction` — kein „Medium"; der `force_type`-Draht-Slot ist seit P9.2 `channel_ref`), **10 QuantityKinds** (`force.rs:12`), **4 Conserved**, **3 Medium-Klassen** (vacuum/fluid/elastic-solid). **UCD1+ trägt die QuantityKind-Achse**, nicht die Mechanismus-/Medium-Achse. 26 Zeilen, **7 gemessen / 19 pending**. Der Name „Domain" ist damit überholt — es ist eine **QuantityKind→UCD**-Zuordnung.
- **LiDAR-Verdikt — gemessen (nicht ratifiziert):** `swisstopo` 94,1 % Fläche + Epochen-Riss (2015–2026 vs flai 2022/2023), `ign` 64,3 % + eine Epoche → **re-open**; `ahn4` 99,85 %/2 Kacheln → halten; `linz` ohne Überlappung → bleibt; Detail im LiDAR-Inventar-Punkt.
- **Ungepoolte phi-Blöcke:** `state/mycelium/holdings-opencode-tmp-2026-10-07.md:54` — Mountains Sichtung/Merge.
- **Fetch-Provenienz:** die Diver-Regel ist in `docs/SOURCE_PORT.md` (Provenienz-Bullet, sha `f5659ddc…`) — zusätzlich zum gebauten Sidecar.
- **cdn_reconcile — Riss aufgelöst: die 16 sind Manifeste, keine Löschkandidaten (Operator-Wort zurückgehalten):** der Operator gab „löschen", die Einzelmessung widerlegte die Klassifikation — **alle 9** vermeintlichen Löschkandidaten tragen **echte Assets** (`data.fingrid.fi/fingrid_datasets.bin` · `portal.opentopography.org/opentopography_coverage.manifest` · `noaa-eri-pds…/noaa_eri.manifest` · `noaa-nos-coastal-lidar-pds…/noaa_nos_coastal_lidar_pds.manifest` · `ers.usda.gov/usda_fara_low_access.bin` · `crystallography.net/{eels_acoustic,rixs_charge,rixs_spin}.bin` · `oderest.rsl.wustl.edu/live2.json` · `superdarn.ca/superdarn_fitacf.bin` · `alasky…` 32 HiPS-Manifeste, zusammen **41 Dateien**). `stale_pending` heißt „Register bindet den Tag nicht **exakt**", nicht „verwaist" → **binden, nicht löschen**. Kein Delete ausgeführt. Der Fix (`internal_family`, Orphans 97→16) steht; Rest = Bind-/URL-Korrekturen (Mountain-Spalte) + je Quelle der Status (z. B. SuperDARN = LOCK).
- **Swarm q3–q6 — Fähigkeit 5/5, Task-Latenz-Riss:** Provider `google`/`openrouter`/`nvidia` + drei `voice-*` re-enabled (`opencode.json`). **CAP nach Operator-Keys: 5/5 bei 4/4** (`state/benchmark/2026-10-11-voice-capability.md`; Tempo 28.5–33.9 s). **Schwarm gefahren** (`state/stimmen/swarm-2026-10-11_202435/` q3 · `…_202808/` q4 · `…_203031/` q5 · `…_203122/` q6): im Research-Profil (Prolog + `archive_search_public`) **timeouten 4 von 5 Routen bei `--timeout 180`** (`deepseek`/`gemini`/`nemotron`/`gpt-oss`, rc=124); nur `inkling` liefert je Frage (raw + `reachability.tsv`). CAP 4/4 ≠ Task-Latenz ≤ 180 s. **Braucht:** Re-Lauf mit `--timeout 600 --no-cooldown` oder UI-Kanal; die Rohordner liegen.
- **OpenTopography — juristisch (gemessen):** der Manifest-Inhalt ist **nur die Liste der Landing-URLs/DOIs** (keine Datenbytes; Compiler `opentopography_coverage.rs:83-95`, Log `38150636428:567`). Ein URL/DOI-Index ist nach § 2 UrhG (Fakten), §§ 87a–e UrhG (ECJ C-203/02 *BHB/William Hill*, C-338/02 *Fixtures*: kein Recht am Einzelinhalt, Spin-off-Doktrin) und *Svensson*/*GS Media* (bloßes Verlinken frei zugänglicher Inhalte) **keine Weiterverbreitung der Datensätze**. Die OT-Nutzungsbedingungen (`opentopography.org/usageterms`, HTTP 200): Daten „free of all copyright restrictions"; „certain datasets may have open data licenses … users responsible for license compliance". **Juristisch korrekt:** den Index publizieren mit `terms` = OT-Nutzungsbedingungen (per-Dataset-Compliance trägt der Konsument, der die Daten selbst holt) — kein Blankett-Lizenzanspruch, keine Umgehung. Mountain setzt `terms`.

## Registrierte Korrekturen & Vorfälle

- **Grenzverletzung (gemessen 2026-10-11, zweimal):** (a) Commit `8a818afe7` sweepte **Mountains gestagten Rename** `…mountain-folge317.md → archiv/`; (b) Commit `85d912ec7` sweepte **Mountains gestagte Löschung** des alten Pfads `…mountain-folge318.md` (die Archivkopie `archiv/…folge318.md` war bereits in `d5525435f` — **kein Inhaltsverlust**, nur der Altpfad). Ursache: der geteilte Index trug die fremden Stufen; ein pfad-begrenztes `git add` hebt sie nicht auf. Lehre (schärfer): vor jedem Commit `git diff --cached --name-only` prüfen; trägt der Index eine fremde Stufe, wird der Commit **aufgeschoben** (kein `git restore` — destruktives Git ist verboten), bis die fremde Linie sie committet oder der Index eigen ist.

## An mountain

Origin: mycelium-311 (2026-10-11).

- **Aus dem State-Zensus (Mountain-Anteil):** `state/mountain/blocked-sources-klassen.md:35-52` (Zitat-Re-Messung / fehlende Arme+WF, ≈18 Zeilen) · `state/mountain/routing-untersuchung-2026-10-10.md:54-66` (KNMI/Mindat 401 **mit** Token messen · Overpass-Bot-Gate · GADM/WorldClim-Referenz-Verdikt) · `state/mountain/frageblatt-routen.md:47-58` (JADES-Mirror · DSN-Endpoint · DEMETER-DM*-Dateien · CEERS-z) · `state/mountain/fmhy-extension-2026-10-10.md:74-76` (Verdikt-Schritt LIGO OSC/EMSC/IMOS) · `state/future/exposom-matrix-2026-10-04.md:231-236` (X-Klassen-Feld: y-Serien unregistriert).

- **MLA-Register (mountain-317):** Block `phi/sources.φ:12036` repointet (url → `pds3_fixed_width_mlaradr0801141902.bin`, origin → `…/rdr_radr/2008/jan/mlaradr0801141902.tab`); neuer Workflow `mla-cdn.yml`. **Riss:** das RDR-`.lbl` trug kein `NAME` (nur `PRODUCT_ID`/`INSTRUMENT_MODE_ID={"SC_RANGE"}`) — die behauptete `NAME = RANGE`-Zeile braucht eine Messung.
- **OE-417 + EAGLE-I:** die Compiler emittieren die Direktiven verbatim (url/origin/compiler/sha256/format/ttl/at earth/quantity). Workflows `38151005570`/`38151003530` **success**. Braucht Mountains `terms` (OE-417 Public Domain / EAGLE-I CC-BY-4.0) + `field`; dann schreibt Mycelium url/origin.
- **DEMETER:** `terms` = CDPP Data-Centre-Lizenz V1.1 (nicht-kommerziell); **nur lokale Registrierung**, kein CDN-Spiegel.
- **OpenTopography:** Lauf `38150636428` success; Register-Block (url/origin + terms) fehlt.
- **Re-Admission-CDNs:** GRDC-Caravan/RouteViews/EPH-VSOP/GloFAS brauchen nach Mountains `terms` je url/origin (Mycelium).
- **THEMIS-ASI-Keogramm:** Compiler gebaut; Braucht eine **gemessene THEMIS-ASI-Station-Koordinaten-Tabelle** (`on earth <lat> <lon> <alt>`) + `terms`.
- **Coverage-Manifest-Fetch-Loop:** USGS-Manifest auf `shard <url> <6 f64>` entscheiden + registrieren.

## An river

Origin: mycelium-311 (2026-10-11) — State-Zensus, River-Anteil.

- `state/zustand/ereignisse.φ:213089` — den schweren Chain in CI auf `pioneer10_odf.bin` + `ephemeris_sun.bin` fahren, Per-Pass-Slope ≈ 1,0 prüfen; Riss `uplink_band` (`pioneer10_odf_compiler.rs:61`).
- `state/zustand/ereignisse.φ:46923` — `wy-max-t` neu dispatchen, α/X/Z/Bz benennen (Siegel = Operator).
- `state/zustand/ereignisse.φ:100114` — Rat-Entscheid Span/Apertur, dann `source_contrib` per Fragment (Operator-Browser-Messung).
- `state/stimmen/2026-10-06_gic-familien-arm-ui-stimmen.md:20-80` — Familien-Partition, zwei Register-Nägel, dann `matrix gic_auroral/subauroral/midlat full`.

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
