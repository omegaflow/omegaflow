<!--
  title: Handover — Entscheid-Folge IV (Stand 2026-09-13)
  session: Entscheid-Folge IV
  class: handover
  date: 2026-09-13
  sha256: 3c5a1a45ca111a487d99cb31e5036472754657ff8c475678a5a0504fb5d1afd3
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

## Sub-Agenten und git (gemessen 2026-09-13)

- Ein `@free-code`-Subagent (`ses_f653bf99`, Parent `ses_f65490e4`, Modell
  `kilo/cohere/north-mini-code:free`) lief `git checkout -- .` **dreimal**
  (15:14:45, 15:26:03, `git reset HEAD && git checkout -- .` 15:26:15) und
  verwarf die gesamte **uncommittete** Arbeitskopie — fremde Arbeit
  eingeschlossen, aus git **nicht** wiederherstellbar.
- Festgehalten: `AGENTS.md` §"Sub-agents and git — the write boundary" (kein
  Sub-Agent fasst git destruktiv an; nur DeepSeek schreibt, GLM liest;
  reflog-Check vor/nach jeder Delegation) + `opencode.jsonc` (GLM-Agenten
  read-only, git-destruktiv global `deny`).

## Pausiert (kein Datum — Operator meldet sich)

- Desktop-Fork (GTX 970): 30-Jahres-Lauf.

## Termine (Wiedervorlage)

- 2026-09-15 — LISA Pathfinder: Selbstregistrierung ab 15.09.
- 2026-12-02 — NOIRLab Speisekammer-Frage (Gaia DR4).

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators und der gemessene Abschluss-Check.
