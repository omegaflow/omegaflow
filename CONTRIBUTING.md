# Beitragen

## Bevor du etwas änderst

Lies `AGENTS.md` und die Dokumente in `bindend/` des Werkstatt-Repositories. Sie
sind kurz und sie gelten: keine `allow`s, keine Testlücken, keine Tests, die nur
die Existenz prüfen.

## Der Weg einer Änderung

1. **Messen.** Was ist der Ist-Zustand, mit welchem Befehl, mit welchem Datum?
2. **Bauen.** Die Änderung im selben Commit wie ihr Test.
3. **Abschließen.** `RUSTFLAGS="-D warnings" cargo check --workspace` und
   `cargo clippy --workspace --all-targets` müssen ohne Befund durchlaufen. Ein
   Cache-Treffer zählt nicht: Clippy muss wirklich übersetzen.
4. **Committen.** Nachricht in Englisch, ein Satz je Sache, die Zahl mit ihrem
   Befehl. Deutsch trägt die Dokumente, Englisch den Code.
5. **Pushen.** Kein grüner Lauf, keine Behauptung.

## Was ein Commit nicht darf

- Werte erfinden: `0.0`, `unwrap_or_default`, erfundene Toleranzen.
- Eine Prüfung abschalten statt eine Ursache zu beheben.
- Eine Testlücke lassen: jedes Paket aus `[workspace] members` läuft in der
  CI-Matrix, kein `--bin`-Filter.

## Fragen und Streit

Der Rat entscheidet Architektur und Ethik. Ein offener Punkt gehört als Datei in
`rat/` des Werkstatt-Repositories — nicht in ein Issue, das nach drei Tagen
untergeht.
