<!--
  title: Handover — Entscheid-Folge 25 (Stand 2026-09-17)
  session: Entscheid-Folge 25
  class: handover
  date: 2026-09-17
  sha256: deadca865579798a12fa9d48d414594306dfc202e92391a78cbe99141dd514fb
  status: live
-->
# Handover — Entscheid-Folge 25 (2026-09-17)

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

## GitHub-PII-Purge (härtester undatierter Punkt)

- Support-Tickets `#4761482` + `#4761801` bestätigt (`state/mail/mail_ledger.φ:59,64`);
  keine GC-Antwort (jüngste Ledger-Zeile `:81` MLZ-Versand). `state/mail/privacy-deletion-request.md`
  ohne `status: sent`. (Schritt: Follow-up zu `#4761801` als Entwurf vorbereiten,
  operator-Sende-Wort.)

## Consent-Akte (per-Akt, Operatorwort)

- **adoption-Block** — `state/mail/adoption-mails.md`: drei sendfertige Entwürfe
  (Toth, Turyshev, Markwardt, je Quelle in der To-Zeile). (Schritt: Operator-Sende-Wort.)
- **Mail-Fang (KV-Namespace)** — `cloudflare/wrangler.toml:4,7,14`: `account_id` leer,
  `FORWARD_TO` leer, `MAIL_QUEUE`-id Platzhalter. (Schritt: KV-Namespace anlegen,
  id + `FORWARD_TO` eintragen, `wrangler deploy` — Operator/Konto.)

## Warten auf Rückmeldung (extern)

- **Fünf Sonden-Anfragen** (NSSDC Voyager/Mariner 10/Viking, Cassini, Juno) —
  gesendet 2026-09-16, Antwort offen.
- **NSE/Haug** (MLZ/FRM-II Lohstroh; MPI-FKF Keimer) — Antwort offen.
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
