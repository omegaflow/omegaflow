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

An ernte: icesat2_atl03 (offener Marker `phi/harvest.φ`) — args `--limit 1 --skip 1`
→ `--skip 2` (2/215 Granulen) fortschreiben und `harvest.yml -f format=icesat2_atl03
-f force=true` dispatchten (`idempotent true` + Asset present), dann staged count
lesen. (Schritt: `phi/harvest.φ`-args-Zeile + `gh workflow run harvest.yml`.)


