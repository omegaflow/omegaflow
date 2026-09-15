<!--
  title: Handover — Entscheid-Folge X (Stand 2026-09-15)
  session: Entscheid-Folge X
  class: handover
  date: 2026-09-15
  sha256: 817156accb94c81e711a514a09beda56ace2a7b6868a3c7616ea7749df51cbc2
  status: live
-->
# Handover — Entscheid-Folge X (2026-09-15)

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

## adoption — Drei-Mail-Block: Entwürfe stehen, Send beim Operator (härtester undatierter Punkt)

- Die drei sendfertigen Entwürfe (Toth/Turyshev/Markwardt, 20-s-Bande) stehen in
  `docs/auftrag/auftrag-adoption-mails.md`: Repo public, gepinnter Sha
  `32d96efe`, der eine Ask = two-/three-way-Split. Die Prüfliste steht 3/4;
  Reg 4 (Amplitude) bleibt `pending` mit gemessenem ~5-Hz-Anker (Station 14,
  1988) — kein unverankerter Wert im Text. (Schritt: senden — Operator; Consent
  `/consent`.)

## Warten auf Rückmeldung (extern gebunden — kein Datum)

- GitHub Support — User→Org / HTTP 422: Ticket ist raus, Antwort offen.
  (Schritt: Postfach auf die Support-Antwort prüfen.)
- Rubin RSP-Datenrechte — Antwort an Shaughnessy (SLAC) gesendet 2026-09-15;
  Entscheidung offen (`docs/auftrag/auftrag-rubin-data-rights-antrag.md`, im
  selben Atom auf „gesendet" korrigiert).
- NSE/Haug — Anfrage raus, Antwort offen (Keimer).
- CSES-Limadou — Anfrage an Sotgiu (ASI SSDC) raus, Antwort offen; der L2-Zugang
  liegt lokal in `.secrets.local` (`SSDC_USER`/`SSDC_PASS`).

## Nachricht an die Bau-Linie

- `number_audit`-Test rot (**vor-existent**): `known_bad_corpus_rows_are_reconciled_with_its_umfang`
  in `tools/register/src/bin/number_audit.rs` erwartet A14/Z3/D3/K1/N3/V5
  (total 29), aber `docs/specs/bekannt-schlecht-korpus.md` stimmt nicht mehr.
  (Schritt: Korpus oder Erwartung abgleichen — Bau.)
- 20-s-Bande-Papier: per-Papier-Release-Tag und Welt-Fassung-Branch sind absent
  (gemessen `git tag` / `git branch -a`). Kein Send-Blocker (der gepinnte Sha
  ist unveränderlich). (Schritt: Tag + Welt-Fassung-Branch — Bau.)

## Nachricht an die Ernte/Bau-Linie (Arbeitsbaum)

- Der Core kompiliert nicht: fremde uncommittete Arbeit in
  `src/archivar/geo.rs` referenziert `crate::gdp_drifter` statt
  `crate::archivar::gdp_drifter` (Zeilen 197, 289), dazu neue untracked
  `src/archivar/{gdp_drifter,gk2a_ami,goes_abi}.rs`. Das blockiert die
  register-Tools (`register_lookup`, `doc_audit`) in jeder Session.
  (Schritt: die fremde Linie schließt/committet ihre Arbeit — Ernte/Bau.)

## Termine (Wiedervorlage)

- 2026-09-22 — AllWISE-Coverage-Verifikation (CDN-Asset `allwise_coverage.fp01`).
- 2026-09-28 — JUICE-Flyby (Kernel 000113+); Feld-Zustand füllen
  (`papers/flyby-path-2-preregistration.md`).
- 2026-12-02 — NOIRLab Speisekammer-Frage (Gaia DR4).
- 2026-12-03 — Europa Clipper (Fenster); Feld-Zustand füllen.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`).
