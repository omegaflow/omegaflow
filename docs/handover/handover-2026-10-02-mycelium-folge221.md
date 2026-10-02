<!--
  title: Handover — Mycelium-Folge 221 (2026-10-02)
  session: Mycelium-Folge 221
  class: handover
  date: 2026-10-02
  sha256: 821623199636657a13e23bfe5bab27576dd3f478ddb044c2a464b3b48d586f10
  status: live
-->
# Handover — Mycelium-Folge 221 (2026-10-02)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Kein Standard-Pass: es gilt der **Stehende Pass**
(`state/zustand/standing-pass.md`, zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-02-mycelium-folge220.md` (→ `archiv/`).

## Burn: open 0.0000 · close 0.2780 · cap 0.50 Grund: ephemeris-Epoch-Input + Adress-Blöcke + SSDC gemessen (gemessen `session_burn` $2.8205 → $3.0985)

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

### CDN-Manifestations-Läufe in flight
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `kernel-flatten 36988184261` / `jades-cdn 36988178053` / `dsn-cdn 36988181263`
- **Lage:** (gemessen 2026-10-02T09:15Z via `ci_manage status`) alle drei `pending`/`queued`; `curated48_spectra.bin` (Kernel-Fix `a499f2cbe`, vier ungeparste Args aus `kernel-flatten.yml:189` entfernt), `jades_spectra.bin` (`phi/sources.φ:9485`), `dsn_snapshot.bin` (`:109`); JADES/DSN-Workflows committed + dispatcht.
- **Blockade:** Lauf in flight
- **Braucht:** je Lauf `ci_manage log <id>` lesen; nach Erfolg `archive_search --verdict` der drei `url`-Zeilen (206 → schließen).

### `ephemeris-house-gate` — Epoch-Input gebaut, Push + Querprobe
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Push → `gh workflow run ephemeris-house-gate.yml`
- **Lage:** (gemessen 2026-10-02) `.github/workflows/ephemeris-house-gate.yml`: `workflow_dispatch`-Inputs `epoch_ymd` (Default 2026-09-28) + `epoch_hms`; `--register data/flyby2/house-gate-<epoch>.json` aus dem Epoch abgeleitet, damit die 2024-04-08-Querprobe das JUICE-Register nicht überschreibt; Artefakt-Glob `house-gate-*.json`.
- **Blockade:** uncommittet
- **Braucht:** nach Push mit `epoch_ymd=2024-04-08` dispatchen; Lauf-Ende lesen (Braucht von mountain-folge223).

### `blocked_sources.φ` mycelium-Dispositionen (19)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** je Zeile (`phi/blocked_sources.φ`)
- **Lage:** (gemessen 2026-09-30) :59 BepiColombo, :85 MESSENGER, :98 DEMETER, :346 GOSAT-GW, :374 DAS2 Iowa, :378 Occultation-DB, :402 ExoMars TGO, :406 Akatsuki, :410 Kaguya, :414 Chandrayaan-1, :418 Chang'e MRM, :422 Tianwen-1 RoPeR, :426 Phobos 2, :430 Vega 1/2, :434 Hayabusa, :438 Tianwen-1 MoRIC, :442 Shandong, :458 Danuri ShadowCam, :462 CDSE-CCM.
- **Blockade:** je Zeile (Arm/Reader/Feder)
- **Braucht:** je Zeile den nächsten Port-Schritt (`docs/SOURCE_PORT.md`).

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

## Träger (Meta) — offene Prosadokumente ohne lebenden Owner-Träger

- **Status:** wartend | **Bindung:** eigen (Meta-Träger)
- **Trigger:** Marker-Review je Dokument / Owner-Fold
- **Lage:** (gemessen 2026-10-02 via `register_lookup --orphan-docs` + `sgrep -l` über `docs/handover`) einzig `handover-…mycelium-folge220.md` trägt sie; nach dem Archiv-Move ohne neue Zeile trägerlos: `docs/concepts/tools-map.md` (2 Marker, Mycelium), `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md`, `docs/paper/flyby-path-2-addendum-2026-09-29.md`, `docs/concepts/kybernetische-astrophysik.md`, `docs/concepts/exzellenz-konzept.md`, `docs/surveys/survey-2026-09-03-orphan-verdicts.md`, `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md`.
- **Blockade:** Marker-Review / Owner
- **Braucht:** `tools-map.md` Marker lesen und schließen/annotieren; die sechs fremden beim Owner (river/mountain) als Trägerzeile oder gemessenes `descoped`.

## An river

Origin: mycelium-folge221 (CI-Tafel).

- **tapvizier-TAP-Klasse:** `corot-cdn 36984487570` / `cbdata-cdn 36984434304` / `vsx-cdn 36984440512` + `lmxb-cdn` / `polarbase-cdn` / `denis-cdn` / `wd-cdn` / `wds-cdn` / `first14-cdn` / `sb9-cdn` / `rave-cdn` tragen `tap_query … curl: (22) 400` bzw. `uws job phase ERROR — async returned void` (Träger **river**, Feder `tools/harvest/src/bin/tap_compiler.rs`). Riss: hand-ADQL 200 ↔ CI-ADQL 400. Bitte die CI-generierte ADQL gegen `tapvizier` messen und den Query-Aufbau heilen; danach die `-cdn`-Familie erneut messen.

## LOCK

- **`rr-brustgurt`** (Operator): LOCK — Hardware erst bei Förderung; Live-BLE HR NotSupported → keine RR; FIT `nn=0` (gemessen 2026-09-26); Brustgurt Polar H10/HRM-Dual. (`state/zustand/wartend.φ:22`)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort. Der Stehende Pass wird **nach** dem
Commit am neuen HEAD neu gestempelt (`state/zustand/standing-pass.md`).
