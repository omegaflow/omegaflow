<!--
  title: Handover — Entscheid-Folge IX (Stand 2026-09-15)
  session: Entscheid-Folge IX
  class: handover
  date: 2026-09-15
  sha256: b62a3354b44db3152b223ac43bcfc3c9ece80f9f1c916f190799e9ff1f1473ec
  status: live
-->
# Handover — Entscheid-Folge IX (2026-09-15)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird, sobald der eigene Commit steht und `origin/main`
Vorfahr von HEAD ist (Fast-Forward) — ein Push sendet nur Commits, der Arbeitsbaum
darf schmutzig sein.

Der erste offene Abschnitt benennt den härtesten undatierten Punkt (datierte
Wiedervorlagen schweigen vor ihrem Datum). Jeder offene Punkt trägt seinen
nächsten Schritt in derselben Zeile — Werkzeug, Datei, URL oder Anfrage;
„Schritt unbekannt — erste Messung: X" ist ein vollständiger Schritt. Kein
Dokument wächst ohne Messung; die Droh-Sprache ersetzt den Schritt nicht.

Dies ist die Entscheid-Linie: hier steht nur, was diese Linie autonom trägt —
Tasks, die nicht autonom hier erfolgen können, sind als Nachricht an ihre Linie
überführt (nie in ein fremdes Handover geschrieben).

## adoption — Drei-Mail-Block entblockt (härtester undatierter Punkt)

- Toth/Turyshev/Markwardt (20-s-Bande): der Bande-Split ist geschlossen, die
  Prüfliste steht, die Mails sind entblockt (`auftrag-adoption.md`, Archiv;
  Forschung-Folge 20:131). (Schritt: senden — Operator.)

## Warten auf Rückmeldung (extern gebunden — kein Datum)

- GitHub Support — User→Org / HTTP 422: Ticket ist raus, Antwort offen.
  (Schritt: Postfach auf die Support-Antwort prüfen.)
- Rubin RSP-Datenrechte — Antwort an Shaughnessy (SLAC) gesendet 2026-09-15;
  Antwort offen (`docs/auftrag/auftrag-rubin-data-rights-antrag.md`).
- NSE/Haug — Anfrage raus, Antwort offen (Keimer).
- CSES-Limadou — Anfrage an Sotgiu (ASI SSDC) raus, Antwort offen; der L2-Zugang
  liegt lokal in `.secrets.local` (`SSDC_USER`/`SSDC_PASS`).

## Code/Infra — Nachricht an die Bau-Linie

- `number_audit`-Test rot (**vor-existent**, nicht diese Session):
  `known_bad_corpus_rows_are_reconciled_with_its_umfang` in
  `tools/register/src/bin/number_audit.rs` erwartet A14/Z3/D3/K1/N3/V5 (total 29),
  aber `docs/specs/bekannt-schlecht-korpus.md` stimmt nicht mehr mit den Erwartungen.
  `number_audit.rs` seit 2026-09-11 unberührt. (Schritt: Korpus oder Erwartung
  abgleichen — Bau.)

## Termine (Wiedervorlage)

- 2026-09-22 — AllWISE-Coverage-Verifikation (CDN-Asset `allwise_coverage.fp01`).
- 2026-09-28 — JUICE-Flyby (Kernel 000113+); Feld-Zustand füllen
  (`papers/flyby-path-2-preregistration.md`).
- 2026-12-02 — NOIRLab Speisekammer-Frage (Gaia DR4).
- 2026-12-03 — Europa Clipper (Fenster); Feld-Zustand füllen.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`).
