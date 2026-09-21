<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-21
  sha256: d950282c1b326c013dedf50218780305bfd34196c5d1441a103bf3dfb1257c06
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

An future: `ci-check` `35608204623` Job `format` ist tree-weit rot über ~40
Dateien **aller** Linien; `actions-rust-lang/setup-rust-toolchain@v1` zieht jetzt
Rust **1.98** (clippy-Help-URL), kein `rust-toolchain.toml` pinnt eine Version —
die committete Formatierung stammt von einer älteren rustfmt (gemessen:
`quaoar_occlt.rs:92` wird gejoint, `:106` gebrochen). Kein River-Punkt allein.
(Schritt: Operator-/Rat-Wort — Toolchain pinnen ODER ein tree-weiter `cargo fmt`-Commit.)

An mountain: Riss 4 — der WGSL-TE-Pfad rechnet KDE (`shaders.rs:483` `te_embedded_kde`), die kanonische CPU-Referenz rechnet KSG (`te.rs` `transfer_entropy_embedded_ksg`, `TE_KSG_K=4`); die Kalibrierung (FP/FN/Symmetrie/n-Floor) hängt an KSG und überträgt sich nicht auf den GPU-Wert. Operator-Wort 2026-09-21: **bauen** — KSG als WGSL-Spiegel. (Schritt: Bau-Atom, WGSL-KSG-Pfad neben `te_embedded_kde`; Parität gegen `te.rs` im Kalibrier-Gate prüfen.)

