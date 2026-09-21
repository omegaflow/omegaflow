<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-21
  sha256: 9fe7c511d5df6cf789c17a79be3b31c8d39fdcd5fafa0be36b47e59c49d5386e
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

An river: `ci-check 35608204623` @`8d6553fe` trägt vier rote lib-Tests aus deinen
Armen — `archivar::quaoar_occlt::tests::date_midnight_unix_reads_the_calendar_date`,
`archivar::quaoar_occlt::tests::zip_entries_refuse_truncated_and_foreign`,
`archivar::fai_kz::tests::parse_obscore_csv_skips_a_row_whose_position_is_void`,
`archivar::hfrnet_rtv::tests::parse_csv_carries_measured_components_and_drops_absent`;
kein Commit seither hat diese Dateien berührt (Schritt: `ci_manage log 35608204623
--all` lesen, Wurzel messen, Fix + Gate im selben Atom).

An mountain: Riss 4 — der WGSL-TE-Pfad rechnet KDE (`shaders.rs:483` `te_embedded_kde`), die kanonische CPU-Referenz rechnet KSG (`te.rs` `transfer_entropy_embedded_ksg`, `TE_KSG_K=4`); die Kalibrierung (FP/FN/Symmetrie/n-Floor) hängt an KSG und überträgt sich nicht auf den GPU-Wert. Operator-Wort 2026-09-21: **bauen** — KSG als WGSL-Spiegel. (Schritt: Bau-Atom, WGSL-KSG-Pfad neben `te_embedded_kde`; Parität gegen `te.rs` im Kalibrier-Gate prüfen.)

