<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-20
  sha256: 64fabf8e1cdcb95779b2ef8b0a1d6178099a3f667cdea8f596d70ba4bb983175
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

An entscheid: Extension „OpenCode Browser" (`cabnfapnafjlijmbpmgjkgobhdkbmpci`, Chrome Profile 1) ist verbunden; Cookie-Editor (`hlkenndednhfkekhgcdicdfddnkalmdm`) in Profile 1 installiert. Der Cookie-Transfer ist mechanisch gebaut: `archive_search --playwright` liest `OMEGAFLOW_COOKIES=<cookie-editor.json>` und setzt die Cookies via `context.addCookies` vor `goto`. Der Export ist session-seitig nicht führbar (gemessen 2026-09-20: die Bridge liest keine fremde `chrome-extension://`-Seite; chrome-devtools-MCP ist ein eigener Browser; kein CDP `9222`) — er bleibt Operator-Akt. (Schritt: bei Bedarf, Ziel-Site aktiv → Cookie-Editor → Export → `state/cookies/<host>.json`.)

An entscheid: Pine64 (`info@pine64.org`, Ledger `1789930255`) sagt einen Ox64 SBC zu und bittet um Versanddaten + Telefonnummer (Dritt-Mail + PII). Operator-gebunden — in die Operator-Queue legen; Antwortentwurf in `state/mail/` (gitignored), PII nie getrackt. (Schritt: Operator-Wort zum Senden; `smail --dry-run` vor dem Akt.)


