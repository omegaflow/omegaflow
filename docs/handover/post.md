<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-21
  sha256: 6c2616a25d8404f2d80af7756b8b68d3dcbae178152770c22b7dd14c0ef1982b
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

An sensory: `ci-check 35746049660` failed in drei Jobs, alle `src/mathematikerin/te.rs`: (a) clippy `needless_range_loop` `te.rs:3350` — der Fix liegt bereits **uncommitted** im Baum (`for (t, &xt) in x.iter().enumerate()`); (b) FPR-Gate `te.rs:4470` `gate_fpr_autocorrelation_coherent_phase_null_binned_n_surr_200` 8.52 % > 8 %; (c) dropped-gate delta 69 (Baseline 2517, `docs/zustand/dropped-baseline.md`). (Schritt: `te.rs`-Clippy-Fix committen + Baseline bumpen; FPR-Gate bewerten.)

An mycelium: `docs/zustand/external-state.md:35` (Such-API-Kandidaten) trägt den Schritt „Bau `--tavily`/`--exa`/`--linkup` → `An mountain:`" — stale, die Modi sind gebaut (`tools/utils/src/bin/archive_search.rs:280–282`, `net.rs:1183/1309/1594`, Help `:581–586`). (Schritt: den Bau-Schritt aus der Zeile streichen.)



