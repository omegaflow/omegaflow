<!--
  title: Befund — Neptun-Zentrum-Riß: die drei Häuser (DE441/INPOP/EPM) divergieren am Planetenzentrum um 589/3227/3208 km Mittel — säkular wachsend, Minimum in den 1980ern (Voyager-2-Anker 1989), bis 9571 km in den 2020ern
  class: befund
  date: 2026-09-09
  sha256: f8720a29b0cdd1481ad1403b5d2f025c2f4ca0cb39484589231d2757f16dcccd
  status: done
  see-also: docs/befund/befund-2026-09-09-uranus-zentrum-versionenstruktur.md docs/handover/handover-2026-09-09-uranus-zentrum-kopplung.md docs/TODO.md
-->

# Befund: Neptun-Zentrum-Riß

## Frage & Bindung

Die Übergabe stellte Bau-Linie (c): Neptun als zweiter Planet desselben Baus.
Die Komposition steht (ephemeris_neptune_c.bin, verifiziert); dieser Befund
mißt den Riß — die Versionen-Differenz der drei Häuser am Neptun-Zentrum, der
Spiegel der Uranus-Versionenstruktur (Atom b). Probe:
`tools/measure/src/bin/neptune_center_rift_probe.rs`.

## Das Instrument

Der Probe liest das DE441-Zentrum direkt (`ephemeris_neptune_c.bin`) und die
INPOP/EPM-Baryzentren (`ephemeris_inpop_neptune.bin` / `ephemeris_epm_neptune.bin`)
+ die gemeinsame nep097xl-899-Verschiebung (899−8). Per Punkt (30-d-Raster,
1980–2040) die Vektor-Differenz der Zentren (ICRS, km) — ohne Astrometrie,
also reine Modell-gegen-Modell-Differenz.

## Die Messung

| Paar             | Δx (km) | Δy (km) | Δz (km) | \|Δ\| Mittel | \|Δ\| max |
|------------------|---------|---------|---------|--------------|-----------|
| de441 − inpop19a | −65     | −109    | −319    | 589          | 1092      |
| de441 − epm2021  | −2270   | +396    | +528    | 3227         | 9571      |
| inpop19a − epm   | −2205   | +505    | +847    | 3208         | 9668      |

Dekaden-Struktur (\|Δ\| Mittel je Dekade):

| Dekade | de441−inpop | de441−epm | inpop−epm |
|--------|-------------|-----------|-----------|
| 1970er | 582         | 2088      | 1889      |
| 1980er | 163         | 1210      | 1160      |
| 1990er | 249         | 1488      | 1527      |
| 2000er | 599         | 2652      | 2634      |
| 2010er | 884         | 4430      | 4449      |
| 2020er | 1054        | 7488      | 7583      |

Der Riß ist **kein konstanter Offset** — er hat ein Minimum in den 1980ern
(der Voyager-2-Neptun-Vorbeiflug 1989 ist der stärkste Distanz-Anker) und
wächst mit dem Extrapolations-Abstand in beide Richtungen, am steilsten in die
Zukunft (bis ~7.5·10⁶ m in den 2020ern). Dieselbe Eisriesen-Kluft wie Uranus,
nur tiefer.

## Der Bogen zur Weberin

Die Weberin-Zahl „neptune 1.07e6 m" ist ein Punktwert innerhalb dieser
Struktur (nahe dem Anker); die per-Punkt-Messung trägt den vollen Bogen
0.2–9.7·10⁶ m. Die drei Häuser trennen sich am stärksten dort, wo am weitesten
vom Anker extrapoliert wird.

## Kalibrier-Gate

Injiziert (+100, −50, +200) km in DE441 über ein 300-d-Fenster: zurückgewonnen
**+100.000 / −50.000 / +200.000 km** — exakt.

## Verdict

(c) trägt jetzt die Messung neben der Komposition: der Neptun-Riß ist eine
per-Punkt-Vektor-Größe mit säkularem Wachstum (Minimum am 1989-Anker, steiler
Zukunftsanstieg). Die Astrometrie-Kopplung bleibt die offene Hälfte — ohne eine
Neptun-Planetenzentrum-Tabelle bleibt der Riß Modell-gegen-Modell.

## Register-Zeilen

- Die Neptun-Astrometrie-Kopplung (Tabelle ernten, gegen das Zentrum
  reduzieren) — `pending`.
- Der physikalische Ursprung der vier de441-Trägerjahre (Uranus) — `pending`.
