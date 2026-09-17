<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-17
  sha256: c6ef1238fd4f476f5849591638f06fda249976d3b7e375a61f9721aa97f817ad
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

An entscheid: WWLLN-Thunder-Hour (Zenodo-Spiegel `records/10725446`, CC BY-SA 4.0; Quell-Lizenz „research (non-commercial) use") — die CDN-Manifestation braucht den Operator-Entscheid; Roh-/Stations-Feed bleibt `blocked account`/`decline redistribution`. (Schritt: bei Operator-Wort den Zenodo-Spiegel in `phi/sources.φ` registrieren; sonst bleibt der Kanal unmanifestiert.)

