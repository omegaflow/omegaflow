<!--
  title: BLATT PAPIER — Kreuz-Screening der Gyirong-Serien (voller Katalog)
  class: sheet
  date: 2026-08-30
  source: meteo_harvest/tibet-flut-2026
  series: gyirong_open-meteo_*
  window: 2026-08-18..27 UTC
  tool: cross_te_screen
  lags: 1,6,12,24
  surrogate: 20
  min-n: 100
  cond: gyirong_open-meteo_temperature_2m (Tagesgang-Proxi)
  sha256: c610dd0ec353c5bd45476a0d4bc6f7723739cea9934addd5362eabc013e6deba
  status: live
-->

# BLATT PAPIER — Kreuz-Screening der Gyirong-Serien (voller Katalog)

**Verdikt-Ordnung:** 0 honored — kein Wert erfunden; fehlende Felder bleiben
`pending`.

> **Richtungskorrektur (registriert):** Der bisherige Lauf druckte die
> Pfeilrichtung invertiert. Die Bibliothek ist maßgeblich —
> `transfer_entropy_lag(x, y) = TE(y → x)` (zweites Argument = Quelle). Die
> Richtungen in diesem Blatt sind die korrigierten. (Kontrollbeleg:
> synthetischer Driver→Effect-Lauf in `/tmp/opencode/dircheck`.)

## Zusammenfassung

Volles Kreuz-Screening der 51 gestündeten Open-Meteo-Serien der Station
Gyirong (M5.2-Trishuli-Lawine 2026, siehe `meteo_harvest/tibet-flut-2026`).
Das Screening sucht unvoreingenommen in allen Kanälen — nicht nur in
kanalisierender Vorauswahl.

**Befunde (gemessen, nicht behauptet):**

1. **Ein starker gemeinsamer Treiber dominiert das Fenster.** Die höchsten
   unbedingten TE-Werte (TE 0.36–0.39) bilden einen Lag-6-Cluster quer durch
   Strahlung ↔ Wind ↔ Druck. Dieser Cluster ist **nicht** ein gerichteter
   Kausalpfeil, sondern die Synchronisierung durch den diurnalen/synoptischen
   Zyklus (Tagesgang + Passat). `gemessen`: er ist über der Schwelle, trägt
   aber keine Richtung.
2. **`wind_speed_10m` ist die breiteste Quelle moderater unbedingter TE** —
   auch das ist der gemeinsame Treiberkonfund. Nach Konditionierung auf den
   Tagesgang (`temperature_2m`) bleibt davon **kein** Pfeil in den
   Niederschlag hinein übrig.
3. **Niederschlags-Treiber (die Flutfrage) — `gemessen`:** Die unbedingten
   Pfeile in `precipitation`/`rain` (`total_column_integrated_water_vapour`,
   `dew_point_2m`, `surface_pressure`, `pressure_msl`, `wind_speed_10m`,
   `wind_gusts_10m`, `wet_bulb_temperature_2m`, `apparent_temperature`,
   `leaf_wetness_probability`; TE 0.13–0.20) **überleben die Konditionierung
   auf den Tagesgang nicht**. Es gibt **0** bedingte Pfeile in
   `precipitation`/`rain`. Befund: **kein lokaler Meteorologie-Kanal treibt
   den Niederschlag direkt über den gemeinsamen Tagesgang/synoptischen Zyklus
   hinaus.** Die früher vermuteten Druck-/Wind-Treiber waren Artefakte des
   gemeinsamen Zyklus.
4. **Niederschlag als Quelle (bedingt, `gemessen`):** Umgekehrt blockiert
   Niederschlag/Regen die Sonnenstrahlung — `precipitation`/`rain →
   {global_tilted_irradiance, shortwave_radiation, direct_normal_irradiance,
   et0_fao_evapotranspiration, sunshine_duration}`, Lag 1/12, cTE ≈ 0.081–0.089.
   Diese Pfeile überleben den Tagesgang (Regen → weniger Einstrahlung).
5. **Artefakte getrennt:** `is_day` (deterministisch 0/1) und `weather_code`
   (kategorial) erzeugen 289 der 1728 unbedingten Funde. Sie werden als
   Quellen ausgeschlossen, nicht als Kausalität gezählt.

**Konsequenz (0 honored):** Nach Konditionierung auf den Tagesgang bleiben 687
bedingte Pfeile (cTE > mean+2σ), getragen von Druck/Feuchte/Strahlung
(Wasserdampf- und Druckkanäle — der synoptische Zyklus jenseits des Tagesgangs).
Für die Flutfrage ist das Verdikt `gemessen`: **kein lokaler Kanal treibt den
Niederschlag direkt**; der Niederschlag folgt dem gemeinsamen Zyklus. Die
Richtungskorrektur ist eingearbeitet; ein offener Baustein bleibt die
räumliche Kopplung (Rasuwa→Gyirong, siehe Kollab-Blatt).

## 1. Methode

- Jede Serie `{"points":[{t,v}]}` aus `meteo_harvest/tibet-flut-2026`
  (`gyirong_open-meteo_*`). `null`-Werte (not a measured value) werden
  übersprungen, nicht geglättet.
- Pro Paar × Richtung × Lag: `transfer_entropy_lag(ziel, quelle, lag)` gegen
  `surrogate_threshold_lag` (phasenrandomisierte Surrogate, mean + 2σ,
  n_surrogate = 20). Signifikant, wenn `TE > Schwelle`.
- **Bedingte Transfer-Entropie** `transfer_entropy_conditional(x, y, c, lag)` =
  `TE_{c}(y → x)`: wie viel `y`'s Vergangenheit die Zukunft von `x` zusätzlich
  erklärt, gegeben den Tagesgang-Proxi `c` = `temperature_2m`. Null-Surrogat:
  Residuen-Surrogat (Quelle auf `c` regressiert, Residuen permutiert,
  treiber-ausgerichteter Anteil erhalten) — so wird die geteilte Diurnale im
  Nullwert nicht erzeugt, sondern gerade ausgeschaltet. Schwelle mean + 2σ,
  n_surrogate = 20.
- Das Screening filtert nicht (0 honored): es listet jeden signifikanten
  Pfeil. 51 Katalog-Kanäle, 32 mit n ≥ min-n = 100 nutzbar, 496 Paare,
  4 Lags → 1984 Vergleiche, 1728 unbedingt signifikant (87 %), 687 bedingt.
- Lauf (isoliert in `/tmp/opencode/gyirong_te`):
  `cross_te_screen --dir /tmp/opencode/gyirong_te --lags 1,6,12,24
  --surrogate 20 --min-n 100 --cond gyirong_open-meteo_temperature_2m`
  → `/tmp/opencode/gyirong_cond.out`.

## 2. Daten (geerntet, 0 honored)

- Quelle: `open-meteo archive-api`, Stundentakt, `window` 2026-08-18…27 UTC.
- 51 Gyirong-Kanäle des vollständigen Katalogs geerntet; 19 unter n = 100
  (inkl. `precipitation`/`rain`/`snowfall`-Nullspuren bei einzelnen Serien)
  ausgeschlossen, keine geglättet.
- Kein Wert nachgetragen; A=A, jede nicht messbare Größe als Fehlstelle.

## 3. Ergebnisse (vollständiges Screening)

### 3.1 Gesamtzahl signifikanter Pfeile

1728 / 1984 Vergleiche signifikant (87 %). Ohne `is_day`/`weather_code`
(289 Artefakte) bleiben 1439.

### 3.2 Stärkste unbedingte Funde (gemeinsamer Treiber)

Lag-6-Cluster, TE ≈ 0.36–0.39 (Strahlung ↔ Wind ↔ Druck):

- `diffuse_radiation → wind_gusts_10m` lag 6 (TE 0.386 > 0.157)
- `wind_gusts_10m → surface_pressure` lag 6 (TE 0.382 > 0.178)
- `direct_normal_irradiance → pressure_msl` lag 6 (TE 0.380 > 0.161)
- `shortwave_radiation → wind_gusts_10m` lag 6 (TE 0.377 > 0.130)
- `global_tilted_irradiance → pressure_msl` lag 6 (TE 0.372 > 0.144)
- `pressure_msl → surface_pressure` lag 6 (TE 0.365 > 0.199)

Alle Teilnehmer derselben diurnalen Synchrongruppe → nicht als Richtung
interpretierbar; der Tagesgang-Anteil wird in §3.4 durch Konditionierung auf
`temperature_2m` abgetrennt.

### 3.3 Niederschlags-Treiber (Flutfrage, `gemessen`)

**Unbedingt** (korrigierte Richtung) führen signifikante Pfeile in
`precipitation`/`rain`, TE 0.13–0.20:

- `total_column_integrated_water_vapour → precipitation` lag 6 (TE 0.202)
- `dew_point_2m → precipitation` lag 6 (TE 0.171)
- `leaf_wetness_probability → precipitation` lag 6 (TE 0.152)
- `wet_bulb_temperature_2m → precipitation` lag 6 (TE 0.150)
- `apparent_temperature → precipitation` lag 6 (TE 0.144)
- `surface_pressure → precipitation` lag 24 (TE 0.150)
- `pressure_msl → precipitation` lag 6 (TE 0.136)
- `wind_speed_10m → precipitation` lag 12 (TE 0.140)
- `wind_gusts_10m → precipitation` lag 6 (TE 0.137)

**Bedingt auf den Tagesgang (`temperature_2m`): 0 Pfeile in
`precipitation`/`rain`.** Keiner der obigen überlebt. Befund (`gemessen`):
**kein lokaler Kanal treibt den Niederschlag direkt über den gemeinsamen
Tagesgang/synoptischen Zyklus hinaus.** Die Druck-/Wind-/Feuchte-Kandidaten
waren Artefakte des gemeinsamen Zyklus, keine isolierbare Kausalität.

**Niederschlag als Quelle — überlebt den Tagesgang (`gemessen`):**
Regen blockt Einstrahlung:

- `precipitation/rain → global_tilted_irradiance` lag 12 (cTE 0.0889)
- `precipitation/rain → shortwave_radiation` lag 12 (cTE 0.0889)
- `precipitation/rain → et0_fao_evapotranspiration` lag 12 (cTE 0.0889)
- `precipitation/rain → direct_normal_irradiance` lag 12 (cTE 0.0839)
- `precipitation/rain → sunshine_duration` lag 1 (cTE 0.0811)
- `rain → apparent_temperature` lag 24 (cTE 0.0899)

### 3.4 Bedingte TE auf den Tagesgang (687 Pfeile)

Konditionierung auf `temperature_2m` entfernt den Diurnal-Konfund. Die
stärksten überlebenden Kanäle (cTE 0.30–0.38) tragen den synoptischen
Moisture-/Druck-Zyklus jenseits des Tagesgangs:

- `total_column_integrated_water_vapour → leaf_wetness_probability` lag 6 (cTE 0.380)
- `total_column_integrated_water_vapour → wet_bulb_temperature_2m` lag 6 (cTE 0.357)
- `cloud_cover_high → wet_bulb_temperature_2m` lag 6 (cTE 0.328)
- `total_column_integrated_water_vapour → wind_speed_10m` lag 6 (cTE 0.325)
- `surface_pressure → relative_humidity_2m` lag 6 (cTE 0.311)
- `surface_pressure → vapour_pressure_deficit` lag 6 (cTE 0.304)

### 3.5 Kein Befund

- `showers`, `snowfall`: keine signifikanten Pfeile (zu sparse Serien).
- 19 Kanäle unter n = 100: ausgeschlossen (0 honored), nicht geglättet.
- **Kein** bedingter Pfeil in `precipitation`/`rain` (die Flutfrage).

## 4. Grenzen (0 honored)

- Surrogat-Schwelle (mean + 2σ) allein trennt **nicht** den gemeinsamen
  Treiber vom gerichteten Pfeil — genau die Lage hier (87 % unbedingt
  signifikant). Die bedingte TE auf `temperature_2m` trennt den Tagesgang ab.
- Die Konditionierung entfernt den **linearen** Anteil, der mit
  `temperature_2m` ausgerichtet ist; ein residualer synoptischer Konfund
  (Feuchte/Druck-Lag-6-Band) bleibt in §3.4 sichtbar und ist nicht per
  Ein-Proxi-Konditionierung auszuschließen. Ein Fenster, ein Proxi.
- `is_day`/`weather_code`: determinstische/kategoriale Serien erzeugen
  Schein-Signifikanz; als Quellen ausgeschlossen.
- Ein-Fenster-Screening (1 Event, 10 Tage): keine Verallgemeinerung über
  Ereignisse (Bordeaux, Aaretal, Japan offen).

## 5. Schlussfolgerung

Das Gyirong-Vollkatalog-Screening misst einen starken gemeinsamen diurnalen
Treiber (Lag-6-Cluster, Strahlung↔Wind↔Druck) und einen breiten moderaten
Kanal `wind_speed_10m`. Für die Flutfrage ist das Verdikt **`gemessen`**:
**kein lokaler Kanal treibt den Niederschlag direkt über den gemeinsamen
Tagesgang/synoptischen Zyklus hinaus** (0 bedingte Pfeile in
`precipitation`/`rain`). Umgekehrt ist Regen als Quelle auf die
Einstrahlungs-Kanäle (Blockierung) `gemessen` (cTE 0.081–0.089). Ein
residualer synoptischer Moisture-/Druck-Kanal bleibt nach Tagesgang-
Konditionierung bestehen (§3.4). Offener Baustein: räumliche Kopplung
(Rasuwa→Gyirong) — im Blatt `blatt-kreuz-screening-kollab.md`, dessen Pfeile
noch die invertierte Richtungskonvention tragen und zu korrigieren sind.
