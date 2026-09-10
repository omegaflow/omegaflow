<!--
  title: Anfrage (extern) — Rubin Science Platform: Data-Rights-Rolle für das bestehende Konto
  class: auftrag
  date: 2026-09-10
  sha256: 4366d030dfb3274e80830e6c75b07f8a5cd8818c9499b2bf44f909e6c7141105
  status: live
  see-also: docs/specs/ref-auth-apis.md docs/handover/handover-2026-09-10-nicht-autonom.md
-->
# Anfrage — Rubin Science Platform: Data-Rights-Rolle

Entworfen 2026-09-10 (noch nicht gesendet — Operator sendet).

Gemessen 2026-09-10: der CILogon-Login (GitHub) funktioniert; die
COmanage-Registry (`id.lsst.cloud`) meldet „SORID
`http://cilogon.org/serverE/users/567987` is already associated with CILogon OIDC
Claims" — **das Konto existiert bereits**, es fehlt nur die **Data-Rights-Rolle**
(„You do not have any current roles"). Die RSP verlangt Rubin data rights;
öffentlich sind nur die world-public Alerts (via Broker), public EPO-Produkte und
Jahres-Releases nach der 2-Jahres-Frist.

## Ablauf

Das Konto ist da (Login erledigt). Nächster Schritt: die **Rollen-Anfrage** an
den RSP-Support (`https://data.lsst.cloud/support`) bzw. das Community-Forum
(Support-Kategorie). Kontakt bei Data-Rights-Fragen: Heather Shaughnessy.

## Text (englisch, sendfertig)

Subject: Rubin data rights request — existing RSP account, independent researcher

Dear Rubin Science Platform Support,

I have an RSP account (CILogon identity
`http://cilogon.org/serverE/users/567987`, via GitHub) but no data-rights role:
the registry reports "You do not have any current roles". I would like to
request data rights.

I am Johannes Tyroller, an independent researcher in Germany doing non-profit
scientific research. My project: I investigate the coupling between the
lithosphere, the atmosphere and the ionosphere (LAIC) — specifically whether
atmospheric and ionospheric signals precede large earthquakes (M >= 6). I work
retrospectively with event-centred time windows and multi-channel time-series
analysis (transfer entropy, cross-correlation). The Rubin/LSST time-domain data
(the transient alert stream and the associated calibrated light curves) is one
of the channels I would like to use.

Why the public options are insufficient: the world-public alerts give me the
detection stream, but for the retrospective multi-channel analysis I need the
calibrated light curves / forced photometry of the selected objects beyond the
alert packets, which requires RSP access.

I confirm that my use is non-profit scientific research, and that I will follow
the Rubin Data Policy and the Acceptable Use Policy, with proper attribution in
all resulting publications.

Could you please grant data rights to my existing account?

With thanks,
Johannes Tyroller
Corneliweg 1, 79875 Dachsberg, Germany
johannes.tyroller@proton.me

## Register-Pflicht

Die Antwort (Datenrechte gewährt oder abgelehnt) ist die Messung. Bis dahin ist
die RSP `blocked account` (proprietär). Die **public Alerts** bleiben davon
unberührt — sie sind der offene Weg (siehe Autonom-Handover: Alert-Compiler).
