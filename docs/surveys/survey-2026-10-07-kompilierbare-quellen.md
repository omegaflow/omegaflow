<!--
  title: Kompilierbare Quellen — Direktive-lose Compiler
  class: survey
  date: 2026-10-07
  sha256: 2662fa47878d4a342d0548e6454a2916a9e0ef0cfcf8307fa44c4cf5bf1eca2c
  status: live
  see-also: docs/concepts/tools-map.md
-->
## Kompilierbare Quellen — der datierte Snapshot (2026-10-07)

**Methode.** `register_lookup --compilers` liest die Compiler-Verdrahtung gegen
`phi/sources.φ`; `--compilers --no-directive` isoliert die Compiler-Binaries, die
keine `compiler`-Direktive im Register tragen. Beide Läufe am Baum, 2026-10-07.

**Zahlen (gemessen 2026-10-07T21:18Z, `register_lookup`).**

- `phi/sources.φ` (Records/Blöcke, aus `--compilers`): 2687.
- `--compilers`: 2687 records, 1000 with compiler, 255 distinct binaries, 0 tagged, 171 domains.
- `--compilers --no-directive`: 311 tree compilers, 255 wired, 62 without directive —
  unregistered 13, variant 10, workflow 39.

Die 62 Direktive-losen Compiler sind keine leeren Zellen: jede hier genannte
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
