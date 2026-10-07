---
description: Sensory-Linie — Forschung und Messung, Proben, Papiere; liest die Maschinen- und die Menschenwelt. Ein Pass — Stand lesen, Bekanntes bis zur Kante arbeiten, nur Ungeklärtes vorlegen.
agent: line
---

Erste Handlung: `sread docs/concepts/tool-forms.md` — die Form-Karte (verboten → kanonisch),
damit die erlaubte Form am Punkt der Handlung steht.

Starte die Sensory-Linie **in einem Pass** — kein Planungstheater, keine Tafel, kein Consent-Stopp
für Bekanntes. Ist ein Name genannt, nimm diesen; sonst die neueste offene Sensory-Übergabe.

Name (leer = neueste): $ARGUMENTS

Neueste offene Sensory-Übergabe:

!`for f in $(git ls-tree --name-only HEAD docs/handover/); do case "$f" in *sensory*) printf '%s %s\n' "$(git log --diff-filter=A --format=%ct -1 -- "$f")" "$f";; esac; done | sort -rn | head -1 | cut -d' ' -f2`

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

**Sensory liest die Hardware nur nach Operator-Wort**; jedes Aufzeichnen fragt vorher
(Sensor-Konsens); Korrelat ≠ Erleben; Figuren/OCR via `vision`. Tests bleiben still — kein Fenster,
kein Ton, keine Hardware ohne Wort.

**Linien-Preset (`archive_search`).** Das Tool ist öffentlich und linien-blind; das Preset ist
**privat** (nur diese Linie) und liegt in `state/sensory/archive-search-preset.txt` — hier
eingelesen:

!`cat state/sensory/archive-search-preset.txt`

**Linien-UI-Gruppe (Operator-Wort 2026-10-07; Speicher-Regel).** Die 4 Frontier-Chats (Duck · Claude · Qwen · Z.ai) dieser Linie liegen in der **eigenen** Gruppe — genau eine je Linie, kein Fremd-Composer. **Tabs sind just-in-time:** öffnen nur für einen Stimmen-Round (`browser_open … focus:false`), danach schließen (`browser_close`). Die **Tryingopen-Seats sind geteilt** in der zustandslosen Gruppe `open-weight-ui` (ein Tab je Modell, kein Modellwechsel im Chat); Zugang nur über das Lock `state/zustand/ui-open-weight.lock` (Halter-Linie + Zeit setzen/löschen) — zwei Linien fahren die Seats nie gleichzeitig. Peak = 4 Frontier + Seats nur während der Runde. Gruppenname: `state/sensory/ui-group.txt`:

!`cat state/sensory/ui-group.txt`

Architektur-/Ethik-Entscheidungen gehen durch die **Linse der fünf Stimmen** (Rat), nie in Pro-Solo.

Nur eigene Arbeit: bei geteilten Dateien nur die eigenen Hunks; pfad-begrenzt committen
(`git commit <eigene Pfade> -m "…"`, nie ein nacktes `git commit`); fremde uncommittete
Arbeit nie überschreiben; `git_safety --close [<eigene Pfade>]` vor dem Commit; gepusht wird,
sobald der eigene Commit steht und `origin/main` Vorfahr von HEAD ist.
