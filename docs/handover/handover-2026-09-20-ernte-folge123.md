<!--
  title: Handover — Ernte-Folge 123 (Stand 2026-09-20)
  session: Ernte-Folge 123
  class: handover
  date: 2026-09-20
  sha256: PLACEHOLDER
  status: live
-->
# Handover — Ernte-Folge 123 (2026-09-20)

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

## Stehender Pass (gemessen 2026-09-20, Folge 123)

- **HEAD** `0fcec24f` (entscheid folge66 / bau folge113) beim Start, ==
  `origin/main`; eigener Commit folgt. Arbeitsbaum: fremde uncommittete Arbeit
  `tools/register/src/bin/register_lookup.rs` (+472, andere Linie) — nicht
  anfassen; eigener Pfad-Satz unten.
- **Postfach** — 2 ernte-Zeilen aus `post.md` gefaltet (unten). Pine64 Ox64
  erledigt (Operator hat Versanddaten gegeben, Antwort gesendet — von entscheid
  geschlossen). Kein neuer Mail-Ledger-Eingang seit `1789930255`.
- **CI-Status** — `ci-check` `35531572974` in_progress; `ps1-cdn` `35531164844`
  @`4502dbfa` pending (Runner-Rückstau, kein Polling). Der in Folge 122
  registrierte Lauf `35531159323` wurde **cancelled** und vom Nachfolger
  `35531164844` (gleicher HEAD) ersetzt.

## Queue-Korpora — Survivor-Review/Disposition (härtester undatierter Punkt)

- **368 Survivor aus 7 Korpora** (`phi/pipeline/ledger.φ:70–96`, Zustand
  `verifiziert`; 13k 148, 14k 25, 15k 156, 183l 4, 2k 3, 7k 21, staging 11).
  Schritt: `phi/pipeline/probe_survivors.φ` lesen, Dedupe gegen `phi/sources.φ`,
  Oszillator-Gate, dann `sources.φ`/`dead_sources.φ` + Ledger fortschreiben
  (SOURCE_PORT §5.4).

## Queue-Korpora — 3 ausstehend (force-gate B gebaut)

- **30-astro, earth-stac-sentinel, exotic-neutrino-ligo** (`ledger.φ:58–66`,
  `ausstehend`) — Block ohne `force`-Direktiv fällt nicht mehr still, bleibt
  `# pending … review` (`src/archivar/port.rs` `field_or_review`). Schritt:
  Re-Lauf auf frischem Binär (`bin/.tools_ensure omegaflow`), `# pending`-Zeilen
  zählen, dann Disposition (SOURCE_PORT §5.4).

## Katalog-Kandidaten — 2 accept review-pflichtig

- **GWOSC `eventapi/jsonfull/allevents/`** (Force `gravity`, GWTC-Event-Katalog)
  und **EPA RadNet `ERM_RESULT/rows/0:100/JSON`** (Force `em`, Cs-137/I-131 in
  Bq/L). Beide 2026-09-20 HTTP 200 gemessen; Extract-Pfad und Feldeinheit sind
  schema-unverifiziert. Schritt: Feldstruktur messen (`sfetch`/`archive_search
  --playwright`), dann `sources.φ`-Block nach §6.

## Katalog-Inventar stale (Prozess-Gap)

- ~90 % der 192 Kandidaten waren bereits disponiert, standen aber weiter als
  `candidate` in den gitignored `phi/pipeline/catalog/*.φ`; der Digest zählt sie
  weiter. Schritt: die Katalogzeile bei Disposition auf ihren Zustand setzen oder
  den Digest gegen die Register deduplizieren.

## PS1 (wartend)

- **PS1-Ernte-Rate** — wartend auf `ps1-cdn` `35531164844` @`4502dbfa` (pending).
  Schritt: `ci_manage log 35531164844` einmalig nach Abschluss.
- **PS1-Fraktional Order-10-Final** — wartend; Order-10-Final-Größe aus dem
  Combine-Log → `asset`-Zeile in `footprints.φ`.

## Wartend (kein Auswahlpunkt)

- **TAP-Backends dachs.fai.kz + pithia.cbk.waw.pl** `ledger.φ:10-20` — Trigger
  sync-QUERY 500→200.
- **Lasair-LSST API** — Backend 502; Trigger Erholung.
- **Sonden-Antworten** (Voyager/Mariner/Viking/Juno) `blocked_sources.φ`.
- **BepiColombo bc_mpo_more** — Freigabe ~April.
- **NRS02-10,12,13 SHAPE** `nrs_stations.φ:17` — Trigger neuer Stations-Prefix.
- **Babamul / IA2 TAP / GHRC** `blocked_sources.φ` — kein gebauter Konsument →
  `pending`.

## Termin

- **EMODnet HFRADAR NADR** `ledger.φ:22-24` — nächste Re-Messung **2026-10-19**.

## Ausgelagerte Fremd-Owner-Punkte (nicht im eigenen Handover)

- post_body-Migration → bau (`post.md`).
- queue-Korpora astro/earth/exotic (Operator-Wort/Lauf-Ort) → entscheid.

## Benchmark

- **Ernte-Folge 123** — Katalog-Kandidaten-Dedupe/Disposition auf `grind-flash`
  (2 Agenten parallel) + Register-Fold auf `grind-flash`; kein pro/max-Doppellauf,
  weil die flash-Antworten vollständig und korrekt waren. Burn nicht gemessen.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** `phi/declined_sources.φ` (+102 Blöcke, **nur angehängt** —
  Bestand unberührt, url-Multiset gegen HEAD geprüft), `phi/blocked_sources.φ`
  (+3), `phi/dead_sources.φ` (+11), `docs/handover/post.md` (2 ernte-Zeilen
  entfernt), neues Handover `handover-2026-09-20-ernte-folge123.md`, Move
  folge122 → `archiv/`.
- **Fremd (nicht anfassen):** `tools/register/src/bin/register_lookup.rs`
  (uncommittet). Nie ein nacktes `git commit`; committet wird pfad-begrenzt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent, nie das Commit-Wort.
