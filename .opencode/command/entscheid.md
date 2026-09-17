---
description: Entscheid-Linie — Planungsmodus (read-only): Auswahl vorschlagen, dann Ausführung via line-Agent.
agent: plan
---

Starte die Entscheid-Linie im **Planungsmodus** (Agent `plan`, read-only). Ist ein Name genannt, nimm diesen; sonst die neueste offene Entscheid-Übergabe.

Name (leer = neueste): $ARGUMENTS

Neueste offene Entscheid-Übergabe:

!`for f in $(git ls-tree --name-only HEAD docs/handover/ | grep -E 'handover-.*entscheid'); do printf '%s %s\n' "$(git log --diff-filter=A --format=%ct -1 -- "$f")" "$f"; done | sort -rn | head -1 | cut -d' ' -f2`

Stehender Pass — beim Start gemessen (kein Auswahlpunkt):

!`printf 'HEAD: %s\n' "$(git rev-parse --short HEAD)"; printf '## CI-Status (Watchdog-Snapshot)\n'; tail -n 30 /tmp/opencode/ci_status.md 2>/dev/null || printf 'snapshot absent — line-Agent: bin/ci_manage list\n'; printf '\n## Postfach — letzte Ledger-Zeile\n'; tail -n 1 state/mail/mail_ledger.φ 2>/dev/null | cut -c1-400`

**Phase 1 — Plan (nur das).** Lies die Übergabe. Planungs-Pass: `register_lookup --live` (offene Punkte über alle lebenden Dokumente) + `git_safety --snapshot` (Arbeitsbaum-Sicherheitsnetz). Der stehende Pass steht oben bereits gemessen — **kein** Auswahlpunkt; die Session schreibt Ledger + Übergabe fort. Bei Lücke: Postfach via `smail` + `state/mail/mail_ledger.φ`, CI-Status via `ci_manage list`/`view`. Nenne die offenen Punkte als kurze nummerierte **Auswahl** und schlage das Atom vor (welche Punkte, welche Delegation, welche Reihenfolge). Kein edit/write/commit, keine Messung, keine Exploration über das Genannte hinaus — der Plan-Agent kann nicht schreiben, das ist die Grenze. **Halte dann an.**

**Phase 2 — Ausführung.** Nach der Auswahl `/consent` (oder `/entscheid_go`) — wechselt auf den auto-bestätigten `line`-Agenten. Zu Beginn führt er den stehenden Pass aus: die fälligen Zustand-Einträge messen — Postfach (`smail` + `state/mail/mail_ledger.φ`), CI-Status (Watchdog-Snapshot `/tmp/opencode/ci_status.md`; bei Lücke/Detail `ci_manage list`/`ci_manage view`, **nie** `gh run list`/`gh run view`) — und Ledger + Übergabe fortschreiben; Werkzeuge statt Rohbefehle (Karte `docs/concepts/tools-map.md`). Der `line`-Agent führt den bestätigten Plan aus; `/commit` schließt. Diese Linie trägt autonom nur Entscheidungen, Korrespondenz und Consent; Quellen-/Bau-Punkte reisen als Nachricht an ihre Linie (nie in ein fremdes Handover). Delegiere: Rat für Architektur, research-max für harte Recherche, vision für Figuren/OCR, grind-* für Bau.

Nur eigene Arbeit: bei geteilten Dateien nur die eigenen Hunks; committet wird pfad-begrenzt (`git commit <eigene Pfade> -m "…"`, nie ein nacktes `git commit`), nur der eigene Teil; fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward) — ein Push sendet nur Commits, der Arbeitsbaum darf schmutzig sein.
