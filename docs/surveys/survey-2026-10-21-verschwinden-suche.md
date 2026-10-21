<!--
  title: Survey — Verschwinden statt Erscheinen: Forced Photometry am DDF-Feld (2026-10-21)
  class: survey
  date: 2026-10-21
  sha256: 5f5fdfe34794a6622d36b27c95f74eb205cfde4b876618493ebd2c913744069a
  status: live
  owner: river
  ack: river 2026-10-21T10:07Z
  see-also: docs/paper/nadel-v-fresh-area-dip-scan.md tools/measure/src/bin/disappearance_probe.rs tools/measure/data/scan_coverage.φ
-->
# Survey — Verschwinden statt Erscheinen: Forced Photometry am DDF-Feld (2026-10-21)

**Frage.** Wird eine feste Stelle ohne Alert leiser? Statt auf ein neues
Objekt zu warten, wird die vorhandene Lichtkurve an einer festen Position
erzwungen photometriert und die späte Hälfte gegen eine Baseline aus der
frühen Hälfte geprüft. Der Kandidat ist ein Verschwinden, nicht ein
Erscheinen.

## Feld und Zugang (gemessen 2026-10-21)

| | |
|---|---|
| Feld | DDF, ICRS `ra 148.8746° dec +2.5208°` |
| Suchkegel | 10″; Fink/LSST `conesearch` liefert **1 Objekt** (Separation **0,06″**) |
| Objekt | `diaObjectId 313998569858662581` |
| Forced Photometry | Fink/LSST `POST /api/v1/fp` — HTTP **200**, **409.433 B** |
| Zeilen | **1.033** FP-Zeilen; **1.032** mit `scienceFlux > 0`; 1 Zeile mit Flux ≤ 0 ist ein gemessener Wert, kein `absent` (und keine erfundene 0,0) |
| Bänder | g 197, i 385, r 188, u 47, y 20, z 195 Messzeilen (z hat 196 FP-Zeilen) |
| Zeitachse | `r:midpointMjdTai` (MJD/TAI) → Unix → TDB über die eingebettete `naif0012.tls` |
| Credential | keins; beide Fink-Endpunkte antworten anonym |

## Methode

1. Feste Position, kein Alert: der 10″-Kegel wählt das nächste `diaObject`,
   danach wird dessen gesamte Forced Photometry geholt.
2. Je Band werden nur Zeilen mit endlichem `scienceFlux > 0` als Messzeilen
   geführt. Negative und Null-Flüsse sind gemessene Werte — nicht `absent`,
   aber auch keine erfundene 0,0.
3. Die Zeilen werden zeitlich sortiert und bei `n/2` in zwei disjunkte
   Fenster geteilt: frühe Hälfte = Baseline, späte Hälfte = Test. Unter
   `N_MIN 6` Zeilen je Fenster ist das Band `pending` (zu wenige Zeilen) und
   trägt keine Schwelle.
4. Baseline: Median und MAD der frühen Hälfte; robuste Streuung
   `σ = 1,4826 · MAD`. Die untere Schwelle ist **Median − 3σ**. Sie kommt
   ausschließlich aus der Baseline, nicht aus einer Vorgabe.
5. `vanishing`, wenn der späte Median unter dieser Schwelle liegt; sonst
   `stable`. Die Drop-Zahl ist `z = (Median_ref − Median_test) / σ`.

## Ergebnis: 0 von 6 Bändern verschwinden

| Band | n ref/test | Baseline-Median | MAD | σ = 1,4826·MAD | Schwelle Median − 3σ | Test-Median | Drop z | Urteil |
|---|---:|---:|---:|---:|---:|---:|---:|---|
| g | 98 / 99 | 26.849,367 | 234,965 | 348,358 | **25.804,291** | 26.975,648 | −0,36 | stable |
| i | 192 / 193 | 35.812,359 | 257,637 | 381,973 | **34.666,441** | 35.819,824 | −0,02 | stable |
| r | 94 / 94 | 36.352,007 | 235,457 | 349,089 | **35.304,741** | 36.595,261 | −0,70 | stable |
| u | 23 / 24 | 24.058,756 | 418,494 | 620,459 | **22.197,378** | 23.954,503 | +0,17 | stable |
| y | 10 / 10 | 48.014,642 | 695,046 | 1.030,475 | **44.923,216** | 49.554,145 | −1,49 | stable |
| z | 97 / 98 | 39.109,863 | 394,410 | 584,752 | **37.355,606** | 39.425,439 | −0,54 | stable |

Fluss in der Fink-Spalte `scienceFlux` (nJy). Die Baseline-Fenster spannen
6,0–31,0 d, die Test-Fenster 32,0–109,7 d; das Feld ist über rund 110 d
abgedeckt. Kein Band liegt unter seiner aus der Baseline gelesenen Schwelle;
das tiefste `z` ist −1,49 (y) und bleibt damit über −3.

| Bilanz | Zahl |
|---|---:|
| feste Felder geprüft | **1** |
| Objekte mit Forced Photometry | **1** |
| Bänder mit belastbarer Baseline | **6** |
| Kandidaten (`vanishing`) | **0** |
| `stable` | **6** |
| `pending` (Band, zu wenige Zeilen) | **0** |
| FP-Zeilen / Messzeilen | **1.033 / 1.032** |

**Kandidatenzahl: 0.** Das ist eine Null über ein Feld und ein Objekt, kein
Zensus. Die Messung ist trotzdem scharf: jedes der sechs Bänder trägt eine
eigene, aus seiner frühen Hälfte gelesene Schwelle, und die späte Hälfte
bleibt in allen sechs darüber.

## Coverage-Register: wo wurde geschaut

`tools/measure/data/scan_coverage.φ` enthält **8 strukturierte Kegelzeilen**.
Das DDF-Feld liegt **außerhalb** des nächstgelegenen registrierten Kegels
(`ra 53,1951° dec −28,8044°`, Radius 900″, Instrument `fink-lsst`, Datum
2026-09-06): Separation **346.284,2″ = 96,19°**. Für dieses Feld gab es
also noch keine Registerzeile; die hier gemessene 10″-Kegelzeile ist neue
Abdeckung, keine Wiederholung.

**Register-Kontrolle (CDF-S).** Der registrierte Kernel `ra 53,1250°
dec −28,1000°` (900″) liegt exakt auf seinem Kegel (Separation 0,0″ und
`inside`). Der nächstgelegene `diaObject` dort ist
`313994143532581107` (Separation 23,56″); seine Forced Photometry antwortet
HTTP 200 mit **0 Zeilen** (2 B, leeres Array). Dieses Objekt ist damit
`pending` (keine FP-Zeile = nicht gemessen), nicht `absent`: dass keine
Zeile kommt, heißt nicht, dass das Objekt nicht existiert. Keine Baseline,
keine Schwelle — nicht `stable`, nicht `vanishing`.

## Was quer liegt

- Die Baseline ist die frühe Hälfte derselben Fink-LSST-Forced-Photometry.
  Ein systematischer Offset zwischen früher und späterer Kadenzen wird nicht
  durch eine zweite Survey gegengeprüft; die Aussage ist eine
  Eine-Survey-Lichtkurve, kein unabhängiger Nachweis.
- Die Fenster sind ungleich lang (Baseline 6–31 d, Test 32–110 d). Der
  Median ist gegen die unterschiedliche Kadenz robust, die Streuung der
  Baseline ist es nicht automatisch; `y` hat nur 10 + 10 Zeilen und die
  breiteste relative Schwelle.
- Das Register kennt das DDF-Feld nicht. Die Survey dokumentiert den Kegel,
  trägt ihn aber nicht in `scan_coverage.φ` ein; das wäre eine eigene
  Registerhandlung außerhalb dieser zwei Artefakte.
- Kein Kanalausfall: `conesearch` und `/api/v1/fp` antworten anonym mit 200.
  Die 0 ist deshalb ein Messwert über 1.032 Zeilen, kein Null-Ersatz für
  einen stummen Kanal.
