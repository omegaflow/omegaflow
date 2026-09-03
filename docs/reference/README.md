# Reference

Externes Referenzmaterial im nativen Format (kein Header). Interne
omegaflow-Specs liegen unter `docs/concepts/` (`binary-protocol.md`,
`force-system.md`, `constants.md`, `time.md`, `url-templates.md`,
`broken-null-control.md`, `te-literatur-matrix.md`,
`gpu-watchdog-device-loss.md`); der generierte Kernel-Index unter
`phi/KERNEL_INDEX.md`.

| File | Content | Source |
|------|---------|--------|
| `NIST_SP330_tables.md` | SI base units, derived units, prefixes | NIST SP 330 (SI Brochure, 9th ed, 2019) |
| `NIST_SP811_units.md` | Unit naming, API normalization, non-SI conversions | NIST SP 811 (SI Usage Guide, 2008) |
| `ucum-essence.xml` | Complete UCUM v2.2 machine-readable unit registry | ucum-org/ucum |
| `naif_body_ids.tsv` | NAIF body-ID ↔ name (eincompiliert in `src/ephemeris.rs`) | NAIF |
| `extractPC.for`, `getascomPC.for` | FORTRAN-Referenz | DASTCOM/JPL |
| `12_intro_to_kernels.pdf` | NAIF-Tutorial (PDF) | NAIF |
| `NAIF_DAF_REQUIRED_READING.md`, `NAIF_PCK_REQUIRED_READING.md` | NAIF-Lesepflicht | NAIF |
