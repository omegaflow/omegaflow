<!--
  title: Handover — Ernte-Folge 119 (Stand 2026-09-20)
  session: Ernte-Folge 119
  class: handover
  date: 2026-09-20
  sha256: 56051858ad8bf39ff7c33a98a319b274884066a548f155e4b00614831c616258
  status: live
-->
# Handover — Ernte-Folge 119 (2026-09-20)

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

## Stehender Pass (gemessen 2026-09-20, Folge 119)

- **HEAD** `cf6b9799` (forschung folge119), == `origin/main`. Arbeitsbaum: fremde
  entscheid-Arbeit staged (`R handover-2026-09-20-entscheid-folge63.md` → `archiv/`,
  `?? handover-2026-09-20-entscheid-folge64.md`) — **nicht angefasst**. Eigener
  Pfad-Satz: `phi/pipeline/probe_wave.φ`, `phi/pipeline/probe_hapi_proposed.φ`,
  `phi/footprints.φ:18`, neues Handover folge119, Move folge118 → `archiv/`.
- **Postfach** — `state/mail/mail_ledger.φ` max-Timestamp `1789922257`
  (Pine64-Antwort auf die Hardware-Anfrage — kein Ernte-Bezug); der Zustand-Ledger
  nennt `1789918147` (forschung-geführt, um einen Eingang stale).
- **CI-Status** — Ernte-relevant: `harvest` skip-10 `35525657301` @`8172288e`
  **in_progress** (17:22:26Z), `harvest-dispatch` `35525623841` **success**;
  skip-9 `35524334872` @`9f9201ea` success (Übergabe folge118 nannte @`66579fc3`
  — Registerfehler). Repo-weit `ci-check` rot (Bau-Domäne).

## Offen

- **PS1-Fraktional volles DR2-Asset** `footprints.φ:18` — measured 2026-09-20:
  Release `ssd.jpl.nasa.gov` am 1000-Asset-Cap, `gh release upload` HTTP 422,
  187 Parts, `combine` nie erreicht; kein `ps1_dr2_coverage.fp01`. Schritt: neues
  Release-Tag (z. B. `ps1-dr2`) + `.github/workflows/ps1-cdn.yml` Repoint
  (Zeilen 30/76/84/87) — oder die 813 Fremd-Assets prunen. `eigen`.
- **probe_hapi_proposed.φ superseded** — alle 7 Blöcke sind in `sources.φ`
  registriert (:1 `CS_OPER_MAG`, :15 KBR, :22 `CH_DNS`, :28 VTEC, :34 `CH_WND`,
  :40 `GR_DNS`, :46 `GO_MAG`); die Datei steht im Kanon (`phi/canon.φ:46`) →
  Austrag/Behalten ist ein Architektur-Akt (Operator/Council-Wort). `operator-gebunden`.
- **Distance-KBR (riss)** — `probe_wave.φ` hält die Spalte pending, `sources.φ:19`
  registriert `inverse-square gravity m` (kanonisch, maßgeblich). Die Manifestation
  folgt dem Register; die Probe-Notiz ist überholt, nicht geglättet. `operator-gebunden`.

## Wartend (kein Auswahlpunkt)

- **skip-10 harvest** `35525657301` @`8172288e` — Trigger Run-Abschluss
  (`ci_manage view 35525657301`).
- **TAP-Backends dachs.fai.kz + pithia.cbk.waw.pl** `ledger.φ:10-20` — PostgreSQL
  `localhost:5432` tot; Trigger sync-QUERY 500→200 bzw. `/tap/tables` 500→200.
- **Lasair-LSST API** — Backend 502 über Proton; Trigger Erholung.
- **Sonden-Antworten** (Voyager/Mariner/Viking/Juno) `blocked_sources.φ:39-53`.
- **BepiColombo bc_mpo_more** `blocked_sources.φ:26-29` — Freigabe ~April.
- **NRS02-10,12,13 SHAPE** `nrs_stations.φ:17` — measured 2026-09-20: GCS-Listing
  `storage.googleapis.com/storage/v1/b/noaa-passive-bioacoustic/o?prefix=nrs/products/sound_level_metrics/&delimiter=/`
  trägt nur `01/`,`11/`; Trigger neuer Stations-Prefix.
- **Babamul / IA2 TAP / GHRC** `blocked_sources.φ` — kein gebauter Konsument → `pending`.

## Termin

- **EMODnet HFRADAR NADR** `ledger.φ:22-24` — nächste Re-Messung **2026-10-19**.

## Ausgelagerte Fremd-Owner-Punkte (nicht im eigenen Handover)

- post_body-Migration → bau (`post.md`).
- queue-Korpora astro/earth/exotic (Operator-Wort/Lauf-Ort) → entscheid (`post.md`).

## Benchmark

- **Ernte-Folge 119** — Delegationen: `grind-pro` (Force-Gate, $0.1314) für die
  41+2 Probe-Marker; 2× `grind-flash` (PS1-Verify, NRS-Messung, zusammen $0.0369).
  Kein Doppellauf — Force-Gate ist die urteilende Klasse (`grind-pro`), die zwei
  Messungen mechanisch (flash). Sieger steht.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** `phi/pipeline/probe_wave.φ` (41 Marker aufgelöst: 8 field,
  29 declined, 2 pending; 15 Kernel int→name; 2 Dichte `erfc`→`gaussian-inverse-square`),
  `phi/pipeline/probe_hapi_proposed.φ` (2 pending aufgelöst; superseded),
  `phi/footprints.φ:18` (PS1-Blocker gemessen), neues Handover
  `handover-2026-09-20-ernte-folge119.md`, Move folge118 → `archiv/`.
- **Fremd (nicht anfassen):** staged `R handover-2026-09-20-entscheid-folge63.md`
  → `archiv/`, `?? handover-2026-09-20-entscheid-folge64.md` (entscheid-Linie).
  Nie ein nacktes `git commit`; committet wird pfad-begrenzt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent, nie das Commit-Wort.
