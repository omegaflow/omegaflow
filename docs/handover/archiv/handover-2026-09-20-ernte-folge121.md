<!--
  title: Handover — Ernte-Folge 121 (Stand 2026-09-20)
  session: Ernte-Folge 121
  class: handover
  date: 2026-09-20
  sha256: 207328e99d1ecf6653eb3013014f3d4cb4f54ecbe984dfdf0fa066217a8e1a42
  status: live
-->
# Handover — Ernte-Folge 121 (2026-09-20)

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

## Stehender Pass (gemessen 2026-09-20, Folge 121)

- **HEAD** `e09a996b` (entscheid folge65) beim Start; während der Session rückte
  HEAD auf `f75e3245` (forschung folge121, fremd — `origin/main` bleibt
  `e09a996b`). Arbeitsbaum **sauber** (`git_safety --snapshot`: the working tree
  equals HEAD). Eigener Pfad-Satz: `src/archivar/cdn.rs`,
  `tools/harvest/src/bin/ps1_coverage_compiler.rs`, `.github/workflows/ps1-cdn.yml`,
  `phi/footprints.φ:18`, neues Handover folge121, Move folge120 → `archiv/`.
- **Postfach** — `state/mail/mail_ledger.φ` max `1789922257` (Pine64-Antwort,
  kein Ernte-Bezug); kein neuer Eingang seit folge120.
- **CI-Status** — `harvest` skip-10 `35525657301` @`8172288e` **completed
  success** (der folge120-Wartend-Trigger ist gefeuert und geschlossen);
  Watchdog 20:18Z: `free-model-bench` `35527517605` + `ci-check` `35526010713`
  in_progress, `ci-check` `35523145456` failed (Bau-Domäne).

## PS1-Fraktional volles DR2-Asset — Sharding gebaut (Council-Wort 2026-09-20)

- `footprints.φ:18` gemessen folge120: Release `ssd.jpl.nasa.gov` am
  1000-Asset-Cap (187 ps1 + 813 fremd), `gh release upload` HTTP 422, combine nie
  erreicht; kein `ps1_dr2_coverage.fp01`.
- **Council-Wort (2026-09-20):** Slab-Tags `ps1-dr2-<lo>` à 80 Bänder (26 Tags,
  ≤1000 Assets je Tag), chunk=10 bleibt; **Order 10 (Nside 1024)** gegen die
  2-GB-Dateigrenze (AllWISE-Präzedenz); hierarchischer Combine chunk → band →
  final; die 187 Order-12-Parts descoped (kein Merge in ein Order-10-Final).
- **Gebaut:** `src/archivar/cdn.rs` `ps1_slab_tag` + `PS1_SLAB_BANDS`;
  `ps1_coverage_compiler.rs` `--order` (Default 12, Band-Grenzen-Guard) + Upload
  auf den Slab-Tag; `.github/workflows/ps1-cdn.yml` neu (Slab-Listing-Cache,
  Band-Combine mit Chunk-Abbau, Final-Combine, Legacy-Order-12-Cleanup).
- **Offen (eigen):** nach Push `gh workflow run ps1-cdn.yml` dispatchen; erster
  Lauf legt die Slab-Releases an und erntet Order-10-Chunks. Schritt:
  `gh workflow run ps1-cdn.yml` → `ci_manage view <id>`; die Order-10-Final-Größe
  beim Combine messen und als `asset`-Zeile in `footprints.φ` tragen, wenn der
  Combine landet.

## Offen

- **PS1-Ernte-Rate** — ~14 Parts/Tag (folge119 gemessen) → Volle-Ernte ist eine
  mehrjährige Serie; der Autoresume trägt sie. Schritt: Plane-Fetch-Parallelität
  + Timeout-Budget messen (`ps1_coverage_compiler.rs` `PROBE_WORKERS`, `curl -m`),
  dann ggf. anheben. `eigen`.

## Wartend (kein Auswahlpunkt)

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

- **Ernte-Folge 121** — das Council (pro/max) entschied das PS1-Sharding-Design;
  die Implementierung ist line-eigen (kein flash-Doppellauf in diesem Atom).
  Sieger für die Architektur: Council.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** `src/archivar/cdn.rs` (`ps1_slab_tag`),
  `tools/harvest/src/bin/ps1_coverage_compiler.rs` (`--order`, Slab-Upload),
  `.github/workflows/ps1-cdn.yml` (Slab-Sharding + hierarchischer Combine),
  `phi/footprints.φ:18` (Council-Design-Note), neues Handover
  `handover-2026-09-20-ernte-folge121.md`, Move folge120 → `archiv/`.
- **Fremd (nicht anfassen):** `f75e3245` (forschung folge121, committet) —
  `docs/zustand/external-state.md`, `src/archivar/extract.rs` u. a.; die Zeile
  im Register wurde von der Forschung-Linie aktuell gehalten. Nie ein nacktes
  `git commit`; committet wird pfad-begrenzt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent, nie das Commit-Wort.
