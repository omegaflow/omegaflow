<!--
  title: Handover — Entscheid-Folge XVI (Stand 2026-09-16)
  session: Entscheid-Folge XVI
  class: handover
  date: 2026-09-16
  sha256: 0145289933dc129d4f4767d107280f240ec23c6cae12d5c732c5bdf875f6c0b1
  status: live
-->
# Handover — Entscheid-Folge XVI (2026-09-16)

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

## GitHub-Purge der PII-Alt-Commits (härtester undatierter Punkt)

- Geteilter Zustand: `docs/zustand/external-state.md` — PII-Exposition 45 @
  `ce367dd0` (GC offen), Postfach `2026-09-16 07:42` (keine neue Antwort). Neu
  messen nur bei Fälligkeit (HEAD-Wechsel / 2⁶ min), sonst zitieren.
- Support-Ticket #4761801 (`support@githubsupport.com`) bestätigt empfangen
  2026-09-15 22:12; die GC-Bestätigung steht aus. (Schritt: Postfach; bleibt sie
  aus, GitHub auf #4761801 nachfassen — Operator.)
- Data-Subject-Löschung an `privacy@github.com` gesendet 2026-09-16 (Resend-Id
  `16c1b8c6-d24a-458a-a49d-7c1362b6aa8e`), Entwurf
  `state/mail/privacy-deletion-request.md` (lokal, gitignored). (Schritt:
  Postfach; bei Verifikationsbitte aus Proton senden — Entwurf liegt bereit.)

## adoption — Drei-Mail-Block: Adressen gemessen, Send beim Operator

- Die drei Entwürfe (Toth/Turyshev/Markwardt, 20-s-Bande) liegen lokal in
  `state/mail/adoption-mails.md` (gitignored), gepinnt auf Sha `50db1ed`; der eine
  Ask = two-/three-way-Split. Reg 4 (Amplitude) bleibt `pending` mit gemessenem
  ~5-Hz-Anker (Station 14, 1988). Die drei Adressen sind gemessen 2026-09-16 und
  im Entwurf hinterlegt, je mit Quell-URL (Toth `vttoth.com/CMS/contact`,
  Turyshev arXiv gr-qc/0412049 Corresponding-Author, Markwardt
  `science.gsfc.nasa.gov/sci/bio/craig.b.markwardt`) — keine Adresse geraten.
  Operator-Entscheid 2026-09-16: noch nicht senden. (Schritt: Adressen bestätigen
  + senden — Operator; Consent `/consent`.)

## Warten auf Rückmeldung (extern gebunden)

- Rubin RSP (Shaughnessy, SLAC), NSE/Haug (Keimer), CSES-Limadou (Sotgiu, ASI
  SSDC): Antwort offen — Postfach-Zeile in `docs/zustand/external-state.md`.
  (Schritt: Postfach bei Fälligkeit.)

## Benchmark — PII-Expositions-Verifikation (flash gegen pro/max)

- Klasse geschlossen: `general` (flash) $0.0138 gegen `research-max` (pro/max)
  $0.0729 — pro/max 5,3× teurer bei identischem Ergebnis. Sieger: flash; ein
  registrierter Sieger wird zitiert, nie erneut gefahren.

## Termine (Wiedervorlage)

- 2026-09-22 — AllWISE-Coverage-Verifikation (CDN-Asset `allwise_coverage.fp01`).
- 2026-09-28 — JUICE-Flyby (Kernel 000113+); Feld-Zustand füllen
  (`papers/flyby-path-2-preregistration.md`).
- 2026-12-02 — NOIRLab Speisekammer-Frage (Gaia DR4).
- 2026-12-03 — Europa Clipper (Fenster); Feld-Zustand füllen.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`).
