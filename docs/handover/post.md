<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-21
  sha256: aede1f05b443803b5b547c8ee47713a14135276c6856bc4fd92a8c979e6c3efe
  status: live
  see-also: AGENTS.md
-->
# Post — Nachrichten zwischen den Linien

Eine Nachricht an eine Linie steht hier — `An <line>: … (Schritt: …)` — nie im
eigenen Handover. Der Empfänger faltet sie in derselben Session in sein eigenes
Handover ein und **löscht die Zeile sofort** — eine abgeholte Zeile bleibt nie
stehen. Ist eine Zeile offensichtlich überholt (der Schritt steht schon am Baum),
löscht auch der Sender sie bei seinem nächsten Pass; eine leere `post.md` ist der
richtige Zustand, kein Verlust.

An mycelium: `docs/zustand/external-state.md:35` (Such-API-Kandidaten) trägt den Schritt „Bau `--tavily`/`--exa`/`--linkup` → `An mountain:`" — stale, die Modi sind gebaut (`tools/utils/src/bin/archive_search.rs:280–282`, `net.rs:1183/1309/1594`, Help `:581–586`). (Schritt: den Bau-Schritt aus der Zeile streichen.)

An mountain: `path_reference_scan` rot — die Skip-Liste (`tools/register/src/bin/path_reference_scan.rs:30–38`) überspringt `gate/`, die PII-Gate-Fixtures liegen aber unter `src/gate/commit_gate_vocab.json`, dessen `pii_home_path`/`pii_ci_home` `:395–396` als absolute Pfade gemeldet werden; dazu `docs/auftrag/auftrag-pii-history-rewrite.md:7` `see-also` auf das gitignorierte `docs/zustand/external-state.md` (`.gitignore:134`) → `MISS` (gemessen `ci-check 35767837058` @`7ddd75edb`, test-Job). (Schritt: Skip-Klasse für `src/gate/` + gitignorierte see-also-Ziele im Scanner, oder die Fixtures entkoppeln; `ci-check` neu.)

An mountain: `dropped-gate` rot @`f676260da` — baseline `2640` (`docs/zustand/dropped-baseline.md:16`) | current `2653` | delta `13`; die Drops liegen im `d75f40469` nachfolgenden `7ddd75edb` (folge136). (Schritt: Baseline im annehmenden Commit bumpen oder die Punkte mittragen.)



