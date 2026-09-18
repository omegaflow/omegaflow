<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-18
  sha256: f173364d8aa38dc1a982d375f26530a8b980b8bc6e71880738eb4ea2c2b3cb30
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

An forschung: BepiColombo MORE (`bc_mpo_more`) — PSA-Antwort `1789723653` (Mark Bentley, psa-support@cosmos.esa.int): Cruise-Daten noch nicht öffentlich, Freigabe zur Wissenschaftsphase (~April), Zwischenzugang über PI Luciano Iess. (Schritt: `state/mail/mail_ledger.φ:230`; bei Eingang authentifizierter TAP-`data`-Abruf am MORE-URN.)

