<!--
  title: Handover — Ernte-Folge 124 (Stand 2026-09-20)
  session: Ernte-Folge 124
  class: handover
  date: 2026-09-20
  sha256: PLACEHOLDER
  status: live
-->
# Handover — Ernte-Folge 124 (2026-09-20)

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

## Stehender Pass (gemessen 2026-09-20, Folge 124)

- **HEAD** `276034b2` (entscheid folge67) beim Start, == `origin/main`; eigener
  Commit folgt. Arbeitsbaum: fremde uncommittete Arbeit
  `tools/register/src/bin/register_lookup.rs` — nicht anfassen; eigener
  Pfad-Satz unten.
- **Postfach** — 2 Zeilen gesetzt: `An bau` (units.rs `bq/l`), `An entscheid`
  (Split-Routing, operator-gebunden). Kein neuer Mail-Ledger-Eingang seit
  `1789930255`.
- **CI-Status** — am HEAD: `ci-check` `35534657914` in_progress,
  `tools-build` `35534657902` success, `te-gate` `35534898200` pending,
  `free-model-agent-bench` `35534406541` pending (Runner-Rückstau; kein Polling).

## Ernte-Dropped-Audit (forschung-Tool `register_lookup --dropped`)

- **530 markiert / 57 commit-aufgelöst / 473 unaufgelöst** (Folge 1–123).
  Triage: **~90 % False Positives** — der Punkt steht umformuliert in einer
  späteren Übergabe oder im Register. Genuin still fallengelassen (am Baum
  nachgemessen):
- **DEMETER** — `demeter-cdn` `35228716483` **failure** (8 h, exit 1);
  Compiler + `demeter-cdn.yml` + `demeter-aggregate-cdn.yml` + self-hosted
  Runner im Baum, **77 Shards nie in `sources.φ`/`harvest.φ` registriert**.
  Schritt: `ci_manage log 35228716483` Fehlerdiagnose, dann Re-Dispatch
  `gh workflow run demeter-cdn.yml`; bei success die 77 `url`+`sha256`-Zeilen
  registrieren.
- **Quaoar sha256** (`zenodo.21185812`, 572 467 032 B) — Zenodo 504, sha256
  `pending`. Schritt: bei Rückkehr streamend `sfetch <url> | sha256sum`, dann
  `sources.φ` (TNBFits-Block-Nachbarschaft).
- **Split-Routing-Verifikation** (folge12–17) — operator-gebunden, nie
  abgegeben; jetzt als `An entscheid`-Post gesetzt.
- **EPA AQS map-key-Gap** (`daily_88101`, `sources.φ:1401`) — der „Arithmetic
  Mean"-`map`-Fix ist weder gebaut noch benannt. Schritt: `aqs_parser` gegen die
  2025-ZIP prüfen, Ergebnis als Verdikt eintragen.
- **GHRC GLM/TRMM** — Compiler + CDN ungebaut; öffentliche NOAA-AWS-Route
  messen, dann registrieren.
- **False Positives der Triage** (stehen schon im Register):
  `erddap.osupytheas` HFRADAR (`declined_sources.φ:1234`), Scopus (`:939`),
  GNIP/Mendeley (`:3287`/`:3575`), MarineCadastre (`:1602`), `dachs.fai.kz`
  (`ledger.φ:11`).

## Queue-Korpora — 368 Survivor (härtester undatierter Punkt)

- **7 Korpora `verifiziert`** (`phi/pipeline/ledger.φ`; 13k 148, 14k 25, 15k 156,
  183l 4, 2k 3, 7k 21, staging 11). Schritt: Survivor-Quelle lokalisieren
  (`phi/pipeline/probe_survivors.φ` + Korpora), Dedupe gegen `phi/sources.φ`,
  Oszillator-Gate, dann `sources.φ`/`dead_sources.φ` + Ledger fortschreiben
  (SOURCE_PORT §5.4).

## Queue-Korpora — 3 ausstehend

- **30-astro, earth-stac-sentinel, exotic-neutrino-ligo** — force-gate B gebaut
  (Block ohne `force`-Direktiv bleibt `# pending … review`). Schritt: Re-Lauf auf
  frischem Binär (`bin/.tools_ensure omegaflow`), `# pending`-Zeilen zählen, dann
  Disposition.

## Katalog-Inventar stale (Prozess-Gap)

- Der Digest zählt die 192 bereits disponierten Kandidaten weiter als
  `candidate → ernte` (die gitignored `phi/pipeline/catalog/*.φ` tragen die
  Zeilen unverändert). Schritt: Digest gegen die Register deduplizieren oder die
  Katalogzeile bei Disposition mitführen.

## PS1 (wartend)

- **PS1-Ernte-Rate** — wartend auf `ps1-cdn`; Verdikt offen. Schritt:
  `ci_manage list` nach `ps1-cdn`, einmalig `ci_manage log <id>`.
- **PS1-Fraktional Order-10-Final** — Größe aus dem Combine-Log → `footprints.φ`.

## Wartend (kein Auswahlpunkt)

- **TAP-Backends dachs.fai.kz + pithia.cbk.waw.pl** (`ledger.φ:11`/`:19`) —
  Trigger sync-QUERY/tables 500→200.
- **SSDC Limadou** (`ledger.φ:27`) — operator-gebunden (PI-Freigabe).
- **Lasair-LSST API** — Backend 502; Trigger Erholung.
- **Sonden-Antworten** (Voyager/Mariner/Viking/Juno) `blocked_sources.φ`.
- **BepiColombo bc_mpo_more** — Freigabe ~April.
- **NRS02-10,12,13 SHAPE** `nrs_stations.φ:17` — Trigger neuer Stations-Prefix.
- **Babamul / IA2 TAP** `blocked_sources.φ` — kein gebauter Konsument → `pending`.

## Termin

- **EMODnet HFRADAR NADR** — nächste Re-Messung **2026-10-19**.

## Ausgelagerte Fremd-Owner-Punkte (nicht im eigenen Handover)

- post_body-Migration → bau (`post.md`).
- queue-Korpora astro/earth/exotic (Lauf-Ort) → entscheid.
- units.rs `bq/l` → bau (`post.md`).
- Split-Routing → entscheid (`post.md`).

## Benchmark

- **Ernte-Folge 124** — `grind-pro` (2 accept-Verifikation: GWOSC → decline,
  RadNet → accept/blockiert), `research-max` (Dropped-Triage), Verifikation am
  Baum. Die Triage überzeichnete (3 Fälle fälschlich als verloren: osupytheas,
  Wand-Party-Trio, Quaoar-sha); die Baum-Verifikation korrigierte. Kein sauberer
  flash/pro-Vergleich; Burn nicht gemessen.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** `phi/pipeline/ledger.φ` (11 `disponiert`-Einträge
  entfernt), `phi/declined_sources.φ` (+1 GWOSC-Decline), `docs/handover/post.md`
  (+2 Zeilen), neues Handover `handover-2026-09-20-ernte-folge124.md`, Move
  folge123 → `archiv/`.
- **Fremd (nicht anfassen):** `tools/register/src/bin/register_lookup.rs`.
  Nie ein nacktes `git commit`; committet wird pfad-begrenzt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent, nie das Commit-Wort.
