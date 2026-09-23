<!--
  title: Handover — Mountain-Folge 145 (2026-09-23)
  session: Mountain-Folge 145
  class: handover
  date: 2026-09-23
  sha256: 98e050141cb0ebeb3eada25ab4c0239ac8542aab2c6f05df7246c414eef14dba
  status: live
-->
# Handover — Mountain-Folge 145 (2026-09-23)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird, sobald der eigene Commit steht und `origin/main`
Vorfahr von HEAD ist (Fast-Forward) — ein Push sendet nur Commits, der Arbeitsbaum
darf schmutzig sein.

Es gibt keine Rangfolge und keinen `härtesten Punkt` — die offenen Punkte werden
**parallel** von Agenten abgearbeitet (Operator-Wort 2026-09-21). Jeder offene
Punkt wird **aufgeschlüsselt** geführt — **Trigger** / **Lage** (gemessen, mit
Messstempel) / **Blockade** / **Braucht** (der wörtliche, kopierbare Schritt).
`operator-gebunden`, `blockiert` und `wartend` werden benannt, nie dispatcht. Jeder
Punkt trägt seinen Status-Tag (`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-23 ~19:4xZ)

- **HEAD** `9f5e4a846` == `origin/main`. Arbeitsbaum trägt **fremde** Arbeit:
  river (staged Rename folge16→`archiv/`, untracked `handover-…-river-folge17.md`),
  sensory (`src/archivar/fit.rs`, `ingress.rs`, `docs/specs/mantis-shrimp-build.md`,
  `firmware/*`, `post.md`-Hunk `An sensory:`) und eine **zweite Session**, die
  `phi/sources.φ` relaxiert (344 `note`-Zeilen gelöscht, gemessen via `git diff` —
  `note` ist in `sources.φ` ohnehin per `phi-sources-note` verboten). Unberührt.
- **CI** (`ci_manage`, 19:4xZ): `te-gate 35893882101` @`6a68c1901` **pending**,
  gehalten von `te-gate 35875025486` @`122d36ef` (**in_progress** seit 14:33Z;
  `.github/workflows/te-gate.yml:3` `concurrency.group` = workflow,
  `cancel-in-progress:false` — der laufende n=1000-Batterie-Lauf blockiert den
  flare-Lauf). `free-model-bench 35893010538` pending + `free-model-agent-bench
  35893014541` queued @`d754ecdf8`.
- **Postfach:** `state/mail/mail_ledger.φ` **absent** → keine Mountain-Zeile.
- **`register_lookup --open`:** 2 mountain-eigene `parser-def`-Zeilen
  (`phi/blocked_sources.φ:51` GMRT, `:55` openSenseMap) — **in diesem Atom gebaut**,
  Einträge entfernt.
- **`open_points_check` folge144:** 14 Pfad-Refs, **0 absent**, 0 guardians.

## In diesem Atom gebaut (git trägt)

- `tools/harvest/src/bin/gmrt_compiler.rs` — GMRT-GridServer-GeoTIFF → `GMR1`-GbcoRec
  (gravity/gestalt, Elevation m), Roundtrip-Verify, `--ci-mode` CDN-Upload;
  `.github/workflows/gmrt-cdn.yml`.
- `tools/harvest/src/bin/opensensemap_compiler.rs` — openSenseMap Temperatur (°C,
  thermal; `/boxes` → `/boxes/:id` → `sensors[N].value`, Auflösung über **title+unit**
  statt harten Index), Roundtrip, CDN-Upload; `.github/workflows/opensensemap-cdn.yml`.
- Core-Arme: `src/archivar/geo.rs` (`GMR1`/`OSM1`), `zeuge.rs` (Gestalt/Oszillator-Identität),
  `main_flow.rs` (gmrt-Konsument), `extract.rs` (opensensemap-Komponente).
- Register: `phi/sources.φ` (2 url-Zeilen), `phi/harvest.φ` (2 Arme `asset fehlt`),
  `phi/witnesses.φ` (GMRT-Gestalt). `cargo check` 0/0 (`-p omegaflow --tests`,
  `-p omegaflow-harvest --all-targets`).

## Offen (aufgeschlüsselt)

### 1. GMRT-CDN-Manifestation
- **Status:** termin | **Bindung:** eigen
- **Trigger:** Push dieses Atoms → `gh workflow run gmrt-cdn.yml`
- **Lage:** Arm + Workflow + Register stehen; `phi/harvest.φ` `asset fehlt` (gemessen 19:4xZ)
- **Blockade:** keine
- **Braucht:** nach Push `gh workflow run gmrt-cdn.yml`; bei success sha256 in
  `phi/sources.φ` und `asset present` in `phi/harvest.φ`

### 2. openSenseMap-CDN-Manifestation
- **Status:** termin | **Bindung:** eigen
- **Trigger:** Push dieses Atoms → `gh workflow run opensensemap-cdn.yml`
- **Lage:** Arm + Workflow + Register stehen; `phi/harvest.φ` `asset fehlt` (gemessen 19:4xZ)
- **Blockade:** keine
- **Braucht:** nach Push `gh workflow run opensensemap-cdn.yml`; bei success sha256 in
  `phi/sources.φ` und `asset present` in `phi/harvest.φ`

### 3. flare-Re-Insert green-confirm
- **Status:** termin | **Bindung:** eigen
- **Trigger:** Abschluss von `te-gate 35893882101` @`6a68c1901`
- **Lage:** pending, gehalten von der `te-gate`-Concurrency (`35875025486` in_progress
  seit 14:33Z, gemessen 19:4xZ via `ci_manage view`)
- **Blockade:** der laufende n=1000-Batterie-Lauf (Job-timeout 330 min) — eine Messung,
  kein cancel
- **Braucht:** nach Abschluss `ci_manage view 35893882101` → `flare`-Job grün
  (assert + `flare power probe:`-Zeilen)

### 4. Free-Model-Bench T4/T7-Verifikation
- **Status:** termin | **Bindung:** eigen
- **Trigger:** Abschluss von `35893010538` (fmb) + `35893014541` (agent-bench) @`d754ecdf8`
- **Lage:** pending/queued (gemessen 19:4xZ via `ci_manage view`)
- **Blockade:** Runner-Queue
- **Braucht:** `ci_manage log 35893010538 --all` → T4/T7 pass counts > 0; der
  CF-`http_401`-Anteil steht als `An future:`-Zeile (operator-gebunden)

### 5. Ox64-Zweitknoten
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** Geräteankunft (Tracking `LZ473049629CN`)
- **Lage:** nicht angekommen (gemessen via `state/mail/mail_ledger.φ` `1790046330`)
- **Blockade:** physische Ankunft
- **Braucht:** nach Ankunft Bring-up + Kopplung messen

### 6. `register_lookup --dropped` Zählwurzel (von mycelium folge148 gemeldet)
- **Status:** termin | **Bindung:** eigen
- **Trigger:** workable (kein Warten)
- **Lage:** `--dropped --count` zählt alle Drops **vor** der Git-Auflösung; ~95–96 %
  sind `commit-resolved` (umformulierte Unterfeld-Zeilen), kein Verlust (gemessen
  2026-09-23 via `grind-flash`)
- **Blockade:** keine
- **Braucht:** `tools/register/src/bin/register_lookup.rs` (`run_dropped`) prüfen, ob
  die Zahl post-Auflösung gemeldet werden soll; sonst den Befund als gelöst registrieren

## Benchmark

- Zwei Parser-Arm-Atome parallel an `grind-pro` (harte Klasse „parser-arm"):
  beide in je **einem** Kontext geliefert (Force-Gate-Verdikt + Arm + Register +
  Tests, `cargo check` 0/0). `grind-max` nicht nötig, kein flash-Kontrolllauf in
  diesem Atom (Routine-Klasse geschlossen 2026-09-16; kein Klassen-Sieger neu zu
  doppeln). Gefundene Korrektur: der GMRT-`PointServer`-Pfad ist **kein json**
  (blanker Skalar `-294`) — die alte `note` war an dieser Stelle falsch.

## Geteilter Baum — eigener Pfad-Satz

- **Neu:** `tools/harvest/src/bin/gmrt_compiler.rs`,
  `tools/harvest/src/bin/opensensemap_compiler.rs`,
  `.github/workflows/gmrt-cdn.yml`, `.github/workflows/opensensemap-cdn.yml`
- **Geändert:** `src/archivar/geo.rs`, `src/archivar/zeuge.rs`,
  `src/archivar/main_flow.rs`, `src/archivar/extract.rs`; `phi/blocked_sources.φ`,
  `phi/harvest.φ`, `phi/witnesses.φ`; `phi/sources.φ` — **nur die eigenen 15 Zeilen**
  (2 url-Blöcke); die 344 fremden `note`-Löschungen der zweiten Session bleiben unstaged
- `docs/handover/handover-2026-09-23-mountain-folge145.md` (neu)
- Move `handover-2026-09-23-mountain-folge144.md` → `archiv/` (eigene Linie, atomar)
- **Fremd/unberührt:** river (staged Rename, folge17), sensory (`fit.rs`, `ingress.rs`,
  `mantis-shrimp-build.md`, `firmware/*`, `post.md`-Hunk), zweite Session (`sources.φ`-Relaxation)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
