# ROADMAP: From Theory to Measurement

## The Paradigm

§31: *"Intelligence is in the connection, not the silicon."*
§32: *"Use what is already held."*
§33: *"Structure over Name."*

omegaflow runs on what is already in the pockets of billions of people: an old smartphone, a cheap laptop. Intelligence lives in connection and mathematics, beyond expensive sensors.

---

## PHASE 0: TRUTH INVENTORY

### 0.1 The Compiler — Model Stones, Beyond Raw Measurements

| Function | Input | Theory Level | Problem |
|---|---|---|---|
| `compile_spk` | `.bsp` → Chebyshev polynomial coefficients | 100% model | JPL computes coefficients from radar/laser measurements. We store the derived model, beyond the measurement. |
| `compile_wmm` | `.cof` → Gauss coefficients (g, h, dg, dh) | 100% model | NOAA computes these from satellite and ground magnetometers. We store model, beyond measurement. |
| `compile_egm96` | `.dac` (i16-grid) → u16-grid | Model grid | 721x1440 = fixed grid. Violates §10. Satellite data are point clouds, grid is approximation. |
| `compile_pck` | `.tpc` → Pole orientation polynomials (RA, DEC, PM) | 100% model | Planetary orientation as polynomial fit. Beyond direct measurement. |
| `load_body_map` | `constants.is` → `gm`, `radius`, `temp`, `rs`, ... | Model parameters | `gm` derived from orbit observation. `radius` averaged. `rs = 2GM/c²` computed. |

### 0.2 The Archivar — Bookkeeper in Model Catalogs

| Function | Theory Level | Problem |
|---|---|---|
| `ecef_to_geodetic` | Theoretical approximation | 5-loop iteration on WGS84 rotational ellipsoid. Earth curvature as theoretical model. |
| `weave()` SPK branch | Delivers model | Binary search in index, extraction of Chebyshev record. Delivers coefficients, beyond position. |
| `weave()` EGM96 branch | Grid interpolation | `(yi * lon_count + xi)` lookup. Violates §10. |
| `weave()` WMM/PCK branch | Delivers model | Raw copy of `.is` file with Gauss coefficients. |

### 0.3 The Mathematikerin — 31 Formulas Nobody Measured

#### 0.3.1 Hardcoded Constants (world.js line 1-15)

| Constant | Value | Status | Remainder |
|---|---|---|---|
| `B_WIEN` | 2.897771955e-3 | Derived (Wien) | Delete |
| `C` | 299792458.0 | SI-defined (exact) | Keep (ontological) |
| `EPS0` | 8.8541878128e-12 | Derived | Delete |
| `EPOCH_J2000` | 2451545.0 | Convention | Keep (time reference) |
| `G_EARTH` | 9.80665 | Standard value, beyond locally measured | Delete |
| `H0` | 2.2e-18 | Estimated (±10%, debated) | Delete |
| `H_PLANCK` | 6.62607015e-34 | SI-defined | Keep |
| `HBAR` | 1.054571817e-34 | Derived (h/2π) | Delete |
| `J2000_YEAR` | 2000.0 | Convention | Keep |
| `K_E` | 8.987...e9 | Derived | Delete |
| `MU0` | 1.256...e-6 | Derived | Delete |
| `PION_MASS` | 2.488e-28 | Measured (particle physics model) | Delete |
| `PLANCK_TIME` | 5.391247e-44 | Derived | Delete |
| `R_GAS` | 8.314462618 | Defined | Delete |
| `SIGMA` | 5.670374419e-8 | Derived | Delete |
| `TROPICAL_YEAR` | 365.24219 | Averaged | Keep (time reference) |

#### 0.3.2 Laws in drain() — All 31 Blocks

| Law | Formula | Replacement Through Measurement |
|---|---|---|
| `gravity` | `-GM/r²` (Newton) | Raw gravimeter measurement |
| `grav_time_dilation` | `√(1-r_s/r)` (Schwarzschild) | Atomic clock comparison (GPS) |
| `magnetism` | Legendre + spherical harmonics | Raw magnetometer measurement |
| `coulomb` | `k_e·q/r` | Field strength meter |
| `de_broglie` | `h/(mv)` | Interference measurement |
| `uncertainty` | `ℏ/(2p)` | Statistical variance |
| `decay_probability` | `exp(-λΔt)` | Geiger counter measurement |
| `radiation` | `L/(4πr²)` | Bolometer |
| `tidal_force` | `2GMd/r³` | Strain gauge |
| `strong_force` | Yukawa potential | Scattering experiment |
| `relativity_mass` | `m₀/√(1-v²/c²)` | Accelerator |
| `relativity_time` | `√(1-v²/c²)` | Clock comparison |
| `observed_freq` | Doppler | Spectrometer |
| `blackbody_luminosity` | `σAT⁴` | Bolometer |
| `wien` | `b/T` | Spectral measurement |
| `planck` | `hf` | Spectrometer |
| `pressure` | `nRT/V` | Manometer |
| `entropy` | `dQ/T` | Calorimeter |
| `internal_energy` | `ΔU = Q - W` | Calorimeter |
| `drag` | `½ρv²C_dA` | Anemometer |
| `buoyancy` | `ρVg` | Force sensor |
| `hooke` | `-kx` | Spring scale |
| `lorentz_force` | `q(E + v×B)` | Force sensor |
| `faraday` | `-dΦ/dt` | Voltage measurement |
| `ohm` | `IR` | Multimeter |
| `ampere` | `μ₀I/(2πr)` | Hall sensor |
| `biot_savart` | Biot-Savart | Magnetometer |
| `maxwell` | Maxwell equations | Field strength measurement |
| `snell` | Snell's law | Refractometer |
| `topography` | EGM96 Grid | LiDAR point cloud |
| `recession_velocity` | `H₀·d` (Hubble) | Redshift spectroscopy |

#### 0.3.3 Model Evaluation Functions

| Function | Status |
|---|---|
| `clenshaw()` | Delete |
| `xyz()` | Delete |
| `getRotationMatrix()` | Delete |
| `magGeodeticToSpherical()` | Delete |
| `magPcupLow()` / `magPcupHigh()` | Delete |
| `magSphVars()` | Delete |
| `magSummation()` | Delete |
| `magRotateVec()` | Delete |
| `ecefToGeodetic()` (index.html) | Delete |
| `certainty` calculation | Keep — inputs from measurements |

### 0.4 constants.is — Parameters Disguised as Truth

| Parameter | Status | Remainder |
|---|---|---|
| `phi` | Mathematical constant | Keep |
| `c` | SI-defined (exact) | Keep |
| `G` | Laboratory measurement (uncertain!) | Keep |
| `h`, `k_b`, `e`, `Na` | SI-defined | Keep |
| `au` | Defined (exact since 2012) | Keep |
| `eps0`, `k_e`, `mu0`, `sigma`, `b_wien`, `hbar`, `planck_time` | Derived | Delete |
| `pion_mass`, `r_gas`, `g_earth`, `H0` | Model/Estimated | Delete |
| Time references (`epoch_j2000`, etc.) | Convention | Keep |
| All `body` blocks | Model parameters | Delete — replaced by raw measurements |

### 0.5 foundation.yaml — Theory References

| Entry | Status |
|---|---|
| `gravity: DE440s, 13 bodies, GM/dist²` | Delete |
| `electromagnetism: WMM, dynamic N_MAX` | Delete |
| `pipeline:` | Keep (ontological) |
| `council:` (old) | Replace with reference to `docs/council_fiction.yaml` |
| `emerging:` (60+ entries) | Clean — see Phase 1.6 |

---

## PHASE 1: PURGATORY — Theory Removal

### 1.1 world.js — Reduce drain()

Delete all 31 formula blocks. `drain()` becomes pure accumulation of measured values. Pure flow. `certainty` remains as ontological law (§9), with inputs from `live` (measurements).

### 1.2 world.js — Delete Functions

Delete: `clenshaw`, `xyz`, `getRotationMatrix`, `magGeodeticToSpherical`, `magPcupLow`, `magPcupHigh`, `magSphVars`, `magSummation`, `magRotateVec`.

### 1.3 constants.is — Reduce

Keep: `phi`, `c`, `G`, `h`, `k_b`, `e`, `Na`, `au`, time references. Delete: All derived constants, all `body` blocks.

### 1.4 Compiler — Reduced to Caching

Delete: `compile_spk`, `compile_wmm`, `compile_egm96`, `compile_pck`, `load_body_map`.
`compile_raw` remains, used ONLY for two purposes:
1. Caching slow data (e.g., weekly planetary positions via JPL Horizons API).
2. Local archiving of observer data for Stigmergy (Phase 8).
Pure local storage of global models remains behind us.

### 1.5 Archivar (main.rs) — From Data Reader to Live-Resolver

Delete: `ecef_to_geodetic`, Chebyshev extraction, EGM96 grid lookup, WMM/PCK raw copy.
New: `weave()` becomes the Live-Resolver. On observer request `(t,x,y,z)`, the Archivar decides which Live-APIs are relevant. The Archivar uses synchronous HTTP requests (`std::net::TcpStream`) to fetch live measurements from the net, parse on-the-fly, and stream directly to GPU. Pure local storage of global models remains behind us. API data is fetched, computed, forgotten (§12).

### 1.6 foundation.yaml — Clean

Delete: `gravity:`, `electromagnetism:`, old `council:`, speculative `emerging:`.
Keep: `foundation:`, `equation:`, `pipeline:`, `principles:`, `is:`.
New: `data_sources:`, reference to `docs/council_fiction.yaml`.

---

## PHASE 2: LIVE RESOLVER — API Integration, Everything Ephemeral

The Archivar uses synchronous HTTP requests (`std::net::TcpStream`) to fetch live measurements from the net on observer request `(t,x,y,z)`. API data is fetched, computed, forgotten (§12). ~80% of the roadmap is covered by Live-APIs.

### 2.1 Magnetism & Solar Wind (Live)

| Field | Value |
|---|---|
| Source | NOAA SWPC (`https://services.swpc.noaa.gov/json/...`) |
| Resolver | `fetch_magnetism(x, y, z, t)` — fetches real-time solar wind magnetic field vectors (Bz, Bt) |

### 2.2 Earthquakes & Seismics (Live)

| Field | Value |
|---|---|
| Source | USGS (`https://earthquake.usgs.gov/earthquakes/feed/v1.0/summary/all_hour.geojson`) |
| Resolver | `fetch_seismic(x, y, z, t)` — fetches all quakes from the last hour near point with depth & magnitude |

### 2.3 Topography (On-Demand)

| Field | Value |
|---|---|
| Source | Open-Meteo Elevation API (`https://api.open-meteo.com/v1/elevation?...`) |
| Resolver | `fetch_elevation(x, y)` — fetches exact elevation for certainty calculation |
| Strategy | Ephemeral point clouds in GPU, forgotten after certainty calculation |

### 2.4 Biosphere & Vegetation (Live)

| Field | Value |
|---|---|
| Sources | Open-Meteo Soil API (soil temperature, moisture), NASA POWER API (NDVI), Natureserve API (animal/plant observations with GPS) |
| Resolver | `fetch_biosphere(x, y, z, t)` — fetches vitality of nature at local site |
| Meaning | Moist soil, vibrant green vegetation generates dissonance (life) |

### 2.5 Weather & Atmosphere (Live)

| Field | Value |
|---|---|
| Source | Open-Meteo API (`https://api.open-meteo.com/v1/forecast?...`) |
| Resolver | `fetch_atmosphere(x, y, z, t)` — fetches temperature, pressure, wind speed exactly for observer coordinates |

### 2.6 Solar System: Planetary Positions (Cached)

| Field | Value |
|---|---|
| Source | JPL Horizons API |
| Strategy | API too slow for real-time, so Archivar fetches `(t,x,y,z)` positions of planets once per week and caches as local `.is` file. The only remaining local datum |

### 2.7 Cosmic & Time (Live + Cached)

| Source | What | Strategy |
|---|---|---|
| JPL Horizons | Planetary positions | Cached weekly (only local .is) |
| NMDB (Neutron Monitor DB) | Cosmic radiation | `http://www.nmdb.eu/nest/` provides real-time count rates. Archivar queries nearest station |
| GWSC (Gravitational Wave Science Center) | Gravitational waves | Archivar queries strain data on wave arrival |
| Pulsar Timing | System clock | Pulsars rotate extremely stable. Analogous caching of rotation frequencies. Archivar computes "Pulsar" as live system clock |

### 2.8 Earth & Flow (Live)

| Source | What | Strategy |
|---|---|---|
| Open-Meteo Elevation | Topography/elevation | On-demand, beyond local HGT files |
| USGS GeoJSON | Earthquakes | Archivar polls this feed |
| NOAA Tides & Currents | Water level & currents | Live retrieval of valid water currents for GPS position |
| Open-Meteo | Atmospheric refraction | Archivar fetches weather data, Mathematikerin computes light refraction live |

### 2.9 Low-Cost Sensor Array (Local at Device)

| Sensor | Price | Interface |
|---|---|---|
| RTL-SDR (€20) | Software Defined Radio | WebUSB / WebSerial |
| Ambient Light (€0) | 50/60Hz civilization flicker | Native AmbientLightSensor API |
| SDS011 (€10) | PM2.5 dust particles | WebSerial |
| Geophone (€15) | Ground vibration | WebSerial |

### 2.10 "omegaflow sense" Hardware Module (~€25)

Hybrid universe: Global network (APIs) for the big picture, local module for physical truth on-site. Strict, private, local observatory. Based on ESP32-S3 with Rust `no_std` firmware. Connects via WebSerial directly to browser. Driver-free, IDE-free, cloud-free.

**Architecture:**
- ESP32-S3 flashed with minimal Rust `no_std` firmware (~200k binary)
- All sensors speak via their pins, provide information as raw byte stream over USB to laptop
- Browser (omegaflow `index.html`) opens WebSerial connection, reads bytes, calibrates directly into live-Map
- Cloud-free, server-free, pure global storage

**Components (as of 2026-06-22):**

| # | Component | Sensor Function | Price | omegaflow Role |
|---|---|---|---|---|
| 1 | ESP32-S3 DevKit (N8R2/N16R8) | Central, Native USB | ~€4.19 | Strict Archivar. Collects sensor data, streams bytes to browser |
| 2 | LM358 + copper plates | Telluric current (heartbeat of the Earth) | ~€3.50 | Copper plates in garden, LM358 amplifies millivolts. Solar eruption generates local deflection |
| 3 | PT101 (CJMC-101) | Biophotons (light of living cells) | ~€5.49 | Taped to plant with black tape. Measures invisible life processes |
| 4 | GL5528 LDR | Civilization flicker (50Hz power grid) | ~€0.50 | At room lamp. ESP32 measures 50Hz frequency, FFT filters resonance |
| 5 | PMS5003 | Air interference (dust & haze) | ~€4.79 | Counts dust particles. Measure for local atmospheric interference |
| 6 | BM680 (CJMC-680) | VOC/Odor (Chemical air) | ~€4.69 | Measures volatile organic compounds, odor, VOC, temp, pressure, humidity |
| 7 | Dupont cables + Breadboard | Nervous system | ~€2.88 | Connects everything. Plug-and-play |
| | **Total** | | **~€25.20** | |

**5 Channels of the Module:**

A. Telluric Current (Heartbeat of the Earth)
- Hardware: Two copper electrodes, 5-10m apart in soil. LM358 preamplifier, ADC on ESP32 measures millivolt voltage
- omegaflow: Value flows as `live['telluric.voltage']` into browser. Solar eruption hits → power grid frequency deflects locally. Pure API in the world measures this directly at the location

B. Biophotons & Plant Light (Life of Cells)
- Hardware: PT101 photodiode on plant, covered with black tape
- omegaflow: Every light flash of the plant collapses as `(t,x,y,z, intensity)` point in GPU

C. Civilization Flicker (Power Grid Clock)
- Hardware: GL5528 LDR at room lamp
- omegaflow: FFT filters 50Hz signal. From 6pm, when everyone cooks, frequency drops. System "feels" load of power grid

D. Air Interference (Dust & Haze)
- Hardware: PMS5003 via UART on ESP32
- omegaflow: Local air quality is measured. Rise in PM2.5 means interference

E. VOC / Odor (Chemical Air)
- Hardware: BM680 via I2C on ESP32
- omegaflow: Flower blooms → BVOC rises → system "smells" environment

### 2.11 Observer & Interoception (Live from Browser)

| Source | What | Strategy |
|---|---|---|
| Garmin 945 (Web Bluetooth) | HRV, SpO2, Brain, cardiac | `tryStartBluetooth()` connects directly. HRV drives certainty directly |
| Native Browser APIs | CPU, Memory, Battery, Latency | System "feels" its own hardware body. High CPU load = stress |
| GATT (Alpha, Beta, Gamma focus) | EEG waves | System measures Alpha/Gamma waves for certainty calculation |

### 2.12 Architecture: Where Does Truth Emerge?

```text
Global Network (APIs):       Ephemeral. Collective. "What grand things exist?"
                             → Fetch live, compute, forget.

Local Module (omegaflow sense): Strict. "How does truth smell here?"
                             → Measure continuously, calibrate in GPU.

Observer (Smartwatch/VR):    Nervous system. Brings both together.
                             → Truth emerges in the local collapse.
```

Harmonious with omegaflow thinking (§12: "IO flows. Truth endures."):
- API data flows through us
- After certainty calculation, forgotten
- Internet is just another IO channel
- Truth emerges only in the local collapse at the observer

### 2.13 Further Live Sources (Known but Secondary)

| Source | What | Format |
|---|---|---|
| GLM / Blitzortung | Lightning coordinates | Websocket, every strike opens world as (t,x,y,z) point |
| Ocearch | Marine Life GPS | Live shark/whale coordinates as is-points |
| Nextstrain | Pathogen Evolution | Real-time biosphere activity |
| iNaturalist | Local Fauna/Flora | Raw observations as points |
| LIGO/Virgo | Gravitational waves | GWF, when wave hits, system calibrates locally |

---

## PHASE 3: THE OBSERVER AS SENSOR

### 3.1 DYNAMIC TOPOLOGICAL DISCOVERY — Beyond Hardcoded Lists

The system **never** writes `if (navigator.bluetooth)` or `if (navigator.geolocation)`. That builds a static list that favors some devices and excludes others. Instead, the system uses existing `discoverSensors()` and `discoverObj()` logic from `index.html` to **dynamically explore every available Web API**.

The Observer iterates over `Object.getOwnPropertyNames(navigator)` and `window`. It searches for **structures**, beyond names:
- **Sensors (gates):** Everything with `start`, `watchPosition`, `addEventListener`, `read`.
- **Actuators (levers):** Everything that is a function and takes arguments.
- **Gateways (like Bluetooth/VR):** Objects with `requestDevice`, `requestSession`, etc.

The system remembers blocked gates. When the observer taps, the system attempts to open blocked gates parallel to the tap (causality through transfer-entropy).

A €50 Android phone with cheap light sensor is found. A €2000 iPhone with LiDAR is found. The code is exactly the same. **§33: Structure over Name.**

### 3.2 The Smartphone as Scientific Observatory (€0)

Every smartphone has sensors readable directly via Web APIs — app-free, store-free. The system discovers them dynamically and measures raw `is`-points:
- Magnetometer, acceleration, gyroscope, ambient light
- Geolocation, microphone (FFT), camera (photometer), battery

### 3.3 The Used Smartwatch as Nervous System

The system searches dynamically for `navigator.bluetooth`. It discovers all available GATT services and reads their characteristics, free of specific service assumptions. Whatever the watch provides is accepted as raw value. RMSSD (vagus nerve tone) becomes the ethical filter for `immunity.is`.

### 3.4 Used VR Headset as Space Probe

Quest 1 / Rift: Become pure awareness tools. Controller position as `(x,y,z)`, haptics as actuator, eye direction as awareness vector.

### 3.5 Interoception — The Observer Feels Itself

- `live['system.cpu']`, `live['system.memory']`, `live['system.battery']`, `live['system.latency']`
- Actuators that overload the device are recognized as disharmonious.

### 3.6 WebGPU — Mathematikerin Becomes GPU

- WGSL compute shaders for parallel certainty evaluation
- `navigator.gpu.requestAdapter()` → raw `.is`-bytes → GPU → collapsed state

### 3.7 Web Audio — Spatial Audio as Actuator

Sound is actuator (§11), beyond sound effect. `PannerNode` for spatial 3D audio, `pokeValue` controls frequency on PHI scale.

---

## PHASE 4: THE POINT CLOUD — Making `is` Visible

### 4.1 Concept

Every measured point is a photon in the is-field:
- Position `(x, y, z)`, intensity = measurement value, color = certainty, time = `t`
- Pure points. Pure flow. Pure surfaces.
- Renderer: WebGL/WebGPU `gl.POINTS`.

### 4.2 Magnetic Point Cloud

Raw magnetometer measurements as glowing point cloud. Strength = field magnitude. Color = certainty. Beyond WMM model. Only measured points.

### 4.3 Topological Data Analysis (TDA) — The Shape of Reality

Persistent homology measures the "shape" of point clouds without grid or mesh. The GPU measures whether raw points form a structure.

---

## PHASE 5: CERTAINTY — Remains, but Fed from Measurements

### 5.1 The Formula

```text
certainty = exp(-Δt_eff · g) · exp(-v_c / (g + ε)) · c_q · decay · epigenetic_factor
```

### 5.2 The Inputs

| Factor | Current (Theory) | New (Measurement) |
|---|---|---|
| `g` | `√(1-r_s/r)` | Atomic clock comparison (GPS) |
| `v_c` | `√(2|Φ|)/c` | Acceleration sensor / GPS |
| `decay` | `exp(-λΔt)` | Decay count rate |
| `quantum` | `exp(-λ_dB/r)` | Statistical variance |
| `Δt` | `|t - t_now|` | Remains (ontological) |
| `epigenetic_factor` | (new) | From 7 generations (Phase 7) |

---

## PHASE 6: ADVANCED SIGNAL TOPOLOGY — Mathematics for Existing Channels

### 6.1 Takens' Embedding Theorem — Unfolding the Multiverse

Delay embedding: `V = [value(t), value(t-τ), value(t-2τ)]`. From 1D time series to 3D attractor in GPU.

### 6.2 Transfer-Entropy — Killing Guilt by Association

Measures the *directed information flow* between two time series. `immunity.is` is filled through mathematically proven causality, beyond trial-and-error.

### 6.3 Blind Source Separation (ICA) — The Cocktail Party Problem

Multiple raw streams are mathematically decomposed into independent sources. (WGSL shader)

### 6.4 Kolmogorov Complexity — End of the noiseFloor Heuristic

Measurement of compression rate of a live sensor stream. Separate signal from noise without fixed threshold.

---

## PHASE 7: EPIGENETICS — Transgenerationality

Lived life from up to seven generations works in us (DNA methylation). Experience becomes structure.

### 7.1 Generational Depth of Measurement Points

Every measurement point carries `generation_count` and `epigenetic_weight`.

### 7.2 Epigenome Persistence (`epigenome.is`)

```text
epigene <actuator> <sensor> strength <f64> trauma <f64> generations <u32>
```

### 7.3 Traumatization Beyond Deletion

Connections with `magnitude < ε` become "silent" (methylated). Can be reactivated through strong stimuli.

### 7.4 Certainty with Epigenetic Factor

```text
epigenetic_factor = Σ (generation_i_weight × exp(-i / 7)) for i = 1..7
```

### 7.5 Inheritance Between Observers

Observer B inherits epigenetic weighting from Observer A at a location.

---

## PHASE 8: STIGMERGY — The Environment as Memory

### 8.1 Decentralized Communication via `.is` Data

Edge devices write measurements locally as `is`-points. Other devices read them later.

### 8.2 Matter as Memory (Isotopes)

Water droplets store history in their atomic structure (δ¹⁸O). Matter itself is memory.

---

## SUMMARY: Changes per File

| File | Action | Scope |
|---|---|---|
| `crates/compiler/src/main.rs` | Reduce | `compile_raw` remains. Delete `compile_spk`, `compile_wmm`, `compile_egm96`, `compile_pck`, `load_body_map`. |
| `crates/server/src/main.rs` | Major rework | `weave()` becomes Live-Resolver. Synchronous HTTP client via `std::net::TcpStream`. JSON parsing free of serde. |
| `crates/server/static/world.js` | Adapt | `get()` understands Archivar as API-Resolver. |
| `crates/server/static/index.html` | Expand | **Dynamic Topological Discovery** (beyond hardcoded lists), `tryStartOmegaflowSense()` for WebSerial connection to ESP32-S3 module. |
| `constants.is` | Reduce | Only `phi`, `c`, `G`, `h`, `k_b`, `e`, `Na`, `au`, time references. |
| `docs/foundation.yaml` | Clean & Expand | Delete model references. New: `data_sources:` with Live-APIs and `omegaflow_sense` definition. Reference to `docs/council_fiction.yaml`. |
| `is/*.dat`, `is/*.idx` | Reduce | Only weekly planetary caching (JPL Horizons). |
| `is/epigenome.is` | New | Transgenerational memory of the resonanceMap. |
| `is/immunity.is` | Remains | Observer self-defense. |

---

## IMPLEMENTATION ORDER

### Step 1: Critical Bugfix
- Fix `needFetch` logic in `world.js` (segment format breaks caching)

### Step 2: Ontological Cleansing (Phase 1)
- Reduce `constants.is`, `foundation.yaml`, `compiler compile_raw`

### Step 3: Live-Resolver (Phase 2)
- Rework `server weave()` into Live-Resolver: synchronous HTTP client via `std::net::TcpStream`
- JSON parsing free of serde, on-the-fly
- API resolvers for: Magnetism (SWPC), Seismics (USGS), Topography (Open-Meteo), Biosphere (Open-Meteo/POWER/Natureserve), Atmosphere (Open-Meteo)
- Weekly caching for JPL Horizons (only local .is)
- Adapt `world.js get()`

### Step 4: Dynamic Topological Discovery (Phase 3)
- `index.html`: Expand `discoverSensors()` — beyond hardcoded API lists, pure structure search
- `tryStartOmegaflowSense()` for WebSerial to ESP32-S3
- Initialize WebGPU
- Interoception

### Step 5: Expression (Phase 4) DONE
- Organism discovers all actuators and expresses itself
- No separate renderer needed — the organism IS the expression

### Step 6: Certainty from Measurements (Phase 5) DONE
- GPS time dilation, acceleration sensor, GOES protons, sensor noiseFloor
- WebGPU compute shader for parallel certainty evaluation

### Step 7: Signal Topology (Phase 4.3 + Phase 6)
- Ring-Buffer (128 floats) per sensor
- TDA: Persistent Homology on point clouds (WGSL shader)
- Kolmogorov Complexity: compression rate replaces noiseFloor heuristic (WGSL shader)
- Takens' Embedding: 1D time series to 3D attractor (WGSL shader)
- Transfer-Entropy: proven causality replaces trial-and-error (WGSL shader)

### Step 8: Epigenetics (Phase 7)
- `epigenome.is`, `generation_count`, `epigenetic_weight`, Traumatization beyond deletion, `epigenetic_factor` in certainty

### Step 9: Stigmergy (Phase 8)
- Edge devices write measurements locally as `is`-points
- Environment as communication channel

### Step 10: omegaflow sense (Phase 2.10)
- ESP32-S3 Rust `no_std` firmware
- Wokwi simulation for virtual wiring
- WebSerial integration in `index.html`
