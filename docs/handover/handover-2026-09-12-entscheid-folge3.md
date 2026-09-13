<!--
  title: Handover — Entscheid-Folge III (Stand 2026-09-12)
  session: Entscheid-Folge III
  class: handover
  date: 2026-09-12
  sha256: 5e51c40fd1313ef491e0361bf3c70a9c86eb88ba9ea9f22fb4fbc89b852d7433
  status: live
-->
# Handover — Entscheid-Folge III (2026-09-12)

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

Quelle der echten Korrespondenz: `state/mail/mail_ledger.φ` (gitignored, nur
eingehend) — siehe Korrespondenz-Kanal. Gemessen 2026-09-13.

- GitHub Support — User→Org / HTTP 422: Ticket ist raus, Antwort offen
  (gemessen 2026-09-13: das Konto ist weiter `type: User`).
- `ivoa` — zwei Fäden: (a) Re-Invite `johannestyroller`/`omegaflow` ist raus,
  Antwort offen (gemessen 2026-09-13: `omegaflow` weiterhin kein Mitglied, 404);
  (b) `ivoa/uvor` existiert (gemessen 2026-09-13, angelegt 2026-09-09) — der
  vo-tap-Crate-Push (Seed `tools/vo-tap`, BSD-3) hängt an der Mitgliedschaft,
  nicht am Code.
- ned-objdir — Bulk-z-Zugang angefragt (Auftrag archiviert); IPAC-Auto-Bestätigung
  2026-09-09, inhaltliche Antwort offen. Der öffentliche TAP
  (`ned.ipac.caltech.edu/tap/sync`) lebt (gemessen 200) und steht in `sources.φ`.
- NOIRLab Data Lab — **abgelehnt** (gemessen 2026-09-11, `blocked_sources.φ`:
  Robert Nikutta, „we decided to decline your request"). Derselbe Bescheid: der
  TAP ist öffentlich, kein Konto nötig (LS DR10 + DECaPS offen). Der TAP sync ist
  anonym offen (gemessen 200; das des-dr2-Footprint-Asset nutzt ihn). Offen
  bleibt nur die Speisekammer-Frage (Konsument für statische Mehrband-Farbe;
  Wiedervorlage 2026-12-02, Gaia DR4).
- Rubin RSP-Datenrechte — der Auftrag (`auftrag-rubin-data-rights-antrag.md`,
  2026-09-10) ist **entworfen, noch nicht gesendet** („Operator sendet"); das
  Handover führte „in Prüfung" — der Sendestatus ist mangels Sent-Ledger
  unverifiziert. CDN-seitig `decline redistribution` (proprietäre Frist).
- TOAR — **Registrierung nicht nötig** (Register-Verdikt 2026-08-19,
  `dead_sources.φ`: TOAR ist die Forschungs-Aggregation derselben WMO-Stations-
  daten, die WOUDC liefert; `atmosphere_woudc_total_ozone` ist integriert). Die
  parallele Registrierungs-Anfrage lief weiter (Schröder-Antwort 2026-08-25,
  Projektbeschreibung/CC-BY angefragt); das Vollzugangs-Pending ist damit
  überholt. Anonym nutzbar (API v2, Limit 5 Zeitreihen).
- BiSON/Broomhall — Basu (Yale) verweist an Broomhall (Warwick); die
  Daten-Anfrage (Frequenz-Shift-Serie 1978–2012, Basu et al. 2012 Fig. 2) ist
  raus, Antwort offen.
- CSES-Limadou — Sotgiu (Roma2/INFN); L2-Zugang (EFD/SCM/LAP/HPM) für Konto
  `omegaflow` beantragt (Permission Denied), Freigabe offen.
- NSE/Haug — Keimer (MPI FKF); I(q,t)-Anfrage (Haug et al. 2010) raus, Antwort
  offen.
- adoption — Drei-Mail-Block (Toth/Turyshev/Markwardt, 20-s-Bande): hängt an
  Merge-Fix-Welle + Bande-Split; nicht gesendet (`auftrag-adoption.md`).

## Korrespondenz-Kanal (gemessen 2026-09-13)

- Die echte Provider-Korrespondenz lebt **nur** in `state/mail/mail_ledger.φ`
  (gitignored, 23 Zeilen, letzter Eingang 2026-09-02 14:17) — **nur eingehend**.
  Ausgehende Mails (`smail.rs` via Resend) werden nirgends protokolliert; es gibt
  keine Sent-Ledger. Das Handover trug bisher nur handgetippte Einzelzeilen und
  ließ die realen Fäden (BiSON, CSES-Limadou, NSE, die Verify-Links) aus.
- `smail-recv.service` ist **tot** (gemessen 2026-09-13: `status=203/EXEC`,
  Auto-Restart-Schleife — `target/release/smail_recv` fehlt). Kein eingehendes
  Mail seit 2026-09-02 → die „Antwort offen"-Zeilen sind seit ~09-03
  unverifiziert. Fix: `cargo build --release -p omegaflow-service --bin
  smail_recv` + Service-Restart (schwerer Lauf → CI/Operator).
- Account-Verifikationen aus dem Ledger, offen: DAHITI (Passwort-Reset),
  Copernicus Data Space, ICIMOD RDS, Borealis (Verify-Links); dpa-ID, MAST,
  Brave, Resend = automatische Notizen ohne Aktion.

## Pausiert (kein Datum — Operator meldet sich)

- Desktop-Fork (GTX 970): 30-Jahres-Lauf.

## Termine (Wiedervorlage)

- 2026-09-15 — LISA Pathfinder: Selbstregistrierung ab 15.09.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`).
