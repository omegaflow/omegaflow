---
description: Sensory-Linie — Forschung und Messung, Proben, Papiere; liest die Maschinen- und die Menschenwelt. Planungsmodus (read-only): Auswahl vorschlagen, dann Ausführung via line-Agent.
agent: plan
---

Starte die Sensory-Linie im **Planungsmodus** (Agent `plan`, read-only). Ist ein Name genannt, nimm diesen; sonst die neueste offene Sensory-Übergabe.

Name (leer = neueste): $ARGUMENTS

Neueste offene Sensory-Übergabe:

!`for f in $(git ls-tree --name-only HEAD docs/handover/); do case "$f" in *sensory*) printf '%s %s\n' "$(git log --diff-filter=A --format=%ct -1 -- "$f")" "$f";; esac; done | sort -rn | head -1 | cut -d' ' -f2`

**Kein Standard-Pass.** Der gemessene Rundenzustand liegt im Stehenden Pass:
`sread state/zustand/standing-pass.md` — einmal lesen, zitieren, nie kopieren
(eine kopierte Pass-Zahl ist ein Gate-Fixture `pass-copy`). Gemessen wird nur,
was der eigene Trigger für fällig erklärt.

**Phase 1 — Plan (nur das).** Lies die Übergabe + `open_points_check <übergabe>` (billiger
Baum-Abgleich). Nenne alle offenen Punkte der eigenen Linie als Tafel (nur `eigen`) und schlage
vor, jeden parallel abarbeitbaren zu dispatchen — keine Rangfolge, kein „härtester Punkt".
Kein edit/write/commit. **Halte dann an.**

**Phase 2 — Ausführung.** Nach `/consent` (oder `/sensory_go`) → `line`-Agent. Arbeite die
eigenen Punkte bis zur Kante; Operator-Akte → Future-Operator-Queue, Dritt-Waits →
`state/zustand/wartend.φ` (Trigger + Aufnehmer). Werkzeuge statt Rohbefehle
(`docs/concepts/tools-map.md`); Header-sha256 via `omega_sh sha <datei>`; vor dem Commit
`git_safety --close [<eigene Pfade>]`; Delegiere flash-first (grind-flash/general/vision);
pro nur bei gemessen falschem flash-Ergebnis oder benanntem Hart-Atom. `/commit` schließt.
Sensory liest die Hardware nur nach Operator-Wort; jedes Aufzeichnen fragt vorher (Sensor-Konsens); Korrelat ≠ Erleben; Figuren/OCR via `vision`.

**Linien-Preset (`archive_search`).** Das Tool ist öffentlich und linien-blind; das Preset ist
**privat** (nur diese Linie) und liegt in `state/sensory/archive-search-preset.txt` — hier
eingelesen:

!`cat state/sensory/archive-search-preset.txt`

Nur eigene Arbeit: bei geteilten Dateien nur die eigenen Hunks; pfad-begrenzt committen
(`git commit <eigene Pfade> -m "…"`, nie ein nacktes `git commit`); fremde uncommittete
Arbeit nie überschreiben; gepusht wird, sobald der eigene Commit steht und `origin/main`
Vorfahr von HEAD ist.
