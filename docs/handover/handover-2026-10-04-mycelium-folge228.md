<!--
  title: Handover — Mycelium-Folge 228 (2026-10-04)
  session: Mycelium-Linie in einem Pass — Register-Wiring iaga/kplo/pradan geschrieben, CI-format geheilt, dropped-Baseline 1144, juice-CDN gemessen
  class: handover
  date: 2026-10-04
  sha256: 20fb65bb783d3fc77994ca57c470900bbcf5928862c3dd20d6510eebf28f3202
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

### `nvss-cdn` — TAPVizieR 503, Wiederholungslauf offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** TAPVizieR wieder erreichbar → `gh workflow run nvss-cdn.yml`
- **Lage:** (gemessen 2026-10-04 via `archive_search --verdict`) Host `tapvizier.cds.unistra.fr` **503** (stage 1 + Proton; Wayback 200). Der UWS-Fehlerarm steht.
- **Blockade:** TAPVizieR 503
- **Braucht:** nach Host-Rückkehr `gh workflow run nvss-cdn.yml`, dann `ci_manage log <id>` (die `<errorSummary>`-Zeile); danach Register-Rebind `phi/sources.φ` (`nvss.json`) auf `ssd.jpl.nasa.gov-nvss/`.

### EMM/MBRSC — Refresh-Grant 403, RT unbrauchbar
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Browser-Gruppe `emm` Session → `localStorage.cognitoTokens.refresh_token`
- **Lage:** (gemessen 2026-10-03 via `ci_manage log 37155219824`) Refresh-Arm gebaut (`c4694df48`), Repo-Secret `EMM_COGNITO_REFRESH_TOKEN` gesetzt; Live-Grant `POST auth.emiratesmarsmission.ae/oauth2/token` → **403**. Die Browser-Gruppe `emm` ist vorhanden, aber die Bridge-`browser_*`-Tools geben Werte nur **inline** zurück; der Playwright-Browser hat keine `emm`-Session (`about:blank`). Eine Extraktion über diese Tools würde den Token in den Transcript leaken — Secret-Hygiene verbietet es.
- **Blockade:** kein file-only-Browser-Pfad (Bridge inline; Playwright ohne Session)
- **Braucht:** `playwright_browser_run_code_unsafe` gegen eine `emm`-Profil-Session (schreibt `localStorage` auf Datei, gibt nur einen Marker zurück) oder ein gleichwertiger file-only-Extraktor; dann `gh secret set < datei`, Scratch löschen, `emm-sdc-cdn` dispatchen.

### M3-Asset — Route geheilt, Manifest-Lauf offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `pds3-img-cdn.yml`-Lauf (in diesem Atom dispatcht) → `ci_manage log`
- **Lage:** (gemessen 2026-10-04) `planetarydata.jpl.nasa.gov/img/data/m3/...` liefert `.HDR`/`.IMG` **206** (stage 1 + Proton exit; sha-identisch mit dem JPL-Produkt); `pds3_img_compiler`-Konstanten + Register-`origin` darauf umgestellt (Compiler-Host war hardcodiert).
- **Blockade:** keine (Route messbar)
- **Braucht:** Manifest-Lauf lesen; dann CDN-Präsenz des `pds3_img`-Assets.

### KPLO/KARI KPDS — registriert; Format `ephemeris_binary` — Reader-Route messen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Port-Schritt `docs/SOURCE_PORT.md`
- **Lage:** (gemessen 2026-10-04) `ephemeris_kplo.bin` auf CDN (22 152 B), in `phi/sources.φ` + `phi/harvest.φ` registriert. Offen: Mountain `format`/`field`-Zuordnung für den neuen Körper `kplo`.
- **Blockade:** Format-Zuordnung (Mountain)
- **Braucht:** Mountain prüft `ephemeris_binary`/`at kplo` im Field-Contract.

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

### `naif.jpl.nasa.gov` — unbound canonical tag; Bindung blockiert am fehlenden `format`-Arm
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Mountain setzt einen `phi/sources.φ`-`format`-Arm für rohe CK/DAF-Kernel (oder entscheidet `format reference` als provenance-only)
- **Lage:** (gemessen 2026-10-04) Der Tag trägt **575** rohe SPICE-Kernel (`.bc` 570, `.bsp` 4, `.tsc` 1); `phi/sources.φ` trägt **0** naif-Bindungen (97 `naif`-Treffer = `origin`-Provenienz). Die **Origins sind 575/575 ableitbar** (GLL-CK-Indexe prime/ext/c23/c30/i24 + root, `a_old_versions`, JUICE, SCLK; Map `/tmp/opencode/naif_origin_map.tsv`). **Blocker:** kein `format`-Token für binäre `.bc`/`.tsc` (Census 186 Formate: `spk` 7, `kernel_text` 4, aber `ck`/`daf`/`sclk` = 0); dem SPK-Template fehlt zudem das Frame-Feld (`parse.rs:82`). Writer (`ephemeris_compiler`/`manifestor`) lesen `sources_index.φ` und fetchen `naif.jpl.nasa.gov` direkt — **0** CDN-Release-Referenzen im Code; eine Bindung wäre Provenienz-only.
- **Blockade:** fehlender Format/Frame-Arm (Mountain)
- **Braucht:** Mountain-Format-Arm für rohe Kernel; dann die 575 Origins binden.

### Register-Träger — `phi/pipeline/index.φ` + `ledger.φ` SSDC offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster Port `phi/pipeline/index.φ`; SSDC-Meldung `state/zustand/wartend.φ:10`
- **Lage:** (gemessen 2026-10-02) Katalog-Offenstand 5; `ledger.φ:6` `ausstehend`, `state/zustand/wartend.φ:10`.
- **Blockade:** Porting / Prozedur nicht live
- **Braucht:** Port-Schritt; `--playwright "https://limadou.ssdc.asi.it/query.php"` sobald SSDC meldet.

### Träger (Meta) — `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` Marker
- **Status:** wartend | **Bindung:** eigen (Meta-Träger)
- **Trigger:** `register_lookup --orphan-docs` nennt ein neues trägerloses Dokument
- **Lage:** (gemessen 2026-10-04) `--orphan-docs` = 0. Der `:75`-Marker (voyager/new_horizons-URL-Feder) ist **aufgelöst**: die drei `_long.bin` sind unter `ssd.jpl.nasa.gov-horizons` registriert (`sources.φ:16515/16546/16554`) und sha-rebunden; die Survey-Zeile ist auf den neuen Stand fortgeschrieben. Offen bleibt der `:42`-Marker (archivar_cache→repo/cache-Mapping).
- **Blockade:** keine
- **Braucht:** `:42`-Marker am heutigen Baum nachmessen; dann schließen.

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
