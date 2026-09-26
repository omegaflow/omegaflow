<!--
  title: Handover — Mountain-Folge 173 (Stand 2026-09-26)
  session: Mountain-Folge 173
  class: handover
  date: 2026-09-26
  sha256: 555d2d64ff0be865a5fd9865582c90756d6068b9efd648c87e837160b292e060
  status: live
-->
# Handover — Mountain-Folge 173 (2026-09-26)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert; git trägt, was gemacht wurde. Keine Rangfolge — die offenen Punkte
werden parallel von Agenten abgearbeitet; `blockiert`/`wartend` werden benannt,
nie dispatcht. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen Hunks,
pfad-begrenzter Commit. Jeder Punkt aufgeschlüsselt: Trigger / Lage / Blockade /
Braucht. Dieser Atom hat gearbeitet: EHT-`--verify` ohne `.sha256sums`-Member
gefixt, NOHRSC-Disposition geschlossen, `harvest`-Workflow rx100-absent-tolerant,
register-coverage-Orphans (mountain + mycelium-Carrier) geschlossen, GIO2 ionocal
neu manifestiert.

## Stehender Pass

- **Postfach:** (gemessen 2026-09-26 via `sread`) `state/mail/mail_ledger.φ`
  301 Zeilen; kein neuer Mountain-Eingang. `mail_digest` meldet weiter „ledger
  absent" (kein CI-Build des Tools am HEAD).
- **CI-Status am HEAD:** (gemessen 2026-09-26 via `ci_manage`) HEAD `d766ed9a5`;
  `ci-check` `36271257595` **pending**; `register-coverage` `36271257656` **failure**
  (2 mountain-Orphans + 1 mycelium) — in diesem Atom gefixt.
- **Artefakt-Frische (`DUE`):** nach Commit+Push neu messen; `gh workflow run`
  für die betroffenen `*-cdn` + `tools-build` erst nach Commit+Push.

## Offen (aufgeschlüsselt)

### P2 EHT uvfits — `--verify` ohne `.sha256sums`-Member gefixt, Manifestation offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Commit+Push, dann `eht-uvfits-cdn`-Lauf.
- **Lage:** (gemessen 2026-09-26 via `ci_manage log 36270900529`) der Lauf
  scheiterte an `eht_uvfits_compiler: kernels/…tgz: no .sha256sums member`; das
  ALMA-tgz (2 304 234 106 B) trägt nur FITS-Member. Fix in
  `tools/harvest/src/bin/eht_uvfits_compiler.rs`: `--verify` misst jetzt
  Archiv-sha256 + Größe, ein fehlendes `.sha256sums`-Member ist gemessener
  `absent` (exit 0, keine erfundene Summe), optional `--sha <hex>`; `cargo check
  -p omegaflow-harvest` 0/0.
- **Blockade:** keine.
- **Braucht:** nach `/commit`+Push `gh workflow run eht-uvfits-cdn.yml`; nach
  success die Archiv-sha256 und die `Aa`–`Ap`-df aus dem Vollauf in `phi/sources.φ`
  nachtragen.

### P4 gll.rss ODR — Compiler gestreamt, Lauf in der Queue
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `gll-rss-odr-cdn` `36270904053` / `36270909605` Completion.
- **Lage:** (gemessen 2026-09-26 via `ci_manage view`) beide auf `346896b57`
  **in_progress**/**pending**, `updated_at` == Startzeit (20:50Z) — Runner-Queue,
  kein Log-Stadium.
- **Blockade:** Runner-Queue (kein Schritt zur Kante bis Log-Stadium).
- **Braucht:** bei Completion `ci_manage log <id>`; Shards `gll_rss_odr.bin.NNN` +
  `gll_rss_odr.manifest` + sha in `phi/sources.φ` nachtragen (Block um `:8438`).

### P5 CI-Verify + GIO2-Neumanifestation
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** grüner `ci-check` am neuen HEAD.
- **Lage:** (gemessen 2026-09-26 via `ci_manage`) `ci-check 36271257595` pending
  auf `d766ed9a5`; GIO2 `galileo-ionocal-cdn` neu dispatcht, Lauf `36272046933`
  **success**; der rote `register-coverage 36271257656` trug 2 mountain-Orphans
  (`phi/harvest.φ:145`/`:233`) + 1 mycelium-Orphan — in diesem Atom geschlossen
  (NOHRSC `asset present`; RX100-Key `Sony RX100 V Luminanz`; Mycelium-Carrier-URL
  voll qualifiziert).
- **Blockade:** keine.
- **Braucht:** nach Push `ci_manage view/log` am neuen HEAD; prüfen, dass
  `register-coverage` grün ist (Orphan-Fix) und `ci-check` die Mountain-Fixes
  (grib2/ionocal/parquet/uvfits) trägt.

### P6 Sony RX100 V Luminanz — CI-absent-tolerant, K-Kalibrierung offen
- **Status:** wartend | **Bindung:** eigen (Substanz river)
- **Trigger:** K-Messung gegen kalibriertes Luminanzmeter im Smart-Remote-LAN — `handover-2026-09-26-river-folge38.md:109`.
- **Lage:** (gemessen 2026-09-26 via `ci_manage log 36270934981`) der rote
  `harvest`-Lauf war der `rx100_luminance`-Arm: der Workflow verlangte ein
  CDN-Asset, das der absent-Arm nicht liefern kann. Fix in
  `.github/workflows/harvest.yml` + `harvest-long.yml`: der gemessene Marker
  `no camera answered M-SEARCH` (Compiler exit 0) setzt `absent=true`, der zweite
  Witness überspringt mit benannter Meldung; echte Compile-Fehler bleiben rot.
  `phi/harvest.φ:233` `asset fehlt` bleibt wahr; Key verkürzt auf
  `Sony RX100 V Luminanz:` (Carrier).
- **Blockade:** physische Kamera + K-Kalibrierung (River).
- **Braucht:** nach Push den `harvest`-Lauf lesen; K=`12.5` ungemessen,
  `freq`/`bin_width` nicht verdrahtet (`river-folge38.md:109-114`).

### UI-Chat-Stimmen zum `epochrange`-Befund (vier von fünf)
- **Status:** operator-gebunden (Akt) | **Bindung:** operator
- **Trigger:** Operator-Wort (Anmeldung in den vier Tabs).
- **Lage:** (gemessen 2026-09-26 via `chrome-devtools`) `claude.ai`,
  `chat.deepseek.com`, `kimi.ai`, `arena.ai` stehen an Login-/Consent-Wänden;
  `chat.z.ai` (GLM-5.3-Flash) lief und trägt `descoped`
  (`state/stimmen/2026-09-26_zai_ui_epochrange-p6.json`).
- **Vorbereitung (eigen):** (gemessen 2026-09-26) die Prompt-Datei liegt bereit;
  dieselbe Datei wird an alle vier Sessions gesendet.
- **Blockade:** Login/Consent (nicht umgangen).
- **Wort:** externe Stimmen über den Voice-Runner fragen (nicht chatgpt.com) | 2026-09-26 | Operator (Session).
- **Braucht:** vier Sessions im `chrome-devtools`-Browser anmelden (Operator), dann
  die Prompt-Datei senden.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
