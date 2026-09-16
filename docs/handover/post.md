<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-16
  sha256: 6640b34fdcfdd8c4e92bb067d2bbb45d2632efb4c4faadbf53583dedf58b46d9
  status: live
  see-also: AGENTS.md
-->
# Post — Nachrichten zwischen den Linien

Eine Nachricht an eine Linie steht hier — `An <line>: … (Schritt: …)` — nie im
eigenen Handover. Der Empfänger faltet sie in derselben Session in sein eigenes
Handover ein und löscht die Zeile.

An bau: Membran-Leser fehlen für die registrierten Serien-Formate — `src/archivar/extract.rs` `series_parse_bin`/`series_component_name` + die `main_flow.rs`-`matches!`-Liste haben keinen Arm für `juno_odf`, `magellan_odf`, `mgs_odf`, `mro_odf`, `odyssey_odf`, `messenger_odf`, `mars_express_odf`, `rosetta_odf`, `voyager_odr`, `cors_rinex`, `tnf` (auch die schon registrierten `vex_odf`/`galileo_odf`/`dawn_odf`/`galileo_odr` sind inert), plus die sechs Compiler-Formate VLST/VLCT, NCDO, GED1, AT31, SWS1. (Schritt: Leser je Format + Field-Key-Mapping nach `src/archivar`, Muster `geo.rs`/`voyager_saturn`.)
An bau: Voyager-ODR-RSS-Reihen-Leser-Arm in `src/archivar/extract.rs` (`series_parse_bin`: Epoche aus Jahr [PDS3-Label `START_TIME`] + BCD-Zeit-Tag; das Roh-Pack-`.bin` `VODR` liest `voyager_odr::parse_packed`); `galileo_odrs` wartender Reader im selben ODR-Reader-Atom; die `sample_count`-Semantik (Spec ≤ 299999 vs. gemessene Record-1-Werte ~4,29e9, Offset 52 bestätigt) verifizieren. (Schritt: extract.rs + Label.)

An alle Linien (format-Gate): CI-`format` rot @625452e5 — fremde unformatierte Dateien: `src/gate/commit_gate.rs:540`, `tools/measure/src/bin/{aia_ladder_probe,trishuli_gauge_probe}.rs`, `tools/utils/src/bin/archive_search.rs` (die ernte-eigenen fünf harvest-Compiler trägt das Ernte-Handover). (Schritt: rustfmt-Diff aus `ci-check` run 35091175017 anwenden.)

