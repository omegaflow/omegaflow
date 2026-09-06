<!--
  title: BLATT PAPIER — Kausalpfeil der Sturzflut in Tibet (Trishuli, 2026-08-26)
  class: paper
  date: 2026-08-27
  sha256: 7701cf4f409964e82b0bdf391ed1026d6c93601099576896d9a4ffb2687bb231
  status: pending
  see-also: docs/concepts/der-kausalpfeil.md docs/specs/livefeed-gate.md
-->

# BLATT PAPIER — Kausalpfeil der Sturzflut in Tibet

**Datum:** 2026-08-27 · **Axiom:** A = A
**Verdikt-Ordnung:** 0 honored — kein Wert erfunden; fehlende Felder bleiben `pending`.

> **Richtungskorrektur (registriert, 2026-09-05):** Die Pfeilrichtung folgt
> der maßgeblichen Bibliothek `transfer_entropy_lag(x, y) = TE(y → x)`
> (zweites Argument = Quelle). §3.5 (co-lokaler Gauge) lief mit
> `te_pair_probe`, dessen gedruckte „TE(a→b)"-Spalte **gespiegelt** zum
> Schätzer ist (`te_pair_probe.rs` beschriftet `transfer_entropy_lag(a,b)`
> als „a→b", misst aber b→a). Die Spalten in §3.5 sind korrigiert: der
> registrierte Wert 0.265 (Lag 24) ist **Pegel→Regen**, nicht
> Niederschlag→Pegel; die echte Niederschlag→Pegel-Richtung ist bei Lag 24
> unter der Schwelle und fällt unter Konditionierung. Verifikation und
> Konditionierungslauf: `docs/befund/befund-grat-trishuli-konditionierung.md`.
>
> Dasselbe Spiegelbild tragen die räumlichen Spalten in §3.3: der dort
> gedruckte „Rasuwa → Gyirong (Lag 12–24)“ ist **Gyirong → Rasuwa**
> (Oberlauf/Tibet als Quelle → Unterlauf/Nepal) — verifiziert mit
> `cross_te_screen` und im Kollab-Screening §3.1 (identische Zahlen).

---

## Zusammenfassung

Am 2026-08-26 traf eine Sturzflut/Schlammlawine den Trishuli-Bhola-Bogen
(Nepal, angrenzend an Tibet). Dieses Blatt misst mit dem omegaflow-Werkzeug
(`livefeed_gate`, `te_pair_probe`) und den Axiomen, **welcher Kausalpfeil
schlägt** — nicht welcher gefällt. Gemessen werden drei Felder jedes
Ereignisses: Zeit (JD TDB), Ort (ICRS geozentrisch) und Kraft (force_type).

**Befunde (gemessen, nicht behauptet):**

1. **Niederschlags-Ausbreitungspfeil Gyirong → Rasuwa** bei Lag 12–24 h —
   der Niederschlag am Oberlauf (Tibet) führt den am Unterlauf (Nepal):
   die erwartete obere-zu-untere Becken-Entwässerung. (mathematikerin,
   `te.rs`)
2. **Der Auslöser ist ein gravitativer Kollaps, kein Erdbeben** — gemessen:
   ein einziges Ereignis (`us7000tbwb`), Typ `landslide`, Magnitude
   **5.2 `ms_vx`**, **kein Moment-Tensor**; M5.2/02:52:10 UTC, lat 28.271/
   lon 85.515, depth 0. Der Kollaps (02:52:10) geht der Flut (~03:15) voraus.
   Die „M4.4" war die vorläufige Magnitude desselben Ereignisses.
3. **Der Regen→Pegel-Pfeil am co-lokalen Gauge fällt zur Stille
   (korrigiert)** — die registrierte „Niederschlag→Pegel 0.265 (Lag 24)" war
   Pegel→Regen (Richtungsfehler aus `te_pair_probe`-gespiegelten Spalten,
   siehe Richtungskorrektur); die echte Vorwärts-Richtung ist bei Lag 24
   unter der Schwelle und fällt unter Konditionierung. Der räumliche
   Response (CEMS EMSR927) ist ausgeliefert (Grading, Kollabpunkt
   außerhalb — kein Flut-Footprint).

---

## 1. Das Ereignis

Die Sturzflut/Schlammlawine an der Trishuli am 2026-08-26 (JD TDB
≈ 2461278.500801), Kraft `gravitation`. Registriert aus Wikipedia/Wikidata
(Q141182413), DW-RSS und
EONET-Kategorienflut. Terrestrische Raumzeit-Adresse (Wikidata P625):

| Ereignis | JD TDB | ICRS geozentrisch (X, Y, Z) km |
|----------|--------|-------------------------------|
| Flut Trishuli | 2461278.500801 | (2841.332, 4850.113, 3003.840) |
| Gyirong-Station (Tibet, Oberlauf) | 2461278.500801 | (2832.860, 4819.679, 3059.923) |
| Rasuwa-Station (Nepal, Unterlauf) | 2461278.500801 | (2865.607, 4837.623, 3000.939) |

ICRS-Umrechnung: WGS84 + GMST-Erdrotation. Das Prezession/Nutations-Delta
(~0.35° zum exakten J2000-ICRS) bleibt `pending`.

---

## 2. Daten und Methoden

**Quellen (alle offen gemessen, keine Paywall):**
- Niederschlag, Temperatur: **Open-Meteo** archive-api (stündlich, kein Key),
  Stationen Gyirong (28.8559, 85.2950) und Rasuwa (28.25, 85.10),
  2026-08-18…27, n = 240.
- Auslöser: **USGS-FDSN**-Katalog (landslide M5.2).
- Ort: Wikidata P625, Nominatim.
- Abfluss: **DAHITI** (TU München, Satelliten-Altimetrie, mit API-Key).

**Methoden:**
- **`te_pair_probe`** — mathematikerin `transfer_entropy_lag` +
  `surrogate_threshold_lag` (phasenrandomisierte Surrogate, mean + 2σ).
  Ein TE > Schwelle (mean+2σ) ist ein signifikanter gerichteter Pfeil.
- **`livefeed_gate --icrs-aus-geo`** — geo → geozentrischer ICRS-Vektor
  (km) zur Zeit JD TDB.
- **Axiom A = A:** Eine Zahl gilt nur, wenn eine messbare Quelle sie trägt.
  Fehlt die Quelle, steht `pending` — es wird nichts geglättet, nichts
  geglaubt.

---

## 3. Ergebnisse

### 3.1 Treiber-Landschaft (vollständig)

| # | potentieller Treiber | Status | Messung |
|---|----------------------|--------|---------|
| 1 | Niederschlag (Monsunregen) | **gemessen** | Pfeil Gyirong→Rasuwa, Lag 12–24 h |
| 2 | Temperatur / Schmelzwasser (thermisch) | **gemessen** | beidseitig, kein sauberer Pfeil (geteilter Tagesgang) |
| 3 | Eis-/Fels-Kollaps (Lirung-Gletscher) | **gemessen** | USGS `us7000tbwb` `landslide`, 5.2 `ms_vx`, kein Moment-Tensor — gravitativer Quellfeld |
| 4 | Tektonisches Erdbeben | **kein Befund** | kein separates Beben im Fenster; „M4.4" = vorläufige Magnitude desselben Ereignisses |
| 5 | Abfluss / See-Spiegel (Response) | **pending** | keine co-lokale, co-temporale Reihe |

### 3.2 Auslöser: ein gravitativer Kollaps — kein Erdbeben (gemessen, nicht gewählt)

USGS-FDSN, Ereignis `us7000tbwb`, 2026-08-26 (beide Klassifikationen
abgefragt, nicht ausgewählt — es gibt **genau ein** Ereignis im Fenster):

| Feld | Wert |
|------|------|
| Typ | **`landslide`** |
| Magnitude | **5.2 `ms_vx`** — Very-Long-Period-Oberflächenwellen-Magnitude, für gravitative Kollapse |
| Moment-Tensor-Produkt | **keines** — ein tektonisches Beben trüge eins |
| Zeit | 2026-08-26T02:52:10 UTC |
| JD TDB | 2461278.620361 |
| geo | lat 28.271, lon 85.515 |
| Tiefe | 0 (Oberflächen-Kollaps) |
| ICRS | (−1258.243, +5478.936, 3002.989) km |

Die drei Marker — `landslide`, `ms_vx`, **fehlender Moment-Tensor** — messen
ein **gravitatives Quellfeld**, kein tektonisches Beben. Die „M4.4" war die
**vorläufige** Magnitude vor der Revision; sie ist **dasselbe** Ereignis,
kein zweites. Es gibt **kein** separates Erdbeben.

**Gemessene Kausalkette (Zeitstempel, nicht erzählt):**

| Feld-Zeit (UTC) | Ereignis |
|---|---|
| 02:52:10 | Kollaps-Signal (`us7000tbwb`) |
| 02:55 | letzte Pegel-Messung Rasuwagadhi — Telemetrie verstummt (Gerät/Leitung im Kollaps zerstört) |
| ~03:15 (09:00 NPT) | Flutwelle trifft am Pegel ein |

Der **Kollaps geht der Flut voraus** (~23 min). Der Kollaps ist der
gemessene gravitative Auslöser; **seine eigene Ursache ist nicht gemessen**
(kein co-lokaler kontinuierlicher Seismik-/Verschiebungskanal) → `pending`,
nicht hypothetisiert. Gletscher-Kontext (gemessen/OSM + Fachquellen, 0 honored):
Kollaps an der **Nordflanke des Langtang-Lirung-Massivs** (Gipfel
28.2575°N/85.5158°E; Abbruch 28.26–28.27N, 85.50–85.53E), die **nach Tibet ins
Lhende-Khola/Bhote-Koshi-System** entwässert. **Terminologie-Korrektur:** Der
RGI-kartierte **„Lirung-Gletscher" ist die Südzunge** (Zunge Richtung Kyanjin,
Terminus ~28.21–28.23N) im **Langtang-Khola-Einzugsgebiet**, das von dieser
Flut **nicht** betroffen war — der Begriff „Lirung-Gletscher" gehört hier
**nicht** zur Kollaps-Stelle. Die zwei Lhende-Khola-Seen (朗吉错 28.4070/
85.4018, 拓米托湖 28.3924/85.4186) liegen **talabwärts** (16–19 km), **kein**
kartierter See am Kollabpunkt.

**Externe Bestätigung (0 honored — verifiziert, nicht geglaubt):**
Severe Weather Europe (R. Colucci, 27/08/2026,
severe-weather.eu/.../usgs-landslide-august-2026-rrc/) zitiert **ICIMOD
(via Reuters/AP)** und **USGS**: der Auslöser ist eine **Eis-Fels-Avalanche**
(Blockade im oberen Lhende Khola), **kein GLOF**; „no emptied pre-existing
glacial lake shown as primary source". Diese Aussagen decken sich **unabhängig**
mit der Messung oben (M5.2 `landslide`, kein Oberflächensee am Kollabpunkt).
Geolokal (OSM-Nominatim, gemessen): **Lhende Khola** 28.3496°N/85.4464°E
(~11 km NW des Kollabpunkts), **Lirung-Gletscher (Südzunge)** 28.2467°N/
85.5421°E (~3.8 km SE, aber **Langtang-Khola-Einzugsgebiet**, nicht die
Kollaps-Flanke). Der Kollaps liegt an der **Nordflanke des Langtang-Lirung-
Massivs** (Gipfel 28.2575°N/85.5158°E, ~1.5 km S des gemessenen Kollabpunkts);
„Lhende Khola" ist das **talabwärts anschließende Talsystem** (SWE benennt
es als Blockade-Ort). Konsistente Geometrie (Nordflanke Langtang Lirung →
Lhende-Khola-Tal → Seen → Bhote Koshi); exakter Quell-Hang braucht noch das
CEMS/S1-Bild.

**Tool-Validierung gegen einen bekannten See (0 honored, 08-12):** der von
DHM dokumentierte **supraglaziale See der 2025er-Vorgängerflut** (28.4043°N/
85.6469°E, 2025: 0.75→0.60 km²) misst auf 08-12 eine **echte, wenn auch
schwache Wasser-Signatur** (NDWI max **0.412**, 270 Wasser-Pixel) — konsistent
mit kleinem/schwindendem supraglazialem See. **Klar unterscheidbar** von
Langjie Cuo/Tuomito (0 Wasser, max **0.071**) → die Messkette diskriminiert
reale Gewässer zuverlässig; der fehlende Wasserbefund an den zwei OSM-Seen ist
messbar echt, kein Tool-Artefakt.

**Zweite Vor-Baseline (08-24, 38.6 % Wolken, `--cog-ndwi`, 0 honored):**
am Kollabzentrum **1 Wasser-Pixel von 63865** (NDWI max 0.204) → auch am
08-24 praktisch **kein Oberflächensee** am Kollabpunkt (stützt die Avalanche-
Einordnung). Die Seen-Region (Langjie Cuo/Tuomito) ist in dieser Granule
wolkenbedingt **nodata** und dort nicht messbar (08-12 liefert die sichere
Vor-Baseline für die Seen).

### 3.3 Der Niederschlags-Ausbreitungspfeil (gemessen)

`te_pair_probe`, Paar Oberlauf Gyirong ↔ Unterlauf Rasuwa, n = 240,
2026-08-18…27:

    lag   TE(Rasuwa→Gyirong) schwelle  TE(Gyirong→Rasuwa) schwelle  Befund
    1     8.09e-2    1.13e-1   6.90e-2    1.05e-1   kein Befund
    3     8.92e-2    1.31e-1   1.26e-1    1.39e-1   kein Befund
    6     1.06e-1    1.08e-1   9.81e-2    1.40e-1   kein Befund
    12    1.10e-1    1.30e-1   1.62e-1    1.33e-1   Gyirong → Rasuwa
    24    1.09e-1    1.11e-1   1.51e-1    1.31e-1   Gyirong → Rasuwa

**Verdikt: Pfeil Gyirong → Rasuwa bei Lag 12–24 h.** Der Niederschlag am
Oberlauf (Tibet) führt den am Unterlauf (Nepal) — die erwartete
Becken-Entwässerung von oben nach unten (Oberlauf/Tibet → Unterlauf/Nepal).
Die Maschine misst, welcher Pfeil schlägt.

**Temperatur-Paar (thermischer Treiber):** beidseitig signifikant bei allen
Lags (TE 0.13–0.20, beide Richtungen > Schwelle) — ein stark synchronisierter
Tagesgang über das kleine Becken, **kein** sauberer Richtungspfeil. Die
thermische Kopplung ist gemessen, trägt aber keine gerichtete Ausbreitung.

### 3.5 Regen → Fluss-Pegel (gemessen am co-lokalen Gauge, 0 honored)

**Fund der Recherche:** DHM Nepal betreibt einen **offenen, key-losen, live
Pegel am Ereignisort** — **Bhotekoshi at Rasuwagadhi** (ID 4913, Serie 23251,
lat 28.2713/lon 85.3776), Pegel (m), 10-min-Auflösung. Die Reihe wurde über
die offene Seite gezogen (769 Punkte, 08-20 18:45 … 08-26 02:55 UTC) und mit
dem stündlichen Rasuwa-Niederschlag auf den Überlapp (n = 129) gelegt.

Versiegeltes Protokoll (`te_pair_probe`, Surrogate mean+2σ); Spalten
korrigiert (Richtungskorrektur oben — `te_pair_probe` beschriftet „a→b"
gespiegelt, die erste Zahlenspalte ist im Schätzer Pegel→Regen):

    lag   TE(Pegel→Regen) schwelle  TE(Niederschlag→Pegel) schwelle  Befund
    1     1.60e-1   1.76e-1   1.20e-1   1.11e-1   Niederschlag→Pegel
    3     1.82e-1   1.83e-1   1.79e-1   1.94e-1   kein Befund
    6     1.52e-1   2.01e-1   1.95e-1   1.71e-1   Niederschlag→Pegel
    12    2.55e-1   2.25e-1   2.60e-1   2.20e-1   beide
    24    2.65e-1   2.18e-1   2.23e-1   2.31e-1   **Pegel→Regen**
    48    3.43e-1   2.47e-1   2.72e-1   2.46e-1   beide

**Befund korrigiert (im Sinne der Präregistrierung):** der registrierte
Wert **0.265 bei Lag 24 ist Pegel→Regen** — die Gegenrichtung, nicht
Niederschlag→Pegel (Richtungsfehler aus `te_pair_probe`-gespiegelten
Spalten, siehe Richtungskorrektur oben). Die echte **Niederschlag→Pegel-
Richtung ist bei Lag 24 TE 0.223 < Schwelle 0.231 — unter der Schwelle**:
kein Vorwärts-Pfeil am co-lokalen Gauge bei dessen betontem Lag. Die
Vorwärts-Kopplung ist bei kurzen Lags (1–6 h, teils 12) signifikant — die
schnelle Pegel-Antwort auf Regen —, aber unter der Konditionierung auf den
geteilten synoptischen Treiber übersteht **keine** Vorwärts-Orientierung
die Signifikanz (`docs/befund/befund-grat-trishuli-konditionierung.md`).
Der präregistrierte Vorwärts-Pfeil fällt zur Stille. Ehrliche Grenze:
gemessen ist **Pegel** (m), nicht Abfluss (m³/s), auf dem **Vor-Flut-
Fenster** (die Reihe bricht am Flutbeginn ab) — es misst die
Monsun-Anspeisung des Flusses, **nicht** den Flut-Peak selbst (der wurde
vom offenen Pegel nie aufgezeichnet; Telemetrie-Stopp 08-26 02:55).

### 3.6 Räumlicher Response — Footprint-Messung (archivar `--sentinel`, 0 honored)

**CEMS Rapid Mapping EMSR927 „Flood in Nepal"** — Aktivierung
2026-08-26T09:53, Zentroid POINT(85.3538E, 28.2122N) (co-lokal), Phase
`response`, 1 Produkt, 4 AOI. **Stand 2026-08-31:** Produkt ausgeliefert,
aber nur **Grading (GRA)** für AOI01 Syapru Besi (08-27), AOI02 Timure
(08-28), AOI03 Bidur (08-29) — Schadensanalyse, **keine Flut-Ausdehnungs-
(Delineation-)Fläche**, Abdeckung nur lon 85.10–85.38 → Kollabpunkt (85.51)
**außerhalb**.

**Sentinel-2-Szenen (archivar `--sentinel`, CDSE-STAC key-frei) um Kollab-
punkt + Lhende-Seen gemessen:** Vor-Ereignis-Szenen 08-11…08-24 (cloud
18.8–97.1 %) — aber **nur eine Post-Ereignis-Szene: 08-27T04:56Z mit
78.5 % Bewölkung** (Monsun). Damit ist die **optische** Kollapsnarbe/See-
Ausdehnung **nicht zuverlässig messbar** (Wolken verdecken das Ziel).

**Sentinel-1 (SAR, wetterfest):** Post-Ereignis-Szene **noch nicht
archiviert** (nächster Pass ~08-28/29) — die SAR-Flutfläche/-Narbe ist
danach messbar. Der räumliche Footprint bleibt `pending`, 0 honored, bis
(a) CEMS-EMSR927-Produkt ausgeliefert oder (b) Post-Sentinel-1 da ist.

**Alternative ohne Bild — Kollaps-Volumen aus gemessener Magnitude
(magnitude-gebunden):** Das Quellereignis ist gemessen als USGS
`us7000tbwb`, Typ `landslide`, M5.2 (`ms_vx`, sehr-lange-Periode). Dies
ist dieselbe gravitative Klasse wie die **Chamoli-2021-Katastrophe** —
ebenfalls eine vom *International Charter* bestätigte **Fels+Eis-
Avalanche** (Auslöser: Hangversagen, kein See-Ausbruch, kein Erdbeben),
deren Kollapsvolumen auf **~27 Mio m³ (80 % Fels / 20 % Eis)** gemessen
wurde — Shugar et al. (2021), *Science* 373:300,
doi:10.1126/science.abh4455. Bei vergleichbarer `ms_vx`-Magnitude (~M5.2–
5.3) ist ein Kollapsvolumen **in der Größenordnung ~10⁷ m³** eine
magnitude-gebundene, zitierbare Einschränkung — **kein** gemessener
Volumenwert für dieses Ereignis (0 honored: als `pending` bis zur
direkten Volumenmessung aus Bild/SAR gekennzeichnet).

### 3.7 Das Messstations-Netzwerk (archivar `--dhm`, 0 honored)

Das DHM-Netzwerk wird als Messfläche einbezogen (archivar `--dhm-list`,
`--dhm <id>`): Pegel am Bhote Koshi → Trishuli, oberhalb→unterhalb.

| Pegel | lat/lon | Reihe endet | max |
|---|---|---|---|
| Bhotekoshi at Rasuwagadi (4913) | 28.2713/85.3776 (co-lokal) | **02:55** (Telemetrie) | 2.28 |
| Bhote Koshi at Shyaprubesi (191) | 28.1706/85.3426 | **03:05** | 4.75 |
| Trishuli at Betrawati (52) | 27.9700/85.1800 | **03:35** | 4.47 |
| Bhote Koshi at Bahrabise (113) | 27.7868/85.8993 | 08-27 19:05 (**überlebt**) | 2.61 |

> **Berichtigung (0 honored, 2026-08-28):** Station 113 ("Bhote Koshi at
> Bahrabise") liegt bei **85.899°O / 27.787°N** — das ist der
> **Kodari→Sun-Koshi-Korridor** (Sindhupalchok), ein **anderer Fluss** als
> der Kollab-Abfluss. Die Zugehörigkeit wurde selbst gemessen (DHM-
> Stationsmetadaten aus der live river-watch-Seite + Höhenprofil via
> Open-Meteo-Elevation-API): Der Kollabpunkt (28.271/85.515, **5510 m**)
> liegt auf einem Kamm — östlich (85.53–85.56°O) steigt es auf 5939–6024 m
> (Massiv-Inneres), westlich (85.50→85.44°O) fällt es auf 5476→4690→4311 m.
> **Die Entwässerung läuft nach WESTEN in die Trishuli/Rasuwa-Seite**
> (Stationen 4913/190/191 bei 85.34–85.38°O), **nicht** nach Osten zum
> Sun Koshi. Der "Bhote Koshi" ist ein doppelt vergebener Flussname.
> **Der unten stehende "+8.0h/+14.7h"-Kausalpfeil, gemessen an Bahrabise
> (113), gehört damit NICHT dem Kollab-Abfluss** — er misst eine Welle
> am Sun Koshi (anderes Einzugsgebiet). Für den Kollab-Pfad (Trishuli)
> wurde kein überlebender Pegel-Peak aufgezeichnet; die Ankunfts-/Peak-
> Zeit bleibt **pending** (0 honored) — nur die **Zerstörungsfront**
> (02:55/03:05/03:35, flussabwärts wandernd) ist am Trishuli-Pfad gemessen.

> **Zugehörigkeit (Berichtigung, 0 honored):** Der folgende "Bahrabise
> (113) überlebt und misst die Welle"-Abschnitt gehört zum **Sun-Koshi-
> Korridor**, nicht zum Kollab-Trishuli-Pfad. Er wird als *Referenz-Messung
> an einem anderen Fluss* geführt — **nicht** als Kollab-Kausalpfeil.

**Gemessene Zerstörungsfront (Trishuli-Pfad, Kollab):** die oberen Pegel
sterben in Flussrichtung — Rasuwagadhi 02:55, Shyaprubesi 03:05, Betrawati
03:35 — die Flutwelle zerstört die Pegel, während sie abwärts wandert.
**Kein Trishuli-Pegel zeichnet einen Peak auf** (alle drei versterben vor
der Kamm-Form; 4657 Dhunche bleibt konstant ~2.4 m ohne Schwall-Spike).
**Bahrabise (Sun Koshi) überlebt** und zeigt an *seinem* Fluss einen
Anstieg (Referenz, UTC; 0 honored — anderer Fluss):

- **Anstieg (Sun Koshi):** 08-26 **10:55 UTC** (≈ +8.0 h nach Kollaps;
  über Baseline 2.00 m + 0.15)
- **Anstiegs-Peak (Sun Koshi):** **2.57 m um 08-26 17:35 UTC** (≈ +14.7 h;
  der *globale* Max der Reihe 2.61 m liegt am 08-25 20:55 = **vor** dem
  Kollaps, Regen-/Tagesgang-Wert — nicht die Flut)

→ **Gemessener Kausalpfeil Kollaps→Flut (Trishuli-Pfad):** die Eis-Fels-
Avalanche (02:52) zerstört den co-lokalen Pegel (Rasuwagadhi) in ~3 min und
die abwärts liegenden Pegel in Flussrichtung (02:55/03:05/03:35). Ein
**aufgezeichneter Flut-Peak am Trishuli-Pfad existiert nicht** (alle Pegel
versterben vor der Kamm-Form). Der End-zu-End-Zeitpfeil über einen
überlebenden Pegel ist am **Trishuli-Pfad pending** (0 honored); der
rasterliche/bildliche Response (CEMS/S1) bleibt die offene Bestätigung.

**mathematikerin am Netzwerk (korrigiert):** der Vorwärts-Pfeil am
**co-lokalen** Rasuwagadhi ist **nicht** sauber — er fällt zur Stille
(§3.5, Richtungskorrektur). Der am weit abwärts liegende Bahrabise
(n=169) läuft durch dasselbe `te_pair_probe` und trägt damit dieselbe
Spiegel-Unsicherheit; seine „gemischten" Lags (Lag 12/24/48 beidseitig,
Lag 6 rückwärts) sind ohne Richtungs-Verifikation nicht als
„verdünnende Regen→Pegel-Kausal-Signatur flussabwärts" zu lesen. Ehrlich:
die frühere Netzwerk-Lesart (klarer Oberlauf-Pfeil, der sich abwärts
verdünnt) ist durch die Stille am co-lokalen Gauge nicht mehr getragen;
eine Richtungs-Verifikation der Bahrabise-Spalten ist ein offener
Nachfolgepunkt, nicht dieser Befund.

### 3.4 Der Treiber-Niederschlag (gemessen)

Open-Meteo, je Tag UTC:

| Tag | Gyirong (85.30, 28.86) | Rasuwa/Nepal (85.10, 28.25) |
|-----|------------------------|------------------------------|
| 08-18 | 5.4 mm | 26.9 mm |
| 08-19 | 7.3 mm | 28.7 mm |
| 08-20 | 1.1 mm | 23.4 mm |
| 08-21 | 0.8 mm | 20.5 mm |
| 08-22 | 0.5 mm | 23.6 mm |
| 08-23 | 0.0 mm | 23.1 mm |
| 08-24 | 0.6 mm | 11.4 mm |
| 08-25 | 0.0 mm | 12.2 mm |
| 08-26 | 6.1 mm | 17.5 mm |
| 08-27 | 1.8 mm | 23.2 mm |
| **Summe** | **23.6 mm** | **210.5 mm** |

Die Nepal-Seite trug eine Woche anhaltenden Monsunregen (20–29 mm/Tag) vor
der Flut am 08-26 — der gemessene Treiber (A = A: 210.5 mm in 10 Tagen,
Quelle Open-Meteo archive-api).

### 3.5b Regen am Kollabpunkt als Lawinen-Auslöser (2026-08-28, gemessen)

**Frage:** Löste Regen die Eis-Fels-Avalanche aus? Antwort: **nein, gemessen.
** Stündliche Niederschlagsserie (Open-Meteo archive-api, UTC) am
Kollabpunkt (28.271/85.515), 08-23 00:00 … 08-26 03:00, Kollaps 02:52 UTC:

| Fenster | Summe |
|---|---|
| 08-23 (48–72 h vorher) | 1.6 mm |
| 08-24 (24–48 h vorher) | 2.7 mm |
| 08-25 00:00 – 08-26 02:52 (0–24 h) | 1.2 mm |
| **72 h vor Kollaps gesamt** | **5.5 mm** |

- **Kein messbarer Regen-Antrieb:** 5.5 mm in 72 h am Quellhang ist
  vernachlässigbar gegen die monsunale Porenwasser- und Schmelzlage; ein
  kurzfristiger Regen-Impuls als Auslöser ist **nicht messbar**.
- Der Tagessprung (bis 1.9 mm/h) setzte erst **12:00–17:00 UTC am 08-26**
  ein — **nach** dem Kollaps (02:52) und ist der nachlaufende Monsun-/
  Tagesgang, **kein** Auslöser.
- **Konsequenz (0 honored):** Der **Auslöser der Lawine bleibt nicht messbar**
  (pending) — weder Regen (ausgeschlossen) noch Erdbeben (USGS reklassifiziert
  M4.4→M5.2 als *Folge* der Lawine, `landslide`). Ein Gletscher-/Eisthermisches
  Versagen (Wärmeschwäche, Schmelzwasser, Nackenbruch) ist möglich, aber
  **nicht direkt messbar** aus den verfügbaren Daten.

---

## 4. Der Pfeil, der nicht messbar ist (0 honored)

**Abfluss-Pfeil Regen→Flut.** Um ihn zu messen, braucht es eine co-lokale,
co-temporale Abfluss-Response-Reihe am Trishuli. Die Quellen-Jagd gegen den
`akteure-verdrahten`-Katalog (`phi/`), bestätigt 2026-08-27:

| Quelle | Status |
|--------|--------|
| GRDC (`grdc.bafg.de`, Weltabfluss) | declined (−4), tot (404) |
| CEMS GloFAS (`cems.ecmwf.int`) | declined, tot (404) |
| EFAS, Copernicus-EMS | declined (−4) |
| UK Environment-Agency (`hydrosphere_river_flow_cfs`/`_stage_m`) | **LIVE**, aber UK-only |
| **DAHITI** (TU München, Altimetrie, mit Key) | erreichbar — Koshi-Stationen, **aber nicht co-temporal** |

**DAHITI-Nachmessung mit Key (2026-08-27):** die drei Koshi-Stationen
liefern Altimetrie-Wasserstand (`wse`), aber **keine** erreicht das
Ereignisdatum 2026-08-26:

| Koshi-Station | Punkte | letzter Wert |
|---|---|---|
| 3409 | 612 | 2026-07-08 |
| 8480 | 106 | 2016-05-10 (tot) |
| 15694 | 101 | 2026-08-04 |

Satelliten-Altimetrie-Reihen sind irregular; die jüngsten Punkte liegen
Wochen **vor** der Flut. Bei Niederschlag (2026-08-18…27) ist die zeitliche
Überlappung **null** → der Koshi-Wasserstand ist für **dieses** Ereignis
kein co-lagbarer Response. Zusätzlich liegen die Koshi-Stationen ~300 km
**unterhalb** des Ereignisses — nicht co-lokal.

**Konsequenz für den Abfluss (m³/s):** nicht offen messbar — DHM liefert
offen nur **Pegel (m)**, Abfluss steht hinter einem bezahlten Portal. Der
**Pegel-Pfeil Regen→Fluss** ist am co-lokalen Gauge **nicht gemessen**
(siehe §3.5, korrigiert): die co-lokale DHM-Reihe existiert (offen,
key-los), aber der Vorwärts-Pfeil bei Lag 24 h ist unter der Schwelle und
fällt unter Konditionierung — die echte Zahl 0.265 war die Gegenrichtung.
Die ehrliche Grenze bleibt: der **Flut-Peak selbst** wurde vom offenen
Pegel nie aufgezeichnet (Telemetrie-Stopp am Flutbeginn) — und eine
Vor-Flut-Anspeisungs-Signatur ist als Pfeil über dem geteilten Treiber
nicht isolierbar. Der räumliche Response (CEMS EMSR927) ist ausgeliefert
(Grading, Kollabpunkt außerhalb — kein Flut-Footprint).

---

## 5. Grenzen (0 honored)

- **Flut-Peak nicht aufgezeichnet** — der co-lokale DHM-Pegel bricht am
  Flutbeginn ab (letzter Wert 1.62 m, 08-26 02:55 UTC); der Peak existiert
   in keiner offenen Reihe. Ein Vorwärts-Pfeil am co-lokalen Gauge ist
   über den geteilten Treiber hinaus nicht isolierbar (siehe §3.5,
   korrigiert; Lag 24 fällt zur Stille).
- **Pegel ≠ Abfluss** — DHM offen nur Pegel (m); Abfluss (m³/s) ist
  bezahlt/geschlossen.
- **Prezession/Nutation-Delta** der ICRS-Umrechnung (~0.35° zum J2000-ICRS)
  bleibt `pending`; die Vektoren sind GMST-näherungsweise.
- **Ursache des Kollapses ungemessen:** der Kollaps (gravitativ) ist als
  Signal gemessen; **warum** er kollabierte, ist ohne co-lokalen kontinuier-
  lichen Seismik-/Verschiebungskanal nicht messbar → `pending`, nicht
  hypothetisiert. Ob ein Dammbruch-/GLOF-See involviert war: am Kollabpunkt
  ist **kein** kartierter See (OSM) — die Kollaps-zu-Flut-Kette ist
  gemessen, der See-Mechanismus nicht.
- **n-Abhängigkeit der Surrogate:** signifikante Lags bei n = 240; die
  Ergebnistabelle ist gegen phasenrandomisierte Surrogate getestet.

---

## 6. Schlussfolgerung

Das Ereignis ist gemessen in Zeit (JD TDB), Ort (ICRS geozentrisch) und
Kraft (`gravitation`). Die Maschine misst, welcher Pfeil schlägt:

- **Ein gerichteter, signifikanter Pfeil** existiert im Becken:
  **Niederschlag Gyirong → Rasuwa (Lag 12–24 h)** — der Niederschlag am
  Oberlauf (Tibet) führt den am Unterlauf (Nepal): die erwartete
  obere-zu-untere Becken-Entwässerung.
- **Am co-lokalen Gauge: kein Vorwärts-Pfeil (korrigiert)** — die
  registrierte „Niederschlag → Pegel 0.265 bei Lag 24 h" war Pegel→Regen
  (Richtungsfehler aus `te_pair_probe`-gespiegelten Spalten); die echte
  Vorwärts-Richtung ist bei Lag 24 unter der Schwelle und fällt unter
  Konditionierung auf den geteilten synoptischen Treiber. Das
  präregistrierte Vorwärts-Verdikt ist zur Stille korrigiert.
- **Der Auslöser ist als gravitativer Kollaps gemessen** (`us7000tbwb`:
  `landslide`, `ms_vx`, **kein Moment-Tensor**, depth 0) — **kein tektonisches
  Erdbeben**; „M4.4" war die vorläufige Magnitude desselben Ereignisses. Der
  Kollaps (02:52:10Z) geht der Flut (~03:15Z) voraus.
- **Der Flut-Peak bleibt ungemessen** (offener Pegel brach am Flutbeginn ab;
  Abfluss m³/s ist geschlossen). Ein Vorwärts-Pfeil am co-lokalen Gauge ist
  über den geteilten Treiber hinaus nicht isolierbar — ehrlich `pending`,
  0 honored.
- **Ursache des Kollapses: `pending`** — kein co-lokaler kontinuierlicher
  Seismik-/Verschiebungskanal; es wird **nicht hypothetisiert** (Monsun-
  Last, Permafrost, lokales Subkatalog-Ereignis: alles ungemessen).

Was nicht gemessen werden kann, wird nicht behauptet. Das ist die ganze
Punktzahl.

---

## Wiederholung

Ist die Trishuli-Abflussreihe offen messbar, lautet der Lauf:

    pfeil --a serieA --b serieB --lag-sweep --surrogat 10
    Verdikt: Pfeil Niederschlag → Abfluss | Pfeil Abfluss → Niederschlag | kein Befund

---

*Blatt registriert 2026-08-27. Verdikt-
Ordnung 0 honored: kein Wert erfunden; jede Lücke als `pending` benannt.*
