<!--
  title: Disequilibrium Survey of 48 JWST Exoplanet Transmission Atmospheres
  class: paper
  date: 2026-09-05
  sha256: 98f11450e9ff207012d546b62cbeaaee0c5d035f4365c014c35665cd7f994460
  status: live
  see-also: 
-->
# Disequilibrium Survey of 48 JWST Exoplanet Transmission Atmospheres

## Abstract

A life-bearing atmosphere departs from thermochemical equilibrium: it carries
species whose coexistence a dead chemistry cannot reproduce. We measure that
departure as a co-indexed fact on the species axis. For 48 curated JWST
transmission-spectroscopy targets we collect the species that the literature
reports as detected, and ask whether thermochemical equilibrium at each
planet's temperature reproduces them. The equilibrium is a gas Gibbs
minimizer over H, C, N, O and S, with condensed water below 500 K, sourced
from JANAF data. Sixteen hosts carry at least one species below the
equilibrium floor, including photochemical SO2 on WASP-39 b and WASP-107 b
and CO2 on K2-18 b, each many orders of magnitude above equilibrium. A
permutation null over the catalog gives P of 0.94: the individual
disequilibrium detections are real, but the catalog as a whole does not
exceed chance pairing of species onto planet temperature. The claim that
these atmospheres collectively break the field does not stand; the individual
photochemical detections do.

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
attributed to a primary paper. The equilibrium model is a gas Gibbs minimizer
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

## 4. The verdict

The individual disequilibrium detections are real and named. The catalog does
not exceed the null: the population of detected species follows the same
tendency the null reproduces, sulfur-rich species landing on hot planets. Life
as a filter that breaks the whole field is not carried by this catalog; the
photochemical excess on specific planets is.

