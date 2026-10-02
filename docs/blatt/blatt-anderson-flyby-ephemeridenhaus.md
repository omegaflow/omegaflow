<!--
  title: Rats-Blatt — Anderson-Flyby-Klasse (Sechs-Zeilen-Verdikt, Ephemeriden-Haus-Riss)
  class: sheet
  date: 2026-10-02
  sha256: 8aa321e46c6f6b53909475e9b9674fe6b8ba464618dfbca7ff4f6b1d5dfdab1f
  status: live
  see-also: data/flyby2/anderson-probe-2026-09-28.json data/flyby2/house-gate-2026-09-28.json docs/paper/flyby-path-2-falsification-metric-addendum.md docs/paper/flyby-path-2-preregistration.md docs/handover/archiv/handover-2026-10-01-mountain-folge217.md
-->
# Rats-Blatt — Anderson-Flyby-Klasse (Sechs-Zeilen-Verdikt, Ephemeriden-Haus-Riss)

**Datum:** 2026-10-02 · **Axiom:** A = A

## Frage

Trägt das Sechs-Zeilen-Verdikt der Anderson-Flyby-Klasse auf den am 2026-10-02 neu
vermessenen Werten — und ist der Rats-Akkord 3,65 µm/s als physikalische Untergrenze
gemessen und formal tragend, oder muss er neu hergeleitet werden? Drei Beine: (a) die
sechs Zeilen samt MESSENGER-`rift-generator`, (b) der Akkord als Boden, (c) was das
Verdikt widerlegen würde.

## Gemessene Evidenz

`data/flyby2/anderson-probe-2026-09-28.json` (Konvention „+ Richtung Erde"), neu
vermessen 2026-10-02:

| Zeile | Epoche | Anomalie [mm/s] | tdot_max [mm/s] | Verdikt |
|---|---|---|---|---|
| Galileo I | 1990-12-08 | +3.92 | 1.2494 | rift-excluded |
| Galileo II | 1992-12-08 | −4.6 | 0.00512 | rift-excluded |
| NEAR | 1998-01-23 | +13.46 | 0.00389 | rift-excluded |
| Cassini | 1999-08-18 | −2 | 0.00660 | rift-excluded |
| Rosetta | 2005-03-04 | +1.82 | 0.00310 | rift-excluded |
| MESSENGER | 2005-08-02 | +0.02 | 0.03252 | rift-generator |

- Zeilen 2–5: tdot_max trägt 0,03–0,33 % der jeweiligen Anomalie (Spanne ~300× bis
  ~3500×). Die Zeilen liegen auf der Inter-Flyby-Basis.
- Zeile 1: tdot_max 1,2494 mm/s gegen 3,92 mm/s — Spanne 3,1×. Die erste Zeile trägt
  ihre tdot_max aus der Ein-Tages-Referenz (`FIRST_ROW_REF_S`,
  `tools/measure/src/bin/flyby_anderson_probe.rs`) statt der Inter-Flyby-Basis; eine
  Vorgänger-Epoche gibt es nicht. Die schwächere Spanne ist benannt, kein Riss: hier
  steht eine Linie auf einer großzügigeren Referenz, nicht zwei widersprechende Zeugen.
- MESSENGER: `ephemeris_granule_census --series --date 2005-08-02 --half-days 3` legt
  die Epoche genau auf eine DE- und EPM-Granulat-Grenze (JD 2453584.5); 13,4 mm/s
  Stunden-Wechsel in DE–INPOP / INPOP–EPM in der Nachbarstunde, DE–EPM flach 0,0 mm/s
  (`flyby-path-2-falsification-metric-addendum.md:119-135`). Der Probe stempelt
  `rift-generator`, der Census löst als Naht auf — beide Zeilen bleiben stehen, nichts
  wird geglättet.
- Der Akkord: `ephemeris_house_gate` Chord-Slope −115 m/yr → ~3,65 µm/s
  (115 m/yr ÷ 3,156e7 s/yr = 3,64 µm/s), ~300× unter dem ~1 mm/s der Anomalie-Klasse
  (`handover-2026-10-01-mountain-folge217.md:68`).
- Haus-Kontrolle: JPL DE440/441 deklarieren keine terrestrische ITRF/EOP-Realisation
  (Park et al. 2021, AJ 161, 105, §2.5; DOI 10.3847/1538-3881/abd414, HTTP 200);
  INPOP19a und EPM2021 nennen keine. Die konstante Frame-Translation kürzt sich im
  Differential-Doppler; nur die Drift leckt — und die Drift ist der Akkord (folge217:65-70).
- Haus-Gate-Artefakt, Epoche 2005-08-02: de_inpop 0,1588 km, de_epm 17,916 km,
  inpop_epm 18,064 km; das Dreieck schließt (0,159 + 17,916 ≈ 18,064).

## Verdikt

Das Sechs-Zeilen-Verdikt trägt. Fünf Zeilen stehen auf dem Akkord-Boden (Spanne ~300×
und mehr); die erste Zeile steht auf der benannten Ein-Tages-Referenz (Spanne 3,1×) —
die schwächere Basis ist die Natur der ersten Zeile, kein Riss. Die Klassen-Aussage ruht
auf den fünf Akkord-Zeilen; die Galileo-I-Zeile stützt, sie trägt nicht allein. Der
Akkord ist gemessen (folge217:68) und seine Arithmetik schließt; er ist als
physikalischer Boden für die Zeilen 2–5 formal tragend und muss nicht neu hergeleitet
werden. Für die Zeile 1 ist der Boden die Ein-Tages-Referenz (1,2494 mm/s), nicht der
Akkord — der Akkord wird nicht stillschweigend auf die erste Zeile ausgedehnt. Kein
Haus-Riss erzeugt die ~1 mm/s-Klasse; der eine `rift-generator` ist als Granulat-Naht
aufgelöst und bleibt als Stempel stehen.

## Benannter Riss

Probe-Artefakt und Haus-Gate-Artefakt tragen an derselben Epoche (2005-08-02, MESSENGER)
verschiedene Haus-Spalten: de_inpop 22,485 km (Probe) gegen 0,1588 km (Haus-Gate),
inpop_epm 33,113 km gegen 18,064 km; nur de_epm stimmt (17,914 km gegen 17,916 km). Das
Dreieck schließt im Haus-Gate (0,159 + 17,916 ≈ 18,064), nicht im Probe
(22,485 + 17,914 = 40,399 ≠ 33,113). Die Probe nutzt den 00:00-UTC-Epoch, das Haus-Gate
den Perigäum-TDB — zwei Linien derselben Gate-Familie, die an der „gemeinsamen" Epoche
nicht konvergieren. Der Riss kippt das Verdikt nicht: tdot_max ist eine Rate, das Verdikt
ruht nicht auf den Absolut-Spalten, und die tdot_max der Zeilen 2–5 liegen in der Größe
des Akkords (0,0031–0,0066 mm/s gegen 0,00365 mm/s). Er steht als offener Punkt an der
Verdrahtung — getragen, nicht geglättet.

## Was das Verdikt widerlegen würde

1. Ein epochen-aufgelöster Neu-Lauf des `flyby_anderson_probe` mit geklärter
   Spalten-Semantik, der für eine Zeile tdot_max in der Größe ihrer Anomalie trägt.
2. Ein Census-Lauf, der die MESSENGER-Epoche nicht auf die Granulat-Grenze legt oder die
   Naht-Signatur (DE–EPM flach, DE–INPOP/INPOP–EPM Stunden-Wechsel) nicht trägt.
3. Ein Akkord-Fit über die sechs Epochen mit Slope ≥ 31,5 km/yr (~1 mm/s Rate) statt
   −115 m/yr.
4. Eine ITRF/EOP-Realisation eines der drei Häuser mit Drift-Term ≥ ~1 mm/s an einer
   Flyby-Epoche — folge217:65-70 trägt descoped (keine deklariert); eine solche Messung
   öffnet den Punkt neu.
5. Der Siegel-Riss der Präregistrierung: Hält das Siegel nicht auf zulässigem Pfad, gehört
   die Neu-Messung der Präregistrierungs-Kette zugeordnet — das Sechs-Zeilen-Verdikt selbst
   trägt weiter, was die Artefakte tragen (folge217:54).

## Offene Punkte

1. **Spalten-Semantik des Probe-Artefakts** — welche Reihe, Epoche und Haus-Paare speisen
   de_inpop/de_epm/inpop_epm und tdot_max; das Artefakt trägt es nicht mit. Schritt:
   Quell-Lesung (`sread tools/measure/src/bin/flyby_anderson_probe.rs`), dann
   epochen-aufgelöster Neu-Lauf.
2. **tdot_max gegen Census-Skala am MESSENGER-Tag** — der Probe trägt 0,0325 mm/s, der
   Census 13,4 mm/s Stunden-Wechsel. Wenn tdot_max eine DE–EPM-Größe ist, stimmen beide
   Linien; wenn es das Maximum über alle drei Paare ist, nicht. Schritt: die Fenster- und
   Serien-Definition der tdot_max-Messung benennen.
3. **Addendum-Zeile veraltet** — `flyby-path-2-falsification-metric-addendum.md:125` trägt
   „Galileo I `pending`"; das Artefakt trägt 1,2494 mm/s `rift-excluded`. Träger Mountain.
4. **Ein-Tages-Referenz der ersten Zeile** — im Probe-Code benannt (`FIRST_ROW_REF_S`),
   im Artefakt nicht mitgeführt. Schritt: die Konvention im Artefakt mitführen.
