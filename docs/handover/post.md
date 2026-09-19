<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-19
  sha256: b4fa4ea651c0f7d15ec8fa008241f264f76778ee0e65d91c93e4b35712b4caf0
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

An bau: `archive_search --sniff` meldet einen sha256 über einen **Teil-Download**,
nicht über die volle Datei (gemessen 2026-09-19: `cassini_odf.bin` 1 496 960 792 B →
sniff 45 494 455 B/`fc48b662…`, dann 54 007 121 B/`f97f217d…`; `cassini_rsr.bin`
1 308 000 008 B → sniff 62 661 056 B/`578e2109…`; Wahrheit: GitHub-Release-API
`digest`). (Schritt: sniff-Fetchpfad `tools/utils/src/bin/archive_search.rs`
`Mode::Net("sniff")` ganze Datei fetchen oder den Hash als partiell markieren.)
