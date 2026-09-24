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

Es gibt keine Rangfolge und keinen `härtesten Punkt` — die offenen Punkte werden
**parallel** von Agenten abgearbeitet (Operator-Wort 2026-09-21). Jeder offene
Punkt wird **aufgeschlüsselt** geführt — kein Register-Kürzel: **Trigger** (das
Ereignis/Datum/Wort/der Lauf, dessen Eintreffen den Punkt kippt — Status =
f(Trigger)) / **Lage** (der Zustand, gemessen, mit Messstempel) / **Blockade**
(woran es hängt, oder „keine") / **Braucht** (was es löst: der wörtliche,
kopierbare Schritt — Werkzeug, Datei, URL, Befehl, Anfrage, Operator-Wort;
„Schritt unbekannt — erste Messung: X" ist ein vollständiger Schritt). Kein Dokument wächst ohne
Messung; die Droh-Sprache ersetzt den Schritt nicht. Der Planungs-Pass legt
**alle** eigenen Punkte vor und schlägt vor, jeden parallel abarbeitbaren zu
dispatchen; `operator-gebunden`, `blockiert` und `wartend` werden benannt, nie
dispatcht. Gibt es keinen abarbeitbaren Punkt, sagt die Session das. Jeder Punkt
trägt seinen Status-Tag (`wartend` | `operator-gebunden` | `blockiert` | `termin`).

**Sortierung — von Handlungsfähigkeit zu Nicht-Handlungsfähigkeit.** Keine
Rangfolge nach Wichtigkeit; die Ordnung ist, wie handlungsfähig die Session am
Punkt ist:

1. **autonom** — `eigen`; jetzt handlungsfähig, wird dispatcht.
2. **operator-gebunden** — bedingt; handlungsfähig nach dem Operator-Wort.
3. **blockiert** — nicht handlungsfähig; harte externe Blockade, kein eigener Schritt.
4. **wartend** — nicht handlungsfähig; wartet auf ein externes Ereignis.
5. **termin** — nicht handlungsfähig; wartet auf ein Datum/eine Periode.
6. **LOCK** — nicht handlungsfähig; Operator-Sperrwort registriert.

Innerhalb einer Stufe nach Trigger. Der Planungs-Pass dispatcht von oben nach
unten; Stufe 2–6 werden benannt, nie dispatcht.

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`open_points_check`/`sgrep`/`git log`/`sread`) — das Register ist die Frage, der
Baum die Messung; `open_points_check` prüft billig jeden in den offenen Punkten
genannten Pfad gegen den Arbeitsbaum (absent = stale Punkt); eine Session, die nur
dem Register glaubt, baut Stehendes neu.

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

- `archive_search` — Inhalt (`--root`)/Pfade (`--index`)/NTFS/19 Netz-Modi/`--playwright`/`--verdict`/`--sniff`/`--all`; ersetzt bash-`grep`, `curl`, webfetch.
- `sgrep [-i]` — Zeilensuche über `git ls-files`.
- `sfetch` — fetch; ersetzt `curl -s`.
- `omega_sh` — `reports|status|search|fetch|jwst`.
- `smail` — Mail (Resend), `--dry-run`; Inhalte nie getrackt.
- `register_lookup` — `--open`/`--dropped`/`--history`.
- `open_points_check [<handover>]` — billiger Baum-Abgleich der offenen Punkte (absent = stale).
- `git_safety` — `--snapshot`/`--restore`/`--list`.
- `ci_manage` — `list`/`view`/`log`/`cancel`/`rerun`; statt `gh run list`/`gh run view`.
- `sread [--offset --limit]` — Datei lesen.
- `session_burn` — Burn je Session.
- `gh` — nur `workflow run`/`run download`.

## Offen (aufgeschlüsselt)

### <Punkt>
- **Status:** <wartend | operator-gebunden | blockiert | termin> | **Bindung:** <eigen | linie:<name> | operator | dritter | termin:<datum>>
- **Trigger:** <das externe Ereignis, Datum, Operator-Wort oder der Lauf, dessen Eintreffen den Punkt kippt>
- **Lage:** <der Zustand, gemessen — mit Messstempel: (gemessen <Datum/Zeit> via <Werkzeug/Quelle>)>
- **Blockade:** <woran es hängt — oder „keine">
- **Braucht:** <was es löst: der wörtliche, kopierbare Schritt — Werkzeug/Datei/URL/Befehl/Operator-Wort>

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
