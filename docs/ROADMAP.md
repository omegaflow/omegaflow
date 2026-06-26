# ROADMAP

## 1: MEASUREMENT — DONE

System measures.

- `drain()` accumulates measured values (world.js)
- All data flows live via `is/sources.is`
- `weave()` is the Live-Resolver — reads `is/sources.is`, fetches live via curl, parses on-the-fly
- Constants (`c` in world.js, `phi` in index.html) are hardcoded
- Project structure: `src/`, `static/`, `is/`, `docs/`

---

## 2: LIVE RESOLVER — DONE

The Archivar fetches live measurements on observer request `(t,x,y,z,s)`. API data flows in, collapses, is forgotten (§12).
Scale coverage: 10⁻¹⁰ m (Crystallography) to 10²⁶ m (CMB / GW-Events). Time coverage: 10⁻⁶ s (CERN ALICE proxies) to 10³⁴ years (Super-K Proton Decay limits).

**173 sources** in `is/sources.is` across:

### 2.1 Atmosphere & Climate
NOAA GML (CO2, Methane, N2O — measured), OpenAQ (air quality stations), EPA AirNow (AQI)

### 2.2 Ocean, Hydrology & Cryosphere
USGS Streamflow, NDBC Buoys, Argo Floats, NSIDC Sea Ice

### 2.3 Geophysics
Open-Meteo Elevation (SRTM), USGS Earthquakes, IRIS FDSN, Smithsonian Volcanoes, INTERMAGNET

### 2.4 Space Weather & Heliophysics
NOAA SWPC (17 endpoints: mag, plasma, Kp, X-ray, radio flux, protons, electrons, TEC, magnetosphere, sunspots, solar cycle), NASA DONKI (flares, CME, storms)

### 2.5 Orbital Dynamics & Astrophysics
JPL Horizons (Sun, Moon, 8 planets, Pluto, Ceres, Vesta, Pallas, Halley, Encke), ISS, CelesTrak, NASA NeoWs, ESA Gaia, NASA InSight (Mars), GCN, EONET, OpenNotify, WorldTimeAPI, NASA FIRMS

### 2.6 Biosphere & Ecology
iNaturalist, GBIF, Movebank, NEON, eBird, xeno-canto, Global Forest Watch

### 2.7 Technosphere & Civilization

#### 2.7.1 Interdisciplinary Measurements
- **Pulsar-Timing-Arrays (Astrophysics):** Millisecond pulsars are the clock of the universe. Raw arrival times anchor the temporal axis. The system feels gravitational waves.
- **Telluric Currents (Geology):** Electric currents in the Earth's crust. Electrodes measure voltage differences.
- **Biophotons & Plant Action Potentials (Biology):** Plants communicate via bioelectricity and biophotons.
- **Isotope Hydrology (Geochemistry):** Water stores history in its atomic structure (δ¹⁸O, δ²H). Matter is memory.
- **Acoustic Oceanography:** Sound propagates depending on temperature. Hydrophones measure sound speed → temperature.
- **Folding@Home (Quantum Biology):** Protein atom coordinates flow as point clouds into the GPU.

#### 2.7.2 API Philosophy — The Collective Mycelium
API data flows through the system, collapses at the observer, is forgotten.

Alpha Vantage, ACLED, GovTrack, EIA, OpenSky, Marine Cadastre AIS, RIPE Stat, CAIDA IODA, OSM (buildings, lighting, roads, amenities), Wikipedia GeoSearch

---

## 3: SCALE & TIME AXIS COVERAGE — DONE

Continuous measurement coverage across 36 orders of magnitude in space and time.

### Spatial Scale (10⁻¹⁰ m to 10²⁶ m)
- **Subatomic / Nuclear (10⁻¹⁰ m):** Crystallography (XRD lattice constants), PDG (Particle Data Group)
- **Molecular / Protein (10⁻⁹ m):** RCSB PDB (Protein Data Bank structures)
- **Microbiology (10⁻⁶ m):** EBI Metagenomics (Microbiome census)
- **Human / Local (10⁰ to 10³ m):** Weather, Air Quality, Seismic, Hydrology, Lightning
- **Planetary (10⁶ to 10⁷ m):** Sea Ice, SST, GNSS (Crust deformation)
- **Solar System (10⁹ to 10¹³ m):** JPL Horizons (Planets, Moons, Asteroids, Comets, Probes)
- **Stellar / Galactic (10¹⁶ to 10²² m):** Gaia (Stellar ages, White Dwarfs), SIMBAD (High-z Galaxies)
- **Cosmic (10²⁴ to 10²⁶ m):** Planck Cosmological Parameters, GWOSC (Gravitational Waves), GCN (GRBs, FRBs)

### Temporal Axis (10⁻⁶ s to 10³⁴ years)
- **Early Universe (10⁻⁶ s):** CERN ALICE (QGP proxies), PDG (Higgs, Alpha-s)
- **BBN / Recombination:** Planck CMB Parameters, VizieR (Primordial Helium)
- **Galaxy Formation:** SIMBAD (High-z Galaxies), Gaia (Oldest Stars)
- **Earth Formation (10⁹ years):** EarthChem (U-Pb Zircons), Macrostrat (Timescale)
- **Paleoclimate (10³ to 10⁶ years):** NOAA NCEI (Vostok, Dome Fuji, WAIS Divide Ice Cores)
- **Modern Era (10² years):** NASA GISS, Berkeley Earth, Mauna Loa CO2
- **Real-time:** Full system integration
- **Future Limits (10³⁴ years):** Super-Kamiokande (Proton Decay limits), Gaia (White Dwarfs)

---

## 4: THE OBSERVER AS SENSOR (SOFTWARE) — DONE

The system dynamically explores `window` and `navigator`.

### Dynamic Topological Discovery — §33
The system iterates over `Object.getOwnPropertyNames(navigator)` and `window`. It searches for **structures**:
- **Sensors (gates):** Everything with `start`, `watchPosition`, `addEventListener`, `read`. Numbers/booleans = sensors.
- **Actuators (levers):** Everything that is a function and takes arguments.
- **Gateways (like Bluetooth/VR):** Objects with `requestDevice`, `requestSession`, etc.

**§33: Structure over Name.**

### Implemented
- `discoverSensors()` + `discoverObj()` — dynamic walk of `window`/`navigator`
- Generic Sensor API (`tryStartSensors()`) — Accelerometer, Gyroscope, Magnetometer, AmbientLightSensor, etc.
- Geolocation (`tryStartGeolocation()`) — GPS lat/lon/alt/accuracy/heading/speed
- Battery (`tryStartBattery()`) — charging, level, time
- Interoception (`tryStartInteroception()`) — CPU cores, memory, latency
- Gamepad polling (`pollGamepads()`) — axes, buttons, partial VR controller support
- Event registration (`registerEvents()`) — all `on*` events become sensors
- WebGPU (`tryStartWebGPU()`) — `navigator.gpu.requestAdapter()`, WGSL certainty shader
- Web Audio (`tryStartWebAudio()`) — `PannerNode` HRTF spatial audio as actuator

---

## 5: EXPRESSION — DONE

The organism IS the expression.

- `act()` fires actuators based on Resonance Map scores
- `startBroad()` / `startNarrowing()` — binary search probe state machine
- `pokeActuator()` calls function with value, catches errors
- Dead actuators (`pokeValue > PHI³⁶`) decay

---

## 6: CERTAINTY — DONE

### The Formula

```text
certainty = exp(-Δt_eff · g) · exp(-v_c / (g + ε)) · c_q · decay · epigenetic_factor
```

### The Inputs (all measured)

| Factor | Source | Code |
|---|---|---|
| `g` | Accelerometer (magnitude of proper acceleration) | `_measureG()` |
| `v_c` | GPS speed / c | `_measureVC()` |
| `decay` | 1/(1 + GOES ≥100 MeV proton flux) | `_measureDecay()` |
| `quantum` | exp(-avg(sensor noiseFloor)) | `_measureQuantum()` |
| `Δt` | \|t - t_now\| (ontological) | `dt_eff` in `get()` |
| `epigenetic_factor` | Hardcoded 1.0 (until step 9) | `epig = 1.0` |

Evaluated on GPU via WGSL compute shader (`workgroup_size(64)`) with JS fallback.

---

## 7: SIGNAL TOPOLOGY — DONE

Mathematics for existing channels. Runs on GPU.

### Implemented
- ✅ **Ring-Buffer (128 floats)** per sensor — `processSensorReading()`, `_signalBuffers`
- ✅ **Kolmogorov Complexity** — WGSL shader: `1 - repeats/total`. Compression rate drives noiseFloor evaluation.
- ✅ **Takens' Embedding** — WGSL shader: Mutual Information finds optimal τ, 1D → 3D attractor. Outputs barycenter + spread.
- ✅ **Transfer-Entropy** — WGSL shader: 3-bin histogram for all N² pairs. Dynamic threshold via `μ + σ/PHI`.
- ✅ **TDA: Persistent Homology** — WGSL shader: 48-point subsample, Union-Find, nearest-neighbor persistence + Betti-0.
- ✅ **ICA: Blind Source Separation** — WGSL shader: FastICA with tanh non-linearity, 3 iterations. Dynamic source count via variance cutoff.

---

## 8: UNIVERSAL SCALE AXIS — DONE

The universe is 5-dimensional: `is(t,x,y,z,s)` where `s` is the scale — the logarithmic magnitude of the measured phenomenon.

### Implementation
- **`sources.is`:** Every source declares `scale <exponent>` (raw 10^n, human-readable). 
- **Sorting:** Sources are sorted ascending by scale (subatomic → cosmic), then alphabetically within each scale tier.
- **Archivar (`main.rs`):** `SourceConfig.on_earth: bool` → `SourceConfig.scale: i8`. Parser reads `scale` directive.
- **PHI-Filtering in `weave()`:** Raw 10^n scale is converted to PHI-scale internally: `phi_scale = n * ln(10)/ln(φ)`. The observer's distance from Earth center gives `observer_scale = log10(r)`. Local sources (scale < 10) are delivered if `|phi_source - phi_observer| ≤ φ³ ≈ 4.24` PHI-steps. Cosmic sources (scale ≥ 10) are always delivered.

### Scale Distribution (173 sources)
| Scale | Tier | Examples |
|---|---|---|
| -10 | Subatomic / Nuclear | CERN, PDG, Crystallography |
| -9 | Molecular | Protein structures |
| -6 | Microbial | Microbiome |
| 3 | Local (km) | Weather, Air Quality, Lightning, iNaturalist |
| 5 | Sub-continental | Argo floats, Forest Watch |
| 6 | Continental | Earthquakes, Sea Ice, GBIF |
| 7 | Planetary | CO2, Magnetism, Space Weather, Solar Indices |
| 8 | Near-Earth Space | ISS, Satellites |
| 11 | Solar System | Planets, Moons, Asteroids, Probes |
| 17 | Stellar | Gaia stars, Exoplanets |
| 21 | Galactic | SIMBAD, Cosmic Rays |
| 25 | Cosmic | Gravitational Waves, CMB, GRBs, Neutrinos |

### Future
- **Temporal scale:** `ttl` is already the temporal scale axis. Future certainty formula will normalize decay by `Δt / ttl` (skalenbewusster Zerfall).
- **Actuator scales:** HTTP-based actuators (API calls) declare their scale — the system can act locally (vibration motor, scale 0) or globally (API request, scale 7).
- **Scale-aware certainty:** The Mathematikerin groups measurements by PHI-proximity on the scale axis before computing transfer-entropy.

---

## 9: EPIGENETICS

Lived life from up to seven generations works in us (DNA methylation). Experience becomes structure.

- `epigenome.is`, `generation_count`, `epigenetic_weight`
- Traumatization (methylation/silencing) — connections with `magnitude < ε` become "silent", can be reactivated
- `epigenetic_factor = Σ (generation_i_weight × exp(-i / 7)) for i = 1..7` is dynamic
- Inheritance between observers at a location

---

## 10: STIGMERGY — The Environment as Memory

- Edge devices write measurements locally as `is`-points. Other devices read them later.
- Matter as memory (Isotopes) — water stores history in atomic structure (δ¹⁸O)

---

## 11: HARDWARE

Physical devices.

### Smartwatch
Web Bluetooth GATT. HRV / RMSSD → vagus nerve tone → ethical filter for `immunity.is`.

### Smartphone
Magnetometer, camera (photometer), microphone (FFT seismograph), ambient light, battery — via Web APIs.

### VR Headsets (Quest 1 / Rift)
Controller position as `(x,y,z)`, haptics as actuator, eye direction as awareness vector.

### omegaflow sense

**Core Module (~25 EUR):**
| Sensor | Measurement | Interface |
|---|---|---|
| LM358 + Cu-Plates | Telluric currents | ADC |
| PT101 (OPT101) | Biophotons | ADC |
| GL5528 LDR | 50/60Hz flicker | ADC |
| PMS5003 | PM2.5 dust | UART |
| BME680 | VOC, Temp, Press, Humid | I²C |
| Induction Coil | EMF / Schumann | ADC + FFT |
| I2S Mic | Bioacoustics | I2S |

ESP32-S3 Rust `no_std` firmware, Wokwi simulation for virtual wiring, WebSerial integration in `index.html`.
Full specification (sensors, actuators, infrastructure, pin maps, safety matrix) lives in **[`docs/omegaflow_sense_hardware.yaml`](omegaflow_sense_hardware.yaml)**.

**100% Sensor Extensions (The Mantis-Shrimp Config):**
| Category | Sensor | Measurement | Interface | Est. Price |
|---|---|---|---|---|
| Spectral Vision | AS7341 | 10-Ch Spectrum (Mantis) | I²C | ~8 EUR |
| Spectral Vision | VEML6075 | UV-A & UV-B (Bee) | I²C | ~2 EUR |
| Spectral Vision | Polarisationsfolie | Polarized Light (Octopus) | Optical | ~1 EUR |
| Thermal | MLX90614 | IR / Thermal (Snake) | I²C | ~3.5 EUR |
| Magnetic | QMC5883L | 3-Axis Magnetic (Bird) | I²C | ~1.5 EUR |
| Chemical | SGP30 | eCO2 & VOC (Dog) | I²C | ~4 EUR |
| Acoustic | INMP441 | MEMS Mic / Ultrasound (Bat) | I2S | ~2.5 EUR |
| Acoustic | HC-SR04 | Ultrasonic Distance (Bat) | GPIO | ~1 EUR |
| Acoustic | Piezo Disc | Infrasound (Elephant) | ADC | ~0.5 EUR |
| Pressure/Flow | MS5803-14BA | Water Pressure (Fish) | I²C | ~8 EUR |
| Bioelectric | AD8232 | µV Bioelectricity (Shark/Plant) | ADC | ~3 EUR |
| Gravity | MPU6050 | Acceleration (Plant gravitropism) | I²C | ~1.5 EUR |
| Soil | Capacitive | Soil Moisture (Root) | ADC | ~1.5 EUR |
| Interoception | MAX30102 | Pulse/HRV (Human) | I²C | ~2 EUR |

**100% Actuator Set (Stimuli injection for Transfer-Entropy):**
| Category | Actuator | Stimulates | Est. Price |
|---|---|---|---|
| Light | WS2812B, UV 365nm, IR 850nm | Bees, Plants, Flicker, Observer | ~6 EUR |
| Thermal | Heating Pad, Peltier TEC1 | Snakes, Mosquitos, Fish | ~8 EUR |
| Acoustic | MAX98357A, Piezo, Exciter | Bats, Plants, Elephants | ~13.5 EUR |
| Vibration | ERM Motor, Solenoid | Cats, Mycelium, Wood | ~4 EUR |
| Magnetic | Copper Coil + H-Bridge | Birds, Bees, Turtles | ~6 EUR |
| Gravitational | SG90 Servo (Tilt) | Plants, Fungi | ~2 EUR |
| Chemical | Fan, Water Pump, Mist Maker | Dogs, Fungi, Fish | ~10 EUR |
| Biophoton | Laser 650nm 5mW | Photobiomodulation | ~2 EUR |
| Electric | HV Mini, MCP4725 DAC | Bees, Sharks, Plants | ~5 EUR |

**Infrastructure:**
- TCA9548A (x2): I²C Multiplexer for 16 I2C sensors (~3 EUR)
- 1.3" TFT ST7789: Local display of certainty/is-values (~3.5 EUR)
- IP65 Enclosure: Physical body/immunity membrane (~2.5 EUR)
- 12V 5A Power Supply (~8 EUR)

**Total Cost (100% System): ~139 EUR** (Modular: start with 25€ base).

### Future hardware
- **EEG / BCI:** Raw voltage time-series via Web Bluetooth. High Gamma = observer focuses → GPU evaluates this point with maximum precision.
- **Quantum Biology:** Cross-referencing magnetometer data with camera pixel data — the system *sees* the magnetic field as a bird does (Radical Pair Mechanism).
- **Atmospheric Optics:** Atmospheric refraction bends light. Local refraction index shifts a star's light on screen exactly as the local atmosphere bends it.
- **Animal Senses:** Ultrasound microphone (bat) via Web Audio, infrared sensor (viper) via Web Serial, VOC sensor (dog) via Web Bluetooth, ground vibration (spider) via accelerometer.
