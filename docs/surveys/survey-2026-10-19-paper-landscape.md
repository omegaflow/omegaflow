<!--
  title: Paper-Landschaft — 35 Papiere gegen die Forschungslage (2026-10-19)
  class: survey
  date: 2026-10-19
  sha256: ee51ecf60db175d128c035d400c817eb46a7bbbb3c6f72310f8677249a37d5ad
  status: live
-->
# Paper-Landschaft — 35 Papiere gegen die Forschungslage

Vier Agenten, je ein Viertel der 35 Papiere aus `docs/paper/`. Pro Papier drei Fragen:
Kernbehauptung gelesen, externe Lage über `archive_search` gesucht, dann *gedeckt?* ·
*neu seit dem Papier?* · *die eine Lücke?*. Keine Änderung an den Papieren.

## Kanäle (gemessen, nicht angenommen)

- `--openalex` und `--crossref`: durchgehend 200.
- `--base`: **200, Key liegt** (Probe `Planet Nine`: Brown & Batygin 2021, P9 reference population).
- `--semanticscholar`: 200, aber intermittierend HTTP 429 (in 6 von 9 Läufen einer Gruppe; mit `S2_API_KEY`).
- `--arxiv`: **HTTP 429 in allen Läufen** — Kanalausfall, kein Null-Ergebnis.
- Werkzeug-Riss: `archive_search` führt pro Aufruf nur **einen** Netzkanal aus (die zuletzt gesetzte Flag gewinnt); die Agenten fuhren deshalb getrennte Aufrufe.

## Bilanz

| | Zahl |
|---|---:|
| geprüft | **35** |
| gedeckt (ja) | **15** |
| teilweise | **18** |
| nicht gedeckt | **2** |
| benannte Lücke | **35** |

Die zwei „nicht gedeckt" sind keine fehlenden Quellen, sondern **widersprochene Prämissen**:
`causal-arrow-preregistration` (die Flut als Regen→Abfluss-Pfeil; die Ereignisliteratur trägt
eine Fels-Eis-Lawine) und `flyby-path-2-preregistration` (eine Vorregistrierung ist kein
Literaturgegenstand — die Messung nach dem Flug fehlt).

## Tabelle

| Papier | gedeckt | neu seit dem Papier | die eine Lücke |
|---|---|---|---|
| big-bang-echo-sheet-12 | teilweise | SPT-3G D1 × DES Y3 (2026); Rubiola 2026 | TE-Bias bei n = 368 Zellen (Kirkley 2025) |
| cross-screening-tibet | ja | EMS2026-155 Entropie-Netzwerk Himalaya | High-order-TE gegen Restkonfund (Stramaglia 2024) |
| flyby-path-1-cold-cases | ja | Hoffmann & Budnik 2026 (Rosetta) | Gezeiten/Tesseral im N-Body (Acedo 2016) |
| flyby-path-2-preregistration-revised | teilweise | 3GM-Beschleunigung LEGA (angeo-44-731, 2026) | DE441/DE442-Rift (Park 2021 deckt DE442 nicht) |
| h0-lines-register | ja | MNRAS 2026: Priors in der Distanzleiter | unabhängiger ≲ 1 %-Faden (Pantos 2026) |
| laic-arrow-direction | teilweise | EGUsphere 2026-2310; JSEAES 2025 | Fidani 2022 — TE-LAIC existiert bereits |
| planet-nine-kbo-residue | ja | Siraj 2025; arXiv 2609.33080 | Selektionsfunktion (Brown 2017) |
| solar-seconds-matrix | teilweise | Fletcher 2024, Solar Flare Spectroscopy | konditionaler Test 211→193 (Stramaglia 2024) |
| tonga-lamb-crosscheck | ja | GRL 2025; NHESS 2025 | frequenzabhängige Admittanz (JGR 2004) |
| broken-null-control | teilweise | Kirkley 2025 (TE-Schätzer ohne Surrogat) | unabhängige FP-Kalibrierung (Rapp 2001) |
| dark-flow-sheet-8 | ja | Cosmicflows-4 (2026) | Cosmicflows-4-Feld mit derselben HEALPix-Maske |
| flyby-path-2-addendum | teilweise | JUICE-MAG LEGA (2026) | ESA/ESOC-SPK mit Kovarianz |
| galileo-rotor-spin-era-floor | teilweise | Spectral broadening (2026) | volle Frame-`-77000`-CK-Serie |
| hyperscanning-te-method | ja | NBS-Predict (2021) | zweite `jointaction`-Kohorte |
| lead-geometry-direction | teilweise | PTB-XL (2020) | PTB-XL II/V1 mit großem n |
| probe-front-dark-matter | ja | OSIRIS-REx (2024) | DSN TRK-2-34 Rohdoppler |
| sturzflut-tibet-pfeil | ja | „A cascade, not a flood" (2026) | Multi-Source-Langtang-Rekonstruktion |
| twenty-second-band-ground-chain | teilweise | Pioneer-mHz-Retrospective (2026) | Stationskalibrierung (Asmar 2005) |
| causal-arrow-preregistration | **nein** | Acharya 2026 (Daten/Code zur Flut) | ko-lokale Abflussreihe 26.08.2026 |
| depth-phase-echo-fleet | ja | Münchmeyer 2024 (ML-Picker) | unabhängiger Picker auf denselben Wellenformen |
| flyby-path-2-falsification-metric | ja | Cooper 2026 (JUICE/Europa-Vorhersagen) | veröffentlichtes JUICE-2026-Residuum (PRIDE) |
| gic-causal-driver | teilweise | Manshour 2021; Huttunen 2008 | gemessene FMI-GIC-Reihe statt dB/dt-Proxy |
| hyperscanning-te-preregistration | teilweise | Yamasaki 2026 (duales EEG-Datenset) | zweite Kohorte mit demselben Triaden-Design |
| nadel-v-fresh-area-dip-scan | ja | Fink-`/api/v1/fp`; LSST-Alert-Kataloge | Positivkontroll-Kegel (RR-Lyrae/EB) |
| signal-cone-audit-sheet | teilweise | Gavassino 2024 (PRL 132:162301) | Positivkanal mit τ < c (Stenner 2003) |
| terminologie-der-gegenstroemung | ja | Rillig 2024; Karst 2023 | öffentlicher Volltext-Phrasenzähler |
| uranus-rift-ephemerides | ja | Uranus-Astrometrie (AJ 2026) | homogene moderne Uranus-Lösung |
| corona-heating-ladder | teilweise | Kuniyoshi & Morton 2026; Metis 2024 | räumlich aufgelöste Aktiv-Region-Zeitreihe |
| eclipse-clock-worldlines | teilweise | Raster-Methode (2024) | unabhängige ΔT/UT1-Reihe (IERS EOP) |
| flyby-path-2-preregistration | **nein** | 3GM LEGA (2026) | JUICE-3GM/PRIDE-Release nach dem Flug |
| ground-sources-20s-band | ja | DSN-Historiebände (Archiv) | Stations-Kalibrierprotokolle 1972–1990 |
| jwst-disequilibrium-survey | teilweise | WASP-96 b (2026); K2-18 b (2025) | homogene Reanalyse aller 48 Ziele |
| neptune-rift-ephemerides | teilweise | Neptune-Refit 2025; Asteroidenauswahl 2026 | Satelliten-Astrometrie-Katalog |
| solar-cycle-dynamo | teilweise | schwaches Polarfeld 2026 | SIDC-Sonnenfleckenzahl 1700–2026 als zweiter Kanal |
| text-as-data-pioneer | teilweise | keine neue Quelle | modellsubtrahierte Pioneer-Residuum-Reihe |

## Was quer liegt

1. **Die Lücken sind Datenlücken, keine Literaturlücken.** Fast überall fehlt eine
   unabhängige Reihe (zweite Kohorte, Rohdoppler, GIC-Messung, Kalibrierlog), nicht ein Paper.
2. **Selektionsfunktionen fehlen systematisch** — P9, Nadel V, Eclipse: die Null hängt an der
   Auswahl; sie zu modellieren ist der wiederkehrende Referee-Einwand.
3. **Positivkontrollen fehlen** bei `nadel-v` und `signal-cone`: ohne bekannten Dip bzw.
   bekannten τ < c ist die Leere nicht als Nachweis lesbar.
4. **Zwei Prämissen sind bestritten**, nicht bloß unbelegt: die Flut-Pfeilrichtung
   (`causal-arrow-preregistration`) und die Vorregistrierung als Literaturgegenstand.
5. **`--arxiv` war die ganze Runde 429** — die Lage ist über OpenAlex/Crossref/BASE/S2 belegt,
   nicht über arXiv; das ist eine Kanal-Grenze, keine Leerstelle.
