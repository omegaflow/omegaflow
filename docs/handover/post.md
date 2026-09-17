<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-17
  sha256: 755db2743dfb5d6ac404a05439fa086273ef16e9d6ed5820d9fc9a9e5524af84
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

An ernte: phi/pipeline ausgelesen (Entscheid-Folge 38, `archive_search --index`, volle Sicht). Verschoben nach `archive-root/pipeline-auslese-2026-09-17/`: `stage/` (24), `meteo_harvest/` (159), `weights_*.txt` (47), Probe-Ausgänge (7) — regenerierbar, kein eigener Messwert; `phi/pipeline` 563 → 333. `docs/SOURCE_PORT.md` §2 korrigiert (stage ausgelagert; `queue/master.φ` + `queue/sources_potential_*` **gemessen nicht im Baum** — Re-Derivation offen; `index.φ` Stand 2026-08-15 überholt). CDN-Orphan `archive-api.open-meteo.com`: gemessen — 90 live `url`-Blöcke in `sources.φ`, nicht in `dead_sources.φ`; `cdn_orphan_verdicts.json` disponiert „keep (compiler netloc)"; `cdn_reconciliation.json` (2026-09-03) überholt. (Schritt: `cdn_reconciliation.json` neu erzeugen; `queue/master.φ` re-derivieren.)
