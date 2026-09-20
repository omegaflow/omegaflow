<!--
  title: Handover — Ernte-Folge 107 (Stand 2026-09-20)
  session: Ernte-Folge 107
  class: handover
  date: 2026-09-20
  sha256: 126e2e1013c08c89740bd3464f266fb1d536245400bbf9982f588986c75bdecb
  status: live
-->
# Handover — Ernte-Folge 107 (2026-09-20)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Der erste offene Abschnitt benennt den härtesten undatierten Punkt. Jeder offene
Punkt trägt seinen nächsten Schritt in derselben Zeile; Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`). Wartestellungen sind
kein Auswahlpunkt. Das Handover wird **vor allem anderen gegen den Baum gehalten**
— das Register ist die Frage, der Baum die Messung.

## Stehender Pass (gemessen 2026-09-20, Folge 107)

- **HEAD** `e3c478af` (== `origin/main`); Arbeitsbaum sauber; `git_safety
  --snapshot` → „working tree equals HEAD — nothing to record". Eigener Pfad-Satz:
  `phi/harvest.φ` (icesat2 `--skip 3` + note), neues Handover, Move
  `handover-2026-09-20-ernte-folge106.md` → `archiv/`.
- **Postfach** — `state/mail/mail_ledger.φ`: jüngster Eingang `1789906306`
  (CSES-Limadou WG, an entscheid); davor nur API-Key-/Hardware-/Security-Mails.
  Kein neuer Ernte-Dateneingang.
- **CI-Status** — Watchdog-Snapshot 16:03: `35515194248` harvest **success**
  (`ci_manage view`, head `3f8d7f7d`); in_progress `ci-check` `35513190719`,
  `te-gate` `35513982359`, `health-check` `35505471538`; failure `release-build`
  `35513611936` + mehrere `ci-check`. Keine ernte-eigene Aktion.

## Offen

- **icesat2_atl03** `phi/harvest.φ:79` — run `35515194248` success gelesen
  (8192 records, 458760 B, CDN sha256 `fdba6385…`); args auf `--skip 3`
  (3/215) gesetzt, committet, `harvest.yml -f format=icesat2_atl03 -f force=true`
  dispatcht: Run `35516529023` (head `37369e95`, `in_progress` bei Dispatch). (Schritt: `ci_manage view
  <run-id>` einmalig den staged count lesen, dann `phi/harvest.φ`-note fortschreiben
  und `--skip 4` dispatchten.)
- **TAP-Backends dachs.fai.kz + pithia.cbk.waw.pl** `phi/pipeline/ledger.φ:10-20`
  — `blockiert` (extern, Backend down). (Schritt: `archive_search --verdict` +
  sync-QUERY, Trigger Backend-Erholung.) `blockiert`.
- **Lasair-LSST API** `external-state.md:23` — `blockiert` (extern, 502).
  (Schritt: `archive_search --verdict https://api.lasair.lsst.ac.uk/api/`, Trigger
  Banner-Wechsel.) `blockiert`.

## Wartend (kein Auswahlpunkt)

- **allwise-cdn** — stündlicher Schedule. (Schritt: `ci_manage view
  <allwise-run>`.) `wartend`.
- **Sonden-Antworten** (Voyager/Mariner/Viking/Juno) `phi/blocked_sources.φ:39-53`
  — Anfragen offen. `wartend`.
- **BepiColombo bc_mpo_more** `phi/blocked_sources.φ:26-29` — Freigabe ~April.
  `wartend`.
- **Babamul / IA2 TAP / GHRC** `phi/blocked_sources.φ` — kein gebauter Konsument →
  `pending`. `wartend`.

## Operator-gebunden

- **Queue-Korpora** `phi/pipeline/ledger.φ:114-120` — `--port` braucht das
  Operator-Wort (gehört zur entscheid-Queue).

## Termin

- **EMODnet HFRADAR NADR** `phi/pipeline/ledger.φ:22-24` — nächste Re-Messung
  **2026-10-19**.

## Benchmark

- **Ernte-Folge 107** — kein Doppellauf: der icesat2-Schritt ist Routine
  (flash-Klasse geschlossen, `grind-flash`), kein pro/max.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** `phi/harvest.φ` (icesat2 args `--skip 3` + note), neues
  Handover `handover-2026-09-20-ernte-folge107.md`, Move
  `handover-2026-09-20-ernte-folge106.md` → `archiv/`.
- **Fremd (nicht anfassen):** uncommittete Arbeit anderer Linien. Nie ein nacktes
  `git commit`; committet wird pfad-begrenzt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent, nie das Commit-Wort.
