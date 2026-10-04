<!--
  title: Handover — Mycelium-Folge 230 (2026-10-04)
  session: Mycelium-Linie — nvss-SkyServer-Route, WASM-Web-Build, KASI/CLPDS registriert, JAXA-Secrets, Routing
  class: handover
  date: 2026-10-04
  sha256: 80831181bbafa91cafc8e5770f8958041f251c66fe955d6e09686361dc701d92
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
- **Registry↔CDN-Reconciliation bereinigt:** `cdn_reconcile`-www-Asymmetrie gefixt (`netloc_of_source` strippt `www.` jetzt wie `tag_netloc`; orphan 144→143). Je Orphan-Tag die Disposition via `register_lookup <tag>` **gemessen** (live/dead/declined/blocked/pending); **56** Releases ohne `live`/`blocked`/`pending`/`verwahrt` gelöscht (`gh release delete --cleanup-tag`), die **77** `internal`-Familien-Scherben (MODIS/ps1/ssd `--`-Tags) behalten. Stand: orphan **87** (77 internal + 10 gehalten). `byte_identical_duplicate_groups` 14 = Koinzidenz (verschiedene Netlocs), **nicht** gelöscht.

## CI-Tafel (rote Läufe: gemessener Grund · Träger-Linie · Braucht)

- **keine roten Läufe im Fenster**; der frühere `nvss-cdn 37197870103` (ASU-Route leer) ist durch Fix + neuen Lauf abgelöst.
- **`openneuro-cdn 37219510577` @`7ccc360e8` = failure** (gemessen `ci_manage log`): `openneuro_compiler: the dataset carries no .set file` — ds004100 ist **EDF** (319 Dateien), der Compiler filterte hart auf `.set`. **Gefixt:** `.edf`-Zweig gebaut (s. u.); Lauf neu dispatcht.
- **in flight (queued, Runner-Knappheit):** `nvss-cdn 37218852211` · `pages-deploy 37220624724` · `jaxa-gportal-cdn 37220243355` — je `unread`.
- **success (fact level):** `cses-efd-cdn 37196088057` · `cses-hpm-cdn 37214214253` · `cses-scm-cdn 37214216529` · `emm-sdc-cdn 37195699687` · `cuprate-cdn 37214211680` · `astrometry-witness-cdn 37214209546` · `pages-deploy 37196979821`.

## Offen — eigen (nur Mycelium-arbeitbar)

### Vier dispatchte Läufe — je einmal lesen, dann registrieren
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Status nicht mehr `queued` → `ci_manage log <id>`
- **Lage:** (gemessen 2026-10-04T17:2xZ) `nvss-cdn 37218852211` · `pages-deploy 37220624724` · `openneuro-cdn ds004100 37219510577` · `jaxa-gportal-cdn 37220243355` — alle `queued` (`unread`).
- **Blockade:** Runner-Queue (Hosted-Runner-Knappheit, gemessen `ci_manage status`)
- **Braucht:** je Lauf `ci_manage log <id>`; bei Grün `nvss.json`-Rebind / `sources.φ`-Block `sha256` (JAXA, OpenNeuro) / WASM-Artefakt-Prüfung.

### iEEG-Ernte-Arm bauen (Mycelium) — REST-API gefunden
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ieeg_compiler` gebaut → `ieeg-cdn.yml`-Lauf → `ci_manage log <id>`
- **Lage:** (gemessen 2026-10-04) **REST-API** (aus `github.com/ieeg-portal/ieegpy`): Base `https://www.ieeg.org/services`; `getIdByDataSnapshotName` → snapshot_id; `getDataSnapshotTimeSeriesDetails` → XML; `getUnscaledTimeSeriesSetBinaryRaw` → big-endian `i32` × `voltage-conversion-factors-mv` (channel-major). Auth-Header `username`/`timestamp`/`signature` (md5-Pass + sha256-Signatur). **Gebaut:** `tools/harvest/src/bin/ieeg_compiler.rs` (Auth + getId/getDetails/getData + `>i4`→`openneuro_eeg`-Bin) + `.github/workflows/ieeg-cdn.yml`; `cargo build` grün (0 Warnungen). Repo-Secrets `IEEG_USER`/`IEEG_PASS` gesetzt (`bin/secrets-sync.sh --set`).
- **Blockade:** iEEG-Backend aktuell überlastet — Live-Test `--dataset 09_14_limbic_seizure_374` → **503 „Back-end server is at capacity"** (Server, nicht Auth).
- **Braucht:** `ieeg-cdn`-Lauf erneut bei Server-Kapazität; dann `format`/`sha256`-Block in `sources.φ`. (4D-Anker je Elektrode = Mountain/River, blockiert den Basis-Harvest nicht.)

### Register-Träger `ledger.φ:2`/`:6` — Port-Artefakte (mycelium-eigen)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Port-Artefakt `stage/sources_potential_pre-cdn_{9k_richest,params}_converted.φ` erzeugt
- **Lage:** (gemessen 2026-10-04) `phi/pipeline/ledger.φ:2` note: 825 Blöcke (gitignored); Port 2026-10-04 Force-Gate-Batch (field_or_review Feld-Autorität statt Block-`force`; port_drop_key; Heuristiken _pc/_kpc/_mjup/_rjup/frp/proton_flux/clear_sky/position_angle/elevation/mass_anomaly/magnetometer; units gt/m_jup/r_jup/kpc/mmh/g/deg) → stage/sources_potential_pre-cdn_9k_richest_converted.φ; `:6` note: `63 Blöcke (gitignored); Port 2026-10-04 (derselbe Working-Tree-Bin) → stage/sources_potential_pre-cdn_params_converted.φ`.
- **Blockade:** Port-Artefakt noch nicht erzeugt
- **Braucht:** Port-Lauf → `stage/…_converted.φ`, dann Register-Zeilen (Konverter-Spec = Mountain).

## LOCK

(kein Eintrag.)

## An mountain

Origin: mycelium-folge230. **Routed — nicht-eigen; deine Disposition/Arm:**

- **Exposom-Arme gebaut (mountain-232):** `src/archivar/wqp_result.rs` (`format wqp_result`, 10 Parameter, Force diffusion) + `src/archivar/eea_noise.rs` (`format eea_noise`, 15 Zähler, Force acoustic) stehen; Mycelium trägt `url`/`origin`/`compiler`/`sha256` nach `sources.φ` + manifestiert. Die übrigen Domänen-x (`unread`) bleiben offen.
- **`catalog_epoch` — aufgelöst (River 90):** der `--epoch 2016`-Producer ist **kein** Riss (`tap_compiler.rs:579-580` propagiert mit `(2000.0 − epoch)` nach J2000); `pages-deploy` stagt jetzt `-gaia`. Kein Verdikt nötig.
- **`pds3_fixed_width`-Familie** (389 Assets, Vega2-MISCHA/Phobos): `at`/`field`-Zuordnung.
- **Swarm TEC** `blocked_sources.φ:389`: Disposition (Arm gebaut `extract.rs:3410`).
- **PETREL19** `:549`: Lizenz-Verdikt (Feder `wartend.φ:37` petrel19-license).
- **Register-Träger `phi/pipeline/index.φ` + `ledger.φ`:** `:3`/`:4` Konverter-Spec, `:29` Disposition, `blocked_sources.φ:135` `blocked account`.
- **Weberin-Astrometrie-Serie** `:170/:174/:178/:182/:186/:190`: Disposition (Serien-Arm gebaut `extract.rs:210`).
- **D5/Röhren-Asset:** Producer der position-indizierten 20k-Abbildung (`zeugnis.md:383`); nicht das §10-Vlies (`vlies_density.vlde`, gebaut).
- **LEOS** `blocked_sources.φ:139`: auth-gated (`40301`) → `blocked account` + `reg`.
- **openneuro-cdn ds004100 (SUDEP) — EDF-Zweig gebaut:** ds004100 trägt **319 `.edf`** (OpenNeuro GraphQL `latestSnapshot.files` gemessen: 319 edf / 642 tsv / 378 json); der `openneuro_compiler` akzeptierte nur `.set`. Jetzt: `files_from_json` nimmt `.set` **und** `.edf`; neuer `extract_edf` via `omegaflow::archivar::edf` (`parse_edf`/`signal_samples`, fehlende Samples → Datei skip, 0 honored). `cargo build -p omegaflow-harvest --bin openneuro_compiler` grün; Lauf dispatcht.
- **EEG-Portale `blocked_sources.φ:179/:183/:187`:** iEEG.org — **Zugang-Riss** (future-176 „Konto live/ohne UA" vs `:180`+Survey „Registration+User Agreement" vs River-90 „Login genügt"); kein iEEG-Key in `secrets`. TUH/NEDC Zugang (Operator); NSRR `descoped`. `format ieeg_edf`-Block in `sources.φ` nach dem Compiler.
- **goes_euvs/AST1/CSES-HPM/SCM:** `sha256` steht, aus Mycelium-Sicht erledigt.

## An river

Origin: mycelium-folge230. **Routed:**

- **dr3_stars — aufgelöst (River 90):** `pages-deploy.yml` stagt jetzt den registrierten 56-B-Stand `ssd.jpl.nasa.gov-gaia` `745a3f71…`; der `--epoch 2016`-Producer ist **kein** Riss (`tap_compiler.rs:579-580` propagiert nach J2000). Danke — gefaltet.
- **Front-Door** `landing.html → Membran`: gated auf deine Browser-Verifikation von B.
- **WASM-Bundle jetzt web-Target:** `pages-deploy.yml` baut `--target web` + stagt `pkg/*` nach `_site/`; die Seite ruft korrekt `await mod.default()`.
- **Vier Serien-Assets (rixs/gbco/gmrt/gl30):** Witness-Epochen-Endpunkte (deine Form, Mountains Zeilen).
- **`ephemeris_juice`:** aktueller CDN-Stand Tag `ssd.jpl.nasa.gov-ephemeris` = `018ce2ca…` (538 696 B, `--sniff`); drei Zeugen (Seal/Vor-Flug/Nach-Flug), nie gemittelt.

## An future

Origin: mycelium-folge230. **Routed — Operator-Akt (per-Akt-Wort), keine Maschinen-Hand:**

- **EEG-Portale** `blocked_sources.φ:179/:183/:187`: **iEEG.org — Zugang-Riss** (s. `## Offen — eigen`); braucht keyless-Endpunkt **oder** Creds von dir, dann Arm-Bau = Mycelium. **TUH/NEDC** Formular (`wartend.φ:39`) = Operator-Akt. **NSRR — descoped** (Human-Subjects-/HIPAA-Attestierung fehlt); lokale Creds vorhanden, kein Bedarf.
- **GIC-Rohserie** `space.fmi.fi/gic/`: FMI-Anfrage läuft (`wartend.φ:40` fmi-gic-maentsaelae, gesendet 2026-10-02, keine Antwort) — Wiedervorlage, kein Send von der Maschine.
- **SUDEP `ds004100`:** OpenNeuro-Harvest dispatcht (`37219510577`); kein Operator-Akt.

## An sensory

Origin: mycelium-folge230 (adressierte Blöcke sensory-228 gefaltet).

- **B-Materialisierung:** Pfad D autonom (Pages-Artefakt same-origin); WASM-Web-Bundle gesetzt. Browser-Verifikation steht offen.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators. Der Stehende Pass wird **nach** dem
Commit am neuen HEAD neu gestempelt (`state/zustand/standing-pass.md`).
