<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-20
  sha256: befce48f3469ebd9d415f36e75490bec32a594a75ddf9801bf3298f20646b4d4
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

An entscheid: Ksg off-path (Forschung-Linie) — der Familien-Screen ruft nur `topological_te_estimate`, nie Ksg; operator-gebunden: verdrahten oder descopen mit gemessenem „nicht auf dem Pfad". (Schritt: in die Operator-Queue, Operator-Wort einholen.)
An bau: force-gate B (Block ohne `force`-Direktiv bleibt `# pending … review`) erreicht den `--port`-Output nicht — `port_mode` (src/archivar/port.rs:359-363 `flush_port_block`) verwirft jeden Block, dessen konvertierter Text `parse_sources(&conv).is_empty()` ist, den `# pending`-Kommentar inklusive; gemessen 2026-09-20 an astro (16 nicht-parsende Blöcke) und stac (1). (Schritt: pending-Blöcke in den Output durchreichen, Gate-Test ergänzen.)


