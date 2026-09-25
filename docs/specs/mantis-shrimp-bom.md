<!--
  title: Mantis-Shrimp BOM — kuratierte Einkaufsliste (Stand 2026-09-25)
  class: ref
  date: 2026-09-25
  sha256: e73ea9851ee526abf7c6d43589188bbf366fe54767422671f50a8e5c9803e527
-->
# Mantis-Shrimp BOM — kuratierte Einkaufsliste (2026-09-25)

Quelle: `docs/specs/omegaflow-sense-hardware.yaml.md` (100% Mantis-Shrimp Observatory).
Preise = AliExpress-Trefferpreise am 2026-09-13, Ergänzungen am 2026-09-25 (können
schwanken). Links: `https://de.aliexpress.com/item/<id>.html`. Kuratiert per
`archive_search --playwright` (kein Login nötig zum Lesen). Die 2026-09-25-Session
rendert die AliExpress-Seiten in **CHF**; ein `*` an einem Preis = EUR abgeleitet aus
dem gemessenen EZB-Kurs 2026-09-24 (1 CHF = 1,0628 EUR) — nicht direkt gemessen.

## Sensoren (canSense)

| Teil | Item-ID | € |
|---|---|---|
| AS7341 Spektralsensor | 1005008897941427 | 6,19 |
| VEML6075 UV | 1005004653958045 | 3,85 |
| Polarisationsfolie | 1005012049647725 | 3,89 |
| MLX90614 (GY-906) | 1005004003178158 | 6,03 |
| DS18B20 (1-Wire, Safety, wasserdicht 1 m) | 1005012179635448 | 0,67 CHF ≈ 0,71 €* |
| QMC5883L (GY-273) | 1005007182895828 | 2,29 |
| SGP30 (GY-SGP30) | 1005005470338431 | 7,31 |
| BME680 (CJMCU-680) | 1005008176567197 | 8,10 |
| INMP441 I2S-Mikrofon | 1005007987577953 | 2,09 |
| HC-SR04 Ultraschall | 1005005467178145 | 1,35 |
| Piezo-Disc 35 mm (10×) | 1005005145747133 | 3,99 |
| MS5803-14BA Druck | 4001278843263 | 15,59 |
| OPT101 Photodiode | 1005012641958616 | 5,99 |
| AD8232 EKG | 1005011725774187 | 2,05 |
| LM358 Verstärker | 1005005926341872 | 1,25 |
| MPU6050 (GY-521) | 1005010057794277 | 1,69 |
| Kapazitiver Bodenfeuchte | 1005009610892245 | 1,99 |
| MAX30102 Puls | 1005007015407514 | 3,15 |
| ATGM336H GNSS (GPS+BDS, UART, EEPROM) | 1005009361234427 | 3,01 CHF ≈ 3,20 €* |

## Aktoren (canRadiate)

| Teil | Item-ID | € |
|---|---|---|
| WS2812B LED-Ring | 1005009768866205 | 2,49 |
| UV-LED 365 nm (10×) | 32991042964 | 15,99 |
| IR-LED 850 nm (5×) | 1005009978510966 | 1,79 |
| Heizfolie PI/Kapton 5V | 1005012798490300 | 3,19 |
| Peltier TEC1-12706 | 1005013011555079 | 14,89 |
| MAX98357A I2S-Verstärker | 1005007629020891 | 1,69 |
| Piezo-Buzzer passiv | 32680813535 | 1,55 |
| Bass-Exciter 20 W | 1005010592499607 | 62,69 |
| Bass-Exciter (Alternative) | 1005002682778172 | 14,89 |
| Vibrationsmotor 3V (10×) | 1005009267048597 | 3,79 |
| Solenoid Push-Pull | 1005002278950915 | 2,59 |
| 1N4007 Freilaufdiode (Solenoid M8, Pflicht) | 1005006454795578 (100 Stk) · Pollin 140020 | 0,92 CHF/100 ≈ 0,98 €/100* · 0,05 €/Stk (Pollin) |
| Kupferlackdraht 0,5 mm | 33057393544 | 3,95 |
| SG90-Servo (Bulk) | 1005006219266362 | 35,19 |
| Mini-Radiallüfter 5V | 1005003595630530 | 1,59 |
| Mini-Wasserpumpe 5V | 1005010574721674 | 4,99 |
| Ultraschall-Nebler 5V | 1005009315240985 | 1,59 |
| Laserdiode 650 nm 5 mW | 1005008143035440 | 4,39 |
| HV-Generator 10–25 kV | 1005009438868068 | 8,19 |
| MCP4725 DAC | 1005010037235676 | 1,59 |

Schwache Picks: SG90 nur als Bulk-Treffer (Einzelstück ~2 €), Bass-Exciter teuer (Alternative oben).

## Infrastruktur

| Teil | Item-ID | € |
|---|---|---|
| ESP32-S3 DevKitC-1 N8R2 | 1005012092039320 | 7,49 |
| TCA9548A Mux | 1005008598660767 | 1,59 |
| IRLZ44N (10×) | 1005007174160996 | 2,89 |
| L298N H-Brücke | 32392774289 | 2,05 |
| INA219 Stromsensor | 1005006960298791 | 1,55 |
| ST7789 1,3" TFT | 1005009313531539 | 3,45 |
| IP65-Gehäuse 100×68×50 | 1005012869888636 | 1,59 |
| PG7 Kabelverschraubung | 1005012013946264 | 2,35 |
| 12V 5A Netzteil | 1005006759578540 | 12,49 |
| Jumper-/Breadboard-Kit 120 | 1005007539811930 | 2,15 |

## Outdoor-Zusatz (Spec PART 7)

| Teil | Item-ID | Preis |
|---|---|---|
| IP67 Gehäuse (PC/ASA, UV-stabil) | 1005007825059822 | 5,22 CHF ≈ 5,55 €* |
| Quarzglas-Fenster (UV, JGS1) | 1005005957896679 | 2,34 CHF ≈ 2,49 €* |
| IR-Fenster (ZnSe, CVD) | 1005004498442545 | 18,19 CHF ≈ 19,33 €* |
| Solarpanel 6V/5W (Waveshare) | 1005009677090192 | 8,45 CHF ≈ 8,98 €* |
| LiFePO4 18650 (3,2 V, 1800 mAh) | 1005011867942107 | 10,87 CHF ≈ 11,55 €* |
| BMS/Laderegler 1S 3,2 V (7 A) | 1005004814230753 | 0,71 CHF ≈ 0,75 €* |
| Silicagel Trockenmittel | 1005006861885143 | 3,05 CHF ≈ 3,24 €* |
| Conformal Coating (Silikon, Pinsel) | 1005012218197836 | 2,39 CHF ≈ 2,54 €* |
| TVS/ESD-Diode (50 Stk, SOD-523) | 1005006300625215 | 0,92 CHF ≈ 0,98 €* |
| Erdungslasche Edelstahl (10 Stk) | 1005009120469685 | 13,47 CHF ≈ 14,32 €* |

## Plattform-Vergleich — ESP32-S3 (aktuell) vs. PINE64 Ox64

| Merkmal | ESP32-S3 (BOM) | PINE64 Ox64 |
|---|---|---|
| Architektur | Xtensa LX7, dual-core 32-bit | RISC-V BL808 (C906 64-bit + E907 + LP) |
| Funk | WiFi + BLE | WiFi + BLE + **ZigBee** (802.15.4 im BL808-Datasheet; Stack-Weg offen) |
| Ökosystem | sehr groß (ESP-IDF/Arduino) | kleiner (RISC-V, Buildroot/OpenWrt) |
| I2C/SPI-Treiber | reichlich | weniger fertig |
| Rolle | gebauter Mantis-Shrimp-Knoten | Alternative/Zweitknoten: RISC-V + ZigBee-Mesh |
| Bezug | AliExpress (BOM) | PINE64 (EU: `pine64eu.com`) |

Verdikt: **ESP32-S3 bleibt die gebaute Plattform** (Bibliotheken, Preis,
Sensorknoten erprobt). **Ox64** ist als Zweitknoten spannend — RISC-V-Erfahrung
und ZigBee-Mesh —, kostet aber Treiber-Arbeit. PINE64 schickt Geräte an
Entwickler; Anfrage 2026-09-20 an `sales@pine64.org` + `info@pine64eu.com`.

**Messung 2026-09-25 (ZigBee-Zeile):** **M1 gemessen — das BL808-Silizium trägt 802.15.4.** Das Bouffalolab-Datasheet (`bl_docs`, `BL808_DS` v1.2, Features: „Zigbee / IEEE 802.15.4" + „Wi-Fi/Bluetooth/Zigbee Coexistence"; PINE64-Ox64-Wiki „Zigbee") belegt ein 802.15.4-Radio im 2,4-GHz-Transceiver (gemessen 2026-09-25 via `general`/flash `archive_search`). Damit ist die Zeile „Funk: WiFi + BLE + ZigBee" für das **Silizium** belegt. Offen bleibt der **Stack-Weg** (Rat): ein öffentliches BL808-ZigBee-SDK/NCP wurde nicht gefunden — `ncp-blz`/`zigpy-blz`/`bl_iot_sdk` sind BL702/BL706-scoped, `bl_mcu_sdk` (BL808) hat 0 Treffer, der BL808-Reference-Manual kein Wireless-Kapitel. Optionen: BL70x-NCP als Koordinator-Dongle oder Espressif-RCP (`esp-zigbee-sdk` `zigbee_gateway`, ESP32-H2/C6 als `ot_rcp`). M2 (nach Ankunft) bleibt: ZigBee-Beispiel für den BL808 bauen.

## Notizen

- Warenkorb erfordert AliExpress-Login (anonyme Session lehnt „In den Warenkorb" ab).
- Spec-Korrekturen 2026-09-13: SPI-Display `dc` GPIO9 → GPIO13 (Konflikt mit I2C `scl` aufgelöst), DS18B20 ergänzt (Safety-Pflicht).
- 1N4007 (Freilaufdiode Solenoid M8, Pflicht laut `mantis-shrimp-build.md` §4): am 2026-09-25 mit `archive_search --playwright` gefunden (1005006454795578, 100 Stk). Die Notiz „kein AliExpress-Treffer" vom 2026-09-24 ist widerlegt — jene Suche lief über `--all`, das AliExpress nicht abfragt. Günstigste gemessene €-Quelle: Pollin 140020, 0,05 €/Stk.
- Chemie-Korrektur 2026-09-25: `TP4056` ist ein 4,2-V-Li-Ion-Lader und darf eine 3,2-V-LiFePO4-Zelle (Ladeschluss 3,65 V) **nicht** laden. Der LiFePO4-Pfad braucht ein 3,2-V-BMS (gemessen: 1005004814230753, 1S 7 A mit Temperaturschutz) — die Spec-Zeile „LiFePO4 + TP4056/BMS" ist damit auf BMS korrigiert.
