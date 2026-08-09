# sources_v2.φ — Canonical Format Specification

## 0. Axiom

A field _is_ its API key. No prefix. No name. No identity beyond the key.
Force and unit are the field's physical signature. A measurement without a
physical unit is metadata — it cannot manifest in the block universe.

## 1. Directive Table

Every directive starts at column 0. No indentation. One directive per line.

| Directive | Tokens | Meaning |
|-----------|--------|---------|
| `url <u>` | 2 | API endpoint. Template variables: `{lat}`, `{lon}`, `{today}`, `{yesterday}`, `{hour_ago}`, `{SECRET}` |
| `ttl <s>` | 2 | Fetch interval in seconds |
| `at <body>` | 2 | Barycentric frame of the named body. SSB origin. |
| `on <body> <lat> <lon>` | 4 | Surface point on the named body. Geodetic coordinates. |
| `on <body> <lat> <lon> <alt>` | 5 | Surface point with altitude in km. |
| `body <name>` | 2 | Data-carried position (map/cmap rows carry their own coords). |
| `method <GET\|POST>` | 2 | HTTP method. Default GET. |
| `header <name> <value>` | 3 | HTTP header. Repeatable. |
| `post_body <json>` | 2 | POST body. |
| `format <json\|text\|csv\|ephemeris_binary\|universal>` | 2 | Response format. Default json. |
| `map <arr>` | 2 | Iterate JSON array. Each element becomes an oscillator. |
| `cmap <arr>` | 2 | Celestial map. ICRS position from ra/dec/plx. |
| `rows <arr>` | 2 | Table rows (column-index or header-name lookup). |
| `flatten <arr>` | 2 | Flatten nested array geometry. |
| `field <key> <force> <unit>` | 4 | **Scalar physical measurement.** Key = JSON path (dot-notation). Force = propagation mechanism. Unit = SI or accepted non-SI. |
| `field <key> <identifier>` | 3 | **Inside map/cmap/rows only.** Annotates a row field. Identifier is the property label. |
| `lat <key> <unit>` | 3 | Latitude. Replaces `lat_key`. Unit: deg. |
| `lon <key> <unit>` | 3 | Longitude. Replaces `lon_key`. Unit: deg. |
| `alt <key> <unit>` | 3 | Altitude. Replaces `alt_key`. Unit: km or m. |
| `ra <key> <unit>` | 3 | Right ascension. Replaces `ra_key`. Unit: deg. |
| `dec <key> <unit>` | 3 | Declination. Replaces `dec_key`. Unit: deg. |
| `plx <key> <unit>` | 3 | Parallax. Replaces `plx_key`. Unit: mas. |
| `pmra <key> <unit>` | 3 | Proper motion RA. Unit: mas/yr. |
| `pmdec <key> <unit>` | 3 | Proper motion Dec. Unit: mas/yr. |
| `radvel <key> <unit>` | 3 | Radial velocity. Unit: km/s or m/s. |
| `z <key> <unit>` | 3 | Redshift. Unit: scalar (dimensionless ratio — admitted, has SI path). |
| `dist <key> <unit>` | 3 | Distance. Unit: pc, ly, m, km. |
| `epoch <key> <unit>` | 3 | Epoch timestamp. Unit: s or d. |
| `tau <key> <unit>` | 3 | Decay constant override. Unit: s. |
| `vel <key> <unit>` | 3 | Velocity. Unit: m/s or km/s. |
| `trk <key> <unit>` | 3 | Track/heading. Unit: deg. |
| `vr <key> <unit>` | 3 | Vertical rate. Unit: m/s. |
| `val <key> <unit>` | 3 | Value filter key. Unit: scalar. |
| `extent <m>` | 2 | Explicit spatial extent in meters. |
| `reach_ttl <s>` | 2 | Override fetch reach TTL. |
| `catalog_epoch <yr>` | 2 | Catalog reference epoch. |
| `target <body>` | 2 | Target body for ephemeris/vectors. |
| `catalog <name>` | 2 | Catalog identifier. |
| `max_freq <hz>` | 2 | Maximum frequency for observation planning. |
| `min_freq <hz>` | 2 | Minimum frequency. |
| `repeat <n> <ra_min> <ra_max> <dec_min> <dec_max>` | 6 | RA-binned repeat. |
| `flux_from_mag <key>` | 2 | Derive flux from magnitude key. |
| `abs_mag_from <key>` | 2 | Derive absolute magnitude. |
| `stations <url>` | 2 | Station list URL. |
| `stations_path <key>` | 2 | Path to station list in response. |
| `stations_lat <key>` | 2 | Station latitude key. |
| `stations_lon <key>` | 2 | Station longitude key. |
| `stations_id <key>` | 2 | Station ID key. |

## 2. Force-Unit Registry (grows with every API fetch)

The registry records every physical unit found in API responses. A unit is NOT a
filter — it is documentation of what exists. The registry grows with every
migration. No field is dropped because "the unit isn't in the matrix yet."

### What qualifies as a physical unit

Any unit with a defined SI conversion or accepted non-SI conversion. This
includes: SI base units (m, kg, s, A, K, mol, cd), SI derived units (Hz, N, Pa,
J, W, C, V, F, Ω, S, Wb, T, H, Gy, Sv, lm, lx, Bq, kat), accepted non-SI
units (min, h, d, deg, ', '', ha, L, t, eV, Da, u, AU), and common derived
combinations (m/s, m/s², kg/m³, W/m², etc.).

Units like knot, ft, mmHg, mbar, bar, atm, psi, cal, kcal, erg, dyn, gal, Ci
are accepted non-SI units with defined SI conversions. They are physical units.

### What is NOT a physical unit

Dimensionless indices without a defined reference scale: ssn (sunspot number),
kp_index, Kp, a_index, uv_index, weather_code, magType, storm_level, noaa_scale.
These are human classification schemas.

Counts: integer tallies of discrete objects or events. `total`, `count`,
`number_spots`, `station_count`. The human operation of counting is not a field
measurement.

Identifiers, timestamps, strings, booleans — these are metadata, not field
quantities.

### Force assignment

The force describes the propagation mechanism of the measured quantity:

| Force | ID | Propagation | What it covers |
|-------|----|-------------|----------------|
| `em` | 0 | c (light) | EM radiation (flux, irradiance), magnetic fields, radio, radar measurements, optical depth (AOD is EM extinction), lightning EM pulse |
| `gravity` | 1 | c (grav. wave) | Gravitational acceleration, tidal force, orbital velocity measured gravitationally, Earth gravity field, elevation (geopotential height) |
| `seismic-body` | 2 | body wave vel. | Subsurface displacement, body wave velocity/acceleration, hypocentral depth, borehole measurements |
| `seismic-surface` | 3 | surface wave vel. | Surface displacement, ground shaking, surface wave amplitude, earthquake effects at surface, volcano deformation |
| `thermal` | 4 | thermal diffusivity | Temperature (any medium), heat flux, brightness temperature (IR), fire temperature |
| `diffusion` | 5 | diffusion coeff. | Gas concentration, aerosol mass, water chemistry, salinity, turbidity, dissolved oxygen, nutrients, humidity (water vapor diffusion), soil moisture, pCO2, CH4, O3, NO2 |
| `advective` | 6 | flow velocity | Wind speed/direction, ocean currents, river discharge, air pressure (advected air mass), water level (advective flow), streamflow, wave energy flux |
| `acoustic` | 7 | sound speed | Wave height/period, sound pressure, acoustic frequency, precipitation (rain/snow accumulation as acoustic displacement), infrasound |
| `electric` | 8 | dielectric relax. | E-field strength, bioelectric potential, conductivity, telluric currents, lightning current, atmospheric potential gradient |

The rule: same physical quantity, different measurement mechanism → different
force. ISS velocity via radar → em. Solar wind via plasma instrument → advective.
Magnetometer → em. Gravimeter → gravity.

### Known units by force (growing — NOT a filter)

This table records all units discovered so far. New units found during migration
are added here:

**em:** W/m2, Wm-2, nT, uT, T, G, sfu, Jy, mJy, μJy, km, m, mm, km/s, m/s,
deg, rad, W, kW, MW, GW, eV, keV, MeV, GeV, erg, erg/s, erg/cm2/s, counts/s,
MHz, GHz, AOD (dimensionless extinction coefficient — physically grounded EM measurement)

**gravity:** m/s2, gal, mGal, μGal, m/s, km/s, m, km, m3/s2, E (Eötvös),
nm/s2, deg

**seismic-body:** m, km, mm, μm, m/s, mm/s, km/s, m/s2, gal, cm/s2, mm/s2, s, ms

**seismic-surface:** m, km, mm, cm, m/s, deg, rad, s, ms

**thermal:** K, C, F, W/m2, kW/m2, W, kW, MW, J, kJ, MJ, MJ/m2, kJ/m2

**diffusion:** p/cm3, cm-3, 1/cm3, p/m3, m-3, ppm, ppb, ppt, mg/m3, μg/m3,
mg/L, μg/L, μmol/L, μmol/mol, nmol/mol, ppmv, ppbv, hPa, Pa, mb, mbar, atm,
mmHg, %, g/kg, g/m3, kg/m3, kg/kg, m3/m3, PSU, NTU, FNU, μS/cm, mS/cm, S/m,
μatm, matm, DU (Dobson), cm, mm (precipitable water, column amount)

**advective:** m/s, km/s, km/h, knot, mph, m3/s, ft3/s, L/s, m3, km3, hPa,
Pa, mb, mbar, atm, bar, psi, deg, rad, K, C, m, ft, cm, mm (water level,
gauge height, stage), kg/m2s (mass flux), kg/m3, W/m2 (kinetic energy flux)

**acoustic:** m, cm, mm, ft, m/s, km/s, Pa, hPa, kPa, MPa, deg, rad, dB, dBA,
dBZ, dB SPL, Hz, kHz, MHz, s, ms, min, h, d, W/m2 (sound intensity), J/m2,
N/m2

**electric:** V/m, mV/m, μV/m, kV/m, V, mV, μV, nV, kV, μS/cm, mS/cm, S/m,
Ω·m, kΩ·m, A, mA, kA, A/m2, W, kW, MW, C/m2, F/m, V/m2
- Satellite velocity via radar Doppler → `em km/s` (measurement IS the radar signal)
- Solar wind proton speed via plasma instrument → `advective km/s` (measurement IS the protons)
- Buoy wind speed → `advective m/s` (air mass advection)
- Buoy wave height → `acoustic m` (surface gravity wave)
- Buoy air temperature → `thermal C` (contact thermometry)
- Buoy pressure → `advective hPa` (air mass pressure)
- Magnetometer B-field → `em nT` (measurement IS electromagnetic induction)
- Earthquake depth → `seismic-body km` (hypocentral location of body wave source)
- GOES X-ray flux → `em W/m2` (measurement IS the X-ray photons)
- Fermi gamma-ray flux → `em W/m2` (measurement IS the gamma-ray photons)
- Radio flux 10.7cm → `em sfu` (measurement IS the radio photons)
- Lightning stroke current → `electric kA` (measurement IS electric discharge)
- Atmospheric E-field → `electric V/m` (measurement IS the potential gradient)
- Telluric current → `electric μS/cm` (measurement IS ground conductivity)
- Bioelectric potential (plant AP, ECG) → `electric mV` (measurement IS the tissue potential)
- Water conductivity → `electric μS/cm` (measurement IS ionic conductance)

## 3. Drop Rules — What Does Not Manifest

These are DROPPED from every source block. No exceptions.

### 3.1 Dimensionless Indices

Any field whose unit is `scalar` or that has no physical SI conversion path.
Examples: `ssn`, `classType`, `kp_index`, `estimated_kp`, `Kp`, `a_running`,
`uv_index`, `weather_code`, `magType`, `type`, `status`, `alert`, `cdi`, `mmi`,
`sig`, `felt`, `tsunami`, `confidence`, `dmin`, `nst`, `rms`, `gap`,
`flare_index`, `storm_level`, `noaa_scale`.

Earthquake magnitude (`mag`) is specifically DROPPED. It is a dimensionless
logarithmic ratio with no direct SI conversion without distance and instrument
type data that the GeoJSON API does not provide.

### 3.2 Counts

`count`, `number_spots`, `station_count`, `total`, `event_count`, `multiplicity`,
any field that reports "how many" rather than a physical quantity.

### 3.3 Identifiers

`hex`, `flight`, `callsign`, `icao24`, `origin_country`, `evid`, `publicID`,
`locality`, `place`, `region`, `flynn_region`, `satellite`, `net`, `source`,
`stationIdentifier`, `name`, `stid`, `ICAO`, `STATION_NAME`, `COUNTRY`,
`siteName`, `variableName`, `Hypocenter`, `station`, `id`, `code`, `wmo`,
`wban`, `usaf`, `buoy_id`, `platform`, `sensor`.

### 3.4 Timestamps

`time`, `time_tag`, `timestamp_utc`, `observed_date`, `generated`,
`local_date_time`, `dateTime`, `timeZone`, `OriginTime`, `obsTime`,
`lastUpdated`, `beginTime`, `peakTime`, `endTime`, `AnnouncedTime`.

### 3.5 Booleans

`isCancel`, `isFinal`, `domesticTsunami`, `isSea`, `isTraining`, `active`.

### 3.6 Coordinates as Scalar Fields

`latitude`, `longitude`, `altitude`, `ra`, `dec`, `plx`, `solar_lat`,
`solar_lon`, `lat`, `lon` when extracted via `field` (4-token) — these are
position data that belong in `lat`/`lon`/`alt`/`ra`/`dec`/`plx` directives.

### 3.7 Strings / Text

Any field whose API value is a string type. The field value must be numeric.
Exception: `field <key> <identifier>` inside map/cmap may annotate string fields
for property tracking (e.g., quality flags, event types coded as strings) — but
the field value itself remains the physical measurement from a numeric field.

#### 3.9 Position-Only Oscillators

A block with position (lat/lon or ra/dec) but ZERO surviving field measurements
is DROPPED ENTIRELY. A point on the map without a physical measurement is
meaningless in the block universe. The oscillator needs a field value.

Before dropping a block, research the API: does it provide ANY physical
measurement? Depth, temperature, flux, velocity, concentration — if the API
returns a number with a physical unit, keep it. If the API truly only returns
identifiers, timestamps, and coordinates with no measurements → DROP.

Any key that does not exist in the live API response is DROPPED. The migration
must fetch every URL and verify every key against the actual response structure.
No field name is ever invented or guessed.

## 4. Frame Semantics

### `at <body>`
ICRS origin at the body's barycenter. Used for:
- Solar phenomena (`at sun`)
- Heliospheric spacecraft at L1 (`at sun`)
- Celestial catalogs with ICRS sky coordinates (`at sun`)
- Planetary ephemeris targets — the body is the TARGET, not the frame (`target mars` + `at sun`)
- Orbiting satellites whose position is data-carried (`pos` + `at sun`)

### `on <body> <lat> <lon> [alt]`
Fixed geodetic point rotating with the body's surface (WGCCRE rotation model).
Used for:
- Ground stations, buoys, weather stations (`on earth`)
- Fixed-frame observations (`on mars 14.0 90.0`, `on moon 0 0`)
- ADSB receivers, METAR stations
- The browser station declares its position via `on <body>` oscillators over WebSocket

### `body <name>`
Declares the body for data-carried position. Each data row carries its own
lat/lon via `lat`/`lon` directives inside a `map` block. The `body` directive
tells the parser which body's rotation model to apply.
- `body earth` — GeoJSON earthquake feeds, OBIS, GBIF
- `body mars` — Mars weather stations with per-row lat/lon
- `body sun` — solar active regions with Stonyhurst coordinates

`body` never appears together with `at` or `on` in the same block.

### `pos <lat_key> <lon_key> [alt_key] [scale]`
Legacy carry-over for data-carried position. Equivalent to `body <body>` +
`lat <lat_key> deg` + `lon <lon_key> deg`. Prefer `body` + `lat`/`lon`.

## 5. Map / CMap / Rows Blocks

```
url https://example.com/geojson
ttl 60
body earth
map features
lat geometry.coordinates.1 deg
lon geometry.coordinates.0 deg
field depth km
field mag seismic-body scalar
```

- `map <arr>` or `cmap <arr>` or `rows <arr>`: the array to iterate.
- `lat`, `lon`, `alt`, `ra`, `dec`, `plx`: position extraction from each row.
- `field <key> <unit>` inside map (3-token): a physical field value extracted from each row.
  The unit goes in the 3rd token. The force is inherited from the block context
  (or can be specified as a 4-token `field <key> <force> <unit>` for mixed-force rows).
- `field <key> <identifier>` (3-token, identifier is NOT a unit): metadata annotation
  (quality flags, instrument IDs encoded as strings). Parsed but not rendered as field value.
- Non-physical fields inside map (IDs, names, timestamps) are dropped per RULE 3.
- Drop rules apply per field_in inside map blocks just as they do for standalone fields.

## 6. API-as-Specification

The unit for every field line comes from the API response. The hierarchy:

1. **API response metadata.** If the API returns a `"units": "kilometers"` field,
   the unit is `km`. If the API returns `"parameter": {"unit": "m/s", "value": 450}`,
   the unit is `m/s`.
2. **API schema documentation.** If the API docs specify units (NDBC header line
   `#YY MM DD hh mm WDIR WSPD WVHT`, SWPC product page "proton_speed in km/s"),
   the unit comes from the docs.
3. **Content-type headers.** CSV column headers may carry unit hints.
4. **Key-name suffix inference (last resort).** Pattern matches:
   - Distance: `_km` → `km`, `_m` → `m`, `_mm` → `mm`, `_nm` → `nm`
   - Speed: `_ms` → `m/s`, `_km_s` → `km/s`, `_kmh` → `km/h`, `_mph` → `mph`, `_knot` → `knot`
   - Acceleration: `_ms2` → `m/s2`, `_gal` → `gal`
   - Temperature: `_c` → `C`, `_k` → `K`, `_f` → `F`
   - Pressure: `_hpa` → `hPa`, `_pa` → `Pa`, `_mb` → `mb`, `_atm` → `atm`
   - Magnetic: `_nt` → `nT`, `_ut` → `uT`, `_t` → `T`, `_gauss` → `G`
   - EM flux: `_wm2` → `W/m2`, `_w_m2` → `W/m2`, `_jy` → `Jy`, `_mjy` → `mJy`, `_sfu` → `sfu`
   - Concentration: `_cm3` → `p/cm3`, `_m3` → `p/m3`, `_ppm` → `ppm`, `_ppb` → `ppb`, `_mgm3` → `mg/m3`, `_ugm3` → `μg/m3`, `_mgl` → `mg/L`
   - Angle: `_deg` → `deg`, `_rad` → `rad`
   - Astrometry: `_mas` → `mas`, `_arcsec` → `arcsec`
   - Frequency: `_hz` → `Hz`, `_khz` → `kHz`, `_mhz` → `MHz`, `_ghz` → `GHz`
   - Electric: `_vm` → `V/m`, `_mvm` → `mV/m`, `_kvm` → `kV/m`, `_mv` → `mV`, `_uv` → `μV`, `_v` → `V`, `_uscm` → `μS/cm`, `_sm` → `S/m`
   - Water quality: `_ntu` → `NTU`, `_psu` → `PSU`, `_umol_l` → `μmol/L`
   - Sound: `_db` → `dB`, `_dba` → `dBA`, `_dbz` → `dBZ`
   - Time: `_s` → `s`, `_ms` → `ms` (careful: `_ms` could be m/s — context-dependent)
   - Percentage: `_pct` → DROP (percentage is dimensionless ratio without physical reference)
   - Indices/dimensionslos: `_index`, `_scale`, `_code` → DROP
5. **If no unit can be determined by any method: DROP the field line.** The Council
   refuses to invent units.

## 7. Example Blocks

### ISS Position
```
url https://api.wheretheiss.at/v1/satellites/25544
ttl 10
at sun
lat latitude deg
lon longitude deg
alt altitude km
field velocity em km/h
field footprint em km
```

API response: `{"latitude": -47.75, "longitude": 78.87, "altitude": 438.28, "velocity": 27528, "footprint": 4599, "units": "kilometers"}`. All keys verified. `velocity` unit: km/h (ISS API docs). Force: EM (radar Doppler tracking). `footprint`: km (API `"units":"kilometers"`), force EM (instrument footprint).

### ADS-B Aircraft
```
url https://api.adsb.lol/v2/point/{lat}/{lon}/250
ttl 10
on earth 52.5 13.4
map ac
lat lat deg
lon lon deg
field alt_baro advective hPa
field gs advective km/h
field track em deg
field baro_rate advective hPa
```

API response: `{"ac": [{"hex":"...","flight":"...","alt_baro":36000,"gs":450,"track":270,"baro_rate":0,"lat":52.5,"lon":13.4}]}`. `hex`, `flight` dropped (IDs). `alt_baro`: ft → hPa, force advective (barometric pressure altitude). `gs`: knots → km/h, force advective. `track`: deg, force em (transponder measurement). `baro_rate`: ft/min → hPa, force advective.

### NDBC Buoy
```
url https://www.ndbc.noaa.gov/data/realtime2/13002.txt
ttl 300
on earth 21.0 -23.0
field WDIR advective deg
field WSPD advective m/s
field WVHT acoustic m
field DPD acoustic s
field APD acoustic s
field MWD acoustic deg
field PRES advective hPa
field ATMP thermal C
field WTMP thermal C
field PTDY advective hPa
```

API response (CSV header): `#YY MM DD hh mm WDIR WSPD GST WVHT DPD APD MWD PRES ATMP WTMP DEWP VIS PTDY TIDE`. Units from NDBC header line: WDIR=degT, WSPD=m/s, WVHT=m, DPD=sec, APD=sec, MWD=degT, PRES=hPa, ATMP=degC, WTMP=degC, PTDY=hPa. Force assignments: wind → advective, waves → acoustic, temperature → thermal, pressure → advective. `GST`, `DEWP`, `VIS`, `TIDE` — additional fields available but shown subset.

### GOES X-Ray Flux
```
url https://services.swpc.noaa.gov/json/goes/primary/xrays-1-day.json
ttl 60
at sun
field flux em W/m2
```

API response: `[{"time_tag":"...","satellite":"...","flux":1.23e-6,"energy":"..."}]`. Array-of-objects. `flux` key exists, value is in W/m2 (GOES XRS product spec). Force: EM. `time_tag`, `satellite`, `energy` dropped (timestamp, ID, metadata).

### Seismic Portal (GeoJSON)
```
url https://www.seismicportal.eu/fdsnws/event/1/query?format=json&limit=100&minmagnitude=2.5&orderby=time&start={today}T00:00:00
ttl 60
body earth
map features
lat geometry.coordinates.1 deg
lon geometry.coordinates.0 deg
field depth seismic-body km
```

API response: GeoJSON FeatureCollection. `features[]` array. Per feature: `geometry.coordinates` = [lon, lat, depth(-km)]. Properties: `evid`, `time`, `mag`, `depth`, `flynn_region`, `evtype`, `auth`, `magtype`. Surviving fields: `depth` (km, seismic-body). Dropped: `mag` (dimensionless), `evid` (ID), `time` (timestamp), `flynn_region` (string), `evtype`/`auth`/`magtype` (metadata strings).

### Celestial Catalog (Fermi 4FGL)
```
url https://heasarc.gsfc.nasa.gov/xamin/vo/tap/sync?REQUEST=doQuery&LANG=ADQL&FORMAT=csv&QUERY=SELECT+*+FROM+fermi4fgl
ttl 604800
at sun
cmap .
ra ra deg
dec dec deg
plx default_plx mas
field flux_1_100_gev em W/m2
field energy_flux em W/m2
field detection_significance diffusion scalar
```

Position keys locate the source in ICRS. `flux_1_100_gev`, `energy_flux`: physical EM flux measurements. `detection_significance`: statistical test value (sigma) — dimensionless but physically grounded statistical measurement. Accept as `diffusion scalar` (it is a property of the detector noise distribution). `name`, `data_release`, `spectrum_type`, time fields — dropped.

## 8. Block Survival

A block survives the migration if it has at least one `field` line (4-token)
with a physical unit. Position alone is not enough — a point without a
measurement is not an oscillator.

Before dropping any block, research the API: does it return ANY numeric value
with a physical unit? Depth, temperature, pressure, concentration, velocity,
flux — if yes, keep it as a field measurement. If the API only returns metadata
(IDs, names, timestamps, counts) → DROP the entire block.

## 9. Migration Algorithm (revised)

```
for each block in sources.φ:
    url = block.url
    fetch live API response
    keep: url, ttl, at/on/body, map/cmap/rows, structural directives
    drop: block-level force

    for each field / last / path / first / last_row line:
        key = extract_key_name(line)
        if key not in API response → DROP
        if key in dropped categories (RULE 3) → DROP
        unit = determine_unit(api_response, key_name_pattern)
        if unit == None → DROP
        force = assign_force(key, unit, block_force_hint)
        if not allowed_units_for_force(force).contains(unit) → DROP
        output: field <key> <force> <unit>

    for each field_in inside map/cmap/rows:
        same filtering as above
        if survives: output: field <key> <unit>  (3-token inside map)

    for each *_key directive:
        output: <keyword> <key> <unit>  (3-token)

    at <body> <scale> → at <body>  (drop scale)
    pos <lat> <lon> [alt] [scale] → lat <lat> deg + lon <lon> deg [+ alt <alt> km]

    if block has fields or position → write to sources_v2.φ
    else → drop block entirely
```

## 10. Non-Goals

This spec defines the canonical DATA FORMAT only. It does not define:
- Parser implementation (separate session)
- Extract variant unification (`first`→`field`, `last_row`→`field` — separate session)
- SI conversion functions (separate session with parser update)
- Anomaly reporting
- CDN/live routing

## 11. Matrix Evolution (initial → complete)

The Force-Unit matrix is initial. It will grow with every API fetched during
migration. The test is always: could an organism evolve a sensory organ for this
measurement?

**Known missing units (discoverable in APIs):**
- EM: Jy, mJy, μJy (radio astronomy flux density), erg/cm²/s (X-ray flux),
  ph/cm²/s (photon flux), counts/s (detector rate — borderline, organism senses
  photon rate not count abstraction)
- Gravity: mGal, μGal (gravimetry), E (Eötvös, gravity gradient), nm/s² (LIGO
  strain-equivalent acceleration)
- Seismic: mm/s (weak motion), nm (displacement noise floor), rad (tilt),
  μrad (borehole tilt)
- Thermal: kJ/m² (heat flux), W/m·K (thermal conductivity — property, not
  measurement)
- Diffusion: mg/m³, μg/m³ (aerosol mass concentration), μmol/mol (mole fraction),
  mol/m³, Bq/m³ (radon activity — borderline, organism senses radiation damage,
  not activity units)
- Advective: m³/s (volumetric flow), L/s, Sv (sediment transport — borderline),
  kg/m²s (mass flux)
- Acoustic: μPa (underwater reference), dB re 1μPa, dB SPL, dB(A), sone (loudness),
  phon (loudness level)
- Electric: V/m, kV/m (atmospheric potential gradient), μV/m, V, mV, μV (potential),
  nV (neural-level), μS/cm, mS/cm, S/m (conductivity), Ω·m (resistivity),
  A/m² (current density), pA (cell-level current), fA (ion channel current)
- Biotic: mm (growth), g/m² (biomass density), individuals/m² (population density —
  borderline, organism senses proximity not count), LAI (leaf area index —
  dimensionless but physically defined ratio)

**ESP32 Mantis-Shrimp Observatory — the litmus test embodied:**
The hardware spec at `docs/omegaflow_sense_hardware.yaml` lists every sensory
channel biology has evolved. Every sensor on the ESP32 defines a physical
measurement with an SI unit. The Force-Unit matrix converges toward complete
coverage of ALL organism-sensible physical quantities.

The migration does not stop at the known units. Every new API response may
surface a new physical unit. The response is: add it to the matrix, assign its
force, and continue.
