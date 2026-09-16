<!--
  title: Handover — Entscheid-Folge XVII (Stand 2026-09-16)
  session: Entscheid-Folge XVII
  class: handover
  date: 2026-09-16
  sha256: e37c4a71b29f146e4a53457191a89771b569a8b2900053e3746545167c51c3bd
  status: live
-->
# Handover — Entscheid-Folge XVII (2026-09-16)

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

- Geteilter Zustand: `docs/zustand/external-state.md` (PII-Exposition, Postfach,
  CI). (Schritt: Postfach bei Fälligkeit; bleibt die GC-Bestätigung auf #4761801
  aus, GitHub nachfassen — Operator; bei Verifikationsbitte der Privacy-Mail aus
  Proton senden — Entwurf liegt bereit.)

## adoption — Drei-Mail-Block: Adressen gemessen, Send beim Operator

- Entwürfe in `state/mail/adoption-mails.md` (gitignored), gepinnt auf Sha
  `50db1ed`; die drei Adressen sind gemessen und im Entwurf hinterlegt, je mit
  Quell-URL — keine geraten. Reg 4 (Amplitude) bleibt `pending` mit gemessenem
  ~5-Hz-Anker (Station 14, 1988). Operator-Entscheid 2026-09-16: noch nicht
  senden. (Schritt: Adressen bestätigen + senden — Operator; Consent `/consent`.)

## Warten auf Rückmeldung (extern gebunden)

- Rubin RSP (Shaughnessy, SLAC), NSE/Haug (Keimer), CSES-Limadou (Sotgiu, ASI
  SSDC): Antwort offen — Postfach-Zeile in `docs/zustand/external-state.md`.
  (Schritt: Postfach bei Fälligkeit.)

## Namens-Entscheid (Rat, 2026-09-16)

- `archive_search` bleibt — keine zweite Taufe; das aufgezeichnete Wort bestätigt.
  Die 16 Netz-Quellen sind selbst Archive (arXiv/ADS/NTRS/Wayback/Zenodo/…); der
  Name trägt beide Naturen. `osearch` bleibt descoped (Kollision mit dem
  Fremdprojekt, das `o` benennt nichts Gemessenes, o-Präfix = Trainingsdaten-Muster).
  Keine Migration, kein Alias.

## Termine (Wiedervorlage)

- 2026-09-22 — AllWISE-Coverage-Verifikation (CDN-Asset `allwise_coverage.fp01`).
- 2026-09-28 — JUICE-Flyby (Kernel 000113+); Feld-Zustand füllen
  (`papers/flyby-path-2-preregistration.md`).
- 2026-12-02 — NOIRLab Speisekammer-Frage (Gaia DR4).
- 2026-12-03 — Europa Clipper (Fenster); Feld-Zustand füllen.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`).
