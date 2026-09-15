<!--
  title: Handover — Entscheid-Folge VIII (Stand 2026-09-15)
  session: Entscheid-Folge VIII
  class: handover
  date: 2026-09-15
  sha256: 8bc5c4a9840d61b9812ba6c6a8219d1aaf600f3bd50bdd1965c2debeed0048ec
  status: live
-->
# Handover — Entscheid-Folge VIII (2026-09-15)

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

## Zugangsanfragen — Notwendigkeits-Abgleich beim Operator (Chat übergeben)

Die Notwendigkeit einer Zugangsanfrage entscheidet sich am Quellen-Register, nicht
an der Korrespondenz. Der Abgleich (14 versandte Mails aus `state/mail/mail_ledger.φ`
+ Register-Befund je Anfrage) liegt beim Operator zur Weitergabe an die Ernte-Linie;
Ernte misst je Anfrage und trägt das Verdikt in `folge28` ein. (Schritt: Operator
übergibt den Abgleich; Ernte misst.)

## Nvidia / free-gold — descoped (gemessen 2026-09-15)

Der free-gold-Pfad trägt nicht und ist verworfen. Gemessen: `nvidia/nemotron-3-super-120b-a12b`
liefert 46 % HTTP 503; ohne Tools fabriziert es Routen (0/6 Beispiel-URLs verifiziert,
Thinking-aus erfindet zu jeder Kandidatin eine Route); sein Verifizierer `archive_search`
ist ein Netz-Sweep, der lokal hängt. `free-research` und `provider.nvidia` entfernt;
`riva_translate` gelöscht — die Modelle tragen alle Sprachen selbst, ein eigener
Übersetzer ist nicht nötig. `nemotron-parse-2.0` (VLM, kein Text-Chat) und
Embedding/Rerank (404) bleiben ohne Andockung.

## Code/Infra — Nachricht an die Bau-Linie

- `number_audit`-Test rot (**vor-existent**, nicht diese Session):
  `known_bad_corpus_rows_are_reconciled_with_its_umfang` in
  `tools/register/src/bin/number_audit.rs` erwartet A14/Z3/D3/K1/N3/V5 (total 29),
  aber `docs/specs/bekannt-schlecht-korpus.md` stimmt nicht mehr mit den Erwartungen.
  `number_audit.rs` seit 2026-09-11 unberührt. (Schritt: Korpus oder Erwartung
  abgleichen — Bau.)

## Warten auf Rückmeldung (extern gebunden — kein Datum)

- adoption — Drei-Mail-Block (Toth/Turyshev/Markwardt) nicht gesendet; der Blocker
  Bande-Split ist geschlossen. (Schritt: senden — Operator.)
- GitHub Support — User→Org / HTTP 422: Antwort offen. (Schritt: Postfach prüfen.)
- Rubin RSP-Datenrechte — Antwort an Shaughnessy (SLAC) gesendet 2026-09-15;
  Antwort offen.
- NSE/Haug — Anfrage raus, Antwort offen (Keimer).
- GAVO — geschlossen: Demleitner hat die ivoa-Umstellung am 2026-09-14 bestätigt
  (`state/mail/mail_ledger.φ`).

## Termine (Wiedervorlage)

- 2026-09-22 — AllWISE-Coverage-Verifikation (CDN-Asset `allwise_coverage.fp01`).
- 2026-09-28 — JUICE-Flyby (Kernel 000113+); Feld-Zustand füllen
  (`papers/flyby-path-2-preregistration.md`).
- 2026-12-02 — NOIRLab Speisekammer-Frage (Gaia DR4).
- 2026-12-03 — Europa Clipper (Fenster); Feld-Zustand füllen.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`).
