<!--
  title: Befund — Grat-Folge: Trishuli Regen→Pegel unter Konditionierung (dritter Pfeil)
  class: befund
  date: 2026-09-05
  status: done
  sha256: 4c59cbcb4bbf647379aeef9b783f58c823b706029156e449325f1b4ce359c016
  see-also: antwortet-auf: docs/auftrag/auftrag-grat-trishuli-konditionierung.md docs/auftrag/auftrag-der-grat.md docs/paper/blatt-pfeil-sturzflut-tibet.md docs/paper/blatt-kreuz-screening-kollab.md docs/blatt/blatt-der-grat.md
-->

# Befund: Trishuli Regen→Pegel unter Konditionierung auf den geteilten synoptischen Treiber

**Datum:** 2026-09-05 · **Axiom:** A = A · **Verdikt-Ordnung:** 0 honored —
kein Wert erfunden; jede Lücke als `pending` benannt.

## Antwort auf die Auftragsfrage

Die Frage des Auftrags: Übersteht die co-lokale Regen→Pegel-Kopplung des
Quell-Blattes (`blatt-pfeil-sturzflut-tibet.md` §3.5, TE 0.265 > 0.218,
Lag 24 h, n = 129) die Konditionierung auf den gemeinsamen synoptischen
Treiber — gyirong temperature_2m als Tagesgang-Proxi, pressure_msl und
relative_humidity_2m als unabhängige Zweit-Proxies, Residuen-Surrogat,
mean + 2σ?

**Verdikt:** Der Pfeil fällt in die Kollab-Stille — auf dem kalibrierten
Instrument misst die im Quell-Blatt als Regen→Pegel registrierte Kopplung
bei Lag 24 h TE 0.223 unter ihrer unbedingten Schwelle (0.246), ist also
nie über der Schwelle; die registrierte Zahl 0.265 gehört der
Gegenorientierung (Pegel→Regen), und die bei anderen Lags unbedingt
messbaren Regen→Pegel-Orientierungen (Lag 1–12 h, 48 h) fallen unter der
Konditionierung auf den geteilten Treiber aus der Signifikanz.

## Das Messergebnis in einem Satz

Die dritte Grat-Zeile trägt keinen dritten vollständig gemessenen Pfeil:
Regen→Pegel ist bei Lag 24 h auf dem richtungs-kalibrierten Instrument
nicht über der Schwelle, und keine Regen→Pegel-Orientierung des Sweeps
übersteht die Konditionierung auf die drei synoptischen Proxies — die
Zeile gehört der Stille, mit einer Richtungs-Korrekturpflicht an das
Quell-Blatt.

## Daten (geerntet, keine neue Ernte)

Verwendet sind die Reihen, die das Quell-Blatt nennt — der DHM-Gauge
Bhotekoshi at Rasuwagadhi (ID 4913) überlappend mit dem stündlichen
Rasuwa-Niederschlag (n = 129, Vor-Flut-Fenster) und die regionalen
Open-Meteo-Stationsreihen der Kreuz-Screenings (n = 240):

| Reihe | Herkunft (auf Platte) | n | Fenster (UTC) |
|---|---|---|---|
| Pegel DHM Bhotekoshi/Rasuwagadhi, stündlich | `dhm_bhotekoshi_stage_1h.csv` + `dhm_stage_align.csv`, archiviert im externen Archivbestand, physisch `knowledge/archive/data/opencode-tmp-2026-09-01/worktree-aufraeum/` | 129 | 08-20 18:00 … 08-26 02:00 |
| Niederschlag Rasuwa, stündlich | Open-Meteo archive-api, `phi/pipeline/meteo_harvest/tibet-flut-2026/rasuwa_open-meteo_precipitation.json`; die 129 Werte am Gauge-Raster == `precip_rasuwa_align.csv` (0 Abweichungen, geprüft) | 129 | dieselben 129 Stunden |
| Treiber-Proxi gyirong temperature_2m / pressure_msl / relative_humidity_2m | `phi/pipeline/meteo_harvest/tibet-flut-2026/gyirong_open-meteo_<var>.json` (n = 240, 08-18…27), auf das 129-Stunden-Raster des Gauges geschnitten | 129 je Proxi | dieselben 129 Stunden |

Keine benötigte Reihe war abwesend; alle drei Proxies lagen auf Platte.
Der Schnitt der Proxies auf das Gauge-Raster ist dieselbe Fenster-Restriktion,
die die n = 129-Überlappung definiert.

## Instrument und Richtungs-Kalibrierung (gemessen)

- Der Schätzer ist die skalare KDE-Transfer-Entropie
  `transfer_entropy_lag(x, y, lag)` in `src/mathematikerin/te.rs`: das
  Dichte-Produkt faktorisiert p(x[t+lag], x[t], y[t]) — das zweite Argument
  trägt den gegenwärtigen Treiber-Wert für die Zukunft des ersten Arguments
  (AGENTS.md: „zweites Argument = Quelle"). `cross_te_screen` reicht die
  Reihen so, dass der gedruckte Pfeil „a → b" die a-Seite als Quelle führt;
  die bedingte Entropie `transfer_entropy_conditional(x, y, c, lag)` folgt
  derselben Orientierung (Null = Residuen-Surrogat, Quelle auf den Treiber
  regressiert, Residuen permutiert, mean + 2σ).
- **Kalibrierungsläufe (durchgeführt, nicht behauptet):** (1) synthetische
  Treiber→Response-Reihe: `cross_te_screen` druckt die Treiber-Seite als
  „a → b" signifikant (TE 0.607), die Gegenrichtung nicht;
  `te_pair_probe` druckt dieselbe Messung unter der Gegenrichtung
  („Response → Treiber") — die a→b-Beschriftung von `te_pair_probe` ist
  gegen die Schätzer-Orientierung gespiegelt. (2) Räumliche Kontrolle im
  Kollab-Fenster (n = 240): `cross_te_screen` reproduziert die
  physikalisch validierten Pfeile des Kreuz-Screenings exakt —
  `rasuwa_precip → kollab_precip` Lag 6 TE 0.2085, `kollab_precip →
  rasuwa_precip` Lag 12 TE 0.1750 — mit der a-Seite als Quelle.
- **Konsequenz für das Quell-Blatt:** §3.5 wurde mit `te_pair_probe`
  (a = Niederschlags-Datei, b = Pegel-Datei) gelaufen; die gedruckte
  „TE(a→b)"-Spalte ist die Schätzer-Orientierung Pegel→Regen. Die Spalte,
  die das Blatt „TE(Niederschlag→Pegel) 0.265" nennt, ist im Schätzer die
  Orientierung mit der Pegel-Reihe als Quelle. Die registrierte
  Pfeilrichtung des co-lokalen Gauges ist gegen die Schätzer-Orientierung
  gespiegelt (derselbe Fehler-Typ, den die Richtungskorrektur des
  Kollab-Blattes für die älteren Blätter registriert). Die Zahlen selbst
  sind reproduziert; die Richtungs-Benennung ist die Korrekturpflicht.

## Lauf 1 — Reproduktion des Quell-Blattes (unbedingt, `te_pair_probe`)

Lauf: `te_pair_probe --a rasuwa_precip_129 --b dhm_stage_129 --lags 1,3,6,12,24,48`.
Die 129-Werte-Reihen reproduzieren die Tabelle §3.5 des Quell-Blattes
vollständig (alle zwölf Zahlen identisch):

| lag | TE(a→b)-Spalte | Schwelle | TE(b→a)-Spalte | Schwelle | Befund (Blatt) |
|---|---|---|---|---|---|
| 1 | 1.596e-1 | 1.765e-1 | 1.195e-1 | 1.111e-1 | Pegel→Niederschlag |
| 3 | 1.817e-1 | 1.830e-1 | 1.787e-1 | 1.938e-1 | kein Befund |
| 6 | 1.520e-1 | 2.012e-1 | 1.952e-1 | 1.711e-1 | Pegel→Niederschlag |
| 12 | 2.546e-1 | 2.254e-1 | 2.597e-1 | 2.202e-1 | beide |
| 24 | 2.652e-1 | 2.178e-1 | 2.232e-1 | 2.310e-1 | Niederschlag→Pegel |
| 48 | 3.428e-1 | 2.466e-1 | 2.717e-1 | 2.464e-1 | beide |

**Richtungs-Lesart (gemessen):** die Spalten tragen die
`te_pair_probe`-Beschriftung. In der Schätzer-Orientierung ist die Spalte
mit 0.265 bei Lag 24 die Pegel→Regen-Messung; die Regen→Pegel-Messung
liegt bei 0.223 und ist dort unter ihrer Schwelle.

## Lauf 2 — unbedingt + konditioniert (`cross_te_screen --cond`, n = 129)

Lauf je Proxi: `cross_te_screen --dir <reihen> --cond <proxi> --lags 1,3,6,12,24,48
--surrogate <n>` (a = rasuwa_precip_129, b = dhm_stage_129; → die Spalte
„a→b" ist die Regen→Pegel-Orientierung, „b→a" die Pegel→Regen-Orientierung).
Unbedingte Schwelle: mean + 2σ, 10 Surrogate, je Paar-Richtung (wie im
Kreuz-Screening); bedingte Schwelle: Residuen-Surrogat mean + 2σ,
n = 20 (Temperatur) bzw. n = 40 (Druck, Feuchte) — exakt die
Surrogat-Zahlen der Kreuz-Läufe §3.2/§3.2b.

| lag | TE(Regen→Pegel) | Schwelle | TE(Pegel→Regen) | Schwelle |
|---|---|---|---|---|
| 1 | 1.195e-1 | 1.111e-1 | 1.596e-1 | 1.823e-1 |
| 3 | 1.787e-1 | 1.593e-1 | 1.817e-1 | 1.974e-1 |
| 6 | 1.952e-1 | 1.896e-1 | 1.520e-1 | 1.876e-1 |
| 12 | 2.597e-1 | 2.001e-1 | 2.546e-1 | 2.133e-1 |
| 24 | **2.232e-1** | **2.460e-1** | 2.652e-1 | 2.232e-1 |
| 48 | 2.717e-1 | 2.572e-1 | 3.428e-1 | 2.694e-1 |

Unbedingt signifikant bei Lag 24 h ist die **Pegel→Regen-Orientierung**
(TE 0.2652 > Schwelle 0.2232); die Regen→Pegel-Orientierung liegt bei
Lag 24 h unter ihrer Schwelle (TE 0.2232 < 0.2460) — kein Pfeil in der
Richtung des Auftrags bei dessen Lag.

**Konditionierte Befunde (überlebende Pfeile, TE > bedingte Schwelle):**

| Proxi (Surrogat n) | Lag | Orientierung | cTE | bedingte Schwelle |
|---|---|---|---|---|
| gyirong temperature_2m (20) | 24 | Pegel→Regen | 0.2977 | 0.2629 |
| gyirong temperature_2m (20) | 48 | Pegel→Regen | 0.2998 | 0.2852 |
| gyirong pressure_msl (40) | 24 | Pegel→Regen | 0.3151 | 0.2695 |
| gyirong pressure_msl (40) | 48 | Pegel→Regen | 0.3635 | 0.3017 |
| gyirong relative_humidity_2m (40) | 24 | Pegel→Regen | 0.3118 | 0.2810 |
| gyirong relative_humidity_2m (40) | 48 | Pegel→Regen | 0.3174 | 0.2976 |

**Regen→Pegel unter der Konditionierung:** keine Regen→Pegel-Orientierung
irgendeines Lags überlebt eine der drei Konditionierungen (der Lauf druckt
unter-Schwellen-Werte nicht; kein Regen→Pegel-Wert erscheint in den
konditionierten Befunden). Die unbedingt signifikanten
Regen→Pegel-Orientierungen der kurzen Lags (Lag 1–12 h: TE 0.119–0.260)
und bei Lag 48 h (0.272) fallen also unter der Konditionierung auf den
geteilten synoptischen Treiber aus der Signifikanz — sie sind auf diesem
Instrument nicht isolierbar.

## Die Pflichten des Laufs

1. **Lag-Sweep:** gelaufen über die Prüf-Lags des Probes {1, 3, 6, 12, 24, 48}
   (Lag 0 ist kein Sweep) — beide Richtungen, unbedingt und bedingt
   (Tabellen oben).
2. **Kontrollrichtung:** Pegel→Regen lief als Kontrolle mit; sie ist die
   einzige überlebende Orientierung. Die Zielrichtung Regen→Pegel fällt —
   bei Lag 24 h bereits unbedingt unter der Schwelle, bei den übrigen Lags
   unter der Konditionierung.
3. **Mehrfachvergleich:** der Lauf meldet jede getestete Konstellation
   (6 Lags × 2 Richtungen × unbedingt + 3 Konditionierungen) vollständig;
   das gebaute Instrument der Kreuz-Screenings wendet je Test die
   Surrogat-Schwelle mean + 2σ an und trägt keine globale Familien-Schranke —
   dieselbe Praxis wie die Kreuz-Läufe, an die der Auftrag bindet.
4. **KDE-Bandbreiten-Sensitivität (h, Faktor 2): `pending`.** Das gebaute
   Instrument (`transfer_entropy_conditional`, `cross_te_screen`) legt die
   Silverman-Bandbreite intern fest und setzt keinen Bandbreiten-Faktor aus;
   neuer Code ist nicht Gegenstand des Auftrags. Die Zelle bleibt `pending`
   (0 honored) — kein h-Wert wird fabriziert.

## Grenzen

- **Richtungs-Benennung des Quell-Blattes:** die in
  `blatt-pfeil-sturzflut-tibet.md` §3.5, `causal-arrow-preregistration.md`
  und der Grat-Zeile registrierte „Regen→Pegel"-Benennung des 0.265-Werts
  ist gegen die Schätzer-Orientierung gespiegelt (Kalibrierung oben). Der
  vorliegende Befund misst die Orientierungen und benennt beide; die
  Bilanz-Korrektur ist Sache des Quell-Blattes.
- **Ein Fenster, ein Gauge, n = 129:** das Vor-Flut-Fenster; gemessen ist
  Pegel (m), nicht Abfluss (m³/s); der Flut-Peak selbst wurde nie
  aufgezeichnet.
- **Schätzer:** die unbedingte Schwelle des Instruments ist der 10er-Shuffle-
  Surrogat-Null (mean + 2σ), nicht der Phasen-Null; die bedingte Schwelle
  ist das Residuen-Surrogat. Beide sind die des gebauten Kreuz-Instruments.
- **KDE-h-Zelle `pending`** (Pflicht 4) — das Verdikt steht auf dem gebauten
  Instrument ohne Bandbreiten-Sensitivität.

## Schluss

Der Lauf steht. Die dritte Zeile der Grat-Bilanz festigt sich nicht: die
als Regen→Pegel registrierte Kopplung ist bei Lag 24 h auf dem
richtungs-kalibrierten Instrument nicht über der Schwelle, und keine
Regen→Pegel-Orientierung übersteht die Konditionierung auf die drei
geteilten synoptischen Proxies. Die Zeile gehört der Kollab-Stille; die
Richtungs-Benennung der registrierten 0.265 ist die Korrekturpflicht, die
dieser Befund an das Quell-Blatt weitergibt.

Was nicht gemessen werden kann, wird nicht behauptet.
