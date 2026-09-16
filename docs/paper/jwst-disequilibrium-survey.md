<!--
  title: Disequilibrium Survey of 48 JWST Exoplanet Transmission Atmospheres
  class: paper
  date: 2026-09-11
  sha256: 33fd7978705887748e16e231b4a9d9bdf9365f56ef5a8bd53cfb48f38150737d
  status: live
  see-also: 
-->
# Disequilibrium Survey of 48 JWST Exoplanet Transmission Atmospheres

## Abstract

A life-bearing atmosphere departs from thermochemical equilibrium: it carries
species whose coexistence a dead chemistry cannot reproduce. We measure that
departure as a co-indexed fact. For 48 curated JWST
transmission-spectroscopy targets we collect the species that the literature
reports as detected, and ask whether thermochemical equilibrium at each
planet's temperature reproduces them. The equilibrium is a gas Gibbs
minimizer over H, C, N, O and S, with condensed water below 500 K, sourced
from JANAF data. Sixteen hosts carry at least one species below the
equilibrium floor, including photochemical SO2 on WASP-39 b and WASP-107 b
and CO2 on K2-18 b, each many orders of magnitude above equilibrium. A
permutation null over the catalog gives P of 0.94: the individual
disequilibrium detections are real, but the catalog does not
exceed chance pairing of species onto planet temperature. The claim that
these atmospheres collectively break the field does not stand; the individual
photochemical detections do. A second cleaning step reads the reservoir
metallicity [Fe/H] and the stellar-activity witnesses: [Fe/H] carries no hit
signal (r = +0.051, P = 0.79), and log R'HK trends against the hits (r =
-0.609, P = 0.07) rather than re-explaining them.

## 1. The measurement

We hold each detected species against the equilibrium composition at the
planet's temperature, at solar element abundances and 1 bar. A species whose
equilibrium mixing ratio is below a named floor, while the literature reports
it detected, is a disequilibrium hit. The signal is not a transfer-entropy
number; it is the coexistence of a species with the temperature at which the
dead chemistry cannot make it.

## 2. Data

The target set is the 48 hosts of the NExScI spectra table that carry a JWST
transmission spectrum in NIRSpec, NIRISS or MIRI, 229 spectrum rows. The
detection registry holds 73 published species detections over 30 hosts, each
attributed to a primary paper. Host effective temperatures, radii, orbital
semimajor axes, metallicities and rotation periods are read from the NExScI
pscomppars table (3481 transiting-planet hosts); [Fe/H] = st_met is present
for all 30 detection hosts. High-resolution [C/H], [O/H] and [N/H] abundances
and the stellar-activity numbers (log R'HK, S-index, L_X/F_X, P_rot) come from
a separate witness register. The equilibrium model is a gas Gibbs minimizer
over the 16 archival H, C, N, O species, extended by 8 sulfur carriers and, below
500 K, by condensed water, all free energies from JANAF Shomate fits. The floor
defaults to 1e-6; sensitivity is measured over 1e-7, 1e-6, 1e-5 and 1e-4.

## 3. Results

Sixteen of 29 evaluable hosts carry a disequilibrium hit, 13 are
equilibrium-present, 1 has no model data, and 0 remain pending. The sulfur
channel is the main carrier. SO2 on WASP-39 b sits at an equilibrium mixing
ratio of 1.6e-16 and on WASP-107 b at 4.9e-22, each detected and therefore many
orders of magnitude from equilibrium. CO2 on K2-18 b is detected near 1 percent
while equilibrium predicts 1e-30, a photochemical excess. Not every detection
is a hit: H2S on TOI-5205 b is equilibrium-present at 3.24e-5, the sulfur
reservoir the chemistry predicts. The permutation null over 10000 draws pairs
species onto planet temperature; the observed 16 hit hosts meet a null mean of
18.30 with a threshold of 21.89 at mean plus 2 sigma, giving P of 0.9424.

## 4. The second cleaning step

The individual hits are tested against the reservoir and activity witnesses.
The reservoir witness is the stellar metallicity [Fe/H] = st_met from
pscomppars, read per host and applied to the metal elements C, O, N, S by the
factor z = 10^[Fe/H] (H fixed). Against the hit indicator over the 29
classifiable hosts it carries no signal: Pearson r = +0.051, permutation P =
0.7921; the 16 hit hosts have mean [Fe/H] +0.003, the 13 equilibrium-present
hosts -0.014. The reservoir scaling moves one species on one host: CO2 on
WASP-166 b crosses the floor from disequilibrium to equilibrium-present at
[Fe/H] +0.19 (reservoir fraction 1.006e-6). The measured [C/H]/[O/H]/[N/H]
witness (12 hosts, 22 analysis rows) moves two species on two hosts, both CO2:
TrES-4 b under the 2018 O-rich budget (fraction 1.089e-6) and WASP-166 b under
the Polanski 2022 budget (fraction 1.338e-6). The activity witness over the 10
hosts with a numeric log R'HK gives Pearson r = -0.609, permutation P = 0.0724:
the hit hosts sit on the quieter stars (mean log R'HK -4.90) than the
equilibrium-present hosts (-4.44). The log10 L_X channel over 9 hosts gives r =
+0.066, P = 0.8731 and re-explains no hit. The activity verdicts leave the
equilibrium classification unchanged: thermochemical equilibrium has no
activity channel, and the XUV photochemistry re-explanation of the SO2/CO2 hits
stays a non-equilibrium model (pending).

## 5. The verdict

The individual disequilibrium detections are real and named. The catalog does
not exceed the null: the population of detected species follows the same
tendency the null reproduces, sulfur-rich species landing on hot planets. Life
as a filter that breaks the whole field is not carried by this catalog; the
photochemical excess on specific planets is. The reservoir and activity
cleaning steps remove none of the 16 hits and re-explain none; they narrow the
claim to the species themselves.

## 6. Pending channels

The disequilibrium channel is the only bio-signature channel built. The O2 and
O3 abundance channel (which spectral features, which data), the vegetation
red-edge channel, and the seasonal/time-series channel are named pending
branches: no data are carried for them, and their absence is unmeasured, not
zero. A first sweep for the O2/O3 branch (ADS and arXiv, 2026-09-16, "JWST O2 O3
transmission spectroscopy exoplanet atmosphere") returns no measured JWST O2/O3
transmission detection in the searched record — only detectability and retrieval
studies (TRAPPIST-1 e O3 under stellar-UV uncertainty, LHS 1140 b, the white-dwarf
opportunity, VIRA retrievals). The branch's absence is now measured: the searched
record carries no detection and no spectrum; the channel stays pending until a
detection and its spectrum enter the registry. A second sweep for the red-edge
and seasonal branches (ADS, arXiv, Crossref, OpenAlex, Zenodo, DataCite, Brave,
2026-09-16) returns no JWST transmission or reflection spectrum carrying a
detected vegetation red edge, and no measured seasonal O2/O3 biosignature
variability from a JWST exoplanet spectrum. The record carries detectability and
retrieval studies only: the Bayesian red-edge detectability of "In Search of the Edge"
(10.3847/1538-4357/acaf59, 2023), "Retrieving the Red Edge … with Heterogeneous
Clouds" (10.3847/1538-4357/ae563d, 2026), the diurnal-variability detectability
study (10.3847/1538-4357/abd6ff, 2021), and the mid-infrared prospects for
spatial and temporal chemistry variations (10.1051/0004-6361/202557807, 2026,
arXiv 2512.16619; ETH deposit 10.3929/ethz-c-000797709, Bern BORIS
10.48620/96177). The seasonality measurements that exist are of Earth as an
exoplanet (10.3847/1538-4357/acbe3c, DSCOVR/MODIS) — with one measured
seasonal JWST counterpart: a NIRSpec/G395H time
series of the eccentric hot Jupiter HD 80606b (arXiv 2407.12456) measures
irradiation-driven seasonal change in CO, CH4 and H2O across the orbit, a
hot-Jupiter thermal/chemical response, not a biosignature. The two branches'
absence is now measured like the O2/O3 branch: no
detection and no spectrum; the channels stay pending. The XUV photochemistry
re-explanation of the SO2/CO2 hits stays pending as a non-equilibrium model.
