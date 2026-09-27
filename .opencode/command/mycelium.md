---
description: Mycelium-Linie — erntet Quellen und Daten, Compiler, CDN-Manifestation, Register. Planungsmodus (read-only): Auswahl vorschlagen, dann Ausführung via line-Agent.
agent: plan
---

Starte die Mycelium-Linie im **Planungsmodus** (Agent `plan`, read-only). Ist ein Name genannt, nimm diesen; sonst die neueste offene Mycelium-Übergabe.

Name (leer = neueste): $ARGUMENTS

Neueste offene Mycelium-Übergabe:

!`for f in $(git ls-tree --name-only HEAD docs/handover/); do case "$f" in *mycelium*) printf '%s %s\n' "$(git log --diff-filter=A --format=%ct -1 -- "$f")" "$f";; esac; done | sort -rn | head -1 | cut -d' ' -f2`

**Meta-Pass (nur diese Linie, einmal pro Runde, vor den anderen Linien).** Miss einmal und
schreibe `state/zustand/standing-pass.md` (Form: `external-state.md`):
- HEAD · CI-Tafel — rote Läufe: `run-id | workflow | gemessener Grund | Träger-Linie | Braucht`;
  den Grund aus dem Log lesen (Browser/GH-API/`ci_manage log`), nie raten; unread = benannt.
- Postfach-Stand + ungetragene Operator-Akte (Faktenebene, nie Mail-Bodies) · Orphan-Zensus
  (`register_lookup --orphan-docs`) · Mehrfach-Träger-Scan (abgeleitet, nie gespeichert) ·
  `git_safety --snapshot` · ein `.tools_ensure`-Sweep.
Schließe den Pass, **bevor** die anderen Linien öffnen.

**Phase 1 — Plan (nur das).** Lies die Übergabe + `open_points_check <übergabe>` (billiger
Baum-Abgleich). Nenne alle offenen Punkte der eigenen Linie als Tafel (nur `eigen`) und schlage
vor, jeden parallel abarbeitbaren zu dispatchen — keine Rangfolge, kein „härtester Punkt".
Kein edit/write/commit. **Halte dann an.**

**Phase 2 — Ausführung.** Nach `/consent` (oder `/mycelium_go`) → `line`-Agent. Arbeite die
eigenen Punkte bis zur Kante; Operator-Akte → Future-Operator-Queue, Dritt-Waits →
`state/zustand/wartend.φ` (Trigger + Aufnehmer). Werkzeuge statt Rohbefehle
(`docs/concepts/tools-map.md`); Header-sha256 via `omega_sh sha <datei>`; vor dem Commit
`git_safety --close [<eigene Pfade>]`; Delegiere flash-first (grind-flash/general/vision);
pro nur bei gemessen falschem flash-Ergebnis oder benanntem Hart-Atom. `/commit` schließt.
Mycelium trägt Quellen: Arbeit über `docs/SOURCE_PORT.md`; eine neue/geänderte Ernte erst schließen, wenn sie in `phi/sources.φ` für die CDN-Manifestation registriert ist.

Nur eigene Arbeit: bei geteilten Dateien nur die eigenen Hunks; pfad-begrenzt committen
(`git commit <eigene Pfade> -m "…"`, nie ein nacktes `git commit`); fremde uncommittete
Arbeit nie überschreiben; gepusht wird, sobald der eigene Commit steht und `origin/main`
Vorfahr von HEAD ist.
