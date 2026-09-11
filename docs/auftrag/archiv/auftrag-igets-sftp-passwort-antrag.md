<!--
  title: Anfrage (extern) — IGETS: SFTP-Passwort für das Datennutzer-Konto
  class: auftrag
  date: 2026-09-10
  sha256: 6b251f51e3e9d76a3770e7bedd533fdcaea22a7b1f32d8f638146fdf687ab45c
  status: done
  see-also: docs/handover/handover-2026-09-11-ernte.md
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

## Ausgang (gemessen 2026-09-11)

Die GFZ-Antwort (2026-09-10, Nico Stolarczuk) riet zu Delete/Neu-Registrierung,
weil das SFTP-Passwort nicht abrufbar sei. Die einzige Registrierungs-Mail des
Operators („Your profile was confirmed and successfully created.", SFTP-Link
`sftp://johannes.tyroller_at_proton.me@igetsftp.gfz.de`) trägt keinen
Delete-Link — die Route ist so nicht gangbar.

Die Messung 2026-09-11 zeigt: der SFTP-Zugang funktioniert mit dem hinterlegten
Passwort (`IGETS_USER`/`IGETS_PASS` in `.secrets.local`, 23 Zeichen).
`curl -u johannes.tyroller_at_proton.me:<pw> sftp://igetsftp.gfz.de/` listet die
Stations-Ordner (40+). Der „Permission denied" der Messung 2026-09-07 traf die
noch unbestätigte Registrierung; nach der GFZ-Bestätigung authentifiziert
dasselbe Passwort. Kein neues Passwort, kein neues Konto nötig.

## Register-Pflicht

Erledigt: der Kanal ist offen. Der Eintrag in `phi/blocked_sources.φ` ist
gestrichen; die Ernte steht in `docs/handover/handover-2026-09-11-ernte.md`.
