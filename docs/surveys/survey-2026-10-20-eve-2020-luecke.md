<!--
  title: EVE L2B 2020 — Lyman-α für die EUVS-Lücke, cross-kalibriert gegen GOES-15
  class: survey
  date: 2026-10-20
  sha256: aaf225befa578560ab9db6a77f4064b55f1544413f11c5ba5f19a68506c9c001
  status: live
  see-also: docs/surveys/survey-2026-10-20-kadenz-ernte.md docs/surveys/survey-2026-10-20-sonne-erde-kette.md
-->
# EVE L2B 2020 — Lyman-α für die EUVS-Lücke, cross-kalibriert gegen GOES-15

**Anlass.** GOES-15 EUVS endet am 2020-03-04; die Lücke 2020-04 … 2020-12
sollte mit SDO/EVE L2B `EVL` geschlossen werden. Die Frage war nicht, ob EVE
Lyman-α überhaupt liefert, sondern **wie viel** der 60-s-Reihe tatsächlich
Messung ist und ob sie gegen GOES-15 auf derselben Skala liegt.

**Grenze vorab.** Kein CDN-Upload (LASP `other-attribution`), kein Edit an
`phi/sources.φ`. Der Registerblock unten ist **Vorschlagstext**, nicht
angewandt. Die Rohdateien liegen lokal unter
`sources/lasp.colorado.edu/`; die Assets unter
`sources/lasp.colorado.edu/compiled/`. Beide Verzeichnisse sind im
Werkstatt-Repo gitignored.

## Quelle und Rohbestand

- Endpunkt: `https://lasp.colorado.edu/eve/data_access/eve_data/products/level2b/<YYYY>/<DDD>/EVL_L2B_<YYYYDDD>_008_01.fit.gz`
  — tägliche Datei, 60 s, 1.440 Zeilen, Version 008, Revision 01.
- Lizenz: LASP/EVE `other-attribution`; FITS-Kopf: „SDO Mission scientific and
  model results are open to all. Users should contact the PI … Appropriate
  acknowledgement …". Kein Weiterverkauf, keine stille CDN-Spiegelung.
- Geladen: **275 Dateien** für 2020-04-01 … 2020-12-31, **108.175.127 B** roh
  (Verzeichnis mit Manifest **108.203.177 B**), je Datei 190.104 … 974.827 B.
  Manifest: `sources/lasp.colorado.edu/eve_l2b_evl_2020/_manifest.tsv`
  (`sha256 aad5647ae50a22cd51c194914022c236558604f5b73311e811e761cc5999ce6e`).
- Gzip-Integrität aller 275 Dateien geprüft (`gzip -t`), keine void-Datei.
- Zusätzlich für die Kalibrierung geladen: 2020-01-01 … 2020-03-04 (64 Dateien,
  **23.685.620 B** inkl. Manifest,
  `sources/lasp.colorado.edu/eve_l2b_evl_2020_overlap/`). Der 2011er
  Überlappungssatz liegt bereits im Hauscache
  `sources/_inbox/2026-10-19/eve_cache/` (91 Tage 2011-02-09 … 2011-05-11 plus
  2014-03-01, 92 Dateien).

## FITS-Schnitt (gemessen)

Die Tabelle `LinesData` trägt 19 Spalten; `DIODE_IRRADIANCE` ist `6E`, die
Reihenfolge im Kommentar ist **QD, CH_8, CH_26, CH_30, CH_36, MEGSP**. Gelesen
wird Index **5** (MEGS-P, 121–122 nm), dazu `TAI` (`D`), `YYYYDOY` (`J`),
`SOD` (`D`), `FLAGS` (`B`), `SC_FLAGS` (`B`), `DIODE_PRECISION` (`6E`) und
`DIODE_ACCURACY` (`6E`). Zeilenbreite 1.274 B, 1.440 Zeilen je Tag.

**Zeit.** Primär aus `TAI`: `unix = TAI − 378.691.200 − ΔAT` (ΔAT aus dem
eingebetteten Leap-Second-Satz), danach `unix_to_tdb`. `YYYYDOY`/`SOD` dient
nur als Fallback. Der TAI-Abgleich lag für alle gelesenen Zeilen bei 0 s; die
eine Abweichung in 2020-04-04 (Zeile 720, `YYYYDOY` einen Tag zurück) wird
durch die TAI-Führung nicht wirksam.

**Flags.** `FLAGS` Bit 3 (8, MEGS-P missing) und Bit 7 (128, too many MEGS-P
integrations) → `absent`. `SC_FLAGS` ≠ 0 (Warmup, Eclipse, Penumbra, Umbra,
Off-Point) → `pending`. Nur `FLAGS`-sauber und `SC_FLAGS` = 0 zählt als
**gut**. Fehlende Werte sind Füllwert `-1.0`, nicht 0; sie werden nicht
geschrieben.

## Reihe 2020

**Asset:** `sources/lasp.colorado.edu/compiled/eve_lya1216_2020_04_12.bin`
— **52.707 Sätze** (50.138 gut + 2.569 pending), **1.686.632 B**,
`sha256 4d89757fb64e0ab8be5b45d72d024e2b4f9da666bedec0965bd6a4df7c40b4f9`.

| | |
|---|---|
| n gesamt | **52.707** (gut 50.138, pending 2.569) |
| erster guter Satz | 2020-04-01T08:53:30Z (`t_tdb 639.003.279,184`) |
| letzter guter Satz | 2020-12-31T17:57:30Z (`t_tdb 662.709.519,184`) |
| Kadenz | **60 s**; MEGS-P-Fenster täglich, Median **181** Samples, p90 **186**, max **755** |
| Feld | `eve_megsp_lya1216`, W/m² in 1 AU |
| Format | `EV20`, 32 B/Satz: `t_tdb f64`, `irradiance f64`, `flags u32` (Low-Byte `FLAGS`, nächtes Byte `SC_FLAGS`), `instrument u32` (2 = EVE MEGS-P), `precision f32`, `accuracy f32` |

**Abdeckung.** 275 Tage × 1.440 min = **396.000** Nominalzeilen. Gut sind
**50.138 = 12,66 %** der Kalenderminuten; mit pending **52.707 = 13,31 %**.
Das ist keine 24-h-Reihe: MEGS-P ist nur in einem Tagesfenster exponiert, im
Beispiel 2020-04-29 in den Zeilen 533–712 (08:53–11:52 UTC). Gegen das
nominelle 180-min-Fenster (275 × 180 = 49.500) sind 50.138 **101,3 %** — das
Fenster ist vollständig geerntet, einzelne Tage sind länger.

| Monat | Tage | gut | absent | pending | Abdeckung gut / Nominal |
|---|---:|---:|---:|---:|---:|
| 2020-04 | 30 | 5.410 | 37.790 | 0 | 12,52 % |
| 2020-05 | 31 | 6.276 | 38.364 | 0 | 14,06 % |
| 2020-06 | 30 | 5.370 | 37.830 | 0 | 12,43 % |
| 2020-07 | 31 | 5.561 | 39.079 | 0 | 12,46 % |
| 2020-08 | 31 | 5.682 | 36.389 | 2.569 | 12,73 % |
| 2020-09 | 30 | 5.551 | 37.649 | 0 | 12,85 % |
| 2020-10 | 31 | 5.598 | 39.042 | 0 | 12,54 % |
| 2020-11 | 30 | 5.131 | 38.069 | 0 | 11,88 % |
| 2020-12 | 31 | 5.559 | 39.081 | 0 | 12,45 % |

**Tage.** 272 von 275 Tagen tragen mindestens einen guten Satz; 271 Tage
≥ 100, 219 Tage ≥ 180. Drei Tage sind vollständig `absent`:
**2020-10-16, 2020-10-21, 2020-11-23** (MEGS-P durchgehend Füllwert `-1.0`;
am 2020-10-16 zusätzlich 46 Zeilen Lunar-Penumbra). Pending konzentriert sich
auf **2020-08-15 … 2020-08-28** (2.569 Sätze). Am 2020-08-15 sind es 221:
145 Warmup nach Erdschatten, 4 Atmosphären-Penumbra, 4 solide Erd-Penumbra,
68 solide Erd-Umbra.

## Cross-Kalibration gegen GOES-15

Gepaart wird über die **UTC-Minute** (GOES-15 60-s-Sätze und EVE 60-s-Sätze
liegen in derselben Minute), nur gute EVE-Sätze (`flags = 0`), nur
GOES-Lyman-α (`goes_euvs_lya1216`, Komponente 2). Angepasst wird
`EVE = a·GOES + b`; zusätzlich ist das Median-Verhältnis `EVE/GOES` geführt.
Die Überlappung liegt **vor** der Lücke: 2011 und 2020-01-01 … 2020-03-04.
Für 2020-04 … 2020-12 gibt es keinen GOES-15-Überlapp; die Skalierung wird
deshalb aus dem Überlappungssatz davor übertragen, nicht still als identisch
angenommen.

| Satz | EVE n | gepaart | a | b (W/m²) | r | Residuum (W/m²) | Median EVE/GOES |
|---|---:|---:|---:|---:|---:|---:|---:|
| 2011-02-09 … 2011-05-11 (+ 2014-03-01) | 27.104 | 26.784 | 1,071978 | −5,327e−4 | 0,970088 | 9,227e−5 | 0,996219 |
| 2020-01-01 … 2020-03-04 | 11.750 | 11.523 | 0,454262 | 3,306e−3 | 0,559136 | 7,148e−5 | 1,011390 |
| **kombiniert** | **38.307** | **38.307** | **0,950128** | **3,485e−4** | **0,986955** | **9,885e−5** | **0,999870** |

Der flache 2020-Slope ist kein Widerspruch zur Skala: in Jan–Mar 2020 ist die
GOES-Dynamik klein, der Fit wird vom Interzept getragen (r nur 0,559). Das
Median-Verhältnis ist in beiden Epochen stabil (0,996 bzw. 1,011). Für die
Merged-Reihe wird deshalb das **kombinierte Median-Verhältnis** benutzt:
`GOES = EVE / 0,999870`, nicht der 2011er Slope. Residuum kombiniert:
**9,885e−5 W/m²** = 1,46 % des mittleren GOES-Flusses 6,791e−3 W/m².

## Merged-Reihe (optional, explizit)

**Asset:** `sources/lasp.colorado.edu/compiled/eve_goes_merged_2010_2020.bin`
— **5.061.048 Sätze**, **161.953.544 B**,
`sha256 3457aeb40779b7a44da18bb8038de6dd6a088d974fe6dd288c925160ec7d494b`.
Format wie `EV20` (32 B/Satz), Magic `MERG`; `instrument` ist Pflicht:
**1 = GOES-15 EUVS roh**, **3 = EVE MEGS-P, mit `GOES = EVE / 0,999870`
skaliert**. EVE-Flags bleiben im Satz erhalten; GOES-Sätze tragen `flags = 0`.
Die Reihe mischt nicht still: die Instrumentenspalte sagt, welcher Arm trägt.
Die 2011er/2020er Überlappungssätze sind **nicht** in der Merged-Reihe, sie
dienen nur der Kalibrierung.

## Verdikt

**Die EVE-Linie trägt die Lücke — als 3-h/Tag-Fenster, nicht als 24-h-Reihe.**
Zahlen: **n gut = 50.138** (n gesamt 52.707), **Abdeckung 12,66 %** der
Kalenderminuten bzw. **101,3 %** des verfügbaren MEGS-P-Fensters;
**a = 0,950128**, **b = 3,485e−4 W/m²**, **r = 0,986955**,
**Residuum = 9,885e−5 W/m²**; 272/275 Tage tragen. Die Lücke 2020-04 … 2020-12
ist damit **teilweise geschlossen**: für Tagesmittel und 3-h-Fenster
brauchbar, für eine vollständige 24-h-60-s-Reihe nicht.

**Was `absent` bleibt.** 343.293 Nominalzeilen (86,69 %) außerhalb des
MEGS-P-Fensters bzw. mit Füllwert `-1.0`; drei ganze Tage (2020-10-16,
2020-10-21, 2020-11-23). **Was `pending` bleibt:** 2.569 Sätze mit
`SC_FLAGS` ≠ 0 (2020-08-15 … 2020-08-28), überwiegend Warmup/Erdschatten.
Keine dieser Zeilen ist als 0 geschrieben.

## Vorschlag Registerblöcke (nicht angewandt)

```
url https://github.com/omegaflow/sources/releases/download/lasp.colorado.edu-eve/eve_lya1216_2020_04_12.bin
terms other-attribution https://lasp.colorado.edu/eve/data_access/
format eve
origin https://lasp.colorado.edu/eve/data_access/eve_data/products/level2b/2020/
compiler tools/harvest/src/bin/eve_lya_2020_compiler.rs
sha256 4d89757fb64e0ab8be5b45d72d024e2b4f9da666bedec0965bd6a4df7c40b4f9
at sun
ttl 3600
field eve_megsp_lya1216_pt1m eve_megsp_lya1216_pt1m inverse-square em wm2_1au 60 0.0 0.0
```

```
url https://github.com/omegaflow/sources/releases/download/lasp.colorado.edu-eve/eve_goes_merged_2010_2020.bin
terms other-attribution https://lasp.colorado.edu/eve/data_access/
format eve_goes_merged
origin https://lasp.colorado.edu/eve/data_access/eve_data/products/level2b/ + ncei.noaa.gov/goes15/geuv-l2-avg1m/
compiler tools/harvest/src/bin/eve_goes_cal_probe.rs
sha256 3457aeb40779b7a44da18bb8038de6dd6a088d974fe6dd288c925160ec7d494b
at sun
ttl 3600
field lya1216_merged_pt1m lya1216_merged_pt1m inverse-square em wm2_1au 60 0.0 0.0
```

## Bilanz

| | Zahl |
|---|---:|
| geprüft | **275 Tagesdateien** 2020-04 … 2020-12 |
| geladen | **275** (108.175.127 B roh) |
| kompiliert | **52.707 Sätze** (50.138 gut, 2.569 pending) |
| cross-kalibriert | **38.307 Paare** (2011 + 2020-01 … 2020-03) |
| benannte Lücke | **1** (kein 24-h-Fenster: MEGS-P nur ~3 h/Tag; 3 ganze Tage absent) |

Die Messung trägt die Lücke für das, was EVE tatsächlich liefert. Sie
behauptet keine 24-h-Abdeckung, die der Instrumentbetrieb nicht hergibt.
