---
description: Starts the newest open Bau handover — plan and work it off, no deferral.
---

Starte die Bau-Linie. Ist ein Name genannt, nimm diesen; sonst die neueste offene Bau-Übergabe.

Name (leer = neueste): $ARGUMENTS

Neueste offene Bau-Übergabe:

!`for f in $(git ls-tree --name-only HEAD docs/handover/ | grep -E 'handover-.*bau'); do printf '%s %s\n' "$(git log --diff-filter=A --format=%ct -1 -- "$f")" "$f"; done | sort -rn | head -1 | cut -d' ' -f2`

Lies sie, nenne die offenen Punkte, plane das Atom. Planungs-Pass: `register_lookup --live` (offene Punkte über alle lebenden Dokumente) + `git_safety --snapshot` (Arbeitsbaum-Sicherheitsnetz). Dann arbeite die Punkte ab — so viele wie möglich (Sub-Agenten tragen eigenen Kontext); keine neuen Register, keine Befunde, kein Aufschieben. Erledigtes wird gelöscht (git trägt es).

Bau trägt Code: `cargo check` muss null Fehler UND null Warnungen liefern; neue Quellen-Dateien bauen auf `src/archivar` und `src/mathematikerin` als Struktur-Vorlage; jede gefundene Fabrikation wird Gate-Fixture + Gate-Test im selben Atom. Delegiere: grind-flash für Mechanik, grind-pro für Urteil/Force-Gate, grind-max für die härtesten Atome (Urteil UND Schreiben in einem Kontext), vision für Figuren/OCR.

Nur eigene Arbeit: bei geteilten Dateien nur die eigenen Hunks; committet wird pfad-begrenzt (`git commit <eigene Pfade> -m "…"`, nie ein nacktes `git commit`), nur der eigene Teil; fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward) — ein Push sendet nur Commits, der Arbeitsbaum darf schmutzig sein.
