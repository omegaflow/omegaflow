<!--
  title: Handover — Entscheid-Folge 23 (Stand 2026-09-16)
  session: Entscheid-Folge 23
  class: handover
  date: 2026-09-16
  sha256: 4e33ff3786069ae5561f717c1ce2f6a815ab9ea17f156504589685b1427b5708
  status: live
-->
# Handover — Entscheid-Folge 23 (2026-09-16)

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
  keine GC-Antwort im Ledger (jüngste Zeilen `:81` MLZ-Versand, `:80` CSES — keine
  GitHub-/GC-Zeile). Der
  `privacy@github.com`-Deletion-Request (`state/mail/privacy-deletion-request.md`)
  trägt keinen `status: sent` — der Sendestatus ist aus dem Baum `unverifizierbar`
  (manuell aus Proton). (Schritt: Postfach/Ledger auf GC-Antwort prüfen; bleibt sie
  aus, GitHub auf `#4761801` nachfassen — Operator.)

## Consent-Akte (per-Akt, Operatorwort)

- **adoption-Block** — `state/mail/adoption-mails.md`: drei sendfertige Entwürfe
  (Toth `vttoth@vttoth.com`, Turyshev `turyshev@jpl.nasa.gov`, Markwardt
  `craig.b.markwardt@nasa.gov`, je Quelle in der To-Zeile, 2026-09-16). (Schritt:
  Operator-Sende-Wort; danach Ledger-Zeile je Mail, Antwort `pending`.)
- **DEMETER/CDPP** — **der Key funktioniert** (selbst gemessen 2026-09-16): Login
  `rs-authentication/oauth/token` (Query-String-Form) → 200, Bearer `role:
  REGISTERED_USER`; `rs-catalog` mit Token → 200, **57 760** `DMT_N1_1144`-Objekte;
  `rs-order` → 42 Orders (`demeter-0000…0026` meist `DONE`, einzelne `FAILED`/`EXPIRED`).
  Der frühere `403` (`archive_search --verdict`) war ein Auth-Gate, kein Route-Block.
  Offen ist die Harvest-Vollständigkeit (~5 %) + Quellenregistrierung — als Post an
  die ernte-Linie. Neue Orders sind ein Dritt-Akt → Consent über diese Linie.
  (Schritt: ernte setzt die Ernte über den Ledger fort und registriert die Quelle
  in `phi/sources.φ`.)
- **Mail-Fang (KV-Namespace)** — `cloudflare/wrangler.toml:12-14` trägt den
  Platzhalter `REPLACE_WITH_WRANGLER_KV_NAMESPACE_ID`; `FORWARD_TO` (`:7`) leer.
  `.secrets.local` trägt `CLOUDFLARE_API_TOKEN` + `CLOUDFLARE_ACCOUNT_ID` (Werte
  nicht gelesen). (Schritt: KV-Namespace `MAIL_QUEUE` im Dashboard anlegen, id +
  `FORWARD_TO` eintragen, `wrangler deploy` — oder Token account-scoped mit
  „Workers KV Storage:Edit" — Operator/Konto.)

## Warten auf Rückmeldung (extern)

- **Fünf Sonden-Anfragen** (NSSDC Voyager/Mariner 10/Viking, Cassini, Juno) —
  gesendet 2026-09-16, Antwort offen; Voyager closed-loop und Juno-Earth-Flyby
  bleiben request-only.
- **NSE/Haug (MLZ/FRM-II, Dr. Lohstroh)** — gesendet 2026-09-16T18:50Z, Antwort
  offen; hält die FRM-II-Spin-Echo-Deposits (NICOS/SciCat/iMPULSE) für die
  TRISP-NSE-Rohdaten vor.
- **NSE/Haug** (Keimer/MPI-FKF) — Anfrage hält (Rohdaten nicht öffentlich);
  `mail_ledger.φ:8,50`, keine Antwort.
- Offene Alternativen gemessen in
  `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md`.

## Operator-gebundene Punkte

- **20-s-Bande — Release-Tag** — Vorschlag Tag `paper/twenty-second-band-v7`
  (lightweight, auf `8ef168af`); **kein** Branch. (Schritt: Operator-Wort → zurück
  an forschung; dann `git tag`/`git push`.)
- **ESP32-Modul** — physischer Träger für Puls/HRV, on hold; BOM
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
