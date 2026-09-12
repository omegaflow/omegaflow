<!--
  title: Cross-screening the Kollab window and the Gyirong series
  class: paper
  date: 2026-09-12
  sha256: c4ffc07e56c33a55aae827e2bea906d6bacc39773df6a464ad6e8b7dd75a23d9
  status: live
  see-also: docs/blatt/blatt-kreuz-screening-kollab.md docs/blatt/blatt-kreuz-screening-gyirong.md docs/paper/sturzflut-tibet-pfeil.md
-->

# Cross-screening the Kollab window and the Gyirong series

## Abstract

Two basins, one question: which arrow survives the basin's shared driver. The cross-screening fleet pairs every series in both directions against a phase-randomized null (mean + 2σ). The verdict is predominantly silence. In the Kollab window (Trishuli 2026-08-26), all three conditionings — diurnal temperature_2m, pressure_msl, relative_humidity_2m — silence the south→north chain: rasuwa_precip → kollab_precip Lag 6 TE 0.2085 falls out, and the north→south arrow kollab_precip → rasuwa_precip Lag 12 flips in the power test (TE 0.1172 > 0.1072 unconditional; not significant after conditioning). The conditioned survivors are marginal: kollab→rasuwa cTE 0.2001 > 0.1970, gyirong→rasuwa cTE 0.1811 > 0.1627. In the Gyirong series, diffuse_radiation → wind_gusts_10m Lag 6 TE 0.386 > 0.157 marks the shared cycle (1728/1984 comparisons significant, 87 %), and precipitation/rain → global_tilted_irradiance Lag 12 cTE 0.0889 is the reverse arrow that survives. The Kollab blatt register line is closed (2026-09-12): the three conditioned cTE values are measured (0.1983 > 0.1815, 0.1602 > 0.1568, 0.1584 > 0.1538).

## The measurement

The cross-screening fleet asks one question of two basins: which arrow survives the basin's shared driver. The machine is `cross_te_screen`; for every pair of harvested series it measures transfer entropy in both directions at lags {1, 6, 12, 24} against a phase-randomized surrogate threshold (mean + 2σ, n_surrogate = 20), then conditions on the basin's shared driver with conditional transfer entropy (`transfer_entropy_conditional`), whose null is a residual surrogate (the source regressed on the driver, residuals permuted). The library convention is `transfer_entropy_lag(x, y) = TE(y → x)` — the second argument is the source. A pair is significant when `TE > threshold`; the threshold is per-pair, not a family correction, and the screening filters nothing (0 honored) — it lists every arrow the instrument carries.

The Kollab window (Trishuli 2026-08-26) carries six hourly Open-Meteo archive-api series — precipitation and temperature at gyirong (Tibet, upper reach), kollab (the collapse point), and rasuwa (Nepal, lower reach) — n = 240, 2026-08-18…27 UTC. The power update extends the three precipitation series and gyirong pressure_msl to the 8-week window 2026-07-01…08-27, n = 1392. The Gyirong series is the full 51-channel Open-Meteo catalogue of the station: 32 channels with n ≥ 100 usable, 496 pairs × 4 lags → 1984 comparisons, the same 2026-08-18…27 window. Both are model/reanalysis output (Open-Meteo archive = Modell/Reanalyse), named as such, not in-situ observation.

## The Kollab window

**The silence comes first.** All three conditionings — the diurnal proxy gyirong temperature_2m, the second proxy pressure_msl, and the second proxy relative_humidity_2m — silence the south→north chain. The unconditional monsoon arrow rasuwa_precip → kollab_precip Lag 6 (TE 0.2085) falls out of significance under each of the three independent shared drivers; kollab_precip → gyirong_precip Lag 6 (TE 0.1377) carries the same shared driver. What remains after conditioning is the marginal north→south family: kollab_precip → rasuwa_precip Lag 12 (cTE 0.2001 > 0.1970) and gyirong_precip → rasuwa_precip Lag 12 (cTE 0.1811 > 0.1627) — just over threshold. Under the second proxies only kollab→rasuwa Lag 12 recurs (pressure cTE 0.1902 > 0.1866; relative humidity cTE 0.1898 > 0.1872).

**The north→south arrow flips in the power test.** In the full 8-week window (n = 1392) the arrow is unconditionally measurable — kollab_precip → rasuwa_precip Lag 12, TE 0.1172 > 0.1072 — but it falls out of significance after conditioning on the shared gyirong pressure_msl driver. Conditioned, the three gyirong-involving arrows survive at Lag 12: gyirong→rasuwa cTE 0.1983 > 0.1815, gyirong→kollab cTE 0.1602 > 0.1568, kollab→gyirong cTE 0.1584 > 0.1538 (measured 2026-09-12, 20 surrogates). A full-lag re-run (lags 1, 6, 12, 24) also carries a marginal rasuwa→kollab Lag 6 cTE 0.2099 > 0.2097 — margin 0.0002, named, not hidden.

**The register line is closed (2026-09-12).** The three conditioned cTE values are measured (above); the blatt `blatt-kreuz-screening-kollab.md` header moves to `live`. Its two statements stay separate: the south→north direction is a supported silence on this instrument (three controls, no physical negation), and the north→south arrow is not isolable over the shared driver.

## The Gyirong series

The Lag-6 cluster across radiation ↔ wind ↔ pressure carries the high unconditional TE values (0.36–0.39) — the shared diurnal/synoptic cycle, not a directed arrow. Its lead member is diffuse_radiation → wind_gusts_10m Lag 6, TE 0.386 > 0.157. Across the catalogue 1728 of 1984 comparisons are unconditionally significant (87 %); after conditioning on the diurnal proxy temperature_2m, 0 arrows enter precipitation/rain — no local channel drives the precipitation over the shared cycle.

The reverse arrow survives conditioning: precipitation/rain → global_tilted_irradiance Lag 12, cTE 0.0889 — rain blocks insolation. The exact conditioned cTE values of the three gyirong-involving arrows that survive the Kollab 8-week power update are measured and carried in the Kollab section (gyirong→rasuwa 0.1983 > 0.1815, gyirong→kollab 0.1602 > 0.1568, kollab→gyirong 0.1584 > 0.1538).

## The finding

The fleet verdict: in the Kollab window the south→north arrow is silent under all three shared drivers and the north→south arrow is not isolable over the shared driver once the power test conditions on it; in the Gyirong series no local channel carries precipitation over the shared cycle, while the reverse arrow precipitation/rain → global_tilted_irradiance Lag 12 cTE 0.0889 survives.

## The form

Limits and pendings, named not assumed:

- One window, one event per basin; no generalization across events (Bordeaux, Aaretal, Japan open).
- Surrogates are phase-randomized: they measure synchronization, not directed causation; direction is read from the lag asymmetry.
- Conditioning removes the linear component aligned with the proxy; a residual synoptic confound (the moisture/pressure Lag-6 band) is not excluded by a single-proxy conditioning.
- n = 240 for the 10-day Kollab window; longer series harden the surrogate thresholds (n = 1392 in the power update).
- The Kollab blatt register line is closed (2026-09-12): the three conditioned cTE values are measured (0.1983 > 0.1815, 0.1602 > 0.1568, 0.1584 > 0.1538); the marginal rasuwa→kollab Lag 6 hit of the full-lag re-run (0.2099 > 0.2097) is named.
- Data are Open-Meteo archive-api model/reanalysis output, not in-situ observations; the co-local Trishuli gauge is the sibling measurement (`docs/paper/sturzflut-tibet-pfeil.md`).

## References

1. Schreiber, T. (2000). Measuring information transfer. Physical Review Letters 85, 461. doi:10.1103/PhysRevLett.85.461. ADS: 2000PhRvL..85..461S.
2. Theiler, J., Eubank, S., Longtin, A., Galdrikian, B., Farmer, J. D. (1992). Testing for nonlinearity in time series: the method of surrogate data. Physica D 58, 77–94. doi:10.1016/0167-2789(92)90102-S. ADS: 1992PhyD...58...77T.
3. Shugar, D. H. et al. (2021). A massive rock and ice avalanche caused the 2021 disaster at Chamoli, Indian Himalaya. Science 373, 300–306. doi:10.1126/science.abh4455. ADS: 2021Sci...373..300S.
