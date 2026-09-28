---
description: Mycelium-Linie — erntet Quellen und Daten, Compiler, CDN-Manifestation, Register. Planungsmodus (read-only): Auswahl vorschlagen, dann Ausführung via line-Agent.
agent: plan
---

Starte die Mycelium-Linie im **Planungsmodus** (Agent `plan`, read-only). Ist ein Name genannt, nimm diesen; sonst die neueste offene Mycelium-Übergabe.

Name (leer = neueste): $ARGUMENTS

Neueste offene Mycelium-Übergabe:

!`for f in $(git ls-tree --name-only HEAD docs/handover/); do case "$f" in *mycelium*) printf '%s %s\n' "$(git log --diff-filter=A --format=%ct -1 -- "$f")" "$f";; esac; done | sort -rn | head -1 | cut -d' ' -f2`

**Kein Standard-Pass — der Stehende Pass gilt, in fester Reihenfolge.** Der gemessene
Rundenzustand liegt in `state/zustand/standing-pass.md` (gitignored, lokal). Zu Beginn liest
die Meta-Linie den **vorigen** Pass (`sread state/zustand/standing-pass.md`) und zitiert ihn —
sie kopiert keine Pass-Zahl in ihre Übergabe (eine kopierte Pass-Zahl ist ein Gate-Fixture
`pass-copy`). Gemessen wird nur, was der eigene Trigger für fällig erklärt. Die Meta-Linie ist
der Ring, der **als erste schließt**: sie arbeitet ihre eigenen Punkte, committet und pusht
(`/commit`), und **erst danach — mit stehendem Push — schreibt sie den frischen Pass am neuen
HEAD** (CI-Tafel: rote Läufe `run-id | workflow | gemessener Grund | Träger-Linie | Braucht`,
Grund aus Log/API lesen, nie raten, unread benennen; Postfach-Stand; Ereignis-Stand; Orphan-Zensus;
Mehrfach-Träger; `git_safety --snapshot`; `.tools_ensure`-Sweep). Die Pass-Schreibung ist ihr
**Schlussakt, nicht ihr erster** — so zitiert jede Linie einen Pass, der beim Öffnen den eben
gemessenen Stand trägt (kein Henne-Ei: der Pass wird nie vor der eigenen Änderung geschrieben).
Danach meldet sie die Runde frei — die Standard-Meldung:

> **Runde offen — Stehender Pass steht.** HEAD `<sha>` (== `origin/main`); Pass
> `state/zustand/standing-pass.md` (gemessen `<zeit>`); rote Läufe: `<n>` (Träger: …).
> Startet die Linien: `/mountain` `/river` `/sensory` `/future`.

**Erst nach dieser Meldung öffnen die anderen Linien.**

**Phase 1 — Plan (nur das).** Lies die Übergabe + `open_points_check <übergabe>` (billiger
Baum-Abgleich). Nenne alle offenen Punkte der eigenen Linie als Tafel (nur `eigen`) und schlage
vor, jeden parallel abarbeitbaren zu dispatchen — keine Rangfolge, kein „härtester Punkt".
Kein edit/write/commit, **keine Messung, keine Exploration über das Genannte hinaus** — der
Plan-Agent kann nicht schreiben, das ist die Grenze. **Halte dann an.**

**Phase 2 — Ausführung.** Nach `/consent` (oder `/mycelium_go`) → `line`-Agent. Zu Beginn
zitiert er den Stehenden Pass (`sread state/zustand/standing-pass.md`) — kein eigener
Standard-Pass. Arbeite die eigenen Punkte bis zur Kante; Operator-Akte → Future-Operator-Queue,
Dritt-Waits → `state/zustand/wartend.φ` (Trigger + Aufnehmer). Werkzeuge statt Rohbefehle
(`docs/concepts/tools-map.md`); Header-sha256 via `omega_sh sha <datei>`; vor dem Commit
`git_safety --close [<eigene Pfade>]`; Delegiere flash-first (grind-flash/general/vision);
pro nur bei gemessen falschem flash-Ergebnis oder benanntem Hart-Atom. `/commit` schließt.
Mycelium trägt Quellen: Arbeit über `docs/SOURCE_PORT.md`; eine neue/geänderte Ernte erst schließen, wenn sie in `phi/sources.φ` für die CDN-Manifestation registriert ist.
Der Stehende Pass wird am Sessionende geschrieben — **nach** dem Push (Schlussakt der Meta-Linie), dann die Runde-Meldung (siehe oben). Nie mitten in der Arbeit: ein Pass vor der eigenen Änderung wäre beim Öffnen der anderen Linien schon stale.

**Sofort-Prinzip.** Dispatcht wird **sofort** im nennenden Atom — kein besprechbarer Punkt
wandert als „nächster Dispatch"/„nächste Session" weiter; besprochene Entscheidungen (ein
Wort, eine Architektur, ein Verdikt) gehen im Moment ihrer Entstehung als Zeile ins Handover.
`commit_check` (status-proof) blockt den unbelegten Status-Tag, `register_lookup
--fired`/`--stale` messen Feuer und Stehen.

Nur eigene Arbeit: bei geteilten Dateien nur die eigenen Hunks; pfad-begrenzt committen
(`git commit <eigene Pfade> -m "…"`, nie ein nacktes `git commit`); fremde uncommittete
Arbeit nie überschreiben; gepusht wird, sobald der eigene Commit steht und `origin/main`
Vorfahr von HEAD ist.
