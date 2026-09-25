<!--
  title: Mantis-Shrimp minimal — build & assembly (CORE BUILT)
  class: ref
  date: 2026-09-21
  sha256: d098ee2afc9f078b7b40e383ea16da70a4f6716585264bd1b6ab8dc646efb1b6
  status: live
  see-also: docs/specs/omegaflow-sense-hardware.yaml.md, docs/specs/mantis-shrimp-bom.md
-->
# Mantis-Shrimp minimal — build & assembly (CORE BUILT)

Scope: the minimal Mantis-Shrimp node, i.e. the set that the single source of
truth `docs/specs/omegaflow-sense-hardware.yaml.md` names **CORE BUILT** — the
ESP32-S3 core (MOSFET bank M1–M8, pan/tilt servo, I2C + TCA9548A, MAX30102
pulse stream, USB-CDC) that lives in `firmware/radiatorium/`. This is the entry
point before the full sense array (the remaining modules stay plan; the source
spec's module count is a carried riss — 34 in its status line, 35 in its
principle line — so no single number is asserted here).

All values below are derived from the two named sources plus the firmware tree;
nothing is invented. What the sources do not carry is `pending`.

## 1. Minimal module set

The core is the brain plus its actuator bank, its servo pair, the I2C
multiplexer and one pulse sensor:

| Role | Module | Bound at |
|---|---|---|
| Brain | ESP32-S3 DevKitC-1 (N8R2) | `firmware/radiatorium/` |
| Actuator bank | MOSFET bank M1–M8 (IRLZ44N) | GPIO48/38/39/40/41/47/42/21 |
| Aiming | Pan/tilt servo (SG90) | GPIO15 / GPIO16 |
| I2C fan-out | TCA9548A mux #1 (`0x70`), #2 (`0x71`) | SDA GPIO8, SCL GPIO9 |
| Pulse sense | MAX30102 (via mux way 0) | I2C `0x70` |
| Host link | USB-CDC (USB Serial JTAG) | `USB_DEVICE` |
| Safety sense | DS18B20 (mandatory cutoff, see §4) | 1-Wire GPIO7 |

Firmware evidence: `firmware/radiatorium/src/main.rs` binds eight LEDC channels
to GPIO48/38/39/40/41/47/42/21 (M1–M8), pan/tilt to GPIO15/16 via MCPWM, I2C0
to SDA GPIO8 / SCL GPIO9 at 400 kHz, selects mux way 0 (`0x70`), configures the
MAX30102 for SpO2 mode at 100 Hz, and streams over `UsbSerialJtag`. The pure
logic (frame parse, Σω duty, mux select, MAX30102 FIFO) is
`firmware/radiatorium-lib/` (`frame`, `max30102`, `mux`, `nn`, `pwm`).

**Firmware state:** the DS18B20 read path is tracked — `radiatorium-lib/src/ds18b20.rs`
(`Cutoff`, CRC8, scratchpad) and `radiatorium/src/one_wire.rs` (1-Wire timing) — and
`radiatorium/src/main.rs:143` binds `GPIO7`, with the cutoff evaluated in the safety
path (`main.rs:165`). The heater/peltier cutoff is mandatory by the safety matrix
while a physical bring-up gap remains: the sensor hardware is LOCKED by the operator,
so verification against a real device is `pending` (a register duty, not a built function).

## 2. Wiring & pin map

Copied from `omegaflow-sense-hardware.yaml.md` PART 4, including the 2026-09-13
correction (SPI display `dc` GPIO9 → GPIO13) and the DS18B20 addition (GPIO7):

| Function | Pin |
|---|---|
| I2C SDA | GPIO8 |
| I2C SCL | GPIO9 |
| 1-Wire DS18B20 | GPIO7 |
| I2S BCLK / LRCK / DIN | GPIO4 / GPIO5 / GPIO6 |
| SPI display SCLK / MOSI / CS / DC / RST | GPIO12 / GPIO11 / GPIO10 / GPIO13 / GPIO14 |
| M1 LED ring | GPIO48 |
| M2 heater | GPIO38 |
| M3 fan | GPIO39 |
| M4 peltier in1 | GPIO40 |
| M5 peltier in2 | GPIO41 |
| M6 UV LED | GPIO47 |
| M7 pump | GPIO42 |
| M8 solenoid | GPIO21 |
| Servo pan | GPIO15 |
| Servo tilt | GPIO16 |

Mux addresses: TCA9548A #1 `0x70`, #2 `0x71`
(`firmware/radiatorium-lib/src/mux.rs`: `MUX_A = 0x70`, `MUX_B = 0x71`,
ways 0–7 on A, 8–15 on B). The core uses way 0; the second mux and the I2S/SPI
buses belong to the full array and are not bound by the built core.

## 3. Firmware build & flash

Derived from `firmware/radiatorium/{rust-toolchain.toml,.cargo/config.toml,Cargo.toml}`
and `.github/workflows/esp32-firmware.yml`:

- Toolchain: `rust-toolchain.toml` pins `channel = "esp"` (the Xtensa fork),
  installed with `cargo install espup --locked` then `espup install`.
- Target: `.cargo/config.toml` sets `target = "xtensa-esp32s3-none-elf"`,
  `build-std = ["core"]`, rustflags `-C link-arg=-Tlinkall.x -C link-arg=-nostartfiles`.
- Host logic tests (no window/audio/port):
  `cargo test --manifest-path firmware/radiatorium-lib/Cargo.toml`.
- Build the firmware (from `firmware/radiatorium`, with the esp environment):
  `. $HOME/export-esp.sh` then `RUSTUP_TOOLCHAIN=esp cargo build --release`.
- Wrap ELF into a merged image:
  `espflash save-image --chip esp32s3 --merge <elf> <bin>`.
- Flash + monitor: `.cargo/config.toml` names the runner
  `espflash flash --monitor --chip esp32s3`, so `cargo run --release` uses it.

Output paths (from the CI artifact step):
`firmware/radiatorium/target/xtensa-esp32s3-none-elf/release/radiatorium` and
`…/radiatorium.bin`.

## 4. Safety matrix — active actuators of the minimal set

Condensed from PART 6 (`safety:`), restricted to the actuators the core drives.
The full matrix holds; this is not a replacement.

| Actuator | Rule |
|---|---|
| Heater (M2) | Temperature feedback (DS18B20 or thermistor) MANDATORY. Cutoff <80 °C. |
| Peltier (M4/M5) | Heatsink on hot side mandatory, otherwise self-destruction in minutes. |
| Solenoid (M8) | Flyback diode (1N4007) MANDATORY, otherwise ESP32 damage. |
| UV LED (M6) | NEVER aim at eyes. UV safety goggles. Max 30 s exposure. |
| Laser (not in the minimal set) | Class 2: never aim at eyes. Eye distance >30 cm. Shielding. Applies when added. |
| General | Active care. Who suffers? Include plant, fungus, animal. |

The DS18B20 cutoff is wired (GPIO7) but its firmware read is `pending` (§1);
the safety rule stands until the read path exists.

## 5. BOM subset for the minimal build

Item IDs and prices from `docs/specs/mantis-shrimp-bom.md` (AliExpress hits,
2026-09-13; prices may vary). Link form: `https://de.aliexpress.com/item/<id>.html`.

Core infrastructure:

| Part | Item-ID | € |
|---|---|---|
| ESP32-S3 DevKitC-1 N8R2 | 1005012092039320 | 7,49 |
| TCA9548A Mux | 1005008598660767 | 1,59 |
| IRLZ44N (10×) | 1005007174160996 | 2,89 |
| L298N H-Brücke (peltier polarity) | 32392774289 | 2,05 |
| INA219 current sensor (safety) | 1005006960298791 | 1,55 |
| 12V 5A Netzteil | 1005006759578540 | 12,49 |
| Jumper-/Breadboard-Kit 120 | 1005007539811930 | 2,15 |

Core sensors:

| Part | Item-ID | € |
|---|---|---|
| MAX30102 Puls | 1005007015407514 | 3,15 |
| DS18B20 (1-Wire, Safety) | — Suche — | 1,50 |

Core actuators (M1–M8 + servos):

| Part | Item-ID | € | Bank |
|---|---|---|---|
| WS2812B LED-Ring | 1005009768866205 | 2,49 | M1 |
| Heizfolie PI/Kapton 5V | 1005012798490300 | 3,19 | M2 |
| Mini-Radiallüfter 5V | 1005003595630530 | 1,59 | M3 |
| Peltier TEC1-12706 | 1005013011555079 | 14,89 | M4/M5 |
| UV-LED 365 nm (10×) | 32991042964 | 15,99 | M6 |
| Mini-Wasserpumpe 5V | 1005010574721674 | 4,99 | M7 |
| Solenoid Push-Pull | 1005002278950915 | 2,59 | M8 |
| SG90-Servo (Bulk) | 1005006219266362 | 35,19 | pan/tilt |

DS18B20 search URL: `https://www.aliexpress.com/wholesale?SearchText=DS18B20+waterproof+temperature`.

## Zweitknoten — PINE64 Ox64

The Ox64 is the named **second node** (Zweitknoten); the ESP32-S3 remains the
built platform. Facts below are from the platform comparison in
`docs/specs/mantis-shrimp-bom.md`.

- SoC: RISC-V BL808 — C906 64-bit + E907 + LP core.
- Radio: WiFi + BLE + **ZigBee** (the reason the second node is interesting:
  ZigBee mesh alongside the ESP32-S3 node).
- Role: **host CPU of the coordinator** — ZNSP over UART to the ESP32-H2 NCP
  radio (Route 2, Rat 2026-09-25); its own 802.15.4 radio stays unused
  (`pending`, no public driver).
- Ecosystem: smaller than ESP32-S3 (RISC-V, Buildroot/OpenWrt).
- I2C/SPI drivers: less finished than the ESP-IDF/Arduino stack.
- Procurement: PINE64 (EU: `pine64eu.com`). PINE64 ships devices to developers;
  a request went to `sales@pine64.org` + `info@pine64eu.com` on 2026-09-20.

`pending` (unmeasured — named, not filled):

- Concrete Buildroot/OpenWrt image path for the Ox64.
- ZigBee mesh stack/protocol choice — **decided 2026-09-25 (Rat, Route 2
  Espressif-RCP):** ESP32-H2 = NCP radio, S3/Ox64 = host CPU, ZNSP over UART,
  the Rust host self-built; the BL808's own 802.15.4 radio stays `pending`.
- Measured I2C/SPI driver readiness on the BL808 (the "less finished" note is
  the BOM's qualitative line, not a per-driver measurement).
- A radiatorium-core port to the BL808.
- Device arrival and bring-up (tied to the 2026-09-20 PINE64 request).

No claim of a running Ox64 node is made: there is no separate Ox64
documentation, and the port is not built.

## ZNSP host skeleton — built 2026-09-25

Measured (Sensory-Folge 168, 2026-09-25 via `research-max` + `grind-flash`):

- The ZNSP host transport lives in `esp-zigbee-sdk` **`examples/esp_zigbee_host/components/`**
  on branch **`release/v1.0`** — not in `components/esp_zigbee_host/`, and absent on
  `main` (v2.x). Host log tag `ESP_ZNSP_FRAME`.
- Wire: SLIP (`END 0xC0`, `ESC 0xDB`) around `[header 7 B][payload len B][CRC16-LE 2 B]`.
  Header LE: `flags:u16` (`version` bits[3:0], `type` bits[7:4], reserved bits[15:8]),
  `id:u16`, `sn:u8`, `len:u16`; `type` 0/1/2 = request/response/notify.
  CRC = reflected poly `0x8408`, init `0x0000`, xorout `0xFFFF` (measured: `esp_rom_crc16_le`
  wraps init and result in `~`; both upstream README captures confirm — `7×00 → 0xFFFF`).
- Command IDs (network subset): INIT `0x0000`, START `0x0001`, FORMNETWORK `0x0004`,
  PERMIT_JOINING `0x0005`, JOINNETWORK `0x0006`; error response id `0xFFFF`; status
  byte `0x00` = success. FormNetwork notify payload 11 B (extPanId[8] + panId u16 +
  channel u8), PermitJoining notify 1 B.
- UART physical layer: 115200 8N1, no flow control. Host TX → H2 GPIO4, host RX → H2 GPIO5.

Built:

- `firmware/radiatorium-lib/src/znsp.rs` — `SlipDecoder`, `crc16_le`, `Frame` parse/encode,
  `cmd`, `Status`, `NetworkMachine` (`Idle | InitSent | FormNetworkSent | Started |
  Steering | Joined`), host tests (CRC + parse fixtures drawn from the upstream README
  captures). Runs in `.github/workflows/esp32-firmware.yml:31`.
- `firmware/radiatorium/src/bin/znsp_host.rs` — esp-hal UART1 binding; no radio, no hardware.
- `.github/workflows/zigbee-host.yml` — builds the upstream `examples/esp_zigbee_host`
  for esp32s3 (measured SHA `c9e2c3e12642c704096dfefe25b62212b47229ba`, ESP-IDF v5.3.2).

`pending`: the `esp_zb_cfg_t` FORMNETWORK request payload is ABI-raw (`sizeof` unmeasured)
— named as `NetworkMachine::form_network_payload_pending()`, never a guessed struct.
