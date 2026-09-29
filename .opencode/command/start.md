---
description: Startet die neueste offene Übergabe im Planungsmodus — oder die benannte; Ausführung via line-Agent.
agent: plan
---

Starte die Übergabe im **Planungsmodus** (Agent `plan`, read-only). Ist ein Name genannt, nimm diesen; sonst die neueste offene.

Name (leer = neueste): $ARGUMENTS

Neueste offene Übergabe:

!`for f in $(git ls-tree --name-only HEAD docs/handover/); do case "$f" in handover-*) printf '%s %s\n' "$(git log --diff-filter=A --format=%ct -1 -- "$f")" "$f";; esac; done | sort -rn | head -1 | cut -d' ' -f2`

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

**Phase 2 — Ausführung.** Nach der Auswahl `/consent` (oder `/start_go`) — wechselt auf den auto-bestätigten `line`-Agenten. Zu Beginn zitiert er den Stehenden Pass (`sread state/zustand/standing-pass.md`) — kein eigener Standard-Pass — und liest die Form-Karte (`sread docs/concepts/tool-forms.md`). Werkzeuge statt Rohbefehle (Karte `docs/concepts/tools-map.md`). Den Übergabe-Header-sha256 setzt `omega_sh sha <datei>`; vor dem Commit prüft `git_safety --close [<eigene Pfade>]` den Abschluss in einem Aufruf. Delegiere flash-first (`grind-flash`/`general`/`vision`); ein pro/max-Dispatch trägt die gemessene falsche/unvollständige flash-Antwort als Handover-Zeile — sonst nicht. Der `line`-Agent führt den bestätigten Plan aus; `/commit` schließt.

**Sofort-Prinzip.** Dispatcht wird **sofort** im nennenden Atom — kein besprechbarer Punkt wandert als „nächster Dispatch"/„nächste Session" weiter; besprochene Entscheidungen (ein Wort, eine Architektur, ein Verdikt) gehen im Moment ihrer Entstehung als Zeile ins Handover, nie am Sessionende gesammelt. Der Commit-Gate `commit_check` (status-proof) blockt den unbelegten Status-Tag, `register_lookup --fired`/`--stale` messen Feuer und Stehen.

Nur eigene Arbeit: bei geteilten Dateien nur die eigenen Hunks; committet wird pfad-begrenzt (`git commit <eigene Pfade> -m "…"`, nie ein nacktes `git commit`), nur der eigene Teil; fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward) — ein Push sendet nur Commits, der Arbeitsbaum darf schmutzig sein.
