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

**Keine Geschichts-Abschnitte (Operator-Wort 2026-09-25).** Das Handover trägt nur
Offenes — ein Stehender-Pass-Ergebnis, ein „In diesem Atom geschlossen"-Register,
ein Benchmark oder ein „Geteilter Baum" sind Historie und gehören nicht hinein:
git trägt, was gemacht wurde. Geteilter externer Zustand lebt in
`docs/zustand/external-state.md`, nie als Kopie im Handover; nach dem Löschen der
Historie bleibt allein die offene Punkt-Liste — ist sie leer, ist das Handover leer.

Es gibt keine Rangfolge und keinen `härtesten Punkt` — die offenen Punkte werden
**parallel** von Agenten abgearbeitet (Operator-Wort 2026-09-21). Jeder offene
Punkt wird **aufgeschlüsselt** geführt — kein Register-Kürzel: **Trigger** (das
Ereignis/Datum/Wort/der Lauf, dessen Eintreffen den Punkt kippt — Status =
f(Trigger)) / **Lage** (der Zustand, gemessen, mit Messstempel) / **Blockade**
(warum es hängt, oder „keine") / **Braucht** (was es löst: der wörtliche,
kopierbare Schritt — Werkzeug, Datei, URL, Befehl, Anfrage, Operator-Wort;
„Schritt unbekannt — erste Messung: X" ist ein vollständiger Schritt). Kein Dokument wächst ohne
Messung; die Droh-Sprache ersetzt den Schritt nicht. Der Planungs-Pass legt
**alle** eigenen Punkte vor und dispatcht **jeden Schritt bis zur Kante**; nur der
Akt am Gegenüber bleibt benannt (die Vorbereitung eines `blockiert`/`wartend`-Punkts
bis zur Kante wird dispatcht wie jeder Schritt). Gibt es
keinen Schritt zur Kante, sagt die Session das. Jeder Punkt trägt seinen
Status-Tag (`wartend` | `operator-gebunden` | `blockiert` | `termin`).

**Vorbereitung ≠ Akt (Operator-Wort 2026-09-24).** Ein `operator-gebundener`
Punkt wird **immer** in zwei Zeilen getrennt geführt, nie in einer: die
**Vorbereitung** ist autonom (`eigen`), läuft bis zur Kante — Draft, Adresse,
QUELLEN, `smail --dry-run`, Feldmap, Messung, Kantenzeile (Artefakt |
Ausführbefehl | Wort erwartet) — und wird dispatcht; `operator-gebunden` ist
**allein der Akt** (senden/absenden/signieren/urteilen/wählen). Eine
`operator-gebunden`-Zeile ohne abgetrennte Vorbereitungs-Zeile behauptet den
ganzen Prozess als gesperrt und droppt die Vorbereitung — ein Registraturfehler.
Es gibt keinen „kompletten Prozess operator-gebunden". **Benannter Ausnahmefall
(Rat 2026-09-24):** wo die Vorbereitung selbst operator-gebunden ist — das Urteil,
die Identitätswahl, kein autonomer Schritt existiert — steht **eine** Zeile mit
`operator-gebunden` auf der Vorbereitung, Trigger = Operator-Wort; sie wird nicht
in zwei Zeilen gespalten, weil die Maschine keine Kante hat. Jede Vorbereitungs-
zeile trägt ihren Lage-Stempel `(gemessen …)` — eine ungemessene Vorbereitungszeile
driftet wie die alte Sammel-Zeile.

**Sortierung — logisch nach Akteur; kein Punkt steht über einem Punkt** (Operator-Wort,
2026-09-26; ersetzt die logisch-chronologische Sortierung). Die Tafel wird logisch
nach **Akteur** gruppiert — **wer handelt**: **Linie** (eigen, die Maschine) |
**Rat** | **Operator** (operator-gebundene Akte + LOCK) | **Dritter** —, nie nach
Thema, nie nach Wichtigkeit, nie nach Chronologie, nie über eine gemischte Leiter,
und innerhalb einer Gruppe in keiner Rangfolge.
Alle offenen Punkte stehen gleich; kein Punkt steht über einem anderen. Der Akteur
ist die Logik, nicht die Reihenfolge.
Die Status-Tags sind die Dispatch-Achse: `autonom` (`eigen`, wird dispatcht)
→ `operator-gebunden` (nur der Akt nach dem Operator-Wort) → `blockiert` →
`wartend` → `termin` → `LOCK`. **Jeder Punkt wird bis zur Kante gearbeitet (Kante =
die Konsensgrenze, die Grenze der eigenen Domäne):**
jeder Punkt wird autonom bis zu seiner Kante vorbereitet (Entwurf, gemessener
Trigger, fertiges Formular, gebautes Artefakt); nur der Akt am Gegenüber bleibt
benannt — kein Punkt wird bloß benannt, wo ein Schritt zur Kante existiert.

Der Planungs-Pass dispatcht jeden Schritt bis zu seiner Kante; nur der Akt am
Gegenüber bleibt benannt — die Vorbereitung jedes Punkts, auch eines
`blockiert`/`wartend`-Punkts, wird bis zur Kante dispatcht. Wo kein Schritt
zur Kante existiert, sagt der Pass es klar (kein Scheinschritt aus einem Warten).

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`open_points_check`/`sgrep`/`git log`/`sread`) — das Register ist die Frage, der
Baum die Messung; `open_points_check` prüft billig jeden in den offenen Punkten
genannten Pfad gegen den Arbeitsbaum (absent = stale Punkt) und meldet
`format-gap`, wo eine Punkt-Zeile fehlt oder die `Lage` keinen Messstempel
`(gemessen …)` trägt; eine Session, die nur dem Register glaubt, baut Stehendes neu.

**Kein Punkt wird dem Operator vorgelegt, bevor er gegen echten Code und git
gemessen ist.** Eine `Lage` ohne `(gemessen … via <Werkzeug/Quelle>)` ist keine
Vorlage, sondern eine Registraturpflicht; eine ungemessene Behauptung wird gemessen
oder als `pending`/`unverified` benannt. Ein bereits mit **Wort** entschiedener
Punkt wird nicht erneut als Frage vorgelegt — nur eine neue Messung öffnet ihn neu.

**CDN-Claims sind Register-Behauptungen.** Eine `Lage`-Aussage über ein CDN-Asset
(die `url`-Zeile in `phi/sources.φ`) ohne `archive_search --verdict`/`--sniff` im
selben Atom ist ungemessen: der Start verifiziert nicht alle Assets, nur die
fälligen (Zustands-Ledger). Ein Asset-Zustand ohne Messung ist `pending`, nie
geerbt als gültig.

## Stehender Pass (automatisch, keine Auswahl)

Die fälligen Einträge aus `docs/zustand/external-state.md` werden zu
Session-Beginn gemessen, bevor die Auswahl steht — ihr Ausgang verändert die
Auswahl, sie sind kein Auswahlpunkt. Ergebnis direkt in dieses Handover + den
Zustand-Ledger. Karte: `docs/concepts/tools-map.md` — bei Widerspruch gilt `--help`.

- **Postfach** — `smail` + `state/mail/mail_ledger.φ` (fällig 2⁶ min).
- **CI-Status am HEAD** — zuerst den Watchdog-Snapshot
  `/tmp/opencode/ci_status.md` lesen (kein API-Aufruf), **aber nur wenn er jünger
  ist als der letzte HEAD-Wechsel** (`stat -c %y /tmp/opencode/ci_status.md` gegen
  `git log -1 --format=%cI`); sonst ist der Snapshot eine Aussage von vor dem
  Commit → `ci_manage list` erzwingen. Bei Lücke/Detail
  `ci_manage list` / `ci_manage view <id>`, Fehllog `ci_manage log <id>`.
  **Nie** `gh run list`/`gh run view`; `gh` nur für `workflow run`/`run download`.
- **Artefakt-Frische (alle erzeugten Klassen)** — nichts Erzeugtes ist HEAD;
  jede Klasse ist ein CI-/Deploy-Produkt und hinkt: **Session-Tools**
  (`tools-latest`), der **Core-Bin** (`target/release/omegaflow`), die
  **CDN-Daten-Assets** (harvest-/measure-Compiler → `omegaflow/sources`), die
  **WGSL-Kernel** (`kernel-flatten`), die **Firmware**. Für jede Klasse, die
  diese Session als Verhalten liest, gilt: den Artefakt-Build-Commit gegen
  `git rev-parse HEAD` halten (`sread target/release/.tools_manifest --limit 1`
  für die Tools; den jüngsten Workflow-Lauf via `ci_manage list` für die
  übrigen). Liegt der Build hinter HEAD, ist die Aussage **stale** — `pending`,
  nie als aktuell. Nach jedem Push `gh workflow run <workflow>` (tools-build,
  kernel-flatten, der betroffene `*-cdn`); `bin/.tools_ensure <tool>` prüft
  sha256-content-addressed. Kein Satz über ein Artefakt hinter HEAD.

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
- **Wort:** <das gegebene Operator-Wort> | <Datum> | <Quelle> — nur bei entschiedenen `operator-gebunden`-Punkten; sonst weglassen

Eine gegebene Entscheidung wird als `**Wort:**`-Zeile registriert, **bevor** sie
ausgeführt wird — ein Punkt mit eingetragenem Wort wird nicht erneut vorgelegt. Der
Rest einer umgesetzten Entscheidung wird als **eigener offener Punkt** geführt, nie
weggelassen (der Key-Rotation-Rest: `http_401` nach Rotation bleibt ein Punkt).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
