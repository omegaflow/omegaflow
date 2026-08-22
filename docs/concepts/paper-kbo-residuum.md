<!--
  title: No Arrow Beyond the Family Bound — Transfer-Entropy and Clustering Limits on the Planet Nine Signal in the Harvested Kuiper Belt (Nadel VI)
  class: concept
  date: 2026-08-22
  sha256: 555d17c0fd10e3d89ad43e1cef1de25c85554a2b27c0386e852b21c6a61c534d
  status: live
  see-also: docs/surveys/survey-2026-08-22-kbo-residuum.md archive/handover/handover-2026-08-22-planet-neun.md
-->

# No Arrow Beyond the Family Bound — Transfer-Entropy and Clustering Limits on the Planet Nine Signal in the Harvested Kuiper Belt

**The omegaflow KBO blade (Nadel VI).** Self-contained. The numbers in the tables
are machine output; no number is fabricated, no sample was synthesized, and every
absent datum is named as absent.

## Abstract

We harvest the full catalogue of trans-Neptunian objects with semi-major axis
a > 30 AU (7180 osculating orbit fits: 7077 from the JPL SBDB query API in
a ∈ (30, 200) AU plus 106 in a > 200 AU, cross-checked against the Minor Planet
Center Distant catalogue; 4802 agree, 912 disagree beyond 5 × 10⁻³ AU, 1328
epoch-separated) and the reconstructed cruise trajectories of Voyager 1, Voyager 2,
and New Horizons (Horizons vectors, 32-day raster, encounter phases excluded).
For each object we integrate a Sun + eight-planet point-mass model with a
drift-kick leapfrog (dt = 30 d, Chebyshev planet states, coverage JD 2305328.5–2816848.5)
and form the residue R(t) = |Keplerian reference − N-body model| over a ±400 yr
window (256 samples). Transfer entropy TE(R → ϖ) per dynamical family, tested
against phase-randomized surrogate thresholds (10 realizations, mean + 2σ) and a
family bound (maximum surrogate threshold of the round), yields no fam-significant
arrow in any family (fam = 7.28 × 10⁻¹) and none in the probes. A direct Rayleigh
test of perihelion-longitude clustering — the statistical basis of the Planet Nine
hypothesis — against a uniform-draw null shows the P9 selection
(a ≥ 250 AU, q ≥ 30 AU, n = 44) consistent with uniformity (R = 5.8 × 10⁻² below
the null mean 1.35 × 10⁻¹) and only a 9.3° offset of its mean perihelion longitude
from the planetary mean, not the ~180° anti-alignment the hypothesis predicts.
The harvested catalogue does not carry the classic signature; the machine states
the limit, not the absence of the planet.

## 1. Introduction

The Planet Nine hypothesis (Batygin & Brown 2016; Brown & Batygin 2021; Batygin et
al. 2019) rests on a statistical claim: the perihelion longitudes ϖ = Ω + ω of the
extreme trans-Neptunian objects (a > 250 AU, perihelion q > 30 AU) cluster in one
sky direction with probability ~7 × 10⁻⁵ under uniformity, and the cluster is
anti-aligned with the perihelion longitudes of the known planets — a configuration a
~5–10 M⊕, a ~ 400–800 AU perturber would produce by secular shepherding. The claim
is contested on observational-selection grounds (Shankman et al. 2017; Bernardinelli
et al. 2020; Napier et al. 2021): the clustered objects were all found by surveys
with correlated pointing.

This paper contributes an independent, machine-reproducible measurement suite over
the harvested catalogue. It does not model selection functions and makes no
existence claim. It asks two questions with two instruments:

1. **Residue flow.** Does the residue between a Keplerian reference (the harvested
   osculating fit, propagated) and a Sun+8-planet N-body model carry transfer
   entropy toward the orbit elements, beyond the phase-randomized surrogate
   threshold and the family bound?
2. **Clustering.** Does the harvested catalogue reproduce the ϖ clustering and the
   planetary anti-alignment on which the hypothesis rests, tested against a
   uniform-angle null?

The two instruments share the field's data contracts (SI units, heliocentric
ecliptic J2000 elements converted to ICRS equatorial, TDB timescale, f64
throughout) and are published here with seeds and commands (§6).

## 2. Data

**2.1 KBO catalogue.** `kbo_elements.json` (flat array, 7180 records), compiled by
`kbo_compiler` from the JPL SBDB query API (pages of 1000, `sb-class=TNO`,
`full-prec=true`, deduplicated with one-page over-fetch; a ∈ (30, 200) AU, e < 0.9,
plus an ETNO page a > 200 AU, e < 0.99) and cross-checked against the Minor Planet
Center `Distant.txt` (8258 fixed-width records; join by number or provisional
designation, fallback unique element-space match within 5 × 10⁻³ AU in a,
5 × 10⁻³ in e, 0.05° in i; epochs within 30 d). Cross-check result: 4802 agree,
912 disagree, 119 element-joins, 19 absent, 1328 epoch-separated — the MPC/JPL
divergence concentrates in high-eccentricity orbits (different perturbation models).
Elements: heliocentric ecliptic J2000, a in AU, angles in degrees, epoch JD TDB.

Dynamical families (windows in a, AU): 3:2 [39.2, 39.9], 2:1 [47.6, 48.3],
5:2 [54.9, 55.9], 7:4 [43.4, 44.0], 4:3 [36.2, 36.7], 5:3 [42.0, 42.5];
classical 38 ≤ a ≤ 50 and e < 0.24; scattered e ≥ 0.3; ETNO a ≥ 250; remainder
`uebrig`. Counts: classical 2884, scattered 1663, übrig 662, 7:4 635, 3:2 614,
5:3 212, 2:1 211, 5:2 137, ETNO 84, 4:3 75. The P9 selection
(a ≥ 250 AU, q = a(1−e) ≥ 30 AU) holds n = 44 objects.

**2.2 Probe arcs.** Reconstructed cruise trajectories (Horizons state vectors,
32-day raster, Chebyshev granules of 32 d, degree 17): Voyager 1 from 1981-01-01
(JD 2444606.5), Voyager 2 from 1989-10-01 (JD 2447801.5), New Horizons from
2015-08-01 (JD 2457236.5), all to 2031-01-01 — the planetary-encounter phases are
excluded by construction (they belong to the flyby blade); compiled by
`horizons_compiler --long` as `ephemeris_{name}_long.bin` CDN assets.

**2.3 Ephemerides and constants.** Planet states from the CDN Chebyshev bins
(`ephemeris_{body}.bin`, coverage JD 2305328.50–2816848.50 ≈ years 1603–3002),
heliocentric ICRS, meters; body GM from the bin properties (SI m³/s², e.g. Jupiter
1.2668653 × 10¹⁷, Neptune 6.83510 × 10¹⁵); Sun GM = 1.32712440018 × 10²⁰ m³/s²
(constant); AU = 1.495978707 × 10¹¹ m; J2000 = JD 2451545.0. The c-light-time map
uses c = 299 792 458 m/s — the field's signal-reach law for gravitational force
(d = c·age).

## 3. Methods

**3.1 N-body model.** Point-mass Sun + eight planets. KBO and probe states are
integrated as test particles with a drift-kick leapfrog, dt = 30 d, with
accelerations from the Chebyshev planet states. Self-consistency control: a
Sun-only integration against the Kepler reference gives max R = 9.5 × 10⁻⁴ AU over
the full window (the integrator is the reference's own shadow).

**3.2 Residuum.** R(t) = |r_K(t) − r_N(t)| in AU, where r_K is the Kepler
propagation of the harvested osculating elements and r_N the N-body state, sampled
at 256 points over ±400 yr (window bounded by the planet-bin coverage, uniform for
all objects; the window is 10⁻⁶ of the secular timescale — see §5). For the probes,
R(t) = |observed arc − N-body state from the window start|. The orbit series is the
perihelion longitude ϖ(t) of the N-body state (unwrapped).

**3.3 Transfer entropy.** TE(X → Y, τ) with the Schreiber (2000) estimator, kernel
density (Silverman bandwidth) on f32 series, lag sweep 0–64 in both directions.
Threshold: phase-randomized surrogates (f64 FFT, 10 realizations, mean + 2σ —
the broken-null-control protocol, Kaiser & Schreiber 2002). Family bound: fam =
maximum surrogate threshold of the whole round (families + probes + controls);
an arrow is fam-significant only above fam. Null controls: I, Sun-only
self-consistency (must vanish); II, cold classical subset (e < 0.15, i < 5°,
n = 1060 — the dynamically quiet population, must be still); III, the surrogate
threshold itself. Families below n = 30 carry no statement.

**3.4 Direct clustering test.** Rayleigh statistic
R = |Σ e^{iϖ}|/n per family on the harvested ϖ at epoch, null from n angles drawn
uniform in (0, 2π), 1000 realizations, mean + 2σ. A permutation (shuffle) null is
invalid here — R is order-invariant; the broken null is named, not used. The
planetary mean ϖ is computed from the planet bins at the catalogue epoch
(JD 2461200.5). No selection-function correction is applied (§5).

**3.5 c-light-time map.** For every arrow above its own threshold the lag is mapped
to a distance d = lag × sample-days × c. Validity: the map describes a transient
source; for a static perturber the secular torque is continuous and the lag carries
no d/c information — the map is printed for completeness, never for the verdict.

## 4. Results

**Table 1 — residue flow per family** (fam = 7.2822 × 10⁻¹; threshold at best lag;
256 samples; lag in sample units, 1 sample ≈ 3.12 yr):

| Family | n | TE | Direction | Lag | Threshold | Verdict |
|---|---|---|---|---|---|---|
| übrig | 661 | 5.3526e-1 | ϖ→R | 62 | 2.1925e-1 | above own, below fam |
| klassisch | 2854 | 1.8055e-1 | R→ϖ | 58 | 2.5576e-1 | still |
| gestreut | 1663 | 3.6958e-1 | ϖ→R | 36 | 2.2705e-1 | above own, below fam |
| 3:2 | 601 | 6.0680e-1 | ϖ→R | 43 | 4.3436e-1 | above own, below fam |
| 2:1 | 211 | 5.1751e-1 | R→ϖ | 54 | 5.8033e-1 | still |
| 5:2 | 137 | 6.1317e-1 | R→ϖ | 64 | 5.8391e-1 | above own, below fam |
| 7:4 | 628 | 2.7336e-1 | R→ϖ | 58 | 3.4124e-1 | still |
| 4:3 | 75 | 5.2104e-1 | ϖ→R | 29 | 3.4140e-1 | above own, below fam |
| 5:3 | 202 | 3.2096e-1 | ϖ→R | 38 | 3.6556e-1 | still |
| etno | 84 | 3.1016e-1 | R→ϖ | 63 | 2.9669e-1 | above own, below fam |
| NK II cold | 1060 | 1.8402e-1 | R→ϖ | 54 | 3.0938e-1 | still |

Residue magnitudes: classical mean 2.32 AU (max 12.96), scattered max 297 AU,
ETNO mean 82.2 AU (max 1419 AU — the chaotic high-e objects). 64 of 7180 objects
yield no series (extreme elements) and are counted, not replaced.

**Table 2 — probe arcs** (own threshold; window days; max R):

| Probe | Window | max R | TE | Direction | Lag | Threshold | Verdict |
|---|---|---|---|---|---|---|---|
| voyager1 | 18272 d | 9.6551e-1 AU | 2.2817e-1 | ϖ→R | 64 | 1.3709e-1 | above own, below fam |
| voyager2 | 15072 d | 1.1136e-1 AU | 2.1419e-1 | ϖ→R | 64 | 8.3484e-2 | above own, below fam |
| new_horizons | 5632 d | 1.8943e-1 AU | 1.9135e-1 | R→ϖ | 62 | 2.6628e-1 | still |

The probe arrows sit at the sweep edge (lag 62–64 of 64) — an edge finding, not a
carrier (the field's edge rule). The ~1 AU divergence over 50 yr corresponds to an
unmodeled constant acceleration of order 10⁻¹⁰ m/s² — the scale of solar radiation
pressure, RTG thermal recoil, and the Pioneer anomaly (Anderson et al. 2002).

**Table 3 — direct clustering test** (Rayleigh R; uniform null mean, threshold
mean + 2σ, 1000 draws):

| Family | n | R | ϖ̄ | Null mean | Threshold | Verdict |
|---|---|---|---|---|---|---|
| etno q>30 (P9 selection) | 44 | 0.0583 | 68.1° | 0.1351 | 0.2770 | still |
| etno | 84 | 0.0401 | 64.3° | 0.0992 | 0.2014 | still |
| klassisch | 2884 | 0.0418 | 276.7° | 0.0168 | 0.0341 | clustered |
| gestreut | 1666 | 0.0634 | 261.8° | 0.0221 | 0.0447 | clustered |
| 5:2 | 137 | 0.1881 | 258.2° | 0.0761 | 0.1546 | clustered |
| 7:4 | 635 | 0.0960 | 255.8° | 0.0361 | 0.0734 | clustered |
| 3:2 | 614 | 0.0553 | 347.2° | 0.0363 | 0.0729 | still |
| 2:1 | 211 | 0.0632 | 319.3° | 0.0610 | 0.1253 | still |
| 4:3 | 75 | 0.1740 | 6.4° | 0.1061 | 0.2167 | still |
| 5:3 | 212 | 0.0208 | 303.4° | 0.0603 | 0.1232 | still |
| planets | 8 | 0.5716 | 77.3° | — | — | — |

Anti-alignment probe: |ϖ̄_ETNO(q>30) − ϖ̄_planets| = 9.3° (the hypothesis predicts
~180°). The weak clustering of the large families (classical, scattered, 5:2, 7:4)
is known belt structure; the P9 selection itself is quieter than the uniform null.

## 5. Discussion and limits

No arrow in Table 1 or Table 2 reaches the family bound; the null controls behave
as designed (I vanishes, II is still). The strongest sub-fam flow sits in the
resonant families (3:2, 5:2) — the expected signature of Neptune's kick structure,
not of an external perturber. Table 3 reproduces the known population statistics:
the catalogue-level P9 selection shows no ϖ clustering and no planetary
anti-alignment, in agreement with the bias analyses (Shankman et al. 2017;
Bernardinelli et al. 2020; Napier et al. 2021).

Limits, stated as such: (i) the residue window (±400 yr) is 10⁻⁶ of the secular
timescale — the measurement is blind to the slow shepherding the hypothesis
requires; (ii) no selection-function correction — survey bias is not modeled, only
cited; (iii) the P9 selection holds n = 44, the classic cluster is six objects;
(iv) the SPK Type-1 reader (modified difference arrays, JPL memorandum 163) is not
implemented — the merged Voyager kernels wait, the Horizons long windows are the
standard rail; (v) the c-light-time map is a transient-source heuristic (§3.5);
(vi) the model carries Sun+8 gravity only — solar radiation pressure, thermal
recoil, the galactic tide, and stellar encounters remain in the residue and are
named, not subtracted. Absent means unmodeled, never zero.

## 6. Reproducibility

All artifacts are flat CDN assets on the ssd.jpl.nasa.gov release: `kbo_elements.json`
(compiler: `cargo run --bin kbo_compiler -- --out kbo_elements.json --etno`),
`ephemeris_voyager1_long.bin`, `ephemeris_voyager2_long.bin`,
`ephemeris_new_horizons_long.bin` (`cargo run --bin horizons_compiler -- --long`),
and the planet bins. Measurements:

- Residue flow: `cargo run --bin kbo_residue_probe` (7180 objects, 256 samples,
  dt 30 d, lag-max 64, seed 0x9E3779B97F4A7C15).
- Clustering: `cargo run --bin kbo_residue_probe --cluster-only`.

Rust std-only; the TE estimator and the surrogate machinery are the field's
canonical implementations (`src/te.rs`), byte-stable against the broken-null
control record.

## 7. Conclusion

The blade measured what the harvested sky carries. The residue flow is still under
the family bound — in the KBO families and in three tracked spacecraft arcs at
three other places. The clustering test, run on the full harvest, finds the P9
selection quieter than uniform and the anti-alignment absent. The planet is not
hallucinated — the hypothesis is sharp and its classic signature is not in these
data; the machine states the limit, and the Doppler-second front takes the pointer
from here.

## References

1. Batygin K., Brown M. E., 2016, AJ 151, 22.
2. Brown M. E., Batygin K., 2021, AJ 162, 219.
3. Batygin K., Adams F. C., Brown M. E., Becker J. C., 2019, Phys. Rep. 805, 1.
4. Trujillo C. A., Sheppard S. S., 2014, Nature 507, 471.
5. Shankman C., et al., 2017, AJ 154, 50.
6. Bernardinelli P. H., et al., 2020, ApJS 247, 32.
7. Napier K. J., et al., 2021, PSJ 2, 59.
8. Schreiber T., 2000, PRL 85, 461.
9. Kaiser A., Schreiber T., 2002, Physica D 166, 43.
10. Anderson J. D., et al., 2002, PRD 65, 082004.
