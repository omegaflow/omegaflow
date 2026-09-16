<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-16
  sha256: f2acaeb7768a3c8a37301fcd323101f5f840fef28638939f7151765da95975a4
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



An ernte: Zenodo sha256 fehlt — die API trägt nur md5: Quaoar `10.5281/zenodo.21185812` `Quaoar_paper.zip` (546 MiB, md5:420a1e94…), TNBFits `10.5281/zenodo.10620251` `Proudfoot23_TNBFits.zip` (13.25 GiB, md5:a1aada71…) und `multimoon-1.0.zip` (914800 B, md5:9bc6a3f9…, sha256 gemessen `07b26a4f01f285d1807ea46cd7b9aa8ae7d7d3b7c2dfd877bbdab5c70cb46ffc`); sha256 der großen Dateien `pending` (Download nötig, API-trägt-md5-only). (Schritt: sha256/Register in `phi/sources.φ` + CDN.)

