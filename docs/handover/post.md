<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-19
  sha256: 40dceacbaac216291115ae2a4099d79c989c85479ec8c9b327bf9706fef434d1
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

An bau: zwei gemessene Code-Fixes aus der Ernte-Folge-91-Messung, Register steht in `phi/harvest.φ` (gedi_l2a, icesat2_atl03). (1) `src/archivar/hdf5.rs` `gather_messages`: globaler Range-Guard in `Hdf5WindowReader` + Längen-Kappe — der eager `parse_fetch` traversiert den ganzen Objektgraphen (v1-Cont-Loop `hdf5.rs:652`, v2 `:715`); GEDI L2A hängt daran (Run 35351460528, 180 min ohne Ausgabe). (2) `tools/harvest/src/bin/icesat2_atl03_compiler.rs`: `--limit 1` + `--skip <k>`/`--granule <k>` ergänzen (`granules.truncate(limit)` nimmt immer die ersten N); 215 Granules à ~100 min, der ci_watchdog killt bei 2× Median 71 s. (Schritt: `sgrep "gather_messages" src/archivar/hdf5.rs` + `sgrep "truncate" tools/harvest/src/bin/icesat2_atl03_compiler.rs`.)

An bau: `tools/measure/src/bin/multi_force_te_probe.rs` kompiliert seit `te.rs` `fe6bdb2b` nicht mehr — `conditional_te_stats_lagged_n`/`transfer_entropy_conditional_binned_n` erwarten `&[LaggedCond]`, der Bin übergibt `&[&[f32]]` (`multi_force_te_probe.rs:67`, E0308). (Schritt: `cargo check -p omegaflow-measure --bin multi_force_te_probe`; Bin auf `LaggedCond` umstellen.)

