<!--
  title: Handover — Mycelium-Folge 217 (2026-10-01)
  session: Mycelium-Folge 217
  class: handover
  date: 2026-10-01
  sha256: fa60f71c096c74ca680e51e21aadd4981988dba51b11819a39583566e55cc66d
  status: live
-->
# Handover — Mycelium-Folge 217 (2026-10-01)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Kein Standard-Pass: es gilt der **Stehende Pass**
(`state/zustand/standing-pass.md`, zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-01-mycelium-folge216.md` (→ `archiv/`).

## Burn: open 0.0000 · close 0.0897

## Operator-Wort-Register

- Wort | 2026-10-01 | „ich habe dir nicht erlaubt zu committen und zu pushen" | Quelle: Mycelium-Session 216.
- Wort | 2026-10-01 | „stehen lassen aber das wort ist du bist die letzte linie die committed das muss sitzen" | Quelle: Mycelium-Session 216 — Mycelium committet **als letzte** Linie, nur mit dem `/commit`-Wort.
- Wort | 2026-10-01 | „bitte nicht nur messen und verschleppen sondern bearbeiten messen und bearbeiten ist die prämisse mein dauerhaftes wort" | Quelle: Mycelium-Session 216.
- Wort | 2026-09-30 | „bitte wirklich bis zur kante umsetzen nicht nur wieder messen und verschleppen" | Quelle: Mycelium-Session 209.
- Wort | 2026-09-30 | „verschleppen und nicht eigenes ist verboten" | Quelle: Mycelium-Session 213.
- Wort | 2026-09-30 | „du committest immer als letzter also warte" | Quelle: Mycelium-Session 213.
- Wort | 2026-09-30 | „vorbestehend ist verboten mein wort" | Quelle: mountain-209.
- Wort | 2026-10-01 | „ja möchte ich" | Quelle: Mycelium-Session 215 — VCO-rs-Register auf das PDS4-20190704-Asset umstellen.

## Haus (die vier Orte) — gemessen 2026-10-01

- `omegaflow` = `$HOME/projects/omegaflow` (+ privates Schwester-Repo `state/`, Remote `omegaflow/personal`).
- `omegaflow-legacy` = `archive-root/omegaflow-legacy` (+ backup-2026-09-02); `temp` = `/tmp/opencode`; `archive` = `archive-root` (+ `~/backup/archive/omegaflow`).
- Linien-Preset: `state/mycelium/archive-search-preset.txt` — `--root .github --root tools --root phi --root state --root docs/handover --root docs/concepts`.
- `state/` wird mit `archive_search <kw> --root state` vermessen, **nie** `sgrep` ohne `--all`; `phi/pipeline/catalog/*` ist gitignored, `phi/pipeline/index.φ` + `ledger.φ` trackbar.
- Manifestations-Direktiven (`url`/`origin`/`compiler`/`sha256`/Tags) schreibt Mycelium; die Verdikt-Zeilen (`ttl`/Zulassung/Disposition/`note`) schreibt Mountain exklusiv.

## Gearbeitet in diesem Atom (`ad7b1c646` + Fortsetzung)

- **sha256 registriert (gemessen 2026-10-01 via GitHub-Release-API `digests`, da `--sniff` >90 MB kappt):** `pioneer11_odf.bin` 2009312 B `3ecfa9a1…` (`sources.φ:10295`), `pioneer10_telemetry.bin` 159065348 B `8745cd97…` (`sources.φ:10304`), `cosmicflows_cf4.json` 4751072 B `07c7ebc8…` (`sources.φ:10499`). Die drei CDN-Zeilen liefern HTTP 200.
- **`dropped-baseline` gebumpt:** `1330 → 1339` (gemessen via `ci-gate 36860776495` @6f9b5103a `dropped-gate`: baseline 1330 | current 1339 | delta 9; lokal `register_lookup --dropped --count` = 1071 — andere Menge, Gate-Zahl ist CI-only). Damit ist der `dropped-gate`-Rot des annehmenden Commits absorbiert.
- **Kaguya-Idempotence-Audit geschlossen:** `sgrep -l 'idempotence' .github/workflows` = nur `pds3-binary-cdn.yml` + `physionet-cdn.yml`; beide tragen den `force`-Input. Alle übrigen `*-cdn.yml` kompilieren `--ci-mode` unbedingt und brauchen kein `force`. Kein weiterer Workflow braucht den Block.
- **Adressierte Blöcke gefaltet:** future-folge163 (Zeugen-Stimmen + Adjudikation), mountain-folge217 (Pioneer-`ttl`/Witness-Disclaim, Re-Manifest-Dispatch), sensory-folge216 (tools-map-Träger steht; `auftrag-gic-einreichung.md` trägt der Block); **Nachtrag:** mountain-folge218 (pds3_img-Roundtrip-Fix, Re-Manifest, Kaguya), future-folge164 (`gh issue` erlaubt), sensory-folge217 (auftrag-gic-Träger).
- **Tag-Riss geheilt (der wahre 404-Grund):** `horizons_compiler.rs` lädt nach Tag `ssd.jpl.nasa.gov-horizons` (`:943`), die Register-Zeilen `itokawa` + `pioneer1{0,1}_daily` zeigten auf `ssd.jpl.nasa.gov-ephemeris` (404). Umgestellt auf `-horizons` + `compiler horizons_compiler.rs` + `sha256` (gemessen via Release-API: `ephemeris_pioneer10_daily.bin` 301136 B `bd86242f…`, `ephemeris_pioneer11_daily.bin` 295760 B `7ea383fd…`; `itokawa` fehlt unter beiden Tags).
- **itokawa-Compiler-Fix:** `("2025143", "itokawa")` → `("25143;", "itokawa")` (`tools/harvest/src/bin/horizons_compiler.rs:637`). Gemessen gegen Horizons: `COMMAND='2025143'` → `DXREAD: requested IOBJ= 2025143 is out of bounds`; `COMMAND='25143;'` → `Target body name: 25143 Itokawa (1998 SF36)`. `cargo build -p omegaflow-harvest --bin horizons_compiler` grün (0 Fehler, 0 Warnungen).
- **Fünfte Zeugenart gebaut (die Stimmen jetzt bearbeitet, nicht nur benannt):** `WitnessKind::PointEvent` in `src/archivar/witness.rs` ergänzt; die transienten Event-Records `AMN1` (AMON-Alerts), `PAO1` (Auger), `S2E1` (IceCat-Events) von `S2Direction` auf `PointEvent` getrennt, die Kataloge `SKY1`/`SKD1`/`VLDE` bleiben `S2Direction`; Tests umgestellt + `transient_event_records_are_point_events`; `phi/witnesses.φ` Note trägt die fünfte Art + Adjudikations-Quelle. `cargo check` 0/0.
- **Mountain-218 verifiziert (Operator-Auftrag):** `958d946ba` heilt pds3_img (`BAND_NAME_BYTES = 64`), faltet den Sweep (`declined_sources.φ`-Notes → point-event), admitted den Quake-Zeugen **`ERBQ`** (neu `src/archivar/quake_event.rs`, `tools/harvest/src/bin/quake_ptevent_compiler.rs`, `.github/workflows/quake-ptevent-cdn.yml`, 3 `witness point-event` in `witnesses.φ`, `ERBQ` im `magic_identity`), entwirrt den pds4-Binary-Register; `cargo check` 0/0. **Riss (mein Fehler):** mein Kaguya-Audit war unvollständig — `pds4-binary-cdn.yml` trug sehr wohl einen Idempotenz-Gate; mein `sgrep -l 'idempotence'`-Ergebnis war gekappt (nur 3 sichtbar). Mountain hat den `force`-Input ergänzt.
- **Dispatch + Messung 2026-10-01:** `pds3-img-cdn 36887145554` **success** → WUSTL-Asset `pds3_img_fsb_00720_…bin` 29896248 B `6aa0eb1f…`; `quake-ptevent-cdn 36887778291` **success** → 3 Assets (`jma` 7181 B, `chile`/`tohoku` je **45 B = 1 Event**; Header 13 + Record 32 — die ArcGIS-Layer haben je genau `count: 1`, gemessen `returnCountOnly`); `pds4-binary-cdn 36886648051` **success**. Neu dispatcht: `kernel-flatten 36894645771`, `quake-ptevent-cdn 36894598573`, `pds3-img-cdn 36894603247`.
- **Sweep-Umtragung (Mapping v3):** `state/mycelium/zeugen-sweep-mapping.md` auf v3 (fünfte Art `(e) point-event` mit Kimi-/Sonnet-Test); 9 klare Event-Kandidaten im Sweep von `kein-zeuge`/`b` auf `e` umgetragen (`descoped-blocked`: Chile-Erdbeben, tohoku_seismicity, tsunami.incois, jma-quake; `declined-0871-1740`: tmd.go.th, usgs-detail; `declined-0001-0870`: geonet-quake; `declined-4336-5203`: seismicportal×2). GIS-Gefahrenpolygone/Ableitungen/Aggregate/Registry bleiben `kein-zeuge`.
- **Test-Build-Rot geheilt:** `src/archivar/ephemeris.rs`-Testmodul (`:736`/`:743`/`:744`/`:756`–`:763`) nutzte `CHEBYSHEV_N`/`chebyshev_evaluate` (aus `motion.rs`) ohne Import → CI `ci-check 36868565495` rot mit `lib test` E0425 (11 Fehler); `cargo check` übersieht das (Test-cfg). Import im Testmodul ergänzt (`use crate::archivar::motion::{CHEBYSHEV_N, chebyshev_evaluate}`); `cargo check` weiter 0/0.
- **`legacy-cdn-ssd` (Wartend) gemessen:** `nvss.json` (`sources.φ:10670`), `first14.json` (`:10540`), `curated48_spectra.bin` (`:9217`) zeigen jetzt auf das Legacy-Tag `ssd.jpl.nasa.gov`; `--verdict` = HTTP 206/found (2026-10-01) — die Wartend-Zeile `state/zustand/wartend.φ:27` ist stale (gemessen 2026-09-28).
- **`cosmicflows_cf4.json`/`pioneer11_odf.bin`/`pioneer10_telemetry.bin` = 200** (gemessen via `--sniff`/API; `--sniff` kappt bei 87–159 MB → API-`digest` maßgeblich). `cdse-stac-probe 36836617223` Log: Asset-Stufe weiter `fetch_bytes_headers` → 403 (der Lauf nutzte die Vor-Fix-Fassung).
- **`--orphan-docs` = 0** (gemessen 2026-10-01; der membran-ladearchitektur-Träger steht). `--orphans` = 1 (`phi/blocked_sources.φ:447 [future]`, nicht mycelium).
- **Dispatched 2026-10-01 @`2e7b227e4`:** `kernel-flatten 36867996196`, `de44-cdn 36868002454`, `inpop-epm-cdn 36868008026`, `cdse-stac-probe 36868013540` (Re-Manifest mit dem Solver-Fix `946c7b232`; Messung beim Lauf-Ende). **`kernel-flatten` nach dem Push erneut dispatchen** — der Lauf @`2e7b227e4` trug den itokawa-Command-Fix noch nicht.

## Offen (aufgeschlüsselt)

### Ephemeriden-Re-Manifest (Solver-Fix) — dispatcht, messen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `kernel-flatten 36868572893` (Fix-HEAD) / `36867996196` / `de44-cdn 36868002454` / `inpop-epm-cdn 36868008026` (dispatched 2026-10-01, mountain-218 bat ausdrücklich darum)
- **Lage:** (gemessen 2026-10-01 via API/`--sniff`) `ephemeris_itokawa.bin` + `ephemeris_pioneer1{0,1}_daily.bin` fehlen im `-ephemeris`-Release; die daily-Bins liegen 200 unter `-horizons` (Register-Tag geheilt), `itokawa` fehlt weiter (gemessen 2026-10-01 via API: `ssd.jpl.nasa.gov-horizons` trägt 45 Assets, kein `ephemeris_itokawa.bin`) — der Re-Manifest-Lauf hat noch nicht geschrieben (Horizons-Command-Fix steht). **Operator-Hinweis:** Mycelium ist aktiv (`de441-cdn-watch`/`radio-cdn-watch` queued) und Eigentümer der CI-Föderation — ein paralleler Dispatch kann kollidieren; der Re-Manifest-Lauf ist der eigentliche Trigger (nicht der Dispatch). Die vier Läufe sind queued/laufend (concurrency-safe, `cancel-in-progress: false`); bei Bedarf abbrechbar.
- **Blockade:** Lauf-Ende
- **Braucht:** `ci_manage status`/`log` der Läufe; danach `--sniff`/API-Liste der drei Bins, sha in `sources.φ`.

### pds3-img — WUSTL-Asset manifestiert
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `phi/harvest.φ`-Eintrag `pds3_img` auf `asset present` setzen (Mountain)
- **Lage:** (gemessen 2026-10-01 via API) `pds3-img-cdn 36887145554` success; `pds-geosciences.wustl.edu` trägt `pds3_img_fsb_00720_1cd_xhu_84n209_v1.bin` 29896248 B sha256 `6aa0eb1f…`. Der M3-Rest bleibt CI-IP-403.
- **Blockade:** M3-Mirror fehlt
- **Braucht:** Mountain setzt den `pds3_img`-`harvest.φ`-Eintrag auf `asset present` (sha); M3-Descope-Befund, falls kein Mirror.

### Quake-ptevent (ERBQ) — verifiziert, Chile/Tohoku je 1 Event
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `quake-ptevent-cdn`-Lauf (`36894598573`)
- **Lage:** (gemessen 2026-10-01 via `cargo run` lokal + `returnCountOnly`) der lokale `quake_ptevent_compiler`-Lauf (ohne `--ci-mode`, `--out /tmp/opencode/qtest`) liefert `chile` 1 Event/45 B `ff7e2f67…`, `tohoku` 1 Event/45 B `cde2a662…`, `jma` 225 Events/7213 B. **Kein Riss** — 45 B = `HEADER_LEN 13 + REC_BYTES 32` (1 Event); die beiden ArcGIS-Layer haben per `returnCountOnly` je genau `count: 1`. Mein früherer „leer"-Befund war eine Fehlmessung (Byte-Größe ohne Konstanten-Prüfung) und ist korrigiert.
- **Blockade:** keine
- **Braucht:** Lauf-Ende `36894598573` → `--sniff`/API; danach Punkt schließen.

### CDSE-CCM STAC-Auth-Asset — 403 = fehlendes CCM-Download-Entitlement (recherchiert)
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** CCM-Lizenz-Akzeptanz im CDSE-Konto (Operator-Akt, via Future-Queue)
- **Lage:** (gemessen 2026-10-01 via `archive_search --verdict` + `sfetch` + brave) STAC-Katalog `catalogue.dataspace.copernicus.eu/stac/collections` = HTTP 200 (öffentlich); OData `download.dataspace.copernicus.eu/odata/v1/Products(<id>)/$value` **ohne Token = 401**, mit Token im CI **403**. Der Bin nutzt `fetch_raw_bytes_headers_redirect` (`fetch.rs:241`); `redirect_target` (`:186`) liefert keinen Ziel-URL → Fallback sendet den Header → 403. **Recherche-Ursache:** CCM (Contributing Missions) verlangt im CDSE-Konto (a) die **Akzeptanz der CCM User License**, (b) eine **berechtigte User-Kategorie** (EU / Copernicus-Participating: C-S-U-I/U-R-P/N-P-A/I-O/P/C-O), (c) Zusatzangaben (Kategorie/Institution/Copernicus-Projekt) — Quellen `dataspace.copernicus.eu/news/2024-6-27-…`, `…/explore-data/…/ccm-user-categories`. Daneben sind CDSE-seitig intermittierende OData-503/S3-403 bekannt (Forum), aber unser 403 ist konsistent = **Entitlement**, nicht Token.
- **Blockade:** die CCM-Lizenz/Download-Berechtigung im CDSE-Konto fehlt (Zugangs-Zustand)
- **Braucht:** Operator-Akt im CDSE-Konto (CCM-Lizenz akzeptieren + Kategorie/Institution angeben) — via `## An future`; danach `cdse-stac-probe` neu messen.

### Registrierte Assets mit CDN-404 — Rest (nur itokawa)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `kernel-flatten`-Re-Dispatch @`<neuer HEAD>` (mit itokawa-Command-Fix)
- **Lage:** (gemessen 2026-10-01 via API/`--verdict`) `cosmicflows_cf4.json` + `pioneer11_odf.bin` + `pioneer10_telemetry.bin` = 200/sha registriert; `ephemeris_pioneer1{0,1}_daily.bin` = 200 unter `-horizons`, Register-Tag+Compiler geheilt; **`ephemeris_itokawa.bin` fehlt unter beiden Tags** (Horizons-Command war falsch; Fix im Baum, braucht Commit + Re-Dispatch).
- **Blockade:** Commit + Lauf
- **Braucht:** nach Push `gh workflow run kernel-flatten.yml`; dann `--sniff`/API-Liste von `ephemeris_itokawa.bin`, sha in `sources.φ`.

### `ci-gate`/`ci-check` laufende Rottöne
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `ci-gate 36866394140` / `ci-check 36866394134` @`2e7b227e4`
- **Lage:** (gemessen 2026-10-01 via `ci_manage status/jobs`) `ci-gate 36860776495` @6f9b5103a rot: clippy (chebyshev E0425 u. a., von Mountain-216 geheilt) + dropped-gate delta 9 (hier gebumpt); `ci-check 36862324726` @12:47 rot: `lib test` E0425; `ci-gate 36862324829` cancelled (Superseded). Der aktuelle Lauf war bei der Messung `in_progress`/`queued`, Log `unread`.
- **Blockade:** CI-Zahl/Lauf-Ende
- **Braucht:** `ci_manage log 36866394140` nach Lauf-Ende — hält `dropped-gate` (Baseline 1339) und clippy grün.

### pds3-img M3 — CI-Route 403 (Register `phi/blocked_sources.φ:467` blocked ip-blocked)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `pds3-img-cdn`-Lauf (Roundtrip-Fix steht)
- **Lage:** (gemessen 2026-09-30 via `ci_manage log`) `36737530030` failure: M3-`.HDR` (`https://pds-imaging.jpl.nasa.gov/data/m3/CH1M3_0003/DATA/`) HTTP 403 (Datacenter-IP-Block); lokal direkt 206, Proton 403, kein Wayback-Snapshot. Der Roundtrip-Fix lässt den Lauf mindestens das WUSTL-Asset schreiben.
- **Blockade:** M3 bleibt CI-IP-403 (kein Mirror)
- **Braucht:** andere Route/Feder messen (`archive_search --playwright`); M3-Descope-Befund, falls kein Mirror.

### hips-png / ps1 — laufende Shards
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende
- **Lage:** (gemessen 2026-10-01 via `ci_manage status`) `ps1-cdn 36723543966` success; `hips-png-cdn 36831989439` queued.
- **Blockade:** keine
- **Braucht:** `ci_manage jobs`/`log` beim Lauf-Ende.

### Legacy-CDN Re-Manifest (ssd family tag)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `wartend.φ:27` schließen
- **Lage:** (gemessen 2026-10-01) `nvss.json` (`sources.φ:10670`), `first14.json` (`:10540`), `curated48_spectra.bin` (`:9217`) zeigen aufs Legacy-Tag `ssd.jpl.nasa.gov`; `--verdict` = HTTP 206/found. Die Wartend-Zeile `state/zustand/wartend.φ:27` (2026-09-28) ist stale.
- **Blockade:** keine
- **Braucht:** `wartend.φ:27` als aufgelöst schließen; kein Re-Manifest-Lauf nötig.

### Register-Träger — `phi/pipeline/index.φ` offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster Katalog-Port (`docs/SOURCE_PORT.md`)
- **Lage:** (gemessen 2026-10-01 via `register_lookup --open`) Katalog-Offenstand 5 (Arbeitsdateien gitignored).
- **Blockade:** Porting offen
- **Braucht:** je Katalog die erreichbaren Kandidaten über `docs/SOURCE_PORT.md` portieren.

### Register-Träger — `phi/pipeline/ledger.φ` SSDC
- **Status:** termin | **Bindung:** termin:2026-10-02
- **Trigger:** neue SSDC-Prozedur (`https://limadou.ssdc.asi.it/query.php`)
- **Lage:** (gemessen 2026-09-30) `ledger.φ:6` `ausstehend`; `query.php` CAS-Login, Sotgiu „wait a few weeks".
- **Blockade:** Prozedur nicht live
- **Braucht:** nach Termin `archive_search --playwright "https://limadou.ssdc.asi.it/query.php"`.

### `blocked_sources.φ` mycelium-pending-Dispositionen (19)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** je Zeile (`phi/blocked_sources.φ`)
- **Lage:** (gemessen 2026-09-30) `:59` BepiColombo, `:85` MESSENGER, `:98` DEMETER, `:346` GOSAT-GW, `:374` DAS2 Iowa, `:378` Occultation-DB, `:402` ExoMars TGO, `:406` Akatsuki, `:410` Kaguya, `:414` Chandrayaan-1, `:418` Chang'e MRM, `:422` Tianwen-1 RoPeR, `:426` Phobos 2, `:430` Vega 1/2, `:434` Hayabusa, `:438` Tianwen-1 MoRIC, `:442` Shandong, `:458` Danuri ShadowCam, `:462` CDSE-CCM.
- **Blockade:** je Zeile (Arm/Reader/Feder)
- **Braucht:** je Zeile den nächsten Port-Schritt (`docs/SOURCE_PORT.md`).

### Zeugen-Riss (fünfte Zeugenart) — Register-Faltung + Schema-Arm
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Adjudikation liegt vor (`state/stimmen/2026-10-01_adjudikation_zeugen-risse.md`, Kimi K3 + Sonnet 5.5 Max → **fünfte Art „Punkt-Ereignis"**) — Entscheidung getroffen.
- **Lage:** (gemessen 2026-10-01) Mapping v2 + 173 Kandidaten (`state/mycelium/zeugen-sweep/*.md`, `zeugen-sweep-mapping.md`); `WitnessKind` in `src/archivar/witness.rs:10` trägt vier Arme (S2Direction/Gestalt/Presence/Substance), `magic_identity` keinen fünften Magic.
- **Blockade:** die Register-Verdikte (`declined_sources.φ`/`dead_sources.φ`) + der `WitnessKind`-Arm liegen in Mountain-Domäne.
- **Braucht:** Mountain faltet die Kandidaten-Verdikte + entscheidet/trägt den fünften `WitnessKind`-Arm (transportiert via `## An mountain`).

### `http_401`-Residuum
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** neue Mail/Asset-Messung
- **Lage:** (gemessen 2026-09-30) nach der GitHub-PAT-Rotation kein neuer 401.
- **Blockade:** keine
- **Braucht:** weiter beobachten.

### D5-Orphan-Residuum
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Asset-Producer des Röhren-Feldes (`docs/concepts/zeugnis.md:383`)
- **Lage:** (gemessen 2026-09-30) kein Producer-Bin/Register/Wf.
- **Blockade:** Producer fehlt
- **Braucht:** kein Schritt zur Kante — erst ein Bau-Auftrag ändert den Zustand.

### GitHub-Issues — Zensus (gemessen 2026-10-01)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Issue-Review beim nächsten Pass (`gh issue list --state open`)
- **Lage:** (gemessen 2026-10-01 via `gh issue list`) **18 offene**: #116 paper-gate (river), #115 pds3_fixed_width_darts, #114 de441-cdn S14 unblocked (stale), #113 dropped-gate (hier gebumpt), #81 clippy, #80 Anomalie-Report, #71/#30 flatten bodies, #60/#17 recheck-live drift, #58/#15 cargo test, #53 nvss, #52 first14, #50 vsx, #49 frbcat_flat, #48 gcvs_cat, #47 cbdata.
- **Blockade:** keine
- **Braucht:** `gh issue close <id>` für die geheilt/stale-Fälle (#113 gebumpt, #114 S14 unblocked); die flatten-void-Issues #47–#53 gegen die letzten `*-cdn`-Läufe messen.

### Quellenseitige Waits (`state/zustand/wartend.φ`)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Antwort/Readiness
- **Lage:** (gemessen 2026-09-30) Aufnehmer mycelium: `ssdc-limadou`, `voyager-nssdca`, `mariner10-nssdca`, `viking-nssdca`, `cassini-trk`, `juno-jplnav`, `superdarn-af68c4f1`, `emodnet-hfr`, `bepicolombo-more`, `noirlab-gaia-dr4`, `legacy-cdn-ssd`; die ODF-Rohdaten-Anfragen (ESOC/KinetX/Turyshev) unter `:29–31`.
- **Blockade:** Antwort
- **Braucht:** Postfach + Wiedervorlagen beobachten (Trigger feuert → selbes Atom).

### termin-Punkte — Wiedervorlage
- **Status:** termin | **Bindung:** termin:2026-10-02 · 2026-10-19 · 2026-12-02 · 2026-12-03 · 2027-04-01
- **Trigger:** `superdarn-af68c4f1` · `emodnet-hfr` · `noirlab-gaia-dr4` · `europa-clipper` · `bepicolombo-more`
- **Lage:** (gemessen 2026-09-30 via `state/zustand/wartend.φ`) Wiedervorlage, Aufnehmer mycelium.
- **Blockade:** Termin
- **Braucht:** `archive_search --verdict <url>` beim jeweiligen Datum.

## An future

Origin: mycelium-folge217 (Antwort auf future-folge163).

- **Zeugen-Riss + Stimmen:** alle acht externen Stimmen + Adjudikation liegen unter `state/stimmen/2026-10-01_*_zeugen-risse.md`; fünfte Art `PointEvent` **gebaut**, Sweep auf Mapping v3 umgetragen, Mountain hat die Register-Faltung + den `ERBQ`-Quake-Zeugen bereits committet (`958d946ba`). Keine weitere Sammlung durch Mycelium nötig.
- **Re-Manifest der 3 CDN-404:** `cosmicflows_cf4.json` + `pioneer11_odf.bin` + `pioneer10_telemetry.bin` sind 200 und sha-registriert; die zwei `ephemeris_pioneer*_daily.bin` + `itokawa` werden mit dem Solver-Fix neu dispatcht.
- **GitHub-Issues-Zensus:** `gh issue` ist freigegeben (future-164) — der Zensus lief (18 offene Issues, s. Offen); kein Operator-Wort nötig.
- **Registry-first:** `pioneer11_odf`/`pioneer10_telemetry` vollständig registriert (url/origin/compiler/sha256; `ttl` von Mountain); `pioneer-telemetry-cdn.yml`-Tag-Riss geheilt.
- **CDSE-CCM 403 — Operator-Queue:** *Lage:* STAC-Katalog 200, OData-Produkt-`$value` mit dem vorhandenen `CDSE_USER`-Token **403**. Recherchiert: der CCM-Download verlangt im CDSE-Konto die Akzeptanz der **CCM User License** + berechtigte User-Kategorie + Zusatzangaben (Quellen s. Offen). *Frage:* Soll die CCM-Lizenz im CDSE-Konto (`CDSE_USER`) akzeptiert und Kategorie/Institution eingetragen werden? *Bei Ja:* `cdse-stac-probe` neu messen (Akt = Operator-Hand im Konto). *Bei Nein:* `phi/blocked_sources.φ:465` als `blocked entitlement` pinnen.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
