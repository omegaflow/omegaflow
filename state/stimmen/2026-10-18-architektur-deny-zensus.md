# Architektur-Commit-Zensus + Deny-Regeln (2026-10-18)

**Frage:** Welche Linien haben an der Architektur gebaut — und brauchen wir die `edit:deny`-Regeln noch?
**Stand:** core HEAD `a32c3cacf` · persönlich `b7a41ab` · Hook `1f73b482e`.

## 1 — Wer an der Architektur gebaut hat (30 Tage, core)

Architektur-Pfade laut R-2026-10-18-d/e: `tools/gate/`, `.opencode/`, `AGENTS.md`, `opencode.json`, `docs/concepts/`.

| Linie | Architektur-Commits | davon mit RAT-ID |
|---|---|---|
| river | 62 | 0 |
| mountain | 52 | 0 |
| mycelium | 50 | 0 |
| future | 25 | 1 |
| sensory | 18 | 0 |
| rat | 3 | 1 |
| merge | 1 | 0 |
| andere (gate/agents/config/docs/opencode/…) | 231 | 2 |
| (ohne Präfix) | 8 | 1 |
| **Summe** | **~450** | **5 (~1 %)** |

**Der dominante Pfad ist `docs/concepts/`: 216 Commits** — river 42 · mountain 34 · mycelium 26 ·
sensory 14 · future 7 · rat 1 · andere 91. Danach `AGENTS.md` 136 · `opencode.json` 101 ·
`.opencode` 79 · `tools/gate` 33.

**Nach dem Hook `1f73b482e`: 0 Architektur-Commits.** Der Hook ist im Betrieb **noch nie gefeuert** —
die Abnahme (1/0/0) ist der einzige Beleg, kein Produktionsfall.

## 2 — Die Deny-Regeln (gemessen)

| Regel | personal | core |
|---|---|---|
| `edit:deny` `.opencode/**` | ✓ | ✓ |
| `edit:deny` `AGENTS.md` | ✓ | ✓ |
| `edit:deny` `opencode.json` | ✓ | ✓ |
| `edit:deny` `tools/gate/**` | ✓ | ✓ |
| `edit:deny` `.secrets.local` | ✓ | ✓ (+ read/shell) |
| `edit:deny` `docs/concepts/**` | **fehlt** | **fehlt** |
| `edit:deny` `omegaflow-personal/**` (aus core) | — | ✓ |

**Dateimodi:** `AGENTS.md` = **444** · `opencode.json` = **664** (schreibbar; nur die Deny-Regel schützt).

**Wurden sie respektiert?** Seit dem Einbau (`620fb02`, 2026-10-17 19:48) berührten **4**
personal-Commits die gewallten Pfade:
- `acdfc95` (R-2026-10-18-d) — **mit** RAT-ID
- `af6a27f` (R-2026-10-18-c) — **mit** RAT-ID
- `ef977cb` config V2 — **ohne** RAT-ID
- `d0dbdee` Start-Prompt — **ohne** RAT-ID, Nachricht wörtlich: *„lift the edit-deny that blocked the fix"*

→ Die Deny-Regel ist **porös und wurde selbst aufgehoben**, als eine erlaubte Änderung sie blockierte.

## 3 — Der entscheidende Fund: der Hook fehlt im persönlichen Repo

```
core/.githooks/commit-msg      → vorhanden (Wrapper auf tools/gate/commit_msg.sh)
personal/.githooks/            → nur pre-commit, KEIN commit-msg
beide: core.hooksPath = .githooks
```

Im **persönlichen** Repo läuft also **keine** RAT-ID-Prüfung. Dort sind `.opencode/**` und
`opencode.json` **nur** durch die Deny-Regel geschützt. Genau die zwei Commits ohne RAT-ID
(`ef977cb`, `d0dbdee`) liegen dort.

## 4 — Was daraus folgt (Empfehlung, nicht Beschluss)

1. **Behalten — immer:** `.secrets.local`. Das ist keine Architektur, das sind Schlüssel.
2. **Behalten:** `tools/gate/**`. Es schützt die Prüfung selbst. Aber: die Deny-Regel ist per Shell
   umgehbar → zusätzlich die Hook-Datei auf **444** legen, sonst ist die Wand nur aus Papier.
3. **Persönliches Repo:** Die Deny-Regeln auf `.opencode/**`, `AGENTS.md`, `opencode.json` sind
   **noch nötig**, weil dort kein `commit-msg`-Hook installiert ist. Sobald er dort läuft, sind sie
   redundant.
4. **Core-Repo:** Die Deny-Regeln auf `.opencode/**`, `AGENTS.md`, `opencode.json` sind **redundant
   mit dem Hook** — sie blockieren auch die *erlaubte* RAT-ID-Änderung. Entweder entfernen oder
   ausdrücklich als „nur Operator" deklarieren.
5. **`docs/concepts/**`:** hat **keine** Deny-Regel und funktioniert (216 Commits, Hook-only).
   Das ist der Präzedenzfall: **Hook-only genügt für Architektur.**
6. **Reihenfolge:** Die Denies nicht entfernen, **bevor** der Hook den ersten echten RAT-ID-Commit
   im Betrieb abgefangen hat (bisher 0). Erst beweisen, dann abbauen.

## Belege / Befehle

```
git log --since='30 days ago' --format='%h|%ci|%s' -- tools/gate .opencode AGENTS.md opencode.json docs/concepts
git log 1f73b482e..HEAD --format='%h' -- tools/gate .opencode AGENTS.md opencode.json docs/concepts | wc -l   # 0
git -C ../omegaflow-personal log --oneline 620fb02..HEAD -- AGENTS.md opencode.json .opencode                 # 4
ls -la ../omegaflow-personal/.githooks/    # nur pre-commit
```

## Nachtrag — der persönliche Hook ist installiert (2026-10-18)

`personal/.githooks/commit-msg` zeigt auf `$OMEGAFLOW_CORE/tools/gate/commit_msg.sh`
(Default `$HOME/projects/omegaflow`) und ist **fail-closed** (fehlt der Core-Hook, exit 1).

Abnahme (gemessen):

| Fall | Exit |
|---|---|
| personal, nichts Architektur gestaged | **0** |
| `.opencode/x` gestaged, Nachricht ohne RAT-ID | **1** |
| derselbe Stage, Nachricht mit `R-2026-10-18-e` | **0** |

Commit `159b0d3` (personal, gepusht). Damit sind die personal Deny-Regeln auf `.opencode/**`,
`AGENTS.md`, `opencode.json` **redundant mit dem Hook** — sie blockieren nur noch die erlaubte
RAT-ID-Änderung. `tools/gate/**` (schützt die Prüfung) und `.secrets.local` (Schlüssel) bleiben.
