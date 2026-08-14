# TODO

AGENTS.md is the primary constraint matrix. Git is the history.
Kanonisch: Diese Datei ist das vollständige Register der offenen Arbeit. Erledigtes
wird entfernt (Git trägt es). Kein Eintrag meldet Erledigtes als offen, kein offener
Punkt fehlt. Widerspricht ein Dokument dieser Datei, gilt diese Datei — solche
Drift-Stellen sind unter „Doku-Drift" registriert.

## Stand — 2026-08-14 (LSK/PCK-Hochzeit, Binary v2, Protokoll v6, reine Per-Pixel-Membran)

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
0 Warnings, 0 Errors; Tests: 5 lib + 20 bin.

---

### Gravity-Komplettierung — Kernel-Flattener (K01–K06)

Entscheidungsgrundlage: Der volle Kernel-Bestand (Multi-GB) wird ausschließlich von
der Rust-CI geflattet — lokal kommt nur das per-Body-Binary vom CDN. Keine lokalen
Python-Umwege. Quellen-Inventar: ssd.jpl.nasa.gov/ftp (Crawl 2026-08-14, siehe
Session-Protokoll). Kernel-Familien laut NAIF „Introduction to Kernels"-PDF.

**K01** Rust-CI-Ephemeriden-Flattener (ersetzt Python; schließt die bisherigen
Einheiten-/Monde-Punkte)
```
CI-Workflow (generate-ephemerides in Rust): lädt die vollen Kernels —
Mond-SPKs jup365/sat441l/mar099/ura184/nep098 (volle Zeitfenster, CI hat den Platz),
Mond-Text-PCKs pck.sat441/pck.jup365/pck.mar099/pck.ura182/pck.plu060,
gm_Horizons.pck (GM aller Körper, 15 KB) — kompiliert mit ephemeris_compiler
(+ --ci-mode-CDNUpload-Pfad), überschreibt die CDN-Assets {netloc}/ephemeris_{body}.bin
(v2, Meter) — auch die stale Planeten-Binaries (km-Legacy) werden ersetzt.
Python-Exzision (generate-ephemerides + mirror-cdn aktivieren);
kernel_flatten.yml + sources_index.φ als Flattener-Struktur.
Verifikations-Schritte: (1) Mond-PCKs auf j2/j4-Einträge prüfen (Crawl-Behauptung) —
falls ja, trägt der Flattener die Monde mit echten Harmonischen aus;
(2) Dev-Beweis vor dem CI-Lauf: volle Kernels einmalig lokal kompilieren
(einmaliger Download, nur zur Compiler-Verifikation — der Weg bleibt CI).
Lokal ändert sich nichts: Die φ-Ephemeris-Blöcke für die Monde existieren bereits;
mit dem CDN-Upload erscheinen die Körper (bis dahin absent, 0 honored).
```

**K02** Binary-PCK-Reader (Erde/Mond-Präzision)
```
Neues Modul auf daf.rs: DAF-Segmente — Typ 2 (Chebyshev-Orientierung RA/Dec/W),
CON-Typen (J2/J4/…), ORBNUM (Satelliten-Bahnelemente). Quellen:
moon_pa_de440_200625.bpc (+ .tf), earth_latest_high_prec.bpc, Eros falls vorhanden.
Compiler: stype-4-Nutationssektion (entschieden: additiv — die v2-Parser-Strenge
bleibt, keine Format-Umstrukturierung). Merge-Regel binary-PCK > text-PCK
in pck_bodies einbauen (NAIF-Precedence-Regel).
Runtime: body_pole_at mit echten Nutationsreihen; Zonal-Term leuchtet für
Mond/Eros (Erde hat die Harmonischen bereits).
Detail-Specs im Repo: docs/reference/NAIF_PCK_REQUIRED_READING.md (pck.req, 2021 —
Text-PCK-Keywords + Binary-PCK-Typen 2/3/20) und docs/reference/NAIF_DAF_REQUIRED_READING.md
(daf.req, 2017 — Dateiarchitektur, Summary-Records, Wortadressen).
```

**K03** Kleinkörper-Katalog
```
dcom5_le.dat (4,6 MB, kompakte DASTCOM-Variante): Orbits, H, Durchmesser, Albedo,
Spektraltyp, Rotationsperiode für ~1,4 Mio. Objekte. Bahnelement-Propagation via
Kepler-Löser — verbindet sich mit dem KeplerMap-Befund im Parser-Abschnitt.
gravity + thermal/em-Parameter der Kleinkörper. Format-Referenzen im Repo:
docs/reference/extractPC.for + getascomPC.for (Record-Layout im Quellkopf,
28 Felder/395 Bytes); ID-Indexe: SPKID.DB/MI.DB.
```

**K04** Tycho-2-Katalog (em)
```
tycho2r.cat / tycho2v.cat (je 88 MB): 2,5 Mio. Stern-Punktquellen im ICRS
(Position, Parallaxe, Helligkeit) — die „Galaxie" als echte Katalog-Messung.
```

**K05** FK-Frames
```
moon_080317.tf / earth_assoc_itrf93.tf als Frame-Assoziationsquelle →
hartcodierte Parent-/Name-Tabellen ersetzen (Bias-Befund unten).
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
| diffusion | NOAA SWPC, NASA OMNI | Kuration |
| advective | DSCOVR/SWPC (Solarwind), NOAA GFS | Kuration |
| electric | SWPC, NASA OmniWeb (B-Felder), ESA Swarm | Kuration |

Device-Sensoren (M05) ergänzen die Kanäle lokal; das Radiatorium (M01/M02) ist die
Aktuator-Seite. Der Kurations-Pfad ist unten registriert (Curation & Quellen).

---

### Zentrismus (3)

**Z03** Sonne (NAIF 10) wird im Ephemeriden-Compiler von der Leere-Prüfung ausgenommen
```
if granules.is_empty() && target_id != 10 { … } — Sonne überspringt die Skip-Logik.
```

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

**F40** NAIF-ID unbekannt → "unknown"
```
Name-Mapping im Ephemeriden-Compiler: _ => "unknown"
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
wgccre in ephemeris_compiler ist bereits gelöscht), ephemeris_compiler.rs
all_target_ids + body_id_to_name Tabelle (→ NAIF-ID-Mapping vervollständigen, F40).
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

**P03** `z`-Redshift-Key für cmap + per-row τ-Override + vel-Einheitenkonvertierung
```
SOURCES_V2_SPEC („Non-Goals & Known Parser Gaps"): z_key (Hubble-Flow),
τ-Override je Zeile, vel m/s fix.
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

**P06** cmap: plx/pmra/radvel-Füllung
```
PARSER_MAGIC.md — Parallaxe/Eigenbewegung/Radialgeschwindigkeit ungenutzt.
```

**P07** extent-pro-Force
```
PARSER_MAGIC.md — gravity → body_radius statt c·τ als Extent-Herleitung.
```

**P08** `field_in` nested + Flatten-Extract
```
PARSER_MAGIC.md / EXTRACT_TYPES.md — geschachtelte Feldpfade und Flatten-Variante.
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
AUTH_APIS.md: 25 Secrets offen (Platzhalter in .secrets.local), Auth-Header-Support
im Fetch-System, Priorität-A-Quellen nach phi/sources.φ.
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

---

### CI Pipeline

- `refresh-protected-data.yml` (Python inline) → Rust (Befund oben unter Infrastruktur).
- Der Ephemeriden-Teil (generate-ephemerides + mirror-cdn) ist im
  Kernel-Flattener-Paket (K01) aufgegangen.
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
