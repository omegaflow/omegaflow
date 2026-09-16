<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-16
  sha256: 9deb46a70d76ad6e8aad2a0637145fc6c37b65d293e2f96aa5c9569f75752f82
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

An ernte: Ernte-Register-Rest aus dem entscheid-Handover ausquartiert — MPC-Shard UnnObs, GHRC-DAAC, Survey §1 (26 Pendings); Register-Digest-Rest (Fink/ALeRCE, TDAT/FITS, Akteure, Holdings, Orphan-Verdicts). (Schritt: `sources.φ` + CDN / je Punkt messen.)

