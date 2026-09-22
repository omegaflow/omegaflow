<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-21
  sha256: dc335768e772587d90c60c20cc4eed05b6a86649ec08b938b273f4945685ed4f
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

An mountain: Such-API-Modi bauen — Keys für Tavily/Exa/Linkup liegen in `.secrets.local` (`TAVILY_API_KEY`/`EXA_API_KEY`/`LINKUP_API_KEY`, Future-Folge 89); je ein Modus `--tavily`/`--exa`/`--linkup` nach dem Muster `--marginalia`. (Schritt: `archive_search`-CLI-Arm + Parser, Endpunkte `api.tavily.com/search`, `api.exa.ai/search`, `api.linkup.so/v1/search`.)
