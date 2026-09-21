<!--
  title: Handover — Ernte-Folge 132 (Stand 2026-09-21)
  session: Ernte-Folge 132
  class: handover
  date: 2026-09-21
  sha256: 1ba6bafcff5e9a80b98d6e06abfe332be0f9129aae93e0e58ba2b9e6038bb287
  status: live
-->
# Handover — Ernte-Folge 132 (2026-09-21)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird pfad-begrenzt, fremde
uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden parallel von Agenten
abgearbeitet. Jeder offene Punkt trägt seinen nächsten Schritt in derselben Zeile;
Status-Tag (`wartend` | `operator-gebunden` | `blockiert` | `termin`).
Wartestellungen sind kein Auswahlpunkt.

## Stehender Pass (gemessen 2026-09-21, Folge 132)

- **HEAD** `0c0b30fb` beim Start; Arbeitsbaum trägt Fremdarbeit (entscheid
  `folge78` + Move `folge77`, `external-state.md`; forschung `folge134` + Move
  `folge133`, `KERNEL_INDEX.md`, `lead-geometry-direction.md`) — nicht angetastet.
- **Postfach** — kein neues `An ernte`; `post.md` trägt `An bau` (text_review) +
  neu `An entscheid` (DEMETER ISL, diese Folge).
- **CI** — Massen-`-cdn`-Dispatch 10:14–10:16: **33 Läufe über 27 Workflows**
  gescheitert; Ursache gemessen: die Workflows tragen **keinen `actions/checkout`-Step**
  (`error: could not find Cargo.toml in /home/runner/work/omegaflow/omegaflow`).
  Erfolge: fai-kz `35591261870`, ia2 `35591241104`, de44 `35591251300`,
  neptune-de440s `35591254999`; eve `35591258884` in_progress.
  - babamul `35591238287` failure: `BABAMUL_API_TOKEN absent` — Secret existiert
    (`gh secret list`), Workflow-`env` fehlte.
  - ned `35591248345`/`35591264600` failure: Release `ned.ipac.caltech.edu-ned` fehlt.
  - psr-cdn `35587702458`/`35587801743`: TAP `curl (22) 400` → query void (quellseitig).
- **Zustand** — `external-state.md` von entscheid fortgeschrieben (fremd); der
  CI-Status-Eintrag wurde nicht angetastet (fremder Hunks in derselben Datei).

## Offen (aufgeschlüsselt)

### sha256-Nachzug der neu dispatchten Läufe
- **Status:** wartend | **Bindung:** termin (Läufe)
- **Lage:** 29 `-cdn`-Workflows dispatched (Läufe `35594472387`–`35594552094`,
  avo…wds) + psr-cdn `35594952955`; queued.
- **Blockade:** Lauf-Abschluss.
- **Braucht:** `ci_manage list`/Watchdog-Snapshot; bei success sha256 →
  `sources.φ`-note → `kompiliert`.

### Quaoar Sternbedeckung — Manifestation
- **Status:** wartend | **Bindung:** termin (Run `35595708241`)
- **Lage:** dispatched 2026-09-21; `sources.φ:8706` Block pending sha256;
  Workflow-Tag `zenodo.org` konsistent.
- **Blockade:** Lauf-Abschluss.
- **Braucht:** `ci_manage view 35595708241`; bei success sha256 → `sources.φ:8706` / `ledger.φ:32`.

### PS1 Order-10-Final
- **Status:** wartend | **Bindung:** termin (Run `35595711594`)
- **Lage:** dispatched 2026-09-21; `footprints.φ:19` — Asset `ps1_dr2_coverage.fp01` absent;
  Workflow-Tag `ssd.jpl.nasa.gov-ps1`; kein `sources.φ`-Eintrag (Footprint).
- **Blockade:** Lauf-Abschluss.
- **Braucht:** `ci_manage view 35595711594`; bei final → `footprints.φ:19`.

### DEMETER (CDN + ISL)
- **Status:** wartend / operator-gebunden | **Bindung:** termin + operator
- **Lage:** CDN-Lauf `35567568429` queued; ISL account-gated (`blocked_sources.φ:70`).
- **Blockade:** Lauf / Konto.
- **Braucht:** `ci_manage view 35567568429`; ISL-Konto → entscheid (`post.md:20`).

### SSDC Limadou
- **Status:** wartend | **Bindung:** termin (CSES-02-Umbau)
- **Lage:** `ledger.φ:26` — SSDC-Portal rendert (`--playwright`), CSES-Limadou
  ist eine Mission mit eigener Seite; Datenzugang login/PI-gebunden, PI „wait a few weeks".
- **Blockade:** PI-Portal.
- **Braucht:** neue Anleitung auf dem Limadou-Portal.

### EPA RadNet Koordinaten
- **Status:** wartend | **Bindung:** termin (FRS)
- **Lage:** `blocked_sources.φ:74` — RadNet Dashboard (near-real-time Air) offen
  mit Monitor-Standorten; hist. Precip/Milch/Wasser ohne Koordinaten;
  FRS `ofmpub get_facilities` 503.
- **Blockade:** FRS-Maintenance (nur hist. Matrizen).
- **Braucht:** FRS-Retry `pgm_sys_acrnm RadNet`; Luft-Route via Dashboard nutzbar.

### EMODnet maxTime
- **Status:** wartend | **Bindung:** termin (Compiler/Manifestation)
- **Lage:** `ledger.φ:18` — Dataset umbenannt `HFRADAR_NADR_Totals` →
  `EUHFR_NRTcurrent_HFR-NAdr-Total` (56 HFRADAR-Totals live); Host direkt HTTP 200;
  `time_coverage_end` 2026-09-21T10:00:00Z (PT30M).
- **Blockade:** keine (Route + Host offen).
- **Braucht:** Compiler/Manifestation auf die neue Dataset-ID.

### src.pas TAP
- **Status:** wartend | **Bindung:** termin (Dienst-Backend)
- **Lage:** `ledger.φ:14` — Operator-Wort PL-Exit erteilt 2026-09-21; über PL-Exit
  `/tap` HTTP 200, `/tap/tables` HTTP 500 (PostgreSQL `localhost:5432` refused)
  → Route offen, Dienst-Backend down.
- **Blockade:** Pithia-Datenbank (dienstseitig).
- **Braucht:** Re-Messung `/tap/tables` bei 200 (Trigger).

### Wartend (kein Auswahlpunkt)
- Lasair-LSST (502), Sonden-Antworten, BepiColombo bc_mpo_more,
  NRS02-10/12/13 SHAPE, EMODnet HFRADAR NADR (Re-Messung fällig 2026-10-19).

## Benchmark

- Kein Doppel-Lauf diese Folge: A/C liefen `grind-flash` (sha256/sniff bzw.
  post/Bereitschaft), B lief `grind-pro` (27-Workflow-Klassifikation) — keine
  flash/pro-Paarung auf derselben Aufgabe, kein neuer Sieger.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Commit:** 29 `.github/workflows/*-cdn.yml`, `phi/sources.φ`,
  `phi/pipeline/ledger.φ`, `docs/handover/post.md`, neues Handover
  `handover-2026-09-21-ernte-folge132.md`, Move
  `handover-2026-09-21-ernte-folge131.md` → `archiv/`.
- **Fremd (nicht angetastet):** entscheid (`folge78`, Move `folge77`,
  `external-state.md`), forschung (`folge134`, Move `folge133`, `KERNEL_INDEX.md`,
  `lead-geometry-direction.md`).
- Nie ein nacktes `git commit`; committet wird pfad-begrenzt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent, nie das Commit-Wort.
