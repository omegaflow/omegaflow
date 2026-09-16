<!--
  title: The echo depth — pP/sP depth phases across a 16-event fleet
  class: paper
  date: 2026-09-12
  sha256: 360e240c483089c8e7d47e0e21c7a84b3655d6ba31878c0cf7c3cd16366891ee
  status: live
  see-also: docs/concepts/die-akteure-im-boden-und-wasser.md
-->
# The echo depth — pP/sP depth phases across a 16-event fleet

## Abstract

Free-surface depth phases pP and sP lag the direct P by a time that ak135 maps to source depth. Across a 16-event fleet in the Hindu Kush box (M6.2–7.5, catalog depth 107.7–231.0 km), the per-event median depths are unbiased: mean offset +1.7 km, standard error 4.7 km, against the ±10-km match gate. The scatter is the finding: 19 km across events and 36 km across stations dominate the gate, so the fleet is a measured series, not a tight depth estimator. The field pilot us10003re5 (M7.5 Hindu Kush) reads median 250 km against catalog 231 km, +19 km, outside the gate. The positive control locates M7.8 Indonesia at 14.6 km offset with rms 1.887 s, ak135 P+S verified against TauP to < 0.15 s. The free-surface reflection R_pp is negative across the pilot band (zero crossing 53.9°) and R_sp ≈ −1. The mww/NDK source-term comparison is measured (depth-identical; polarity 5/4 vs 4/5); the full CMT radiation term is now fetched (GCMT NDK parser, 70039 centroids; the pilot's per-station pP sign gate reads 2 agree / 1 oppose / 9 pending, `cmt-ndk-fleet`, 2026-09-16) and the calibration gate is registered (six pilot azimuths, 2026-09-16); the ak135 depth-model extension above 250 km is closed (2026-09-09: inversion to 700 km).

## The measurement

The free surface reflects the upgoing P and S radiation of a deep source. The reflected phases pP (up as P, down as P) and sP (up as S, down as P) arrive after the direct P by a lag that ak135 maps to source depth. At a fixed distance the lag is nearly linear in depth; the depth phase is a pure depth measure, independent of the epicentral location.

The model: ak135 P and S travel times in `src/archivar/ak135.rs`, read from `src/archivar/kernels/ak135.dat` (sha256 7518894268980b2591d539ff442d630be3d0e5acfbec9003433e455ef65feba4). Source depth enters through reciprocity — the upward leg of pP/sP is the depth leg of the direct phase at the same ray parameter. The P+S times are verified against TauP to < 0.15 s and against the geometric chord to < 0.01 s. The free-surface reflection coefficients R_pp and R_sp follow from the ak135 surface layer (α = 5.80 km/s, β = 3.46 km/s), with energy conservation as the sign test.

The picking lib (`tools/measure/src/depthphase.rs`): bandpass 0.5–2 Hz, first break (5× noise floor) with an STA/LTA trigger as the secondary pick, an SNR gate ≥ 3 at the P onset before the pP window opens, cross-correlation of a 4-s P wavelet in the lag window [0.7·lag, 1.3·lag] from the ak135 diagonal, then a 1-km depth inversion raster. An absent sP is a skip, never 0.0.

The fleet: the selection rule is registered before the first fetch — depth ≥ 35 km, magnitude ≥ 6, land epicenter in the Hindu Kush box (34–38 N, 68–74 E), a GBCO land witness per event; station band 30–90°, up to 12 BHZ stations, SNR gate ≥ 3. Catalog: USGS FDSN event; stations: IRIS fdsnws/station; waveforms: EarthScope fdsnws/dataselect; the flat CDN asset fdsn_waveform.bin carries the waveform route. 26 deep events are registered in the box; the 16 largest are measured (M6.2–7.5, catalog depth 107.7–231.0 km).

The field pilot us10003re5 (M7.5, 36.5244 N / 70.3676 E, catalog depth 231.0 km, origin 2015-10-26T09:09:42, GBCO land 3273 m): six BHZ stations pass the SNR gate.

The positive control: the seismic location of us6000tkt2 (M7.8, Indonesia) at −8.2240/121.3800 (20-km grid) returns an offset of 14.6 km against the catalog, rms 1.887 s.

## The finding

Over the 16 events the per-event offsets (median − catalog) are [+11, −3, +19, −18, +33, −13, +5, +14, −27, +25, −14, +25, −7, −11, −25, +12] km. The fleet mean is +1.7 km with a standard error of 4.7 km — inside the ±10-km match gate. The pilot's +19 km is one draw from this distribution, not a bias. The scatter is the finding: 19 km across events and 36 km across stations both exceed the gate, and the within-event station scatter is the larger term. The bottleneck is the correlation of the picks, not the ak135 model. The fleet is a measured series; a single-event depth from this chain carries the 36-km station scatter.

## The form

Named limits and pendings:

- The source radiation term: the USGS mww centroid and the GCMT/NDK term were compared on the pilot (depth-phase-mww vs cmt-ndk-fleet, 2026-09-13). The depth inversion is source-term-independent — per-station depths are byte-identical (median 230 km, sd 35.6 km, offset −1.0 km), the term feeding only the polarity gate and the pP-window anchor (the mww centroid depth 231.0 km equals the catalog anchor). The polarity gate favors mww (5 agree / 4 oppose / 3 pending) over NDK (4 / 5 / 3), a single-station flip (CB.NJ2, mww + / NDK −). The sP sign is carried by the free surface alone. The full CMT radiation shape is now fetched (cmt-ndk-fleet, 2026-09-16): the GCMT NDK parser loads 70039 centroids and computes the upgoing P radiation R_P = r̂·M·r̂ per station. For the pilot's centroid C201510260909A (strike/dip/rake 279/21/85, M0 2.241e27 dyne-cm, Mw 7.50, 36.55 N/70.42 E, depth 209.4 km) R_P spans [−0.63, +0.84] at reference Δ 39.9° with nodal azimuths 77.8°/310.9°; the free surface is uniform (R_pp < 0 across the steep band), so the measured 4+/2− pP mix is the source radiation, not the surface. The per-station sign comparison now runs in the fleet: 2 stations agree, 1 opposes, 9 pending (pP absent or fold-skipped); the full fleet run (beyond `--max-events 1`) is the next step.
- The calibration gate (measured 2026-09-16): the field pilot's six SNR-passing stations and their great-circle azimuths from the event (36.5244 N / 70.3676 E) are registered — GE.EIL 267.3°, KO.MDUB 289.5°, IU.CHTO 117.5°, TM.CMMT 117.5°, II.PALK 159.6°, IC.XAN 82.9° (12 BHZ stations in the 30–90° band, six pass SNR ≥ 3; `depth_phase_azimuth_probe`). The fleet gate now tests each station's predicted R_P sign against the measured pP (2 agree / 1 oppose / 9 pending on the pilot).
- The ak135 depth-model extension above 250 km is closed (2026-09-09): MAX_DEPTH_KM 250 → 700, DEPTH_KM raster to 700 km, inversion to 700 km; the model file carries depth rows to 6371 km.
- The ambiguous pP branch at Δ ≈ 30° (triplication) and the coda locking of the correlation are the measured bottleneck; the Δ-gate skips branch-unstable stations by name, never feeding an ambiguous pick into the inversion.
- R_pp/R_sp: R_pp is negative across the pilot band (zero crossing 53.9°, pilot ≈ 27.5° → −0.65), R_sp ≈ −1; derived from the ak135 surface layer with energy conservation.
- Data provenance, named honestly: USGS FDSN event catalog, IRIS fdsnws/station, EarthScope fdsnws/dataselect, GEOFON fdsnws/event; the flat CDN asset fdsn_waveform.bin. The NASA ADS bibcodes are a literature crosscheck only.

## References

1. B. L. N. Kennett & E. R. Engdahl, "Traveltimes for global earthquake location and phase identification", Geophysical Journal International 105, 429–465 (1991). DOI: 10.1111/j.1365-246X.1991.tb06724.x. NASA ADS bibcode: 1991GeoJI.105..429K.
2. B. L. N. Kennett, E. R. Engdahl & R. Buland, "Constraints on seismic velocities in the Earth from traveltimes", Geophysical Journal International 122, 108–124 (1995). DOI: 10.1111/j.1365-246X.1995.tb03540.x. NASA ADS bibcode: 1995GeoJI.122..108K. The ak135 model file carries this citation.
3. D. A. Storchak, D. Di Giacomo, I. Bondár, E. R. Engdahl, J. Harris, W. H. K. Lee, A. Villaseñor & P. Bormann, "Public release of the ISC-GEM Global Instrumental Earthquake Catalogue (1900–2009)", Seismological Research Letters 84, 810–815 (2013). DOI: 10.1785/0220130034. Dataset DOI: 10.31905/D808B825. NASA ADS bibcode: 2013SeiRL..84..810S.
4. USGS Earthquake Hazards Program, ANSS Comprehensive Earthquake Catalog (ComCat). DOI: 10.5066/F7MS3QZH. No NASA ADS bibcode. Live route: earthquake.usgs.gov/fdsnws/event/1/query (`phi/sources.φ`).
5. IRIS Consortium / EarthScope, FDSN web services (fdsnws/station, fdsnws/dataselect). No DOI registered in `phi/sources.φ`. Live routes: service.iris.edu/fdsnws/station/1/query, service.earthscope.org/fdsnws/dataselect/1/query; flat CDN asset fdsn_waveform.bin.
6. GEOFON, GFZ German Research Centre for Geosciences, FDSN event service. No DOI registered in `phi/sources.φ`. Live route: geofon.gfz-potsdam.de/fdsnws/event/1/query.
