<!--
  title: Anfrage (extern) — IGETS: SFTP-Passwort für das Datennutzer-Konto
  class: auftrag
  date: 2026-09-10
  sha256: 247bccc250adc9691e902e340cbfeddcafac1cec80aea1e8e39879f46839c103
  status: live
  see-also: docs/handover/handover-2026-09-10-nicht-autonom.md
-->
# Anfrage — IGETS: SFTP-Passwort

Gesendet am 2026-09-10 (Operator).

Der Datenweg ist SFTP-only (`igetsftp.gfz.de`, gemessen 2026-09-10: die
data-access-Seite nennt „download is only possible via encrypted sftp"; der
frühere FTP ist abgeschaltet). Der Login ist die E-Mail mit `@`→`_at_`
(`johannes.tyroller_at_proton.me`). Das Website-Passwort ist NICHT das
SFTP-Passwort — der authentifizierte Versuch antwortet „Permission denied
(publickey,password)". Das Konto existiert (die Registrierung meldet „Value of
field Email is already in use").

Empfänger (gemessen): igets-support@gfz.de.

## Text (englisch, sendfertig)

Subject: IGETS SFTP access — password for my data user account

Dear IGETS support team,

I hold a registered IGETS data user account and would like to download the
superconducting gravimeter (SG) data.

The data-access page states that download is only possible via SFTP on
igetsftp.gfz.de, and that one logs in with the account name formed by replacing
"@" with "_at_" in the email address. My account is:

- SFTP login: johannes.tyroller_at_proton.me
- Email: johannes.tyroller@proton.me

The password stored with my website account does not authenticate on the SFTP
server — the login attempt returns "Permission denied (publickey,password)".
The former plain-FTP access is switched off, so SFTP is the only route.

Could you please send me the SFTP password for my account? (If a public SSH key
is the preferred route for data users, I can provide one instead.)

My account details:
- Name: Johannes Tyroller
- Email: johannes.tyroller@proton.me
- Affiliation: Independent
- Address: Corneliweg 1, 79875 Dachsberg, Germany

With thanks,
Johannes Tyroller

## Register-Pflicht

Die Antwort (SFTP-Passwort oder SSH-Key-Route) ist die Messung. Bis dahin bleibt
der Kanal `blocked account` (kein offener Datenweg). Nach Erhalt: SFTP-Zugang
messen (Port 22, Loginname + Passwort), dann die 25 Stations-DOIs über
`igetsftp.gfz.de` ernten.
