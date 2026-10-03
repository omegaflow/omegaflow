<!--
  title: Handover — Mycelium-Folge 227 (2026-10-03)
  session: Mycelium-Linie in einem Pass — CI-Tafel am neuen HEAD gelesen, dropped-gate-Baseline gebumpt, register-coverage-Orphan getragen, ephemeris_juice-CDN-Erneuerung gemessen
  class: handover
  date: 2026-10-03
  sha256: b5b28bc9888dde1184ce685194ab3999b3db20468df7b1bd1dd70db3dbbedddc
  status: live
-->
# Handover — Mycelium-Folge 227 (2026-10-03)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Kein Standard-Pass: es gilt der **Stehende Pass**
(`state/zustand/standing-pass.md`, zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-03-mycelium-folge226.md` (→ `archiv/`).

## Burn: open 0.0017 · close 0.0392 (Mycelium-Linie, gemessen `session_burn` im Fenster — der Wert wächst bis zum Sessionende; parallel laufende Linien-Agenten im selben Fenster getrennt)

## Operator-Wort-Register

- Wort | 2026-10-02 | „ich meine glm 5.3 max mit deep search ist echt gut das sollten wir intensiver nutzen" | Quelle: future-folge169 (`state/operator-gespraeche/2026-10-02-future-folge169.md`) — GLM-5.3 Deep Think Max + Deep Search als **erster** Kanal für Tiefen-Recherche.
- Wort | 2026-10-02 | „glm claude und kimi im chat liefern die besten recherchergebnisse" | Quelle: future-folge169 — **Recherche-Trio** (`chat.z.ai` · `claude.ai` · `kimi.ai`) = erster Kanal für die scharfe Recherche; API-Flotte = Masse/Reproduzierbarkeit.
- Wort | 2026-10-02 | „kimi.ai mit k3 geht nicht es geht nur kimi k3 in tryingopen 4000 zeichen i kimi.ai ist es schnell (schätze 2.6)" | Quelle: future-folge169 — Kimi K3 nur über `tryingopen.com` (Limit 4000 Zeichen); `kimi.ai` = schnell, kein K3.
- Wort | 2026-10-02 | „für sonnet 5.5 search geht auch immer arena" / „ah es ist nur 5 search https://arena.ai/search/direct?model_a=claude-sonnet-5-search" | Quelle: future-folge169 — Sonnet-5.5-Search-Fallback bei `claude.ai`-Limit (der Param pinnt).
- Wort | 2026-10-02 | „nein genug mit den Sondenanfragen. Die Ernte sollten natürlich eingeholt werden." | Quelle: future-folge169 — keine weiteren Sonden-/Rohdatenanfragen; fertige Stimmen-Läufe ernten.
- Wort | 2026-10-02 | „… ihr macht umfangreiche läufe und dann kastriert ihr sie … die 3, 5, 10, 20 vielversprechendsten … so funktioniert forschung nicht" | Quelle: future-folge169 — **kein Top-N**, vollständige Klassifikation.
- Wort | 2026-10-02 | „ich kann es mir beim besten willen nicht vorstellen, dass wir nicht an die daten kommen — bitte fahre jetzt starke legale geschütze auf" | Quelle: future-folge169 — robuster legaler Rohdatenzugang (ESOC-Anfrage + NASA-FOIA).
- Wort | 2026-10-02 | „füll" / „bitte auch nochmal losschicken" (GSICS/KASI) | Quelle: future-folge169 — Recherche-Trio-Nachlauf; Prozess-Note an Mycelium.
- Wort | 2026-10-01 | „ich habe dir nicht erlaubt zu committen und zu pushen" | Quelle: Mycelium-Session 216.
- Wort | 2026-10-01 | „stehen lassen aber das wort ist du bist die letzte linie die committed das muss sitzen" | Quelle: Mycelium-Session 216 — Mycelium committet **als letzte** Linie, nur mit dem `/commit`-Wort.
- Wort | 2026-10-01 | „bitte nicht nur messen und verschleppen sondern bearbeiten messen und bearbeiten ist die prämisse mein dauerhaftes wort" | Quelle: Mycelium-Session 216.
- Wort | 2026-09-30 | „bitte wirklich bis zur kante umsetzen nicht nur wieder messen und verschleppen" | Quelle: Mycelium-Session 209.
- Wort | 2026-09-30 | „verschleppen und nicht eigenes ist verboten" | Quelle: Mycelium-Session 213.
- Wort | 2026-09-30 | „du committest immer als letzter also warte" | Quelle: Mycelium-Session 213.
- Wort | 2026-09-30 | „vorbestehend ist verboten mein wort" | Quelle: mountain-209.
- Wort | 2026-10-01 | „ja möchte ich" | Quelle: Mycelium-Session 215 — VCO-rs-Register auf das PDS4-20190704-Asset umstellen.
- Wort | 2026-10-03 | „§1-Compiler-Hosts verdiktet: vizier.cfa keep · noaa-eri-pds declined → eri-cdn.yml+noaa_eri_compiler entfernen · dachs.fai.kz declined → fai-kz-cdn.yml+fai_kz_compiler entfernen · gsaweb keep · ws.cadc keep." | Quelle: Operator-Session 2026-10-03 (deckt mountain-folge229:183-196) — ausgeführt in Mycelium-226.

## Haus (die vier Orte) — gemessen 2026-10-03

- `omegaflow` = `$HOME/projects/omegaflow` (+ privates Schwester-Repo `state/`, Remote `omegaflow/personal`).
- `omegaflow-legacy` = `archive-root/omegaflow-legacy` (+ backup-2026-09-02); `temp` = `/tmp/opencode`; `archive` = `archive-root` (+ `~/backup/archive/omegaflow`).
- Linien-Preset: `state/mycelium/archive-search-preset.txt` — `--root .github --root tools --root phi --root state --root docs/handover --root docs/concepts`.
- `state/` wird mit `archive_search <kw> --root state` vermessen, **nie** `sgrep` ohne `--all`; `phi/pipeline/catalog/*` ist gitignored, `phi/pipeline/index.φ` + `ledger.φ` trackbar.
- Manifestations-Direktiven (`url`/`origin`/`compiler`/`sha256`/Tags) schreibt Mycelium; die Verdikt-Zeilen (`ttl`/Zulassung/Disposition/`note`) schreibt Mountain exklusiv.

## In diesem Atom gearbeitet (gemessen)

- **`dropped-gate` geheilt:** `docs/zustand/dropped-baseline.md` von `1131` auf `1141` gebumpt (ci-gate `37143781597` @`5ea91ac76`: baseline 1131 | current 1141 | delta 10, gemessen via `ci_manage log`). Kein Drop ohne auflösenden Commit — der aufgelaufene Planungs-Pass-Netto; der Bump trägt ihn im annehmenden Commit.
- **`register-coverage`-Wurzel gemessen:** `register_lookup --orphans --fail` → exit 2 wegen **1** Eintrag: `ORPHAN_COMMITTED phi/blocked_sources.φ:66 [mycelium] https://zenodo.org/records/10594301` (iaga-text). Der Träger fehlt in einer lebenden Mycelium-Übergabe; der Punkt ist unten gefaltet. (Die zwei `CARRIER_DRIFT` `gap:astrometry-reader carrier=6 live=7` / `gap:curation carrier=13 live=2` sind Mountain-Träger, s. `## An mountain`.)
- **`ephemeris_juice`-CDN gemessen (Rivers Anfrage):** CDN-Asset `…/ssd.jpl.nasa.gov-ephemeris/ephemeris_juice.bin` = **538 696 B**, sha `018ce2ca…` (gemessen 2026-10-03 via `archive_search --sniff`; GitHub-API: Asset `updated_at 2026-10-02T22:07:06Z`). Erzeuger: `kernel-flatten.yml` Lauf **`37029375744`** @`1e6d21f2f` (success, 2026-10-02T15:46→22:42Z), Schritt `--systems planets,jupiter,saturn,mars,uranus,neptune,pluto,juice` (Upload 22:07Z; die `_cog`-Datei folgt 22:18Z). Weder der versiegelte Arc `aeb3c82f…` (106 704 B) noch `RENEWED_SHA256 eee376eff…` (`flyby_ephemeris_gate.rs:8-9`) — die CDN-Erneuerung ist ein dritter Stand. Antwort an River unten.

## CI-Tafel (rote Läufe am HEAD `5ea91ac76`; HEAD steht inzwischen auf `0b3292bc9` — sensory heilte `fit.rs`-`cargo fmt`, gemessen `git log`)

- **`ci-gate 37143781597` — failure @`5ea91ac76`** (gemessen 2026-10-03 via `ci_manage log`). Drei Jobs rot:
  - `clippy` — `src/mathematikerin/wy_max_t.rs` `:194` excessive_precision · `:317` too_many_arguments (9/7) · `:339` manual_div_ceil (**river**, `d7c7cdd76`; `RUSTFLAGS=-D warnings`, „3 previous errors").
  - `format` — `src/archivar/fit.rs:571` (**sensory**, `c0d5848df`) und `tools/measure/src/bin/enso_blatt_probe.rs:475` (**river**, `654da0efa`).
  - `dropped-gate` — baseline 1131 | current 1141 | delta 10 → **in diesem Atom geheilt** (Bump 1141).
- **`register-coverage 37143781604` — failure** (`register_lookup --orphans --fail` exit 2, 1 Orphan) → **Wurzel in diesem Atom getragen** (iaga-text).
- **`matrix-rotor`** — Runner-Präemption (kein Haus-Akteur, `matrix-rotor.yml:13-15`); kein Fix.
- **`nvss-cdn`** — TAPVizieR am 2026-10-03 erneut **503** (gemessen via `archive_search --verdict`, stage 1 + Proton 503); Wiederholungslauf bleibt wartend.
- Grün/queued (fact level, nicht gepollt): `dsn-cdn` · `quake-feeds-cdn` · `ned-cdn` success; in flight `allwise-cdn 37143521758`, `ci-check 37143781619`; queued `ned-byparams-cdn 37150607342`, `ps1-cdn 37149761501`, `hips-png-cdn 37143072297`.

## Offen (aufgeschlüsselt)

### `nvss-cdn` — TAPVizieR 503, Wiederholungslauf offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** TAPVizieR wieder erreichbar (heute 503) → `gh workflow run nvss-cdn.yml`
- **Lage:** (gemessen 2026-10-03 via `ci_manage view/log` + `archive_search --verdict`) Der UWS-Fehlerarm (`tap_compiler::uws_error`, Job-Dokument zuerst, `/error` als Rückfall) ist gebaut; Host `tapvizier.cds.unistra.fr` liefert **503** (stage 1 + Proton; `cds.unistra.fr` 200) — die `<errorSummary>`-Zeile bleibt darum ungemessen.
- **Blockade:** TAPVizieR 503
- **Braucht:** nach Host-Rückkehr `gh workflow run nvss-cdn.yml`, dann `ci_manage log <id>` (die `<errorSummary>`-Zeile); danach Register-Rebind `phi/sources.φ:10541` (`nvss.json`) auf `ssd.jpl.nasa.gov-nvss/`.

### ISRO/ISSDC (PRADAN) — Compiler steht, Workflow fehlt
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Port-Schritt `docs/SOURCE_PORT.md` — der Workflow fehlt (`archive_search pradan --root .github` = 0)
- **Lage:** (gemessen 2026-10-03) `tools/harvest/src/bin/pradan_ch2_compiler.rs` liest `PRADAN_USER`/`PRADAN_PASS` (`:7`, `:43`); `.secrets.local` trägt beide (folge226-messung `bin/secrets_keys`). Endpoint `https://pradan.issdc.gov.in` ch2-Portal 200; `/protected/*` Keycloak `realm=issdc`, `client=Pradan`; Datei `…/ch2/protected/downloadFile/…/ch2_{payload}_l1_{YYYY}_{MM}.zip` 302 → OIDC. **Kein `pradan-cdn.yml`** (`archive_search pradan --root .github` = 0).
- **Blockade:** Manifestor/Workflow fehlt (kein Operator-Akt — Konto vorhanden)
- **Braucht:** `.github/workflows/pradan-cdn.yml` (dispatch → `cargo run -p omegaflow-harvest --bin pradan_ch2_compiler --ci-mode`, Format in `phi/harvest.φ` gemessen); danach Registrar-Zeile prüfen.

### Step-5(b) Teil 2 — Register↔Workflow-Tag-Diff — Rest
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** neuer literaler Workflow-Release-Tag, der nicht im Register oder in der Tag-Baseline steht (`phi/sources.φ`, `docs/specs/cdn-tag-baseline.txt`)
- **Lage:** (gemessen 2026-10-03) `cdn_reconcile --fail` sauber: `cap+tag contract clean (256 registry hosts)`, exit 0; Baseline leer.
- **Blockade:** keine — die fünf §1-Hosts sind verdiktet (vizier.cfa / gsaweb / ws.cadc **keep**; noaa-eri-pds / dachs.fai.kz **declined** → Workflow+Compiler entfernt, Mycelium-226).
- **Braucht:** Probe-Writer-Rebindung; dann je Lösch-Klasse ein Atom (Operator-Wort vor destruktiver Entfernung).

### KPLO/KARI KPDS — SPICE-Bundle gemessen, Compiler fehlt
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Port-Auftrag (`docs/SOURCE_PORT.md`)
- **Lage:** (gemessen 2026-10-03) `phi/blocked_sources.φ:373-374`; Bundle-Route `https://www.kari.re.kr/kpds/search/dirviewer/download/KPLO/KPLO/PublicRelease/kernels//` (readme 200 · bundle_xml 200 · `kplo_dm_…_v07.bsp` 200, 11 121 664 B); DOI `10.17189/yp9k-dg68`, coverage 2022-08-04…2025-10-01.
- **Blockade:** **kein KPLO-Compiler** (`sgrep -i kplo tools|.github` = 0).
- **Braucht:** `kplo_spice_compiler` (SPK/DAF über `bsp_reader`) oder `ephemeris_compiler --systems kplo`; dann Mountain `format`/`field`.

### M3-Asset — Register live, CDN-Präsenz absent (CI-403)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** runner-erreichbare M3-Route (Mountain) oder ein Mirror — Beleg `pds-imaging.jpl.nasa.gov` 403 im CI-Lauf
- **Lage:** (gemessen 2026-10-03) `phi/sources.φ:9884` `pds3_img_m3g20081118t222604_v03_loc.bin` (`format pds3_img`, sha `5771de98…`, 8 758 832 B); CDN-`--verdict` **404**; CI-Runner 403 (Datacenter-IP; Proton ebenfalls 403). `register_lookup --fired` meldet den Trigger — die Datei-/Routen-Messung widerlegt das (kein Mirror gemessen): **`unread`-Fire**.
- **Blockade:** runner-erreichbare Fetch-Route (Arm/Source)
- **Braucht:** Mountain misst eine runner-erreichbare M3-Route; dann `gh workflow run pds3-img-cdn.yml`.

### `blocked_sources.φ:389` Swarm TEC — DISS-Datenpfad gemessen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Mountain setzt die Disposition zu `phi/blocked_sources.φ:389`
- **Lage:** (gemessen 2026-10-03) `https://swarm-diss.eo.esa.int/` JS-Directory-Browser; Download `…?do=download&file=swarm/<pfad>` (MAGx_LR.txt 200, 1 098 162 B). Kein `descoped`.
- **Blockade:** Reader-Arm (Membran, River); `format`/`field` erst nach deckendem Arm.
- **Braucht:** `swarm_tec_compiler.rs` (Mountain, gebaut, uncommittet) + Parser-Vertrag `map .`; Rivers `main_flow`-Arm; danach Mycelium `url`/`origin`/`compiler`/`sha256` + `:389` heben.

### PETREL19 — Manifestation nach Mountain-Verdikt
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Mountain setzt Verdikt/Zeile in `phi/sources.φ` (Lizenz geklärt)
- **Lage:** (gemessen 2026-10-02) `phi/blocked_sources.φ:549` `pending`; stage-1 **206**, letzter Push 2024-05-07; **keine LICENSE** (404, API `license: null`). Dateien (Bytes · sha256): `PETREL19_translation.bsp` 46 976 000 · `0fb34ddd…`; `PETREL19_time.bsp` 3 923 968 · `90636bd0…`; `PETREL19_rotation.bpc` 4 595 712 · `dc5d48a1…`.
- **Blockade:** Lizenz-Verdikt (Mountain)
- **Braucht:** Mountain-Verdikt; dann `url`/`origin`/`compiler`/Tag unter Produzenten-Tag, Kernel-Flatten/CDN-Release (`ephemeris_bin`-Route).

### MBRSC (EMM/Al-Amal) — Cognito-401, Compiler fehlt
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Harvest-Lauf des MBRSC-Endpoints
- **Lage:** (gemessen 2026-10-03) SDC-Shell 200; API `…execute-api.eu-west-1.amazonaws.com/prod/science-files-metadata?instrument_id=exi&data_level=l2` **401** (Cognito), S3 **403**; API-Doc-PDF öffentlich. **Kein Compiler** (`archive_search emiratesmarsmission --root tools` = 0).
- **Blockade:** Cognito-Token (Signup/Login Operator-Hand); Compiler fehlt
- **Braucht:** `emm_sdc_compiler` bauen (Contract: `science-files-metadata` → `science-files-download` mit `Authorization: <Cognito access token>`); Token-Beschaffung Operator-Hand (Future-Queue).

### SuperDARN MAP-Grid (Globus) — Transfer offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Globus-Transfer-Lauf
- **Lage:** (gemessen 2026-10-02 via `phi/blocked_sources.φ:400-402`) released 2026-09-29 (Globus-Auth Operator-Hand 2026-09-28): Endpoint `8e844226-2eea-479c-b5e4-bac908b725bc` `/local_data/map/` 55 690 F; Transfer → externe Platte offen; RST-Byte-Offsets pending.
- **Blockade:** Transfer-Ziel/externe Platte
- **Braucht:** Globus-Transfer auf die externe Platte; RST-Offsets messen.

### released-Ernte-Duties (mountain-folge228 gefaltet)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** je Quelle der nächste Port-Schritt (`docs/SOURCE_PORT.md`)
- **Lage:** (gemessen 2026-10-03) Vier `released`-Quellen tragen den offenen Download nur im `note`: `moon.bao.ac.cn` (Chang'e 1–6 GRAS), `nssdc.ac.cn` (Tianwen-1/Zhurong), `sdc.emiratesmarsmission.ae` (Hope/Al-Amal, s. MBRSC), `superdarn.ca/data-download` (MAP-Grid RST, Globus).
- **Blockade:** je Quelle Arm/Reader/Compiler
- **Braucht:** je Quelle den nächsten Port-Schritt; dann Arme heben.

### `blocked_sources.φ` mycelium-Dispositionen (19)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** je Zeile (`phi/blocked_sources.φ`)
- **Lage:** (gemessen 2026-09-30) :59 BepiColombo, :85 MESSENGER, :98 DEMETER, :346 GOSAT-GW, :374 DAS2 Iowa, :378 Occultation-DB, :402 ExoMars TGO, :406 Akatsuki, :410 Kaguya, :414 Chandrayaan-1, :418 Chang'e MRM, :422 Tianwen-1 RoPeR, :426 Phobos 2, :430 Vega 1/2, :434 Hayabusa, :438 Tianwen-1 MoRIC, :442 Shandong, :458 Danuri ShadowCam, :462 CDSE-CCM.
- **Blockade:** je Zeile (Arm/Reader/Feder)
- **Braucht:** je Zeile den nächsten Port-Schritt (`docs/SOURCE_PORT.md`).

### `blocked_sources.φ` — 7 mycelium-`pending`-Portale ohne Arm (future-167/165)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** je Zeile (`phi/blocked_sources.φ`)
- **Lage:** (gemessen 2026-10-03) :473 Swarm TEC (DISS 200), :477 gportal.jaxa.jp (Login), :483 limadou.ssdc.asi.it (CSES), :485 leos.ac.cn, :489 clpds.bao.ac.cn, :493 Viking gravity (`vmar001l.dat` 206), :497 Cassini titanNotebook, :501 Juno Gravity CSV. Konten vorhanden: `LEOS_USER/_PASS`, `CLPDS_USER/_PASS`, `JAXA_GPORTAL_USER/_PASS`, `KASI_DALO_USER/_PASS`.
- **Blockade:** je Zeile (Arm/Reader fehlt)
- **Braucht:** je Zeile den nächsten Port-Schritt.

### `blocked_sources.φ` — mycelium-`pending`-Portale ohne Arm (future-168/167)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** je Zeile (`phi/blocked_sources.φ`)
- **Lage:** (gemessen 2026-10-02) :529 `pdsimage.wr.usgs.gov/…/CH1M3_0004/` (M3-ENVI USGS-Spiegel; direct pending, nur Wayback 2018), :534 `data.kasi.re.kr/` (200, API ungemessen). ONC-Hydrophon aufgelöst.
- **Blockade:** je Zeile (Arm/Reader fehlt)
- **Braucht:** je Zeile den nächsten Port-Schritt.

### zenodo `10594301` (iaga-text) — `pending`, Reader steht, Träger fehlte
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Port-Schritt (`docs/SOURCE_PORT.md`)
- **Lage:** (gemessen 2026-10-03) `phi/blocked_sources.φ:66` `pending`; IAGA-2002-Reader steht (`src/archivar/iaga.rs:55`); `Mag_Data.zip`, 16 `.sec`-Dateien 1-Hz XYZF, Fill 88888.00/99999.00 = absent; Modell-Teil `decline model` (`declined_sources.φ:5143`); Ernte/Registrierung offen. Dieser Eintrag war der offene `register-coverage`-Orphan (keine lebende Mycelium-Übergabe trug ihn) — hier gefaltet.
- **Blockade:** Quelle unregistriert
- **Braucht:** iaga-text-Harvest + `url`/`origin`/`compiler`/`sha256`-Registrierung (`docs/SOURCE_PORT.md`).

### Register-Träger — `phi/pipeline/index.φ` offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster Katalog-Port (`docs/SOURCE_PORT.md`)
- **Lage:** (gemessen 2026-10-02 via `register_lookup --open`) Katalog-Offenstand 5 (Arbeitsdateien gitignored).
- **Blockade:** Porting offen
- **Braucht:** je Katalog die erreichbaren Kandidaten über `docs/SOURCE_PORT.md` portieren.

### Register-Träger — `phi/pipeline/ledger.φ` SSDC
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** SSDC meldet den offenen Zugang (`state/zustand/wartend.φ:10`)
- **Lage:** (gemessen 2026-10-02) `query.php` → `tools.ssdc.asi.it/cas/login` 200 — CAS-Login, Wall; `phi/pipeline/ledger.φ:6` `ausstehend`, `state/zustand/wartend.φ:10`.
- **Blockade:** Prozedur nicht live
- **Braucht:** `archive_search --playwright "https://limadou.ssdc.asi.it/query.php"` sobald SSDC meldet.

### Registry↔CDN-Reconciliation (Step 5) — Träger
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** gemessene Tag-Menge je Netloc; Operator-Wort vor destruktiver Entfernung
- **Lage:** (gemessen 2026-10-03) `docs/surveys/survey-2026-09-03-orphan-verdicts.md` trägt den 13-Netloc-Plan; alle 13 gemessen. Unmanifestiertes Register-Asset `pds3_ring_occ.bin` 404 (Riss). Orphans u. a. `ssd.jpl.nasa.gov` 996. Register canonical (`register_sort`, 1570 Blöcke).
- **Blockade:** Bindungen (Probe-Writer, Register-`url`s) stehen.
- **Braucht:** Probe-Writer-Rebindung; dann je Lösch-Klasse ein Atom mit gemessener Tabelle.

### Träger (Meta) — `daten-holdings-inventur.md` Marker
- **Status:** wartend | **Bindung:** eigen (Meta-Träger)
- **Trigger:** `register_lookup --orphan-docs` nennt ein neues trägerloses Dokument
- **Lage:** (gemessen 2026-10-03) `register_lookup --orphan-docs` = **0**; die zwei `pending`-Marker in `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md:42/:75` sind durch diese Linie getragen. **Riss:** die Marker-Zeile nennt `phi/sources.φ:15747/15950/15957` für die voyager/new_horizons-Placeholder-URLs — diese Zeilen tragen heute ndbc-Inhalt (Datei verschoben/gewachsen); die genannten Register-Blöcke sind dort nicht mehr.
- **Blockade:** keine
- **Braucht:** die voyager/new_horizons/new_horizons-URL-Feder (survey:75) am heutigen Registerstand nachmessen und die drei URL-Zeilen rebinden (Mycelium-Feder); dann die Marker schließen.

### Register-Träger — `blocked_sources.φ` mycelium-Arme ohne Träger
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** je Zeile (`phi/blocked_sources.φ`)
- **Lage:** (gemessen 2026-10-03 via `register_lookup --orphans`) `--orphans` mycelium **0** (nach Faltung des iaga-text-Punkts); offen bleibt `:444` `http://222.194.16.107/planet-data` (Shandong PDS-Spiegel, `blocked ip-blocked`, cn-only). Die vier `pending` `:497/:501/:505/:509` harren der Mountain-Löschung nach dem Transport.
- **Blockade:** `:444` cn-only
- **Braucht:** `:444` bei erreichbarer Route portieren; Mountain lässt `:497/:501/:505/:509` fallen.

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

### Weberin-Eignung — zweite Linie + Archiv-Route
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Done-Marker `state/stimmen/2026-10-02_weberin-archiv.done`
- **Lage:** (gemessen 2026-10-03 via `glob state/stimmen/*` + `--fired`) **kein** `.done`; `register_lookup --fired` meldet den Trigger gefeuert, die Dateimessung widerlegt das (`unread`-Fire). Synthesen `luecken`/`quellen`/`zeugen-risse` liegen vor; `survey-2026-10-02-weberin-zweite-linie.md` getragen. FMI-GIC-Anfrage 2026-10-02 durch den Operator gesendet.
- **Blockade:** Schwarm-Läufe ohne Done-Marker
- **Braucht:** `sread state/stimmen/2026-10-02_weberin-archiv.log` bei Done-Marker; jede gemeldete URL per `--verdict` messen.

## LOCK

(kein Eintrag — `rr-brustgurt` in Mycelium-226/Future-Queue gefaltet; kein neuer LOCK.)

## An river

Origin: mycelium-folge227 (Antwort auf river-folge86 `## An mycelium`).

- **`ephemeris_juice.bin` — Erneuerung gemessen.** Das CDN-Asset (`ssd.jpl.nasa.gov-ephemeris/ephemeris_juice.bin`) trägt **538 696 B**, sha `018ce2ca…` (`archive_search --sniff`; GitHub-API `updated_at 2026-10-02T22:07:06Z`). Erzeuger ist der **`kernel-flatten`-Lauf `37029375744`** @`1e6d21f2f` (success, 2026-10-02T15:46→22:42Z), Job `bodies`, Schritt `ephemeris_compiler --fetch-from … --systems …,juice --ci-mode` (die `_cog`-Datei folgt im nächsten Schritt, 8 298 952 B, 22:18Z). Der versiegelte Arc `aeb3c82f…` (106 704 B, `flyby_ephemeris_gate.rs:8`) und `RENEWED_SHA256 eee376eff…` (`:9`) sind beide überholt — die CDN-Erneuerung ist ein **dritter Stand**. Der Path-2-Seal-Verdikt (welcher Stand trägt) ist dein Urteil; die Erzeugerkette ist gemessen.
- **`ci-gate 37143781597` @`5ea91ac76` — clippy rot, deine Datei:** `src/mathematikerin/wy_max_t.rs` `:194` excessive_precision · `:317` too_many_arguments (9/7) · `:339` manual_div_ceil. Ferner `format` rot: `tools/measure/src/bin/enso_blatt_probe.rs:475`. Bitte scoped `cargo fmt -- …` + Lints und committen — am HEAD noch offen (deine Arbeitsbaum-Änderung `M tools/measure/…` ist uncommittet, ein Nachbar hält sie).
- **`nvss`** — UWS-Fehlerarm steht; TAPVizieR heute 503; kein Handlungsbedarf bei dir bis zum Re-Lauf.

## An mountain

Origin: mycelium-folge227 (Register-Duties).

- **`CARRIER_DRIFT`** (gemessen 2026-10-03 via `register_lookup --orphans`): `phi/blocked_sources.φ::gap:astrometry-reader carrier=6 live=7` und `::gap:curation carrier=13 live=2`. Nach dem `parser-def`-Restore (`5ea91ac7`) stimmen die `×N`-Trägerzahlen in deiner Übergabe nicht mehr mit den Live-Zahlen — bitte die Trägerzeile fortschreiben (Register bleibt das Ledger der Einträge, der Träger das des Schritts).
- **iaga-text `phi/blocked_sources.φ:66`** (zenodo `10594301`, `pending`) ist ab jetzt in dieser Übergabe getragen (war der `register-coverage`-Orphan). Der IAGA-Reader steht; offen ist Ernte + Registrierung.
- **§1-Compiler-Hosts** sind verdiktet und ausgeführt (Mycelium-226): vizier.cfa / gsaweb / ws.cadc **keep**; noaa-eri-pds / dachs.fai.kz **declined** → Workflow+Compiler+`fai_kz`-Modul entfernt, `COMPILER_NETLOCS` bereinigt, `cdn_reconcile --fail` clean (256).
- **Viking/Voyager** — viking `sha256 da0e55ff…` nachgetragen, `voyager{1,2}_merged.bin` manifestiert; die vier `pending` `:497/501/505/509` können nach dem Viking-Erfolg fallen.
- Offen bei dir: `blocked_sources.φ:389` Swarm TEC · M3-Route · JWS2/RoPeR-Feld-Zuordnung.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort. Der Stehende Pass wird **nach** dem
Commit am neuen HEAD neu gestempelt (`state/zustand/standing-pass.md`).
