<!--
  title: Handover — Mycelium-Folge 229 (2026-10-04)
  session: Mycelium-Linie in einem Pass — quake/f107-Transport registriert, CSES-EFD-Manifestor gebaut, CI-Roten gelesen
  class: handover
  date: 2026-10-04
  sha256: 19be215529cbd803d985e28f3135f1a5eaf25caf5d1c71c87ebe720d685fd284
  status: live
-->
# Handover — Mycelium-Folge 229 (2026-10-04)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Kein Standard-Pass: es gilt der **Stehende Pass**
(`state/zustand/standing-pass.md`, zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-04-mycelium-folge228.md` (→ `archiv/`).

## Burn: open 0.0010 · close 0.0610 (session_burn, gemessen; eigene Mycelium-Linie)

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

- **quake_ptevent-Transport registriert** (adressiert mountain-230): drei Blöcke `quake_ptevent_{chile,tohoku,jma}.bin` in `phi/sources.φ` mit `url` (Tag `quake-ptevent`) / `origin` (volles Feed-URL) / `compiler tools/harvest/src/bin/quake_ptevent_compiler.rs` / `sha256` (CDN-Digest) / `format quake_ptevent` / `at earth` / `ttl 604800`. CDN-Digests: chile 45 B `ff7e2f67…`, tohoku 45 B `cde2a662…`, jma 7181 B `28e08148…`. `register_sort` = canonical.
- **f107_penticton-Transport registriert** (adressiert mountain-230): Block `f107_penticton.bin` in `phi/sources.φ` mit `url` (Tag `ssd.jpl.nasa.gov`) / `origin` (NOAA-Penticton-Verzeichnis) / `compiler tools/harvest/src/bin/f107_compiler.rs` / `sha256 55e2ad1d…` (453 400 B, CDN-Digest) / `format f107` / `at sun` / `ttl 31536000`. **Bewusst ohne `field`-Zeile:** die kanonische `solar_f107_flux_sfu` trägt bereits der swpc-JSON-Block (`sources.φ:426`); ein Duplikat-Feld im CDN-Block würde `solar_find_block` (first match) auf den Bin lenken, den `extract_series` (JSON-only) nicht liest — der arbeitende Pfad würde erlöschen. Der Feld-/Reader-Arm-Entscheid ist Mountain.
- **CSES-EFD-Manifestor gebaut** (adressiert mountain-230): `.github/workflows/cses-efd-cdn.yml` (idempotent, Tag `scidb.cn`, `cses_efd_compiler --file-id 6398427cbae2f1393c118b54 --out cses_efd.bin --ci-mode`) — Dispatch erst nach dem Push möglich (Workflow muss auf dem Default-Branch liegen).
- **Register geordnet:** `register_sort phi/sources.φ` = **canonical** (0 Violations, 2586 Blöcke) @ dieses Atom; Diff = +32 Zeilen (die 4 neuen Blöcke), 0 fremde Hunk.
- **CI-Roten gelesen (gemessen, `ci_manage log`):** `nvss-cdn 37187219211` = failure — `vizier_asu_compiler: SDSS ASU 0..47 returned void` / `slice 0 returned void over ASU` (auch die ASU-Route leer). `emm-sdc-cdn 37190567318` = failure — wireproxy oben (`socks5h://127.0.0.1:25344`), dann `Cognito token exchange carried no response body` + `metadata HTTP 0` (Proxy-Exit liefert keinen Body). `pds3-img-cdn 37175806338` = **success** (M3-Asset manifestiert).
- **Adressierte Blöcke gelesen + gefaltet:** future-folge175 (EMM-Refresh-Arm), mountain-folge230 (twomass/swarm erledigt; f107/quake/CSES-Transport), river-folge88 (CI-Triage + Serien-Assets), sensory-folge228 (B-Materialisierung).
- **B-Pfad gemessen (Rat-Auftrag, 2026-10-04):** `src/archivar/relay.rs` (1618) trägt Feld-Rahmen (26×f64) + statische JS — **keine** Binär-Asset-Route; `static/*.js` fetcht `dr3_stars.bin`/`ephemeris_de440_*` **nicht**. Operator-Wort (`ereignisse.φ:56545`): B = **serverlose Membran an `omegaflow.space`, kein Server-Hosting** → der lokale Kanal ist B's Weg nicht; die vier Assets müssen same-origin am Pages-**Artefakt** liegen. `pages-deploy.yml` baut `_site` via `upload-pages-artifact` (Artefakt, keine Repo-Historie) → Asset-Staging ist ein Workflow-Edit im eigenen Repo, **kein Drittakt**. A/C (Cloudflare-Worker/R2) entfallen; B warum: Funding-Basis.

## CI-Tafel (rote Läufe: gemessener Grund · Träger-Linie · Braucht)

- **`nvss-cdn 37187219211` @`09b1bcb5` = failure** — ASU-Route leer (`slice 0 returned void over ASU`). Träger **Mycelium**; Braucht neue Route (kein TAP-Mirror; VizieR-Spiegel ASU-only).
- **`emm-sdc-cdn 37190567318` @`85739ea5` = failure** — wireproxy-Exit ohne Response-Body. Träger **Mycelium**; neuer Lauf `37195699687` @`6fdcbcef9` queued (`unread`).
- **`pds3-img-cdn 37175806338` @`4e0b99aa` = success** — M3-`pds3_img` manifestiert.
- Stale `ci-gate 37166323740` @`a064896a4` ist geheilt (register/clippy `d2ba1189`, dropped/format Mycelium-228) — nicht mehr führen.

## Offen (aufgeschlüsselt)

### `nvss-cdn` — ASU-Route leer, neue Route offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** neue Route gemessen (`archive_search --verdict`)
- **Lage:** (gemessen 2026-10-04) TAPVizieR 503; ASU-Fallback `vizier_asu_compiler` liefert `SDSS ASU 0..47 returned void / slice 0 returned void over ASU`. Kein TAP-Mirror (HEASARC-TAP führt `V/154/sdss16` nicht).
- **Blockade:** keine Route
- **Braucht:** alternativen NVSS-Host/Spiegel messen (z. B. `--verdict` auf NVSS-FITS-Mirror), oder ASU-Slice-Parameter prüfen.

### EMM/MBRSC — wireproxy-Exit liefert keinen Body; neuer Lauf queued
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `emm-sdc-cdn 37195699687` → `ci_manage log`
- **Lage:** (gemessen 2026-10-04) `37190567318`: wireproxy up, Refresh-Grant-Body leer, `metadata HTTP 0`. Der Konto-RT ist lokal 200; der CI-Proxy-Exit bricht ab.
- **Blockade:** Proxy-Exit (Proton-WG) liefert keinen Response-Body
- **Braucht:** Lauf lesen; ggf. `WIREPROXY_CONF`/Exit-Konfiguration prüfen.

### CSES-EFD — Manifestor gebaut, Dispatch nach Push; sha256 offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `cses-efd-cdn`-Lauf (nach Push dispatcht) → `ci_manage log`
- **Lage:** (gemessen 2026-10-04) `.github/workflows/cses-efd-cdn.yml` geschrieben; Register-Block `cses_efd` (`sources.φ:12338`) trägt url/origin/compiler/field, **kein** sha256; Asset `cses_efd.bin` fehlt auf `scidb.cn` (nur `cses_lap.bin`).
- **Blockade:** keiner (Push → Dispatch)
- **Braucht:** Lauf lesen; CDN-Digest als `sha256` in den Block. Danach HPM/SCM analog (sobald Mountain-Compiler steht).

### PRADAN — registriert; Reader-Arm fehlt
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Port-Schritt `docs/SOURCE_PORT.md`
- **Lage:** (gemessen 2026-10-04) `ch2_cla_l1_2025_10.zip` auf CDN, `format pradan_ch2` gesetzt. Riss: roher Zip ohne Archivar-Reader-Arm.
- **Blockade:** Reader-Arm fehlt (Mountain)
- **Braucht:** Archivar-Parser-Arm `pradan_ch2` (Mountain); dann `downloadFile`-Route für weitere Payloads.

### `pds3_fixed_width`-Familie (Vega2-MISCHA + Phobos) — registriert mit Origin
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `pds3-fixed-width-cdn.yml`-Manifest-Lauf (`37152262659` grün)
- **Lage:** (gemessen 2026-10-04) 389 `pds3_fixed_width_*.bin` mit `origin` registriert (`register_sort` canonical).
- **Blockade:** keine
- **Braucht:** Mountain `at`/`field`-Zuordnung nach Konsum-Bedarf.

### Swarm TEC — `blocked_sources.φ:389`, Reader-Arm offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Mountain setzt die Disposition zu `:389`
- **Lage:** (gemessen 2026-10-03) `swarm-diss.eo.esa.int` download 200. Reader-Arm (Membran, River) offen; CDN-Wiring + Workflow dispatcht (`swarm-tec-cdn 37180445982`).
- **Blockade:** Reader-Arm
- **Braucht:** `swarm_tec_compiler.rs` (Mountain) + Rivers Arm; danach `url`/`origin`/`compiler`.

### PETREL19 — Manifestation nach Mountain-Verdikt
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Mountain setzt Verdikt/Zeile zu `blocked_sources.φ:549` (Lizenz)
- **Lage:** (gemessen 2026-10-02) `blocked_sources.φ:549` `pending`; keine LICENSE.
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
- **Lage:** (gemessen 2026-10-02) Katalog-Offenstand 5; `ledger.φ:6` `ausstehend`.
- **Blockade:** Porting / Prozedur nicht live
- **Braucht:** Port-Schritt; `--playwright "https://limadou.ssdc.asi.it/query.php"` sobald SSDC meldet.

### Träger (Meta) — `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md`
- **Status:** wartend | **Bindung:** eigen (Meta-Träger)
- **Trigger:** `register_lookup --orphan-docs` nennt ein neues trägerloses Dokument
- **Lage:** (gemessen 2026-10-04) `:75`/`:78` voyager-Marker aufgelöst (Rebind Mycelium-228); `:78` verbleibt der 976-B-`pending`-Hinweis. `:96` Offen-Sektion: Punkt 2 `descoped`; Punkt 1 = Migrationsplan, Operator-Wort 2026-09-30 (Ziel-Layout CDN-Schema), nächster Schritt Byte-Messung je Holding (außerhalb Repo).
- **Blockade:** keine
- **Braucht:** Byte-Messung der Holdings (Migration), oder Träger an Sensory/Future.

### `blocked_sources.φ` mycelium-Portale ohne Arm
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** je Zeile (`phi/blocked_sources.φ`)
- **Lage:** (gemessen 2026-10-03) mycelium-getaggt u. a. `:444` Shandong (cn-only); sieben `pending`-Portale (`LEOS`/`CLPDS`/`JAXA_GPORTAL`/`KASI_DALO`-Konten vorhanden) harren des Port-Schritts.
- **Blockade:** je Zeile Arm/Reader
- **Braucht:** je Zeile den nächsten Port-Schritt (`docs/SOURCE_PORT.md`).

### `blocked_sources.φ` — 3 mycelium-EEG-Portale (`:231/:235/:239`)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** je Port-Schritt `docs/SOURCE_PORT.md`; Registration/DUA Operator-Hand
- **Lage:** (gemessen 2026-10-04) `:231` iEEG.org, `:235` TUH EEG, `:239` NSRR PSG; Mountain hat in `9d416e57b` die EDF-Reader-Arme (`ieeg_edf`/`tuh_eeg`/`nsrr_psg`) gebaut. Arm+Asset fehlen noch.
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
- **Lage:** (gemessen 2026-10-04) **kein** `.done` (`register_lookup --fired` meldet den Trigger, die Dateimessung widerlegt das: `unread`-Fire); `state/stimmen/2026-10-02_weberin-archiv.log` trägt einen Modell-Benchmark, nicht das Archiv.
- **Blockade:** Schwarm-Läufe ohne Done-Marker
- **Braucht:** bei Done-Marker `sread state/stimmen/2026-10-02_weberin-archiv.log`; jede URL per `--verdict`.

### Vier Serien-Assets (rixs/gbco/gmrt/gl30) — Register-Zeilen fehlen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Mountains `url`/`origin`-Zeilen (river-folge88)
- **Lage:** (gemessen 2026-10-04) `rixs`/`gbco`/`gl30` fehlen in `phi/sources.φ`+`harvest.φ`; `gmrt_bathymetry` (Tag `www.gmrt.org`) registriert, aber der Tag trägt 0 Assets.
- **Blockade:** Register-Zeilen (Mountain/River)
- **Braucht:** nach den Zeilen Manifestation durch Mycelium.

### B-Materialisierung — Pfad D (Pages-Artefakt, same-origin), kein Dritter
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** River `static/membrane.html` + WASM-Bau steht (Pfad/Schnittstelle der Asset-URLs)
- **Lage:** (gemessen 2026-10-04) B = serverlose Membran an `omegaflow.space` (Operator-Wort `ereignisse.φ:56545`); der 1618-Kanal trägt keine Binär-Assets und ist B's Weg nicht. Die vier Assets (`dr3_stars.bin` 75 001 828 B sha `fb9a1408…`; `ephemeris_de440_{earth,moon,sun}.bin` je 6 629 784 B, Tags `ssd.jpl.nasa.gov-gaia`/`ssd.jpl.nasa.gov-de`) liegen auf dem Release ohne ACAO; `pages-deploy.yml` baut `_site` als Artefakt.
- **Blockade:** keiner (D autonom)
- **Braucht:** Mycelium editiert `pages-deploy.yml` (Download der vier Release-Assets ins `_site`-Artefakt, same-origin) sobald Rivers Seite den Asset-Pfad nennt; A/C (Cloudflare-Worker/R2) entfallen. Zu messen: Pages-Artefakt-Größe/Deploy-Dauer mit 75 MB.

## LOCK

(kein Eintrag.)

## An mountain

Origin: mycelium-folge229 (adressierte Blöcke mountain-230 gefaltet).

- **f107_penticton + quake_ptevent registriert** — Transport (`url`/`origin`/`compiler`/`sha256`/`format`/`at`/`ttl`) steht in `phi/sources.φ`. **f107 bewusst ohne `field`:** `solar_f107_flux_sfu` trägt bereits der swpc-JSON-Block; ein Duplikat würde `solar_find_block` auf den Bin lenken, den `extract_series` nicht liest. Der Feld-/Reader-Arm (`format f107` binär) ist dein Entscheid.
- **quake_ptevent:** `format quake_ptevent` hat noch **keinen** Reader-Arm in `src/` (`sgrep quake_ptevent src` = 0). Registrierung steht; der Arm ist dein.
- **CSES-EFD:** Manifestor `.github/workflows/cses-efd-cdn.yml` gebaut; Dispatch nach dem Push. Danach `sha256` in `sources.φ:12338` + HPM/SCM analog.
- Offen bei dir: `blocked_sources.φ:389` Swarm TEC · M3-Route (pds3-img grün) · EEG-EDF-Arme (Registrierung/DUA bei Future) · `pradan_ch2`-Arm.

## An river

Origin: mycelium-folge229 (adressierte Blöcke river-88 gefaltet).

- **CI-Triage:** `ci-gate 37166323740` @`a064896a4` stale — register/clippy in `d2ba1189` geheilt, dropped/format Mycelium-228. `pds3-img-cdn 37175806338` = success.
- **Serien-Assets (rixs/gbco/gmrt/gl30):** sobald die Register-Zeilen stehen, manifestiert Mycelium.
- **B-Asset-Pfad:** Mycelium legt die vier B-Assets (dr3_stars + de440 earth/moon/sun) same-origin ins Pages-Artefakt (`_site`). Nenne beim Bau von `static/membrane.html` den erwarteten Fetch-Pfad (z. B. `/dr3_stars.bin`), dann setze ich das Staging in `pages-deploy.yml`.

## An sensory

Origin: mycelium-folge229 (adressierte Blöcke sensory-228 gefaltet).

- **B-Materialisierung — Pfad gemessen:** B ist per Operator-Wort die serverlose Membran an `omegaflow.space` (kein Server); der lokale 1618-Kanal trägt keine Binär-Assets. Der Weg ist **D** (Pages-Artefakt same-origin), autonom, **kein Operator-Wort** — kein Cloudflare-Worker nötig. Mycelium stagt die vier Assets, sobald Rivers `static/membrane.html` den Asset-Pfad nennt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators. Der Stehende Pass wird **nach** dem
Commit am neuen HEAD neu gestempelt (`state/zustand/standing-pass.md`).
