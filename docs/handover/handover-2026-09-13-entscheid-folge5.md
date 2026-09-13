<!--
  title: Handover — Entscheid-Folge V (Stand 2026-09-13)
  session: Entscheid-Folge V
  class: handover
  date: 2026-09-13
  sha256: 3511370a11455886d668f1be1a527ca6204e9cf1cf454c1add50ef8dd328da33
  status: live
-->
# Handover — Entscheid-Folge V (2026-09-13)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

Dies ist die Entscheid-Linie: hier steht nur, was diese Linie autonom trägt —
Tasks, die nicht autonom hier erfolgen können, sind in das Handover ihrer Linie
überführt. Wiedervorlage gilt nur für Termine; „Warten auf Rückmeldung" und
„Pausiert" tragen kein Datum.

## Warten auf Rückmeldung (extern gebunden — kein Datum)

- GitHub Support — User→Org / HTTP 422: Ticket ist raus, Antwort offen (Konto
  weiter `type: User`).
- `ivoa` — (a) Re-Invite offen; als `johannestyroller` prüfen. Gemessen
  2026-09-13 als `omegaflow`: keine Einladung (`/orgs/ivoa/invitation` →
  „Invitation not found"), keine Mitgliedschaft (404); `ivoa/uvor` existiert
  (angelegt 2026-09-09). Der Operator-Browser ist als `omegaflow` eingeloggt —
  für `johannestyroller` muss das GitHub-Konto gewechselt werden. (b)
  vo-tap-Crate-Push hängt an der Mitgliedschaft.
- ned-objdir — Bulk-z angefragt, IPAC-Auto-Bestätigung 2026-09-09, inhaltliche
  Antwort offen; der öffentliche TAP lebt.
- NOIRLab Data Lab — abgelehnt (Nikutta); TAP anonym offen. Offen bleibt nur
  die Speisekammer-Frage (Wiedervorlage 2026-12-02, Gaia DR4).
- Rubin RSP-Datenrechte — Konto registriert (CILogon/GitHub `omegaflow`). Die
  Registry meldet (gemessen 2026-09-13) „request for enrollment is still being
  processed" — die Petition ist eingereicht und wartet auf die Rubin-Freigabe.
  Der Code `F34K-3J2N` liegt im Ledger (2026-09-13); die Code-Schritt-Seite
  (`.../co_petitions/start/coef:6/efwid:2`) rendert im Addon-Browser leer.
- Account-Verifikationen (Ledger `state/mail/mail_ledger.φ`):
  - Copernicus Data Space — Verify-Mail neu angefordert (2026-09-13, Ledger);
    der Link lebt nur im Postfach `code@omegaflow.space` → dort bestätigen.
  - ICIMOD RDS — alter Verify-Link abgelaufen; Reset-Link für `omegaflow` neu
    angefordert (2026-09-13); Login bis Email-Verify blockiert → Link im
    Postfach bestätigen.
- BiSON/Broomhall, CSES-Limadou, NSE/Haug — Anfragen raus, Antwort offen.
- adoption — Drei-Mail-Block (Toth/Turyshev/Markwardt) nicht gesendet; hängt an
  Merge-Fix + Bande-Split.

## Pausiert (kein Datum — Operator meldet sich)

- Desktop-Fork (GTX 970): 30-Jahres-Lauf.

## Termine (Wiedervorlage)

- 2026-09-15 — LISA Pathfinder: Selbstregistrierung ab 15.09.
- 2026-12-02 — NOIRLab Speisekammer-Frage (Gaia DR4).

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators und der gemessene Abschluss-Check.
