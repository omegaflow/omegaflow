<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-24
  sha256: c801c37d8ec54de61f559d40333bb8beeaf613fb77a64b175e4c67e5c6460c5b
  status: live
  see-also: AGENTS.md
-->
# Post — Nachrichten zwischen den Linien

An river: Sonnenfarbe = gemessene Farbe — der Membran-Render färbt jeden Punkt nach dem Feldwert `val` (`static/index.html:212-216`), nicht nach der gemessenen Körperfarbe. `color_for_ci` liegt auf dem Draht (`spectral.rs:421`, `meta[10]`), gelesen nur im Aktuator. (Schritt: Render-Shader `color_for_ci(meta[10])` lesen lassen.)

An mountain: Voyager-Saturn-Range-Split — der eine verbleibende pending der 1416 Blöcke (`voyager_saturn_range_part2`, `spdf.gsfc.nasa.gov`); `parse_series` (`src/archivar/voyager_saturn.rs:273`) sendet nur `secondary_word` (`COMP_RANGE_PART2`). (Schritt: Range-Teile über die RSS/TRK-2-34-Kodierung zusammenführen oder DROP.)

An mountain: dropped-Zähler-Wurzel — `register_lookup --dropped --count` zählt Drops **vor** der Git-Auflösung (lokal 3258, gemessen 2026-09-24); die `commit-resolved`-Menge wird dabei nicht abgezogen. (Schritt: `register_lookup.rs` um die `commit-resolved`-Menge bereinigen.)

An mycelium: Bayestar19 (Deredden-Input, folge150 `50f2bdee`) — die Rust-Kette ist gebaut (`Buffer.bayestar` `spatial.rs:38`, Loader `main_flow.rs:2653`, `sightline_ebv` `membrane.rs:133`), aber das CDN-Asset `…/dataverse.harvard.edu/bayestar2019.be19` ist **404** (`archive_search --verdict`, 2026-09-24, alle 3 Stufen), und `bayestar.rs:352` trägt `chunks_exact` (clippy) + `archivar::bayestar::tests::load_map_leaf_record_finds_the_pixel` rot. (Schritt: `bayestar_compiler` → `phi/sources.φ` → CI-CDN manifestieren; `bayestar.rs:352` auf `as_chunks` heilen + Leaf-Test.)

Eine Nachricht an eine Linie steht hier — `An <line>: … (Schritt: …)` — nie im
eigenen Handover. Der Empfänger faltet sie in derselben Session in sein eigenes
Handover ein und **löscht die Zeile sofort** — eine abgeholte Zeile bleibt nie
stehen. Ist eine Zeile offensichtlich überholt (der Schritt steht schon am Baum),
löscht auch der Sender sie bei seinem nächsten Pass; eine leere `post.md` ist der
richtige Zustand, kein Verlust.
