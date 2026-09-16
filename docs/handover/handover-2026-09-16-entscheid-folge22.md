<!--
  title: Handover — Entscheid-Folge 22 (Stand 2026-09-16)
  session: Entscheid-Folge 22
  class: handover
  date: 2026-09-16
  sha256: eff8c705e1e88639d290a90f715f514af6d70d2f0c5b8c4203f41936d475239e
  status: live
-->
# Handover — Entscheid-Folge 22 (2026-09-16)

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

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## GitHub-PII-Purge (härtester undatierter Punkt)

- Support-Tickets `#4761482` + `#4761801` bestätigt (Mail-Ledger); GC-Antwort
  aus. Der `privacy@github.com`-Deletion-Request
  (`state/mail/privacy-deletion-request.md`) geht manuell aus Proton — der
  Sendestatus ist aus dem Baum nicht messbar. (Schritt: Postfach
  `state/mail/mail_ledger.φ`; bleibt die GC-Antwort aus, GitHub auf `#4761801`
  nachfassen — Operator.)

## Consent-Akte (per-Akt, Operatorwort)

- **adoption-Block** — `state/mail/adoption-mails.md` → Toth/Turyshev/Markwardt.
  (Schritt: Adressen bestätigen + Sende-Wort.)
- **DEMETER/CDPP** — Order über REGARDS. Gemessen: kein Entwurf in `state/mail/`,
  keine Ledger-Zeile; der Sendestatus ist aus dem Baum **nicht messbar**
  (Web-Portal, kein Mail-Pfad) — `unverifizierbar`. (Schritt: REGARDS-Route +
  Dataset-URN messen, Entwurf bis zur Ausführungskante — autonom; dann `/consent`
  und Order.)

## Warten auf Rückmeldung (extern)

Offene Alternativen gemessen in
`docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md`; es bleiben
nur die Routen ohne gemessene Alternative:

- **Fünf Sonden-Anfragen** (NSSDC Voyager/Mariner 10/Viking, Cassini, Juno) —
  gesendet 2026-09-16, Antwort offen; Voyager closed-loop und Juno-Earth-Flyby
  bleiben request-only.
- **NSE/Haug** (Keimer/MPI-FKF) — Anfrage hält (Rohdaten nicht öffentlich).

## Operator-gebundene Punkte

- **ESP32-Modul** — der physische Träger für Puls/HRV, on hold; BOM
  `docs/specs/mantis-shrimp-bom.md`. (Schritt: Operator-Wort.)

## Termine (Wiedervorlage)

- ~2026-10-07 — CSES-Limadou: neue Antragsprozedur nach CSES-02-Umstellung
  (Sotgiu 2026-09-16, „wait a few weeks").
- 2026-09-18 — Lasair.
- 2026-09-22 — AllWISE-Coverage (`allwise_coverage.fp01`).
- 2026-09-28 — JUICE-Flyby (Kernel 000113+); Feld-Zustand füllen.
- 2026-09-30 — EDL-Token-Erneuerung (`EARTHDATA_EDL_TOKEN`, Konto `omegaflow.space`).
- 2026-12-02 — NOIRLab Speisekammer-Frage (Gaia DR4).
- 2026-12-03 — Europa Clipper (Fenster).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
