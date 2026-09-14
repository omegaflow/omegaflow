<!--
  title: Handover — Entscheid-Folge VI (Stand 2026-09-14)
  session: Entscheid-Folge VI
  class: handover
  date: 2026-09-14
  sha256: 6f08de54372e3ee537f826398cc81e962b47aa846e9c55f5a78ae45b85e9b593
  status: live
-->
# Handover — Entscheid-Folge VI (2026-09-14)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

Der erste offene Abschnitt benennt den härtesten undatierten Punkt (datierte
Wiedervorlagen schweigen vor ihrem Datum). Jeder offene Punkt trägt seinen
nächsten Schritt in derselben Zeile — Werkzeug, Datei, URL oder Anfrage;
„Schritt unbekannt — erste Messung: X" ist ein vollständiger Schritt. Kein
Dokument wächst ohne Messung; die Droh-Sprache ersetzt den Schritt nicht.

Dies ist die Entscheid-Linie: hier steht nur, was diese Linie autonom trägt —
Tasks, die nicht autonom hier erfolgen können, sind in das Handover ihrer Linie
überführt. Wiedervorlage gilt nur für Termine; „Warten auf Rückmeldung" und
„Pausiert" tragen kein Datum.

## Warten auf Rückmeldung (extern gebunden — kein Datum)

- adoption — Drei-Mail-Block (Toth/Turyshev/Markwardt) nicht gesendet; hängt an
  Merge-Fix + Bande-Split (Schritt: Merge-Fix + Bande-Split in den Bau-/Forschungs-
  Linien; dann die drei Mails senden).
- GitHub Support — User→Org / HTTP 422: Ticket ist raus, Antwort offen (Konto
  weiter `type: User`). (Schritt: Postfach auf die Support-Antwort prüfen)
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
    angefordert (2026-09-13), aber keine Reset-Mail im Ledger (gemessen) →
    Reset-Link erneut anfordern; Login bis Email-Verify blockiert.
- BiSON/Broomhall, CSES-Limadou, NSE/Haug — Anfragen raus, Antwort offen.

## Pausiert (kein Datum — Operator meldet sich)

- Desktop-Fork (GTX 970): 30-Jahres-Lauf.

## Termine (Wiedervorlage)

- 2026-09-15 — LISA Pathfinder: Selbstregistrierung ab 15.09.
- 2026-12-02 — NOIRLab Speisekammer-Frage (Gaia DR4).

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`).
