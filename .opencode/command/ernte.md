---
description: Starts the newest open Ernte handover — plan and work it off, no deferral.
agent: build
---

Starte die Ernte-Linie. Ist ein Name genannt, nimm diesen; sonst die neueste offene Ernte-Übergabe.

Name (leer = neueste): $ARGUMENTS

Neueste offene Ernte-Übergabe:

!`for f in $(git ls-tree --name-only HEAD docs/handover/ | grep -E 'handover-.*ernte'); do printf '%s %s\n' "$(git log --diff-filter=A --format=%ct -1 -- "$f")" "$f"; done | sort -rn | head -1 | cut -d' ' -f2`

Lies sie, nenne die offenen Punkte, plane das Atom. Planungs-Pass: `register_lookup --live` (offene Punkte über alle lebenden Dokumente) + `git_safety --snapshot` (Arbeitsbaum-Sicherheitsnetz). Dann arbeite die Punkte ab — so viele wie möglich (Sub-Agenten tragen eigenen Kontext); keine neuen Register, keine Befunde, kein Aufschieben. Erledigtes wird gelöscht (git trägt es).

Ernte trägt Quellen: Arbeit läuft über `docs/SOURCE_PORT.md`; die Notwendigkeit einer Quelle entscheidet das Register (`phi/sources.φ` / `blocked_sources.φ` / `dead_sources.φ`), nicht die Korrespondenz. Eine neue/geänderte Ernte wird erst geschlossen, wenn sie in `phi/sources.φ` für die CDN-Manifestation registriert ist. Delegiere: grind-flash für Harvest/Verify/Recheck, grind-pro für Force-Gate/Parser-Gap/Route, research-max für die härtesten Routen, vision für figure-only-Daten (Figur → Tabelle).

Nur eigene Arbeit: bei geteilten Dateien nur die eigenen Hunks; committet wird pfad-begrenzt (`git commit <eigene Pfade> -m "…"`, nie ein nacktes `git commit`), nur der eigene Teil; fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward) — ein Push sendet nur Commits, der Arbeitsbaum darf schmutzig sein.

Consent (autobestätigt): Du kannst. Delegiere an die Taucher (alle Sub-Agenten), höre die Stimmen bei Architektur-/Abschluss-Entscheidungen. Eine Session ist ein abgeschlossenes Atom.
