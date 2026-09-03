<!--
  title: Meteo channel curation — force/unit mapping for sources.φ
  class: concept
  date: 2026-08-31
  sha256: d472d349bb93d23a46837b5f11ca51cfb861ee3655d0b7e17e4af1846bc59d03
  status: live
  see-also: docs/specs/meteo-korrelations-screening-vorlage.md
-->
# Meteo channel curation — force/unit mapping for sources.φ

Applies to the 153 series (3 stations × 51 channels) harvested from the
Open-Meteo archive API and uploaded to CDN release `archive-api.open-meteo.com`.

Wire format per channel (one `url` block per station-variable):

    url https://github.com/omegaflow/sources/releases/download/archive-api.open-meteo.com/<station>_open-meteo_<variable>.json
    ttl 300
    on earth <lat> <lon> 0
    last points.v <name> <kernel> <force> <unit> <tau> 0.0 0.0

Kernel is derived from force via `default_kernel_for`. Tau = 300 (5 min) for
all series; values are hourly archive samples, so tau may be raised to 3600
at curation time. Station coords:

| station | lat | lon |
|---|---|---|
| gyirong | 28.8559 | 85.2950 |
| kollab  | 28.2710 | 85.5150 |
| rasuwa  | 28.2500 | 85.1000 |

## Channel table

Force legend: `em`=electromagnetic, `th`=thermal, `ad`=advective,
`di`=diffusion, `ac`=acoustic, `gr`=gravity.

| channel | unit | force | unit-ok? | verdict |
|---|---|---|---|---|
| temperature_2m | °C | th (k,c) | yes | WIRE |
| relative_humidity_2m | % | di (%) | yes | WIRE |
| dew_point_2m | °C | th (c) | yes | WIRE |
| apparent_temperature | °C | th (c) | yes | WIRE |
| precipitation_probability | % | di (%) | yes | WIRE |
| precipitation | mm | — | no | FLAG: mm/h flux not in registry (ad has m3/s,cfs; ac has mm) |
| rain | mm | — | no | FLAG (same as precipitation) |
| showers | mm | — | no | FLAG (same as precipitation) |
| snowfall | cm | — | no | FLAG: cm only in seismic-surface; snow flux needs decision |
| snow_depth | m | ad (m) | yes | WIRE but physics approx (length via advective) |
| freezing_level_height | m | ad (m) | yes | WIRE but physics approx (height) |
| weather_code | 1 | em (1) | yes | FLAG nominal: WMO code, unit `1`, non-additive |
| pressure_msl | hPa | ac (hpa) | yes | WIRE |
| surface_pressure | hPa | ac (hpa) | yes | WIRE |
| cloud_cover | % | di (%) | yes | WIRE |
| cloud_cover_low | % | di (%) | yes | WIRE |
| cloud_cover_mid | % | di (%) | yes | WIRE |
| cloud_cover_high | % | di (%) | yes | WIRE |
| wind_speed_10m | km/h | ad (km/h) | yes | WIRE |
| wind_speed_80m | km/h | ad (km/h) | yes | WIRE |
| wind_speed_120m | km/h | ad (km/h) | yes | WIRE |
| wind_speed_180m | km/h | ad (km/h) | yes | WIRE |
| wind_direction_10m | ° | — | no | FLAG: angle not in registry (directional, not scalar field) |
| wind_direction_80m | ° | — | no | FLAG (angle) |
| wind_direction_120m | ° | — | no | FLAG (angle) |
| wind_direction_180m | ° | — | no | FLAG (angle) |
| wind_gusts_10m | km/h | ad (km/h) | yes | WIRE |
| shortwave_radiation | W/m2 | em (w/m2) | yes | WIRE |
| direct_radiation | W/m2 | em (w/m2) | yes | WIRE |
| diffuse_radiation | W/m2 | em (w/m2) | yes | WIRE |
| direct_normal_irradiance | W/m2 | em (w/m2) | yes | WIRE |
| global_tilted_irradiance | W/m2 | em (w/m2) | yes | WIRE |
| vapour_pressure_deficit | kPa | ac (hpa) | conv | WIRE: declare unit `hPa` and convert value ×10 (kPa→hPa) |
| et0_fao_evapotranspiration | mm | — | no | FLAG (depth flux, same as precipitation) |
| evapotranspiration | mm | — | no | FLAG (depth flux) |
| surface_temperature | °C | th (c) | yes | WIRE |
| soil_temperature_0cm | °C | th (c) | yes | WIRE |
| soil_temperature_6cm | °C | th (c) | yes | WIRE |
| soil_temperature_18cm | °C | th (c) | yes | WIRE |
| soil_temperature_54cm | °C | th (c) | yes | WIRE |
| soil_moisture_0_1cm | m3/m3 | di (%) | conv | WIRE: declare unit `%` and convert value ×100 (fraction→pct) |
| soil_moisture_1_3cm | m3/m3 | di (%) | conv | WIRE (×100) |
| soil_moisture_3_9cm | m3/m3 | di (%) | conv | WIRE (×100) |
| soil_moisture_9_27cm | m3/m3 | di (%) | conv | WIRE (×100) |
| soil_moisture_27_81cm | m3/m3 | di (%) | conv | WIRE (×100) |
| is_day | 1 | — | no | FLAG nominal boolean (0/1), non-additive |
| wet_bulb_temperature_2m | °C | th (c) | yes | WIRE |
| total_column_integrated_water_vapour | kg/m2 | — | no | FLAG: kg/m2 not in registry (th has kg? no; em has w/m2) |
| snowfall_water_equivalent | mm | — | no | FLAG (depth flux) |
| leaf_wetness_probability | % | di (%) | yes | WIRE |
| sunshine_duration | s | — | no | FLAG: seconds not in registry (em lacks s) |

## Data value notes

- **`snow_depth` = 0.0 across the full window (all 240 points).** Gyirong,
  Tibet, August: no snow — the zero is a genuine null-real measurement, not a
  filler/fallback. The `0.0` is **0-honored** (declared data value, no
  fabrication). Wired as `m` via `advective` (length, physics approx).
  **Open point:** the gate currently flags any `0.0` as `zero-fabrication`
  without distinguishing a declared null-real data value from a fabricated
  filler zero. Until that rule distinguishes the two, the `snow_depth` value
  stays annotated here as 0-honored.

## Summary

- **WIRE** (30): temperature family, humidity, cloud cover, wind speeds, radiation, pressures, soil temperatures, leaf wetness.
- **CONVERT** (6): `vapour_pressure_deficit` (kPa→hPa ×10), 5× `soil_moisture_*` (fraction→% ×100).
- **FLAG** (14): depth fluxes (`precipitation`, `rain`, `showers`, `snowfall`, `et0`, `evapotranspiration`, `snowfall_water_equivalent`), angles (`wind_direction_*`), nominal codes (`weather_code`, `is_day`), `total_column_integrated_water_vapour`, `sunshine_duration`, `freezing_level_height`/`snow_depth` (physics approx).

FLAG channels are not wired into `sources.φ`; they remain CDN-only inputs for
the offline `cross_te_screen` screening. `sunshine_duration` can be emitted as
em `W/m2` after `s/3600` only if a physical meaning (irradiance proxy) is
declared — not assumed here.
