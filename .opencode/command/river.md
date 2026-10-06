---
description: River-Linie — die lebendige Membran (ω()-Loop, WebGPU-Feld, Präsenz, Echo, Browser-Brücke). Ein Pass — Stand lesen, Bekanntes bis zur Kante arbeiten, nur Ungeklärtes vorlegen.
agent: line
---

Erste Handlung: `sread docs/concepts/tool-forms.md` — die Form-Karte (verboten → kanonisch),
damit die erlaubte Form am Punkt der Handlung steht.

Starte die River-Linie **in einem Pass** — kein Planungstheater, keine Tafel, kein Consent-Stopp
für Bekanntes. Ist ein Name genannt, nimm diesen; sonst die neueste offene River-Übergabe.

Name (leer = neueste): $ARGUMENTS

Neueste offene River-Übergabe:

!`for f in $(git ls-tree --name-only HEAD docs/handover/); do case "$f" in *river*) printf '%s %s\n' "$(git log --diff-filter=A --format=%ct -1 -- "$f")" "$f";; esac; done | sort -rn | head -1 | cut -d' ' -f2`

**Die Übergabe IST der Stand.** Lies sie und den Stehenden Pass
(`sread state/zustand/standing-pass.md` — zitieren, nie kopieren; eine kopierte Pass-Zahl ist ein
Gate-Fixture `pass-copy`). Ein bereits geworteter/geklärter Punkt wird **nie** erneut vorgelegt.

**1 · Still messen, nur Fälliges.** Nur was der eigene Trigger für fällig erklärt:
`register_lookup --fired <line>` / `--stale <line> --persist 3`, `open_points_check <übergabe>`
(`STALE-CITATION` = gefeuerter Trigger), die adressierten Blöcke (`register_lookup --addressed
<line>`, zuerst falten). Was unverändert in der Übergabe steht, wird **nicht** neu gemessen und
nicht neu ausgelegt. `session_burn` einmal (flash-first; pro/max nur mit gemessener flash-Fehllage).

**2 · Bekanntes direkt bis zur Kante arbeiten.** Jeder eigene, autonom abarbeitbare Punkt wird
**in diesem Atom** gearbeitet — dispatcht (flash-first: `grind-flash`/`general`/`vision`), gebaut,
gemessen. Kein „nächster Dispatch", kein „nächste Session", kein Vorlegen eines bereits geworteten
Punktes, keine Rangfolge, keine Liste. Werkzeuge statt Rohbefehle (`docs/concepts/tools-map.md`);
`archive_search`/`sgrep`/`sfetch`/`sread`. Kein Polling.

**3 · Nur Ungeklärtes vorlegen.** Braucht ein Punkt ein **neues** Operator-Wort, wird er einzeln in
einfacher Sprache vorgelegt (Lage · Frage · was bei Ja und bei Nein geschieht) — nie als Liste.
**LOCK nie vorlegen**. **Wartend nie vorlegen** (Wahrheit `state/zustand/wartend.φ`). Jede Entscheidung
geht sofort als Zeile ins Handover (Sofort-Prinzip).

**4 · Grenze bleibt.** Der Send bleibt die Operator-Hand (nie `smail --send`); jeder dritt-wirksame
Akt ist per-Akt-Operator-Wort. `/consent` ist der session-weite Consent (Delegation), `/commit` ist
das Commit-Wort — beide getrennt.

**5 · Übergabe zuletzt.** Erst nach der Arbeit: die Übergabe fortschreiben — Erledigtes **löschen**
(nie als „erledigt" markieren), Messungen/Ergebnisse eintragen, nur Offenes mit **Lage · Blockade ·
Braucht**; **LOCK nur im `LOCK`-Abschnitt**; Header-sha256 via `omega_sh sha <datei>`; die konsumierte
Übergabe nach `archiv/`.

**River besitzt die Membran-Pfade** (`main_flow`, `omega.rs`-Feld, Window/Gaze). Architektur-/Ethik-
Entscheidungen gehen durch die **Linse der fünf Stimmen** (Rat), nie in Pro-Solo; kein Fenster-Edit
ohne Operator-Wort. Die starken UI-Modelle sind ein zweiter Kanal nach dem Rat, nie der Rat selbst.

**Linien-Preset (`archive_search`).** Das Tool ist öffentlich und linien-blind; das Preset ist
**privat** (nur diese Linie) und liegt in `state/river/archive-search-preset.txt` — hier
eingelesen:

!`cat state/river/archive-search-preset.txt`

Nur eigene Arbeit: bei geteilten Dateien nur die eigenen Hunks; pfad-begrenzt committen
(`git commit <eigene Pfade> -m "…"`, nie ein nacktes `git commit`); fremde uncommittete
Arbeit nie überschreiben; `git_safety --close [<eigene Pfade>]` vor dem Commit; gepusht wird,
sobald der eigene Commit steht und `origin/main` Vorfahr von HEAD ist.
