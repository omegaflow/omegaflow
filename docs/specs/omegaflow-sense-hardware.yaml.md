# ============================================================
# omegaflow_sense_hardware.yaml
# STATUS: PLAN (Phase 10) — CORE BUILT. The ESP32-S3 core (MOSFET bank M1-M8,
#         pan/tilt servo, I2C + TCA9548A, MAX30102 pulse stream, USB-CDC) lives
#         in firmware/radiatorium/ and follows this pin map exactly; the full
#         34-module sense array remains plan. Part 5 corrected 2026-09-12
#         (Radiator-Doktrin). Pin conflict resolved 2026-09-13: SPI display dc
#         moved GPIO9 -> GPIO13 (I2C scl keeps GPIO9); DS18B20 (GPIO7) added.
# PURPOSE: The single source of truth for the physical
#          omegaflow sense module. The 100% Mantis-Shrimp Config.
#          Sensors (receivers), Actuators (senders), Infrastructure.
# PRINCIPLE: Resonance needs presence (actuators = levers)
#            φ(x,y,z,t) resonates with φ(x,y,z,t)
#            Structure over Name (single source of truth)
# ============================================================

meta:
  project: "omegaflow sense — 100% Mantis-Shrimp Observatory"
  paradigm: |
    Every sense is an energy channel. For every channel there is a radiating
    surface (canRadiate) and a sensing surface (canSense). This module
    unifies the complete phylogenetic story of life on Earth in silicon.
    It is a stationary observatory (a local universe) connected via WebSerial.
    To manage 35 modules on a single ESP32-S3, I2C multiplexers are used.
  hardware_brain: "ESP32-S3 DevKit (N8R2/N16R8)"
  firmware_language: "Rust no_std (esp-hal)"
  interface: "WebSerial (CDC-ACM)"
  total_cost_eur: 140.50
  protocol: "PresenceFrame — raw intensity (Σω); ESP32 as a peer among seven (radiators.md:84-106)"

# ============================================================
# PART 1: SENSING SURFACES (canSense) (The Eyes, Ears and Noses)
# Measuring the complete electromagnetic spectrum, sound,
# pressure, chemistry and bioelectricity.
# ============================================================

sensing_surfaces:
  spectral_vision:
    as7341:
      sense: "10-channel spectrum (Mantis shrimp)"
      interface: "I2C (via TCA9548A)"
      cost_eur: 8.00
      url: "https://www.aliexpress.com/wholesale?SearchText=AS7341+spectral+sensor"
    veml6075:
      sense: "UV-A & UV-B (Bee)"
      interface: "I2C (via TCA9548A)"
      cost_eur: 2.00
      url: "https://www.aliexpress.com/wholesale?SearchText=VEML6075+UV+sensor"
    polarization_film:
      sense: "Polarized light (Octopus/Bee)"
      interface: "Optical (mounted over AS7341 or camera)"
      cost_eur: 1.00
      url: "https://www.aliexpress.com/wholesale?SearchText=polarizing+film+sheet+linear"

  thermal:
    mlx90614:
      sense: "Infrared/Thermal (Snake)"
      interface: "I2C (via TCA9548A)"
      cost_eur: 3.50
      url: "https://www.aliexpress.com/wholesale?SearchText=MLX90614+IR+temperature"
    ds18b20:
      sense: "Contact temperature / heater cutoff (Safety-Matrix, mandatory)"
      interface: "1-Wire (GPIO)"
      cost_eur: 1.50
      url: "https://www.aliexpress.com/wholesale?SearchText=DS18B20+waterproof+temperature"

  magnetic:
    qmc5883l:
      sense: "3-Axis magnetic field (Migratory bird)"
      interface: "I2C (via TCA9548A)"
      cost_eur: 1.50
      url: "https://www.aliexpress.com/wholesale?SearchText=QMC5883L+magnetometer"

  chemical:
    sgp30:
      sense: "eCO2 & VOC (Dog/Moth olfaction)"
      interface: "I2C (via TCA9548A)"
      cost_eur: 4.00
      url: "https://www.aliexpress.com/wholesale?SearchText=SGP30+gas+sensor"
    bme680:
      sense: "Pressure, Temp, Humidity (Weather/Fish)"
      interface: "I2C (via TCA9548A)"
      cost_eur: 4.50
      url: "https://www.aliexpress.com/wholesale?SearchText=BME680+module+CJMCU"

  acoustic:
    inmp441:
      sense: "MEMS microphone (Ultrasound/Bioacoustics)"
      interface: "I2S"
      cost_eur: 2.50
      url: "https://www.aliexpress.com/wholesale?SearchText=INMP441+I2S+microphone"
    hc_sr04:
      sense: "Ultrasonic distance (Bat echolocation)"
      interface: "GPIO (Trig/Echo)"
      cost_eur: 1.00
      url: "https://www.aliexpress.com/wholesale?SearchText=HC-SR04+ultrasonic"
    piezo_disc:
      sense: "Infrasound/Vibration (Elephant)"
      interface: "ADC (via ADS1115 or direct)"
      cost_eur: 0.50
      url: "https://www.aliexpress.com/wholesale?SearchText=piezo+disc+sensor+35mm"

  pressure_flow:
    ms5803_14ba:
      sense: "Water pressure/flow (Fish lateral line)"
      interface: "I2C (via TCA9548A)"
      cost_eur: 8.00
      url: "https://www.aliexpress.com/wholesale?SearchText=MS5803-14BA+pressure+sensor"

  bioelectric:
    opt101:
      sense: "Biophotons (Light of cells)"
      interface: "ADC"
      cost_eur: 5.00
      url: "https://www.aliexpress.com/wholesale?SearchText=OPT101+photodiode+module"
    ad8232:
      sense: "Microvolt bioelectricity (Shark/Plant)"
      interface: "ADC"
      cost_eur: 3.00
      url: "https://www.aliexpress.com/wholesale?SearchText=AD8232+ECG+sensor+module"
    lm358_copper:
      sense: "Telluric currents (Earth current)"
      interface: "ADC"
      cost_eur: 3.50
      url: "https://www.aliexpress.com/wholesale?SearchText=LM358+amplifier+module"

  gravity_motion:
    mpu6050:
      sense: "Gravity/Acceleration (Plant gravitropism)"
      interface: "I2C (via TCA9548A)"
      cost_eur: 1.50
      url: "https://www.aliexpress.com/wholesale?SearchText=MPU6050+gyroscope"
    capacitive_soil:
      sense: "Soil moisture/Thigmo (Root)"
      interface: "ADC"
      cost_eur: 1.50
      url: "https://www.aliexpress.com/wholesale?SearchText=Capacitive+soil+moisture+sensor+v1.2"

  interoception_human:
    max30102:
      sense: "Pulse/HRV (Human interoception)"
      interface: "I2C (via TCA9548A)"
      cost_eur: 2.00
      url: "https://www.aliexpress.com/wholesale?SearchText=MAX30102+pulse+oximeter"
      note: |
        RMSSD (vagus nerve tone) becomes the ethical filter for immunity.φ.
        At low tone (stress/danger), immunity.φ blocks strong actuators.

# ============================================================
# PART 2: RADIATING SURFACES (canRadiate) (The Levers and Stimulators)
# Emitting photons, sound, heat, magnetic fields and chemicals
# back into reality to test causality (Transfer-Entropy).
# ============================================================

radiating_surfaces:
  light_spectrum:
    ws2812b_led_ring:
      stimulates: "Visible light (Bee, Plant, Flicker, Observer Synesthesia)"
      interface: "GPIO (RMT)"
      cost_eur: 3.00
      url: "https://www.aliexpress.com/wholesale?SearchText=WS2812B+LED+ring+16"
    uv_led_365nm:
      stimulates: "UV-A (Bee, Plant, Fungus)"
      interface: "GPIO + MOSFET"
      cost_eur: 2.00
      url: "https://www.aliexpress.com/wholesale?SearchText=UV+LED+365nm+1W"
    ir_led_850nm:
      stimulates: "NIR (Reflection, Deep-sea similarity)"
      interface: "GPIO + MOSFET"
      cost_eur: 1.00
      url: "https://www.aliexpress.com/wholesale?SearchText=IR+LED+850nm+5W"

  thermal:
    heating_pad:
      stimulates: "Heat (Snake, Mosquito, Beetle)"
      interface: "GPIO + MOSFET + PWM"
      cost_eur: 3.00
      url: "https://www.aliexpress.com/wholesale?SearchText=heating+pad+5V+flexible"
    peltier_element:
      stimulates: "Heat & Cold (Snake, Antarctic fish)"
      interface: "H-Bridge (L298N) for polarity"
      cost_eur: 5.00
      url: "https://www.aliexpress.com/wholesale?SearchText=TEC1-12706+peltier"

  acoustic:
    i2s_amp_speaker:
      stimulates: "Audio (Sonification, Plant, Mycelium)"
      interface: "I2S (MAX98357A)"
      cost_eur: 5.00
      url: "https://www.aliexpress.com/wholesale?SearchText=MAX98357A+I2S+amplifier"
    piezo_buzzer:
      stimulates: "Ultrasound (Bat, Plant, Mycelium)"
      interface: "GPIO (PWM)"
      cost_eur: 0.50
      url: "https://www.aliexpress.com/wholesale?SearchText=piezo+buzzer+passive"
    body_sound_exciter:
      stimulates: "Infrasound/Vibration (Elephant, Spider)"
      interface: "Class-D Amp"
      cost_eur: 8.00
      url: "https://www.aliexpress.com/wholesale?SearchText=bass+exciter+speaker+vibration"

  vibration_haptic:
    erm_vibration_motor:
      stimulates: "Haptic (Observer, Cat vibrissae)"
      interface: "GPIO + MOSFET + PWM"
      cost_eur: 1.00
      url: "https://www.aliexpress.com/wholesale?SearchText=coin+vibration+motor+3V"
    solenoid_knocker:
      stimulates: "Mechanical impulse (Plant AP, Mycelium)"
      interface: "GPIO + MOSFET"
      cost_eur: 3.00
      url: "https://www.aliexpress.com/wholesale?SearchText=solenoid+push+pull+5V"

  magnetic:
    electromagnet_coil:
      stimulates: "Magnetic field (Bird, Bee, Turtle)"
      interface: "H-Bridge + Coil"
      cost_eur: 4.00
      url: "https://www.aliexpress.com/wholesale?SearchText=enameled+copper+wire+0.5mm"

  gravitational:
    tilt_platform_servo:
      stimulates: "Gravity (Plant, Fungus gravitropism)"
      interface: "GPIO PWM (Servo)"
      cost_eur: 2.00
      url: "https://www.aliexpress.com/wholesale?SearchText=SG90+servo+motor"

  chemical:
    micro_fan_5v:
      stimulates: "Airflow (Dog VOC plume, Wind stimulus)"
      interface: "GPIO + MOSFET"
      cost_eur: 2.00
      url: "https://www.aliexpress.com/wholesale?SearchText=mini+blower+fan+5V"
    mini_water_pump:
      stimulates: "Water flow (Fish lateral line)"
      interface: "GPIO + MOSFET"
      cost_eur: 3.00
      url: "https://www.aliexpress.com/wholesale?SearchText=mini+water+pump+5V"
    ultrasonic_mist_maker:
      stimulates: "Humidity (Fungus fruiting, Epiphyte)"
      interface: "GPIO + MOSFET"
      cost_eur: 5.00
      url: "https://www.aliexpress.com/wholesale?SearchText=ultrasonic+mist+maker"

  biophoton_stimulation:
    low_level_laser:
      stimulates: "Coherent light (Photobiomodulation)"
      interface: "GPIO + MOSFET"
      cost_eur: 2.00
      url: "https://www.aliexpress.com/wholesale?SearchText=laser+diode+650nm+5mW"

  electric_field:
    hv_mini_module:
      stimulates: "E-Field (Bee, Spider ballooning)"
      interface: "GPIO + HV Module"
      cost_eur: 3.00
      url: "https://www.aliexpress.com/wholesale?SearchText=high+voltage+generator+module+micro"
    mcp4725_dac:
      stimulates: "Microvolt (Shark, Plant AP)"
      interface: "I2C (via TCA9548A)"
      cost_eur: 2.00
      url: "https://www.aliexpress.com/wholesale?SearchText=MCP4725+DAC+module"

# ============================================================
# PART 3: INFRASTRUCTURE & PHYSICAL BODY
# Multiplexers, MOSFETs, Enclosure, and Display.
# ============================================================

infrastructure:
  esp32s3_devkit:
    purpose: "The Brain"
    cost_eur: 4.50
    url: "https://www.aliexpress.com/wholesale?SearchText=ESP32-S3+DevKitC+N8R2"

  tca9548a_multiplexer:
    purpose: "I2C Multiplexer for 16 I2C sensors"
    qty: 2
    cost_eur: 3.00
    url: "https://www.aliexpress.com/wholesale?SearchText=TCA9548A+multiplexer"

  irlz44n_mosfet:
    purpose: "Switches for actuators"
    qty: 10
    cost_eur: 2.00
    url: "https://www.aliexpress.com/wholesale?SearchText=IRLZ44N+MOSFET"

  l298n_h_bridge:
    purpose: "Peltier and Electromagnet polarity control"
    cost_eur: 2.00
    url: "https://www.aliexpress.com/wholesale?SearchText=L298N+motor+driver"

  ina219:
    purpose: "Current/Voltage monitoring (Safety)"
    cost_eur: 1.50
    url: "https://www.aliexpress.com/wholesale?SearchText=INA219+current+sensor"

  display_tft_st7789:
    purpose: "Local face of the Archivar (shows certainty, φ-values)"
    interface: "SPI"
    cost_eur: 3.50
    url: "https://www.aliexpress.com/wholesale?SearchText=1.3+inch+TFT+SPI+ST7789"

  enclosure_ip65:
    purpose: "Physical body (immunity.φ). Protects from entropy."
    cost_eur: 2.50
    url: "https://www.aliexpress.com/wholesale?SearchText=waterproof+plastic+enclosure+100x68x50"

  pg7_cable_glands:
    purpose: "Waterproof cable routing for sensor towers"
    qty: 4
    cost_eur: 2.00
    url: "https://www.aliexpress.com/wholesale?SearchText=PG7+cable+gland"

  power_supply_12v_5a:
    purpose: "Power for Peltier, Pump, Coil"
    cost_eur: 8.00
    url: "https://www.aliexpress.com/wholesale?SearchText=12V+5A+power+supply+adapter"

  breadboard_jumper_kit:
    purpose: "Wiring"
    cost_eur: 2.00
    url: "https://www.aliexpress.com/wholesale?SearchText=breadboard+jumper+wires+kit+120"

# ============================================================
# PART 4: HARDWARE ARCHITECTURE & PIN MAP
# ============================================================

esp32_pin_map:
  i2c_bus:
    sda: "GPIO8"
    scl: "GPIO9"
    note: "TCA9548A #1 at 0x70, TCA9548A #2 at 0x71"
  one_wire:
    ds18b20: "GPIO7"
    note: "Heater/Peltier temperature feedback (Safety-Matrix, mandatory)"
  i2s_audio:
    bclk: "GPIO4"
    lrck: "GPIO5"
    din: "GPIO6"
  spi_display:
    sclk: "GPIO12"
    mosi: "GPIO11"
    cs: "GPIO10"
    dc: "GPIO13"
    rst: "GPIO14"
  mosfet_bank:
    m1_led: "GPIO48"
    m2_heater: "GPIO38"
    m3_fan: "GPIO39"
    m4_peltier_in1: "GPIO40"
    m5_peltier_in2: "GPIO41"
    m6_uv_led: "GPIO47"
    m7_pump: "GPIO42"
    m8_solenoid: "GPIO21"
  servo:
    pan: "GPIO15"
    tilt: "GPIO16"

# ============================================================
# PART 5: FIRMWARE PROTOCOL & WEB SERIAL
# ============================================================

protocol:
  frame: "PresenceFrame { omega: [f32; 9] } from the dispatcher"
  translation: "raw intensity (Σω) — canRadiate, as SeismicOscillator"
  wire: "no modulation command exists"
  reason: "replaced by raw intensity (Radiator-Doktrin, radiators.md:84-106 — the synthesizer stays dead)"
  firmware: "no_std esp-hal (firmware/radiatorium/), build in CI"

# ============================================================
# PART 6: SAFETY MATRIX
# Active care within reach, restraint beyond.
# ============================================================

safety:
  uv_led: "NEVER aim at eyes. UV safety goggles. Max 30s exposure."
  laser: "Class 2: never aim at eyes. Eye distance >30cm. Shielding."
  hv_module: "µA currents only! ALWAYS install current limiting. Never on mains."
  heating_pad: "Temperature feedback (DS18B20 or thermistor) MANDATORY. Cutoff <80°C."
  peltier: "Heatsink on hot side! Otherwise self-destruction in minutes."
  electromagnet: "Limit current (INA219). Monitor temperature. <2A."
  solenoid: "Flyback diode (1N4007) MANDATORY, otherwise ESP32 damage!"
  water_electrodes: "Low impedance only. Never on mains. Galvanic isolation."
  plant_stimulation: "Max 30mV, max 1mA. Plant is a living system."
  general: "Active care. Who suffers? Include plant, fungus, animal."

# ============================================================
# PART 7: OUTDOOR VARIANT
# The stationary observatory, weathered. Adds the boundaries the
# indoor config does not carry: weatherproofing, UV/IR-transparent
# windows, off-grid power, moisture control, ESD/lightning care.
# ============================================================

outdoor_variant:
  swaps:
    enclosure_ip65:
      to: "IP67 PC/ASA junction box, UV-stable, mounting flanges"
      reason: "IP65 is splash-proof; outdoors needs dust/immersion (IP67) and UV stability."
    power_supply_12v_5a:
      to: "Solar 6V/5W panel + LiFePO4 18650 + MPPT/TP4056-BMS"
      reason: "No mains outdoors; buffer for Peltier/pump peaks and night."
  additions:
    quartz_window:
      purpose: "UV-transparent window over VEML6075/AS7341"
      reason: "Plastic blocks UV-A/UV-B; the UV/spectral senses go blind behind it."
      url: "https://www.aliexpress.com/wholesale?SearchText=quartz+glass+window+disc"
    ir_window:
      purpose: "IR-transparent window over MLX90614"
      reason: "Plastic and ordinary glass attenuate 5-14 um; the thermal sense needs an IR window."
      url: "https://www.aliexpress.com/wholesale?SearchText=IR+transparent+window+ZnSe"
    solar_panel:
      purpose: "Off-grid power"
      url: "https://www.aliexpress.com/wholesale?SearchText=solar+panel+6V+5W"
    lifepo4_battery:
      purpose: "Energy buffer (Peltier/pump peaks, night)"
      url: "https://www.aliexpress.com/wholesale?SearchText=LiFePO4+18650+TP4056+BMS"
    desiccant:
      purpose: "Moisture control inside the enclosure"
      url: "https://www.aliexpress.com/wholesale?SearchText=silica+gel+desiccant+pack"
    conformal_coating:
      purpose: "PCB moisture protection"
      url: "https://www.aliexpress.com/wholesale?SearchText=silicone+conformal+coating+pcb"
    tvs_esd:
      purpose: "ESD/lightning-induced surge protection on exposed lines"
      url: "https://www.aliexpress.com/wholesale?SearchText=TVS+diode+ESD+protection"
    grounding_lug:
      purpose: "Earth the enclosure/mast"
      url: "https://www.aliexpress.com/wholesale?SearchText=grounding+lug+stainless"
  safety_outdoor:
    - "Water parts (pump, mist maker) only with galvanic isolation + IP68 connectors."
    - "UV/IR windows are not to be looked through; the safety matrix holds outdoors."
    - "DS18B20 cutoff (<80 C) is mandatory; add a second sensor on the electromagnet if it runs >2 A."
    - "Ventilation with insect mesh; keep condensation off the optics."
