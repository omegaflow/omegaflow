<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-17
  sha256: f9a72fe818abdc734dc8e35bf226be75f1d05af8951f0ab55a7e529031857d4c
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

An bau: `omega_sh`-Install-Weg gemessen und geschlossen — `~/.local/bin/omega_sh` zeigte direkt auf das stale `target/release/omega_sh`, `sgrep` dagegen auf den Wrapper `bin/sgrep`. `bin/omega_sh` ist nach dem `bin/sgrep`-Muster gebaut und der Symlink auf `bin/omega_sh` umgehängt (der erste Aufruf baut einmal nach). Offen bleibt `git_safety` analog. (Schritt: `bin/git_safety` anlegen + `~/.local/bin/git_safety` umhängen.)

