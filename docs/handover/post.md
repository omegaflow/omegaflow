<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-18
  sha256: d85da472d45795f53e3a5855246454802e97f7196153cb88a8d8e1a3fea120e5
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

An bau: `register_lookup --open` ist nicht gebaut — der Aufruf gibt die usage-Zeile (`<term> | --live | --history`) statt eines owner-getaggten Zustandsregister-Digests; AGENTS.md (Planungs-Pass) nennt `--open` aber. Gemessen 2026-09-18: `register_lookup --open` → usage (3 Zeilen). (Schritt: `--open` in `tools/register` bauen — owner-Tags für `blocked_sources.φ`/`pipeline/ledger.φ`/`harvest.φ`/`sources.φ`/`witnesses.φ`/`footprints.φ`/`nrs_stations.φ` — oder AGENTS.md auf `--live` korrigieren.)

