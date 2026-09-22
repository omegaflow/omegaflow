<!--
  title: Handover — Ernte-Folge 122 (Stand 2026-09-20)
  session: Ernte-Folge 122
  class: handover
  date: 2026-09-20
  sha256: f95ca50326e6fe986aebeab4a5a48ffbf617678655b603a9cd269f7cb74cbe7b
  status: live
-->
# Handover — Ernte-Folge 122 (2026-09-20)

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

## Stehender Pass (gemessen 2026-09-20, Folge 122)

- **HEAD** `db9f489c` (forschung folge122) beim Start, == `origin/main`; eigener
  Commit folgt. Arbeitsbaum: fremde uncommittete Arbeit der entscheid-Linie
  (`handover-2026-09-20-entscheid-folge65.md` → `archiv/`, neues folge66) —
  nicht anfassen; eigener Pfad:
  `tools/harvest/src/bin/ps1_coverage_compiler.rs`.
- **Postfach** — letzter Ledger-Eingang `1789930255` (`info@pine64.org`: Ox64
  zugesagt, bittet um Versanddaten + Telefon — operator-gebunden; von
  Forschung-Folge 122 in `post.md` + zustand gefaltet). Kein neuer Eingang seit
  `1789930255`.
- **CI-Status** — der zustand-Eintrag (Forschung-Folge 122, HEAD-Wechsel) ist
  aktuell. ps1-cdn: `35530153972` @`e09a996b` in_progress, `35530552549`
  @`cd742454` pending (Vor-Fix-HEADs); **Neu-Dispatch `35531159323` @`4502dbfa`**
  (dieser Commit, nach Push) — Verdikt offen, einmalig lesen.

## PS1-Ernte-Rate — sequentieller Plane-Fetch (Engpass gemessen)

- Gemessen (grind-flash, Code + Workflow): `harvest_skycell` holte die 5 Bänder
  g,r,i,z,y streng sequentiell (`curl -sSf -m 300` + `regrid_plane`);
  `PROBE_WORKERS=8` parallelisierte nur die g-Band-Enumeration, nicht die
  Downloads — das war der Engpass. `curl_bytes` hatte keinen Retry: ein
  transienter 429/503 fiel als `Refused` weg, der Chunk wurde dennoch
  geschrieben/hochgeladen — ein stilles Coverage-Loch.
- **Offen (eigen, wartend auf `ps1-cdn` `35531159323` @`4502dbfa`):** die Rate
  und die Loch-Freiheit am lebenden Lauf verifizieren; `PLANE_WORKERS=4` ggf.
  tunen. Schritt: `ci_manage log 35531159323` (Parts je Fenster +
  `planes_unreadable`-Abort prüfen) — einmalig, nie pollen.

## PS1-Fraktional Order-10-Final (wartend)

- Das Slab-Sharding steht (`cd742454`); der erste ps1-cdn-Lauf (`35530153972`
  @`e09a996b`, `35530552549` @`cd742454`) trägt noch kein Verdikt. Offen: beim
  Combine die Order-10-Final-Größe messen → `asset`-Zeile in `footprints.φ`.
  Schritt: `ci_manage view 35530153972` / `35530552549` / `35531159323`; Größe
  aus dem Final-Combine-Log.

## Wartend (kein Auswahlpunkt)

- **TAP-Backends dachs.fai.kz + pithia.cbk.waw.pl** `ledger.φ:10-20` —
  PostgreSQL `localhost:5432` tot; Trigger sync-QUERY 500→200 bzw.
  `/tap/tables` 500→200.
- **Lasair-LSST API** — Backend 502 über Proton; Trigger Erholung.
- **Sonden-Antworten** (Voyager/Mariner/Viking/Juno) `blocked_sources.φ:39-53`.
- **BepiColombo bc_mpo_more** `blocked_sources.φ:26-29` — Freigabe ~April.
- **NRS02-10,12,13 SHAPE** `nrs_stations.φ:17` — Trigger neuer Stations-Prefix.
- **Babamul / IA2 TAP / GHRC** `blocked_sources.φ` — kein gebauter Konsument →
  `pending`.

## Termin

- **EMODnet HFRADAR NADR** `ledger.φ:22-24` — nächste Re-Messung **2026-10-19**.

## Ausgelagerte Fremd-Owner-Punkte (nicht im eigenen Handover)

- post_body-Migration → bau (`post.md`).
- queue-Korpora astro/earth/exotic (Operator-Wort/Lauf-Ort) → entscheid
  (`post.md`).

## Benchmark

- **Ernte-Folge 122** — Analyse und Implementierung liefen beide auf `grind-flash`
  (billigster tragender Vertreter); kein pro/max-Doppellauf, weil die
  flash-Antwort vollständig und korrekt war. Burn Implementierung $0.0135.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** `tools/harvest/src/bin/ps1_coverage_compiler.rs` (paralleler
  Plane-Pool `PLANE_WORKERS=4` + curl-Retry + Abort bei `planes_unreadable`),
  neues Handover `handover-2026-09-20-ernte-folge122.md`, Move folge121 →
  `archiv/`.
- **Fremd (nicht anfassen):** entscheid-Move (`folge65` → `archiv/`, `folge66`).
  Nie ein nacktes
  `git commit`; committet wird pfad-begrenzt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent, nie das Commit-Wort.
