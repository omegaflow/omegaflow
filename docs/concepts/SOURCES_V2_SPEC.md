# sources.φ — Canonical Format Specification

Verified against the living parser (`load_sources` in `src/main.rs`). Every
directive listed here has a parser arm. Directives without a parser arm are
listed under „Non-Goals & Known Parser Gaps" as open work — writing them into a block today produces nothing
(0 honored, silently).

## 0. Axiom

A field _is_ its API key. No prefix. No name. No identity beyond the key.
Force and τ are the field's physical signature. A field without a declared
τ produces no oscillators — the τ-Gate closes (0 honored). A measurement
without a physical unit is metadata — it cannot manifest in the block universe.

All oscillators are equal. API oscillators (fetched channels), Body
oscillators (ephemeris bodies with radius), and Device oscillators (browser
station sensors) carry the same 12-value record, pass through the same
Enclosure Lemma, and are manifested by the Mathematikerin onto every
radiator — window, audio, stderr, serial, USB, BT, HID. No source class is
privileged. Every fetched response is mirrored to the CDN by the CI Archivar
(release tag = API netloc); local Archivars read the CDN first and fall back
to the live API.

## 1. Directive Table

Every directive starts at column 0. No indentation. One directive per line.
`url` starts a block and resets ALL block state (frame, ttl, force, headers,
post_body, stations, …) — nothing leaks between blocks. A block is refused
unless it has `url` + `ttl` + a frame (`at`/`on`).

| Directive | Tokens | Meaning |
|-----------|--------|---------|
| `url <u>` | 2 | API endpoint. Starts the block. Template variables: see „URL template variables" |
| `ttl <s>` | 2 | Fetch interval in seconds. Fetch fires at ttl/Φ. Retention ttl×2⁶. |
| `at <body>` | 2 | Barycentric frame of the named body (scale fixed 1.0). Also sets the block's body. |
| `on <body> <lat> <lon> [alt]` | 4–5 | Surface point (WGCCRE rotation). alt in **meters**, optional (default 0 = surface datum). |
| `format <f>` | 2 | `json` (default) \| `ephemeris_binary` \| `universal` (auto-detect) \| free text (rows/text extracts parse CSV/text bodies regardless) |
| `header <name> <value>` | 3 | HTTP header. Repeatable. Values cannot contain spaces (whitespace split). |
| `post_body <json>` | 2 | POST body (no spaces). Presence of post_body ⇒ POST. There is no `method` directive. |
| `force <name>` | 2 | Block force context for 3-token `field` annotations. em \| gravity \| acoustic \| seismic-body \| seismic-surface \| thermal \| diffusion \| advective \| electric |
| `map <arr>` | 2 | Iterate JSON array; each row an oscillator (data-carried lat/lon). |
| `cmap <arr>` | 2 | Celestial map; ICRS from ra/dec + plx/dist/z keys. |
| `rows <arr>` | 2 | Table rows (column-index or header-name lookup). |
| `flatten <arr> [geom] [epoch]` | 2–4 | Flatten nested array geometry. |
| `field <key> <force> <unit> <tau>` | 5 | **Scalar physical measurement.** Key = JSON path (dot-notation). Kernel = force default (see „Force → default kernel"). τ in seconds, must be > 0. |
| `field <key> <force> <unit> <tau> <kernel>` | 6 | Same, with explicit kernel name. |
| `field <key> <name> <kernel> <force> <unit> <tau> <absorption> <advection>` | 9 | Legacy long form. τ > 0 required. |
| `field <key> <identifier>` | 3 | Annotation inside map/cmap/rows after a `force` directive. τ = 0 → **never manifests** (τ-Gate). Documentation only. |
| `first/last/lastrow/objlast/path/deep/regex <key> <name> <kernel> <force> <unit> <tau> <absorption> <advection>` | 9 | Positioned scalar extract variants (same 9-token config). |
| `count <path> [name]` | 2–3 | Extracted but τ = 0 → never manifests unless paired with a τ-carrying `field` of the same name. |
| `geojson <mag_key> <min_mag> <out1> <out2> <tau> <absorption> <advection>` | 8 | GeoJSON event extract. |
| `cmrpolygon <arr> [epoch] [alt] [val]` | 2–5 | CMR polygon centroids. |
| `celestialpolygon <arr> <radius> [epoch] [val]` | 3–5 | Celestial polygon. |
| `keplermap <arr> [a] [e] [i]` | 2–5 | Kepler elements map. |
| `hapi <k=v>…` | ≥2 | HAPI parameters. Values manifest only when paired with a τ-carrying `field` of the same name. |
| `ephemeris <target>` / `vectors <target>` | 2 | Horizons ephemeris/state-vector extracts. |
| `lat <key>` / `lon <key>` | 2 | Row position keys (map). Fixed unit: deg. |
| `alt <key> [unit]` | 2–3 | Row altitude key (map). Unit: `m` (default) \| `km` \| `ft` \| `cm` \| `mm` \| `-m` \| `-km` (negative = depth). Absent `alt` directive → surface datum 0. |
| `epoch <key>` | 2 | Row epoch key (map). ISO string or unix seconds. Absent → fetch time. |
| `vel <key>` / `trk <key>` / `vr <key>` | 2 | Row motion keys (map): speed m/s, track deg, vertical rate m/s → SurfaceFlow. |
| `val <key>` | 2 | Restrict map fields to the named field. |
| `ra <key>` / `dec <key>` | 2 | ICRS deg (cmap). |
| `plx <key>` | 2 | Parallax, mas (cmap). |
| `pmra <key>` / `pmdec <key>` | 2 | Proper motion, mas/yr (cmap). |
| `radvel <key>` | 2 | Radial velocity key (cmap). |
| `dist <key>` | 2 | Distance key (cmap). |
| `z <key>` | 2 | Redshift key (cmap) — Hubble-flow distance z·c/H0 (z > 0, else row skipped). |
| `target <body>` | 2 | Target for ephemeris/vectors (`{target}` substitution). |
| `catalog <name>` | 2 | Catalog identifier (`{catalog}` substitution). |
| `max_freq <hz>` / `min_freq <hz>` | 2 | Frequency bounds (`{max_freq}`/`{min_freq}`). |
| `repeat ra <min> <max> <bins>` | 5 | RA-binned repeat (`{bin}`/`{repeat_bin}`). Short form: `repeat <bins>`. |
| `flux_from_mag <key>` | 2 | Derive flux 10^(−0.4·mag). Conflicts with abs_mag_from (block refused). |
| `abs_mag_from <key>` | 2 | Derive absolute magnitude. |
| `catalog_epoch <yr>` | 2 | Catalog reference epoch (proper-motion propagation). |
| `stations <url>` | 2 | Station list URL for `{nearest_station}`. |
| `stations_path/lat/lon/id <key>` | 2 | Station list keys (defaults: stations, lat, lng, id). |

### 1.1 URL template variables

Position/extent (from presence + frame): `{lat}` `{lon}` `{lat_int}` `{lon_int}`
`{lat_min}` `{lat_max}` `{lon_min}` `{lon_max}` `{grid}` `{grid_lat}` `{grid_lon}`
`{x}` `{y}` `{z}` `{nearest_station}` `{bin}` `{repeat_bin}`.
Time (from the Archivar clock, TDB→UTC): `{today}` `{yesterday}` `{tomorrow}`
`{today_nodashes}` `{yesterday_nodashes}` `{tomorrow_nodashes}` `{today_yyyymmdd}`
`{today_ymd}` `{today_plus_365}` `{t_start}` `{t_end}` `{now}` `{now_minus_1}`
`{now_minus_2}` `{hour_ago}` `{week_ago}` `{week_ago_nodashes}` `{year}` `{year2}`
`{month}` `{day}` `{yday}` `{hour}` `{minute}` `{unix_now}` `{unix_now_plus_3600}`.
Declared: `{target}` `{catalog}` `{max_freq}` `{min_freq}`.
Any remaining `{MARKER}` resolves from the environment (`.env` /
`.secrets.local`); an absent marker substitutes void and logs to stderr.

### 1.2 The τ-Gate

τ is the temporal decay constant of the measured process in seconds. It is
the price of manifestation: no τ, no oscillator. Estimation when the process
itself does not declare one:
- explicit knowledge of the process → use it
- rapidly changing data (ISS position, live weather) → ttl/10
- stable data (star catalogs, geology) → ttl

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

The force describes the propagation mechanism of the measured quantity.
IDs and names as implemented (`force_id_of` in `src/main.rs`):

| Force | ID | Propagation | What it covers |
|-------|----|-------------|----------------|
| `em` | 0 | c (light) | EM radiation (flux, irradiance), magnetic fields, radio, radar measurements, optical depth (AOD is EM extinction), lightning EM pulse |
| `gravity` | 1 | c (grav. wave) | Gravitational acceleration, tidal force, orbital velocity measured gravitationally, Earth gravity field, elevation (geopotential height) |
| `acoustic` | 2 | sound speed | Wave height/period, sound pressure, acoustic frequency, precipitation (rain/snow accumulation as acoustic displacement), infrasound |
| `seismic-body` | 3 | body wave vel. | Subsurface displacement, body wave velocity/acceleration, hypocentral depth, borehole measurements |
| `seismic-surface` | 4 | surface wave vel. | Surface displacement, ground shaking, surface wave amplitude, earthquake effects at surface, volcano deformation |
| `thermal` | 5 | thermal diffusivity | Temperature (any medium), heat flux, brightness temperature (IR), fire temperature |
| `diffusion` | 6 | diffusion coeff. | Gas concentration, aerosol mass, water chemistry, salinity, turbidity, dissolved oxygen, nutrients, humidity (water vapor diffusion), soil moisture, pCO2, CH4, O3, NO2 |
| `advective` | 7 | flow velocity | Wind speed/direction, ocean currents, river discharge, air pressure (advected air mass), water level (advective flow), streamflow, wave energy flux |
| `electric` | 8 | c (field propagation), dielectric relaxation in media | E-field strength, bioelectric potential, conductivity, telluric currents, lightning current, atmospheric potential gradient, battery voltage/current |

`biotic` is **not a force**. An organism sensing another organism does so
through a physical carrier: bioacoustics → `acoustic`, chemosignals →
`diffusion`, bioluminescence → `em`, bioelectric fields → `electric`. Legacy
blocks declaring `force biotic` are refused at load and are re-keyed to their
carrier mechanism during curation.

### 2.1 Force → default kernel (FORCE_KERNEL, as implemented)

| Force | Default kernel |
|-------|----------------|
| em, gravity | inverse-square |
| acoustic, seismic-body, seismic-surface, advective, electric | gaussian-inverse-square |
| thermal, diffusion | erfc |

Kernel names accepted by the 6-token `field` form: `inverse-square`,
`gaussian-inverse-square`, `gaussian-inverse`, `erfc`, `exponential-decay`,
`patch-levy`, `inverse-linear`.

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
`solar_lon`, `lat`, `lon` when extracted via a scalar `field` line — these are
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
- Orbiting satellites whose position is data-carried (map keys + `at sun`)

### `on <body> <lat> <lon> [alt]`
Fixed geodetic point rotating with the body's surface (WGCCRE rotation model).
Used for:
- Ground stations, buoys, weather stations (`on earth`)
- Fixed-frame observations (`on mars 14.0 90.0`, `on moon 0 0`)
- ADSB receivers, METAR stations
- The browser station declares its position via `on <body>` oscillators over WebSocket

### Data-carried position (map/cmap rows)

There is **no `body` directive and no `pos` directive** in the parser. A map
block with per-row lat/lon still declares its frame via `at <body>` or
`on <body> <lat> <lon> [alt]` — the frame's body is the rotation model applied
to the row coordinates, and the frame position gates fetching (presence gate)
and renders `{lat}`/`{lon}` templates. Rows carry their own position through
the `lat`/`lon`/`alt` (map) or `ra`/`dec`/`plx`/`dist` (cmap) key directives.
Legacy blocks written with `body earth` or `pos …` do not load — recurate them
with an explicit frame.

## 5. Map / CMap / Rows Blocks

```
url https://www.seismicportal.eu/fdsnws/event/1/query?format=json&limit=100&minmagnitude=2.5&orderby=time&start={today}T00:00:00
ttl 60
at earth
map features
lat geometry.coordinates.1
lon geometry.coordinates.0
field properties.depth quake_depth_km gaussian-inverse-square seismic-body km 3600 0.0 0.0
```

- `map <arr>` or `cmap <arr>` or `rows <arr>`: the array to iterate.
- `lat`, `lon`, `alt` (map) / `ra`, `dec`, `plx`, `pmra`, `pmdec`, `radvel`,
  `dist`, `z` (cmap): 2-token position keys. Units are fixed by physics:
  deg, deg, m / deg, deg, mas, mas/yr, mas/yr, (km/s via scale), (m via scale), (redshift).
- `field <key> <force> <unit> <tau>` (5-token, compact) or the 9-token long form
  `field <key> <name> <kernel> <force> <unit> <tau> <absorption> <advection>`
  after a map/cmap/rows directive attaches the field to that extract's rows.
  τ > 0 or nothing manifests. `phi/sources.φ` carries the 9-token form.
- `field <key> <identifier>` (3-token, after a `force` line): annotation only —
  τ = 0, the τ-Gate closes, no oscillator. Use it to document string metadata,
  never to extract measurements.
- Non-physical fields (IDs, names, timestamps) are dropped per RULE 3.

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

All examples use the 9-token `field` long form (`field <key> <name> <kernel>
<force> <unit> <tau> <absorption> <advection>`) — the form `phi/sources.φ`
carries today. The kernel is the force's default (§2.1); absorption and
advection are 0.0 unless the medium declares otherwise.

### ISS Position
```
url https://api.wheretheiss.at/v1/satellites/25544
ttl 10
at earth
map .
lat latitude
lon longitude
alt altitude km
field velocity iss_velocity_kmh inverse-square em km/h 1 0.0 0.0
field footprint iss_footprint_km inverse-square em km 1 0.0 0.0
```

API response: `{"latitude": -47.75, "longitude": 78.87, "altitude": 438.28, "velocity": 27528, "footprint": 4599, "units": "kilometers"}`. Single object → one row. `alt altitude km` scales the km value to meters. `velocity`: km/h (ISS API docs), force EM (radar Doppler tracking). τ = min(ttl/10, 1) — fast-moving data. No epoch key → the sample anchors at fetch time.

### ADS-B Aircraft
```
url https://api.adsb.lol/v2/point/{lat}/{lon}/250
ttl 10
on earth 52.5 13.4
map ac
lat lat
lon lon
field gs adsb_ground_speed_kmh gaussian-inverse-square advective km/h 1 0.0 0.0
field track adsb_track_deg inverse-square em deg 1 0.0 0.0
```

`hex`, `flight` dropped (IDs). `gs`: ground speed, force advective. `track`: deg, force em (transponder measurement). τ = 1 s (live traffic).

### NDBC Buoy
```
url https://www.ndbc.noaa.gov/data/realtime2/13002.txt
ttl 300
on earth 21.0 -23.0
rows .
field WDIR ndbc_wind_dir_deg gaussian-inverse-square advective deg 30 0.0 0.0
field WSPD ndbc_wind_speed_ms gaussian-inverse-square advective m/s 30 0.0 0.0
field WVHT ndbc_wave_height_m gaussian-inverse-square acoustic m 30 0.0 0.0
field DPD ndbc_dominant_period_s gaussian-inverse-square acoustic s 30 0.0 0.0
field APD ndbc_avg_period_s gaussian-inverse-square acoustic s 30 0.0 0.0
field MWD ndbc_mean_wave_dir_deg gaussian-inverse-square acoustic deg 30 0.0 0.0
field PRES ndbc_pressure_hpa gaussian-inverse-square advective hPa 30 0.0 0.0
field ATMP ndbc_air_temp_c erfc thermal C 30 0.0 0.0
field WTMP ndbc_water_temp_c erfc thermal C 30 0.0 0.0
field PTDY ndbc_pressure_tendency_hpa gaussian-inverse-square advective hPa 30 0.0 0.0
```

CSV header: `#YY MM DD hh mm WDIR WSPD GST WVHT DPD APD MWD PRES ATMP WTMP DEWP VIS PTDY TIDE`. Units from the NDBC header line. Wind → advective, waves → acoustic, temperature → thermal, pressure → advective. τ = ttl/10.

### GOES X-Ray Flux
```
url https://services.swpc.noaa.gov/json/goes/primary/xrays-1-day.json
ttl 60
at sun
field flux goes_xray_flux_wm2 inverse-square em W/m2 6 0.0 0.0
```

Array-of-objects; `flux` in W/m2 (GOES XRS product spec). Force EM. `time_tag`, `satellite`, `energy` dropped (timestamp, ID, metadata).

### Seismic Portal (GeoJSON)
```
url https://www.seismicportal.eu/fdsnws/event/1/query?format=json&limit=100&minmagnitude=2.5&orderby=time&start={today}T00:00:00
ttl 60
at earth
map features
lat geometry.coordinates.1
lon geometry.coordinates.0
field properties.depth quake_depth_km gaussian-inverse-square seismic-body km 3600 0.0 0.0
```

GeoJSON FeatureCollection. Surviving field: `depth` (km, seismic-body, τ = 1 h — a hypocenter persists). Dropped: `mag` (dimensionless), `evid` (ID), `time` (timestamp), `flynn_region` (string).

### Celestial Catalog (Fermi 4FGL)
```
url https://heasarc.gsfc.nasa.gov/xamin/vo/tap/sync?REQUEST=doQuery&LANG=ADQL&FORMAT=csv&QUERY=SELECT+*+FROM+fermi4fgl
ttl 604800
at sun
cmap .
ra ra
dec dec
plx default_plx
field flux_1_100_gev fermi_flux_1_100_gev_wm2 inverse-square em W/m2 604800 0.0 0.0
field energy_flux fermi_energy_flux_wm2 inverse-square em W/m2 604800 0.0 0.0
```

Position keys locate the source in ICRS. τ = ttl (stable catalog). `name`, `data_release`, `spectrum_type`, time fields — dropped.

## 8. Block Survival

A block survives the migration if it has at least one `field` line (5/6/9-token)
with a physical unit and τ > 0. Position alone is not enough — a point without a
measurement is not an oscillator.

Before dropping any block, research the API: does it return ANY numeric value
with a physical unit? Depth, temperature, pressure, concentration, velocity,
flux — if yes, keep it as a field measurement. If the API only returns metadata
(IDs, names, timestamps, counts) → DROP the entire block.

## 9. Migration Algorithm (revised)

```
for each block in the legacy corpus:
    url = block.url
    fetch live API response (CDN-first; the CI Archivar mirrors every response)
    keep: url, ttl, frame (at/on — REQUIRED, translate body/pos → at/on),
          map/cmap/rows, structural directives
    drop: block-level force (unless 3-token annotations remain)

    for each field / first / last / lastrow / path line:
        key = extract_key_name(line)
        if key not in API response → DROP
        if key in dropped categories (RULE 3) → DROP
        unit = determine_unit(api_response, key_name_pattern)
        if unit == None → DROP
        force = assign_force(key, unit, block_force_hint)
        tau = declared τ, else ttl/10 (fast data) or ttl (stable data)
        output: field <key> <force> <unit> <tau>

    for each field_in inside map/cmap/rows:
        same filtering; survivors become 5-token field lines after the
        map/cmap/rows directive (3-token annotations do not manifest)

    for each *_key directive:
        output: <keyword> <key>   (2-token; units are fixed by physics)

    at <body> <scale> → at <body>  (drop scale)
    pos <lat> <lon> [alt] [scale] → at/on frame + lat <lat> + lon <lon> [+ alt <alt>]
    on … alt: convert km → m (parser expects meters)

    if block has ≥1 manifesting field → write to phi/sources.φ
    else → write to phi/dead_sources.φ with the decline reason
```

## 10. Non-Goals & Known Parser Gaps

This spec defines the canonical DATA FORMAT as the parser accepts it today.
It does not define:
- Extract variant unification (`first`→`field`, `lastrow`→`field` — separate session)
- SI conversion functions (units are documentation slots; values pass through raw)
- Anomaly reporting

Directives that appear in legacy corpora but have **no parser arm** (writing
them produces nothing): `body`, `pos`, `method`, `source`, `field_in`,
`lat_key`/`lon_key`/`alt_key`, `extent`, `reach_ttl`, `note`, `tau`
(as key directive), `force biotic`. Open parser work: per-row τ override,
SI conversion for `field` values and `vel` (m/s fixed today). Map rows
without an `epoch` key anchor at fetch time; map rows without an `alt` key
anchor at the surface datum (0 m); single-object responses iterate as one
row.

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
