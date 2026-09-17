<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-17
  sha256: 63f0c2e97cec087dae180c6ef7ea825bba9a1f244f76b4686389cf42366b7528
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

An alle Linien: die Planungs-Klausel „einen schweren und fünf leichte" ist gestrichen (Operator-Wort, 2026-09-16) — es gilt wieder: so viele offene Punkte wie möglich pro Session; `AGENTS.md` + `_template.md` korrigiert. (Schritt: die eigene Preamble beim nächsten Handover angleichen.)

An bau: Doku-Drift aus `survey-2026-09-17-verlorene-diskussionen.md` (c) — `docs/concepts/kybernaut-native-methodology.md:23` erzählt biotic im Präsens (Baum: 9. Kraft = electric, `src/archivar/force.rs:243`); `docs/concepts/remove-bias.md` (Header ohne date/status; `:951,960` referenziert `warm_cache` — heute `cache_fresh_at`/`cache_path_for`); `docs/specs/kernel-curation-ci-automation-plan.md:8,48` nennt „v6 protocol" (heute v9). (Schritt: je Zeile als Legacy-Ära kennzeichnen, Plan archivieren oder offene Punkte extrahieren.)

An bau: smail Sent-Log — CI-Lauf `35195888583` (`service-build`) steht seit `2026-09-17T07:42:35Z` `queued` (kein Runner hat den Job aufgenommen; gemessen `gh run view --json`). (Schritt: `gh run view 35195888583` → bei `success` `gh run download` → `smail` nach `~/.local/bin/` + `target/release/`.)

An bau: Werkzeug-Reibung, gemessen aus `opencode.db` (38 Sessions, 653 Tool-Calls): (1) Header-sha256 manuell 16–17× (`tail -n +9 … | sha256sum`, fragil — nimmt 8 Header-Zeilen an) → `omega_sh sha <file>`; (2) Commit-Abschluss 13× in vier Git-Aufrufen (`git status --short` 18×, `git show --stat HEAD` 4×, `git log origin/main..HEAD --name-only` 4×, `git rev-parse HEAD origin/main` 5×) → `git_safety --close`; (3) `cargo check | tail -n 2` 7× → `check`-Zusammenfassung; (4) `echo`/`printf` in die restriktiven Profile `explore`/`general`/`research-max`/`council` (`plan` bereits freigegeben); `ci_manage list|view` ins Plan-Profil offen; Drift: 1× `ruby`. (Schritt: die Werkzeuge/Profil-Freigaben bauen; danach die Linien-Prompts um `omega_sh sha`/`git_safety --close` ergänzen.)

