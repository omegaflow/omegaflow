# TODO

AGENTS.md is the primary constraint matrix. Git is the history.
Kanonisch: Diese Datei ist das vollständige Register der offenen Arbeit. Erledigtes
wird entfernt (Git trägt es). Kein Eintrag meldet Erledigtes als offen, kein offener
Punkt fehlt. Widerspricht ein Dokument dieser Datei, gilt diese Datei — solche
Drift-Stellen sind unter „Doku-Drift" registriert.

## Stand — 2026-08-14 (LSK/PCK-Hochzeit, Binary v2, Protokoll v6, reine Per-Pixel-Membran, K01 geschlossen)

Zeit aus naif0012.tls (LSK-Reader, keine TT_MINUS_UTC-Konstante). PCK-Reader pck.rs
(gm_de440, pck00010, geophysical.ker → GM/J2/J4/Radii/POLE). stype-1 v2: gcount=12,
96 B festes Stride; non-v2 → None (Körper absent). WS-Protokoll v6: 19-B-Header
(0xCF 0x86 0x06 + response_epoch f64 + id u32 + count u32), Record 168 B / 21 f64
(+ pole, j2, j4, r_eq); JS field 12 + meta 12 floats, WGSL field[j*3]/props[j*3].
body_channels sendet ausschließlich {body}.mass (GM, Kernel 0, gravity) und
{body}.radius. fs evaluiert osc_field für jeden Pixel (kontinuierliche Mathematik,
keine Stützstellen, keine Interpolation); Nebra-Rampe t2 = clamp((log2(Ω_total)+14)/22, 0, 1)
(4 mix()-Segmente Blau→Cyan→Orange→Weiß) IST die Tonemap; Ω_total = Σ|omegas[k]|.
Exposure-Kette getilgt (keine lvls, keine e/E). SSAA als dichtere Messung: q/Q in
Φ-Schritten (1.00–8.00, Start 1.00×), Canvas-Backing skaliert, CSS nativ.
28 lokale v2-Binaries unter /tmp/omegaflow_eph_*.bin (Meter, verifiziert).
K01 geschlossen: kernel_flatten.yml (Rust-Flattener-CI) ersetzt
generate-ephemerides.yml; ephemeris_compiler mit --index (voller rekursiver
HTTPS-Crawl ssd.jpl.nasa.gov/ftp + naif.jpl.nasa.gov/pub → phi/sources_index.φ),
--summarize (docs/reference/KERNEL_INDEX.md), --fetch-from (Index-getriebene
Kernel-Auswahl), --ci-mode (gh release upload ssd.jpl.nasa.gov, --clobber);
dynamische Target-Enumeration aus DAF-Segmenten statt all_target_ids;
NAIF-ID↔Name-Tabelle docs/reference/naif_body_ids.tsv; gm_Horizons.pck als
GM-Quelle; Horizons-Compiler trägt --ci-mode. Kleinkörper-Flatten-Pass ist am
K03-Zweig registriert (DASTCOM+Kepler). K02 geschlossen: src/bpc.rs
(Binary-PCK-Leser, DAF Typ 2) + stype-4-Nutationssektion (additiv) in
Compiler/Runtime; Moon-PA-Merge parkt an K05. K03-Compiler-Einheit
geschlossen: src/kepler.rs + src/bin/dastcom_compiler.rs (Dev-Beweis Ceres
0,001″ gegen Horizons). 32 neue φ-Ephemeris-Blöcke für die Flattener-Monde
(absent bis der Flattener-CI-Lauf die CDN-Assets trägt — 0 honored).
0 Warnings, 0 Errors; Tests: 5 lib + 20 bin.

---

### Gravity-Komplettierung — Kernel-Flattener (K02–K06)

Entscheidungsgrundlage: Der volle Kernel-Bestand (Multi-GB) wird ausschließlich von
der Rust-CI geflattet — lokal kommt nur das per-Body-Binary vom CDN. Keine lokalen
Python-Umwege. Quellen-Inventar: phi/sources_index.φ (Crawl, kernel_flatten.yml)
+ docs/reference/KERNEL_INDEX.md (Lesefassung, 9-Kanal-Matrix). K01 (Flattener-CI,
--index-Crawler, --fetch-from, --ci-mode, NAIF-ID-Tabelle, dynamische
Target-Enumeration) ist geschlossen; Git trägt es.

**K02** Binary-PCK-Reader (Erde/Mond-Präzision)
```
Modul src/bpc.rs auf daf.rs: DAF-Segmente Typ 2 (Chebyshev-Orientierung RA/Dec/W;
Typ 3/20 werden gemeldet und übersprungen — 0 honored). Precedence-Regel:
Binary-PCK > Text-PCK (pck.req). Der Compiler sampelt das volle Orientierungsmodell
und schreibt die stype-4-Nutationssektion (additiv: Chebyshev-Fit von voll − linear,
RA/DEC/PM); die Runtime addiert sie in orientation_angles_at (K02 geschlossen —
Leser, Kanal, Protokoll-Doku, CI-Anschluss). Befund 2026-08-14: Text-PCKs der Monde
tragen echte NUT_PREC-Reihen (Io −0,083°, Iapetus −0,451°, Uranus-Monde …) — sie
fließen über die stype-4-Röhre. Binary-PCKs tragen NUR Orientierung (pck.req,
Typen 2/3/20) — die CON-J2/J4-Behauptung war falsch; Mond-Harmonische liegen nicht
in den Flattener-Wurzeln (bleiben j2/j4 = 0 honored, Kanal-Forschung: GRAIL-Modelle).
```
Verifikations-Befund moon_pa_de440_200625.bpc (Session 2026-08-14): Der
DAF-Leser ist korrekt (Trailer, Record-Adressen, Fenster 1550–2650, Grad-9-Fits
verifiziert) — aber die gespeicherten Winkel folgen nicht der IAU-Pol-Konvention:
bei J2000 liefert die Datei Pol ≈ (RA −0,05°, DEC +0,43°) und W-Drift 0,23°/Tag
statt 13,176°/Tag. Die SPICE-Kette geht über MOON_PA_DE440 (31008) + TKFRAME_31009
(67,85"/78,69"/0,28", AXES 3-2-1, ARCSECONDS, moon_de440_250416.tf). Der
Binary-PCK-Merge für den Mond parkt deshalb an K05 (FK-Kette) — der Leser steht.

**K05** FK-Frames (erweitert um den K02-Befund)
```
moon_de440_250416.tf / moon_080317.tf / earth_assoc_itrf93.tf als
Frame-Assoziationsquelle → hartcodierte Parent-/Name-Tabellen ersetzen
(Bias-Befund unten). Trägt außerdem den Moon-PA-Merge: MOON_PA_DE440 (31008,
PCK-Klasse 2, src/bpc.rs steht) + TKFRAME_31009-Festrotation = ME-Frame;
erst dann trägt der Flattener die Binary-PCK-Präzision für den Mond.
```

**K03** Kleinkörper-Katalog
```
GESCHLOSSEN (2026-08-15): Compiler-Einheit (src/kepler.rs Kepler-Löser +
elements_to_icrs_state, src/dastcom.rs Record-Format 92-B-Stride + Hill-Radius,
src/bin/dastcom_compiler.rs, Dev-Beweis Ceres 1,3 km = 0,001″ gegen Horizons)
UND Runtime-Konsum: φ-Format catalog_dastcom (at sun, ttl 86400), TTL-Fetch
nach /tmp/omegaflow_catalog_dastcom_asteroids.bin, AsteroidHash (Enclosure-
Lemma, Zellen auf Epoch-Positionen, vmax/amax datenabgeleitet ×Φ), Query
query_asteroid_hash (Pre-Filter mit per-Record-Reach, Kepler-Evaluation zum
Query-Zeitpunkt, Exact-Filter auf Hill+pad), Emission Massen- (GM m³/s²,
kernel 0, force 1, τ ∞) + Radius-Kanal (m), Extent = Hill-Radius
a·(GM/3GM_sun)^(1/3). **GM-Gate:** nur Körper mit gemessenem GM manifestieren
(17 Records — alle anderen sind absent, 0 honored; das Gate löst zugleich die
Mengenfrage: 17 statt 1,56 Mio. Kandidaten pro Query). WS-Beweis: Ceres-
Oszillator bei Fenster-Mitte auf der Epoch-Position, Distanz 1,3 km.
Befund: Kepler-Positionen driften vom n-body-Wahren (Elemente sind Zwei-Körper,
Epochs altern) — das ist die ehrliche Physik des Katalogs, die TTL-Frische
regelt den Takt.
Verbleibend: Kometen-Records (dcom5_le.dat, 976 B, Multi-Apparitionen) und
der Asteroiden-SPK-Flatten-Pass (Familie spk im Index registriert).
```

**K04** Tycho-2-Katalog (em)
```
tycho2r.cat / tycho2v.cat (je 88 MB): 2,5 Mio. Stern-Punktquellen im ICRS
(Position, Parallaxe, Helligkeit) — die „Galaxie" als echte Katalog-Messung.
```

**K06** EOP (erst nach K01–K05)
```
Erdrotation (Polbewegung, UT1−UTC) für präzise Erd-Stationen; Konzept liegt in
docs/concepts/IAU-2000_EOP.md.
```

Nicht im Zielbild (NAIF-PDF-Bewertung): CK/SCLK/IK/EK/DBK — Sonden-Attitude,
Bordzeit, Instrumente, Events sind für Punkt-Sonden ohne Belang; DSK (Shape-Modelle)
als spätere Option für Asteroiden-Monde.

---

### Kraft-Abdeckung — Zielbild (alle 9 Kanäle)

JPL-SSD trägt nur gravity vollständig sowie em/thermal teilweise (Crawl 2026-08-14);
die übrigen Kanäle sind Kurations-Arbeit an den φ-API-Quellen. Regel: Jeder Kanal
braucht mindestens eine gemessene, physisch gate-konforme Quelle.

| Kanal | Messquelle | Offene Arbeit |
|---|---|---|
| em | Tycho-2-Katalog, Radar-OEMs, DASTCOM-Albedo; Zeitreihen: NOAA GOES, HEASARC, PDS | K04 (Katalog); Kuration (Zeitreihen) |
| gravity | JPL SSD komplett: DE442/441/440, Mond-Systeme, Kleinkörper, Sonden, GM-PCK, DASTCOM | K01 (Monde+CI), K02 (Binary-PCK), K03 (Katalog) |
| acoustic | GONG/SOHO (Helioseismologie), NOAA (Atmosphäre) | Kuration |
| seismic-body | USGS, IRIS, GFZ, PDS InSight/Apollo | Kuration (USGS bereits aktiv) |
| seismic-surface | USGS, IRIS, GFZ | Kuration |
| thermal | DASTCOM H/Albedo + Yarkovsky-Listen; GOES-Thermal | K03 (Parameter); Kuration (Zeitreihen) |
| diffusion | PurpleAir (globales PM-Sensorfeld, live), NOAA SWPC, NASA OMNI | Kuration |
| advective | DSCOVR/SWPC (Solarwind), NOAA GFS | Kuration |
| electric | SWPC, NASA OmniWeb (B-Felder), ESA Swarm | Kuration |

Device-Sensoren (M05) ergänzen die Kanäle lokal; das Radiatorium (M01/M02) ist die
Aktuator-Seite. Der Kurations-Pfad ist unten registriert (Curation & Quellen).

---

### Zentrismus (2)

**Z04** Ephemeris-Kanäle hardcoded auf gravity
```
tau: 86400.0 * 365.0, kernel: 0, force: 1 — drei Stellen im Extract-Pfad.
```

**Z08** Device-Daten verworfen ohne Körper-Ephemeriden
```
/device-Pfad: st_lat/st_lon/st_alt-Gate ohne eph
```

---

### Hack (2)

**H11** Hot-Path-Clones: `all.push(s.clone())` jeden Tick
```
Quell-Liste wird im Loop kloniert statt referenziert.
```

**H12** `query_hash` klont Zellen-Referenzen
```
out.push(samples) — Zell-Container wird pro Treffer kopiert.
```

---

### Fabrikation (3)

**F33** Geschwindigkeit `[0.0, 0.0, 0.0]` — CelestialPolygon
```
CelestialPolygon: v: [0.0, 0.0, 0.0],
```

**F35** State-Vector hardcoded gravity
```
kernel: 0, force: 1, tau: 86400.0 * 365.0, (drei Stellen — gleicher Befund wie oben)
```

---

### Daten zweiter Klasse (2)

**D07** Kein Oszillator-Cap in Rust
```
Das Frontend beschneidet über maxBufferSize (c * 96 > maxBuf); Rust kennt kein Cap.
```

**D08** `/device` scannt nur Device-Quellen
```
matches!(osc.source, OscillatorSource::Device) — API-Quellen fehlen im Scan.
```

---

### Bias (2)

**B05** ISS bekommt spezielles Datenfenster
```
Horizons-Compiler: let months = if *name == "iss" { 0.9 } else { 1.0 };
```

**B06** Hardcodierte Body-Listen in Compilern
```
Verbleibend: horizons_compiler.rs wgccre_for_body-Zwillingstabelle (→ PCK-Hochzeit,
siehe M08). Der Ephemeriden-Compiler ist seit K01 tabellegetrieben
(docs/reference/naif_body_ids.tsv + dynamische Segment-Enumeration, F40 geschlossen).
```

---

### Parser & Spec (8 — aus PARSER_MAGIC.md, PARSER_EVALUATION_MATRIX.md, SOURCES_V2_SPEC.md „Non-Goals")

**P01** Tote Grammatik wird still akzeptiert
```
Der Parser akzeptiert die tote force-Direktive und den 3-Token-field (setzt τ=0)
fehlerfrei → stille 0 Oszillatoren (0 honored, aber ohne sichtbares Signal).
Entscheidung: laut ablehnen (Refused) oder Migration mit Lautsignal. Betrifft
archeology/sources/* (alte force-Grammatik) und phi/research/batches/* (source-Köpfe).
```

**P02** SI-Konvertierung + Unit-Kraft-Matrix
```
convert_to_si + allowed_units_for_force (PARSER_EVALUATION_MATRIX.md); inkl.
mag → W/m² (SUNSPOTS-Rest: Sterne blieben). SOURCES_V2_SPEC („Non-Goals & Known
Parser Gaps"): „units are documentation slots" — heute roher Durchfluss.
```

**P03** per-row τ-Override + vel-Einheitenkonvertierung
```
`z`-Redshift-Key für cmap ist implementiert (z_key-Direktive, Hubble-Flow,
TNS-Transienten-Quelle, 2026-08-14). Rest aus SOURCES_V2_SPEC: τ-Override
je Zeile, vel m/s fix.
```

**P04** kepler_map-Bahnlöser fehlt
```
Extract existiert, Kepler-Gleichungs-Bahnrechnung fehlt (PARSER_MAGIC.md,
gegen Code verifiziert).
```

**P05** HorizonsVec: 0B-Fetches + falsche Timestamps
```
PARSER_MAGIC.md — Horizons-Text-Extract produziert leere/zeitversetzte Samples.
```

**P06** cmap: pmra/pmdec/radvel-Füllung
```
pmra/pmdec/radvel sind seit 2026-08-14 optional (Absenz = Geschwindigkeit 0,
statt Zeile zu verwerfen). Verbleibt: echte Füllung der Eigenbewegung/
Radialgeschwindigkeit (Gaia-Werte einspeisen); plx-Füllung bleibt offen.
```

**P07** extent-pro-Force
```
PARSER_MAGIC.md — gravity → body_radius statt c·τ als Extent-Herleitung.
```

**P08** `field_in` nested + Flatten-Extract
```
PARSER_MAGIC.md / EXTRACT_TYPES.md — geschachtelte Feldpfade und Flatten-Variante.
```

**P09** Fanout vollständig — nur noch Feintuning offen
```
Parallelität (3er-Fenster + fanout_delay-Rate-Limit), Präsenz-Sortierung
(angular distance zur Frame-Oberfläche) und OpenAQ-v3-Anbindung
(stations_flatten + stations_filter) sind implementiert (2026-08-15).
NOAA CDO (thermal) und OpenAQ pm25 (diffusion) leben im Fanout. Offen:
echte Operator-Präsenz statt Frame-Punkt als Sortierzentrum, generisches
Flatten über mehrere Ebenen.
```

---

### Infrastruktur (3)

**I01** Universal Anomaly Reporter
```
PARSER_EVALUATION_MATRIX.md — GitHub-Issue via gh; Kategorien: Physics Mismatch,
API Unreachable, Empty, Malformed, Invalid.
```

**I02** refresh-protected-data: Python → Rust
```
Der API-Mirror-Workflow (refresh-protected-data.yml) läuft weiter in Python —
Rust-Umsetzung mit Auth-Header-Support (siehe Auth-APIs unten). Vorlage:
archeology/ci/*.yml + archeology/ci/secrets.template. (Der Ephemeriden-Teil der
CI ist im Kernel-Flattener-Paket oben aufgegangen.)
```

**I03** Auth-APIs
```
Auth-Header-Support steht (render_headers: {SECRET}-Substitution in Header-Werten,
PurpleAir lebt mit X-API-Key-Header). Verbleibend: Basic-Auth (base64 user:pass)
für GBIF-Fallback — GBIF selbst ist als Presence-Catalog declined (dead_sources.φ).
Priorität-A-Quellen nach phi/sources.φ.
```

---

### Membran & Wahrnehmung (8)

**M01** WebSerial flow-Protokoll
```
Zwei Spezifikationen konsolidieren: 4D-MEMBRANE.md (`flow <force_name> <force_id>
<|Ω|> 1 <tick_ms> <t> <x> <y> <z>`) vs. docs/omegaflow_sense_hardware.yaml
(`flow <channel> <mode> <value> <unit> <duration_ms> <t> <x> <y> <z>`).
```

**M02** ESP32-Mantis-Shrimp-Firmware
```
docs/omegaflow_sense_hardware.yaml existiert (35 Sensoren/Aktuatoren, Pin-Map,
Safety-Matrix). Offen: no_std-Rust-Firmware; Browser-Seite (actuate) + M01.
```

**M03** Audio-Gain ohne tanh
```
index.html: windowMedianExtent() → tanh(Ω·median) — Median mit ∞-Extents ungelöst;
Normalisierung auf die reine Messung steht aus.
```

**M04** Navigation: Wheel-Divisor + Initial-Scale (Nebra-Kalibrierung)
```
Wheel-Divisor 128 im Hauptpfad (gridStep /= 2^(deltaY/128); Touch-Pfad nutzt 512).
Initial-Scale: gridStep = 2**31 → 2³⁷.
```

**M05** Device-Sensoren als SI-4-Token
```
RADIATOREN.md: recordSample(name, value, force, unit) + convert_to_si im Archivar
(Mikrofon→Pa, Kamera→lx, Accelerometer→m/s², Magnetometer→µT). „biotic" kollidiert
mit der Force-Registry — klären.
```

**M06** Wetterstation-Debug-Konsole
```
WETTERSTATION.md: Konsole als 4-Token-Spiegel `name [force, unit]: SI-Wert`;
Frontend-Lookup + SI-Anzeige fehlen.
```

**M07** Command Palette ⌘K
```
SEARCH_COMMAND-PALETTE.md: SIMBAD-TAP-Objektsuche (Presence-Jump), lokaler
Source-Index, Force-Filter, Fuse.js, 3 Phasen.
```

**M08** Horizons-Zwillingstabelle löschen
```
wgccre_for_body in horizons_compiler → PCK (die stype-4-Nutation ist Teil des
Binary-PCK-Pakets oben).
```

---

### Curation & Quellen

- TNS-Transienten (em, celestisch) live seit 2026-08-14: Vollkatalog
  `tns_public_objects.csv.zip` (`format csv_zip`, std-only Inflate in
  src/inflate.rs, z-Distanz via Redshift, abs_mag_from). ~20k
  redshift-tragende Transienten manifestieren; Parse ~70 s pro Fetch
  (Hintergrund-Thread, TTL 3600); das Frontend beschneidet über D07.
- Astro-Broker-Sweep (2026-08-15): A/B-Kandidaten der TNS-Bot-Liste verifiziert —
  keine Accept-Kandidaten (ALeRCE/Fink/OSC dead, ANTARES/IRSA key-needed,
  GCN/SCiMMA/Swift/CASDA parser-def, LSDB decline, HERMES decline
  direction-only — Token verifiziert, aber nur RA/Dec ohne Feldwert/Distanz).
  Dispositionen in `phi/dead_sources.φ`. TNS bleibt der lebende em-Transienten-Feed.
- Host-Kuration 2026-08-15 (Home-Verzeichnis-URLs: 10598 Hosts → 326 daten-artige
  Kandidaten geprobt): 195 mechanische Dispositionen (dead/key-needed/method) +
  119 200er-Klassifikationen in `phi/dead_sources.φ`. 13 Accept-Kandidaten
  (Stand der einzelnen: siehe Einbau-Eintrag):
  · CENC Erdbeben `api.wolfx.jp/cenc_eqlist.json` — seismic-body; Objekt No1..NoN,
    Felder latitude/longitude/magnitude/depth (Strings)
  · JMA Erdbeben `www.jma.go.jp/bosai/quake/data/list.json` — seismic-body; Array,
    Position kodiert in `cod` „+lat+lon+tiefe", mag folgt
  · Pegelonline Wasserstand `www.pegelonline.wsv.de/webservices/rest-api/v2/
    stations/{id}/W/measurements.json?start=P1D` — Wasserstand (gravity/advective);
    Array {timestamp,value}; Station via Stations-Endpoint (lat/lon dort)
  · NHC Hurrikans `www.nhc.noaa.gov/CurrentStorms.json` — advective; activeStorms[],
    latitudeNumeric/longitudeNumeric/intensity/pressure
  · GWOSC `gwosc.org/api/v2/` — gravity; Routen-Index (runs/O1/events) — Event-Route
    wählen, FAR/Skymaps dort
  · ResonanceOne `resonanceone.app/api/now` — em (Schumann); global — Frame at earth,
    Felder schumann_frequency_hz/schumann_index/kp_index/solar_flare_class
  · TeVCat2 `tevcat2.tevcat.org/api/sources` — em cmap; Array, ra/dec Sexagesimal-
    Strings, glat/glon als Float (galaktisch) — Umrechnung nötig
  · DSN `eyes.nasa.gov/dsn/data/dsn.json?t={unix}` — em; dishes{}.az/el/ws/sigs
    (Signalstärke); Positionen = statische DSN-Standortkoordinaten
  · SO2 Vulkanemission ArcGIS `services7.arcgis.com/WSiUmUhlFx4CtMBB/.../SO2datanew/
    ...f=geojson` — diffusion; GeoJSON-Punkte, properties mit VolcanoNumber/Emission
  · Schildkröten-Tracks ArcGIS `services6.arcgis.com/2DGR1sZBUvcPcd8Z/.../
    C_mydas_SSM/...f=geojson` — biotic; lat/lon/date/turtleid
  · Magnetar-Tabelle `www.physics.mcgill.ca/~pulsar/magnetar/TabO1.csv` — em cmap;
    CSV mit Period/Pdot/B/Flux/Dist — RA/Dec-Spalten bestätigen
  · ALFALFA HI-Katalog `egg.astro.cornell.edu/alfalfa/data/a40files/a40.datafile1.csv`
    — em cmap; CSV — RA/Dec/HI-Fluss-Spalten bestätigen
  · MPC cometels `minorplanetcenter.net/Extended_Files/cometels.json.gz` — em;
    Kometen-Elemente (q/e/i/peri/node/tp/H/G) — DASTCOM-analoger Kanal (K03),
    Kepler zur Query-Zeit, kein RA/Dec im Katalog
  Recheck-Liste (URL fehlte f=json/Accept-Header bzw. nur Index geprobt):
  Tides&Currents-Datagetter, coastwatch/ifremer/emodnet-ERDDAP (.json-Endpoints),
  PMEL-CO2-Moorings (ERDDAP, Subset nötig), SERVIR-SoilMoisture + NDBC + Active-
  Hurricanes-ArcGIS (f=geojson), AFAD tadas (Accept-Header), Safecast
  measurements.json (nicht bgeigie_imports), MPC-Daten-API-Unterrouten.
- Host-Kuration Batch 2 (2026-08-15, 300 Kandidaten): 199 mechanische + 91
  klassifizierte Dispositionen in `phi/dead_sources.φ`. 8 weitere Accept-
  Kandidaten (Stand: siehe Einbau-Eintrag):
  · P2PQuake `api.p2pquake.net/v2/jma/quake?limit=100&order=-1` — seismic-body;
    Array, earthquake.hypocenter.latitude/longitude/magnitude/depth
  · Safecast `api.safecast.org/en-US/measurements.json` — em (Gammadosis);
    Array {value, unit:"cpm", latitude, longitude, captured_at}
  · JMA-EEW `api.wolfx.jp/jma_eew.json` — seismic-body; Objekt {Latitude,
    Longitude, OriginTime, Hypocenter} (Magnitude-Feld prüfen)
  · GraceDB `gracedb.ligo.org/api/` — gravity; Routen-Index (api/v2/superevents,
    api/v2/events) — Event-Route wählen
  · USGS-Geomag `geomag.usgs.gov/ws/data/?id=BOU&type=adjusted&format=json`
    — em (Magnetometer-Timeseries XYZF); Stations-Koordinaten in
    metadata.intermagnet.coordinates; INTERMAGNET-IAGA-Codes
  · MSL-Wetter `mars.nasa.gov/rss/api/?feed=weather&category=msl&feedtype=json`
    — thermal (REMS-Druck/Temperatur/Wind auf Mars); ~1,7 MB Sol-Archiv, Felder prüfen
  · SatNOGS `network.satnogs.org/api/observations/` — em (Satelliten-
    Radiobeobachtungen); Array {station_lat/lng/alt, start/end}; Signalfelder prüfen
  · ADSB `api.adsb.lol/v2/point/{lat}/{lon}/1500` — advective (Flugzeuge,
    dynamische lat/lon-Keys); in sources.φ noch nicht live (nur Spec-Beispiel)
  Recheck-Liste erweitert: Argovis (korrekte Query), sensor.community airrohr
  (area-ID), environment.data.gov.uk Readings-Endpoint, BGS-GIN Observation-
  Endpoint, IRSA-Gator (spatial-Syntax), Exoplanet-Archiv TAP, ACTRIS, GTN-P,
  GONG, OceanNetworks, OMNIWeb, AstDyS. SERVIR-SMAP/1km: Service entfernt (404).
- Host-Kuration Batch 3 (2026-08-15, 160 Kandidaten): 107 mechanische + 47
  klassifizierte Dispositionen in `phi/dead_sources.φ`. 5 weitere Accept-
  Kandidaten (Stand: siehe Einbau-Eintrag):
  · EMSC `www.seismicportal.eu/fdsnws/event/1/query?format=json&limit=100&
    minmagnitude=2.5&orderby=time` — seismic-body; GeoJSON-FeatureCollection,
    geometry [lon,lat,depth], properties.source_catalog=EMSC-RTS (mag-Felder
    im Einbau prüfen); noch nicht in sources.φ
  · NDBC-Bojen `services5.arcgis.com/7weheFjxuNkGGiZi/.../National_Data_Buoy_
    Center_Station_Observations/...f=json` — thermal/advective (SST/Strömung);
    Recheck bestätigt: f=json liefert FeatureService (f=geojson für
    FeatureCollection)
  · Hurrikans `services9.arcgis.com/RHVPKKiFTONKtxq3/.../Active_Hurricanes_
    Sampler/...f=json` — advective; Recheck bestätigt
  · Schädliches Phytoplankton `services7.arcgis.com/yHbO69mL1QTGCPQG/.../
    PSF_harmful_phyto/...f=geojson` — biotic; properties {lat/long/date/depth_m,
    chaetocero, rhizosolen, alexandrium…} (Arten-Zählungen)
  · SIMBAD-TAP `simbad.u-strasbg.fr/simbad/sim-tap/sync?request=doQuery&lang=
    adql&format=json&query=…` — em cmap; JSON-TAP-Antwort (ra/dec); noch nicht
    in sources.φ
  Recheck-Liste erweitert: SWPC ace_mag_1h (SWPC-Familie lebt mit 4 Blöcken —
  prüfen, ob ACE-Mag dabei), SuperMAG, WOUDC, AAVSO-VSX, ATNF-PSRCAT-TAP,
  SDSS-SkyServer-SQL, TESS-Target-CSV (RA/Dec-Spalten).
- Einbau 2026-08-15: 9 Kandidaten sind live in `phi/sources.φ` (Sektion
  „Host-Kuration Einbau", probe-verifiziert: 5 via `--probe` mit LSK, 4 key-
  verifiziert — Daten + Keys bestätigt, Probe-Auto-Detect limitiert):
  EMSC, ADSB (mit Berlin-Presence verifiziert), NHC (scalar_of koerziert
  Strings), P2PQuake, Safecast, ResonanceOne, JMA-EEW (Einzelobjekt = eine
  Zeile, Spec §5), SO2-ArcGIS, NDBC-ArcGIS (1726 Samples verifiziert).
  `load_sources_from`-Fix: format/kernel_text-Arm + Flush-Bedingung — Probe
  erkennt LSK-Blöcke wieder (vorher „time absent" für alle).
  Geparkt (Parser-Gap, offen): Pegelonline (wartet auf fanout der parallelen
  Session), USGS-Geomag (Komponenten-Timeseries), GWOSC/GraceDB (Position nur
  via Skymap), TeVCat (Sexagesimal/galaktisch), DSN (statische Dish-Positionen,
  Keyed Object), CENC (Keyed Object No1..NoN), JMA-Quake (Position im
  `cod`-String), Magnetar/ALFALFA (dist_scale: kpc/Mpc roh statt Meter),
  MPC-cometels (K03-Katalog-Kanal).
  Korrigiert zu decline (Spec §3): C_mydas_SSM (position-only, §3.9),
  PSF-Phytoplankton (Zählwerte, §3.2), SatNOGS (position-only), Active-
  Hurricanes-Sampler (Forecast-Track-Punkte, §Model), SIMBAD-TAP (VOTable
  trotz format=json).
- `phi/research/agent_output/batch_*_accepted.φ` (32 Dateien, neue Grammatik):
  konvertierte Blöcke, die nie nach phi/sources.φ übernommen wurden.
- `archeology/sources/sources_gold_pre-cdn_27k_359-domains.φ` (2572 Blöcke, alte
  force-Grammatik) + `sources_recovery_pre-cdn_25k_211-domains.φ` (1924): Migration
  nach Protokoll (docs/source_curation.md); die alte Grammatik wird derzeit noch
  still geparst (Befund oben).
- `archeology/sources/sources_new_untested_14k_new-unchecked.φ` (873),
  `sources_astro_untested_*` (30), `sources_exotic_untested_*` (16, ohne force
  → Gate), `sources_earth_untested_*` (3). Der ehemalige UNTESTED_index.txt ist
  nicht archiviert — per-Domain-Index aus diesen vier Dateien rekonstruieren.
- `sources_recovery_cdn-merged_60k_lost-blocks.φ` (5701 urls, 0 field-Tokens):
  Extract-Parameter aus history/recovery-Dateien zuordnen.
- `phi/research/batches/` (283) + `probe_batches/` (242): alte Grammatik bzw.
  `source`-Köpfe — nicht ladbar (P01).
- `archeology/arena/` (batch_01–21): API-Vorschläge als Fließtext, ungeprüft.
- `scripts/ARCHIVED/` + `scripts/__pycache__/`: historische Python (migrate/verify)
  — nur als Vorlage.
- `archeology/foundation/`: ALIGNMENT_PROTOCOL.md (in AGENTS eingebettet),
  APIs.md/collection.md/gaps.md (Curation-Inventar), sources.φ.spec (tote Grammatik).
- `archeology/reconstruction/*.bak`: Vor-v6-Versionen — als Data-Contract-Referenz
  brüchig, nur Historie.
- `archeology/failed_eph_rust/`: abgelöst durch src/ephemeris_compiler.rs + bsp_reader/pck.

---

### Validation

- `--verify` CLI existiert (URL-Erreichbarkeit); lädt noch keine Quellen.
- Tote Tokens (`force`, 3-Token-`field`, `field_in`, `pos`) werden laut P01 still
  geparst — Lautablehnung fehlt.
- Test-Limit der Curation über 200 Blöcke hinaus erhöhen; 6 Rest-FAILs sind
  Daten-Artefakte (docs/source_curation.md).
- **Laufzeit-Browser (verifiziert 2026-08-14, Desktop GTX 970 / NVIDIA 580):**
  **Firefox = 60 fps stabil** mit der Per-Pixel-Membran (wgpu im Content-Prozess,
  kein Kill-on-Deadline-Watchdog) — der empfohlene Laufzeit-Browser.
  Chrome = 2 fps (Verdacht Software-Adapter: `chrome://gpu` prüfen — zeigt die
  WebGPU-Adapterzeile llvmpipe/SwiftShader, dann NVIDIA-Vulkan erzwingen:
  `--enable-unsafe-webgpu --use-angle=vulkan --enable-features=Vulkan`).
- Browser-Verifikation (Fables Weg, 2026-08-14 verifiziert): Echter Browser + CDP —
  `DISPLAY=:0 chrome --no-sandbox --enable-unsafe-webgpu --no-default-browser-check
  --remote-debugging-port=9224` (WebGPU ist unter Linux per Default aus, der Flag ist
  Pflicht), Consent per CDP-Klick, Beweis: `__of_state().gpu === true`, frames > 0.
- Befund HD 515 + Chrome: Der GPU-Prozess stirbt unter der Per-Pixel-Last —
  Mechanik offiziell geklärt (docs/reference/GPU_WATCHDOG_AND_DEVICE_LOSS.md):
  `exit_code=512` = Chromium-GpuWatchdogThread-Kill (`RESULT_CODE_HUNG=2 << 8`,
  Linux-Frist 15 s, erster verpasster Timeout tötet). Mitigation:
  `--gpu-watchdog-timeout-seconds=60` im Verifikations-Start. Firefox hat keinen
  Kill-on-Deadline-Watchdog — Laufzeit-Verifikation dort noch offen (BiDi-Weg
  skizziert: user.js mit dom.webgpu.enabled + devtools-Prefs, WS auf /session).
- Headless-WebGPU bleibt unverfügbar (adapter null auch mit voller Flag-Matrix;
  Vulkan-WSI braucht ein Display) — der Echt-Browser-CDP-Weg oben ersetzt das.

---

### CI Pipeline

- `refresh-protected-data.yml` (Python inline) → Rust (Befund oben unter Infrastruktur).
- Ephemeriden-Flatten läuft seit K01 in `kernel_flatten.yml`: Index-Job (voll rekursiver
  --index-Crawl → phi/sources_index.φ + docs/reference/KERNEL_INDEX.md, Bot-Commit)
  + Flatten-Job (--fetch-from --systems planets,jupiter,saturn,mars,uranus,neptune,pluto
  --ci-mode → CDN-Assets ephemeris_{body}.bin, --clobber; Body-Manifest in
  sources_index.φ; Horizons-Sonden via horizons_compiler --ci-mode). Monatlich +
  workflow_dispatch.
- Das Python `refresh.yml` im sources-Repo (Kataloge/TAP/Gaia, Release v1.0) bleibt
  bis I02 auf Python — K01-Grenze.
- CDN-Asset-Naming: `{name}.json` (ein Asset pro Quelle, CI überschreibt) — Konvention
  ist der Resolver.

---

### Feature Backlog

- Advective per-Quelle: Wind in tm.w (Kanal verdrahtet, Messquelle fehlt).
- OPeNDAP-Integration.
- Kepler-Bahnlöser (P04), HorizonsVec-Fix (P05), Flatten (P08), field_in nested (P08).
- Command Palette (M07).
- Minkowski 4D Weighting (spacelike→0; kosmisches Skalenproblem: Sonne wäre
  spacelike — scale-Anpassung nötig; MINKOWSKI_FIELD-PERMEABILITY.md).
- Camera: ~19k Pixel-Quellen (4×4-Raster) → WS-Traffic-Hotspot.
- `sensor_config`/`probe_classify`-τ/TTL-Konstanten (60/300/0.01/3600) ohne Herleitung.
- `getRto` min/max 100/5000 ms — nicht 2ⁿ/Φ-hergeleitet (constants.js).
- Probe: `coordinates.2` als alt vs. Tiefe bei Seismik — Vorzeichen offen.

---

### Deferred

- Temporal Topology (TDA, Takens, Transfer Entropy, Surrogates) — LOST_CONCEPTS.md.
- Field Permeability (tanh(vC/g)-Variante ohne TE) — MINKOWSKI_FIELD-PERMEABILITY.md.
- Forschungs-Iterationen (Council): Backend als „langsamer Prior" für Exposure-Kaltstart
  (aktuell: fixe Rampe, keine Anpassung); Exposure-EMA auf dem Silizium (gegenstandslos
  solange die Rampe fix ist).
- Future: Aggregation of Presence, Retro-Manifestation, Total Coherence Integration,
  Nostr-Stationsweb (kollidiert mit LOST_CONCEPTS-Entscheidung) — FUTURE_CONCEPTS.md.

---

### Rejected

- Unknown-Force soft fallback → Parser lehnt unbekannte Kraft ab.
- Default τ-Werte → Gate schließt, wenn nicht deklariert.
- World Bank Indicators → forceless, DROP.
- Yahoo Finance → forceless, DROP.
- Hexagon-Grid, Quadtree-AMR, temporale Akkumulation, Blue-Noise-Rieseln,
  Nahfeld-Splitting → Interpolations-/Zeit-Lügen (Council-Urteil, WGSL_ SHADER.md).
