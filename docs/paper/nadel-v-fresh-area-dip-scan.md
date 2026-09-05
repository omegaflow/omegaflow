<!--
  title: The fresh-area achromatic dip scan over forced photometry (Nadel V)
  class: paper
  date: 2026-09-06
  sha256: 7563c1c7d74c9a62f5ac7bc756e0bb17fd5918e95fde88135dd8dee526435019
  status: live
  see-also: docs/concepts/kybernetische-astrophysik.md docs/TODO.md tools/measure/src/bin/lsst_anomaly_probe.rs tools/measure/src/bin/ztf_anomaly_probe.rs
-->
# The fresh-area achromatic dip scan over forced photometry (Nadel V)

## Abstract

The fresh-area result of the Nadel V structural-technosignature scan: a bounded search for unexplained, achromatic, non-periodic optical dimming over a newly harvested surface resolved by dense anonymous forced photometry. Five anonymous routes were measured (Lasair-LSST, Fink, the Lasair-ZTF cone, the fresh IRSA-ZTF harvest `ztf_lightcurves_fresh.bin`, and ANTARES), all through the repaired scale-invariant FAP gate. A scale-dependence in the Lomb-Scargle false-alarm probability had closed the aperiodic gate on every row; the negative control caught it, and the fixed gate now carries genuine 6σ-10σ achromatic injections while still excluding a genuinely periodic natural variable. On the dense Fink forced-photometry surface the dip gates finally measure: two pre-exclusion candidates, both catalogued natural dimmers (an AGN and an SN; one matched to a SIMBAD galaxy), hence zero unexcluded candidates. The quantitative limit: no unexplained achromatic non-periodic dip above 3σ across the freshly measured objects. The reach is bounded honestly per object: single-band sparsity in the early alert cadence, too few coincident two-band visits. The zero is a limit over this surface, not a galaxy census.

## 1. The measurement

Nadel V tests the structural technosignature hypothesis on the light curve: an opaque structure occulting a source dims it achromatically, equally at every wavelength, whereas natural dimming is chromatic (dust reddens, spots are cooler). The anomaly is the aperiodic achromatic dip that survives the exclusion of the catalogued natural dimmers (SN, AGN, QSO, BL Lac, CV, YSO, TDE, Flare, RR Lyrae, Eclipsing, EB, Blazar). A negative result is a quantitative limit, and a quantitative limit is a measurement (0 honored).

The candidate gate, shared by `lsst_anomaly_probe` and `ztf_anomaly_probe`, asks four things of a two-band light curve: a dip significance of at least DIP_SIG 3σ in both bands with both dips negative; the two dip depths achromatic (depth ratio within 0.5-2, the ACHROMATIC_RATIO 2.0 window); aperiodicity under the Lomb-Scargle FAP gate (FAP at least FAP_GATE 0.01); and the sample floors: N_MIN 24 samples per band and N_COINC_MIN 12 coincident two-band visits within COINCIDENCE_S 1800 s. A two-band object below the floors is not measured; its bound is the coverage, not a candidate zero. Every series is evaluated on the jd/MJD to TDB fold axis, so the two bands compare the same emitted moment.

## 2. The scan surface

The scan measured five anonymous routes. The surfaces differ in cadence, band coverage and reach; each route below states its measured surface and its limit.

- Lasair-LSST: the LSST DIA alert stream behind the Lasair REST cone/object endpoints. Early LSST alert cadence is single-band sparse; most objects never accumulate the two-band coincident visits the gate needs, and that sparsity is the per-object bound.
- Fink (LSST): `api.lsst.fink-portal.org` conesearch over real g/r/i/z/u/y LSST light curves, plus the dense forced-photometry endpoint POST `/api/v1/fp` per `diaObjectId`. A single FP query returned 1,033 rows (g 197, i 385, r 188, u 47, y 20, z 196) across MJD 61,090-61,205: the dense multiband history the achromatic test requires, measured at a fixed coordinate and independent of detection.
- Lasair-ZTF cone: historical ZTF object records carrying candidate and forced-photometry rows, folded on the same jd to TDB axis. ZTF bands alternate nightly, so genuine two-band coincidence within 1800 s is rare and the coincidence floor binds most objects.
- Fresh IRSA-ZTF harvest: the region ra 210° dec +30° (radius 0.02°) harvested anonymously from IRSA `nph_light_curves` into the flat CDN asset `ztf_lightcurves_fresh.bin`: 78 light curves, 12,074 samples in g+r+i, the asset 148,328 bytes. This is new sky for the scan: the earlier swept 7-cone ZTF sample (2,253 objects, no i band) was not re-scanned. A fresh measurement carries its own name.
- ANTARES: the anonymous loci corpus. Per-locus alert bundles collapse after duplicate-packet removal to few distinct band epochs; the densest locus, ANT2020bf6im, holds 486 alerts that reduce to 210 genuine g+r visits over roughly 6.4 a, of which only 2 lie within 1800 s. The N_MIN-24/N_COINC-12 floor stays honestly closed on such bundles: the anonymous alert plane carries no forced-photometry surface.

## 3. The gate repair

`lomb_scargle_fap` carried the series variance in the denominator (`den = Σ x² sin² ...`), so the test statistic scaled as n/σ² and stayed bound to the amplitude scale. Quiet fractional photometry (sd 0.02) read FAP 0.00e0 on every row, including genuine 6σ-10σ achromatic dips: the aperiodic gate was shut on everything. The earlier zero candidates across the LSST routes were that closed gate, not a measurement. The negative control caught it (2026-09-05): an achromatic dip injected into a real two-band series never opened the candidate gate, naming the gap instead of a silent zero.

The fix is the standard variance-normalized periodogram: both quadrature terms, the Lomb-Scargle window offset set by tan(2ωτ) = Σ sin 2ωt / Σ cos 2ωt, amplitude-invariant. Measured on three decades of white noise (sd 0.02, 3, 30): FAP 9.52e-1 on all three scales, spread 1.6e-8 (f32 rounding). On the real cone (ra 148.87°, dec 2.52°) an injected 8σ achromatic dip reads -6.1σ in i and -5.9σ in z (depth ratio 0.93) with FAP 3.74e-2 at least 0.01, and is carried; 6σ reads FAP 1.61e-2, 10σ reads FAP 7.48e-2. A genuine 2 h periodic sinusoid carrying an 8σ achromatic dip stays excluded (FAP 3.79e-4 below 0.01): the repaired gate separates the aperiodic anomaly from the periodic natural variable.

## 4. The result

With the repaired gate the dense forced-photometry surface finally measures. On the freshly measured objects two pre-exclusion candidates appeared: achromatic dips above DIP_SIG 3σ, aperiodic under the FAP gate, above the sample floors. Both were identified as catalogued natural dimmers by the exclusion crossmatch (broker classes AGN and SN; one object matches a SIMBAD galaxy) and were excluded before the verdict. Result: 0 unexcluded candidates.

The earlier open-gate cone runs that preceded the forced-photometry surface showed the same shape (DDF cone, ra 148.87°, dec 2.52°): 26 objects, 10 excluded as periodic by the FAP gate (FAP 2.45e-9 to 7.08e-3), 9 aperiodic achromatic pre-exclusion candidates, all removed as natural dimmers, 0 unclassified post-exclusion. No candidate measured so far has survived the exclusion filter.

The quantitative limit over the fresh area: no unexplained achromatic non-periodic dip above 3σ across the objects measured. The bound is per object, not pooled: each object that did not reach the gate carries its own coverage reason, single-band sparsity in the early LSST/ZTF alert cadence or a coincident two-band visit count below N_COINC_MIN 12. Those are coverage limits. The zero applies to the objects whose cadence actually reached the gate.

## 5. The verdict

0 unexcluded candidates on the fresh-area surface (0 honored). The silence is the measured response of the field through a working gate, not a defect and not a closed door. The exclusion filter, not the gate, is what the surviving dips meet, and every survivor so far is a catalogued natural dimmer.

The claim is bounded: a quantitative limit over the freshly measured objects, not a galaxy census. The surfaces that would widen the reach are named: the credentialed alert streams (Fink Kafka, ANTARES Key+Secret by e-mail) and the Rubin science platform, none of which holds a consumer credential in `.secrets.local` today. A variable-rich positive-control cone (genuine RR Lyrae and eclipsing binaries with LSST multiband series) remains the pending next control, and the IR-excess axis at 10-60 μm of the Nadel V design is a separate measurement, untouched here.

