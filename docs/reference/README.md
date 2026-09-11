# Reference

Externes Referenzmaterial im nativen Format (kein Header). Interne
omegaflow-Specs liegen unter `docs/concepts/` (`binary-protocol.md`,
`force-system.md`, `constants.md`, `time.md`, `url-templates.md`,
`broken-null-control.md`, `te-literatur-matrix.md`,
`gpu-watchdog-device-loss.md`); der generierte Kernel-Index unter
`phi/KERNEL_INDEX.md`. Publizierte Literatur liegt unter `docs/paper/`;
Seeds/Zensus unter `docs/surveys/`.

| File | Content | Source |
|------|---------|--------|
| `NIST_SP330_tables.md` | SI base units, derived units, prefixes | NIST SP 330 (SI Brochure, 9th ed, 2019) |
| `NIST_SP811_units.md` | Unit naming, API normalization, non-SI conversions | NIST SP 811 (SI Usage Guide, 2008) |
| `ucum-essence.xml` | Complete UCUM v2.2 machine-readable unit registry | ucum-org/ucum |
| `naif_body_ids.tsv` | NAIF body-ID ↔ name (eincompiliert in `src/ephemeris.rs`) | NAIF |
| `extractPC.for`, `getascomPC.for` | FORTRAN-Referenz | DASTCOM/JPL |
| `12_intro_to_kernels.pdf` | NAIF-Tutorial (PDF) | NAIF |
| `NAIF_DAF_REQUIRED_READING.md`, `NAIF_PCK_REQUIRED_READING.md` | NAIF-Lesepflicht | NAIF |
| `42-122-keihm-wvr-fullyear.pdf` (`.txt`) | Goldstone tropospheric delay fluctuations, full-year WVR (DSS 13, Oct 93–Sep 94) — wet-delay ASD vs Δt, seasonal/day-night | Keihm 1995, TDA Prog. Rep. 42-122, pp. 1–11 |
| `42-158-keihm-media-cal.pdf` (`.txt`) | Tropospheric delay statistics + MCS calibration performance at DSS 25 (19-month archive) — wet/dry ASD, structure functions, spectra | Keihm/Tanner/Rosenberger 2004, IPN Prog. Rep. 42-158 |
| `42-148-mcs-part3.pdf` (`.txt`) | Media Calibration System for Cassini Radio Science, Part III — tracking-ASD requirements (1.5×10⁻¹⁵ two-way), WVR-calibrated CEI residuals | Resch et al. 2002, IPN Prog. Rep. 42-148 |
