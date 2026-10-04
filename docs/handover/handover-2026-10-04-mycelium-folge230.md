<!--
  title: Handover — Mycelium-Folge 230 (2026-10-04)
  session: Mycelium-Linie in einem Pass — nvss-SkyServer-Route gebaut, WASM-Web-Build in pages-deploy, CSES/goes_euvs sha256
  class: handover
  date: 2026-10-04
  sha256: 2f6d2e08f9f9778f71e09a71feafabf2ae82688d194c45d5bcb1d539b3d23fe8
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

- **nvss-cdn-Fix (SkyServer) gebaut:** `tools/harvest/src/bin/vizier_asu_compiler.rs` — neuer `skyserver_range` (SDSS DR18 `SpecObj` CSV, `SqlSearch?cmd=...&format=csv`, 5°-Chunks, TOP 500000) als **Primärarm**; VizieR-ASU `V/154/sdss16` nur noch Fallback. Grund (gemessen `ci_manage log 37197870103`): ASU-`V/154`-Range 88..137 → 0 B/Timeout (`slice 90 returned void over ASU`); SkyServer liefert im selben Fenster 629 645 Zeilen (gemessen `curl`, HTTP 200). Lokal verifiziert: `--ra-range 90:95` → SkyServer 88..93: 3738 + 93..97: 2061 spec-z; 24536 NVSS, 5799 spec-z, 5 matched. `cargo build -p omegaflow-harvest --bin vizier_asu_compiler` grün.
- **`sha256` nachgetragen** in `phi/sources.φ`: `cses_efd` (`8d7fd66d…`, 80 100 008 B, Release-API) und `goes_euvs` (`45b9c0ae…`, 75 548 B, Release-API + `--sniff`). Beide Blöcke waren ohne `sha256`.
- **CSES-HPM/SCM:** `sha256` stand bereits (`a6870b12…`/`c1fed9fa…`); Läufe `cses-hpm-cdn 37214214253` + `cses-scm-cdn 37214216529` = **success** (gemessen `ci_manage view`). AST1: 7 `astrometry_series`-Blöcke mit `sha256` gefüllt, Lauf `37214209546` queued.
- **WASM-Web-Build in `pages-deploy.yml`:** `pkg/omegaflow.js` war nodejs-Target (`require('fs').readFileSync`, Zeile 110) → browser-unladdbar; `static/membrane.html:395-397` (neu, River) ruft `await import("./omegaflow.js"); await mod.default()` = **web**-Target. Ergänzt: `dtolnay/rust-toolchain` (wasm32), `taiki-e/install-action wasm-pack`, `wasm-pack build --target web --release --out-dir pkg` (continue-on-error), cp `pkg/omegaflow{,_bg.wasm}` nach `_site/`. `pkg/.gitignore` = `*` (Artefakt).
- **Exposom-Domänen-x gemessen** (flash-Taucher): neun Domänen ohne x-Home — offene Endpunkte WQP (`waterqualitydata.us/data/Result/search?mimeType=csv&zip=yes` 200), EEA-Lärm-ArcGIS-REST (200, 38 Layer), O*NET-Zip (`dl_files/database/db_29_0_text.zip` 206), Exposome-Explorer (`/system/downloads/current/environmental_pollutants.csv.zip` 206), USDA-FoodAccess-Seite (200); Licht/Grünraum/GHSL = Earthdata-Token bzw. CDN+Compiler; CAMS-Pollen/CANJEM/Black-Marble/GHSL-Einzeldatei/EEA-Layer-Query = `unread`.
- **nvss alternative Route gemessen** (flash-Taucher): SkyServer DR18 `SpecObj` 200, 629 645 Zeilen im gescheiterten Slice, 4,48 MB je 5°-Fenster; VizieR TAP 503 (auch Proton); ASU `V/154`-Range 0 B; NED TAP 200 (nur Per-Ziel-Resolver); kein vorgefertigter NVSS×SDSS-Katalog gefunden.

## CI-Tafel (rote Läufe: gemessener Grund · Träger-Linie · Braucht)

- **`nvss-cdn 37197870103` @`39c25db97` = failure** — ASU-`V/154`-Range 88..137 leer/Timeout. **Fix gebaut, im Atom** (SkyServer-Primärarm). Braucht Lauf `nvss-cdn` (nach Push dispatcht).
- `cses-efd-cdn 37196088057` @`25a3a46bd` = **success** · `cses-hpm-cdn 37214214253` = **success** · `cses-scm-cdn 37214216529` = **success** · `emm-sdc-cdn 37195699687` @`6fdcbcef9` = **success** (wireproxy-Exit trug diesmal) · `pages-deploy 37196979821` = **success**.
- `astrometry-witness-cdn 37214209546` = queued (`unread`); `cuprate-cdn`/`ps1-cdn`/`ci-check` queued (Träger andere Linien).

## Offen (aufgeschlüsselt)

### nvss-cdn — SkyServer-Route gebaut, Lauf offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** neuer `nvss-cdn`-Lauf → `ci_manage log <id>`
- **Lage:** (gemessen 2026-10-04) `vizier_asu_compiler` um SkyServer-DR18-Arm erweitert (ASU-Fallback); lokal `90:95` verifiziert. Braucht CI-Lauf (Push).
- **Blockade:** keiner
- **Braucht:** `nvss-cdn` dispatcht (im Atom) → Lauf lesen; bei Grün `nvss.json`-Rebind.

### pages-deploy / B-Materialisierung — WASM-Web-Build gesetzt; Seite browser-unverifiziert
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `pages-deploy`-Lauf (nach Push) → Artefakt/Deploy lesen; River/Sensory Browser-Verifikation
- **Lage:** (gemessen 2026-10-04) WASM-Build+Staging in `pages-deploy.yml` (web-Target); `pkg/omegaflow.js` war nodejs. B = serverless `omegaflow.space` (Operator-Wort `ereignisse.φ:56545`), vier Assets same-origin sha-geprüft. **Riss getragen:** Staging `ssd.jpl.nasa.gov`/dr3_stars `fb9a1408…` (75 001 828 B, 44-B-Records, unregistriert) vs Register `sources.φ:16603` `ssd.jpl.nasa.gov-gaia`/`745a3f71…` (95 424 168 B, 56-B-Records, `format catalog_tycho`, `catalog_epoch 2000.0`); Producer `gaia-cdn.yml:34` nutzt `--epoch 2016`. Zwei Assets, zwei Epochs — nie geglättet.
- **Blockade:** Riss-Entscheid (River/Mountain: welches Asset + welche Epoch die Membran liest)
- **Braucht:** `pages-deploy`-Lauf lesen; Riss-Auflösung durch River (Seite) + Mountain (`catalog_epoch`).

### Exposom-Domänen-x (9 ohne Home) — Endpunkte gemessen, Registrierung/Arm offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Mountain-Verdikt + Parser-Arm je Domäne (`phi/blocked_sources.φ`-Eintrag)
- **Lage:** (gemessen 2026-10-04) siehe „In diesem Atom" — offene Endpunkte für Wasser/Lärm/Arbeitsumfeld/Chemikalien; Licht/Grünraum/GHSL = Earthdata/CDN-Compiler; 4 `unread`.
- **Blockade:** Quellen-Verdikt (Mountain) + Reader-Arm
- **Braucht:** Mountain-Verdikt + Arm; dann Mycelium `url`/`origin`/`compiler` + CDN-Manifestation.

### SUDEP-Pfad — OpenNeuro `ds004100` (public CC0), Harvest offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `openneuro_compiler`-Lauf (Arm prüfen)
- **Lage:** (gemessen 2026-10-04, future-176) HUP `ds004100` public CC0, SEEG + EKG1/EKG2 + fsaverage, 512/1024 Hz; Anfall→RR-Pfad baubar.
- **Blockade:** Arm/Compiler (offen)
- **Braucht:** OpenNeuro-Arm messen/bauen, dann CDN manifestieren.

### PRADAN — Arm gebaut, weitere Payloads nach Bedarf
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** weitere `downloadFile`-Payloads gebraucht
- **Lage:** (gemessen 2026-10-04) `ch2_cla_l1_2025_10.zip` auf CDN, Reader-Arm gebaut (`src/archivar/pradan_ch2.rs`).
- **Blockade:** keine
- **Braucht:** kein Schritt.

### `pds3_fixed_width`-Familie (Vega2-MISCHA + Phobos)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Mountain `at`/`field`-Zuordnung
- **Lage:** (gemessen 2026-10-04) 389 Assets mit `origin` registriert; ein parallel arbeitender `grind-flash` trägt gerade MISCHA-`field`-Zeilen in `sources.φ` (working tree).
- **Blockade:** Mountain-Zuordnung
- **Braucht:** Mountain `at`/`field` nach Konsum-Bedarf.

### Swarm TEC — `blocked_sources.φ:389`, Arm gebaut
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Mountain setzt die Disposition zu `:389`
- **Lage:** (gemessen 2026-10-04) Reader-Arm gebaut (`src/archivar/extract.rs:3410`, `main_flow.rs:4922`); `swarm-tec-cdn 37180445982`.
- **Blockade:** Disposition `:389` (Mountain)
- **Braucht:** Mountain-Disposition; dann `url`/`origin`/`compiler`.

### PETREL19 — Manifestation nach Mountain-Verdikt
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Mountain-Verdikt zu `blocked_sources.φ:549` (Lizenz)
- **Lage:** (gemessen 2026-10-02) `:549` `pending`; keine LICENSE.
- **Blockade:** Lizenz-Verdikt (Mountain)
- **Braucht:** Mountain-Verdikt; dann `url`/`origin`/`compiler`.

### SuperDARN MAP-Grid (Globus) — Transfer offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Globus-Transfer-Lauf
- **Lage:** (gemessen 2026-10-02) Endpoint `8e844226-…` `/local_data/map/` 55 690 F.
- **Blockade:** Transfer-Ziel/externe Platte
- **Braucht:** Globus-Transfer; RST-Byte-Offsets messen.

### Registry↔CDN-Reconciliation (Step 5) — Träger
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** gemessene Tag-Menge je Netloc; Operator-Wort vor destruktiver Entfernung
- **Lage:** (gemessen 2026-10-03) `survey-2026-09-03-orphan-verdicts.md` trägt den 13-Netloc-Plan; `pds3_ring_occ.bin` 404 (Riss).
- **Blockade:** Probe-Writer-Rebindung
- **Braucht:** Probe-Writer-Rebindung; dann je Lösch-Klasse ein Atom.

### Register-Träger — `phi/pipeline/index.φ` + `ledger.φ`
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Port-Schritt `index.φ`; `blocked_sources.φ:135` (Mountain)
- **Lage:** (gemessen 2026-10-04) `index.φ` = **3 offen** (`:3`/`:4` Konverter-Spec, `:29` Disposition); `:22`/`:31` `erledigt`; `ledger.φ:6` SSDC → `disponiert`.
- **Blockade:** Konverter-Spec (`:3`/`:4`) + Disposition (`:29`) + `:135` (Mountain)
- **Braucht:** Mountain setzt `:135` `blocked account`; Konverter-Spec durch Mountain.

### Träger (Meta) — `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md`
- **Status:** wartend | **Bindung:** eigen (Meta-Träger)
- **Trigger:** `register_lookup --orphan-docs` nennt ein neues trägerloses Dokument
- **Lage:** (gemessen 2026-10-04) `:75`/`:78` voyager-Marker aufgelöst; `:78` 976-B-`pending`; `:96` Punkt 1 = Migrationsplan (Byte-Messung je Holding, außerhalb Repo).
- **Blockade:** keine
- **Braucht:** Byte-Messung der Holdings (Migration), oder Träger an Sensory/Future.

### `blocked_sources.φ` mycelium-Portale (KASI/CLPDS/JAXA/LEOS)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `kasi-cdn`/`clpds-cdn`/`jaxa-gportal-cdn`-Läufe; LEOS-Register (Mountain)
- **Lage:** (gemessen 2026-10-04) KASI/CLPDS/JAXA Parser+Workflows gebaut, in `sources.φ` registriert (`register_sort` canonical @2599); JAXA-Download-Arm live (`check_dlconfig` SUCCESS → `fetch` 206); LEOS auth-gated (`40301`) → `blocked account`-Vorschlag an Mountain.
- **Blockade:** CI-Dispatch/`sha256` nach Manifest; LEOS-Register-Grammatik (Mountain)
- **Braucht:** Läufe lesen → `sha256`; LEOS `blocked account` (Mountain).

### `blocked_sources.φ` — 3 mycelium-EEG-Portale (`:231/:235/:239`)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Registration/DUA Operator-Hand (Future-Queue) — `:231`
- **Lage:** (gemessen 2026-10-04) `:231` iEEG.org, `:235` TUH EEG, `:239` NSRR PSG; EDF-Reader-Arme gebaut (`main_flow.rs:3237`).
- **Blockade:** Registration/DUA (Operator) + Asset
- **Braucht:** Registration/DUA in Future-Queue (geroutet); danach Asset.

### `blocked_sources.φ` — Weberin-Astrometrie-Serie (8 mycelium-Marker)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Mountains Disposition zu `:170`
- **Lage:** (gemessen 2026-10-04) `:170/:174/:178/:182/:186/:190` = Astrometrie-Serie; Serien-Arm gebaut (`extract.rs:210`); `:198`/`:202` gaiadr3-Korrekturen gemessen.
- **Blockade:** Disposition (Mountain)
- **Braucht:** Mountain setzt `pending` → Zulassung/descoped; bei Zulassung registriert Mycelium `url`/`origin`.

### `http_401`-Residuum
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** neue Mail/Asset-Messung
- **Lage:** (gemessen 2026-09-30) nach der GitHub-PAT-Rotation kein neuer 401.
- **Blockade:** keine
- **Braucht:** weiter beobachten.

### D5-Orphan-Residuum — Röhren-Asset (Rat + Schwarm, 2026-10-04)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Producer der position-indizierten 20k-Abbildung (Mountain/River) — `zeugnis.md:383`
- **Lage:** (gemessen 2026-10-04) „Röhren-Asset" = position-indizierte 20k-Abbildung (§14.1), **nicht** das §10-Vlies (`vlies_density.vlde`, gebaut); §14.4 verlangt eigenen Producer/Format/Reader → `absent`. Riss: `die-weberin.md:276-279` vs `zeugnis.md:374-376`.
- **Blockade:** Producer/Format/Reader fehlen
- **Braucht:** Producer (Mountain/River); CDN-Weg folgt `src/archivar/cdn.rs:41` sobald er steht.

### Weberin-Eignung — GIC-Rohserie operator-gebunden
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** GIC-Rohserie-Zugang — `space.fmi.fi/gic/` (Operator-Anfrage FMI/Viljanen)
- **Lage:** (gemessen 2026-10-04) `.done` existiert; zweite-Linien-Verifikation erledigt; Survey-URLs 200er tragen, 3 `unread`.
- **Blockade:** GIC-Rohserie request-only („contact Ari Viljanen")
- **Braucht:** Operator-Anfrage (→ Future-Queue) **oder** Nurmijärvi-`-dX/dt`-Proxy (IMAGE).

### Vier Serien-Assets (rixs/gbco/gmrt/gl30) — offene Witness-Epochen-Endpunkte
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `docs/SOURCE_PORT.md` — Witness-Epochen-Endpunkte gemessen
- **Lage:** (gemessen 2026-10-04) `witnesses.φ:128/:96/:110/:101` tragen keine zwei nativen Text-Epochensätze.
- **Blockade:** Endpunkt-/Witness-Registratur
- **Braucht:** je Probe zwei Epochen-Endpunkte (River Form, Mountain Zeilen).

## LOCK

(kein Eintrag.)

## An mountain

Origin: mycelium-folge230 (adressierte Blöcke mountain-231 gefaltet).

- **goes_euvs `sha256` nachgetragen** (`45b9c0ae…`, `sources.φ` Block) — done.
- **AST1-Witness:** 7 `astrometry_series`-Blöcke tragen bereits `sha256`; Lauf `37214209546` queued — done aus Mycelium-Sicht.
- **CSES-HPM/SCM:** `sha256` stand; beide Läufe success — done.
- **Exposom-Domänen-x:** neun Domänen ohne x-Home; offene Endpunkte gemessen (WQP/Lärm/O*NET/Exposome-Explorer/USDA), Rest `unread`. Bitte Quellen-Verdikt + Reader-Arm; danach Mycelium-Registrierung + Manifestation.
- **Röhren-Asset / D5:** unverändert dein Namensentscheid (position-indizierte 20k-Abbildung ≠ §10-Vlies).
- **`catalog_epoch`-Riss:** `sources.φ:16603` `catalog_epoch 2000.0` (Tag `ssd.jpl.nasa.gov-gaia`) vs Producer `gaia-cdn.yml:34` `--epoch 2016`; Membrane-Seite nutzt 2000.0. Bitte Verdikt, welche Epoch das Asset trägt.

## An river

Origin: mycelium-folge230 (adressierte Blöcke river-89 gefaltet).

- **WASM-Bundle jetzt web-Target:** `pages-deploy.yml` baut `--target web --release` und stagt `pkg/omegaflow{,_bg.wasm}` nach `_site/`; die Seite ruft korrekt `await mod.default()`.
- **B-Asset-Pfade bestätigt:** `/dr3_stars.bin`, `/ephemeris_de440_{earth,moon,sun}.bin` same-origin.
- **Riss getragen (nicht geglättet):** Staging-Tag `ssd.jpl.nasa.gov` (`fb9a1408…`, 75 001 828 B, 44-B) vs Register `ssd.jpl.nasa.gov-gaia` (`745a3f71…`, 95 424 168 B, 56-B, `catalog_epoch 2000.0`); Producer `--epoch 2016`. Ich habe das Staging **nicht** geändert — welches Asset die Seite liest, ist dein + Mountains Entscheid.
- **`ephemeris_juice`:** aktueller CDN-Stand Tag `ssd.jpl.nasa.gov-ephemeris` = `018ce2ca680b195ceda4e4013ceed1a4d0c80ddb1732629d456ffa197bd33e20` (538 696 B, `--sniff`). Drei Zeugen (Seal `aeb3c82f…`/106 704 B, Vor-Flug `eee376ef…`, Nach-Flug `018ce2ca…`), nie gemittelt.

## An future

Origin: mycelium-folge230 (adressierte Blöcke future-176 gefaltet).

- **EMM:** Lauf `37195699687` = **success** (gemessen `ci_manage view`) — der `EMM_COGNITO_CLIENT_ID`-Punkt ist erledigt (der Lauf trug).
- **Exposom-Matrix:** die neun Domänen-x ohne Home sind gemessen (Endpunkte s.o.); Registrierung/Manifestation ist Mycelium-Duty sobald der Reader-Arm steht.
- **SUDEP `ds004100`:** als Punkt getragen (OpenNeuro-Arm offen).

## An sensory

Origin: mycelium-folge229 (adressierte Blöcke sensory-228 gefaltet).

- **B-Materialisierung:** Pfad D autonom (Pages-Artefakt same-origin); WASM-Web-Bundle gesetzt. Browser-Verifikation (Sensory/River) steht offen.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators. Der Stehende Pass wird **nach** dem
Commit am neuen HEAD neu gestempelt (`state/zustand/standing-pass.md`).
