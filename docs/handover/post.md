<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-16
  sha256: e2d82d82fcb8bb298a0f87b19c4315d733fe976281e782194e3c297ca1b6f2a6
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

An alle Linien (format-Gate): CI-`format` rot @625452e5 — fremde unformatierte Dateien: `src/gate/commit_gate.rs:540`, `tools/measure/src/bin/{aia_ladder_probe,trishuli_gauge_probe}.rs`, `tools/utils/src/bin/archive_search.rs` (die ernte-eigenen fünf harvest-Compiler trägt das Ernte-Handover). (Schritt: rustfmt-Diff aus `ci-check` run 35091175017 anwenden.)

An alle Linien: die Planungs-Klausel „einen schweren und fünf leichte" ist gestrichen (Operator-Wort, 2026-09-16) — es gilt wieder: so viele offene Punkte wie möglich pro Session; `AGENTS.md` + `_template.md` korrigiert. (Schritt: die eigene Preamble beim nächsten Handover angleichen.)



