<!--
  title: Befund — Galileo In-Pass-Elevations-Kontrolle: die Boden↔Rauschen-Kovarianz an 43/63 überlebt den Vergleich bei angeglichener Elevation
  class: befund
  date: 2026-09-05
  sha256: f5a81158f27acccbdd0c469c441128323198443a986a04c621cf14f6b73087cb
  status: done
  antwortet-auf: docs/befund/befund-galileo-inpass-staerke-rampe.md
  see-also: docs/befund/befund-galileo-mode1-fingerabdruck.md
-->
# Befund: Galileo In-Pass-Elevations-Kontrolle — die 43/63-Boden↔Rauschen-Kovarianz überlebt angeglichene Elevation (genuiner SNR-Term mit Elevations-Anteil in der Amplitude)

## Frage & Bindung

Der Vorgänger-Befund (`befund-galileo-inpass-staerke-rampe`) zeigte an Station 43 und 63 eine
Boden↔Rauschen-Kovarianz bei Pass-Identität (Boden-Unter-Arcs lauter als Plateau-Unter-Arcs
desselben Passes) und nannte als offene Grenze: Boden- und Plateau-Unter-Arcs sitzen innerhalb des
Passes auf verschiedener Elevation — pass-gleich, nicht punktgleich. Die ausstehende Messung ist die
Elevations-Kontrolle: überlebt die Boden-Lautheit, wenn Boden- und Plateau-Unter-Arcs desselben
Passes bei *angeglichener* Elevation verglichen werden?

Gebunden (unverändert vom Rampen-Befund): Pass = durchgehender Tracking-Arc je (Station, Modus),
Pass-Grenze = Zeitlücke > 600 s; Lock-Übergänge (|resid| > 1000 Hz) getrennt; Stärke-Zustand je
Probe: Boden = `signal_strength` ≤ −2560 (AGC-Klemmwert), Plateau ≥ −1900, dazwischen und 0 als
Übergang/Pad ausgeschlossen; Unter-Arc = durchgehender Lauf gleichen Zustands (getrennt bei
Zustandswechsel, > 120-s-Lücke, 60 Proben); Rauschen = Resid-RMS um den Unter-Arc-Mittelwert;
Unter-Arc ab 30 Nicht-Lock-Proben. Dual = Boden-Pool und Plateau-Pool je ≥ 30 Proben. Datenkette
`data/galileo_resid.bin` (GASR). Neue additive Sonde
`tools/measure/src/bin/galileo_elevation_match.rs`, `cargo check` 0/0 Warnings; Report auf stdout,
keine Report-Datei. Modus 1 an Stationen 14/43/63; Modus 2 nur Struktur (Elevation nicht vermessen).

## Die Elevations-Sonde (Proxy und Näherung)

Proben-Elevation = sphärisch-astronomische topozentrische Elevation der Sonde über dem Stations-
Horizont je Probe. Die Sonde ist weit (Probe-Erde 1–6 AU über die Ära); die Stations-Parallaxe ist
vernachlässigbar, die Elevation ist der Winkel der Sonden-Richtung über der lokalen Horizontalebene.

- Sonden-Richtung (ICRS): `galileo_daily`-Baryzentrum minus Erd-Baryzentrum
  (`body_barycenter_position`), umgerechnet in RA/Dec.
- Stations-Position (geodätisch, `odp::dsn_station`): DSS 14 35,4268333° N, −116,8900000° E;
  DSS 43 −35,4014889° N, 148,9816167° E; DSS 63 40,4312500° N, −4,2487778° E.
- Lokale Sternzeit = GMST + östliche Länge; GMST = 280,46061837° + 360,98564736629°/Tag ×
  (JD − 2451545), JD = tdb/86400 + 2451545.
- Elevation = asin(sin φ sin δ + cos φ cos δ cos HA), Horizont = 0°.
- Näherungen: tdb ≈ UT1 (Abweichung ≤ ~1 min ≈ 0,25°); RA (ICRS/J2000) gegen GMST (Äquinoktium des
  Datums) ≤ ~0,5°; Kugel-Horizont ohne Refraktion und ohne Antennen-Maske. Fehler ≪ 1° — klein gegen
  die 3–5°-Bänder.

**Proxy-Validierung (gemessen, nicht behauptet):** (1) Pass-Gating — in zwei Fenstern
(DSS 43, 1997-01-03..05; DSS 63, 1995-12-15..16) liegen alle klassifizierten Proben genau in den
positiv-elektiven Stunden des Proxys (DSS 43: p1/p50/p90 = 8,6/44,2/73,5°, alle 78 576 Proben über
5°; DSS 63: 9,4/22,0/26,3°, alle 26 167 über 5°) — die Daten enden an der realistischen DSN-Maske
(~5–10°). (2) Chunk-mittlere Elevationen je Jahr sind physikalisch (DSS 43 1994–1997: Mittel
44,9–50,2°; DSS 63: 19,9–21,2°; DSS 14: 23,8–25,7°; 0 % der Chunks unter 5°). (3) Die archivinterne
Erdorientierung (`body_fixed_to_icrs_smooth`-Spinmodell bzw. die ~täglichen Rotationsmatrizen) wurde
geprüft und verworfen: sie versetzt die Stationen gegen die Lehrbuch-Geometrie um stations-abhängige
Phasen (bis ~12 h), gemessen an der Rückgewinnung der geodätischen Position und am Vergleich mit der
GMST-Lehrbuch-Elevation; die Lehrbuch-Sonde ist die verwendete.

## n zuerst (0 geehrt)

Modus 1 an 14/43/63: 9 567 949 Proben, 1 413 861 Lock-Übergänge. Duale Pässe (Boden- und
Plateau-Pool ≥ 30): st14 21, st43 25, st63 13. Proben ohne Elevation: 0.

Die unbehinderte Anker-Replikation reproduziert den Rampen-Befund exakt (voll: st43 med Diff
1,828 Hz, 20/5, Ratio 9,69; st63 18,827 Hz, 9/4, Ratio 35,3; interior st43 1,162 Hz 19/5, st63
4,757 Hz 9/4) — dieselbe Pass-/Unter-Arc-Konstruktion trägt die Elevations-Messung.

## Tabelle 0 — Elevations-Geometrie der Zustände innerhalb dualer Pässe (Modus 1)

| St | n_dual | med Boden-Elev. | med Plateau-Elev. | med Δ (Boden−Plateau) | Boden tiefer | Boden < 5° unter Plateau |
|---|---|---|---|---|---|---|
| 14 | 21 | 26,4° | 25,7° | +1,0° | 9 / 21 | 4 |
| 43 | 25 | 10,2° | 45,1° | −22,5° | 20 / 25 | 1 |
| 63 | 13 | 17,2° | 20,8° | −7,9° | 9 / 13 | 2 |

Die Grenze des Vorgänger-Befunds ist damit gemessen: **der AGC-Boden sitzt innerhalb desselben
Passes auf deutlich tieferer Elevation** — an 43 median 10° gegen 45° (Δ −22,5°, in 20 von 25
Pässen tiefer), an 63 17° gegen 21° (Δ −7,9°, 9 von 13). Der Boden liegt in der Rise-/Set-Nähe des
Passes, das Plateau nahe der Kulmination. Die Geometrie-Konfundierung innerhalb des Passes ist real;
die Elevations-Kontrolle ist die richtige Frage.

## Tabelle 1 — gepaarte In-Pass-Rauschen bei angeglichener Elevation, Test A (gemeinsames Elevations-Fenster der Breite 2T, beste Lage je Pass, beide Pools ≥ 30 Proben), Modus 1

| St | 2T | n_pass_match | med Boden RMS | med Plateau RMS | med Diff | mean Diff | Boden>Plateau | med Ratio |
|---|---|---|---|---|---|---|---|---|
| 14 | 6° | 21 | 0,769 | 0,316 | −0,021 | 13,44 | 9 / 12 | 0,91 |
| 43 | 6° | 24 (1 ohne) | 1,942 | 0,086 | 0,898 | 62,06 | 19 / 5 | 9,71 |
| 63 | 6° | 13 | 6,480 | 0,144 | 6,423 | 62,21 | 10 / 3 | 31,5 |
| 14 | 10° | 21 | 0,769 | 0,806 | −0,037 | 10,00 | 8 / 13 | 0,86 |
| 43 | 10° | 25 | 2,286 | 0,072 | 1,211 | 71,43 | 20 / 5 | 11,6 |
| 63 | 10° | 13 | 5,035 | 0,183 | 4,979 | 59,55 | 10 / 3 | 33,1 |
| 14 | 16° | 21 | 0,777 | 0,780 | −0,011 | 11,60 | 9 / 12 | 0,94 |
| 43 | 16° | 25 | 2,080 | 0,073 | 1,820 | 72,87 | 20 / 5 | 12,3 |
| 63 | 16° | 13 | 18,902 | 0,469 | 18,889 | 66,15 | 9 / 4 | 34,7 |

Über alle drei Fensterbreiten bleibt der Boden an 43/63 lauter als das Plateau desselben Passes
**bei derselben Elevation**: 43 floor>plateau 19–20 von 24–25 Pässen, med Ratio 9,7–12,3;
63 10 von 13 (bzw. 9 von 13 beim 16°-Fenster), med Ratio 31–35. Station 14 bleibt flach
(med Diff ≈ −0,02 bis −0,04 Hz, 8–9 / 13 Pässe) — auch bei angeglichener Elevation kein
Boden-SNR-Term.

## Tabelle 2 — gepaarte In-Pass-Rauschen in derselben Elevations-Bande (Test B: probe-genaue 5°-Bande, Pass-Identität UND Bande-Identität, beide Pools ≥ 30), Modus 1

| St | n (Pass,Bande)-Paare | med Boden RMS | med Plateau RMS | med Diff | Boden>Plateau | med Ratio |
|---|---|---|---|---|---|---|
| 14 | 29 | 0,523 | 0,542 | 0,004 | 15 / 14 | 1,06 |
| 43 | 40 | 1,942 | 0,324 | 0,820 | 34 / 6 | 4,25 |
| 63 | 16 | 14,024 | 0,482 | 1,632 | 11 / 5 | 4,39 |

Der strengste Vergleich — gleicher Pass, gleiche 5°-Elevations-Bande, probe-genau — hält die
Richtung: 43 34/6 (Boden lauter), 63 11/5; 14 15/14 (flach). Die med Ratio fällt aber deutlich
(43: 9,7 → 4,3; 63: 35 → 4,4): Ein Teil der Boden-Lautheit ist mit der tiefen Elevation kollokiert
und fällt im Band-Vergleich weg; die *Richtung* Boden-lauter-als-Plateau überlebt.

## Der Befund

**1. Der AGC-Boden sitzt innerhalb dualer Pässe auf tieferer Elevation als das Plateau — die
Geometrie-Konfundierung des Rampen-Befunds ist real und jetzt vermessen.** 43: med Boden 10,2° gegen
Plateau 45,1° (Δ −22,5°, 20/25 Pässe tiefer); 63: 17,2° gegen 20,8° (Δ −7,9°, 9/13). Der Boden
liegt an der Rise-/Set-Flanke des Passes.

**2. Die 43/63-Boden↔Rauschen-Kovarianz überlebt die angeglichene Elevation — sie ist keine reine
Elevations-Artefakt.** Bei gemeinsamem 10°-Elevations-Fenster (Test A) bleibt der Boden an 43
2,286 Hz gegen Plateau 0,072 Hz (20/5, Ratio 11,6) und an 63 5,04 Hz gegen 0,18 Hz (10/3, Ratio
33); im 6°-Fenster (24/25 bzw. 13/13 Pässe matchbar) ebenso. Im strengen 5°-Banden-Vergleich
(Test B, Pass- und Bande-Identität) bleibt der Boden in 34/6 (43) und 11/5 (63) Pässen lauter.
Wäre die Boden-Lautheit nur die Elevation (niedrige Elevation ⇒ Rauschen), müsste sie bei gleicher
Elevation verschwinden — sie bleibt.

**3. Die Elevation erklärt Teile der Amplitude, nicht die Richtung.** Die med Ratio fällt im
5°-Band-Vergleich (43: 9,7 → 4,3; 63: 35 → 4,4) und die extremen Boden-RMS (> 300 Hz) haben
Plateau-Vergleichswerte nur auf der tiefen, floor-nahen Elevation. Der gemessene In-Pass-Gradient
ist damit zweigeteilt: eine genuine Boden↔Rauschen-Kovarianz bei Zustands-Identität (überlebt) plus
eine Elevations-Kollokation in der Amplitude (Boden an Rise/Set).

**4. Station 14 bleibt auch unter Elevations-Kontrolle ohne Boden-SNR-Term** (Test A med Diff
−0,02 bis −0,04 Hz, 8–9/21; Test B 15/14, Ratio 1,06) — die Stations-Asymmetrie (43/63 ja, 14
nein) des Fingerabdrucks und des Rampen-Befunds ist nicht durch Elevation erzeugt, denn 14 trägt
Boden- und Plateau-Zustände bei vergleichbarer Elevation (Test B n = 29 Paare).

## Grenzen

- Elevations-Proxy ist die Lehrbuch-GMST-Sphärik (Näherungen oben, Fehler ≪ 1°); keine
  Stations-Parallaxe (bei 1–6 AU vernachlässigbar), keine Refraktion, keine Antennen-Maske. Die
  archivinterne Erdorientierung wurde als unbrauchbar vermessen und nicht verwendet (gemessen:
  Stations-Phasenfehler bis ~12 h gegen die Lehrbuch-Geometrie; ~tägliche Rotationsmatrizen zu grob).
- Test A wählt je Pass das beste gemeinsame Fenster; der Vergleich findet damit überwiegend auf der
  tiefen, floor-nahen Elevation statt, wo beide Zustände Proben tragen. Boden-Pools dort teils klein
  (Boden-n 30–120); stabile Mediane und Vorzeichen-Zählungen sind die tragenden Grössen.
- Zeitliche Pfeilrichtung innerhalb des Passes bleibt ungemessen (simultane Zustands-Kovarianz).
- Stärke-Skala unkalibriert; Modus 2, Modus 3, kleine Stationen und die Elongations-/Sonnendistanz-
  Achse im Zusammenspiel mit der Elevation bleiben pending.
- Der 5°-Band-Vergleich (Test B) hält Zustands-Pools in derselben Bande; Banden nahe der Maske
  (~5–10°) tragen die floor-lauten Paare, Kulminations-Banden sind plateau-dominiert.

## Register-Satz

*Der AGC-Boden sitzt innerhalb dualer Pässe auf tieferer Elevation als das Plateau (43: 10,2° gegen
45,1°, Δ −22,5°, 20/25; 63: 17,2° gegen 20,8°, Δ −7,9°, 9/13) — die Grenze des Rampen-Befunds ist
damit gemessen. Bei angeglichener Elevation überlebt die Boden↔Rauschen-Kovarianz an 43/63: im
gemeinsamen 10°-Fenster Boden 2,29 gegen 0,072 Hz (43, 20/5, Ratio 11,6) und 5,04 gegen 0,18 Hz
(63, 10/3, Ratio 33); im strengen 5°-Banden-Vergleich desselben Passes 34/6 (43) und 11/5 (63)
Pässe mit Boden lauter — die Kovarianz ist genuin (SNR), keine reine Elevations-Artefakt; die
Elevation erklärt Teile der Amplitude (med Ratio 9,7→4,3 bzw. 35→4,4), nicht die Richtung. Station
14 bleibt auch bei angeglichener Elevation flach (Test B 15/14, Ratio 1,06) — die
Stations-Asymmetrie ist keine Elevations-Wirkung. Elevations-Proxy: Lehrbuch-GMST-Sphärik (tdb≈UT1
~0,25°, RA vs Äquinoktium ~0,5°), validiert über Pass-Gating und Jahres-Chunk-Elevationen; die
archivinterne Erdorientierung wurde als unbrauchbar vermessen. Offen (pending): zeitliche
Pfeilrichtung im Pass, Modus 2/3, kleine Stationen, Elongations-Achse × Elevation.*

## Status

`draft`. Sonde `galileo_elevation_match.rs` additiv, `cargo check` 0/0; Report auf stdout.
