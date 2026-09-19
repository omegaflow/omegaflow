<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-19
  sha256: 2b03eabc29db09dc97ceab650f48078dbaaf0d01f861ae1fe4f9aaf42e237eb6
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

An bau: `ci-check` `35451506666` @`5cba9f70` **failure** — fremd: 5 clippy-Lints (`src/archivar/fugin.rs:43`, `zarr.rs:203`, `src/mathematikerin/omega.rs:1626`, `te.rs:1972`, `src/archivar/tests.rs:4722`), 2 test-fails (`te.rs` `flare_envelope_conditional_keeps_true_coupling`, `synthetic_dag_recovers_known_direction`), fmt `tools/utils/src/bin/archive_search/{pdf,psychporta,pubmed,server,archive_search}.rs`. (Schritt: `ci_manage log 35451506666 --all`.)

An bau: **Betti-0-Schwelle messen** — Operator-Wort 2026-09-19: „muss auf jeden Fall gemessen werden". Null-Verteilungs-Quantil über ≥100 Realisierungen gegen die unkalibrierte 0.5-Schwelle (`te.rs:3432,3454`), in `measure-gates`; `betti0_probe.rs` konsumiert die Funktion bereits. (Schritt: Quantil-Arm + FP/FN-Rate bauen, `gh workflow run measure-gates.yml`.)

An bau: **vC-Permeabilität — Messakt lokal, kein CDN** — Operator-Wort 2026-09-19: die Permeabilität darf **nicht** auf CDN. Messakt: `OMEGAFLOW_HIDDEN=1 OMEGAFLOW_PERM_LOG=<pfad> ./target/release/omegaflow` nach Core-Release-Build, danach `perm_target_probe --live <pfad>` (lokal). Herleitung (council + research-max): keine Inversion; `tanh(v_c/(g+ε))` (`omega.rs:15-17`) ist die Permeabilität, Legacy `exp(-vC/(g+1/C))` war eine Certainty; Argument dimensionslos (`v_c=|Δω_sum|`, `g=|ω_sum|`, `omega.rs:1620-1621`). Offen: (a) Fallback ohne Surrogat-Nullkontrolle, (b) Saturations-Skala v_c/g. (Schritt: Messakt lokal fahren, Verteilung messen.)

An bau: **Pipeline-Port force-Gate — erster Schritt ohne Wort** — `force_type`-Verteilung + Fixture über die 10 `phi/pipeline/queue/*.φ`-Korpora (gitignored, ~3.683 Blöcke); A/B (lokaler Lauf vs. CI) wartet auf Operator-Wort. (Schritt: Verteilungs-Probe + Fixture.)
