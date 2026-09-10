<!--
  title: Anfrage (extern) — Rubin Science Platform: Datenrechte als unabhängiger Forscher
  class: auftrag
  date: 2026-09-10
  sha256: 7cfbd72e7d69cca6b378db86d811858287431ff5d776e819b1c3302a75a720a2
  status: live
  see-also: docs/specs/ref-auth-apis.md docs/handover/handover-2026-09-10-nicht-autonom.md
-->
# Anfrage — Rubin Science Platform: Datenrechte

Entworfen 2026-09-10 (noch nicht gesendet — Operator sendet).

Gemessen 2026-09-10 (`rsp.lsst.io`): die RSP verlangt **Rubin data rights**;
öffentlich sind nur die **world-public Alerts** (via Broker), public EPO-Produkte
und Jahres-Releases **nach der 2-Jahres-Frist**. Für Amateure/unabhängige
Forscher (nicht an US-Institution) gibt es einen Antragsweg: den RSP-Account
starten (CILogon mit ORCID/GitHub), dann auf die Data-Rights-Verifikations-Mail
mit einer Begründung antworten, warum die public Optionen nicht reichen.

## Ablauf

1. RSP-Account starten: `https://data.lsst.cloud/` → Log in → CILogon (ORCID oder
   GitHub; beide frei). Danach schreibt Rubin-Personal zur Data-Rights-Prüfung.
2. Auf diese Mail die Begründung senden (unten). Kontakt bei Fragen:
   Heather Shaughnessy (Data Rights).

## Text (englisch, sendfertig)

Subject: Rubin data rights request — independent researcher (LAIC transient analysis)

Dear Rubin Science Platform / Data Rights team,

I am Johannes Tyroller, an independent researcher in Germany doing non-profit
scientific research. I would like to request data rights for the Rubin Science
Platform.

Research project: I investigate the coupling between the lithosphere, the
atmosphere and the ionosphere (LAIC) — specifically whether atmospheric and
ionospheric signals precede large earthquakes (M >= 6). I work retrospectively
with event-centred time windows and multi-channel time-series analysis (transfer
entropy, cross-correlation). The Rubin/LSST time-domain data (the transient
alert stream and the associated calibrated light curves) is one of the channels
I would like to use.

Why the public options are insufficient: the world-public alerts give me the
detection stream, but for the retrospective multi-channel analysis I need the
calibrated light curves / forced photometry of the selected objects beyond the
alert packets, which requires RSP access.

I confirm that my use is non-profit scientific research, and that I will follow
the Rubin Data Policy and the Acceptable Use Policy, with proper attribution in
all resulting publications.

I would be grateful if you could grant me data rights for the Rubin Science
Platform.

With thanks,
Johannes Tyroller
Corneliweg 1, 79875 Dachsberg, Germany
johannes.tyroller@proton.me

## Register-Pflicht

Die Antwort (Datenrechte gewährt oder abgelehnt) ist die Messung. Bis dahin ist
die RSP `blocked account` (proprietär). Die **public Alerts** bleiben davon
unberührt — sie sind der offene Weg (siehe Autonom-Handover: Alert-Compiler).
