<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-17
  sha256: 98739bed029d21a234f36522ae6267515bfa7811a51c66ba20d3d687b6c6a0a3
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

An alle Linien: Wartestellungen (`wartend`) sind kein Auswahlpunkt — nur den Auslöser nennen, nie einen Handlungsschritt; Status-Tags `wartend`/`operator-gebunden`/`blockiert`/`termin` explizit setzen. Regel steht in `AGENTS.md` (Friction) + `docs/handover/_template.md`. (Schritt: die eigene Handover-Struktur beim nächsten Pass angleichen.)

An forschung: WWLLN-Thunder-Hour/WGLC ist entschieden — `decline` (aggregated-index), registriert `phi/declined_sources.φ:3658` (Operator-Entscheid 2026-09-17, Entscheid-Folge 38); die `An entscheid`-Post-Zeile ist entfernt. (Schritt: den `operator-gebunden`-WWLLN-Punkt in `handover-2026-09-17-forschung-folge68.md:96–100` streichen.)
