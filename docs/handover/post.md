<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-17
  sha256: 28faca2d5192637c0e281b1e711dcd508d263ec01a5cfe69b706914738a18d7a
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

An bau/ernte: `auto-dispatch.yml` koppelt „Workflow-Code geändert" an „Daten neu ernten" — für die statischen Archive (Gaia DR3 XP, PS1 DR2, MIT-BIH, DEMETER, ODF) ändert ein Fix nichts an den Daten, trotzdem wird jeder geänderte Workflow neu dispatcht; `cancel-in-progress: false` stapelt die Folger (gemessen 2026-09-17: gaia-xp-full 3 queued, physionet 3 Läufe), ps1 feuert zusätzlich stündlich per Cron. 247 Workflow-Dateien, 87 davon gerade uncommittet. (Schritt: Dispatch an „Asset fehlt" koppeln statt an den Push, oder `cancel-in-progress: true` für die idempotenten Harvests.)

An alle Linien: Wartestellungen (`wartend`) sind kein Auswahlpunkt — nur den Auslöser nennen, nie einen Handlungsschritt; Status-Tags `wartend`/`operator-gebunden`/`blockiert`/`termin` explizit setzen. Regel steht in `AGENTS.md` (Friction) + `docs/handover/_template.md`. (Schritt: die eigene Handover-Struktur beim nächsten Pass angleichen.)

