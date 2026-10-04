<!--
  title: Handover — Mycelium-Folge 230 (2026-10-04)
  session: Mycelium-Linie — nvss-SkyServer-Route, WASM-Web-Build, KASI/CLPDS registriert, JAXA-Secrets, Routing
  class: handover
  date: 2026-10-04
  sha256: c41a8b649eb9f86bbbf0071d880f1dd8d5b9a67c4c90ff79cf9893fd3bc849af
  status: live
-->
# Handover — Mycelium-Folge 230 (2026-10-04)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Kein Standard-Pass: es gilt der **Stehende Pass**
(`state/zustand/standing-pass.md`, zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-04-mycelium-folge229.md` (→ `archiv/`).

## Burn: open 0.0000 · close 0.1070 (session_burn, gemessen; Mycelium-Linie + 2 flash-Taucher)

## Operator-Wort-Register

- Wort | 2026-10-02 | „ich meine glm 5.3 max mit deep search ist echt gut das sollten wir intensiver nutzen" | Quelle: future-folge169 (`state/operator-gespraeche/2026-10-02-future-folge169.md`) — GLM-5.3 Deep Think Max + Deep Search als **erster** Kanal für Tiefen-Recherche.
- Wort | 2026-10-02 | „glm claude und kimi im chat liefern die besten recherchergebnisse" | Quelle: future-folge169 — **Recherche-Trio** (`chat.z.ai` · `claude.ai` · `kimi.ai`) = erster Kanal.
- Wort | 2026-10-02 | „kimi.ai mit k3 geht nicht es geht nur kimi k3 in tryingopen 4000 zeichen i kimi.ai ist es schnell (schätze 2.6)" | Quelle: future-folge169.
- Wort | 2026-10-02 | „für sonnet 5.5 search geht auch immer arena" | Quelle: future-folge169.
- Wort | 2026-10-02 | „nein genug mit den Sondenanfragen. Die Ernte sollten natürlich eingeholt werden." | Quelle: future-folge169.
- Wort | 2026-10-02 | „… ihr macht umfangreiche läufe und dann kastriert ihr sie … so funktioniert forschung nicht" | Quelle: future-folge169 — **kein Top-N**, vollständige Klassifikation.
- Wort | 2026-10-02 | „ich kann es mir beim besten willen nicht vorstellen, dass wir nicht an die daten kommen — bitte fahre jetzt starke legale geschütze auf" | Quelle: future-folge169.
- Wort | 2026-10-02 | „füll" / „bitte auch nochmal losschicken" (GSICS/KASI) | Quelle: future-folge169.
- Wort | 2026-10-01 | „ich habe dir nicht erlaubt zu committen und zu pushen" | Quelle: Mycelium-Session 216.
- Wort | 2026-10-01 | „stehen lassen aber das wort ist du bist die letzte linie die committed das muss sitzen" | Quelle: Mycelium-Session 216 — Mycelium committet **als letzte** Linie, nur mit dem `/commit`-Wort.
- Wort | 2026-10-01 | „bitte nicht nur messen und verschleppen sondern bearbeiten messen und bearbeiten ist die prämisse mein dauerhaftes wort" | Quelle: Mycelium-Session 216.
- Wort | 2026-09-30 | „bitte wirklich bis zur kante umsetzen nicht nur wieder messen und verschleppen" | Quelle: Mycelium-Session 209.
- Wort | 2026-09-30 | „verschleppen und nicht eigenes ist verboten" | Quelle: Mycelium-Session 213.
- Wort | 2026-09-30 | „du committest immer als letzter also warte" | Quelle: Mycelium-Session 213.
- Wort | 2026-09-30 | „vorbestehend ist verboten mein wort" | Quelle: mountain-209.
- Wort | 2026-10-01 | „ja möchte ich" | Quelle: Mycelium-Session 215 — VCO-rs-Register auf das PDS4-20190704-Asset umstellen.
- Wort | 2026-10-03 | „§1-Compiler-Hosts verdiktet: vizier.cfa keep · noaa-eri-pds declined · dachs.fai.kz declined · gsaweb keep · ws.cadc keep." | Quelle: Operator-Session 2026-10-03 (deckt mountain-folge229:183-196) — ausgeführt in Mycelium-226.

## Haus (die vier Orte) — gemessen 2026-10-04

- `omegaflow` = `$HOME/projects/omegaflow` (+ privates Schwester-Repo `state/`, Remote `omegaflow/personal`).
- `omegaflow-legacy` = `archive-root/omegaflow-legacy` (+ backup-2026-09-02); `temp` = `/tmp/opencode`; `archive` = `archive-root` (+ `~/backup/archive/omegaflow`).
- Linien-Preset: `state/mycelium/archive-search-preset.txt` — `--root .github --root tools --root phi --root state --root docs/handover --root docs/concepts`.
- `state/` mit `archive_search <kw> --root state`, **nie** `sgrep` ohne `--all`; `phi/pipeline/catalog/*` gitignored.
- Manifestations-Direktiven (`url`/`origin`/`compiler`/`sha256`/Tags) schreibt Mycelium; Verdikt-Zeilen (`ttl`/Zulassung/Disposition/`note`) Mountain exklusiv.

## In diesem Atom gearbeitet (gemessen)

- **nvss-cdn-Fix (SkyServer) gebaut:** `tools/harvest/src/bin/vizier_asu_compiler.rs` — neuer `skyserver_range` (SDSS DR18 `SpecObj` CSV, `SqlSearch?cmd=...&format=csv`, 5°-Chunks, TOP 500000) als **Primärarm**; VizieR-ASU `V/154/sdss16` nur noch Fallback. Grund (gemessen `ci_manage log 37197870103`): ASU-`V/154`-Range 88..137 → 0 B/Timeout; SkyServer liefert im selben Fenster 629 645 Zeilen (gemessen `curl`, HTTP 200). Lokal verifiziert: `--ra-range 90:95` → 5799 spec-z, 24 536 NVSS, 5 matched. `cargo build -p omegaflow-harvest --bin vizier_asu_compiler` grün.
- **`sha256` nachgetragen** in `phi/sources.φ`: `cses_efd` (`8d7fd66d…`), `goes_euvs` (`45b9c0ae…`), **CLPDS** (`clpds_catalogue.json` `6b584941…`, `clpds_files.jsonl` `21154461…`).
- **KASI registriert** (Lauf `37199896276` = success, aber **kein** Register-Block vorhanden): `kasi_miris.json` `3ce1dee2…`, `kasi_kmtnet.json` `da1f1654…`, `kasi_kvn.json` `de2540bd…`, `format kasi`, `at sun`, ttl 604800. `register_sort` = canonical (2602 Blöcke).
- **WASM-Web-Build in `pages-deploy.yml`:** `pkg/omegaflow.js` war nodejs-Target (`require('fs')`); `static/membrane.html:395-397` ruft `await import("./omegaflow.js"); await mod.default()` = **web**-Target. Ergänzt: `dtolnay/rust-toolchain` (wasm32), `taiki-e/install-action wasm-pack`, `wasm-pack build --target web --release --out-dir pkg`, cp `pkg/omegaflow{,_bg.wasm}` nach `_site/`.
- **JAXA-`absent`-Ursache gemessen:** nicht die Quelle fehlte — `JAXA_GPORTAL_USER/_PASS` sind lokal in `.secrets.local` (`bin/secrets_keys`), fehlten aber als Repo-Secrets (`gh secret list`; der Workflow liest `secrets.JAXA_GPORTAL_*`). `bin/secrets-sync.sh --set` gesetzt (33 Secrets inkl. JAXA + `OMEGAFLOW_SECRETS_FILE`).
- **SUDEP dispatcht:** `openneuro-cdn ds004100` → `37219510577`.
- **Exposom- + nvss-Recherche** (2 flash-Taucher, ~$0.035): Exposom-Endpunkte gemessen, NVSS-SkyServer-Route belegt.

## CI-Tafel (rote Läufe: gemessener Grund · Träger-Linie · Braucht)

- **keine roten Läufe im Fenster**; der frühere `nvss-cdn 37197870103` (ASU-Route leer) ist durch Fix + neuen Lauf abgelöst.
- **in flight (queued, Runner-Knappheit):** `nvss-cdn 37218852211` · `pages-deploy 37220624724` · `openneuro-cdn 37219510577` · `jaxa-gportal-cdn 37220243355` — je `unread`.
- **success (fact level):** `cses-efd-cdn 37196088057` · `cses-hpm-cdn 37214214253` · `cses-scm-cdn 37214216529` · `emm-sdc-cdn 37195699687` · `cuprate-cdn 37214211680` · `astrometry-witness-cdn 37214209546` · `pages-deploy 37196979821`.

## Offen — eigen (nur Mycelium-arbeitbar)

### Vier dispatchte Läufe — je einmal lesen, dann registrieren
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Status nicht mehr `queued` → `ci_manage log <id>`
- **Lage:** (gemessen 2026-10-04T17:2xZ) `nvss-cdn 37218852211` · `pages-deploy 37220624724` · `openneuro-cdn ds004100 37219510577` · `jaxa-gportal-cdn 37220243355` — alle `queued` (`unread`).
- **Blockade:** Runner-Queue (Hosted-Runner-Knappheit, gemessen `ci_manage status`)
- **Braucht:** je Lauf `ci_manage log <id>`; bei Grün `nvss.json`-Rebind / `sources.φ`-Block `sha256` (JAXA, OpenNeuro) / WASM-Artefakt-Prüfung.

### dr3_stars-Staging — Staging-Tag vs Register (Riss am eigenen `pages-deploy.yml`)
- **Status:** wartend | **Bindung:** eigen (Staging) / Verdikt River+Mountain
- **Trigger:** Asset-/Epoch-Verdikt (`## An mountain` `catalog_epoch`; `## An river` Seite)
- **Lage:** (gemessen 2026-10-04) `pages-deploy.yml:59` stagt `ssd.jpl.nasa.gov`/dr3_stars `fb9a1408…` (75 001 828 B, 44-B) — **unregistriert**; Register `sources.φ:16603` `ssd.jpl.nasa.gov-gaia`/`745a3f71…` (95 424 168 B, 56-B, `format catalog_tycho`, `catalog_epoch 2000.0`); Producer `gaia-cdn.yml:34` `--epoch 2016`. Zwei Assets, zwei Epochs — nie geglättet.
- **Blockade:** Verdikt (River Seite / Mountain `catalog_epoch`)
- **Braucht:** Verdikt → Staging-Tag angleichen **oder** Register-`catalog_epoch` korrigieren; beides dann im eigenen Workflow.

### Registry↔CDN-Reconciliation (Step 5)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `cdn_reconcile` — gemessene Tag-Menge je Netloc
- **Lage:** (gemessen 2026-10-03) `survey-2026-09-03-orphan-verdicts.md` trägt den 13-Netloc-Plan; `pds3_ring_occ.bin` 404 (Riss).
- **Blockade:** Probe-Writer-Rebindung
- **Braucht:** Probe-Writer-Rebindung; dann je Lösch-Klasse ein Atom.

### Träger (Meta) — `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md`
- **Status:** wartend | **Bindung:** eigen (Meta-Träger)
- **Trigger:** `register_lookup --orphan-docs` nennt ein neues trägerloses Dokument
- **Lage:** (gemessen 2026-10-04) `:78` 976-B-`pending`; `:96` Punkt 1 = Migrationsplan (Byte-Messung je Holding, außerhalb Repo).
- **Blockade:** keine
- **Braucht:** Byte-Messung der Holdings (Migration).

### `http_401`-Residuum
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** neue Mail/Asset-Messung
- **Lage:** (gemessen 2026-09-30) nach der GitHub-PAT-Rotation kein neuer 401.
- **Blockade:** keine
- **Braucht:** weiter beobachten.

### PRADAN — Arm gebaut
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** weitere `downloadFile`-Payloads gebraucht
- **Lage:** (gemessen 2026-10-04) `ch2_cla_l1_2025_10.zip` auf CDN, Reader-Arm gebaut (`src/archivar/pradan_ch2.rs`).
- **Blockade:** keine
- **Braucht:** kein Schritt.

## LOCK

(kein Eintrag.)

## An mountain

Origin: mycelium-folge230. **Routed — nicht-eigen; deine Disposition/Arm:**

- **Exposom-Domänen-x (9 ohne Home):** offene Endpunkte gemessen — WQP `waterqualitydata.us/data/Result/search?mimeType=csv&zip=yes` 200, EEA-Lärm-ArcGIS-REST 200, O*NET `dl_files/database/db_29_0_text.zip` 206, Exposome-Explorer `…/current/environmental_pollutants.csv.zip` 206, USDA-FoodAccess 200; Licht/Grünraum/GHSL = Earthdata/CDN+Compiler; CAMS-Pollen/CANJEM/Black-Marble = `unread`. Braucht: Quellen-Verdikt + Reader-Arm; danach Mycelium `url`/`origin`/`compiler` + Manifestation.
- **`catalog_epoch`-Riss:** `sources.φ:16603` `catalog_epoch 2000.0` (Tag `ssd.jpl.nasa.gov-gaia`) vs Producer `gaia-cdn.yml:34` `--epoch 2016`. Braucht: Verdikt, welche Epoch das Asset trägt.
- **`pds3_fixed_width`-Familie** (389 Assets, Vega2-MISCHA/Phobos): `at`/`field`-Zuordnung.
- **Swarm TEC** `blocked_sources.φ:389`: Disposition (Arm gebaut `extract.rs:3410`).
- **PETREL19** `:549`: Lizenz-Verdikt (Feder `wartend.φ:37` petrel19-license).
- **Register-Träger `phi/pipeline/index.φ` + `ledger.φ`:** `:3`/`:4` Konverter-Spec, `:29` Disposition, `blocked_sources.φ:135` `blocked account`.
- **Weberin-Astrometrie-Serie** `:170/:174/:178/:182/:186/:190`: Disposition (Serien-Arm gebaut `extract.rs:210`).
- **D5/Röhren-Asset:** Producer der position-indizierten 20k-Abbildung (`zeugnis.md:383`); nicht das §10-Vlies (`vlies_density.vlde`, gebaut).
- **LEOS** `blocked_sources.φ:139`: auth-gated (`40301`) → `blocked account` + `reg`.
- **goes_euvs/AST1/CSES-HPM/SCM:** `sha256` steht, aus Mycelium-Sicht erledigt.

## An river

Origin: mycelium-folge230. **Routed:**

- **dr3_stars-Riss** — welches Asset die Membran liest: `ssd.jpl.nasa.gov`/`fb9a1408…` (75 MB/44 B) vs `ssd.jpl.nasa.gov-gaia`/`745a3f71…` (95 MB/56 B, `catalog_epoch 2000.0`); Producer `--epoch 2016`. Staging (Mycelium) folgt deinem + Mountains Verdikt.
- **Front-Door** `landing.html → Membran`: gated auf deine Browser-Verifikation von B.
- **WASM-Bundle jetzt web-Target:** `pages-deploy.yml` baut `--target web` + stagt `pkg/*` nach `_site/`; die Seite ruft korrekt `await mod.default()`.
- **Vier Serien-Assets (rixs/gbco/gmrt/gl30):** Witness-Epochen-Endpunkte (deine Form, Mountains Zeilen).
- **`ephemeris_juice`:** aktueller CDN-Stand Tag `ssd.jpl.nasa.gov-ephemeris` = `018ce2ca…` (538 696 B, `--sniff`); drei Zeugen (Seal/Vor-Flug/Nach-Flug), nie gemittelt.

## An future

Origin: mycelium-folge230. **Routed — Operator-Akt (per-Akt-Wort), keine Maschinen-Hand:**

- **EEG-Portale** `blocked_sources.φ:231/:235/:239` (iEEG.org / TUH EEG / NSRR PSG): Registration/DUA. TUH läuft bereits (`wartend.φ:39` tuh-eeg-access; `:38` eligibility resolved). iEEG.org + NSRR Registration offen.
- **GIC-Rohserie** `space.fmi.fi/gic/`: FMI-Anfrage läuft (`wartend.φ:40` fmi-gic-maentsaelae, gesendet 2026-10-02, keine Antwort) — Wiedervorlage, kein Send von der Maschine.
- **SUDEP `ds004100`:** OpenNeuro-Harvest dispatcht (`37219510577`); kein Operator-Akt.

## An sensory

Origin: mycelium-folge230 (adressierte Blöcke sensory-228 gefaltet).

- **B-Materialisierung:** Pfad D autonom (Pages-Artefakt same-origin); WASM-Web-Bundle gesetzt. Browser-Verifikation steht offen.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators. Der Stehende Pass wird **nach** dem
Commit am neuen HEAD neu gestempelt (`state/zustand/standing-pass.md`).
