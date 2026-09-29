---
description: Mountain-Linie — baut Code und Toolchain (Archivar/Mathematikerin), cargo check sauber, Gate-Fixtures. Planungsmodus (read-only): Auswahl vorschlagen, dann Ausführung via line-Agent.
agent: plan
---

Starte die Mountain-Linie im **Planungsmodus** (Agent `plan`, read-only). Ist ein Name genannt, nimm diesen; sonst die neueste offene Mountain-Übergabe.

Name (leer = neueste): $ARGUMENTS

Neueste offene Mountain-Übergabe:

!`for f in $(git ls-tree --name-only HEAD docs/handover/); do case "$f" in *mountain*) printf '%s %s\n' "$(git log --diff-filter=A --format=%ct -1 -- "$f")" "$f";; esac; done | sort -rn | head -1 | cut -d' ' -f2`

**Kein Standard-Pass.** Der gemessene Rundenzustand liegt im Stehenden Pass:
`sread state/zustand/standing-pass.md` — einmal lesen, zitieren, nie kopieren
(eine kopierte Pass-Zahl ist ein Gate-Fixture `pass-copy`). Gemessen wird nur,
was der eigene Trigger für fällig erklärt.

**Phase 1 — Plan (nur lesend).** Der Plan-Agent misst, schreibt nicht — das ist die Grenze.
Lies die Übergabe **als Spur, nicht als Gesetz**, und miss jede offen/geschlossen-Aussage am
Baum und Register, **bevor** du die Tafel legst: `open_points_check <übergabe>` (Pfad- **und**
Register-Zitat-Abgleich; `STALE-CITATION` = gefeuerter Trigger), `register_lookup --addressed
<line>` (die an die eigene Linie gerichteten Nachrichten zuerst falten), `register_lookup --open`,
Secrets/Ledger/`state/zustand/*` gegen die genannten Zeilen; `session_burn` (der Burn der Runde —
flash-first, pro/max nur mit gemessener flash-Fehllage). Nenne dann alle offenen Punkte der
eigenen Linie als Tafel (nur `eigen`) und schlage vor, jeden parallel abarbeitbaren zu dispatchen —
keine Rangfolge, kein „härtester Punkt". Kein edit/write/commit. **Halte dann an.**

**Phase 2 — Ausführung.** Nach `/consent` (oder `/mountain_go`) → `line`-Agent. Arbeite die
eigenen Punkte bis zur Kante; Operator-Akte → Future-Operator-Queue, Dritt-Waits →
`state/zustand/wartend.φ` (Trigger + Aufnehmer). Werkzeuge statt Rohbefehle
(`docs/concepts/tools-map.md`); Header-sha256 via `omega_sh sha <datei>`; vor dem Commit
`git_safety --close [<eigene Pfade>]`; Delegiere flash-first (grind-flash/general/vision); ein pro/max-Dispatch trägt die gemessene
falsche/unvollständige flash-Antwort als Handover-Zeile — sonst nicht. `/commit` schließt.
Mountain trägt die Register-Verdikte: alleiniger Schreiber der Verdikt-Zeilen (`phi/sources.φ`/`declined`/`dead`); Register-Ordnung (`register_sort`); Datenkontrakt 26 × f64; kein Verdikt ohne Messung.

**Linien-Preset (`archive_search`).** Das Tool ist öffentlich und linien-blind; das Preset ist
**privat** (nur diese Linie) und liegt in `state/mountain/archive-search-preset.txt` — hier
eingelesen:

!`cat state/mountain/archive-search-preset.txt`

Nur eigene Arbeit: bei geteilten Dateien nur die eigenen Hunks; pfad-begrenzt committen
(`git commit <eigene Pfade> -m "…"`, nie ein nacktes `git commit`); fremde uncommittete
Arbeit nie überschreiben; gepusht wird, sobald der eigene Commit steht und `origin/main`
Vorfahr von HEAD ist.
