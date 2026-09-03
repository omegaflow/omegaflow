# NIST SP 811 — Unit Naming and API Normalization
# Guide for the Use of the International System of Units (2008)
# Public domain — US government work
# Sections 4–6: unit names, symbols, conversions

## Section 4 — Unit Names and Symbols

### Rules
- Unit symbols are invariant in plural: `1 m, 2 m`
- Unit symbols are lowercase except: named for persons (Pa, N, K, Hz, W, J, V, A, T)
- Exception: L for litre (avoid confusion with digit 1)
- No space between prefix and base symbol: km, mg, µs
- No period after unit symbols (they are mathematical entities, not abbreviations)

## Section 5 — API Unit String Normalization

API responses use varied unit notations. This table maps common variants to SI standard symbols.

| API variant | SI standard | Quantity |
|-------------|-------------|----------|
| m, meters, metre | m | length |
| km, kilometers | km | length |
| cm, centimeters | cm | length |
| mm, millimeters | mm | length |
| s, sec, seconds | s | time |
| ms, millisec | ms | time |
| min, minutes | min | time |
| h, hr, hours | h | time |
| d, days | d | time |
| Hz, hertz | Hz | frequency |
| kHz, kilohertz | kHz | frequency |
| MHz, megahertz | MHz | frequency |
| GHz, gigahertz | GHz | frequency |
| kg, kilograms | kg | mass |
| g, grams | g | mass |
| t, tonne, metric_ton | t | mass |
| N, newton | N | force |
| Pa, pascal, pascals | Pa | pressure |
| hPa, hectopascal | hPa | pressure |
| kPa, kilopascal | kPa | pressure |
| MPa, megapascal | MPa | pressure |
| J, joule, joules | J | energy |
| kJ, kilojoule | kJ | energy |
| MJ, megajoule | MJ | energy |
| W, watt, watts | W | power |
| kW, kilowatt | kW | power |
| MW, megawatt | MW | power |
| GW, gigawatt | GW | power |
| V, volt, volts | V | electric potential |
| mV, millivolt | mV | electric potential |
| kV, kilovolt | kV | electric potential |
| A, amp, ampere | A | electric current |
| mA, milliamp | mA | electric current |
| kA, kiloampere | kA | electric current |
| Ω, ohm, Ohm | Ω | electric resistance |
| S, siemens | S | electric conductance |
| S/m, S.m-1 | S/m | conductivity |
| µS/cm, uS/cm | µS/cm | conductivity |
| mS/cm | mS/cm | conductivity |
| T, tesla, Tesla | T | magnetic flux density |
| nT, nanotesla | nT | magnetic flux density |
| µT, microtesla | µT | magnetic flux density |
| G, gauss | G | magnetic flux density |
| rad, radian | rad | plane angle |
| deg, degree, ° | ° | plane angle |
| C, degC, celsius | °C | Celsius temperature |
| K, kelvin | K | thermodynamic temperature |
| F, degF, fahrenheit | °F | Fahrenheit |
| L, l, liter, litre | L | volume |
| mL, ml | mL | volume |
| m³, m3, cum | m³ | volume |
| km³, km3 | km³ | volume |
| eV, electronvolt | eV | energy |
| keV, kiloelectronvolt | keV | energy |
| MeV, megaelectronvolt | MeV | energy |
| GeV, gigaelectronvolt | GeV | energy |
| AU, au, astronomical_unit | au | length |
| pc, parsec | pc | length |
| mas, milliarcsecond | mas | plane angle |

## Section 6 — Non-SI Unit Conversions

| Unit | Symbol | SI equivalent |
|------|--------|---------------|
| inch | in | 0.0254 m |
| foot | ft | 0.3048 m |
| yard | yd | 0.9144 m |
| mile | mi | 1609.344 m |
| nautical mile | nmi | 1852 m |
| knot | kn | 0.514444 m/s |
| km/h | km/h | 1/3.6 m/s |
| mph | mph | 0.44704 m/s |
| gallon (US) | gal | 3.785412e-3 m³ |
| pound | lb | 0.45359237 kg |
| atmosphere | atm | 101325 Pa |
| bar | bar | 100000 Pa |
| mbar | mbar | 100 Pa |
| mmHg | mmHg | 133.322 Pa |
| psi | psi | 6894.76 Pa |
| calorie | cal | 4.184 J |
| erg | erg | 1e-7 J |
| dyne | dyn | 1e-5 N |
| gal (Galileo) | Gal | 0.01 m/s² |
| mGal | mGal | 1e-5 m/s² |
| µGal | µGal | 1e-8 m/s² |
| E (Eötvös) | E | 1e-9 s⁻² |
| gauss | G | 1e-4 T |
| jansky | Jy | 1e-26 W·m⁻²·Hz⁻¹ |
| solar flux unit | sfu | 1e-22 W·m⁻²·Hz⁻¹ |
| dobson unit | DU | 2.687e16 molecules/cm² |
