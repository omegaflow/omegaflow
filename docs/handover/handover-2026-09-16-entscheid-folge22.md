<!--
  title: Handover — Entscheid-Folge 22 (Stand 2026-09-16)
  session: Entscheid-Folge 22
  class: handover
  date: 2026-09-16
  sha256: dca47f34c27a5615ae06bf12632f48268443f9f4adb07cd4295d52d6755706b5
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

## GitHub-Purge der PII-Alt-Commits (härtester undatierter Punkt)

- Geteilter Zustand: `docs/zustand/external-state.md`. Nachmessung `pii-exposure`
  run 35101796537 @ `8d0c9090` (2026-09-16): **45** Kombinationen, 15/15
  Pre-Rewrite-Commits erreichbar, PII retrievable: yes — unverändert zur
  Vormessung; die GitHub-GC hat nichts entfernt. GC-Bestätigung #4761801 offen;
  keine `privacy@github.com`-Antwort. (Schritt: Postfach; bleibt die
  GC-Bestätigung aus, GitHub auf #4761801 nachfassen — Operator.)

## Consent-Akte (Operator — per-Akt-Consent)

**C Anfragen an Dritte (per-Akt-Consent):**
- BiSON-Team (`bison@contacts.bham.ac.uk`): Entwurf
  `state/mail/bison-team-anfrage.md`. (Schritt: `/consent`, dann `smail … --send`.)
- DEMETER/CDPP: Order über REGARDS; Zhangheng-1/CSES SSDC-Formular
  `https://tools.ssdc.asi.it/UserManager/requestUser.jsp`. (Schritt: `/consent`,
  dann Formular/Order.)

**D Route-/Exit-Wort (gemessen 2026-09-16):**
- Kein freier Proton-Exit für `.com`/`.edu`/`.org` — Host-Routen fallen auf
  Browser-Bridge oder absent.
- TNF (`pdssbn.astro.umd.edu/…/lunocc2012.tnf`): Host 0, Wayback 503 → Proton.
  (Schritt: Operatorwort.)
- Haw 1997 (`10.2514/2.3240`): Paywall → ILL/Proton. (Schritt: Operatorwort.)
- Hinson 1997 (`10.1029/97GL01608`): 403 Cloudflare; Zahlen aus offenen
  PDS-Daten re-derivierbar (`GO-J-RSS-1-ODF-V1.0`) → Bau. (Schritt:
  PDS-Re-Derivation als Ernte-/Bau-Atom.)
- HAWC: CA-Bundle-Route (`OMEGAFLOW_CA_BUNDLE`); Register/Bau, nicht Operator.
- LIS/OTD: Route 200; Blocker ist das GHRC-Konto → E.

**E Konsumenten (Tor 1) — Bau-Reihenfolge, kein Operator:**
- ERI/VLASS/CORS-Konsument, LASzip-Decoder, JVO skynode-TAP, Babamul,
  GHRC-DAAC, WFAU VSA/WSA; Parser-Magic Gaps 1 (`Frame::Data`), 8 (`flush!()`),
  12 (Category/Group). (Schritt: an die Bau-Linie — als Post gepostet.)

**A Register-offene Registrierungen:** keine.

## adoption — Drei-Mail-Block

- Entwürfe `state/mail/adoption-mails.md`, Adressen gemessen. Operator-Entscheid
  2026-09-16: noch nicht senden. (Schritt: Adressen bestätigen + senden — Operator.)

## Ernte-Folge 42/43 — Register-/Ernte-Rest

- MPC-Shard UnnObs, GHRC-DAAC, Survey §1 (26 Pendings); Register-Digest-Rest
  (Fink/ALeRCE, TDAT/FITS, Akteure, Holdings, Orphan-Verdicts). (Schritt:
  `sources.φ` + CDN / je Punkt messen.)

## smail_recv — leerer Body bei verschachteltem MIME

- Rekursiver MIME-Abstieg (`collect_text` in `tools/service/src/bin/smail_recv.rs`)
  gebaut + Test (nested multipart/mixed → multipart/alternative); der direkte
  Sotgiu-Reply kam mit leerem Body an. Root cause nicht bestätigt (das Roh wird
  nicht aufbewahrt). (Schritt: bei erneutem leerem Body die Worker-`message.raw`-
  Quelle messen — `cloudflare/email_worker.js`.)

## Warten auf Rückmeldung (extern gebunden)

- Rubin RSP (Shaughnessy, SLAC), NSE/Haug (Keimer) — Antworten offen.
- Fünf Sonden-Anfragen (NSSDC CRUSO `gsfc-dl-nssdca-request@mail.nasa.gov`,
  JPL-NAV Asmar) — gesendet 2026-09-16 aus Proton, Antwort offen.
- CSES-Limadou (Sotgiu) — beantwortet 2026-09-16: CSES-02-Umstellung, „wait a
  few weeks" vor einem neuen Antrag (Wiedervorlage).
  (Schritt: Postfach bei Fälligkeit, `state/mail/mail_ledger.φ`.)

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
