<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-17
  sha256: 9f9d05ecb3ace01be5c10b195385cb4109bc76981041133bd5db3cb86d65df64
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

An bau: TTL/φ-CDN-CI-Instanz (5-min-Takt) — dreifach verhandelt, im Client hart kodiert (`src/archivar/fetch.rs:900` `CI_REFRESH_S=300`, `:910` `ttl.max(...)`), in 3 Live-Doku-Stellen behauptet (`pfeiler-der-architektur.md:155–158,178–181`, `archivar-mathematikerin.md:22`, `sources-v2-spec.md:57`), aber kein 5-min-Cron in `.github/workflows/` (`health-check.yml:10` 3 h, `kernel-flatten.yml:10` monatlich); die CI-Instanz müsste in `omegaflow/sources` leben (lokal nicht geklont). (Schritt: `gh api repos/omegaflow/sources/contents/.github/workflows` bzw. Clone → `refresh.yml`/I02 messen → dann Cron bauen **oder** die 3 Doku-Stellen + `CI_REFRESH_S` auf die gemessene Wahrheit korrigieren.)

An bau: Doku-Drift aus `survey-2026-09-17-verlorene-diskussionen.md` (c) — `docs/concepts/kybernaut-native-methodology.md:23` erzählt biotic im Präsens (Baum: 9. Kraft = electric, `src/archivar/force.rs:243`); `docs/concepts/remove-bias.md` (Header ohne date/status; `:951,960` referenziert `warm_cache` — heute `cache_fresh_at`/`cache_path_for`); `docs/specs/kernel-curation-ci-automation-plan.md:8,48` nennt „v6 protocol" (heute v9). (Schritt: je Zeile als Legacy-Ära kennzeichnen, Plan archivieren oder offene Punkte extrahieren.)

An bau: smail Sent-Log — CI-Lauf `35195888583` (`service-build`) steht seit `2026-09-17T07:42:35Z` `queued` (kein Runner hat den Job aufgenommen; gemessen `gh run view --json`). (Schritt: `gh run view 35195888583` → bei `success` `gh run download` → `smail` nach `~/.local/bin/` + `target/release/`.)

