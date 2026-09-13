<!--
  title: Handover — Entscheid-Folge IV (Stand 2026-09-13)
  session: Entscheid-Folge IV
  class: handover
  date: 2026-09-13
  sha256: f456a604dc5be9ee3d3f7940ca7f8e3541088a349eeec6f943123967e6f9101a
  status: live
-->
# Handover — Entscheid-Folge IV (2026-09-13)

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
- `ivoa` — (a) Re-Invite offen (`omegaflow` kein Mitglied, 404); die
  msdemlei-Einladung (Mail 2026-09-10) ist im `omegaflow`-Konto **nicht**
  sichtbar (Org-/Repo-Einladungen `[]`) — als `johannestyroller` prüfen.
  (b) `ivoa/uvor` existiert; der vo-tap-Crate-Push hängt an der Mitgliedschaft.
- ned-objdir — Bulk-z angefragt, IPAC-Auto-Bestätigung 2026-09-09, inhaltliche
  Antwort offen; der öffentliche TAP lebt.
- NOIRLab Data Lab — **abgelehnt** (Nikutta); TAP anonym offen. Offen bleibt nur
  die Speisekammer-Frage (Wiedervorlage 2026-12-02, Gaia DR4).
- Rubin RSP-Datenrechte — Konto **neu registriert** (2026-09-13, CILogon/GitHub,
  Username `jtyroller`, Email `code@omegaflow.space` verifiziert), Antrag in
  Prüfung; die „Confirming your data rights"-Mail kommt jetzt an (Empfänger
  läuft). CDN-seitig `decline redistribution`.
- TOAR — Registrierung nicht nötig (WOUDC liefert dieselben WMO-Daten).
- BiSON/Broomhall, CSES-Limadou, NSE/Haug — Anfragen raus, Antwort offen.
- adoption — Drei-Mail-Block (Toth/Turyshev/Markwardt) nicht gesendet; hängt an
  Merge-Fix + Bande-Split.

## Korrespondenz-Kanal (repariert 2026-09-13)

- Empfänger `smail_recv`: Binary fehlte (`203/EXEC`) + Unit-State-Pfad falsch →
  gebaut + Units auf `%h/projects/omegaflow/state` gezogen; Health `ok`, Eingang
  gemessen (Test-Mail im Ledger).
- Worker `omegaflow-email-recv` um `message.forward("[redacted-email]")`
  erweitert + deployt (Version `654fc291`): jede Mail an `code@` → **Proton +
  Webhook → Ledger**. Catch-all steht auf `Drop`.
- Resend: `omegaflow.space` (EU, `eu-west-1`) verifiziert; `RESEND_API_KEY`
  gesetzt; `smail` sendet von `code@omegaflow.space` (gemessen).
- Brave Search API: `BRAVE_API_KEY` (Search-Plan) in `.secrets.local`;
  `archive_search --brave` liefert echte Treffer.

## Offene Bau-/Register-Punkte

- `aia2014_lines.bin` (CDN-Release `jsoc.stanford.edu`) ist **nicht** in
  `sources.φ` registriert → Register-Schuld.
- Split-Routing: `Table = off` + DNS aus in den 5 Proton-Configs gesetzt; die
  Verifikation direct↔Tunnel (`./bin/proton-exit.sh ca` neu hochfahren) und die
  8 `000`-Hosts je Exit stehen aus.
- `.playwright-mcp/` (MCP-Artefakte) gehört in `.gitignore`.
- opencode-Settings (Provider-Whitelists, free-Agenten, Playwright-MCP,
  opencode-browser-Plugin) liegen außerhalb des Repos; Backup unter
  `~/backup/opencode-settings-2026-09-13/`.

## Pausiert (kein Datum — Operator meldet sich)

- Desktop-Fork (GTX 970): 30-Jahres-Lauf.

## Termine (Wiedervorlage)

- 2026-09-15 — LISA Pathfinder: Selbstregistrierung ab 15.09.
- 2026-12-02 — NOIRLab Speisekammer-Frage (Gaia DR4).

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators und der gemessene Abschluss-Check.
