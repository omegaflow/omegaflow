<!--
  title: Befund — Bz/LAIC n_surr=100: die Bz-Pfeile halten, die LAIC-Stille bleibt (der 10-Surrogat-Null trägt keinen Umzug)
  class: befund
  date: 2026-09-09
  sha256: 34276817234308b9ea9f5c1ff22fdaae540f514b65bf6dcdd4c9da4c5450981c
  status: done
  see-also: docs/handover/archiv/handover-2026-09-09-te-galileo-nsurr-folge.md docs/handover/archiv/handover-2026-09-08-nobel-dag-atom.md docs/befund/befund-klassen-benchmark-pcmci.md
-->
# Befund — Bz/LAIC n_surr=100: die Bz-Pfeile halten, die LAIC-Stille bleibt

## Frage & Bindung

Frage (die Duty aus dem Handover `handover-2026-09-09-te-galileo-nsurr-folge.md`):
Halten die Nobel-DAG-Befunde (Bz als gemeinsamer Treiber von AE/Dst/SYM-H, die
LAIC-Stille), wenn die Schwelle mean+2σ über **100** statt über **10** Surrogaten
geschätzt wird? Die alte 10-Null ist eine Kleinstichproben-Schätzung; der
Richtungsbefund Bz→AE 4,3× / Bz→Dst 2,4× (2015–2026 stündlich) hing an ihr. Der
Ist-Zustand bleibt, bis diese Erhebung ihn gegen 100 prüft. Kein stiller Shift.

Bindung (A = A): `nobel_probe_bz` rechnet den Binned-Schätzer mit Residual-Null;
`n_surr` steckt nur in der Schwellen-Schätzung (mean+2σ über n Surrogaten), nicht
in der TE-Schätzung — derselbe Baum, dieselben Daten, derselbe Seed, nur die
Null-Zählung wandert 10 → 100. `nobel_probe_laic` ist die Common-Cause-Kontrolle.
Dieselben CDN-Assets (`omni2_indices.bin`, `laic.bin`), derselbe Commit-SHA
`85898ec`. Der Befund spricht nur über das gemessene Sheet.

## Das Sheet

CI `te-bz-laic-nsurr100.yml` (Run 34401571351, done), zwei Punkte. Bz: 95483
gemeinsame Stunden (10,9 Jahre), Kanäle V n Bz |B| AE Dst; LAIC: 1846 Fenster
(1726 Ereignisse), 1400 Ereignis-Fenster in der Kanten-Statistik. Pfeil =
TE > mean+2σ. `load_indices` liest das CDN-Asset (der Fallback aus Commit
`f546019` trägt den Punkt — vorher „Bz unmeasured").

## Die Bz-Hälfte unter n_surr=100

Die Index-Kanäle messen jetzt; die Bz-Pfeile stehen:

| Kante | n=10 (alt) | n=100 |
|---|---|---|
| Bz→AE | 4,3× | lag1 4,67× (lag2 3,89×) |
| Bz→Dst | 2,4× | lag2 3,12× (lag1 2,42×) |
| Bz→SYM-H | 2,63× | lag2 3,08× (lag1 2,80×) |
| Bz→\|B\| | — | lag1 3,22× (lag2 2,31×) |
| Bz→n | — | lag2 4,27× (lag1 2,40×) |

Kein Bz-Pfeil kehrt um. Die ratio-Zahlen wandern, weil die Schwelle unter 100
Surrogaten neu geschätzt wird — die TE-Werte sind dieselben, die Kante hält; das
Wort „schärfer" trägt die Schwellen-Wanderung nicht als Kante (benannt). Die alte
4,3×/2,4× ist damit unter 100 nachgemessen — kein Artefakt der 10-Null.

Die direkte AE→Dst-Kante überlebt die Konditionierung auf Bz bei **1,64×** (lag2)
bzw. 1,58× (lag1); die Umkehr Dst→AE bleibt still (0,71×/0,70×). Unter n=10 war
die Kante „marginal (1,1×)". Der Probe selbst liest daraus eine
Substorm→Ringstrom-Kopplung, die Abweichung von Runges „keine direkte Kante" —
eine Lesart: Runges Betriebspunkt ist ein anderer, die Abweichung ist damit der
Verdikt des Probes, keine unabhängige Nachmessung. Die Rückkanten
(AE/Dst/SYM-H → Solarwind-Kanäle) stehen weiterhin als Pfeile — physikalisch
unmöglich, die zeitgleiche Kopplungs-Leckage des Schätzers, benannt, nicht
verschleiert.

## Die LAIC-Hälfte unter n_surr=100

Der mittlere Exzess ist in **jeder** Richtung negativ (der Surrogat-Floor-Bias,
kein gemessener Fluss) — die Stille hält unter der Common-Cause-Kontrolle:

| Richtung | Pfeile | mittlerer Exzess |
|---|---|---|
| litho→F | 71/1400 | −4,647e-2 |
| F→litho | 101/1400 | −2,246e-2 |
| Bz→F | 285/1400 | −2,847e-2 |
| Bz→litho | 127/1400 | −2,675e-2 |
| litho→Bz | 72/1400 | −4,900e-2 |
| F→Bz | 142/1400 | −3,301e-2 |

litho→F (0,051) und F→litho (0,072) sitzen auf demselben Floor — keine dominante
Lithosphäre↔Ionosphäre-Richtung. Die Sonnen-Kontrolle trägt den einzigen Rohwert
über dem Floor: Bz→F 0,204 > Floor 0,072 (unter n=10 war die Zahl 0,27) — ein
Rohwert-über-Floor-Befund, nicht ein Exzess-über-Surrogat (dessen mittlerer
Exzess ist auch für Bz→F negativ). Die Stille ist unter 100 nachgemessen.

## Der Verdikt-Satz

Unter n_surr=100 (Schwelle mean+2σ über 100 Surrogaten) halten alle Nobel-DAG-
Befunde: die fünf Bz→Index-Pfeile stehen mit ratio 2,31–4,67×, die direkte
AE→Dst-Kante trägt 1,64× (die Umkehr bleibt still), und die LAIC-Stille bleibt
(mittlerer Exzess überall negativ; der einzige Rohwert über dem Floor ist Bz→F
0,204 > 0,072). Die 10-Surrogat-Null trug weder einen Umzug noch eine Inflation —
der Betriebspunkt hält seine Schlüsse bei 100. Die Rückkanten-Leckage bleibt, was
sie war: benannt.

## Register-Satz

Register: **Bz/LAIC n_surr=100 — die 10-Surrogat-Null trägt keinen Umzug.** Die
Bz-Pfeile (AE 4,67×, Dst 3,12×, SYM-H 3,08×, |B| 3,22×, n 4,27×) halten unter
100; die AE→Dst-Kante ist unter 100 ein 1,64×-Pfeil (die Umkehr still); die
LAIC-Stille bleibt. Die Duty aus dem Handover ist beantwortet.

## Status

`done`. Messung: CI `te-bz-laic-nsurr100.yml` (Run 34401571351), Sheet
`bz-laic-nsurr100-sheet.txt` (SHA 85898ec).
