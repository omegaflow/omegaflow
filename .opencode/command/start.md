---
description: Starts the newest open handover — or the named one — plan and work it off, no deferral.
---

Starte die Übergabe. Ist ein Name genannt, nimm diesen; sonst die neueste offene.

Name (leer = neueste): $ARGUMENTS

Neueste offene Übergabe:

!`for f in $(git ls-tree --name-only HEAD docs/handover/ | grep handover-); do printf '%s %s\n' "$(git log --diff-filter=A --format=%ct -1 -- "$f")" "$f"; done | sort -rn | head -1 | cut -d' ' -f2`

Lies sie, nenne die offenen Punkte, plane das Atom. Planungs-Pass: `register_lookup --live` (offene Punkte über alle lebenden Dokumente) + `git_safety --snapshot` (Arbeitsbaum-Sicherheitsnetz). Dann arbeite die Punkte ab — so viele wie möglich (Sub-Agenten tragen eigenen Kontext); keine neuen Register, keine Befunde, kein Aufschieben. Erledigtes wird gelöscht (git trägt es).

Nur eigene Arbeit: bei geteilten Dateien nur die eigenen Hunks; committet wird pfad-begrenzt (`git commit <eigene Pfade> -m "…"`, nie ein nacktes `git commit`), nur der eigene Teil; fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward) — ein Push sendet nur Commits, der Arbeitsbaum darf schmutzig sein.
