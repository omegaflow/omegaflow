<!--
  title: Befund — Dispersions-Ortungstest: ein Ereignis, drei Kegel, der Offset ist die Zahl
  class: befund
  date: 2026-09-09
  sha256: ea94bb544eb36c1175daf1a153cbfa4f9b8a3f8aa4c8f76408d8d184c9ba3fc6
  status: done
  see-also: docs/auftrag/auftrag-dispersions-ortungstest.md docs/auftrag/auftrag-dispersionsrelation.md docs/TODO.md
-->

# Befund: der Dispersions-Ortungstest

## Frage & Bindung

Auftrag `docs/auftrag/auftrag-dispersions-ortungstest.md` (`probe-commit` b0b7b7d):
die gemessene Band-Latenz muss das Ereignis auf den bekannten Sonnenort
abbilden — Ortung statt Wartung, das Dispersion-Verdikt von außen geprüft.
Probe: `tools/measure/src/bin/dispersion_ortung_probe.rs` (24-s-Zellen, Fenster
±40 min, drei Kegel = drei unabhängige Mess-Wege mit eigener Latenz, die Sonne
als Kalibrier-Referenz aus Ephemeriden).

## Das Ereignis — Beleg, nicht erfunden

Identifiziert aus dem gemessenen Bestand: **22 Ereignis-Kandidaten** (94 Å
24-s-Median über der Jahresschwelle Median+10σ, Refraktär 75 Zellen,
2013–2015). Die Rangfolge ist skalenfrei (Spitze/Jahresschwelle), weil der
94-Å-Korpus 2014 eine andere Größenskala trägt (Jahresschwelle 1.759e2 gegen
1.998e0 / 1.830e0 — gemessen, benannt, nicht gedeutet).

Rang 1 (2014-05-23 19:35:12) trägt keinen sauberen Anstieg in 94 Å oder 335 Å
im Fenster — abgelehnt.

Gewählt: **Rang 2 — 2013-05-14 01:11:36** (unix 1368493896, TDB
421765963.184). **Beleg:** XRSB 24-s-Median-Spitze **4.647e-4 W/m²** über der
Korpus-Schwelle 5e-6 (GOES-15, `xr_20130513/14/15.nc`). t_arr trägt den
Zellenanfang der Spitzen-Zelle; die halbe Zelle (±12 s) liegt in δτ_min.

## Die Lichtzeit-Basis (Ephemeriden)

- Emissions-Epoche t_E = t_arr − τ_lt, iteriert bis < 1e-9 s: **421765458.862 s TDB**.
- τ_lt = d/c = **504.322 s**; d_eph = |r_sun(t_E) − r_earth(t_arr)| =
  **1.511921e11 m = 1.010656 AU** (`ephemeris_sun.bin` / `ephemeris_earth.bin`,
  ICRS, Barycenter; der GOES-15-Offset 42164 km = 0.14 Licht-s liegt unter der
  Zelle — benannt).
- Fehlerkreis-Radius δd = c·δτ_min = **7.195e9 m** (δτ_min = 24 s); Skala
  Δt·c/τ_lt = **1.43e7 m/s = 4.8 % von c**.

## Die drei Kegel

Kegel-Anstieg = die erste Zelle im Fenster über dem Mittelpunkt der eigenen
Exkursion (min + 0.5·(max−min)), vorausgegangen von einer Zelle darunter —
eine echte Anstiegskante, kein Abklingrest.

| Kegel | λ | f | Δt_i = t_on − t_E | d_i = c·Δt_i | eps_i [s] | eps_i [m] | Spitze nach Anstieg |
|---|---|---|---|---|---|---|---|
| XRSA | 2.25e-10 m | 1.332e18 Hz | 264.3 s | 7.924e10 m | −240.0 s | −7.195e10 m | 144 s |
| 94A | 9.40e-9 m | 3.189e16 Hz | 480.3 s | 1.440e11 m | −24.0 s | −7.195e9 m | 240 s |
| 335A | 3.35e-8 m | 8.949e15 Hz | 504.3 s | 1.512e11 m | −0.0 s | −1.3 m | 240 s |

Kegel-Basis: 8.949e15 Hz … 1.332e18 Hz (2.17 Dekaden).

## Der Fehlerkreis

Schnitt der drei Kreise (je d_i ± 7.195e9 m): **leer** — die Lücke
**5.756e10 m** ist die gemessene Nichtübereinstimmung. d_eph = 1.511921e11 m.

## Verdikt

Die Ortung trifft den Kalibrier-Ursprung nicht innerhalb des Fehlerkreises —
der Schnitt der drei Kreise ist leer. **Der Offset ist die Zahl des Befunds:**

- **94A und 335A tragen den Ursprung innerhalb einer Zelle (24 s):** eps −24.0 s
  bzw. −0.0 s (d_eph liegt in beiden Fehlerkreisen).
- **XRSA trägt eps −240.0 s (−7.195e10 m)** — gemessen, nicht gedeutet.
- Offset gegen die Kreis-Mitte (Mittel der drei d_i): **2.638e10 m = 88.0 s
  Lichtzeit**.
- Der t_E-Anker ist die Korpus-Konvention (XRSB-Spitzen-Zelle − τ_lt), nicht
  die wahre Emissionszeit — jede eps_i trägt die XRSB-Antwortzeit mit; benannt,
  nicht herausgerechnet.

## Lieferung

Die drei Kegel, die gemessenen Δt, der Fehlerkreis, die Antwort gegen die
Sonne — alle Zahlen stehen oben. Register-Zeile im Register des Auftrags.

## Folge

Zwei Zeilen trägt das Blatt für die nächste Session:

- XRSA-Ausreißer −240 s: Quellen-Zeitstruktur oder Kanal-Taktung — nächste
  Messung ist die konditionale Sonde auf die drei Kanäle.
- Ein leerer Kegel-Schnitt mit 2/3 Konvergenz ist ein Ausreißer-Befund, keine
  Fehlortung.

