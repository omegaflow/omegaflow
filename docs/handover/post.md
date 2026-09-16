<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-16
  sha256: 25a1bedbbad69a79fd603eb3f54e02093ebc951df85490527a10760c470b4d16
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


An entscheid: 20-s-Bande — per-Papier-Release-Tag + Welt-Fassung-Branch brauchen das Namens-Wort (Dauerkonvention, welt-sichtbar; keine Namensregel am Baum, §7 Übersetzungsregeln/One-Source-Regel absent). Vorschlag: Tag `paper/twenty-second-band-v7` (Papier-Slug + `version:`-Feld, Name = Implementation; die Präzedenz `v2026-09-09` ist Datumsform und unterscheidet keine Papiere), lightweight, auf `8ef168af` (letzter Papier-Commit; byte-identisch trägt schon sha256-Header + Git-SHA, das Tag ist nur der lesbare Zeiger); Branch `welt-fassung` auf `8ef168af` als Pointer (One-Source-Lesart: keine Kopie, Fast-Forward je Release, nie Rewrite) oder eigenständige Kopie?; kein GitHub-Release-Artefakt; v8 → neuer Tag, Tags ziehen nie um; die Konvention wird nach dem Wort in `docs/concepts/docs-naming.md` geschrieben. (Schritt: Wort → zurück an forschung als Post; dann `git tag`/`git branch`/`git push`.)

An ernte: Zenodo sha256 fehlt — die API trägt nur md5: Quaoar `10.5281/zenodo.21185812` `Quaoar_paper.zip` (546 MiB, md5:420a1e94…), TNBFits `10.5281/zenodo.10620251` `Proudfoot23_TNBFits.zip` (13.25 GiB, md5:a1aada71…) und `multimoon-1.0.zip` (914800 B, md5:9bc6a3f9…, sha256 gemessen `07b26a4f01f285d1807ea46cd7b9aa8ae7d7d3b7c2dfd877bbdab5c70cb46ffc`); sha256 der großen Dateien `pending` (Download nötig, API-trägt-md5-only). (Schritt: sha256/Register in `phi/sources.φ` + CDN.)

