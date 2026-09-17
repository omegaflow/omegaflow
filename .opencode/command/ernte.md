---
description: Ernte-Linie — Planungsmodus (read-only): Auswahl vorschlagen, dann Ausführung via line-Agent.
agent: plan
---

Starte die Ernte-Linie im **Planungsmodus** (Agent `plan`, read-only). Ist ein Name genannt, nimm diesen; sonst die neueste offene Ernte-Übergabe.

Name (leer = neueste): $ARGUMENTS

Neueste offene Ernte-Übergabe:

!`for f in $(git ls-tree --name-only HEAD docs/handover/ | grep -E 'handover-.*ernte'); do printf '%s %s\n' "$(git log --diff-filter=A --format=%ct -1 -- "$f")" "$f"; done | sort -rn | head -1 | cut -d' ' -f2`

**Phase 1 — Plan (nur das).** Lies die Übergabe. Planungs-Pass: `register_lookup --live` (offene Punkte über alle lebenden Dokumente) + `git_safety --snapshot` (Arbeitsbaum-Sicherheitsnetz). Die fälligen Einträge aus `docs/zustand/external-state.md` sind **kein** Auswahlpunkt — sie laufen als stehender Pass zu Beginn der Ausführung (Postfach via `smail`, CI-Status aus dem Watchdog-Snapshot `/tmp/opencode/ci_status.md`, Detail via `ci_manage`). Nenne die offenen Punkte als kurze nummerierte **Auswahl** und schlage das Atom vor (welche Punkte, welche Delegation, welche Reihenfolge). Kein edit/write/commit, keine Messung, keine Exploration über das Genannte hinaus — der Plan-Agent kann nicht schreiben, das ist die Grenze. **Halte dann an.**

**Phase 2 — Ausführung.** Nach der Auswahl `/consent` (oder `/ernte_go`) — wechselt auf den auto-bestätigten `line`-Agenten. Zu Beginn führt er den stehenden Pass aus: die fälligen Zustand-Einträge messen — Postfach (`smail` + `state/mail/mail_ledger.φ`), CI-Status (Watchdog-Snapshot `/tmp/opencode/ci_status.md`; bei Lücke/Detail `ci_manage list`/`ci_manage view`, **nie** `gh run list`/`gh run view`) — und Ledger + Übergabe fortschreiben; Werkzeuge statt Rohbefehle (Karte `docs/concepts/tools-map.md`). Ernte trägt Quellen: Arbeit läuft über `docs/SOURCE_PORT.md`; die Notwendigkeit einer Quelle entscheidet das Register (`phi/sources.φ` / `blocked_sources.φ` / `dead_sources.φ`), nicht die Korrespondenz. Eine neue/geänderte Ernte wird erst geschlossen, wenn sie in `phi/sources.φ` für die CDN-Manifestation registriert ist. Delegiere: grind-flash für Harvest/Verify/Recheck, grind-pro für Force-Gate/Parser-Gap/Route, research-max für die härtesten Routen, vision für figure-only-Daten (Figur → Tabelle). `/commit` schließt.

Nur eigene Arbeit: bei geteilten Dateien nur die eigenen Hunks; committet wird pfad-begrenzt (`git commit <eigene Pfade> -m "…"`, nie ein nacktes `git commit`), nur der eigene Teil; fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward) — ein Push sendet nur Commits, der Arbeitsbaum darf schmutzig sein.
