<!--
  title: Handover — Entscheid-Folge II (Stand 2026-09-11)
  session: Entscheid-Folge II
  class: handover
  date: 2026-09-11
  sha256: b722362804cb8d3f055021fefa689f828f54d4b8cc73e9fc2b159158e8fc1b1d
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

## GitHub-Konto (Kern der Session 2026-09-12)

- Sicherheit — der im Chat exponierte Token `ghp_KHhu…` ist zu widerrufen
  (https://github.com/settings/tokens). Falls er der aktive `gh`-Token ist,
  vorher einen frischen anlegen und `gh auth login -h github.com --with-token`
  damit; sonst bricht der Push.
- `ivoa` — `omegaflow` ist kein Mitglied mehr (beim Umwandlungsversuch
  entfernt). Markus-Brief (Re-Invite `johannestyroller`/`omegaflow`) ist raus,
  Antwort offen.
- ToS / User→Org — self-serve blockiert: GitHub retired beim Rename die alten
  Repo-Namen (`omegaflow/omegaflow`, `omegaflow/sources`,
  `omegaflow/omegaflow-legacy`; Transfer → HTTP 422). Support-Text ist
  formuliert → senden. Fallback: zwei Personenkonten (`omegaflow` Owner,
  `johannestyroller` privat) bewusst führen.
- Zustand verifiziert (2026-09-12): `gh` = `omegaflow` (Token-Weg, Scopes
  `admin:org, repo, workflow`), volle Rechte auf allen drei Repos, lokales Repo
  sauber (`HEAD == origin/main`), `git push --dry-run` grün. Org `omegaflow-tmp`
  (Rest des Fehlversuchs) ist gelöscht.

## Votable-TAP

- Die 22 `blocked parser-def votable` sind als **ein** `pending`-Eintrag in
  `phi/blocked_sources.φ` konsolidiert (Commit a5d9384; Messung 2026-09-11:
  TABLEDATA via `<root>/sync` + `FORMAT=votable`, Haus-Parser `votable_rows`
  konsumiert; LIneA = Schema-Name, CADC-ARGUS = `votable` ja/`votable/td` nein).
  Offen: Ernte-Welle — je Katalog Feldblock (TAP_SCHEMA-Durchgang) + Compiler +
  CDN-Manifestation; Eintritt in `sources.φ` erst mit gemessenem Feldblock.

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
