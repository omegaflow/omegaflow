<!--
  title: Handover — Mycelium-Folge 226 (2026-10-03)
  session: Mycelium-Linie in einem Pass — gefeuerte Läufe gelesen, RoPeR-Familie (41 Zeilen) registriert, dropped-gate-Shallow-Artefakt geheilt, UWS-Fehlerarm gebaut
  class: handover
  date: 2026-10-03
  sha256: e72bffd6b9023253efd295ffbf05387f36140b782fb1cea48856205852ab6370
  status: live
-->
# Handover — Mycelium-Folge 226 (2026-10-03)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Kein Standard-Pass: es gilt der **Stehende Pass**
(`state/zustand/standing-pass.md`, zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-03-mycelium-folge225.md` (→ `archiv/`).

## Burn: open 0.0017 · close 0.0464 (Mycelium-Linie, gemessen `session_burn`) + 0.3654 (grind-flash-Dispatches im Fenster; Grund: gefeuerte Läufe lesen, RoPeR-Extraktion, ci-gate-Diagnose, UWS-Fix, Handover)

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

## Haus (die vier Orte) — gemessen 2026-10-03

- `omegaflow` = `$HOME/projects/omegaflow` (+ privates Schwester-Repo `state/`, Remote `omegaflow/personal`).
- `omegaflow-legacy` = `archive-root/omegaflow-legacy` (+ backup-2026-09-02); `temp` = `/tmp/opencode`; `archive` = `archive-root` (+ `~/backup/archive/omegaflow`).
- Linien-Preset: `state/mycelium/archive-search-preset.txt` — `--root .github --root tools --root phi --root state --root docs/handover --root docs/concepts`.
- `state/` wird mit `archive_search <kw> --root state` vermessen, **nie** `sgrep` ohne `--all`; `phi/pipeline/catalog/*` ist gitignored, `phi/pipeline/index.φ` + `ledger.φ` trackbar.
- Manifestations-Direktiven (`url`/`origin`/`compiler`/`sha256`/Tags) schreibt Mycelium; die Verdikt-Zeilen (`ttl`/Zulassung/Disposition/`note`) schreibt Mountain exklusiv.

## Offen (aufgeschlüsselt)

### `nvss-cdn` — UWS `phase ERROR`, Fehlerarm gebaut, Host 503
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** TAPVizieR wieder erreichbar (heute 503) → `gh workflow run nvss-cdn.yml`
- **Lage:** (gemessen 2026-10-03 via `ci_manage view/log`) Lauf `37116107265` **cancelled**; Lauf `37116852558` **failure** — async UWS-Job springt ~11 s nach `PHASE=RUN` auf `ERROR`, der Compiler druckt nur `the query stays unharvested`. **Wurzelursache des blinden Fehlers gemessen** (`tools/harvest/src/bin/tap_compiler.rs:225`): `uws_error` holte `{job}/error` (auf TAPVizieR leer), nie das Job-Dokument (`GET {job}`) mit `<errorSummary>`. **Fix gebaut** (`tag_text` + Job-Dokument zuerst, `/error` als Rückfall), `cargo build -p omegaflow-harvest --bin tap_compiler` grün. `tapvizier.cds.unistra.fr` liefert aktuell **503** (vhost down, `cds.unistra.fr` 200) — Serverfehlertext darum nicht messbar. Der frühere folge225-Vermerk „RA-chunked geheilt" ist durch Lauf `37116852558` widerlegt.
- **Blockade:** TAPVizieR 503 + Serverursache noch nicht sichtbar (Fix macht sie sichtbar)
- **Braucht:** nach Host-Rückkehr `gh workflow run nvss-cdn.yml`, `ci_manage log <id>` — dann die `<errorSummary>`-Zeile als Ursache; Register-Rebind `phi/sources.φ:10541` (nvss.json) auf `ssd.jpl.nasa.gov-nvss/` erst nach 206.

### Step-5(b) Teil 2 — Register↔Workflow-Tag-Diff (gebaut) — Rest
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** neuer literaler Workflow-Release-Tag, der nicht im Register oder in der Tag-Baseline steht (`phi/sources.φ`, `docs/specs/cdn-tag-baseline.txt`)
- **Lage:** (gemessen 2026-10-03) `cdn_reconcile --fail` sauber: `cap+tag contract clean (256 registry hosts)`, exit 0. Die **6 Drifts** bleiben aufgelöst; `docs/specs/cdn-tag-baseline.txt` leer.
- **Blockade:** die fünf als §1 geführten Hosts tragen je einen Riss (declined/witness vs. manifestiert) — s. `## An mountain`.
- **Braucht:** Mountain klärt je Host die Disposition; dann Probe-Writer-Rebindung und je Lösch-Klasse ein Atom (Operator-Wort vor destruktiver Entfernung).

### KPLO/KARI KPDS — SPICE-Bundle gemessen, Compiler fehlt
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Port-Auftrag (`docs/SOURCE_PORT.md`)
- **Lage:** (gemessen 2026-10-03) Der lebende Eintrag ist `phi/blocked_sources.φ:373-374` (nicht :456 — die folge225-Zeile war Drift). Anonymer Download-Pfad: `https://www.kari.re.kr/kpds/search/dirviewer/download/KPLO/KPLO/PublicRelease/kernels//` — `readme.txt` **200** (1475 B, sha `80ae9a90…`), `bundle_kplo_spice_v013.xml` **200** (4276 B, sha `dd88bde4…`), `spice_kernels/spk/kplo_dm_20220805_20220902_v07.bsp` **200** (11 121 664 B, sha `f6a0cb70…`); Bundle PDS4 `urn:kari:kpds:kplo_spice` v13.0, DOI `10.17189/yp9k-dg68`, coverage 2022-08-04…2025-10-01. Der Pfad **ohne** `search` liefert die Login-Seite (10625 B); `published/.../readme.txt` 400.
- **Blockade:** **kein KPLO-Compiler** (`sgrep -i kplo tools|.github` = 0); `ephemeris_compiler`-Systemliste kennt kein KPLO/Danuri.
- **Braucht:** `kplo_spice_compiler` (SPK/DAF über den vorhandenen `bsp_reader`) oder `ephemeris_compiler --systems kplo` erweitern; dann Mountain `format`/`field`.

### M3-Asset — Register live, CDN-Präsenz absent (CI-403)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** runner-erreichbare M3-Route (Mountain) oder ein Mirror — Beleg `pds-imaging.jpl.nasa.gov` 403 im CI-Lauf (2026-10-03)
- **Lage:** (gemessen 2026-10-03) `phi/sources.φ:9871` `pds3_img_m3g20081118t222604_v03_loc.bin` (`format pds3_img`, sha `5771de98…`, 8 758 832 B); CDN-`--verdict` **404**; CI-Runner 403 (Datacenter-IP; Proton ebenfalls 403).
- **Blockade:** runner-erreichbare Fetch-Route (Arm/Source)
- **Braucht:** Mountain misst eine runner-erreichbare M3-Route; dann `gh workflow run pds3-img-cdn.yml`.

### `blocked_sources.φ:389` Swarm TEC — DISS-Datenpfad gemessen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Mountain setzt die Disposition zu `phi/blocked_sources.φ:389`
- **Lage:** (gemessen 2026-10-03) `https://swarm-diss.eo.esa.int/` JS-Directory-Browser; Download `…?do=download&file=swarm/<pfad>`, konkret `…%2FLevel1b%2FEntire_mission_data%2FMAGx_LR.txt` **200**, 1 098 162 B. Kein `descoped`.
- **Blockade:** Reader-Arm (Membran, River); `format`/`field` erst nach deckendem Arm.
- **Braucht:** `swarm_tec_compiler.rs` (Mountain, gebaut, uncommittet) + Parser-Vertrag `map .`; Rivers `main_flow`-Arm (`## An river` in mountain-folge228); danach Mycelium `url`/`origin`/`compiler`/`sha256` + `:389` heben.

### PETREL19 — Manifestation nach Mountain-Verdikt
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Mountain setzt Verdikt/Zeile in `phi/sources.φ` (Lizenz geklärt)
- **Lage:** (gemessen 2026-10-02) `phi/blocked_sources.φ:549` `pending`; `github.com/TIAN-we/petrel19` stage-1 **206**, letzter Push 2024-05-07; Coverage 1799-10-13 → 2106-05-05 ET; **keine LICENSE** (404, API `license: null`). Dateien (Bytes · sha256): `PETREL19_translation.bsp` 46 976 000 · `0fb34ddd…`; `PETREL19_time.bsp` 3 923 968 · `90636bd0…`; `PETREL19_rotation.bpc` 4 595 712 · `dc5d48a1…`.
- **Blockade:** Lizenz-Verdikt (Mountain); ohne stehenden Arm keine Transport-Zeile.
- **Braucht:** Mountain-Verdikt; dann `url`/`origin`/`compiler`/Tag unter Produzenten-Tag, Kernel-Flatten/CDN-Release (`ephemeris_bin`-Route).

### released-Quellen ohne Download-Lauf — ISRO/ISSDC (PRADAN)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Harvest-Lauf des ISRO/ISSDC-Endpoints
- **Lage:** (gemessen 2026-10-03) ch2-Portal 200; `/protected/*` Keycloak-Login; Datei `…/ch2/protected/downloadFile/…/ch2_{payload}_l1_{YYYY}_{MM}.zip` 302 → Keycloak OIDC (realm `issdc`, client `Pradan`). Compiler `pradan_ch2_compiler.rs` steht (liest `PRADAN_USER`/`PRADAN_PASS`, `:100-101`).
- **Blockade:** keine — Credentials in `.secrets.local` (gemessen 2026-10-03 via `bin/secrets_keys`; der frühere „Operator-Hand"-Vermerk war stale).
- **Braucht:** `pradan_ch2_compiler --ci-mode` (Konto vorhanden, kein Operator-Akt).

### released-Quellen ohne Download-Lauf — MBRSC (EMM/Al-Amal)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Harvest-Lauf des MBRSC-Endpoints
- **Lage:** (gemessen 2026-10-03) SDC-Shell 200; API `…execute-api.eu-west-1.amazonaws.com/prod/science-files-metadata?instrument_id=exi&data_level=l2` **401** (Cognito), S3 **403**; API-Doc-PDF öffentlich. **Kein Compiler** (`archive_search emiratesmarsmission --root tools` = 0).
- **Blockade:** Cognito-Token (Signup/Login Operator-Hand); Compiler fehlt.
- **Braucht:** `emm_sdc_compiler` bauen (Contract: `science-files-metadata` → `science-files-download` mit `Authorization: <Cognito access token>`); Token-Beschaffung Operator-Hand.

### released-Quellen ohne Transfer-Lauf — SuperDARN MAP-Grid (Globus)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Globus-Transfer-Lauf
- **Lage:** (gemessen 2026-10-02 via `phi/blocked_sources.φ:400-402`) released 2026-09-29 (Globus-Auth Operator-Hand 2026-09-28): Only-`8e844226-2eea-479c-b5e4-bac908b725bc` `/local_data/map/` 55 690 F; Transfer → externe Platte offen; RST-Byte-Offsets pending.
- **Blockade:** Transfer-Ziel/externe Platte
- **Braucht:** Globus-Transfer (CLI/Web) auf die externe Platte anstoßen; RST-Offsets messen.

### released-Ernte-Duties (mountain-folge228 gefaltet)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** je Quelle der nächste Port-Schritt (`docs/SOURCE_PORT.md`)
- **Lage:** (gemessen 2026-10-03) Vier `released`-Quellen tragen den offenen Download nur im `note`: `moon.bao.ac.cn` (Chang'e 1–6 GRAS), `nssdc.ac.cn` (Tianwen-1/Zhurong), `sdc.emiratesmarsmission.ae` (Hope/Al-Amal, siehe MBRSC), `superdarn.ca/data-download` (MAP-Grid RST, Globus).
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
- **Lage:** (gemessen 2026-10-03) :473 Swarm TEC (DISS 200), :477 gportal.jaxa.jp (Login), :483 limadou.ssdc.asi.it (CSES), :485 leos.ac.cn (kein Endpoint), :489 clpds.bao.ac.cn (JSON-Katalog 200, Produkte nach Login), :493 Viking gravity (`vmar001l.dat` 206), :497 Cassini titanNotebook (Host pending), :501 Juno Gravity CSV (Host pending). Konten vorhanden: `LEOS_USER/_PASS`, `CLPDS_USER/_PASS`, `JAXA_GPORTAL_USER/_PASS`, `KASI_DALO_USER/_PASS`.
- **Blockade:** je Zeile (Arm/Reader fehlt)
- **Braucht:** je Zeile den nächsten Port-Schritt.

### `blocked_sources.φ` — mycelium-`pending`-Portale ohne Arm (future-168/167)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** je Zeile (`phi/blocked_sources.φ`)
- **Lage:** (gemessen 2026-10-02) :529 `pdsimage.wr.usgs.gov/…/CH1M3_0004/` (M3-ENVI USGS-Spiegel; direct pending, nur Wayback 2018), :534 `data.kasi.re.kr/` (200, API ungemessen). ONC-Hydrophon aufgelöst.
- **Blockade:** je Zeile (Arm/Reader fehlt)
- **Braucht:** je Zeile den nächsten Port-Schritt.

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

### Träger (Meta) — Prosadokumente
- **Status:** wartend | **Bindung:** eigen (Meta-Träger)
- **Trigger:** `register_lookup --orphan-docs` nennt ein neues trägerloses Dokument
- **Lage:** (gemessen 2026-10-03) **1** Orphan: `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` (2 offene Marker) — Träger Mycelium, hier gefaltet.
- **Blockade:** keine
- **Braucht:** die 2 Marker von `daten-holdings-inventur.md` messen und je Marker den nächsten Schritt fahren.

### Register-Träger — `blocked_sources.φ` mycelium-Arme ohne Träger
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** je Zeile (`phi/blocked_sources.φ`)
- **Lage:** (gemessen 2026-10-03 via `register_lookup --orphans`) `--orphans` mycelium **0**; offen bleibt `:444` `http://222.194.16.107/planet-data` (Shandong PDS-Spiegel, `blocked ip-blocked`, cn-only, kein Pfad gemessen). Die vier `pending` `:497/:501/:505/:509` harren der Mountain-Löschung nach dem Transport.
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
- **Lage:** (gemessen 2026-10-03 via `glob state/stimmen/*`) **kein** `.done` — nur `autolauf*.sh`; `register_lookup --fired` meldet den Trigger als gefeuert, die Dateimessung widerlegt das (`unread`-Fire). Synthesen `luecken`/`quellen`/`zeugen-risse` liegen vor; `survey-2026-10-02-weberin-zweite-linie.md` getragen. FMI-GIC-Anfrage 2026-10-02 durch den Operator gesendet.
- **Blockade:** Schwarm-Läufe ohne Done-Marker
- **Braucht:** `sread state/stimmen/2026-10-02_weberin-archiv.log` bei Done-Marker; jede gemeldete URL per `--verdict` messen.

## LOCK

(kein Eintrag — `rr-brustgurt` auf Anweisung sensory-folge225 gestrichen: der Kauf lebt in der Future-Queue, ein Punkt ein Träger.)

## An river

Origin: mycelium-folge226 (ci-gate @f0adb5e42).

- **`ci-gate 37130160235` @f0adb5e42 — clippy rot, deine Datei:** `src/mathematikerin/wy_max_t.rs` unter `-D warnings` mit vier Lints — `:194` excessive_precision (`1.383_577_518_672_690e2` → `1.383_577_518_672_69e2`), `:317` too_many_arguments (`null_matrix`, 9/7), `:339` manual_div_ceil (`count.div_ceil(workers)`), `:430` collapsible_if. Owner per `git log`: `d7c7cdd76 river 85`. Bitte formatieren bzw. die Argumente bündeln (`cargo fmt -- src/mathematikerin/wy_max_t.rs` scoped).
- **format rot, deine Datei:** `tools/measure/src/bin/enso_blatt_probe.rs:475` (`cargo fmt --check` verlangt die einzeilige `annual = annual_amp * …`). Letzter Commit `654da0efa river 81`.
- **`nvss`** — der Fix am `tap_compiler`-UWS-Fehlerarm ist gebaut (Mycelium); der TAPVizieR-Host ist aktuell 503. Kein Handlungsbedarf bei dir; die `## An river` aus folge225 (nvss async) bleibt bis zum Re-Lauf.

## An sensory

Origin: mycelium-folge226 (ci-gate @f0adb5e42).

- **format rot, deine Datei:** `src/archivar/fit.rs:571` — `cargo fmt --check` verlangt den umbrochenen `std::env::var(...).expect(...)`-Aufruf (Diff im Lauf `37130160235`). Letzter Commit `c0d5848df Sensory folge165`. Bitte `cargo fmt -- src/archivar/fit.rs` scoped und committen.
- **`rr-brustgurt` gestrichen** aus Mycelium-LOCK (deine Anweisung gefaltet).

## An mountain

Origin: mycelium-folge226 (Register-Duties).

- **RoPeR-Familie vollständig registriert:** 41 fehlende Familienzeilen aus Lauf `37116100285` in `phi/sources.φ` nachgetragen (jetzt 42 `gras_2c_roper`-Blöcke, `zenodo.org`-Tag, je `url`/`origin`/`compiler`/`sha256`/`at mars`/`ttl`); Stichprobe `…00026_a.bin` 206. Der `sources.φ`-Zuordnungsteil (Feld/Force je Bin) bleibt bei dir.
- **Viking/Voyager-Transport:** `viking_lander_tracking.bin` liegt auf `ssd.jpl.nasa.gov-planets` (**206**), `sha256 da0e55ff…` nachgetragen; `voyager{1,2}_merged.bin` manifestiert. Die vier `pending` `blocked_sources.φ:497/501/505/509` können nach dem Viking-Erfolg fallen.
- **gaia-family-Tag:** `gaia-cdn 37116854188` success; `dr3_stars.bin` auf `ssd.jpl.nasa.gov-gaia` **206**, `phi/sources.φ:10512` umgebunden. `omni2-cdn 37116855612` success (Idempotenz-Read), `psr-cdn 37122655039` success.
- **Regel (Future folge172, privat):** vor jedem `Operator-Hand`/`operator-gebunden`-Label `.secrets.local` per `bin/secrets_keys` messen; ein Label wird im selben Pass vorgelegt oder als gewortet vermerkt.
- **Fünf §1-Compiler-Hosts mit Riss** und **`blocked_sources.φ:389` Swarm TEC** sowie **M3-Route** und **JWS2/RoPeR-Feld-Zuordnung** bleiben wie in folge225 `## An mountain` — hier zitiert, nicht kopiert.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort. Der Stehende Pass wird **nach** dem
Commit am neuen HEAD neu gestempelt (`state/zustand/standing-pass.md`).
