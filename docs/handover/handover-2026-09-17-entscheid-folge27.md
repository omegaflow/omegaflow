<!--
  title: Handover — Entscheid-Folge 27 (Stand 2026-09-17)
  session: Entscheid-Folge 27
  class: handover
  date: 2026-09-17
  sha256: be395ba26dc107d8cd1d5c0cc13ae32e2a5c1f0eca31395c2087562cca64813d
  status: live
-->
# Handover — Entscheid-Folge 27 (2026-09-17)

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
Der Planungs-Pass nennt die offenen Punkte als nummerierte Auswahl (der erste ist
der härteste undatierte); die Session arbeitet so viele ab wie möglich.

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## Consent-Akte (per-Akt, Operatorwort)

- **adoption-Block** — `state/mail/adoption-mails.md`: drei Entwürfe (Toth,
  Turyshev, Markwardt, je Quelle in der To-Zeile); je cleanen Body extrahieren
  (`smail --body` sendet verbatim). (Schritt: Body-Extraktion, dann
  Operator-Sende-Wort.)
- **Mail-Fang (KV-Namespace)** — `cloudflare/wrangler.toml:4,7,14`: `account_id`
  leer, `FORWARD_TO` leer, `MAIL_QUEUE`-id Platzhalter. (Schritt: KV-Namespace
  anlegen, id + `FORWARD_TO` eintragen, `wrangler deploy` — Operator/Konto.)

## Warten auf Rückmeldung (extern)

- **Fünf Sonden-Anfragen** (NSSDC Voyager/Mariner 10/Viking, Cassini, Juno) —
  gesendet 2026-09-16, Antwort offen.
- **NSE/Haug** (MLZ/FRM-II Lohstroh; MPI-FKF Keimer) — Antwort offen.
- **Rubin-Review** (Shaughnessy, high volume) — offen.
- **GitHub GC #4761801** — Follow-up gesendet 2026-09-17 (Resend
  `01a0aded-75a3-7548-a994-682b9ef54843`), Antwort offen.
- **GitHub Privacy-Löschung** — gesendet 2026-09-17 (Resend
  `01a0aded-819e-75db-aaae-2ff654b6ec15`), Antwort offen.
- Offene Alternativen: `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md`.

## Operator-gebundene Punkte

- **ESP32-Modul** — physischer Träger für Puls/HRV, on hold; BOM
  `docs/specs/mantis-shrimp-bom.md`. (Schritt: Operator-Wort.)

## Termine (Wiedervorlage)

- ~2026-10-07 — CSES-Limadou: neue Antragsprozedur nach CSES-02-Umstellung.
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
