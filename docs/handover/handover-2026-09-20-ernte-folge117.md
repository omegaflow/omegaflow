<!--
  title: Handover — Ernte-Folge 117 (Stand 2026-09-20)
  session: Ernte-Folge 117
  class: handover
  date: 2026-09-20
  sha256: ca6efc57465e55f481db86acd1cb31fead9ae067716a7bd4ef24d8e5e302077a
  status: live
-->
# Handover — Ernte-Folge 117 (2026-09-20)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich. Nur eigene Arbeit: bei geteilten Dateien nur die
eigenen Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit
wird nie überschrieben; gepusht wird, sobald der eigene Commit steht und
`origin/main` Vorfahr von HEAD ist (Fast-Forward).

Der erste offene Abschnitt benennt den härtesten undatierten Punkt (datierte
Wiedervorlagen schweigen vor ihrem Datum). Jeder offene Punkt trägt seinen
nächsten Schritt in derselben Zeile; Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`). Wartestellungen sind
kein Auswahlpunkt. Das Handover wird **vor allem anderen gegen den Baum gehalten**
— das Register ist die Frage, der Baum die Messung.

## Stehender Pass (gemessen 2026-09-20, Folge 117)

- **HEAD** `1963be30` (forschung-folge114), == `origin/main`. Arbeitsbaum:
  **sauber** — `git_safety --snapshot`: tree equals HEAD. Eigener Pfad-Satz:
  `phi/harvest.φ:79,83`, `phi/sources.φ:7980`, neues Handover folge117, Move
  folge116 → `archiv/`.
- **Postfach** — `state/mail/mail_ledger.φ` max-Timestamp `1789918147`
  (unverändert seit Folge 115 → kein Neu-Schreiben der `external-state.md`-Zeile).
- **CI-Status** — Ernte-relevant: skip-8 `harvest` `35522225969` @`1963be30`
  **completed/success** (16:20Z), Granule `ATL03_20260630005144_02443211_007_01.h5`
  (1241513984 B) → 8192 records, 458760 B, CDN `--sniff` sha256
  `bc670bf6…`. Repo-weit `ci-check`-Kette rot (Bau-Domäne); `external-state.md`-CI-Zeile
  bleibt Bau-geführt und **nicht angefasst**.

## Offen

- **Kein abarbeitbarer undatierter Punkt.** Der skip-8-Lauf `35522225969` ist
  geschlossen: `phi/harvest.φ:79` `args --skip 9`, `:83` `note` fortgeschrieben;
  `phi/sources.φ:7980` sha256 aktualisiert (war skip 7). Der Push triggert
  `harvest-dispatch` (args geändert, idempotent true) → nächster Lauf (skip 9)
  `wartend` (Trigger Run-Abschluss; nächster Pass: `ci_manage list` /
  `ci_manage view <id>`).

## Wartend (kein Auswahlpunkt)

- **TAP-Backends dachs.fai.kz + pithia.cbk.waw.pl** `ledger.φ:10-20` — beide
  DaCHS-Prozesse laufen, PostgreSQL `localhost:5432` tot; Trigger: sync-QUERY
  500→200 bzw. `/tap/tables` 500→200. `blockiert`.
- **Lasair-LSST API** `external-state.md` — Backend 502 über Proton, direct 000;
  Token vorhanden. Trigger: Backend erholt sich. `blockiert`.
- **allwise-cdn** — stündlicher Schedule. `wartend`.
- **Sonden-Antworten** (Voyager/Mariner/Viking/Juno) `blocked_sources.φ:39-53`. `wartend`.
- **BepiColombo bc_mpo_more** `blocked_sources.φ:26-29` — Freigabe ~April. `wartend`.
- **Babamul / IA2 TAP / GHRC** `blocked_sources.φ` — kein gebauter Konsument → `pending`. `wartend`.

## Termin

- **EMODnet HFRADAR NADR** `ledger.φ:22-24` — nächste Re-Messung **2026-10-19**.

## Ausgelagerte Fremd-Owner-Punkte (nicht im eigenen Handover)

- post_body-Migration → bau (`post.md`).
- queue-Korpora astro/earth/exotic (Operator-Wort/Lauf-Ort) → entscheid (`post.md`).

## Benchmark

- **Ernte-Folge 117** — kein Doppellauf: Routine (CI-Verdikt-Read `ci_manage log`
  + `archive_search --sniff` + Register-Fortschreibung), Hauptsession (flash-Tier).
  Der skip-8-Lauf ist ein CI-Job, kein lokaler Funktionslauf.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** `phi/harvest.φ:79,83` (args + note), `phi/sources.φ:7980`
  (sha256), neues Handover `handover-2026-09-20-ernte-folge117.md`, Move
  `handover-2026-09-20-ernte-folge116.md` → `archiv/`.
- **Fremd (nicht anfassen):** keine. Nie ein nacktes `git commit`; committet wird
  pfad-begrenzt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent, nie das Commit-Wort.
