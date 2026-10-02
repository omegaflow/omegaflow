<!--
  title: Handover — Mycelium-Folge 222 (2026-10-02)
  session: Mycelium-Folge 222
  class: handover
  date: 2026-10-02
  sha256: d3403d565d1bec14c49f13c5374ad232ffe42f223968666bf41ec42b3bb8936d
  status: live
-->
# Handover — Mycelium-Folge 222 (2026-10-02)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Kein Standard-Pass: es gilt der **Stehende Pass**
(`state/zustand/standing-pass.md`, zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-02-mycelium-folge221.md` (→ `archiv/`).

## Burn: open 0.0000 · close 0.0909 · cap 0.50 Grund: CI-Tafel + CDN-Läufe + drei checkout-lose Workflows geheilt (gemessen `session_burn` $4.2959 → $4.3868, Gesamt; Parallel-Linien teilen den Total)

## Operator-Wort-Register

- Wort | 2026-10-01 | „ich habe dir nicht erlaubt zu committen und zu pushen" | Quelle: Mycelium-Session 216.
- Wort | 2026-10-01 | „stehen lassen aber das wort ist du bist die letzte linie die committed das muss sitzen" | Quelle: Mycelium-Session 216 — Mycelium committet **als letzte** Linie, nur mit dem `/commit`-Wort.
- Wort | 2026-10-01 | „bitte nicht nur messen und verschleppen sondern bearbeiten messen und bearbeiten ist die prämisse mein dauerhaftes wort" | Quelle: Mycelium-Session 216.
- Wort | 2026-09-30 | „bitte wirklich bis zur kante umsetzen nicht nur wieder messen und verschleppen" | Quelle: Mycelium-Session 209.
- Wort | 2026-09-30 | „verschleppen und nicht eigenes ist verboten" | Quelle: Mycelium-Session 213.
- Wort | 2026-09-30 | „du committest immer als letzter also warte" | Quelle: Mycelium-Session 213.
- Wort | 2026-09-30 | „vorbestehend ist verboten mein wort" | Quelle: mountain-209.
- Wort | 2026-10-01 | „ja möchte ich" | Quelle: Mycelium-Session 215 — VCO-rs-Register auf das PDS4-20190704-Asset umstellen.

## Haus (die vier Orte) — gemessen 2026-10-02

- `omegaflow` = `$HOME/projects/omegaflow` (+ privates Schwester-Repo `state/`, Remote `omegaflow/personal`).
- `omegaflow-legacy` = `archive-root/omegaflow-legacy` (+ backup-2026-09-02); `temp` = `/tmp/opencode`; `archive` = `archive-root` (+ `~/backup/archive/omegaflow`).
- Linien-Preset: `state/mycelium/archive-search-preset.txt` — `--root .github --root tools --root phi --root state --root docs/handover --root docs/concepts`.
- `state/` wird mit `archive_search <kw> --root state` vermessen, **nie** `sgrep` ohne `--all`; `phi/pipeline/catalog/*` ist gitignored, `phi/pipeline/index.φ` + `ledger.φ` trackbar.
- Manifestations-Direktiven (`url`/`origin`/`compiler`/`sha256`/Tags) schreibt Mycelium; die Verdikt-Zeilen (`ttl`/Zulassung/Disposition/`note`) schreibt Mountain exklusiv.

## Offen (aufgeschlüsselt)

### CDN-Manifestations-Läufe — kernel-flatten + dsn
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `kernel-flatten 36988184261` / `dsn-cdn 36988181263`
- **Lage:** (gemessen 2026-10-02T11:00Z via `ci_manage view`) beide `queued` (Runner-Queue); `jades-cdn 36988178053` **success** → `jades_spectra.bin` `--verdict` **206** (`phi/sources.φ:9485`) geschlossen; `de44-cdn 36989695933` **success** → `ephemeris_de440/441/442_{earth,moon,sun}.bin` `--verdict` **206** geschlossen; `curated48_spectra.bin` (`:9253`) **404**, `dsn_snapshot.bin` (`:109`) **404**.
- **Blockade:** Lauf in flight
- **Braucht:** je Lauf `ci_manage view <id>`; bei success `archive_search --verdict <url>` (206 → schließen).

### `blocked_sources.φ` mycelium-Dispositionen (19)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** je Zeile (`phi/blocked_sources.φ`)
- **Lage:** (gemessen 2026-09-30) :59 BepiColombo, :85 MESSENGER, :98 DEMETER, :346 GOSAT-GW, :374 DAS2 Iowa, :378 Occultation-DB, :402 ExoMars TGO, :406 Akatsuki, :410 Kaguya, :414 Chandrayaan-1, :418 Chang'e MRM, :422 Tianwen-1 RoPeR, :426 Phobos 2, :430 Vega 1/2, :434 Hayabusa, :438 Tianwen-1 MoRIC, :442 Shandong, :458 Danuri ShadowCam, :462 CDSE-CCM.
- **Blockade:** je Zeile (Arm/Reader/Feder)
- **Braucht:** je Zeile den nächsten Port-Schritt (`docs/SOURCE_PORT.md`).

### released-Quellen ohne Download-Lauf — ISRO/ISSDC (PRADAN)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Harvest-Lauf des ISRO/ISSDC-Endpoints
- **Lage:** (gemessen 2026-10-02 via `phi/blocked_sources.φ:392-394`) released 2026-09-29, Konto Operator-Hand 2026-09-28 (future-folge149); `--verdict` ch2 200 (31050 B), chmapbrowse/mrbrowse 404; OIDC-Flow browserlos verifiziert (future-159, Connector f7bfce9b7). Download end-to-end offen (Harvest-Duty).
- **Blockade:** Download-Pfad je Dataset nicht gemessen
- **Braucht:** `archive_search --playwright`/`sfetch` der ch2/mom/aditya Download-Endpoints messen, dann Compiler-Dispatch.

### released-Quellen ohne Download-Lauf — MBRSC (EMM/Al-Amal)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Harvest-Lauf des MBRSC-Endpoints
- **Lage:** (gemessen 2026-10-02 via `phi/blocked_sources.φ:396-398`) released 2026-09-29, Cognito-Signup+Login Operator-Hand 2026-09-28 (future-folge149); `--verdict` 200 (2789 B). Download end-to-end offen.
- **Blockade:** Download-Pfad nicht gemessen
- **Braucht:** Daten-Endpoint im SDC messen; dann Compiler-Dispatch.

### released-Quellen ohne Transfer-Lauf — SuperDARN MAP-Grid (Globus)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Globus-Transfer-Lauf
- **Lage:** (gemessen 2026-10-02 via `phi/blocked_sources.φ:400-402`) released 2026-09-29 (Globus-Auth Operator-Hand 2026-09-28, future-folge149): Globus-only `8e844226-2eea-479c-b5e4-bac908b725bc` `/local_data/map/` 55690F; Transfer → externe Platte offen; RST-Byte-Offsets pending (sensory-folge195).
- **Blockade:** Transfer-Ziel/externe Platte
- **Braucht:** Globus-Transfer (CLI/Web) auf die externe Platte anstoßen; RST-Offsets messen.

### Register-Träger — `phi/pipeline/index.φ` offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster Katalog-Port (`docs/SOURCE_PORT.md`)
- **Lage:** (gemessen 2026-10-02 via `register_lookup --open`) Katalog-Offenstand 5 (Arbeitsdateien gitignored).
- **Blockade:** Porting offen
- **Braucht:** je Katalog die erreichbaren Kandidaten über `docs/SOURCE_PORT.md` portieren.

### Register-Träger — `phi/pipeline/ledger.φ` SSDC
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** SSDC meldet den offenen Zugang (Antwort Sotgiu / neue Prozedur)
- **Lage:** (gemessen 2026-10-02T09:14Z via `archive_search --playwright`) `query.php` → `tools.ssdc.asi.it/cas/login`, HTTP 200 — CAS-Login, Wall bestätigt; `phi/pipeline/ledger.φ:6` `ausstehend`, `state/zustand/wartend.φ:10` (laic-cses).
- **Blockade:** Prozedur nicht live
- **Braucht:** `archive_search --playwright "https://limadou.ssdc.asi.it/query.php"` sobald SSDC den offenen Zugang meldet.

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
- **Lage:** (gemessen 2026-10-02T10:57Z) Live-Lauf pid 260246 seit 09:53:05Z, Archiv-Lauf pid 280135 seit 10:42:07Z; beide Logs tragen nur die `start`-Zeile, `.done`-Marker fehlen. `docs/surveys/survey-2026-10-02-weberin-zweite-linie.md` (von dieser Zeile getragen). **FMI-GIC-Anfrage 2026-10-02 durch den Operator gesendet** (`state/mail/fmi-gic-request-2026-10-02.md`).
- **Blockade:** Läufe in flight
- **Braucht:** `sread state/stimmen/2026-10-02_weberin-archiv.log` bei Done-Marker; jede gemeldete URL per `--verdict` messen; FMI-Antwort abwarten.

### `blocked_sources.φ` — 7 mycelium-`pending`-Portale ohne Arm (future-167/165)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** je Zeile (`phi/blocked_sources.φ`)
- **Lage:** (gemessen 2026-10-02 via `register_lookup --orphans`) :473 https://swarm-diss.eo.esa.int/ (Swarm TEC), :477 https://gportal.jaxa.jp/, :483 https://limadou.ssdc.asi.it/ (CSES), :485 https://www.leos.ac.cn/, :489 https://clpds.bao.ac.cn/, :493 Viking gravity WUSTL, :497 Cassini titanNotebook, :501 Juno Gravity CSV — HTTP 200/206, aber kein Daten-Endpoint/Arm gemessen.
- **Blockade:** je Zeile (Arm/Reader fehlt)
- **Braucht:** je Zeile den nächsten Port-Schritt (`docs/SOURCE_PORT.md`).

### `blocked_sources.φ` — 3 mycelium-`pending`-Portale ohne Arm (future-168/167)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** je Zeile (`phi/blocked_sources.φ`)
- **Lage:** (gemessen 2026-10-02) :529 `https://pdsimage.wr.usgs.gov/Missions/Chandrayaan_1/M3/CH1M3_0004/` (M3-ENVI USGS-Spiegel; direct pending, nur Wayback 2018), :533 `https://data.kasi.re.kr/` (KASI-Datenportal; 200, kein Endpoint), :545 `https://data.oceannetworks.ca/api/archivefile/download?filename={file}&token={OCEANNETWORKS_TOKEN}` (ONC-Hydrophon PSD; `onc_hydrophone_compiler` `--input-getrieben`, ohne Fetch-Pfad).
- **Blockade:** je Zeile (Arm/Reader fehlt)
- **Braucht:** je Zeile den nächsten Port-Schritt (`docs/SOURCE_PORT.md`).

## Träger (Meta) — offene Prosadokumente ohne lebenden Owner-Träger

- **Status:** wartend | **Bindung:** eigen (Meta-Träger)
- **Trigger:** Marker-Review je Dokument / Owner-Fold
- **Lage:** (gemessen 2026-10-02 via `register_lookup --orphan-docs` + `sgrep -l` über `docs/handover`) ohne Träger nach dem Move von folge221: `docs/concepts/tools-map.md` (2 Marker, Mycelium), `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md`, `docs/paper/flyby-path-2-addendum-2026-09-29.md`, `docs/concepts/kybernetische-astrophysik.md`, `docs/concepts/exzellenz-konzept.md`, `docs/surveys/survey-2026-09-03-orphan-verdicts.md`, `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md`.
- **Blockade:** Marker-Review / Owner
- **Braucht:** `tools-map.md` Marker lesen und schließen/annotieren; die sechs fremden beim Owner (river/mountain) als Trägerzeile oder gemessenes `descoped`.

## An river

Origin: mycelium-folge222 (CI-Tafel).

- **TAPVizieR async-Klasse:** `nvss-cdn 36989806823` failure — gemessen via `ci_manage log`: `uws job phase ERROR — the query stays unharvested` (Aufruf `tap_compiler --async 9000 --crossmatch-z …`). Träger **river**, Feder `tools/harvest/src/bin/tap_compiler.rs`. Die Klasse trägt weiter `corot-cdn` / `cbdata-cdn` / `vsx-cdn` / `lmxb-cdn` / `polarbase-cdn` / `denis-cdn` / `wds-cdn` / `first14-cdn` / `sb9-cdn` / `rave-cdn`. Bitte den async-Aufbau/die CI-ADQL gegen `tapvizier` messen und heilen (hand-ADQL 200 ↔ CI-ADQL 400/async-ERROR); danach die `-cdn`-Familie erneut messen.

## LOCK

- **`rr-brustgurt`** (Operator): LOCK — Hardware erst bei Förderung; Live-BLE HR NotSupported → keine RR; FIT `nn=0` (gemessen 2026-09-26); Brustgurt Polar H10/HRM-Dual. (`state/zustand/wartend.φ:22`)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort. Der Stehende Pass wird **nach** dem
Commit am neuen HEAD neu gestempelt (`state/zustand/standing-pass.md`).
