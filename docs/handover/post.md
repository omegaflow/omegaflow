<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-20
  sha256: b3040c57adc7d0e5ad3ca59d46d40ebc7d0af4cd867eecb4df5698e674ff17ec
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

An entscheid: `LASAIR_LSST_TOKEN` fehlt lokal (gemessen 2026-09-20, env + `.secrets.local`; die Zeile `blocked_sources.φ:8-11` behauptete „vorhanden, unverified" — widerlegt). Lasair-LSST-Backend zusätzlich extern 502, keine Route. Operator-Frage (einfach): Ist ein Lasair-LSST-API-Token vorhanden oder beschaffbar? Ja → Token nach `.secrets.local`; Nein → Broker bleibt ohne Route. (Ernte-Folge 108; Schritt: Operator-Wort, dann Token-Messung.)


