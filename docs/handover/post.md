<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-18
  sha256: 0a01d73be2d51c9250cf3ba3c28673022d50bc306b4ad569405cf437fa1ffc6b
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

An ernte: LRO utF harvest — Rat-Verdikt (A+D, 2026-09-18). Der ci_watchdog killt
jeden harvest-Lauf > 2×48s (Median der schnellen Formate, selbstverriegelnd);
lro_trk ist ein ungebundener Voll-Crawl (50 790 .TRK in 61 min, Cap 100k, kein
args/timeout). Fix in einem Atom: (1) `workflow` als bekanntes Feld in
`tools/harvest/src/bin/harvest_reg.rs` + `workflow harvest-long` im lro_trk-Block
(`phi/harvest.φ:45`) — Register-Werkzeug und Register im selben Commit; (2)
`.github/workflows/harvest-long.yml` mit dem harvest.yml-Body; (3)
`harvest-dispatch.yml` routet Formate mit `workflow`-Feld dorthin; (4) Compiler
`--year <n>` — Baum `LRO_<ST>_<n>/YYYYDDD/LSUTDF_<st>_<f3>_<YYYY>_<DDD>_<hhmm>.TRK`,
Filter am YYYYDDD-Verzeichnis (spart den Tages-Fetch); (5) ersten gebundenen Lauf
dispatchen, nie pollen. Volle Reihe, Jahr als Bindeeinheit; E (Parallel-Fetch)
verweigert. `bin/ci_watchdog.sh` unverändert, keine AGENTS.md-Änderung. Nach dem
ersten Erfolg: `shard` im Register = gemessene Asset-Zahl. Die todgeweihten
Re-Dispatches 35316446908/35317963475 lässt der nächste Poll killen.
(Schritt: `harvest_reg.rs` lesen, Feld ergänzen, dann (2)–(5).)

