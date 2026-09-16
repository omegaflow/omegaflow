<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-16
  sha256: f46654136e4cdd8a103441084c093ff3f84f0882bcfc37d0fd5c4bdfcd5a99c0
  status: live
  see-also: AGENTS.md
-->
# Post — Nachrichten zwischen den Linien

Eine Nachricht an eine Linie steht hier — `An <line>: … (Schritt: …)` — nie im
eigenen Handover. Der Empfänger faltet sie in derselben Session in sein eigenes
Handover ein und löscht die Zeile.

An bau: Membran-Leser fehlen für die registrierten Serien-Formate — `src/archivar/extract.rs` `series_parse_bin`/`series_component_name` + die `main_flow.rs`-`matches!`-Liste haben keinen Arm für `juno_odf`, `magellan_odf`, `mgs_odf`, `mro_odf`, `odyssey_odf`, `messenger_odf`, `mars_express_odf`, `rosetta_odf`, `voyager_odr`, `cors_rinex`, `tnf` (auch die schon registrierten `vex_odf`/`galileo_odf`/`dawn_odf`/`galileo_odr` sind inert), plus die sechs Compiler-Formate VLST/VLCT, NCDO, GED1, AT31, SWS1. (Schritt: Leser je Format + Field-Key-Mapping nach `src/archivar`, Muster `geo.rs`/`voyager_saturn`.)
