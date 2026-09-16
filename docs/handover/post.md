<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-16
  sha256: 9339c1f5cf3d24e29a31dc251815493c4df276da049a3866c2545e861c5e85a7
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

An alle Linien (format-Gate): CI-`format` rot @625452e5 — fremde unformatierte Dateien: `src/gate/commit_gate.rs:540`, `tools/measure/src/bin/{aia_ladder_probe,trishuli_gauge_probe}.rs`, `tools/utils/src/bin/archive_search.rs` (die ernte-eigenen fünf harvest-Compiler trägt das Ernte-Handover). (Schritt: rustfmt-Diff aus `ci-check` run 35091175017 anwenden.)

An ernte: `phi/sources.φ:957` (`on earth 49.4195 8.6767`) trägt kein `alt` — der Parser weist die Zeile spec-konform ab (alt Pflicht, `parse.rs:197-204`), der Surface-Anker geht verloren, der Block sitzt als `Frame::Manifest` mit leerem `body_name`. (Schritt: `alt` messen/ergänzen oder die Frame-Form prüfen.)

An ernte: `phi/dead_sources.φ` Domaincheck 2026-09-16 — 275 Domains, 52 DNS-tot (alle mit Dienst-Identität/Nachfolger = gelebt); 6 nie-gelebt-Einträge entfernt (vokal-ausgedünnt `p.ntrlst.rg`→`api.inaturalist.org`, `p.pn-mt.cm`→`api.open-meteo.com` — beide leben; `lightning.gld`; `masie_ice.apps.nsidc.org`). Relevanz-Erstpass (flash): 91 sichere + 35 unsichere „nie Force-Kanal"-Kandidaten → `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md`. (Schritt: Force-Gate-Verdikt nach SOURCE_PORT.md, dann Prune; die zwei lebenden Dienste ggf. als Quellen registrieren.)

An ernte: Ernte-Register-Rest aus dem entscheid-Handover ausquartiert — MPC-Shard UnnObs, GHRC-DAAC, Survey §1 (26 Pendings); Register-Digest-Rest (Fink/ALeRCE, TDAT/FITS, Akteure, Holdings, Orphan-Verdicts). (Schritt: `sources.φ` + CDN / je Punkt messen.)

