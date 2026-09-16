<!--
  title: Handover — Entscheid-Folge 21 (Stand 2026-09-16)
  session: Entscheid-Folge 21
  class: handover
  date: 2026-09-16
  sha256: 80e35a29c5547988ea9293358229b27ec593fcf16b1ce0ad20e3cdd33bb0b96e
  status: live
-->
# Handover — Entscheid-Folge 21 (2026-09-16)

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

- Geteilter Zustand: `docs/zustand/external-state.md`. Neu gemessen 2026-09-16:
  `pii-exposure` run 35081360127 @ `2187c30c` → 45 (Datei, Ref)-Kombinationen aus
  zehn PII-tragenden Dateien, 15/15 Pre-Rewrite-Commits erreichbar (Vortag 44) —
  die GitHub-GC hat die Objekte nicht entfernt. Postfach (2026-09-16 10:53Z):
  GitHub-Support-Bestätigung #4761801 empfangen 2026-09-15 20:12Z; keine
  GC-Bestätigung, keine `privacy@github.com`-Antwort. (Schritt: Postfach; bleibt
  die GC-Bestätigung aus, GitHub auf #4761801 nachfassen — Operator.)

## Consent-Akte (Operator — per-Akt-Consent)

- GES-DISC `client_id`: `.secrets.local` trägt nur `EARTHDATA_EDL_TOKEN`;
  GES-DISC braucht den OAuth-`client_id` (App bei `urs.earthdata.nasa.gov`).
  Konto/API-Key-Anlage bei Dritten = per-Akt-Consent. (Schritt: `/consent` für
  die OAuth-App; danach Nachricht an die Bau-Linie — `S3CredentialRoute::OAuth`
  in `src/archivar/range.rs:243` ist der leere Zweig.)
- limadou-PI-Freigabe (Sotgiu, ASI SSDC): Account gültig, CAS-Login 200,
  „Permission Denied" — PI-Freigabe offen. Entwurf bereit
  `state/mail/limadou-pi-nachfassen.md` (gitignored). (Schritt: `/consent`, dann
  `smail --to alessandro.sotgiu@roma2.infn.it --subject "Re: CSES-Limadou L2
  data access request" --body state/mail/limadou-pi-nachfassen.md --send`.)

## adoption — Drei-Mail-Block: Adressen gemessen, Send beim Operator

- Entwürfe in `state/mail/adoption-mails.md` (gitignored), gepinnt auf Sha
  `50db1ed`; der gepinnte Papier-Link verifiziert 2026-09-16 (HTTP 200). Die drei
  Adressen sind gemessen und im Entwurf hinterlegt, je mit Quell-URL — keine
  geraten. Reg 4 (Amplitude) bleibt `pending` mit gemessenem ~5-Hz-Anker
  (Station 14, 1988). Operator-Entscheid 2026-09-16: noch nicht senden. (Schritt:
  Adressen bestätigen + senden — Operator; Consent `/consent`.)

## Warten auf Rückmeldung (extern gebunden)

- Rubin RSP (Shaughnessy, SLAC): 2026-09-15 21:11 in die Einzelprüfung genommen,
  Antwort offen. NSE/Haug (Keimer), CSES-Limadou (Sotgiu, ASI SSDC): Antwort
  offen — Postfach-Zeile in `docs/zustand/external-state.md`. (Schritt: Postfach
  bei Fälligkeit.)

## Termine (Wiedervorlage)

- 2026-09-22 — AllWISE-Coverage-Verifikation (CDN-Asset `allwise_coverage.fp01`).
- 2026-09-28 — JUICE-Flyby (Kernel 000113+); Feld-Zustand füllen
  (`papers/flyby-path-2-preregistration.md`).
- 2026-12-02 — NOIRLab Speisekammer-Frage (Gaia DR4).
- 2026-12-03 — Europa Clipper (Fenster); Feld-Zustand füllen.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
