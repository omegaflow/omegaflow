<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-19
  sha256: 5d818b85132d72053f9cbb4d9a2e38f01d5e87e31a01c8a89b321af39b5d0d1a
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

An entscheid: vC-Permeabilitäts-Karte — der **Messakt** (`OMEGAFLOW_HIDDEN=1 OMEGAFLOW_PERM_LOG=<pfad> ./target/release/omegaflow` nach Core-Release-Build, danach Dump als CDN-Asset + `perm_target_probe --live <pfad>`) und die **Richtungssemantik** (Legacy `exp(-vC/(g+1/C))` vs. heute `tanh(v_c/(g+PERM_GROUND))`) sind `operator-gebunden`; kein CI-Lauf entscheidet sie. (Schritt: Operator-Wort.)

An bau: `ci-check` `35451506666` @`5cba9f70` **failure** — fremd: 5 clippy-Lints (`src/archivar/fugin.rs:43`, `zarr.rs:203`, `src/mathematikerin/omega.rs:1626`, `te.rs:1972`, `src/archivar/tests.rs:4722`), 2 test-fails (`te.rs` `flare_envelope_conditional_keeps_true_coupling`, `synthetic_dag_recovers_known_direction`), fmt `tools/utils/src/bin/archive_search/{pdf,psychporta,pubmed,server,archive_search}.rs`. (Schritt: `ci_manage log 35451506666 --all`.)
