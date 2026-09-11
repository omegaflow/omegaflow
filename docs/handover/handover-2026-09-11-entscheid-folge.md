<!--
  title: Handover — Entscheid-Folge (Stand 2026-09-11)
  session: Entscheid-Folge
  class: handover
  date: 2026-09-11
  sha256: d5a728030279bf7319359fdb9518e6b9f60ae9635da87c971ca5e8ff2d687011
  status: live
-->
# Handover — Entscheid-Folge (2026-09-11)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab, wie sie kann — Sub-Agenten tragen eigenen Kontext, die Anzahl
ist kein Aufwand. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

Jede Zeile trägt ihr Wiedervorlage-Datum — vor dem Datum wird sie nicht erwähnt;
nur fällige Zeilen (Datum ≤ heute) kommen auf den Tisch.

## Wartet auf Operator-Wort (sofort machbar, kein Datum)

- Desktop-Fork (GTX 970): 30-Jahres-Lauf.

## Wiedervorlage

- 2026-09-14 — de441 Re-Verifikation: der jup365-OOM-Fix (DafFile `pread`) liegt
  auf `main` (f67b14d); der nächste `kernel-flatten`-Dispatch verifiziert
  (bodies-Job grün + jup365-Release am CDN).
- 2026-09-14 — ned-Crawl: Void-Kegel-Fix + Listing-Void-Abbruch auf `main`; CDN
  1/40 Slices (gemessen 2026-09-11), nächster Cron-Lauf.
- 2026-09-15 — Token-Tausch verifizieren: `OMEGAFLOW_TOKEN` ersetzt durch einen
  PAT von `johannestyroller` (eigenes 5.000/h-Bucket pro Konto, gemessen
  2026-09-11; App bringt bei 2 Repos nichts — gleiches Mindestkontingent).
  Operator-Schritte: Collaborator-write auf `omegaflow/sources` + PAT
  (Contents-write) + Secret tauschen. Nächster `kernel-flatten`-Dispatch
  verifiziert (grün + kein Rate-Limit-Hit). johannestyrollers eigene
  Bucket-Nutzung ist vor dem Tausch ungemessen.
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
