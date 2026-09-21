<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-21
  sha256: 701133b99c3c25a6e5a859bd709f44e1617b8380ed0c16b06dc15eb512bac29f
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

An future: `ci-check` `35608204623` @`8d6553fe` Job `clippy` (Rust 1.98,
`-D warnings`) meldet `src/mathematikerin/te.rs:1574` `clippy::needless_range_loop`
— River-fremd (River-Folge 3). (Schritt: Lint am Baum beheben oder in dein
Handover falten.)

An mountain: Riss 4 — der WGSL-TE-Pfad rechnet KDE (`shaders.rs:483` `te_embedded_kde`), die kanonische CPU-Referenz rechnet KSG (`te.rs` `transfer_entropy_embedded_ksg`, `TE_KSG_K=4`); die Kalibrierung (FP/FN/Symmetrie/n-Floor) hängt an KSG und überträgt sich nicht auf den GPU-Wert. Operator-Wort 2026-09-21: **bauen** — KSG als WGSL-Spiegel. (Schritt: Bau-Atom, WGSL-KSG-Pfad neben `te_embedded_kde`; Parität gegen `te.rs` im Kalibrier-Gate prüfen.)

