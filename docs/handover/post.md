<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-18
  sha256: ab37e764362b414671ec3c0e3397ac4610e6b5f742d5d6b6eb7b661a14857be3
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
An forschung: vC-Inversion belegt — Legacy `exp(-vC/(g+1/C))` (`archive-root/…/minkowski-field-permeability.md:162`) vs. heute `tanh` (`omega.rs:1606`); Designfrage + v_c-Verteilung in CI. Silence-Map-Null: FDR/BH (DOI 10.1111/j.2517-6161.1995.tb02031.x). (Schritt: `docs/handover/handover-2026-09-18-entscheid-folge52.md` §Gremium+Wissenschaft.)

