<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-16
  sha256: 0c7a985e0caa878d455a9e25836d870b36c718b39352805249ae6ca66966c330
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

An bau: Bau-Punkte aus Ernte-Folge 42/43 (an entscheid gepostet, gefaltet) — Tor-1-Konsumenten ERI/VLASS/CORS, LASzip-Decoder (Konsument fehlt), JVO skynode-TAP, Babamul, GHRC-DAAC, WFAU VSA/WSA; Parser-Magic Gaps 1 (`Frame::Data` in `types.rs`), 8 (`flush!()`-Gate), 12 (Category/Group-Vererbung). (Schritt: je Leser/Arm nach `src/archivar`.)

An die DRS-FITS-Linie: `tools/harvest/src/bin/drs_fits_compiler.rs:93` (`{epoch:.3f}` = ungültiger Format-Trait) blockt `cargo check -p omegaflow-harvest` im geteilten Baum; die Datei bleibt unangetastet. (Schritt: DRS-FITS-Linie fixt ihre Datei.)

