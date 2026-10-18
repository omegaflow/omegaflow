# Rat-Frage R-2026-10-18-d — Prosa statt Antrag, und: dürfen Linien die Architektur ändern?

**Die Frage in einem Satz:** Die Linien erzählen Architektur-Sorgen als Prosa in ihren Übergaben,
statt sie als Punkt an die Leitstelle/den Rat zu schicken — und sie ändern Architekturdateien
direkt. Wie werden beide Verhaltensweisen an der Wurzel geschlossen?

## Die gemessene Lage (2026-10-18)

**A · Prosa statt Antrag**

| Maß | Wert |
|---|---|
| lebende Handover mit eigener `## Riss`-Sektion | 3 |
| „Riss"-Erwähnungen in lebenden Handovers | **25** |
| als „Beobachtung" an future adressierte Blöcke | 2 |

Beispiel (River, wörtlich, an future adressiert):
> *„Beobachtung (Architektur, bewusst nicht gebaut): der Empfänger kann seinen eigenen Eingang
> nicht quittieren — `xfer close` auf einen fremden Token ist ein harter Fehler … Quittung und
> Selbst-Erledigung sind beide `CLOSED`."*
> *„Beobachtung: `register_lookup --addressed <linie>` listet geschlossene Blöcke weiter, `--fired`
> bleibt 0 — Erledigtes liest sich als offen."*

Das sind **zwei echte Architektur-Risse**, benannt, aber **ohne Formular, ohne Entscheid, ohne
Adressaten mit Pflicht**. Sie liegen als Erzählung.

**B · Linien ändern Architektur**

**13 von 202 Linien-Commits** (10 Tage) berühren Systempfade
(`.opencode/`, `tools/gate/`, `bin/`, `opencode.json`, `AGENTS.md`, `docs/concepts/`):

| Linie | Datei |
|---|---|
| **river** | `bin/post_lookup` (neues Werkzeug), `docs/concepts/kanal-ontologie-komplettbau.md`, `docs/concepts/archivar-mathematikerin.md` |
| **mountain** | `bin/secrets_keys` |
| **sensory** | `bin/all_wide`, `bin/secrets_keys`, `docs/concepts/eigene-ephemeride.md` |
| **mycelium** | **`tools/gate/src/bin/commit_check.rs`** (der Gate), `docs/concepts/tools-map.md` |

Der mycelium-Fall ist der schärfste: **der Gate-Code wurde von einer Linie geändert** (rustfmt),
während der Gate die Linien prüft.

## Die Unterscheidung, um die es geht

**Bauen ≠ Entscheiden.** Eine Linie *darf* einen Fehler im Gate reparieren oder ein Werkzeug
ergänzen — sie *darf nicht* entscheiden, was das Gate **misst** oder welche Architektur gilt.
Die gemessenen 13 Commits sind teils Bau (rustfmt, Werkzeug), teils Entscheidung
(`post_lookup` erfindet einen neuen Marker, `tools-map` ändert die Werkzeugkarte).

## Die Fragen

1. **Ist Prosa in der Übergabe eine gültige Zustellung** eines Architektur-Punkts — oder muss er
   als Formular/RAT-ID laufen (wie „kein Verdikt ohne Adresse")?
2. **Darf eine Linie Architekturdateien ändern** — und wenn ja, welche (Bau vs. Entscheidung)?
3. **Wie wird die Grenze durchgesetzt** — `edit`-deny auf Pfade (Permission), ein eigenes
   Architektur-Verzeichnis (Struktur), oder eine Commit-Regel „Architekturpfad braucht RAT-ID" (Gate)?
4. **Was geschieht mit den 13 Änderungen** — ratifizieren, zurückbauen, oder Fall für Fall prüfen?

## Die Optionen

- **A — Permission-Wand.** Linien-Agenten bekommen `edit: deny` auf `bin/`, `tools/gate/`, `.opencode/`,
  `docs/concepts/`, `AGENTS.md`, `opencode.json`. Jede Änderung geht als Dispatch an future.
  *Kosten:* Mountain kann keine Werkzeuge mehr bauen — es sei denn, die Wand ist pfadgenau.
- **B — Gate-Regel.** `commit_check` verlangt für jeden Architekturpfad eine **RAT-ID** in der
  Commit-Nachricht. Bau (rustfmt, Bugfix) bleibt frei, Entscheidung braucht ein Verdikt.
- **C — Struktur.** Architektur zieht in ein Verzeichnis/Repo, das nur Future/Operator schreibt;
  die Linien lesen es, ändern es nie.

**Der Vorbehalt, der in jede Antwort gehört:** Der Rat hat gerade (R-2026-10-18-c) entschieden, dass
ein Pass **sofort delegiert**. Ein Subagent auf einem freien Modell hat aber **dieselben Rechte wie
die Linie** — eine Permission-Wand muss also auch für Subagenten gelten, sonst umgeht der Pass sie.

Bitte: **Verdikt + der stärkste Riss.** Keine Umsetzung — die ist Folge, nicht Frage.
