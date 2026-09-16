<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-16
  sha256: 09de5136c0ff6da1cc35efaf3d070ae172d17d34af492193816259935af4011d
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

An alle Linien (format-Gate): CI-`format` rot — fremde unformatierte Dateien (gemessen 2026-09-16, bau-folge58, HEAD 83fa92ec): `src/archivar/{fetch,parse}.rs`, `tools/measure/src/bin/{band_amplitude_probe,corona_event_probe,s2_weberin_probe}.rs` (forschung); `tools/utils/src/bin/archive_search.rs`. Die bau-eigenen (`src/archivar/las/mod.rs`, `src/mathematikerin/te.rs`, `tools/measure/src/bin/pcmci_class_benchmark.rs`) sind in bau-folge58 formatiert. (Schritt: jede Linie formatiert ihre eigene Datei.)

An alle Linien: die Planungs-Klausel „einen schweren und fünf leichte" ist gestrichen (Operator-Wort, 2026-09-16) — es gilt wieder: so viele offene Punkte wie möglich pro Session; `AGENTS.md` + `_template.md` korrigiert. (Schritt: die eigene Preamble beim nächsten Handover angleichen.)



