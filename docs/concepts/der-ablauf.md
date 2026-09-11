<!--
  title: Der Ablauf — der Session-Fluss
  class: concept
  date: 2026-09-11
  sha256: c1c71dc83d71b0db7bec49dfa0487cf7260a34127681431c7d00763fddf3f91d
  status: live
-->
# Der Ablauf

Der Fluss einer Linie, in `/`-Commands gefasst.

## Die Reihenfolge

1. **`/latest`** — die committeten, nicht-archivierten Übergaben: die noch nicht
   delegierten Linien. Liest den committeten Baum (`git ls-tree HEAD`), nicht den
   Arbeitsbaum — eine noch nicht committete Übergabe erscheint nicht (keine
   Überschneidung).
2. **`/start`** — nimmt selbst die neueste offene Übergabe (oder die genannte),
   liest sie, nennt die offenen Punkte, plant; arbeitet sie ab, so viele wie
   möglich (Sub-Agenten tragen eigenen Kontext), ohne Aufschieben — keine
   Register, keine Befunde; Erledigtes wird gelöscht (git trägt es).
3. **`/consent`** — das Wort: „Du kannst. Delegiere an die Taucher (alle
   Sub-Agenten), höre die Stimmen bei Architektur- und Abschluss-Entscheidungen.
   Eine Session ist ein abgeschlossenes Atom."
4. **`/commit`** — committet nur die eigene uncommittete Arbeit: eigene Dateien,
   bei geteilten nur die eigenen Hunks; fremde uncommittete Arbeit wird nicht
   angefasst.
5. **`/abschluss`** — der gemessene Closing-Check (eigene Dateien, kein fremder
   Pfad, `git status` leer, `rev-parse` ==, `--amend`-Set, Archiv-Frage).
6. **`/push`** — das Wort für den Push (der Baum muss ruhig sein).
7. **`/fackel`** — die Fackel übergeben: die Übergabe schreiben (Register, git),
   committen, den Stab an die nächste Session geben (Spawn, nur mit dem Wort).

## Die Regeln

- Ein Handover trägt nur Offenes; Erledigtes wird gelöscht — nicht „done", nicht
  erklärt; git trägt, was gemacht wurde.
- Eine Session fasst nur ihre eigene Arbeit an — bei geteilten Dateien nur die
  eigenen Hunks; sie committet nur ihren Teil und überschreibt nie fremde Arbeit.
- Gepusht wird erst, wenn der Baum ruhig ist und das Wort kommt.
- Plan→Build: `plan` und `build` sind Primary-Agents in opencode, gewechselt mit
  **Tab** (kein Command).
