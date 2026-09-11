<!--
  title: Handover — Entscheid-Folge II (Stand 2026-09-11)
  session: Entscheid-Folge II
  class: handover
  date: 2026-09-11
  sha256: bdfee87d8a9264f908a38e2fe1de8615cbbb0c787c508f9fc71cace2028c7550
  status: live
-->
# Handover — Entscheid-Folge II (2026-09-11)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab, wie sie kann — Sub-Agenten tragen eigenen Kontext, die Anzahl
ist kein Aufwand. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

Jede Zeile trägt ihr Wiedervorlage-Datum — vor dem Datum wird sie nicht erwähnt;
nur fällige Zeilen (Datum ≤ heute) kommen auf den Tisch.

## Wiedervorlage

- 2026-09-14 — Desktop-Fork (GTX 970): 30-Jahres-Lauf.
- 2026-09-14 — de441 Re-Verifikation: der jup365-OOM-Fix (DafFile `pread`) liegt
  auf `main` (f67b14d); der nächste `kernel-flatten`-Dispatch verifiziert
  (bodies-Job grün + jup365-Release am CDN).
- 2026-09-14 — ned-Crawl: Void-Kegel-Fix + Listing-Void-Abbruch auf `main`; CDN
  1/40 Slices (gemessen 2026-09-11), nächster Cron-Lauf.
- 2026-09-14 — Token-Tausch verifizieren: `OMEGAFLOW_TOKEN` trägt jetzt einen
  classic PAT von `johannestyroller` (`public_repo`, eigenes 5.000/h-Bucket;
  API-gemessen 2026-09-11: `push:true` auf `omegaflow/sources`, Bucket frisch
  4999/5000). Der nächste `kernel-flatten`-Dispatch verifiziert (grün + kein
  Rate-Limit-Hit).
- 2026-09-14 — CI-Dedup (Endzustand der Concurrency-Arbeit, Commit 5d80061):
  `lead-geometry-cdn` soll das CDN-Asset `physionet.org/mitdb_arrhythmia.bin`
  konsumieren (`body_url`) statt es neu zu bauen; NOAA-Einzelstation
  (`noaa-ghcn/gsod/isd-cdn`) messen, ob sie in den 16 Shards der `-allstations`
  liegt — ja: falten, nein: Asset-Namen trennen (Namensverletzung). Bis dahin
  tragen die Namespace-Gruppen `mitdb-arrhythmia` / `noaa-*-assets`.
- 2026-09-14 — CI-Pending-Scans: Upload-Ziele der Probe-Workflows (`te-*`,
  `eikonal-tohoku`, …) lagen nicht im Contention-Scan — unverifiziert; die
  `17 5 1 * *`-Überlappung `kernel-flatten`/`signal-cone-audit` ist nur unter
  der unverifizierten Annahme „Audit liest nur" benannt.
- 2026-09-15 — CI-Sättigung messen: `ps1-cdn` + `allwise-cdn` (je ~180 min,
  stündlicher Cron) — Kadenz gegen Actions-Minuten/Rate-Limit messen; bewusst
  kein Per-Tag-Gate für `ssd.jpl.nasa.gov` (Drain 415 min/h verhungert).
- 2026-09-15 — LISA Pathfinder: Selbstregistrierung ab 15.09.
- 2026-09-16 — ned-objdir: IPAC-Auto-Bestätigung 2026-09-09, Antwort offen.
- 2026-09-18 — NOIRLab Data Lab: `jtyroller` registriert 2026-09-11, wartet auf
  menschliche Freigabe.
- 2026-09-18 — Rubin RSP-Datenrechte: Antrag in Prüfung.
- 2026-09-18 — papier-kleinpass (nach Merge).
- 2026-09-18 — sicherung-risiko-heime: wöchentliche Kopie.
- 2026-09-22 — AllWISE: CDN-Stand; `allwise_coverage.fp01` nach Abschluss
  verifizieren.
- 2026-09-25 — TOAR-Vollzugang: Jülich-Antwort (Schröder).
- 2026-09-25 — TNO-Kette: keine MPC-unabhängige Linie (not-published).
- 2026-09-28 — Nadel Ⅱ: JUICE-Flyby 28./29.9.
- 2026-12-02 — Nadel Ⅰ: Jeans-Residuum bis Gaia DR4.
- 2026-12-02 — gaia-dr4-iapetus.
- 2026-12-03 — Nadel Ⅱ: Europa Clipper 3.12.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check (`/abschluss`).
