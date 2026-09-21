<!--
  title: Handover — Ernte-Folge 125 (Stand 2026-09-20)
  session: Ernte-Folge 125
  class: handover
  date: 2026-09-20
  sha256: 44ec5b2d1b473282f96582d8d9a1d6cb217062e5fbf9fa537c3a58e97a896431
  status: live
-->
# Handover — Ernte-Folge 125 (2026-09-20)

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

## Stehender Pass (gemessen 2026-09-20, Folge 125)

- **HEAD** `5894b345` (forschung) beim Start; eigener Commit folgt. Arbeitsbaum:
  nur eigener Pfad-Satz (unten); fremde uncommittete Arbeit nicht anfassen.
- **Postfach** — kein neuer Mail-Ledger-Eingang seit `1789930255`; 3 `An ernte`-
  Zeilen gefaltet (Rosetta/Idempotenz, fmt-Rot `ps1_coverage_compiler`, `bq/l`→
  EPA-RadNet) und gelöscht; ein `An bau`-Post gesetzt (force-gate-B-Flush-Lücke).
- **CI-Status** — Watchdog-Snapshot 21:22: `ci-check` `35531572974`,
  `te-gate` `35531196101`, `ps1-cdn` `35530153972`, `free-model-bench`
  `35527517605` in_progress; `ci-check` `35527911970`/`35526010713` attempt-1 failed.

## DEMETER — Route blockiert (härtester undatierter Punkt)

- **`demeter-cdn` `35228716483` real gescheitert** (8 h, exit 1): 12× F5 ASM block
  (`Request Rejected`) auf `rs-order` create, dann curl exit 6
  (`Could not resolve host: regards.cnes.fr`); Token ok (286), 0/578 orders, 0 files;
  77 Shards nie registriert. Als `blocked ip-blocked` in `phi/blocked_sources.φ`
  eingetragen. Schritt: Runner-Egress + DNS `regards.cnes.fr` gegen CDPP messen
  (`research-max`), dann Re-Dispatch `gh workflow run demeter-cdn.yml`; bei success
  die 77 `url`+`sha256`-Zeilen registrieren.

## Ernte-Dropped-Audit (Rest)

- **EPA AQS map-key-Gap** (`daily_88101`, `sources.φ:1401`) — der „Arithmetic
  Mean"-`map`-Fix ist weder gebaut noch benannt. Schritt: `aqs_parser` gegen die
  2025-ZIP prüfen, Ergebnis als Verdikt eintragen.
- **GHRC GLM/TRMM** — Compiler + CDN ungebaut; öffentliche NOAA-AWS-Route messen,
  dann registrieren.
- **Quaoar sha256** (`zenodo.21185812`, 572 467 032 B) — Zenodo 504, sha256
  `pending`. Schritt: bei Rückkehr streamend `sfetch <url> | sha256sum`, dann
  `sources.φ` (TNBFits-Block-Nachbarschaft).

## Katalog-Inventar stale (Prozess-Gap)

- Der Digest zählt die 192 bereits disponierten Kandidaten weiter als
  `candidate → ernte` (die gitignored `phi/pipeline/catalog/*.φ` tragen die
  Zeilen unverändert). Schritt: Digest gegen die Register deduplizieren oder die
  Katalogzeile bei Disposition mitführen.

## Aus dem Postfach gefaltet (offen)

- **EPA RadNet ERM_RESULT** (`bq/l`/`bq/m3` in `units.rs` vorhanden, force-0-Liste)
  — Registrierung als em/Bq/L kann laufen. Schritt: ERM_RESULT als em/Bq/L in
  `sources.φ` registrieren (Parser + CDN-Manifestation).
- **fmt-Rot** `ci-check` `35531572974` @`8218f46a`: `tools/harvest/src/bin/ps1_coverage_compiler.rs`
  (Z. 605,644) nicht rustfmt-konform (letzter Pfad-Commit `4502dbfa`). Schritt:
  eigene Datei fmt-sauber machen, dann `ci-check` dispatch.
- **Rosetta ungelaufene Pfade / Idempotenz-Gate** (`wartend`) — letzte Nennung
  `handover-2026-09-17-ernte-folge75.md`, `idempotenz` 0 Treffer in rs/φ, kein
  zentraler Trigger in `external-state.md`. Schritt: Auslöser benennen.

## PS1

- **PS1-Ernte-Rate** — `wartend` auf `ps1-cdn`; Verdikt offen. Schritt:
  `ci_manage list` nach `ps1-cdn`, einmalig `ci_manage log <id>`.
- **PS1-Fraktional Order-10-Final** — Größe aus dem Combine-Log → `footprints.φ`.

## Wartend (kein Auswahlpunkt)

- **TAP-Backends dachs.fai.kz + pithia.cbk.waw.pl** (`ledger.φ`) — Trigger sync-QUERY/tables 500→200.
- **SSDC Limadou** — operator-gebunden (PI-Freigabe).
- **Lasair-LSST API** — Backend 502; Trigger Erholung.
- **Sonden-Antworten** (Voyager/Mariner/Viking/Juno) `blocked_sources.φ`.
- **BepiColombo bc_mpo_more** — Freigabe ~April.
- **NRS02-10,12,13 SHAPE** `nrs_stations.φ:17` — Trigger neuer Stations-Prefix.
- **Babamul / IA2 TAP** `blocked_sources.φ` — kein gebauter Konsument → `pending`.
- **Split-Routing-Verifikation** — operator-gebunden; als `An entscheid`-Post gesetzt.

## Termin

- **EMODnet HFRADAR NADR** — nächste Re-Messung **2026-10-19**.

## Ausgelagerte Fremd-Owner-Punkte (nicht im eigenen Handover)

- post_body-Migration → bau (`post.md`).
- units.rs `bq/l` → bau (`post.md`).
- Split-Routing → entscheid (`post.md`).
- force-gate-B-Flush-Lücke (`port_mode`) → bau (`post.md`).

## Benchmark

- **Ernte-Folge 125** — `grind-flash` (Survivor-Recovery: 7 Probe-Läufe, 358
  verifiziert / 164 neu; DEMETER-Log; 3-Korpora-Re-Run), `grind-pro`
  (Survivor-Registrierung: 2 → sources.φ, 126 → declined_sources.φ, 52 skipped),
  Verifikation am Baum. Kein sauberer flash/pro-Vergleich; Burn nicht gemessen.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** `phi/sources.φ` (+2 wolfx-Blöcke), `phi/declined_sources.φ`
  (+126 Declines), `phi/pipeline/ledger.φ` (7 Review + 3 ausstehend disponiert),
  `phi/blocked_sources.φ` (+DEMETER `blocked ip-blocked`), `docs/handover/post.md`
  (3 `An ernte` entfernt, 1 `An bau` gesetzt), `src/archivar/port.rs`
  (probe schreibt `{input}.survivors.φ`), neues Handover
  `handover-2026-09-20-ernte-folge125.md`, Move folge124 → `archiv/`.
- Nie ein nacktes `git commit`; committet wird pfad-begrenzt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent, nie das Commit-Wort.
