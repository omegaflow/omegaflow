<!--
  title: Befund — Mode-3 (three-way) Sample-Noise/Spec-Analyse st43 9457–9471 (das einzige ≥10-Tage-Fenster)
  class: befund
  date: 2026-09-05
  sha256: bfca9c9bc604a6ca06a5707280e2b321512d111ea0a310efb0906a499d876c8c
  status: done
  antwortet-auf: docs/befund/befund-galileo-mode3-und-s1-replikation.md
-->

# Befund — Mode-3 (three-way) Sample-Noise/Spec-Analyse st43 9457–9471

## Auftrag

G5 hat als einzige ≥-10-Tage-Serie irgendeiner Art **st43 9457–9471 (15 Tage)**
gemessen (three-way = Minderheitsmode des Tages, dom-1/2-geführt). Diese Serie ist
im G5-Register `pending` geblieben: Noise/Spec-Analyse auf der
three-way-Sample-only-Serie. Dieser Befund führt genau diese Messung aus und
urteilt, ob der drei-way-Kanal (Mode 3) an st43 auf diesen Tagen einen eigenen
Noise-/Spec-Charakter trägt.

## Daten & Messschritt

Quelle `data/galileo_resid.bin` (GASR). Reinigung unverändert (Konvention
`galileo_te_floor_direction`/`galileo_spec_te`): resid endlich, |resid| ≤ 1000 Hz,
strength ≠ 0. Selektion: st 43, Tage 9457–9471 (= 1995-11-23 .. 1995-12-07,
Tages-Konvention `unix_day = round(jd − 2440587.5)`, exakt G5). Mode-3-Samples =
ground_mode 3. Lock ausgeschlossen.

Sonde: `tools/measure/src/bin/galileo_mode3_st43_run.rs` (additiv,
`cargo check` 0 Warnungen). Spek-Scan spiegelt die Band-Sonde: Blöcke mit Lücke
≤ 60 s, Detrend nur Blöcke ≥ 120 Samples, LS 30–70 mHz @ 0,05 mHz, Floor =
Band-Median, Members = lokale Maxima ≥ 3× Floor; dazu das 44–56-mHz-Spiegelfenster
und die st43-Referenz 51,55 mHz. Mode 2 gleiche Tage, gleiche Methode = Kontext.

## Mode-3-Fenster-Selektion (gemessen)

| Kanal | cleaned Records | Tage |
|---|---|---|
| m2 (zwei-way) | 401 061 | 15 |
| m3 (three-way) | 131 124 | 15 |

Alle 15 Tage tragen ≥ 30 three-way-Samples — die G5-Fenster-Definition wird
reproduziert.

## Per-Tag-Noise (m3 = three-way, m2 = zwei-way, gleiche Tage)

`med` = Median\|resid\|, `rms` = sqrt(mean resid²); n = cleaned Samples des Tages
an st43.

| day | m2 n | m2 med Hz | m2 rms Hz | m3 n | m3 med Hz | m3 rms Hz | med 3/2 | rms 3/2 |
|---|---|---|---|---|---|---|---|---|
| 9457 | 25672 | 8.800e-2 | 2.328e0 | 5696 | 8.400e-2 | 8.890e-2 | 0.955 | 0.038 |
| 9458 | 3085 | 4.340e-1 | 1.080e0 | 870 | 9.400e-2 | 1.852e0 | 0.217 | 1.714 |
| 9459 | 4137 | 1.070e-1 | 4.358e-1 | 8871 | 8.600e-2 | 9.404e0 | 0.804 | 21.58 |
| 9460 | 25586 | 7.800e-2 | 8.122e-2 | 7346 | 7.900e-2 | 2.642e0 | 1.013 | 32.53 |
| 9461 | 33795 | 8.100e-2 | 2.180e0 | 14251 | 8.300e-2 | 1.044e-1 | 1.025 | 0.048 |
| 9462 | 36128 | 2.130e-1 | 2.150e-1 | 3323 | 2.600e-1 | 2.654e-1 | 1.221 | 1.235 |
| 9463 | 38532 | 2.060e-1 | 2.086e-1 | 6249 | 2.130e-1 | 2.163e-1 | 1.034 | 1.037 |
| 9464 | 8969 | 1.780e-1 | 1.823e-1 | 12563 | 1.890e-1 | 1.918e-1 | 1.062 | 1.053 |
| 9465 | 37623 | 1.620e-1 | 1.546e0 | 7400 | 1.850e-1 | 1.869e-1 | 1.142 | 0.121 |
| 9466 | 22114 | 1.410e-1 | 1.458e1 | 6266 | 1.550e-1 | 1.132e1 | 1.099 | 0.776 |
| 9467 | 44943 | 6.900e-2 | 4.999e0 | 13849 | 8.400e-2 | 2.491e-1 | 1.217 | 0.050 |
| 9468 | 43338 | 7.200e-2 | 8.256e-2 | 14993 | 6.700e-2 | 1.048e1 | 0.931 | 126.96 |
| 9469 | 41316 | 3.600e-2 | 6.681e-2 | 19452 | 4.700e-2 | 9.430e0 | 1.306 | 141.15 |
| 9470 | 13091 | 1.670e-1 | 6.031e-1 | 3679 | 1.310e-1 | 1.421e-1 | 0.784 | 0.236 |
| 9471 | 22732 | 7.340e-1 | 1.489e1 | 6316 | 4.070e-1 | 4.222e-1 | 0.554 | 0.028 |

## Floor über den Lauf (m3 vs m2, gleiche 15 Tage)

Gepoolt über Samples: m3 n 131124, m2 n 401061.

| Metrik | m2 | m3 | Ratio m3/m2 |
|---|---|---|---|
| gepoolter Median \|resid\| | 1.110e-1 Hz | 1.010e-1 Hz | 0.910 |
| gepoolte RMS | 5.300e0 Hz | 6.189e0 Hz | 1.168 |
| Tages-Mittel (Median der Tages-Mediane) | 1.410e-1 Hz | 9.400e-2 Hz | 0.667 |
| Tages-Mittel (Median der Tages-RMS) | 6.031e-1 Hz | 2.654e-1 Hz | 0.440 |

Tagesstruktur: beide Kanäle teilen dieselbe Tag-für-Tag-Lage (ruhig 9467–9469:
m3 0,047–0,084 Hz, m2 0,036–0,072 Hz; erhöht 9462–9466: beide ~0,15–0,26 Hz;
gestört 9471: m3 0,407 Hz, m2 0,734 Hz). Die RMS ist auf diesen Tagen in beiden
Kanälen von seltenen Ausreißer-Exkursionen dominiert (Tages-RMS bis ~10¹ Hz,
Verhältnis 3/2 streut 0,03–141,1 über die Tage) — die RMS ist pro Tag kein
stabiler Kanal-Diskriminator; die Mediane sind es.

## Spec-Scan 30–70 mHz (drei-way-Serie, st43, Lauf)

Kadenz der drei-way-Serie über den Lauf ist fein genug: 22 Blöcke (Lücke ≤ 60 s),
längster Block 16 933 Samples (3,0 h); 131 070 von 131 124 Samples liegen in
Blöcken ≥ 120 (detrendbar). Sampler: 1,0 s ×131 087, 60 s ×35, 22 s ×1, 33 s ×1.

| Kanal | n_scan | Band-Floor | Band-Peak | 44–56-mHz-Peak | ref 51,55 mHz | Members ≥ 3× |
|---|---|---|---|---|---|---|
| m2 (Kontext) | 400 994 | 8.98e1 | 34.0 mHz 4.5× | 49.5 mHz 3.2× (2.9× Subwin) | 0.4× Floor | 26 |
| m3 (three-way) | 131 070 | 1.01e2 | 34.2 mHz 4.7× | 54.2 mHz 4.6× (4.5× Subwin) | 0.3× Floor | 40 |

m3-Members: dichter Wald über das ganze Band (stärkste 34,2 mHz 4,7×, 54,2 mHz
4,6×, 58,05–58,9 mHz ~4,0–4,4×, 62,5–63,6 mHz ~3,6–3,7×); m2 zeigt denselben
Wald (26 Members, Band-Peak 34,0 mHz 4,5×). Die 51,55-mHz-Spiegelreferenz liegt in
beiden Kanälen unter dem Floor (m3 0,3×) — die Linie ist im three-way-Kanal nicht
vorhanden.

Signifikanz-Schranke (Mess-Eigenschaft der LS-Leistung): unter weißem Rauschen ist
die LS-Leistung pro Frequenz exponentiell verteilt; die Wahrscheinlichkeit, dass
eine einzelne Frequenz k× den Median übersteigt, ist 2^(−k). Bei der 3×-Schwelle
überschreiten erwartungsgemäß ~1/8 der 801 Gitterfrequenzen die Schwelle — der
beobachtete Wald (40 m3-Members, Spitzen nur 4,6–4,7×) liegt in dieser
Rausch-Erwartung. Keine isolierte schmale Linie; kein m3-Member ragt über das
m2-Rauschmuster hinaus.

## Verdikt (Rat gehalten, 2026-09-05)

- **Floor:** Der three-way-Kanal (Mode 3) an st43 ist auf diesen 15 Tagen nicht
  lauter als der zwei-way-Kanal (Mode 2) — zentraler Median gepoolt 0,910×,
  Tages-Ebene 0,667× (m3 unter m2); beide Kanäle teilen dieselbe Tages-Lage. Der
  Kanal ist damit in seiner zentralen Rauschlage **nicht distinkt** von Mode 2.
- **RMS:** Auf diesen Tagen von seltenen Exkursionen dominiert (beide Kanäle),
  kein stabiler Kanal-Unterschied pro Tag.
- **Spec:** Der three-way-Kanal trägt **keine kohärente Struktur** im 30–70-mHz-
  Band: der LS-Wald liegt in der Rausch-Erwartung, die Spiegel-Referenz 51,55 mHz
  ist abwesend (0,3× Floor), und der Spektral-Charakter gleicht dem von Mode 2
  derselben Tage.
- 0 honored: Es gibt keinen Hinweis auf einen eigenständigen drei-way-Kanal-
  Charakter; die Fenster-Daten-Grenze von G5 (three-way = Minderheitsmode, kein
  mode-3-dominiertes Fenster) bleibt unverändert bestehen.

## Register (was bleibt pending)

- Der Lauf ist die einzige ≥-10-Tage-three-way-Sample-Serie der Mission, aber kein
  mode-3-dominiertes Fenster — ein Urteil über drei-way-**Dominanz** bleibt
  ausstehend (G5-Register, unverändert).
- Per-Tag-Spec-Scans der Serie sind nicht ausgeführt (pro Tag zu wenige
  zusammenhängende Samples für eine Band-Auflösung); nur der Lauf-gepoolte Scan
  ist gemessen.
- Der Ursprung der Tages-RMS-Exkursionen (transiente Ereignisse) ist benannt,
  nicht untersucht.
- Ein Surrogat-Signifikanz-Gate für den LS-Wald ist nicht gelaufen; statt dessen
  steht die analytische Exponential-Erwartung 2^(−k) als Mess-Eigenschaft der
  LS-Leistung.

## Status

`done` (2026-09-05). Sonde `tools/measure/src/bin/galileo_mode3_st43_run.rs`,
`cargo check` 0 Warnungen.
