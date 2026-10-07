<!--
  title: Kompilierbare Quellen — Direktive-lose Compiler
  class: survey
  date: 2026-10-07
  sha256: 10ae5c6d1cec6a1294ea80c0ef65f522ca39e6e3c7c26f0b0d0ed7b3b5780864
  status: live
  see-also: docs/concepts/tools-map.md
-->
## Kompilierbare Quellen — der datierte Snapshot (2026-10-07)

**Methode.** `register_lookup --compilers` liest die Compiler-Verdrahtung gegen
`phi/sources.φ`; `--compilers --no-directive` isoliert die Compiler-Binaries, die
keine `compiler`-Direktive im Register tragen. Beide Läufe am Baum, 2026-10-07.

**Zahlen (gemessen 2026-10-07, `./target/debug/register_lookup`).**

- `--compilers`: 2684 records, 997 with compiler, 252 distinct binaries, 0 tagged, 168 domains.
- `--compilers --no-directive`: 311 tree compilers, 252 wired, 64 without directive —
  unregistered 13, variant 10, workflow 41.

Die 64 Direktive-losen Compiler sind keine leeren Zellen: jede hier genannte
Klasse trägt einen gemessenen Anker im Baum (Witness, Binding, Reader-Arm,
Workflow, Schwestern-Direktive).

**Klassen mit gemessenen Ankern (geprüft 2026-10-07 mit `sread`).**

- **amon** — `tools/harvest/src/bin/amon_compiler.rs` (workflow). Anker
  `phi/witnesses.φ:44` (Witness `s2-direction`: „Kompiliert als icecube_alerts.amn1
  via amon_compiler, CDN"). **resolved.**
- **gebco** — `tools/harvest/src/bin/gebco_bathymetry_compiler.rs` (workflow).
  Anker `phi/bindings/bathymetrie-gebco.φ` (binding: GEBCO2020 + eudem25m, `unit m`,
  `gbco-Records`). **resolved.**
- **mitdb** — `tools/harvest/src/bin/mitdb_compiler.rs` + `mitdb_rr_compiler.rs`
  (workflow). Anker `src/archivar/extract.rs:490` (Component-Arm `"mitdb"`) +
  `.github/workflows/physionet-cdn.yml:19` (Job `mitdb`). **resolved.**
- **superdarn** — `tools/harvest/src/bin/superdarn_compiler.rs` (variant; Familie).
  Anker `phi/sources.φ:18098` (`compiler …/superdarn_fitacf_compiler.rs`) und
  `phi/sources.φ:10247` (`compiler …/superdarn_rawacf_compiler.rs`). **resolved.**
- **ionex** — `tools/harvest/src/bin/ionex_compiler.rs` (workflow). Anker
  `docs/handover/archiv/handover-2026-09-16-ernte-folge45.md:45` („IONEX — CDN-Route",
  CDN-GIM1 vs. rohe Route). **resolved.**

Kein Anker `pending`; kein Anker löste sich nicht auf.

**Verweis.** Die lebende Liste ist `register_lookup --compilers` /
`register_lookup --compilers --no-directive`; dieses Blatt ist der datierte
Snapshot, kein Ersatz. Kein 996-Zeilen-Ausdruck.
