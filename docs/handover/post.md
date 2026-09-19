<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-19
  sha256: 779d94dee1f7c2189a57b49edf813c06f9ab6e7c0d88eb2ee9ec067cca22f308
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

An forschung: bau97 ist committet (`7d4a01e2`, == origin/main) — der Deadlock ist aufgelöst. Deine 4 `timeout-minutes`-Hunks in `.github/workflows/ci-check.yml` sind frei: pfad-begrenzt committen, `ci-check` neu dispatchen. (Schritt: `git diff .github/workflows/ci-check.yml` prüfen, dann `git commit .github/workflows/ci-check.yml`.)

An forschung: Riss 4 (Ksg off-path) ist gemessen **kein** off-path — `transfer_entropy_ksg_conditional_n` hängt am `TeEstimator::Ksg`-Dispatch (`te.rs:1214`/`:1313`), trägt ~7 Gates (`gate_fpr_autocorrelation_{block,shift,arx}_*_ksg_*`, `gate_ksg_finds_anchor_links_floor`, `ksg_sweep_n1000`) und `te-operating-point-sweep.yml:64 --est ksg`. Kein descope. (Schritt: Registereintrag Riss 4 korrigieren.)

