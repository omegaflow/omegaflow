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
| `Radio Science - 2005 - Asmar - …precision radio science.md` (`.pdf`) | Doppler-noise budget (noise sources & Allan deviations, transfer functions, two-way spectrum, sensitivity limits) — Deduktion-43 anchor | Asmar/Armsstrong/Iess/Tortora 2005, Radio Sci. 40, RS2001, doi:10.1029/2004RS003101 |
| `42-122-keihm-wvr-fullyear.pdf` (`.txt`) | Goldstone tropospheric delay fluctuations, full-year WVR (DSS 13, Oct 93–Sep 94) — wet-delay ASD vs Δt, seasonal/day-night | Keihm 1995, TDA Prog. Rep. 42-122, pp. 1–11 |
| `42-158-keihm-media-cal.pdf` (`.txt`) | Tropospheric delay statistics + MCS calibration performance at DSS 25 (19-month archive) — wet/dry ASD, structure functions, spectra | Keihm/Tanner/Rosenberger 2004, IPN Prog. Rep. 42-158 |
| `42-148-mcs-part3.pdf` (`.txt`) | Media Calibration System for Cassini Radio Science, Part III — tracking-ASD requirements (1.5×10⁻¹⁵ two-way), WVR-calibrated CEI residuals | Resch et al. 2002, IPN Prog. Rep. 42-148 |
| `armstrong-woo-estabrook-1979-interplanetary-scintillation.pdf` (`.txt`) | Interplanetary phase scintillation & VLF gravitational-wave search — S-band (2.3 GHz) fractional-frequency floor ~3×10⁻¹⁴ at 1000 s, elongation dependence, Kolmogorov (a=8/3) | Armstrong, Woo & Estabrook 1979, ApJ 230, 570—574 |

**Journal-Artikel — Volltext paywalled, Abstract-Record angelegt (0 honored):**
- `woo-armstrong-1979-jgr-abstract.md` — Woo & Armstrong 1979, *Spacecraft radio scattering… electron density fluctuations in the solar wind*, JGR 84, 7288, doi:10.1029/JA084iA12p07288. Abstract verbatim; Pioneer-Deduktion-3-Komponente. α=1.65, R^−3.45.
- `armstrong-1998-phase-scintillation-abstract.md` — Armstrong 1998, *Radio wave phase scintillation and precision Doppler tracking of spacecraft*, Radio Sci. 33, 1727, doi:10.1029/98RS02317. Abstract verbatim; kosin²-Modulation des Troposphären-Spektrums am two-way light time.
