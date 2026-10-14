<!--
  title: Postfach — privat ↔ öffentlich (öffentliche Ankündigungen)
  session: Future-Folge 256 (Postfach-Brücke)
  class: handover
  date: 2026-10-14
  sha256: 10d770fc0d9d8f58c4de4e6408990144d61db45aa08cb153757d40fe64cb2570
  status: live
-->
# Postfach — privat ↔ öffentlich (2026-10-14)

Die Brücke zwischen privatem Register und öffentlichem Baum. Die **private Linie
(Future)** trägt hier **nur die Namen ihrer öffentlichen Artefakte** ein — keine
Register-, Operator- oder PII-Inhalte. Der Commit-Gate (`commit_check`,
`own-handover`) liest diese Postfach-Folge als **öffentlichen Träger**: ein neu
hinzugefügtes `docs/*`-Dok ist gültig angekündigt, wenn es hier (oder in einer
öffentlichen Linien-Übergabe) genannt ist. So bleibt Futures Register privat und
der öffentliche Baum erfährt trotzdem, welches Dok die nächste Linie findet.

Grund: Futures Übergabe ist vollständig privat (`state/future/handover/`,
gitignored) — `docs/handover/` trägt keine `future`-Folge. Ohne diese Brücke
bliebe der `own-handover`-Gate für jede öffentliche Future-Arbeit zu (gemessen
2026-10-14 am AGENTS-Schnitt).

## Öffentliche Ankündigungen

- `docs/concepts/kybernetische-ethik.md` — Volltext der Kybernetischen Ethik (aus `AGENTS.md` ausgelagert; die fünf Axiome + der 0-Kanon-Kern bleiben in der AGENTS).
- `docs/concepts/code-rules.md` — Code-Rules-Volltext.
- `docs/concepts/house-rules.md` — Haus-Regeln: Register/Handover/Commit, System-Directive, Session-Hygiene.
- `docs/concepts/source-curation.md` — Quellen-Kuration (Der eine Pfad).
- `docs/concepts/agent-profiles.md` — Agenten-Profile (Roster `line`/`taucher`/`rat`/`future`, ein Anbieter: DeepSeek).
- `docs/concepts/tools-map.md` — Werkzeug-/Recherche-Abschnitte angehängt (Header-sha neu).

## Burn: open 0.0 · close 0.0 — Grund: Brücken-Dok, kein Session-Register
