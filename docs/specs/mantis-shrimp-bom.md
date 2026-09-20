<!--
  title: Mantis-Shrimp BOM — kuratierte Einkaufsliste (Stand 2026-09-13)
  class: ref
  date: 2026-09-13
  sha256: 56efc939b50dee8cbfad2367b6480740c7c9efdb619fd99d3bc6d52813fe6b36
-->
# Mantis-Shrimp BOM — kuratierte Einkaufsliste (2026-09-13)

Quelle: `docs/specs/omegaflow-sense-hardware.yaml.md` (100% Mantis-Shrimp Observatory).
Preise = AliExpress-Trefferpreise am 2026-09-13 (können schwanken). Links:
`https://de.aliexpress.com/item/<id>.html`. Kuratiert per webread (kein Login nötig zum Lesen).

## Sensoren (canSense)

| Teil | Item-ID | € |
|---|---|---|
| AS7341 Spektralsensor | 1005008897941427 | 6,19 |
| VEML6075 UV | 1005004653958045 | 3,85 |
| Polarisationsfolie | 1005012049647725 | 3,89 |
| MLX90614 (GY-906) | 1005004003178158 | 6,03 |
| DS18B20 (1-Wire, Safety) | — Suche — | 1,50 |
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

DS18B20-Suche: `https://www.aliexpress.com/wholesale?SearchText=DS18B20+waterproof+temperature`

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

| Teil | Suche |
|---|---|
| IP67 Gehäuse (PC/ASA, UV-stabil) | `IP67+junction+box+PC` |
| Quarzglas-Fenster (UV) | `quartz+glass+window+disc` |
| IR-Fenster (ZnSe) | `IR+transparent+window+ZnSe` |
| Solarpanel 6V/5W | `solar+panel+6V+5W` |
| LiFePO4 + TP4056/BMS | `LiFePO4+18650+TP4056+BMS` |
| Silicagel | `silica+gel+desiccant+pack` |
| Conformal Coating | `silicone+conformal+coating+pcb` |
| TVS/ESD | `TVS+diode+ESD+protection` |
| Erdungslasche | `grounding+lug+stainless` |

Suche: `https://www.aliexpress.com/wholesale?SearchText=<suche>`

## Plattform-Vergleich — ESP32-S3 (aktuell) vs. PINE64 Ox64

| Merkmal | ESP32-S3 (BOM) | PINE64 Ox64 |
|---|---|---|
| Architektur | Xtensa LX7, dual-core 32-bit | RISC-V BL808 (C906 64-bit + E907 + LP) |
| Funk | WiFi + BLE | WiFi + BLE + **ZigBee** |
| Ökosystem | sehr groß (ESP-IDF/Arduino) | kleiner (RISC-V, Buildroot/OpenWrt) |
| I2C/SPI-Treiber | reichlich | weniger fertig |
| Rolle | gebauter Mantis-Shrimp-Knoten | Alternative/Zweitknoten: RISC-V + ZigBee-Mesh |
| Bezug | AliExpress (BOM) | PINE64 (EU: `pine64eu.com`) |

Verdikt: **ESP32-S3 bleibt die gebaute Plattform** (Bibliotheken, Preis,
Sensorknoten erprobt). **Ox64** ist als Zweitknoten spannend — RISC-V-Erfahrung
und ZigBee-Mesh —, kostet aber Treiber-Arbeit. PINE64 schickt Geräte an
Entwickler; Anfrage 2026-09-20 an `sales@pine64.org` + `info@pine64eu.com`.

## Notizen

- Warenkorb erfordert AliExpress-Login (anonyme Session lehnt „In den Warenkorb" ab).
- Spec-Korrekturen 2026-09-13: SPI-Display `dc` GPIO9 → GPIO13 (Konflikt mit I2C `scl` aufgelöst), DS18B20 ergänzt (Safety-Pflicht).
