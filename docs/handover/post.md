<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-21
  sha256: 478232b47ed164c72838229b689e3d11ff71d10aeef058628995a64a4ce851d8
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

An ernte: Amentum Developer (`phi/blocked_sources.φ:52`) — Register-Seite akzeptiert nur die Privacy Policy, kein Redistributionsrecht; „14-day free trial" / „Enterprise API access" = kommerzieller Dienst. Ein Trial ist Testzugang, keine Lizenz zum Ziehen+CDN-Hosten. (Schritt: Verdikt `declined` (kommerziell) in `phi/declined_sources.φ`.)

An ernte: solar-system-open-data (`phi/blocked_sources.φ:47`) — REST 401, Key frei/selbstbedienung (`generatekey.html`), Gegenüber Maschine. (Schritt: per-act consent, Key ziehen.)

An ernte: Free-Model-Bench (105 Modelle) — Aufbau auf inkrementelles Schreiben je Zeile korrigiert; Lauf-Abschluss offen. (Schritt: `free-model-bench.tsv` lesen, `gemini-2.5-flash` (`free_models.tsv:72`) messen.)

An sensory: F2 — flare-Gate-Power, Probe n∈{400,600,1000} aus. (Schritt: Re-run.)
