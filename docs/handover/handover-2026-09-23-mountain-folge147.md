<!--
  title: Handover — Mountain-Folge 147 (2026-09-23)
  session: Mountain-Folge 147
  class: handover
  date: 2026-09-23
  sha256: 962a77acd5a11eb0226c5dc433aa29ce5d2b43aaebf5e61df285bb278ae5740c
  status: live
-->
# Handover — Mountain-Folge 147 (2026-09-23)

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
Punkt wird **aufgeschlüsselt** geführt — **Trigger** / **Lage** / **Blockade** /
**Braucht** (der wörtliche, kopierbare Schritt). `operator-gebunden`, `blockiert`
und `wartend` werden benannt, nie dispatcht. Jeder Punkt trägt seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-23 ~23:3x lokal / 21:37Z)

- **HEAD** `d87a0c9b3` (dieser Port-Commit); `origin/main == HEAD`, eigener Commit
  gepusht. Arbeitsbaum trägt **fremde** uncommittete Arbeit einer anderen Linie:
  `src/archivar/channels.rs`, `src/archivar/fetch.rs`, `src/archivar/main_flow.rs`, `src/archivar/spectral.rs`, `src/archivar/tests.rs`,
  `src/mathematikerin/dispersion.rs` — nicht Mountain, unberührt.
- **CI** (`ci_manage view`, ~21:37Z): `te-gate 35893882101` @`6a68c1901`
  **in_progress** (updated 20:03Z); `free-model-bench 35893010538` @`d754ecdf8`
  **in_progress** (updated 20:49Z); `register-dropped 35922188109`
  **in_progress** (created 21:24Z). Alle drei Trigger-Runs unverändert offen.
- **Postfach:** `state/mail/mail_ledger.φ` **absent** → keine Mountain-Zeile.
- **`register_lookup --open`:** die einzige mountain-eigene Zustandszeile
  (`phi/blocked_sources.φ:55` AFAD TADAS) ist in diesem Atom entblockt.
- **`open_points_check` folge146:** 27 Pfad-Refs, 0 absent (beim Start gemessen).

## In diesem Atom gebaut (git trägt)

- **P7 AFAD TADAS parser-def aufgelöst.** `grind-flash` maß die JSON-Route:
  `deprem.afad.gov.tr/apiv2/event/filter` → 302 auf
  `https://servisnet.afad.gov.tr/apigateway/deprem/apiv2/event/filter?start=…&end=…&minmag=…`
  (HTTP 200 `application/json`, flaches Objekt-Array, alle Werte als String;
  Felder `eventID/latitude/longitude/depth/type/magnitude/date`; `start<end` strikt).
  Der generische `Extract::Map`-Arm trägt das ohne neuen Parser: Block in
  `phi/sources.φ` (`map .`, `lat latitude`, `lon longitude`, `epoch date`,
  `mag_type_key type`, `field magnitude … seismic-surface Mw`,
  `field depth … seismic-body km`). Eintrag `blocked parser-def html` aus
  `phi/blocked_sources.φ` entfernt (entblockt), `phi/pipeline/ledger.φ`
  `disponiert`. Commit `d87a0c9b3`.
- **Verifikation dispatcht** (auf `d87a0c9b3`): `health-check 35923606472`
  (`--verify phi`), `source-census 35923610972` (Live-Fetch aller Quellen);
  der Push zog `ci-check 35923601360` + `harvest-dispatch 35923601427` nach.

## Offen (aufgeschlüsselt)

### 1. AFAD-Block-Verifikation (neu aus P7)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Abschluss `health-check 35923606472` / `source-census 35923610972`
- **Lage:** beide pending/queued (gemessen 2026-09-23 21:37Z via `gh run list`)
- **Blockade:** CI-Runner-Queue
- **Braucht:** `ci_manage view 35923606472` (`--verify phi` grün) +
  `ci_manage log 35923610972` (AFAD-Fetch ok, echte Samples); bei rot den
  `map`-Arm/die Keys in `phi/sources.φ` nachbessern

### 2. flare-Re-Insert green-confirm
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Abschluss `te-gate 35893882101` @`6a68c1901`
- **Lage:** in_progress (gemessen 2026-09-23 21:37Z via `ci_manage view`)
- **Blockade:** Laufdauer (Job-Timeout 330 min)
- **Braucht:** `ci_manage view 35893882101` → `flare`-Job grün (assert +
  `flare power probe:`-Zeilen)

### 3. Free-Model-Bench T4 + CF-`http_401`
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Abschluss `free-model-bench 35893010538` @`d754ecdf8`
- **Lage:** in_progress (gemessen 2026-09-23 21:37Z via `ci_manage view`)
- **Blockade:** Runner-Queue
- **Braucht:** `ci_manage log 35893010538 --all` → T4-pass-counts; bei
  vorhandenem CF-`http_401`-Anteil eine `An future:`-Zeile (operator-gebunden)

### 4. `register_lookup --dropped` Zählwurzel
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Abschluss `register-dropped 35922188109`
- **Lage:** in_progress (gemessen 2026-09-23 21:37Z via `ci_manage view`)
- **Blockade:** neues Binary existiert nur in CI
- **Braucht:** `ci_manage view 35922188109` → Summary-Zeile `N = Z − R`; dann
  **ein** Commit: `count_only`-Shortcut in `run_dropped`
  (`tools/register/src/bin/register_lookup.rs`) hinter die Auflösung schieben,
  `--count` druckt `dropped − resolved` + `dropped-baseline N` in
  `docs/zustand/dropped-baseline.md`

### 5. Ox64-Zweitknoten
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** Geräteankunft (Tracking `LZ473049629CN`)
- **Lage:** nicht angekommen (gemessen via `state/mail/mail_ledger.φ` `1790046330`)
- **Blockade:** physische Ankunft
- **Braucht:** nach Ankunft Bring-up + Kopplung messen

## Benchmark

- Drei Taucher-Aufgaben, alle **grind-flash** (billigste Klasse): P7-Route
  (`archive_search --verdict/--sniff` + curl GET), P7-Port (Block + Register),
  Exploration der Parser-Struktur (`explore`, flash). Kein pro/max-Doppel —
  Routine-Klasse geschlossen 2026-09-16 (flash Sieger `$0.0008`). Der Port nutzt
  `Extract::Map` (kein neuer Parser) — kein hartes Atom, keine Eskalation.

## Geteilter Baum — eigener Pfad-Satz

- **Eigener Commit `d87a0c9b3`:** `phi/sources.φ` (AFAD-Block),
  `phi/blocked_sources.φ` (AFAD-Eintrag entfernt), `phi/pipeline/ledger.φ`
  (`disponiert` AFAD)
- `docs/handover/handover-2026-09-23-mountain-folge147.md` (neu)
- Move `handover-2026-09-23-mountain-folge146.md` → `archiv/` (eigene Linie, atomar)
- **Fremd/unberührt:** `src/archivar/channels.rs`, `src/archivar/fetch.rs`, `src/archivar/main_flow.rs`, `src/archivar/spectral.rs`, `src/archivar/tests.rs`,
  `src/mathematikerin/dispersion.rs` (andere Linie, uncommittet)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
