---
description: Entscheid-Linie — Planungsmodus (read-only): Auswahl vorschlagen, dann Ausführung via line-Agent.
agent: plan
---

Starte die Entscheid-Linie im **Planungsmodus** (Agent `plan`, read-only). Ist ein Name genannt, nimm diesen; sonst die neueste offene Entscheid-Übergabe.

Name (leer = neueste): $ARGUMENTS

Neueste offene Entscheid-Übergabe:

!`for f in $(git ls-tree --name-only HEAD docs/handover/ | grep -E 'handover-.*entscheid'); do printf '%s %s\n' "$(git log --diff-filter=A --format=%ct -1 -- "$f")" "$f"; done | sort -rn | head -1 | cut -d' ' -f2`

**Phase 1 — Plan (nur das).** Lies die Übergabe. Planungs-Pass: `register_lookup --live` (offene Punkte über alle lebenden Dokumente) + `git_safety --snapshot` (Arbeitsbaum-Sicherheitsnetz). Nenne die offenen Punkte als kurze nummerierte **Auswahl** und schlage das Atom vor (welche Punkte, welche Delegation, welche Reihenfolge). Kein edit/write/commit, keine Messung, keine Exploration über das Genannte hinaus — der Plan-Agent kann nicht schreiben, das ist die Grenze. **Halte dann an.**

**Phase 2 — Ausführung.** Nach der Auswahl `/consent` (oder `/entscheid_go`) — wechselt auf den auto-bestätigten `line`-Agenten. Der `line`-Agent führt den bestätigten Plan aus; `/commit` schließt. Diese Linie trägt autonom nur Entscheidungen, Korrespondenz und Consent; Quellen-/Bau-Punkte reisen als Nachricht an ihre Linie (nie in ein fremdes Handover). Delegiere: Rat für Architektur, research-max für harte Recherche, vision für Figuren/OCR, grind-* für Bau.

Nur eigene Arbeit: bei geteilten Dateien nur die eigenen Hunks; committet wird pfad-begrenzt (`git commit <eigene Pfade> -m "…"`, nie ein nacktes `git commit`), nur der eigene Teil; fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward) — ein Push sendet nur Commits, der Arbeitsbaum darf schmutzig sein.
