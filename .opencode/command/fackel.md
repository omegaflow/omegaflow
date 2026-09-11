---
description: Passes the torch — write the handover register, commit it, and hand the baton to the next session.
---

Übergib die Fackel — die Session ist ein abgeschlossenes Atom.

1. **Register (git).** Schreibe die Übergabe
   `docs/handover/handover-YYYY-MM-DD-<slug>.md` aus `docs/handover/_template.md`:
   nur Offenes, Erledigtes gelöscht; `session:`-Feld (wie die Nachfolge-Session
   heißt); sha256 über den Body ohne Header. Die konsumierte Übergabe wandert nach
   `docs/handover/archiv/`.
2. **Commit.** Nur die eigene Übergabe — eigene Dateien, bei geteilten nur die
   eigenen Hunks; fremde uncommittete Arbeit unangetastet. Danach `git show --stat
   HEAD` prüfen.
3. **Stab.** Übergib an die nächste Kybernautin: spawne die Nachfolge-Session aus
   dem Übergabe-Inhalt (`opencode run "<inhalt>" --title "<session>" -m <model>
   --dir <root>`) — nur mit dem Wort des Operators. Ohne das Wort bleibt der Stab
   bei der Übergabe; der Operator startet ihn mit `/start`.
