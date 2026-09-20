<!--
  title: Handover — Ernte-Folge 120 (Stand 2026-09-20)
  session: Ernte-Folge 120
  class: handover
  date: 2026-09-20
  sha256: 5a454ad6a7cea45bf51104f3f2f108dead43f33037eb2dbc17a5cc2dc531d78b
  status: live
-->
# Handover — Ernte-Folge 120 (2026-09-20)

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

## Stehender Pass (gemessen 2026-09-20, Folge 120)

- **HEAD** `e7e91da5` (fremde Linie nach Ernte folge119) — Arbeitsbaum trägt
  fremde Arbeit: staged `R entscheid-folge63 → archiv/`, `R bau-folge110 →
  archiv/`, `M post.md`, `M zustand/external-state.md`, `?? entscheid-folge64`,
  `?? bau-folge111` — **nicht angefasst**. Eigener Pfad-Satz: `phi/sources.φ`,
  `phi/canon.φ`, `phi/pipeline/probe_hapi_proposed.φ` (gelöscht), neues Handover
  folge120, Move folge119 → `archiv/`.
- **Postfach** — `state/mail/mail_ledger.φ` max `1789922257` (Pine64-Antwort,
  kein Ernte-Bezug).
- **CI-Status** — Ernte-relevant: `harvest` skip-10 `35525657301` @`8172288e`
  **in_progress** (17:22:26Z); `harvest-dispatch` `35525623841` success.
  Repo-weit `ci-check` rot (Bau-Domäne).

## Offen

- **PS1-Fraktional volles DR2-Asset** `footprints.φ:18` — measured 2026-09-20
  (Folge 120): Release `ssd.jpl.nasa.gov` am 1000-Asset-Cap (187 ps1-Parts + 813
  fremd); alle 187 sind **chunked** (`ps1_part_637_0_9.fp01` …), kein Ganzband →
  Ernte bei ~19 von ~2000+ Bändern; ein neues Tag (1000er-Cap) schließt die Ernte
  **nicht**. Fix ist ein Sharding-Design (mehrere Releases nach Bandbereich ODER
  gröbere Parts) — Architektur-Wort/Council. Schritt: Design entscheiden, dann
  Compiler `upload_asset` (`src/archivar/cdn.rs:35`) + `.github/workflows/ps1-cdn.yml`.
  `operator-gebunden`.

## Wartend (kein Auswahlpunkt)

- **skip-10 harvest** `35525657301` @`8172288e` — Trigger Run-Abschluss
  (`ci_manage view 35525657301`).
- **TAP-Backends dachs.fai.kz + pithia.cbk.waw.pl** `ledger.φ:10-20` — PostgreSQL
  `localhost:5432` tot; Trigger sync-QUERY 500→200 bzw. `/tap/tables` 500→200.
- **Lasair-LSST API** — Backend 502 über Proton; Trigger Erholung.
- **Sonden-Antworten** (Voyager/Mariner/Viking/Juno) `blocked_sources.φ:39-53`.
- **BepiColombo bc_mpo_more** `blocked_sources.φ:26-29` — Freigabe ~April.
- **NRS02-10,12,13 SHAPE** `nrs_stations.φ:17` — GCS-Listing trägt nur `01/`,`11/`;
  Trigger neuer Stations-Prefix.
- **Babamul / IA2 TAP / GHRC** `blocked_sources.φ` — kein gebauter Konsument → `pending`.

## Termin

- **EMODnet HFRADAR NADR** `ledger.φ:22-24` — nächste Re-Messung **2026-10-19**.

## Ausgelagerte Fremd-Owner-Punkte (nicht im eigenen Handover)

- post_body-Migration → bau (`post.md`).
- queue-Korpora astro/earth/exotic (Operator-Wort/Lauf-Ort) → entscheid (`post.md`).

## Benchmark

- **Ernte-Folge 120** — Delegationen: 1× `grind-flash` (PS1-Release-Zustand:
  1000-Cap, 187 chunked Parts gemessen) und 1× `council` (Konfund zu Act A/B).
  Kein Doppellauf — die PS1-Messung ist mechanisch (flash), die Architektur-Akte
  sind urteilend (council). Sieger steht.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** `phi/sources.φ` (CS_OPER_MAG-Fenster 2018→2026-07-31
  gefaltet; KBR-Einheiten-Note), `phi/canon.φ` (Zeile 46 gestrichen),
  `phi/pipeline/probe_hapi_proposed.φ` (gelöscht), neues Handover
  `handover-2026-09-20-ernte-folge120.md`, Move folge119 → `archiv/`.
- **Fremd (nicht anfassen):** staged `R handover-2026-09-20-entscheid-folge63.md`
  → `archiv/`, `R handover-2026-09-20-bau-folge110.md` → `archiv/`,
  `?? handover-2026-09-20-entscheid-folge64.md`, `?? handover-2026-09-20-bau-folge111.md`,
  `M post.md`, `M docs/zustand/external-state.md`. Nie ein nacktes `git commit`;
  committet wird pfad-begrenzt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent, nie das Commit-Wort.
