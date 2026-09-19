<!--
  title: Handover — <Titel> (Stand {date})
  session: <Sitzungstitel — wie die Session heißt, die diese Linie fährt>
  class: handover
  date: {date}
  sha256: <hex — über den Body ohne Header: sed '/^<!--/,/^-->/d' <f> | sha256sum>
  status: live
-->
# Handover — <Titel> ({date})

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
Der Planungs-Pass nennt die offenen Punkte als nummerierte Auswahl (der erste ist
der härteste undatierte); die Session arbeitet so viele ab wie möglich.
Wartestellungen (`wartend`) sind kein Auswahlpunkt — sie nennen nur ihren Auslöser
und werden nie als Handlungsschritt geführt; gibt es keinen abarbeitbaren
undatierten Punkt, sagt die Session das. Jeder Punkt trägt seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## Stehender Pass (automatisch, keine Auswahl)

Die fälligen Einträge aus `docs/zustand/external-state.md` werden zu
Session-Beginn gemessen, bevor die Auswahl steht — ihr Ausgang verändert die
Auswahl, sie sind kein Auswahlpunkt. Ergebnis direkt in dieses Handover + den
Zustand-Ledger. Karte: `docs/concepts/tools-map.md` — bei Widerspruch gilt `--help`.

- **Postfach** — `smail` + `state/mail/mail_ledger.φ` (fällig 2⁶ min).
- **CI-Status am HEAD** — zuerst den Watchdog-Snapshot
  `/tmp/opencode/ci_status.md` lesen (kein API-Aufruf); bei Lücke/Detail
  `ci_manage list` / `ci_manage view <id>`, Fehllog `ci_manage log <id>`.
  **Nie** `gh run list`/`gh run view`; `gh` nur für `workflow run`/`run download`.

## Werkzeuge (gebaut — nutzt sie)

- `archive_search` — Inhalt (`--root`)/Pfade (`--index`)/NTFS/18 Netz-Modi/`--playwright`/`--verdict`/`--sniff`/`--all`; ersetzt bash-`grep`, `curl`, webfetch.
- `sgrep [-i]` — Zeilensuche über `git ls-files`.
- `sfetch` — fetch; ersetzt `curl -s`.
- `omega_sh` — `reports|status|search|fetch|jwst`.
- `smail` — Mail (Resend), `--dry-run`; Inhalte nie getrackt.
- `register_lookup` — `--live`/`--open`/`--history`.
- `git_safety` — `--snapshot`/`--restore`/`--list`.
- `ci_manage` — `list`/`view`/`log`/`cancel`/`rerun`; statt `gh run list`/`gh run view`.
- `sread [--offset --limit]` — Datei lesen.
- `session_burn` — Burn je Session.
- `gh` — nur `workflow run`/`run download`.

## <Sektion — härtester undatierter Punkt zuerst>

- <offener Punkt — mit nächstem Schritt> (Schritt: <Tool/Datei/URL/Anfrage>)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
