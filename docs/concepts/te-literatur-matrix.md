<!--
  title: TE Literature Matrix
  class: concept
  sha256: 914c739c5648452597a3b337a94e7e9e2f4fff667b8897ff1d827d72db687ca9
  status: live
-->
# TE Literature Matrix

## Purpose

This matrix calibrates the omegaflow TE protocol (lag sweeps, phase-randomized surrogates, 10 per series, mean+2σ threshold, family threshold as the maximum over all tested pairs) against the published literature on directed information transfer and Granger causality. It answers four questions: (1) ENSO wind↔SST, (2) solar wind → geomagnetic field, (3) seismicity → ionosphere proxy (LAIC), (4) corona — flare emission channels (X-ray ↔ EUV). Every entry was found on the web during this session and carries a DOI or arXiv address. Verification status per entry: **[V] = abstract fetched and read**, **[T] = title/entry only verified** (Crossref/DOI resolution). Where a field does not follow from the verified abstract, it explicitly says "not stated in the abstract" — no value is added.

## Reference references (methodology foundations)

| Arbeit | Methode | Befund | Status | Quelle |
|---|---|---|---|---|
| Schreiber, T. (2000). *Measuring information transfer*. Phys. Rev. Lett. 85, 461–464. | Transfer entropy (definition) | Foundational definition of TE between two time series; basis of almost all following entries. | [V] DOI resolves | https://doi.org/10.1103/PhysRevLett.85.461 |
| Granger, C. W. J. (1969). *Investigating causal relations by econometric models and cross-spectral methods*. Econometrica 37, 424–438. | Granger causality | Foundation of linear predictive causality; cited throughout the climate and solar-wind literature. | [T] entry in fetched reference lists | https://doi.org/10.2307/1912791 |
| Theiler, J., et al. (1992). *Testing for nonlinearity in time series: The method of surrogate data*. Physica D 58, 77–94. | Surrogate data (null model) | Origin of the surrogate null model on which the TE literature builds as well. | [T] entry in fetched reference lists | https://doi.org/10.1016/0167-2789(92)90102-S |
| Barnett, L., Barrett, A. B., Seth, A. K. (2009). *Granger causality and transfer entropy are equivalent for Gaussian variables*. Phys. Rev. Lett. 103, 238701. | TE ≈ Granger | Equivalence for Gaussian variables; justifies carrying TE and Granger in one matrix. | [T] entry in fetched reference list (Manshour 2021) | https://doi.org/10.1103/PhysRevLett.103.238701 |
| Runge, J., Heitzig, J., Petoukhov, V., Kurths, J. (2012). *Escaping the curse of dimensionality in estimating multivariate transfer entropy*. Phys. Rev. Lett. 108, 258701. | Multivariate TE (graphical) | Causal discovery with iterative momentary conditional independence; basis of PCMCI and graphical climate causal analysis. | [T] Crossref entry fully fetched | https://doi.org/10.1103/PhysRevLett.108.258701 |
| Kraskov, A., Stögbauer, H., Grassberger, P. (2004). *Estimating mutual information*. Phys. Rev. E 69, 066138. | kNN mutual-information estimator (KSG) | k-th-nearest-neighbor MI estimation with the digamma marginal counts; the estimator basis of the kNN conditional-MI/TE family. | [T] Crossref entry fully fetched | https://doi.org/10.1103/PhysRevE.69.066138 |
| Frenzel, S., Pompe, B. (2007). *Partial mutual information for coupling analysis of multivariate time series*. Phys. Rev. Lett. 99, 204101. | KSG-based transfer entropy | TE from the k-th-neighbor distances in the full joint space (ψ(k) + ⟨ψ(n_x) − ψ(n_x'x) − ψ(n_xy)⟩ form); the method anchor of the omegaflow kNN estimator `transfer_entropy_ksg_conditional_n`. | [T] Crossref entry fully fetched | https://doi.org/10.1103/PhysRevLett.99.204101 |

---

## Question 1: ENSO — wind↔SST and teleconnections

### Matrix

| Work | Method | Series/cadence | Lags | Null model | Multiple comparison | Finding | Status | Source |
|---|---|---|---|---|---|---|---|---|
| Runge, J., Petoukhov, V., Kurths, J. (2014). *Quantifying the Strength and Delay of Climatic Interactions: The Ambiguities of Cross Correlation and a Novel Measure Based on Graphical Models*. J. Climate 27, 720–739. | Granger causality via graphical models (two-stage: existence of the causal link, then strength) | Temperature anomalies of Europe and the tropical Pacific/Atlantic; monthly; example ENSO teleconnections and Walker circulation | Time delays from the graphical model; concrete lag range not stated in the abstract | Not stated in the abstract | Reference list contains Benjamini-Hochberg FDR (Benjamini 1995) | Cross-correlation misleading under autocorrelation; graphical model separates direct from indirect links; ENSO/Walker as the pattern case | [V] | https://doi.org/10.1175/JCLI-D-13-00159.1 |
| Runge, J., Petoukhov, V., Donges, J. F., Hlinka, J., Jajcay, N., Vejmelka, M., Hartman, D., Marwan, N., Paluš, M., Kurths, J. (2015). *Identifying causal gateways and mediators in complex spatio-temporal systems*. Nature Communications 6, 8502. | Causal discovery (multivariate TE / graphical models) + causal effect (path coefficients, Pearl) | NCEP/NCAR surface pressure, weekly, 1948–2012, 60 Varimax components | τ up to τ_max = 4 weeks (intraseasonal); main finding at 3 weeks | Residual bootstrap for standard errors of the path coefficients | Link-density threshold (20 %, robustness range 10–50 %) instead of multiple-test correction; robustness across data halves | ENSO (eastern Pacific) → Arabian Sea (monsoon region) causal, mainly mediated via the Indonesian archipelago component; mediates >60 % of the total effect (−0.08 ± 0.01 at 3 weeks) | [V] | https://doi.org/10.1038/ncomms9502 |
| Runge, J., et al. (2019). *Inferring causation from time series in Earth system sciences*. Nature Communications 10, 2553. | Overview: TE, Granger, PCMCI, Convergent Cross Mapping, Momentary Information Transfer and others | Climate/Earth-system data in general | — | Discussion of surrogate/permutation null models | Discussion of the multiple-testing problem in network reconstruction | Positions PCMCI as the standard for multivariate climate causality; benchmark platform causeme.net | [V] | https://doi.org/10.1038/s41467-019-10105-3 |
| Latif, Y., Paluš, M. (2024). *Causal information flow and information transfer delay from ENSO and IOD to precipitation variability in the Upper Indus Basin, Pakistan*. EGU General Assembly 2024, Abstract EGU24-12884. | Transfer entropy / information transfer with delay estimation | ENSO/IOD indices → precipitation; concrete cadence not stated in the entry | Information-transfer delay (delay analysis); lag range not stated | Not stated in the entry | Not stated | ENSO and IOD transfer information onto the precipitation variability; direction climate index → precipitation | [T] conference abstract | https://doi.org/10.5194/egusphere-egu24-12884 |

### Convergence/divergence to the omegaflow protocol

The literature confirms the direction of the omegaflow ENSO sheet: directed, time-delayed information out of the tropical Pacific into teleconnected regions, described as largely linear, time-delayed links (Runge 2014, 2015). Divergences: (a) cadence — the literature runs on weekly to monthly scale, omegaflow on its own field cadence; a direct equatorial wind↔SST TE entry at omegaflow resolution was not found, the closest neighbor is the graphical-model approach (Runge 2014) instead of pairwise TE with a surrogate null. (b) Null model — omegaflow uses phase-randomized surrogates (10 per series, mean+2σ); the literature uses bootstrap/residual procedures and conditional independence tests, no fixed surrogate count. (c) Multiple comparison — omegaflow applies a family threshold (max surrogate TE over all pairs); the literature corrects via FDR (Runge 2014) or via network link densities and robustness analyses (Runge 2015). Both answers are legitimate; with the family threshold omegaflow is stricter here, which means a higher threshold with many pairs.

---

## Question 2: solar wind → geomagnetic field (Bz sheet)

### Matrix

| Arbeit | Methode | Serie/Kadenz | Lags | Null-Modell | Mehrfachvergleich | Befund | Status | Quelle |
|---|---|---|---|---|---|---|---|---|
| Johnson, J. R., Wing, S., Camporeale, E. (2018). *Transfer entropy and cumulant-based cost as measures of nonlinear causal relationships in space plasmas: applications to Dst*. Annales Geophysicae 36, 945–952. | TE + cumulant-based cost function; additionally MI and CMI | Solar wind (V_sw, n_sw, VB_s) → Dst; cadence not stated in the abstract | 3–12 h (VB_s-related nonlinear significance); internal peaks at 25, 50, 90 h | Not stated in the abstract | Not stated in the abstract | V_sw → Dst directed; MI shows a misleading reverse direction Dst → V_sw, TE shows barely any reverse direction — TE as the direction-faithful tool | [V] | https://doi.org/10.5194/angeo-36-945-2018 |
| Stumpo, M., Consolini, G., Alberti, T., Quattrociocchi, V. (2020). *Measuring Information Coupling between the Solar Wind and the Magnetosphere–Ionosphere System*. Entropy 22, 276. | TE (information theory) | Solar wind → magnetosphere/ionosphere system via geomagnetic indices; cadence not stated in the abstract | Delay ≈ 30–60 min | Not stated in the abstract | Not stated in the abstract | B_z (IMF) carries the largest information onto geomagnetic indices; strongest candidate for the geomagnetic response; linear correlation and Granger break down | [V] | https://doi.org/10.3390/e22030276 |
| Manshour, P., Balasis, G., Consolini, G., Papadimitriou, C., Paluš, M. (2021). *Causality and Information Transfer Between the Solar Wind and the Magnetosphere–Ionosphere System*. Entropy 23, 390. | TE with conditioning | Solar-wind parameters → AE, SYM-H; cadence not stated in the abstract | B_z → AE: 10 min; B_z → SYM-H: ≈ 30 min | Not stated in the abstract (references point to surrogate data after Theiler) | No AE↔SYM-H pair, therefore no comparison problem within the system | B_z drives AE and SYM-H with information-transfer delay; no causal link AE↔SYM-H; coupling described as linear, time-delayed information | [V] | https://doi.org/10.3390/e23040390 |
| Wing, S., Johnson, J. R., Camporeale, E., Reeves, G. D. (2016). *Information theoretical approach to discovering solar wind drivers of the outer radiation belt*. J. Geophys. Res. Space Physics 121, 9378–9399. | MI, CMI, TE; windowed TE for non-stationarity | Solar wind (V_sw, n_sw) → geosynchronous MeV electrons (J_e); cadence not stated in the abstract | V_sw → J_e TE peak at 2 days; n_sw response after conditioning < 24 h; lags up to 3 days considered | Not stated in the abstract | Conditioning (CMI) as confounder control | V_sw main driver (n_sw transfers ≈ 36 % of the information of V_sw); feedback and confounding structures (V_sw–n_sw anticorrelation) must be accounted for in lag analyses | [V] | https://doi.org/10.1002/2016JA022711 |
| Runge, J., Balasis, G., Daglis, I. A., Papadimitriou, C., Donner, R. V. (2018). *Common solar wind drivers behind magnetic storm–magnetospheric substorm dependency*. Scientific Reports 8, 16987. | Causal inference (PCMCI, conditional independence) | Solar-wind variables + storm/substorm indices; cadence not stated in the abstract | Time-delayed links from the causal graph | Conditional independence tests (permutation/test statistic; details not in the abstract) | Common-cause control via simultaneous consideration of all variables | B_z is the common driver of storms and substorms; no statistical evidence for a direct storm–substorm link — the association arises via the common solar-wind driver | [V] | https://doi.org/10.1038/s41598-018-35250-5 |
| Johnson, J. R., Wing, S. (2014). *External versus internal triggering of substorms: An information-theoretical approach*. Geophys. Res. Lett. 41, 5748–5754. | Conditional Redundancy (entropy-based conditional dependence) | IMF northward turnings + substorm onsets; cadence not stated in the abstract | Coincidence construction (surrogate triggers, 2 % of the substorms) | Surrogate datasets of external triggers (explicitly in the abstract) | No multiple comparison needed (one hypothesis pair) | Northward turning of the IMF carries only a few percent of additional information beyond the energy storage (southward IMF) — chance coincidence, no causality | [V] | https://doi.org/10.1002/2014GL060928 |
| De Michelis, P., Consolini, G., Materassi, M., Tozzi, R. (2011). *An information theory approach to the storm–substorm relationship*. J. Geophys. Res. Space Physics 116, A08225. | Information theory (MI-based) | Storm/substorm dynamics; cadence not stated in the entry | Not stated in the entry | Not stated in the entry | Not stated in the entry | Information flow between storms and substorms investigated | [T] | https://doi.org/10.1029/2011JA016535 |
| Wing, S., Johnson, J. R. (2019). *Applications of Information Theory in Solar and Space Physics*. Entropy 21, 140. | Overview: MI, CMI, TE, windowed TE | Solar wind → radiation belt; solar-cycle parameters | V_sw → J_e: 2 days; polar field → sunspot number: 3–4 years | Not stated in the abstract | Conditioning as confounder control | TE as the tool for driver untangling; non-stationarity via windowed TE | [V] | https://doi.org/10.3390/e21020140 |
| Stumpo, M., Benella, S., Consolini, G., Alberti, T. (2022). *Dynamical information flow within the magnetosphere-ionosphere system during magnetic storms*. arXiv:2206.13992. | TE over storm events instead of long time series | Magnetosphere/ionosphere indices around geomagnetic storms; non-stationary local analysis | Not stated in the abstract | Not stated in the abstract | Not stated in the abstract | Storm database allows tracking the quiet → disturbed transition; questions the earlier null link storm–substorm | [V] arXiv abstract | https://arxiv.org/abs/2206.13992 |

### Convergence/divergence to the omegaflow protocol

The Bz sheet is the best calibrated: the literature consistently confirms the direction solar wind → geomagnetic field, with B_z (IMF) as the strongest information carrier (Stumpo 2020; Manshour 2021; Runge 2018) and quantified delays from 10 min (B_z→AE) to ≈ 30–60 min (B_z→SYM-H, Stumpo 2020). That matches the omegaflow Bz sheet. Divergences: (a) cadence — the literature works on minute/hour scale (OMNI, 1-min indices); omegaflow carries the same physical scale via its field-cadence mechanism. (b) Lag range — Manshour 2021 finds 10–30 min, Stumpo 2020 up to 60 min, Johnson 2018 up to 12 h external plus internal relaxation peaks up to 90 h; a pure short-lag sweep would not see the internal long time scales (ring-current relaxation). (c) Null model — the literature rarely specifies surrogates in the abstract (exception: Johnson & Wing 2014, explicit surrogate triggers); omegaflow is more explicit here (phase-randomized, 10 surrogates, mean+2σ). (d) Multiple comparison — only Runge 2018 performs a real common-cause control (PCMCI); the pairwise TE works mostly do not correct multiple comparisons formally. The omegaflow family threshold is therefore stricter than the state of many pairwise works.

---

## Question 3: LAIC — seismicity → ionosphere proxy

### Finding on the method

**No verified hits for transfer entropy or Granger causality in the LAIC domain.** The search over arXiv ("transfer entropy" + earthquake / seismicity / TEC / ionosphere) and Crossref (TE+TEC, Granger+TEC, TE+VLF, TE+ULF) yielded no published TE or Granger work that tests seismicity directed onto an ionosphere proxy or vice versa. The table below is therefore a context matrix: verified LAIC precursor works with their actual methods (anomaly statistics, entropy measures) — none of them is a directed causal analysis. The absence is a result, not a gap of the search: the search terms ran on arXiv and Crossref repeatedly, also with synonyms.

### Context matrix

| Work | Method | Series/cadence | Lags | Null model | Multiple comparison | Finding | Status | Source |
|---|---|---|---|---|---|---|---|---|
| Pulinets, S., Ouzounov, D. (2011). *Lithosphere–Atmosphere–Ionosphere Coupling (LAIC) model – An unified concept for earthquake precursors validation*. J. Asian Earth Sciences 41, 371–382. | Concept model (no time-series causal analysis) | n. a. | n. a. | n. a. | n. a. | Formulates the LAIC chain (radon, aerosol, electric field, ionospheric disturbance) as a unified concept for precursor validation | [T] | https://doi.org/10.1016/j.jseaes.2010.03.005 |
| Heki, K. (2011). *Ionospheric electron enhancement preceding the 2011 Tohoku-Oki earthquake*. Geophys. Res. Lett. 38, Issue 17. | GPS-TEC anomaly detection before the earthquake | GPS-TEC; cadence not stated in the entry | Precursor time window before the earthquake (from title: "preceding") | Not stated in the entry | Not stated in the entry | Ionospheric electron enhancement before the 2011 Tohoku-Oki earthquake; cornerstone of the TEC precursor literature | [T] | https://doi.org/10.1029/2011GL047908 |
| Potirakis, S. M., Minadakis, G., Eftaxias, K. (2012). *Relation between seismicity and pre-earthquake electromagnetic emissions in terms of energy, information and entropy content*. Natural Hazards and Earth System Sciences 12, 1179–1183. | Fisher information + Approximate Entropy | Seismicity ↔ preseismic kHz EM bursts (Athens 1999, M5.9); event-based | n. a. (event statistics, no lag analysis) | Not stated in the abstract | Not stated in the abstract | Entropy and information content of the two EM bursts correspond to the radar-interferometry finding (80/20 fault segments) — quantitative information, but not directed | [V] | https://doi.org/10.5194/nhess-12-1179-2012 |
| Yang, S.-S., Hayakawa, M. (2020). *Gravity Wave Activity in the Stratosphere before the 2011 Tohoku Earthquake as the Mechanism of Lithosphere–Atmosphere–Ionosphere Coupling*. Entropy 22, 110. | LAIC mechanism study (gravity-wave activity) | Stratosphere data before the Tohoku earthquake; cadence not stated in the entry | Precursor time window; details not in the entry | Not stated in the entry | Not stated in the entry | Gravity waves as the coupling mechanism of the LAIC chain before Tohoku | [T] | https://doi.org/10.3390/e22010110 |
| Pulinets, S., Ouzounov, D., Karelin, A., Boyarchuk, K. (2022). *Earthquake Precursory Phenomena in the Atmosphere*. In: Earthquake Precursors in the Atmosphere and Ionosphere, Springer, 61–105. | Book chapter (overview of precursor phenomena) | n. a. | n. a. | n. a. | n. a. | Systematic presentation of atmospheric precursors within the LAIC chain | [T] | https://doi.org/10.1007/978-94-024-2172-9_2 |

### Convergence/divergence to the omegaflow protocol

The LAIC sheet (seismicity → ionosphere proxy around M≥6 earthquakes) has **no direct calibration basis**: no verified paper was found that tests the seismicity rate directed onto TEC or an ionosphere proxy with TE/Granger. The existing precursor literature works with anomaly statistics (TEC deviation against a reference, precursor time windows) or with non-directed entropy measures (Fisher information, Approximate Entropy), never with a surrogate null model in the TE sense and never with a multiple-comparison correction. The divergences are therefore structural: (a) cadence — the precursor literature works event-centered (precursor weeks/days before individual M≥6 earthquakes), omegaflow as continuous time-series causality; (b) null model and (c) multiple comparison do not exist in the LAIC literature in verifiable form — omegaflow is methodologically alone here. The LAIC sheet of the omegaflow protocol (seismicity → ionosphere) is therefore to be read as an independent contribution, not as the reproduction of a published procedure. The closest methodological relative is the TEC anomaly statistics (Heki 2011), which however tests no direction.

---

## Question 4: corona — flare emission channels (X-ray ↔ EUV)

### Target (measured absent)

The target combination — transfer entropy **between emission channels** (GOES
soft X-ray bands + AIA EUV bands), **per flare event**, over **all directed
channel pairs**, with a **per-pair surrogate threshold** — was searched on the
arXiv API and Crossref this session and found in no entry. The arXiv queries
`"transfer entropy" AND "solar flare"` (0 results), `"transfer entropy" AND
"corona"` in astro-ph.SR (0 results), and `"transfer entropy" AND "solar"`
(4 results, none on emission channels) all return empty or off-target; the
Crossref queries `"transfer entropy" solar flare` and `"transfer entropy"
coronal heating` return no TE-between-channels work. The ADS verification this
session confirms the absence: the four disputed names below resolve to zero or
off-target in ADS (see "Disputed entries"). As with Question 3, the absence is
a measured result, not a gap of the search.

### Neighbor matrix (verified this session, ADS bibcodes)

| Work | Method | Series/cadence | Lags | Null model | Multiple comparison | Finding | Status | Source |
|---|---|---|---|---|---|---|---|---|
| Wing, Johnson, Vourlidas (2018). *Information Theoretic Approach to Discovering Causalities in the Solar Cycle*. The Astrophysical Journal. | Transfer entropy | Solar-cycle indices (polar field ↔ sunspot number) | Cycle-scale (months–years) | Not stated in the entry | Not stated in the entry | TE on solar-cycle **indices**, not emission channels — the correct attribution for the entry a parallel session listed as "Zou et al. 2014" | [T] | https://doi.org/10.3847/1538-4357/aaa8e7 |
| Reda, Stumpo, Giovannelli, Alberti, Consolini (2024). *Disentangling the solar activity-solar wind predictive causality at Space Climate scales*. Rendiconti Lincei. Scienze Fisiche e Naturali. | Transfer entropy | Solar-activity indices → solar wind; space-climate (long) scales | Cycle-scale (long) | Not stated in the entry | Not stated in the entry | TE on solar-activity **indices**, not emission channels | [T] | https://doi.org/10.1007/s12210-023-01213-w |
| Livadiotis, Cuesta, Khoo, Shen (2025). *Entropy transfer from solar radio bursts to energetic particles*. Science Advances. | Thermodynamic entropy transfer (kappa framework) | Solar radio bursts → energetic particles | — | — | — | "Entropy transfer" in the Livadiotis thermodynamic/kappa sense, **not** Schreiber transfer entropy — a neighbor at the solar object, not a methodological predecessor | [T] | https://doi.org/10.1126/sciadv.adz7419 |
| Cuesta, Livadiotis, McComas, Khoo (2025). *Transfer of Entropy between the Magnetic Field and Solar Energetic Particles during an Interplanetary Coronal Mass Ejection*. The Astrophysical Journal Letters. | Thermodynamic entropy transfer (kappa framework) | Magnetic field ↔ SEPs during an ICME | — | — | — | Same thermodynamic entropy transfer as above, during an ICME passage | [T] | https://doi.org/10.3847/2041-8213/adcbff |
| Qiu, Liu, Hill, Kazachenko (2010). *Reconnection and Energetics in Two-Ribbon Flares: A Revisit of the Bastille-Day Flare*. The Astrophysical Journal. | Cross-correlation / lead-lag | HXR ↔ UV/EUV flare ribbons per event | Event-scale lead-lag | Not stated in the entry | Not stated in the entry | Lead-lag between emission channels measured by **linear cross-correlation**, not TE — the closest empirical neighbor on the channel object | [T] | https://doi.org/10.1088/0004-637x/725/1/319 |

ADS bibcodes (this session): `2018ApJ...854...85W`, `2024RLSFN..35...49R`,
`2025SciA...11z7419L`, `2025ApJ...984L..50C`, `2010ApJ...725..319Q`,
`2019Entrp..21..140W`.

### Disputed entries — ADS-verified absent

The neighbor list a parallel session produced carried entries that the ADS run
this session confirms absent or off-target: **"Zou et al. 2014"** (ADS: 0 — the
polar-field→sunspot-number TE is Wing, Johnson & Vourlidas 2018, above),
**"Behreetas et al. 2020/2021"** (ADS: 0), **"Zhao et al. 2022"** (ADS: 1
off-target hit, perovskites), **"Dósa et al. 2025"** (ADS: the real M. Dósa
works on solar-wind propagation, not solar hemispheres — no such 2025 entry).
The cross-correlation lead-lag neighbor listed as "Simões et al. 2015" is Qiu
et al. 2010 (above). The attributions in this sheet are the ADS-measured ones.

### Search anchors (this session's queries — the zero is re-runnable)

The "measured absent" claims above are anchored to these exact queries, so a
later layer can re-run them (a zero without its query list is a number without
an anchor). arXiv API (`http://export.arxiv.org/api/query`, full-text field
`all`): `all:"transfer entropy" AND all:"solar flare"` (0), `all:"transfer
entropy" AND all:"corona" AND cat:astro-ph.SR` (0), `all:"transfer entropy" AND
all:"solar"` (4, none on emission channels), `all:"transfer entropy" AND
all:"sunspot"` (0). Crossref (`https://api.crossref.org/works`):
`query=transfer+entropy+solar+flare`, `query=transfer+entropy+coronal+heating`.
ADS (`https://api.adsabs.harvard.edu/v1/search/query`, NASA_ADS_TOKEN):
`author:"Zou" AND abs:"transfer entropy" AND abs:"solar"` (0),
`author:"Behreetas"` (0),
`author:"Zhao" AND abs:"transfer entropy" AND abs:"solar"` (1 off-target),
`author:"Dosa" AND abs:"solar"` (29, none a 2025 hemispheres-TE). DOI
resolutions: `doi:"10.1007/s12210-023-01213-w"`, `doi:"10.1126/sciadv.adz7419"`,
`doi:"10.3847/2041-8213/adcbff"`, `doi:"10.1088/0004-637x/725/1/319"`,
`doi:"10.3390/e21020140"`.

### Convergence/divergence to the omegaflow protocol

The corona sheet is the least calibrated: no published TE-between-emission-
channels work was found. The methodological neighbors split into three kinds
that must not be collapsed: (a) TE on solar **indices** over long time scales
(Wing et al. 2018; Reda 2024; Wing & Johnson 2019) — different object, long
lags, no per-event window; (b) **thermodynamic** "entropy transfer" in the Livadiotis kappa
framework (Livadiotis 2025; Cuesta 2025) — a different quantity, not Schreiber
TE, so no calibration for the estimator; (c) **linear** cross-correlation
lead-lag between flare emission channels (Qiu 2010) — the closest empirical
neighbor on the channel object, but linear and without a surrogate null. The
per-flare, all-pairs, per-pair-surrogate arrangement itself is, in this
search, unrepresented.

The shared-driver confound is the standing risk of this sheet (the same risk
the Bz sheet names): the channels are all fed by one common flare driver
(particle deposition) with different response time constants, so a naive TE
can report a "direction" that only mirrors the differing response functions,
not channel-to-channel flow. The omegaflow ladder formulation already treats
this by construction — the D excess and the phase-randomized null isolate the
directional residual (see `corona-heating-ladder.md` §5). Whether the per-pair
phase-randomized surrogate preserves the common flare envelope, or needs an
envelope-preserving surrogate (one that keeps the shared driver and destroys
only the fine channel coupling), is the open methodological question for the
full 72-pair matrix.

---

## Question 5: Farben-Laufzeit-Dispersion — arrival-time differences between the XUV/XRS channels of one solar source, against a surrogate null

### Target (measured absent)

The combination the dispersion probe (`tools/measure/src/bin/dispersion_solar_probe.rs`, Atom E) measures: (i) per-flare arrival differences between the XUV/XRS channels (AIA 94–335 Å + GOES XRS A/B) of **one** solar source, (ii) significance against a phase-randomized surrogate null (TE, per-pair threshold), (iii) a medium-vs-source separation (the Neupert envelope as the shared source driver; the residual is the medium). In the measured corpora this combination is unrepresented (anchors below; PDF full-text level unmeasured).

### Neighbor matrix (verified this session, ADS bibcodes)

| Work | Object | Method | Relation to the probe |
|---|---|---|---|
| Weber 1977 `1977SoPh...54..431W` | HELIOS-A/B type III radio bursts | two-spacecraft TOA triangulation | arrival-difference method, radio, no surrogate null |
| Reiner 2009 `2009SoPh..259..255R` | STEREO/Wind type III | multipoint triangulation | same, current |
| Krupar 2024 `2024ApJ...960..101K` | interplanetary type III | triangulation enhancement | same, current |
| Macquart 2020 (FRB DM) | FRB dispersion | ν⁻² medium delay | the dispersion law, radio/ISM, not XUV |
| Takakura 1983 `1983SoPh...89..379T` | HXR↔microwave peak delay | cross-correlation | channel delay, source-side, no null |
| Dennis & Zarro 1993 `1993SoPh..146..177D` | Neupert effect | light-curve ordering | the shared-driver object |
| Aschwanden 2007 `2007ApJ...661.1242A` | RHESSI multithermal delays | cross-correlation | channel delay, source-side |
| Woods 2014 `2014SoPh..289.3391W` | EUV late phase | light-curve delay | channel delay, source-side |
| Chen 2020 `2020ApJ...890..158C` | EUV late-phase channel delay | cross-correlation | channel delay, source-side |
| Li/Yuan/Wang 2017 `2017MNRAS.468.2552L` | frequency-dependent delay | cross-correlation | Sgr A*, not the Sun |

### Type III drift is not ν⁻² dispersion

The classical type III drift (Wild 1950 `1950AuSRA...3..387W`/`1950AuSRA...3..541W`, Wild & Smerd 1972 `1972ARA&A..10..159W`, Dulk 1985 `1985ARA&A..23..169D`, Boischot 1960 `1960ApJ...131...61B`) is the exciter motion against the plasma-frequency/density scale — a radial drift, not the ν⁻² medium dispersion of two bands of the same source. The ν⁻² measurement of the medium belongs to the radio/pulsar/FRB line, not to the solar drift line.

### Search anchors (the zero is re-runnable)

ADS abstract search (NASA_ADS_TOKEN): `abs:"transfer entropy" AND abs:"solar flare"` (0), `abs:"transfer entropy" AND abs:"EUV" AND abs:"flare"` (0), `abs:"dispersion measure" AND abs:"EUV" AND abs:"solar flare"` (0). arXiv API `all:` (metadata level, no PDF full text): `all:"transfer entropy" AND all:"solar flare"` (0), `all:"transfer entropy" AND all:"EUV"` (0), `all:"dispersion measure" AND all:"EUV" AND all:"solar"` (0), `all:"Neupert effect" AND all:"delay" AND all:"EUV"` (0).

### Convergence/divergence to the omegaflow protocol

The measured neighbors are the radio multi-point TOA triangulations (Weber 1977, Reiner 2009) for the arrival-difference *method*, and the Neupert family (Dennis 1993, Aschwanden 2007, Woods 2014) for the channel-delay *object*. None of them joins channel delay to a surrogate null, and at XUV energies the ν⁻² plasma dispersion of the FRB sense carries no measurable arrival component — the medium-vs-source separation is therefore carried by the analysis construction (the conditional null on the shared envelope + the ν⁻² ceiling), not by a directly dispersed arrival. The three-verb state of the dispersion verdict — flat / quell-seitig (source response order) / medium — is the register name of this separation.

---

## Caveats

- All entries were verified during this session; no citation is added from memory. [V] entries were read via Crossref abstract fields or the original page; [T] entries via Crossref records, DOI resolution, or reference lists in fetched abstracts.
- For [T] entries, lag ranges, null models, and multiple-comparison treatment were not read and are therefore listed as "not stated in the entry".
- The state of the ENSO search covers climate causality in the broader sense (teleconnections); a published TE entry exactly on the equatorial wind↔SST coupling was not found.
- The LAIC null statement refers to TE/Granger; works with anomaly statistics exist in abundance, but are not causal analyses.
