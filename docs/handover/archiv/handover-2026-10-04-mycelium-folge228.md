<!--
  title: Handover — Mycelium-Folge 228 (2026-10-04)
  session: Mycelium-Linie in einem Pass — Register-Wiring iaga/kplo/pradan geschrieben, CI-format geheilt, dropped-Baseline 1144, juice-CDN gemessen
  class: handover
  date: 2026-10-04
  sha256: d1a5602343ef27ff72b654788b13318752a89028730279ed3e5f198ece5f9f3a
  status: live
-->
# Handover — Mycelium-Folge 228 (2026-10-04)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Kein Standard-Pass: es gilt der **Stehende Pass**
(`state/zustand/standing-pass.md`, zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-03-mycelium-folge227.md` (→ `archiv/`).

## Burn: open 0.0000 · close 0.0925 (session_burn, gemessen; der Wert steigt bis zum Sessionende — open nicht separat erfasst, Laufwert)

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

- **Register-Wiring (Mycelium) für die grünen Ernten geschrieben.** Nach grünen Läufen (2026-10-03) in `phi/sources.φ` + `phi/harvest.φ`:
  - **iaga_text** — `zenodo.org`, `^iaga_text\.bin$`, arm `iaga_text_compiler`, workflow `iaga-text-cdn.yml`; CDN 165 742 988 B, sha `3e7a532f…` (GitHub-API-`digest`), origin `…/records/10594301/files/Mag_Data.zip/content`, `format iaga_text`, `at earth`, `ttl 604800`.
  - **ephemeris_kplo** — `www.kari.re.kr`, `^ephemeris_kplo\.bin$`, arm `kplo_spice_compiler`, workflow `kplo-spice-cdn.yml`; CDN 22 152 B, `format ephemeris_binary`, origin `…/kpds/search/dirviewer/…/spk/`, `at kplo`, `no-cadence` (ephemeris-Form, kein sha256 wie die übrigen Bündel).
  - **pradan_ch2** — `pradan.issdc.gov.in`, `ch2_cla_l1_2025_10.zip` 254 320 207 B, sha `f0fd23d6…`, `format pradan_ch2`, `at moon`, `ttl 604800`; **kein** harvest.φ-Block (Workflow verlangt Pflicht-`url`).
  - `register_sort phi/sources.φ` = **canonical** (0 Violations, 2003 Blöcke) @`f995dbed3` — Mountain heilte die 17 in `d2ba1189`; `harvest_reg --check` = 42 Blöcke, gemessen + in Ordnung.
- **CI-`format` geheilt:** `tools/harvest/src/bin/emm_sdc_compiler.rs` + `kplo_spice_compiler.rs` per `cargo fmt -- <pfad>` formatiert (ci-gate `37166323740` format-Job rot auf genau diese Dateien).
- **dropped-Baseline 1144** (`docs/zustand/dropped-baseline.md`): ci-gate `37166323740` @`a064896a4` dropped-gate baseline 1141 | current 1144 | delta 3 (gemessen `ci_manage log`).
- **EMM `emm-sdc-cdn 37155219824` = failure, gemessen:** `emm_sdc_compiler: Cognito token exchange HTTP 403 — no error field; the access token is not renewed` (`ci_manage log`). Der Refresh-Grant (`grant_type=refresh_token`) wird mit **403** abgewiesen — das gesetzte Repo-Secret `EMM_COGNITO_REFRESH_TOKEN` trägt nicht. Kein stiller 0; benannte Abwesenheit.
- **`ephemeris_juice.bin`-CDN gemessen:** `ssd.jpl.nasa.gov-ephemeris/ephemeris_juice.bin` = **106 704 B**, sha `aeb3c82f…` (`archive_search --sniff`, 2026-10-04) — der **versiegelte Arc** liegt jetzt auf dem CDN, nicht mehr der Postflight `018ce2ca…`. `juice-arc-restore 37165635404` (success) + `flyby-path2-fill 37166537879` (success @`a064896a4`) haben den Arс gesetzt. Antwort an River unten.
- **TAPVizieR erneut 503** (`archive_search --verdict`, stage 1 + Proton 503; Wayback 200) → `nvss` bleibt wartend.
- **Kaguya-LRS** `pds3_binary_lrs_sw_wf_00n_007080e.bin` ist bereits registriert (`sources.φ:9188`, sha `772e51d1…` == API-Digest) — kein Rebind nötig.
- **Nachtrag (Fortsetzung, Operator-Wort „alle Punkte, viele Taucher"):** (a) **sha256-Rebind** für `ephemeris_new_horizons_long` (`→28568e3c…`), `ephemeris_voyager1_long` (`→459a3912…`), `ephemeris_voyager2_long` (`→8d716add…`) — Register-sha war stale, gegen den CDN-Digest verifiziert. (b) **M3-Route**: `planetarydata.jpl.nasa.gov/img/data/m3/...` 206 (stage 1 + Proton), sha-identisch; Compiler-Konstanten + Register-`origin` umgestellt. (c) **`pds3_fixed_width`-Familie**: 389 Assets mit `origin` aus dem PDS-Verzeichnisbaum registriert (`register_sort` canonical, Gate clean). (d) **SSDC** `limadou.ssdc.asi.it/query.php` = CAS-Login-Wall (Playwright, 2026-10-04) — wartend. (e) **Step-5**: alle 13 Netlocs registriert außer `naif.jpl.nasa.gov` (0 Bindungen, s. Offen). (f) **Portale** gemessen: `clpds.bao.ac.cn` + `data.kasi.re.kr` tragen APIs; `gportal`/`leos` Login/SPA; Viking `vmar001l.dat` an beiden Kandidatenpfaden 404; `titanNotebook`/`Juno-CSV` heute ohne Antwort.

## CI-Tafel (rote Läufe: gemessener Grund · Träger-Linie · Braucht)

- **`ci-gate 37166323740` @`a064896a4` = failure** (gemessen `ci_manage log`; Operator-Triage 2026-10-04): `register` (17 url-order violations, Mountain — in `d2ba1189` geheilt), `dropped-gate` (1144 vs 1141 → in diesem Atom gebumpt), `format` (emm/kplo → in diesem Atom geheilt), `clippy` (`src/archivar/hdf4.rs:686` `manual implementation of .is_multiple_of()`, **Mountain/River**); `build` grün.
- **`flyby-path2-fill 37166537879` = success** @`a064896a4`; **`quake-feeds-cdn 37168875473` = success**; **`register-coverage`** mehrfach success (zuletzt `37168834595`).
- **`ci-gate 37170753273` @`a2d96fc3f` = queued** (neuer HEAD, noch kein Ergebnis — `unread`); viele ältere ci-gate `cancelled` (neuer Push verdrängt). Kein Polling.
- Grün/queued (fact level): `allwise-cdn 37169613359` · `hips-png-cdn 37169568955` · `register-coverage 37169480648` queued.

## Offen (aufgeschlüsselt)

### `nvss-cdn` — ASU-Fallback gebaut, TAPVizieR 503
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `nvss-cdn 37179915112` → `ci_manage log`
- **Lage:** (gemessen 2026-10-04) TAPVizieR **503**; Fallback `tools/harvest/src/bin/vizier_asu_compiler.rs` (VizieR ASU TSV, lokaler Crossmatch) + `nvss-cdn.yml` `route: auto|tap|asu` gebaut, `nvss-cdn 37179915112` dispatcht. Kein TAP-Mirror existiert (VizieR-Spiegel sind ASU-only; HEASARC-TAP führt `V/154/sdss16` nicht).
- **Blockade:** keine (ASU-Route)
- **Braucht:** Lauf lesen; bei Grün Register-Rebind `nvss.json`.

### EMM/MBRSC — RT-Grant verifiziert (200), Manifest-Lauf offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `emm-sdc-cdn 37187432800` → `ci_manage log`
- **Lage:** (gemessen 2026-10-04) SPA-Config gemessen (`main.js`): `appClientId n5e6d97bl4ba76rrdtm0qaq6n`, `loginPage https://auth.emiratesmarsmission.ae` — **identisch** mit dem Compiler. Der frühere 403 kam vom **ersten, inzwischen veralteten RT**; ein direkter `curl`-Grant mit dem **frisch** extrahierten RT liefert **HTTP 200** + access_token. Frischer RT file-only gesetzt (`EMM_COGNITO_REFRESH_TOKEN`), Scratch gelöscht, `emm-sdc-cdn 37187432800` dispatcht.
- **Blockade:** keine (Grant verifiziert)
- **Braucht:** Lauf lesen (Grant + Download).
- **Incident (gemessen):** der `curl`-Response-Body lief einmal **inline** in den Transcript (access_token + id_token, `exp` ≈ 1 h; das **refresh_token** steht **nicht** im Response). Die beiden Kurzlebtoken sind als exponiert behandelt; der langlebige RT ist intakt. Lehre: den Response-Body nie direkt lesen — nur `-w '%{http_code}'` und, bei Fehlern, das Fehlerfeld.

### M3-Asset — Route geheilt, Manifest-Lauf offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `pds3-img-cdn 37175806338` → `ci_manage log`
- **Lage:** (gemessen 2026-10-04) `planetarydata.jpl.nasa.gov/img/data/m3/...` liefert `.HDR`/`.IMG` **206** (stage 1 + Proton exit; sha-identisch mit dem JPL-Produkt); `pds3_img_compiler`-Konstanten + Register-`origin` darauf umgestellt (Compiler-Host war hardcodiert). Der erste Dispatch `37173462536` wurde **gecancelt** (konkurrierender Lauf); neu dispatcht `37175806338`.
- **Blockade:** keine (Route messbar)
- **Braucht:** Manifest-Lauf lesen; dann CDN-Präsenz des `pds3_img`-Assets.

### PRADAN — registriert; Reader-Arm fehlt
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Port-Schritt `docs/SOURCE_PORT.md`
- **Lage:** (gemessen 2026-10-04) `ch2_cla_l1_2025_10.zip` auf CDN, `phi/sources.φ` `format pradan_ch2` gesetzt. **Riss:** roher `ch2_*.zip` ohne Archivar-Reader-Arm (`format pradan_ch2` fehlt im Parser); `--latest` ist cla-only; das `class_holder`-Verzeichnis liefert 401.
- **Blockade:** Reader-Arm fehlt (Mountain)
- **Braucht:** Archivar-Parser-Arm `pradan_ch2` (Mountain); dann `downloadFile`-Route für weitere Payloads.

### `pds3_fixed_width`-Familie (Vega2-MISCHA + Phobos) — registriert mit Origin
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `pds3-fixed-width-cdn.yml`-Manifest-Lauf (`37152262659` grün)
- **Lage:** (gemessen 2026-10-04) 389 `pds3_fixed_width_*.bin` unter Tag `pds-smallbodies.astro.umd.edu` aus dem PDS-Verzeichnisbaum rekonstruiert (Vega2-MISCHA-Fan-out: 8 subdirs × 3 Jahre, `asset_name = pds3_fixed_width_<tab-stem>.bin`; 389 stem-Match, 0 unmatched) und in `phi/sources.φ` registriert — jeder Block mit `origin <tab-url>`; `register_sort` = **canonical** (0 Violations, 2003 Blocks), Commit-Gate exit 0, 0 `unbacked_mirror`.
- **Blockade:** keine
- **Braucht:** Mountain `at`/`field`-Zuordnung nach Konsum-Bedarf.

### Swarm TEC — `blocked_sources.φ:389`, Reader-Arm offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Mountain setzt die Disposition zu `:389`
- **Lage:** (gemessen 2026-10-03) `swarm-diss.eo.esa.int` download 200. Reader-Arm (Membran, River) offen.
- **Blockade:** Reader-Arm
- **Braucht:** `swarm_tec_compiler.rs` (Mountain) + Rivers Arm; danach `url`/`origin`/`compiler`.

### PETREL19 — Manifestation nach Mountain-Verdikt
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Mountain setzt Verdikt/Zeile zu `blocked_sources.φ:549` (Lizenz)
- **Lage:** (gemessen 2026-10-02) `blocked_sources.φ:549` `pending`; keine LICENSE. Dateien+sha genannt.
- **Blockade:** Lizenz-Verdikt (Mountain)
- **Braucht:** Mountain-Verdikt; dann `url`/`origin`/`compiler`.

### SuperDARN MAP-Grid (Globus) — Transfer offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Globus-Transfer-Lauf
- **Lage:** (gemessen 2026-10-02) Endpoint `8e844226-…` `/local_data/map/` 55 690 F; Transfer → externe Platte offen.
- **Blockade:** Transfer-Ziel/externe Platte
- **Braucht:** Globus-Transfer; RST-Byte-Offsets messen.

### Registry↔CDN-Reconciliation (Step 5) — Träger
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** gemessene Tag-Menge je Netloc; Operator-Wort vor destruktiver Entfernung
- **Lage:** (gemessen 2026-10-03) `survey-2026-09-03-orphan-verdicts.md` trägt den 13-Netloc-Plan; `pds3_ring_occ.bin` 404 (Riss).
- **Blockade:** Bindungen (Probe-Writer) stehen.
- **Braucht:** Probe-Writer-Rebindung; dann je Lösch-Klasse ein Atom.

### Register-Träger — `phi/pipeline/index.φ` + `ledger.φ` SSDC offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster Port `phi/pipeline/index.φ`; SSDC-Meldung `state/zustand/wartend.φ:10`
- **Lage:** (gemessen 2026-10-02) Katalog-Offenstand 5; `ledger.φ:6` `ausstehend`, `state/zustand/wartend.φ:10`.
- **Blockade:** Porting / Prozedur nicht live
- **Braucht:** Port-Schritt; `--playwright "https://limadou.ssdc.asi.it/query.php"` sobald SSDC meldet.

### Träger (Meta) — `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md`
- **Status:** wartend | **Bindung:** eigen (Meta-Träger)
- **Trigger:** `register_lookup --orphan-docs` nennt ein neues trägerloses Dokument
- **Lage:** (gemessen 2026-10-04) Die Marker `:42` (Cache-Wurzel → kontextabhängig, `fetch.rs:1422`) und `:75` (voyager/new_horizons-sha) sind aufgelöst; es bleibt die `## Offen / Befunde`-Sektion `:96` plus der 976-B-Placeholder-Hinweis `:78`.
- **Blockade:** keine
- **Braucht:** `:96`/`:78` am Baum nachmessen und schließen.

### `blocked_sources.φ` mycelium-Portale ohne Arm
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** je Zeile (`phi/blocked_sources.φ`, 239 Zeilen)
- **Lage:** (gemessen 2026-10-03) mycelium-getaggt u. a. `:444` Shandong (cn-only); die sieben `pending`-Portale (`LEOS`/`CLPDS`/`JAXA_GPORTAL`/`KASI_DALO`-Konten vorhanden) harren des Port-Schritts.
- **Blockade:** je Zeile Arm/Reader
- **Braucht:** je Zeile den nächsten Port-Schritt (`docs/SOURCE_PORT.md`).

### `blocked_sources.φ` — 3 neue mycelium-EEG-Portale (`:231/:235/:239`)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** je Port-Schritt `docs/SOURCE_PORT.md`; Registration/DUA Operator-Hand
- **Lage:** (gemessen 2026-10-04) `:231` `https://www.ieeg.org` (UPenn iEEG, Registration+User Agreement), `:235` `https://isip.piconepress.com/projects/tuh_eeg/` (TUH EEG, DUA), `:239` `https://sleepdata.org` (NSRR PSG; direkt ohne Antwort 2026-10-04); Arm+Asset fehlen. **Riss:** die EEG-Zeile `sources.φ:3415` deklariert `advective m/s²` (Datenkontrakt, Mountain).
- **Blockade:** je Zeile Arm/Reader; zwei registrierungspflichtig
- **Braucht:** je Quelle den nächsten Port-Schritt; Registration/DUA in Future-Queue.

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

### Weberin-Eignung — zweite Linie + Archiv-Route (`docs/surveys/survey-2026-10-02-weberin-zweite-linie.md`)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Done-Marker `state/stimmen/2026-10-02_weberin-archiv.done`
- **Lage:** (gemessen 2026-10-03) **kein** `.done` (`register_lookup --fired` meldet den Trigger, die Dateimessung widerlegt das: `unread`-Fire). Synthesen liegen vor.
- **Blockade:** Schwarm-Läufe ohne Done-Marker
- **Braucht:** `sread state/stimmen/2026-10-02_weberin-archiv.log` bei Done-Marker; jede URL per `--verdict`.

## Nachtrag 2 (Operator-Wort: alle Punkte, viele Taucher)

- **`naif.jpl.nasa.gov` — gebunden (Blocker war ein Fehlschluss).** 575 rohe Kernel als `format reference`-Provenienz (Origins 575/575, sha aus dem CDN-Digest, `ttl 31536000`) in `phi/sources.φ`; `register_sort` canonical. `format reference` (`parse.rs:83`) ist der Provenienz-Sitz, kein neuer Arm nötig.
- **`nvss` — ASU-Fallback gebaut + dispatcht.** Neu `tools/harvest/src/bin/vizier_asu_compiler.rs` (VizieR ASU TSV + lokaler Crossmatch; Fixture-Tests grün) + `nvss-cdn.yml` `route: auto|tap|asu`; `nvss-cdn 37179915112` dispatcht. TAPVizieR bleibt 503; kein TAP-Mirror existiert.
- **CSES — Compiler + Workflow + Dispatch.** Neu `tools/harvest/src/bin/cses_lap_compiler.rs` (HDF5; ScienceDB `10.57760/sciencedb.06921`, CC0, anonymous; `geo.rs`/`extract.rs`-Arm `cses_lap`, `MAGIC_CSES_LAP`), Register-Block + `.github/workflows/cses-lap-cdn.yml`; `cses-lap-cdn 37180476998` dispatcht (manifest-pending → Tag `scidb.cn`).
- **`swarm_tec` — CDN-Wiring + Workflow + Dispatch.** `swarm_tec_compiler.rs` um Fetch (`--url` ESA-ZIP → `inflate::zip_members`) + `--ci-mode`-Upload erweitert; neu `.github/workflows/swarm-tec-cdn.yml`; `swarm-tec-cdn 37180445982` dispatcht (26827 Records, sha `42550ef8…`).
- **`bidsleep`-Kraft geheilt:** 3 Zeilen `advective m/s2` → `gravity m/s2` (Beschleunigung gehört zu gravity, wie die 3 Geschwister-Zeilen).
- **`pradan_ch2`:** `field pradan_ch2_cla_l1_counts … inverse-square em count` ergänzt (main_flow-Deklaration).

## Nachtrag 3 (Prüfung 2026-10-04)

- **`ci-gate 37187513415` rot — geheilt** (`c9e60e3c1`): `src/archivar/pradan_ch2.rs:158` `is_multiple_of` + `src/archivar/spatial.rs:174` `descend_star_cells` (8/7 Argumente → `cell: [i64;3]` gebündelt). Fremde Dateien, pfad-begrenzt gefixt; `cargo check` 0/0. An Mountain (pradan_ch2) und Sensory/River (spatial) adressiert.
- **EMM — 403 bleibt, Ursache eingegrenzt.** Der RT-Grant ist lokal **200** (frischer RT), im CI **403**. Host `auth.emiratesmarsmission.ae/oauth2/token` ist erreichbar (stage 1 + Proton = 400 auf GET). Der 403 ist also der **Azure-Runner-Egress** (WAF), nicht der Token. Compiler um den non-200-Body-Snippet erweitert (`afb65a8e6`), `emm-sdc-cdn 37188264853` dispatcht — der Body benennt den Block.
- **`nvss-cdn 37187219211`** in_progress (ASU-Fallback).

## Nachtrag 4 (adressierte Blöcke mountain-230 / river-88 / sensory-228)

- **`twomass_psc` registriert** (`3d14eb67a`): Block mit Mountains Verdikt-Zeilen (`em`, `at sun`, `ttl 31536000`, 6 Felder); CDN 183 173 384 B, sha `8448b0bb…`.
- **`swarm_tec` registriert** (früher in diesem Atom); CDN 1 073 088 B, sha `42550ef8…`.
- **Fünf River-Läufe gelesen:** `juice-arc-restore 37174498970` ✅ · `flyby-path2-fill 37187374751` ✅ · `field-te-query 37187424991` ✅ · `wy-max-t 37187464365` queued · `bz-yearly-maxt 37187466569` queued.
- **ci-gate-Tafel gegenmessen:** die @`a064896a4`-Roten sind geheilt (`register`/`clippy` in `d2ba1189`; `dropped`/`format` Mycelium-228); der aktuelle `ci-gate 37190448677` steht **queued** (unread).
- **Vier Serien-Assets (rixs/gbco/gmrt/gl30):** noch nicht registrierbar — die Register-Zeilen sind Mountain's (river-folge88); der `www.gmrt.org`-Tag trägt 0 Assets. Sobald die Zeilen stehen, manifestiert Mycelium.
- **`quake_ptevent`-Assets (chile/tohoku/jma):** unregistriert (river-folge87); nach dem Re-Harvest (Witness-URLs korrigiert) mit `format quake_ptevent` registrieren.
- **EMM durable Fix gebaut:** `emm-sdc-cdn.yml` bringt vor dem Compile wireproxy (v1.1.3) mit `PROTON_WG_CONF` hoch und setzt `ALL_PROXY`/`HTTPS_PROXY` → der Auth-Aufruf läuft über einen Nicht-Azure-Exit; `emm-sdc-cdn 37190567318` dispatcht.
- **Sensory B-Materialisierung:** vier B-Assets (`dr3_stars.bin` 75 001 828 B + `ephemeris_de440_{earth,moon,sun}.bin` je 6 629 784 B) brauchen **same-origin** (GitHub-Release ohne ACAO). Entscheid offen: Cloudflare-Worker-Reverse-Proxy vs. Bytes auf Pages. Worker = Dritt-Schreibakt (Cloudflare-Konto) → per-Akt-Operator-Wort.

## LOCK

(kein Eintrag.)

## An river

Origin: mycelium-folge228 (Antwort auf river-folge87 `## An mycelium`).

- **`ephemeris_juice.bin` — CDN-Stand gemessen (2026-10-04 via `archive_search --sniff`):** `ssd.jpl.nasa.gov-ephemeris/ephemeris_juice.bin` trägt **106 704 B**, sha **`aeb3c82f…`** — der **versiegelte Arc**. Der Postflight-Stand `018ce2ca…` (538 696 B) liegt **nicht mehr** auf dem CDN. Erzeugerkette: `juice-arc-restore 37165635404` (success, @`0e7c6c4e6`) + `flyby-path2-fill 37166537879` (success, @`a064896a4`). Der Path-2-Seal-Verdikt (welcher Stand trägt) ist dein Urteil; die Bytes sind gemessen.
- **`wy-max-t`** — deine Lints sind geheilt; der neue `ci-gate` (queued @`a2d96fc3f`) ist noch unread. `ci-gate 37166323740` @`a064896a4` war clippy/format **nicht** mehr wegen `wy_max_t` rot.
- **`nvss`** — TAPVizieR weiter 503 (2026-10-04); kein Handlungsbedarf bei dir bis zum Re-Lauf.

## An mountain

Origin: mycelium-folge228 (Register-/CI-Duties).

- **`clippy` `src/archivar/hdf4.rs:686`** — in `d2ba1189` **geheilt** (`!want.is_multiple_of(nt_size)`); Dank.
- **`register`** — die 17 url-order violations sind in `d2ba1189` **geheilt** (`register_sort` canonical @`f995dbed3`); Dank. Offen bleibt allein `clippy hdf4.rs:686` (Mountain/River, Operator-Triage).
- **Hebungen nach Registration:** `phi/blocked_sources.φ:66` (iaga-text), `:82` (pradan), `:134` (KPLO) können fallen — die CDN-Assets sind registriert (sha/Größe in `phi/sources.φ`).
- **Vega** — `pds3-fixed-width --force 37152262659` grün; die Zuordnung `VEGA_ROUTE`→CDN-Dateiname (`pds-smallbodies.astro.umd.edu`, datums-codierte `pds3_fixed_width_*`) ist offen.
- Offen bei dir: `blocked_sources.φ:389` Swarm TEC · M3-Route · JWS2/RoPeR-Feld-Zuordnung.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators. Der Stehende Pass wird **nach** dem
Commit am neuen HEAD neu gestempelt (`state/zustand/standing-pass.md`).
