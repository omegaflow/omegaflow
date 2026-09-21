<!--
  title: Handover — Ernte-Folge 131 (Stand 2026-09-21)
  session: Ernte-Folge 131
  class: handover
  date: 2026-09-21
  sha256: a1fa318fe236dc68334ea9235d5df75dfa6c72971e524e5871f700d5055de381
  status: live
-->
# Handover — Ernte-Folge 131 (2026-09-21)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird pfad-begrenzt, fremde
uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden parallel von Agenten
abgearbeitet. Jeder offene Punkt trägt seinen nächsten Schritt in derselben Zeile;
Status-Tag (`wartend` | `operator-gebunden` | `blockiert` | `termin`).
Wartestellungen sind kein Auswahlpunkt.

## Stehender Pass (gemessen 2026-09-21, Folge 131)

- **HEAD** `61d3049f` beim Start; Arbeitsbaum trägt Fremdarbeit (entscheid
  `folge78` + Move `folge77`, `post.md`, `external-state.md`; forschung `folge134`
  + Move `folge133`, `KERNEL_INDEX.md`, `free_model_bench.rs`) — nicht angetastet.
- **Postfach** — kein neues `An ernte`; `post.md` trägt nur `An entscheid` (Riss 4).
- **CI** — Massen-`-cdn`-Dispatch 10:14–10:16; viele attempt-1-Failures
  (tess/avo/nvss/sb9/vsx/chandra-csc/polarbase/merlin/gcvs/frb-a279/wd/bzcat5/
  first14/sncat/lmxb/exoplanets/wds/swiftgrb/pastel/magnetar/tevcat/pangaea);
  `hfrnet-cdn 35581429287` success; `quaoar-occlt-cdn 35587817913` queued;
  `ps1-cdn` neu pending; `demeter-cdn 35567568429` queued seit 06:14.
- **Zustand** — `external-state.md` von entscheid fortgeschrieben (fremd).

## Offen (aufgeschlüsselt)

### CDN-Tag-Migration — Re-Manifestation + Dispatch
- **Status:** offen | **Bindung:** eigen
- **Lage:** Alle 148 `sources.φ`-`url`-Zeilen vom gekappten Tag
  `releases/download/ssd.jpl.nasa.gov/` auf Familien-Tags umgestellt
  (`sgrep -c` = 0); Compiler `eve`/`ned`/`de`/`neptune_ephemeris` auf
  `upload_release(<familie>)` migriert; `cargo check` 0/0. Geänderte Workflows
  dispatched: eve `35590913080`, ned `35590915445`, de44 `35590918321`,
  neptune-de440s `35590921109`.
- **Blockade:** neue Workflows (babamul/ia2/fai-kz) sind vor dem Push nicht
  dispatchbar (HTTP 404 auf default branch).
- **Braucht:** nach Push `gh workflow run {babamul,ia2,fai-kz}-cdn.yml`; die
  migrierten Tags brauchen die Re-Manifestation (betroffene `-cdn.yml` prüfen,
  Vormittags-Failures zuordnen).

### neptune_ephemeris_compiler — Registerzeile fehlt
- **Status:** offen | **Bindung:** eigen
- **Lage:** lädt nach `ssd.jpl.nasa.gov-neptune`, aber `phi/sources.φ` trägt
  keine `url`-Zeile für diesen Compiler (gemessen: `sgrep "neptune_ephemeris_compiler"` = 0).
- **Blockade:** keine.
- **Braucht:** Registerzeile anlegen (Registraturpflicht) oder den Arm als
  nicht-manifestiert benennen.

### Babamul / IA2 — Manifestation
- **Status:** wartend | **Bindung:** termin (Push)
- **Lage:** Workflows `babamul-cdn.yml`/`ia2-cdn.yml` gebaut (Muster
  `hfrnet-cdn.yml`, Tags `babamul.caltech.edu`/`ia2-tap.oats.inaf.it`);
  `sources.φ`-Blöcke eingetragen (pending sha256); Routen 200 (2026-09-21).
- **Blockade:** Push.
- **Braucht:** nach Push `gh workflow run {babamul,ia2}-cdn.yml`; bei success
  sha256 → `sources.φ` + `ledger.φ` → `kompiliert`.

### TAP fai.kz — Manifestation + Feld-Klassifikation
- **Status:** wartend | **Bindung:** termin (Push)
- **Lage:** `src/archivar/fai_kz.rs` (Format FAI1, 57 B/Record) +
  `tools/harvest/src/bin/fai_kz_compiler.rs` gebaut (`cargo check` 0/0);
  `sources.φ`-Block eingetragen; `ledger.φ:10` bleibt `verifiziert` (Compiler
  gebaut, Asset pending); obscore 5048 Zeilen (image 3003/spectrum 2045).
- **Blockade:** Push; Feld-Klassifikation (obscore trägt keinen skalaren Fluss).
- **Braucht:** nach Push `gh workflow run fai-kz-cdn.yml` → sha256 → `kompiliert`;
  Feld-/force-/τ-Zeilen sind ein Oszillator-Gate-Entscheid (`pending`).

### Quaoar Sternbedeckung — Manifestation
- **Status:** wartend | **Bindung:** termin (Run `35587817913`)
- **Lage:** Arm/Compiler/Workflow stehen; `sources.φ:8706`-Block pending sha256;
  `_chi2_`-Chord (Astrometrie) unkompiliert.
- **Blockade:** Lauf-Abschluss.
- **Braucht:** `ci_manage view 35587817913`; bei success sha256 →
  `sources.φ:8706`/`ledger.φ:32` → `kompiliert`.

### PS1 Order-10-Final
- **Status:** wartend | **Bindung:** termin
- **Lage:** Run `35569486280` success, final combine nicht erreicht; Asset
  `ps1_dr2_coverage.fp01` absent; neuer Lauf pending.
- **Blockade:** Lauf.
- **Braucht:** `gh workflow run ps1-cdn.yml`; bei final → `footprints.φ:19`.

### DEMETER (CDN + ISL)
- **Status:** wartend | **Bindung:** termin (Run `35567568429`)
- **Lage:** CDN-Lauf queued (seit 06:14); ISL-Route account-gated (rs-order 403
  ohne Token; CDPP nennt E-Mail-Konto als Voraussetzung).
- **Blockade:** Lauf-Abschluss / Konto.
- **Braucht:** `ci_manage view 35567568429`; ISL-Konto (`operator-gebunden` →
  entscheid).

### SSDC Limadou
- **Status:** wartend | **Bindung:** termin (CSES-02-Umbau)
- **Lage:** `ledger.φ:26` — PI Sotgiu 2026-09-16 „wait a few weeks"; kein
  Follow-up fällig.
- **Blockade:** PI-Website.
- **Braucht:** neue Anleitung auf dem Limadou-Portal.

### EPA RadNet Koordinaten
- **Status:** wartend | **Bindung:** termin (FRS)
- **Lage:** `blocked_sources.φ:74` — ERM_LOCATION nur city/state; FRS
  `ofmpub get_facilities` 503 (Maintenance); AGOL nur 140 Luftmonitore.
- **Blockade:** FRS-Maintenance.
- **Braucht:** FRS-Retry `pgm_sys_acrnm RadNet`.

### EMODnet maxTime
- **Status:** wartend | **Bindung:** termin (Host)
- **Lage:** `ledger.φ:18` — `erddap.emodnet-physics.eu` HTTP 0 (ERR_TIMED_OUT);
  Stand 2026-09-19 unverändert.
- **Blockade:** Host nicht erreichbar.
- **Braucht:** Re-Messung bei Erreichbarkeit.

### src.pas TAP
- **Status:** wartend | **Bindung:** termin (Route)
- **Lage:** `ledger.φ:14` — direct + proton-exit HTTP 0, wayback 503.
- **Blockade:** Route absent.
- **Braucht:** `bin/proton-wg.sh suggest pithia.cbk.waw.pl` (Operator-Wort).

### Wartend (kein Auswahlpunkt)
- Lasair-LSST (502), Sonden-Antworten (`blocked_sources.φ`), BepiColombo
  bc_mpo_more, NRS02-10/12/13 SHAPE, EMODnet HFRADAR NADR (Re-Messung fällig
  2026-10-19).

## Benchmark

- **CDN-Tag-Migration (P1):** grind-pro lieferte die 134 url-Zeilen + eve/ned;
  der mechanische Rest (de/neptune: 2 consts + 7 url-Zeilen + 1 Workflow) lief
  **grind-flash** korrekt (`sgrep -c` = 0, `cargo check` 0/0) — flash trägt die
  mechanische Migration, pro den koordinierten Kern. Kein Doppel-Lauf.
- **P4 Babamul/IA2:** grind-flash (Workflow-YMLs aus Muster) — korrekt.
- **P5 fai.kz:** grind-pro (novel Format/Compiler) — korrekt.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Commit:** `.github/workflows/{eve,ned,de44,neptune-de440s}-cdn.yml`,
  `.github/workflows/{babamul,ia2,fai-kz}-cdn.yml` (neu),
  `src/archivar/{mod.rs, fai_kz.rs}`, `tools/harvest/src/bin/{eve_compiler,
  ned_cone_compiler, de_compiler, neptune_ephemeris_compiler, fai_kz_compiler}.rs`,
  `phi/sources.φ`, `phi/pipeline/ledger.φ`, neues Handover
  `handover-2026-09-21-ernte-folge131.md`, Move
  `handover-2026-09-21-ernte-folge130.md` → `archiv/`.
- **Fremd (nicht angetastet):** entscheid (`folge78`, Move `folge77`, `post.md`,
  `external-state.md`), forschung (`folge134`, Move `folge133`, `KERNEL_INDEX.md`,
  `free_model_bench.rs`).
- Nie ein nacktes `git commit`; committet wird pfad-begrenzt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent, nie das Commit-Wort.
