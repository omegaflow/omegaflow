<!--
  title: The H₀ lines register — roots instead of witnesses
  class: paper
  date: 2026-09-12
  sha256: 48d29a82f231cb5955e20f9fecd6a5ed03e6ec42d646683f44c874a12fa224f1
  status: live
  see-also: docs/blatt/blatt-h0-linien-register.md
-->
# The H₀ lines register — roots instead of witnesses

## Abstract

The register weighs ~40 published H₀ measurements into ten root families plus a compilation ridge, counting shared roots rather than witnesses. Two trees stand on separate ground: the distance-ladder root (geometric anchors: NGC 4258 maser, LMC DEB, Gaia parallaxes) and the CMB root (the sound horizon r_d/θ*). They share no root; the common root is absent. Planck 2018 gives 67.36 ± 0.54 km/s/Mpc; the ladder gives 73.04 ± 1.04 (SH0ES), about 5σ apart. The asymmetry: the ladder's Cepheid class is weighed in-house through a Gaia TAP query of gaiadr3.vari_cepheid, type_best_classification = 'DCEP', N = 1606, inverse-variance weighted parallax mean 0.2619 ± 0.0004 mas; the CMB likelihood is weighed in-house from the R3.00 baseline chain (100θ_* = 1.041099 ± 0.000307, r_d = 147.091 ± 0.265 Mpc, H₀ = 67.358 ± 0.539 km/s/Mpc). No third thread carries ≲1–2% precision, so no arbiter is possible yet. The ladder side is weighed end-to-end in-house: H₀ = 73.56 ± 1.40 km/s/Mpc (full STAT+SYS covariance); the own 75-source crossmatch stands — all 74 table rows carry their Gaia DR3 source_id through the 2″ identity gate (offset median +21 μas).

## The measurement

The register weighs ~40 published H₀ measurements into ten root families plus a ridge. Each row carries a witness, year, reference, method, value and uncertainty in km/s/Mpc, a root chain, a separation level, and a route. A source that was fetched beats a citation; a source the harvest did not reach is named `absent` or `pending`, never fabricated.

The ten root families:

1. **Distance ladder · Cepheid** — the own in-house ladder (73.56 ± 1.40, this work), SH0ES (73.04 ± 1.04), the 2021 MW calibration (73.0 ± 1.4), the HST Key Project (72 ± 8), the JWST-era validation (73.4 ± 2.1 Cepheid; 72.6 ± 2.0 combined), the HST full set (73.2 ± 0.9), "The Perfect Host" (73.49 ± 0.93; +TRGB 73.18 ± 0.88).
2. **Distance ladder · TRGB** — Freedman et al. 2021 (69.8 ± 0.6 stat ± 1.6 sys), CCHP 2020 (69.6 ± 0.8 stat ± 1.7 sys), CCHP 2025 (70.39 ± 1.22 stat ± 1.33 sys ± 0.70 σ_SN; JWST-only 68.81 ± 1.79 stat ± 1.32 sys; JAGB 67.80 ± 2.17 stat ± 1.64 sys), Hoyt et al. 2023 (zero point only, H₀ absent), TRGB-SBF III (73.8 ± 0.7 stat ± 2.3 sys).
3. **Distance ladder · Mira** — Huang et al. 2020 (73.3 ± 4.0), Bhardwaj et al. 2025 (73.06 ± 2.67), Sanders 2023 (73.7 ± 4.4).
4. **CMB** — Planck 2018 (67.36 ± 0.54; in-house weighed from the R3.00 lensed baseline chain: 100θ_* = 1.041099 ± 0.000307, r_d = 147.091 ± 0.265 Mpc, H₀ = 67.358 ± 0.539 km/s/Mpc), ACT DR4 alone (67.9 ± 1.5), WMAP9 alone (70.0 ± 2.2).
5. **BAO + CMB** — the sound horizon r_d inherited from the CMB: Planck 2018 + BAO (67.66 ± 0.42), eBOSS CMB+BAO (67.60 ± 0.43), DESI 2024 VI (67.97 ± 0.38), DESI DR2 (68.17 ± 0.28).
6. **BAO + BBN** — the r_d route outside the CMB: eBOSS BAO+BBN (67.33 ± 0.98), DESI 2024 VI BAO+BBN (68.52 ± 0.62), DESI DR2 BAO+BBN (68.51 ± 0.58).
7. **Gravitational sirens** — GW170817 (70.0 +12.0/−8.0), dark sirens O4a (75.4 +12.8/−9.1), GWTC-5.0 (71.7 +9.4/−7.5).
8. **Radio and PTA** — the megamaser MCP XIII (73.9 ± 3.0), the FRB-DM estimate (65.13 ± 2.52, model spread 51–77), the PTA parallax (`absent`).
9. **Plasma and optical one-step** — SZ + X-ray (76.9 +3.9/−3.4 stat, +10.0/−8.0 sys), H0LiCOW (73.3 +1.7/−1.8), TDCOSMO IV (67.4 +4.1/−3.2), gamma EBL (67.4 +6.0/−6.2), TDCOSMO 2025 (71.6 +3.9/−3.3).
10. **Fundamental plane and Tully-Fisher** — Said et al. 2025 (76.05 ± 0.35 stat ± 0.49 sys(FP) ± 4.86 stat(Kal.)), Scolnic et al. 2024 (76.5 ± 2.2), Schombert et al. 2020 (75.1 ± 2.3 stat ± 1.5 sys).

The ridge is the compilation layer: Perivolaropoulos 2024 (ladder N = 20: 72.8 ± 0.5; one-step N = 33: 69.0 ± 0.48; KS p = 0.0001) and Pantos & Perivolaropoulos 2026 (88 sound-horizon-free H₀ in four classes; class 1 ladder n = 30: 72.73 ± 0.39; class 2 local-ΛCDM: 67.61 ± 0.96; class 3 local-only n = 16: 71.03 ± 0.69; class 4 CMB-sound-horizon-free: 69.07 ± 0.44).

The register weighs data from NASA missions: the Hubble Space Telescope (HST Key Project; SH0ES HST), the James Webb Space Telescope (SH0ES JWST validation and "The Perfect Host"; CCHP/Freedman 2025), with bibliographic records through the NASA Astrophysics Data System. The Gaia parallaxes that anchor the Cepheid branch are an ESA mission, named as such.

**The white field.** The CMB root (r_d/θ*) and the ladder root (geometric anchors: NGC 4258 maser, LMC DEB, Gaia parallaxes) share no common root. Common root: `absent`. That absence is the finding itself, not a gap in the register; the independence of the two trees is the crack, and an arbiter over the ~5σ would be fabrication.

**The DCEP weigh.** The in-house weighings on the ladder side are the Cepheid parallax class (here) and the end-to-end ladder H₀ (the finding). The probe `cepheid_parallax_weigh` queries the Gaia TAP service (`gea.esac.esa.int/tap-server/tap/sync`), joining `gaiadr3.vari_cepheid` to `gaiadr3.gaia_source`, keeping rows with `type_best_classification = 'DCEP'` and a positive parallax above five times its uncertainty. N = 1606. The inverse-variance weighted parallax mean is 0.2619 ± 0.0004 mas. The broad floor (all `vari_cepheid` types, N = 2078) weighs 0.2530 mas.

## The finding

**The asymmetry.** The two roots do not stand alike. On the ladder side the register carries the end-to-end in-house weighing (`h0_ladder_weigh`: Cepheid PL in parallax space → SN-Ia calibration → own H₀): H₀ = 73.56 ± 1.40 km/s/Mpc (full STAT+SYS covariance; diagonal checkpoint 73.53 ± 1.14), M_W1 = −5.914 ± 0.017 (published −5.915 ± 0.022), zp = −13 ± 5 μas (published −14 ± 6), M_B = −19.2469 ± 0.0299 (77 calibrators), a_B = 0.7159 ± 0.0018 (flat ΛCDM Ωm = 0.3). The reproduction gate PASSES — every parameter within 1σ of the published value. The CMB side is weighed from the baseline chain: the R3.00 `base-plikHM-TTTEEE-lowl-lowE_lensing` product (25 225 samples, IRSA mirror, sha256 `52cf6793f14e250ffc1436ce7f6fe6d92f6a066c433ec9efc66e4178f3d45a1f`) reproduces every cited number in-house — 100θ_* = 1.041099 ± 0.000307, r_d = 147.0908 ± 0.2653 Mpc, H₀ = 67.3576 ± 0.5388 km/s/Mpc, against the published 1.0411 ± 0.0003, 147.09 ± 0.26, 67.36 ± 0.54; the lensing-free baseline `base-plikHM-TTTEEE-lowl-lowE` (24 497 samples) weighs H₀ = 67.28 ± 0.61, r_d = 147.05 ± 0.30 — the same root one rung lower, the cited row is the lensed baseline. The Planck 2013 R1.10 power spectrum and likelihood tarball (PLA) are fetched and measured (LOW-ELL 48 rows, HIGH-ELL 74 rows, a 74×74 COV-MAT; the 2013 release) and carry no 2018 row — measured, not assumed. The likelihood's own re-evaluation (clik/CosmoMC) remains outside the session; the chain statistics are weighed.

**The separation-level map.** Where the threads part:

- **early** — the ladder's anchor rung: the parallax source (only the Cepheid branch calibrates directly with Gaia EDR3 MW Cepheids and HST scanning; the TRGB branch uses Gaia as a 5% check), the indicator (Cepheid Leavitt law vs TRGB M_I ≈ −4.05 vs Mira PL), and the anchor topology (NGC 4258, LMC DEB — every ladder branch shares the same anchor foundation).
- **late** — the SN-Ia rung: SH0ES uses Pantheon+ (42 calibrators + 277 Hubble flow); CCHP/TRGB uses CSP. The 73.04-vs-69.8 distance sits partly on this rung, not only at the early anchor.
- **none** — the CMB/BAO sound-horizon route and every one-step route.

**The verdict.** Two trees, common root `absent`. No third thread carries ≲1–2%, so no arbiter is possible yet; the young forest's uncertainties (GW sirens ±9–15 km/s, megamaser ±3.0, FRB-DM model spread 51–77) are compatible with both roots. The arbiter is `pending` — the measurement exists, the sharpness does not. The ridge carries the consequence: the tension is not a clean early-vs-late gradient but ladder (72.8) against all others (69.0), KS p = 0.0001.

**The identity that keeps the weigh honest.** The own Gaia-TAP crossmatch of the 75 SH0ES Cepheids stands (2026-09-13): all 74 rows of the 2012.08534 table are resolved through SIMBAD ident→basic into RA/Dec and carry their Gaia DR3 source_id through the 2″ identity gate (offset median +21 μas; 7 rows are `absent` in π_EDR3 and are skipped and counted). The per-source identity lives in the blatt register (`blatt-h0-linien-register.md`, §"Per-Source-Identität — Gaia DR3 source_id"); the weigh does not overstate itself. The DCEP class weigh (N = 1606, 0.2619 ± 0.0004 mas) is the class field, not the ladder H₀; the ladder H₀ is the end-to-end weigh above.

## The form

- **The Gaia TAP leg.** `https://gea.esac.esa.int/tap-server/tap/sync` (`GAIA_TAP_SYNC`, `src/archivar/gaia_sso.rs`), with the DCEP filter: `type_best_classification ∈ {DCEP, T2CEP, ACEP}` in `gaiadr3.vari_cepheid`; the filter narrows to `'DCEP'`, the classical Cepheid subset the SH0ES calibration draws on. Measured 2026-09-13: the ESA host answered 000 from this machine; the ARI mirror `gaia.ari.uni-heidelberg.de/tap/sync` carried the crossmatch probe.
- **The weighing discipline.** A source fetched beats a citation. The Riess 2021 table (arXiv:2012.08534) is not in VizieR — TAPVizieR answers 400 for `J/ApJ/908/L6`; the table lives in the arXiv source package (`bigtable_redux3.tex`). The BBN route's r_d origin is quasar D/H (Cooke, Pettini & Steidel 2018), not CMB-borrowed; that is a measured route, not an assumption.
- **A named measurement without registration.** The living TAP leg materializes no asset (pure measurement, stdout); precedent `cepheid_parallax_weigh`. The frame-less `format reference` seat in `phi/sources.φ` carries byte streams, not a live TAP leg; the leg stays a named measurement.
- **0 honored.** Absence is a realized property: the PTA parallax row, the "BAO alone" row, and the chronometer rows carry `absent`, named. What the harvest did not reach is `pending`, not zero.
- **The NASA binding.** HST (NASA/ESA) and JWST (NASA/ESA/CSA) data are weighed through the published HST Key Project, SH0ES, and CCHP/Freedman 2025 rows; the NASA Astrophysics Data System is the bibliographic archive named for the record route.
- **Named pending points.** (1) the CMB likelihood's own re-evaluation (clik/CosmoMC) — **descoped by measurement (2026-09-16): never built, not needed.** The chain statistics are already weighed in-house from the R3.00 baseline chain (100θ_* = 1.041099 ± 0.000307, r_d = 147.0908 ± 0.2653 Mpc, H₀ = 67.3576 ± 0.5388 km/s/Mpc), so a build would measure nothing new. The path exists — `github.com/benabed/clik` (HTTP 200, C/Fortran/Python, 52.6 MB, clik 16.0, **no LICENSE file** — measured) and the PLA tarballs `COM_Likelihood_Code-v3.0_R3.10.tar.gz` + `COM_Likelihood_Data-baseline_R3.00.tar.gz` (PLA wiki HTTP 200; the earlier 404 was a filename error, `.tgz` vs `.tar.gz`) — but the licence/redistribution gap is measured: no LICENSE file and unclear PLA terms, therefore no CDN manifestation. A later build, if ever needed, has its measured path: waf (Python build tool) + gfortran, **R3.10** for publication parity (not 16.0 — name = implementation), in CI; (2) a third thread with ≲1–2% precision (the arbiter); (3) the live TAP leg is a named measurement without an asset.

## References

1. Riess, A. G., et al. 2022, ApJ 934, L7 — arXiv:2112.04510 (SH0ES)
2. Riess, A. G., et al. 2021, ApJ 908, L6 — arXiv:2012.08534 (MW Cepheid calibration)
3. Planck Collaboration 2020, A&A 641, A6 — arXiv:1807.06209
4. Freedman, W. L., et al. 2021, ApJ 919, 16 — arXiv:2106.15656
5. Freedman, W. L., Madore, B. F., & Hoyt, T. J. 2020 — arXiv:2002.01550 (CCHP)
6. Freedman, W. L., et al. 2025, ApJ 985, 203 — arXiv:2408.06153 (CCHP JWST)
7. Riess, A. G., et al. 2024, ApJ 977, 120 — arXiv:2408.11770 (JWST validation)
8. Riess, A. G., et al. 2025, ApJL 992, L34 — arXiv:2509.01667 ("The Perfect Host")
9. Hoyt, T. J., et al. 2023, Nat. Astron. 7, 590 — arXiv:2106.13337
10. Huang, C. D., et al. 2020 — arXiv:1908.10883 (Mira)
11. Bhardwaj, A., et al. 2025 — arXiv:2507.10658 (Mira)
12. Sanders, J. L. 2023, MNRAS — arXiv:2304.01671
13. Alam, S., et al. 2021, PRD 103, 083533 — arXiv:2007.08991 (eBOSS)
14. DESI Collaboration 2025, JCAP 2025, 02, 021 — arXiv:2404.03002 (DESI 2024 VI)
15. DESI Collaboration 2025, PRD 112, 083515 — arXiv:2503.14738 (DESI DR2)
16. Abbott, B. P., et al. 2017, Nature 551, 85 — arXiv:1710.05835 (GW170817)
17. LVK 2025 — arXiv:2509.04348 (dark sirens O4a)
18. Perivolaropoulos, L. 2024, PRD 110, 123518 — arXiv:2408.11031
19. Pantos & Perivolaropoulos 2026 — arXiv:2601.00650
20. Said, K., et al. 2025, MNRAS 539, 3627 — arXiv:2408.13842 — DOI 10.1093/mnras/staf700
21. Scolnic, D., et al. 2024 — arXiv:2409.14546
22. Freedman, W. L., et al. 2001, ApJ 553, 47 — arXiv:astro-ph/0012376 (HST Key Project)
23. Aiola, S., et al. 2020, JCAP 12, 004 — arXiv:2007.07288 (ACT DR4)
24. Hinshaw, G., et al. 2013, ApJS 208, 19 — arXiv:1212.5226 (WMAP9)
25. Jensen, J. B., et al. 2025 — arXiv:2502.15935 (TRGB-SBF III)
26. Scolnic, D., et al. 2021; Brout, D., et al. 2021 — Pantheon+ SH0ES data release (GitHub)
