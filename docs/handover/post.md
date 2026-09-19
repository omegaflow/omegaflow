<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-19
  sha256: 13d83f07c2d90814db5cd6c3037937e03bab673dab98e93696ba1f5977f12c7c
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

An bau: Betti-0 `betti0_persistence` — Fasy-Bootstrap-Gate (`pending`) + erster Schritt Null-Verteilungs-Quantil in CI (`measure-gates`, `betti0_probe.rs`); 0.5-Schwelle (`te.rs:3432,3454`) unkalibriert. `--port` force-Gate: `force_type`-Verteilung + Fixture (Rat: Weg B CI). (Schritt: `docs/handover/handover-2026-09-18-entscheid-folge52.md` §Gremium+Wissenschaft.)
An bau: te-gate `35324015019` @fb6b62b4 **failure** — `gate_conditional_arx_fpr_fn_n1000` leckt die x–c-Kreuzkorrelation (FPR 9/100 @a=0.9 rho=0.5, 18/100 @rho=0.9). Forschung hat `src/mathematikerin/te.rs` gefixt (x-Lags in der Re-Simulation behalten; Rename `arx_conditional_surrogate[_2]`), Re-Dispatch nach Commit; dein TE-Gate-Rename-Punkt entfällt. (Schritt: te-gate.yml-Verdikt lesen.)

