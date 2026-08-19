# TODO

AGENTS.md is the primary constraint matrix. Git is the history.
Kanonisch: Diese Datei ist das vollständige Register der offenen Arbeit. Erledigtes
wird entfernt (Git trägt es). Kein Eintrag meldet Erledigtes als offen, kein offener
Punkt fehlt. Widerspricht ein Dokument dieser Datei, gilt diese Datei — solche
Drift-Stellen sind unter „Doku-Drift" registriert.
(Prüf-Rolle 2026-08-19: die ganze Datei wurde gegen den Code gelesen — Erledigtes
ist entfernt, die offenen Reste aus den geschlossenen Atomen sind hierher gezogen.)

## Nadel Ⅲ — Coronal Heating (TE-Messprotokoll)

Plan: `docs/surveys/handover-nadel3-plan.md` (selbsttragend, 2026-08-19).
Atom 1 (Extraktion) ERLEDIGT (2026-08-19): Archivar-Kern nach lib
(`omegaflow::archivar` — Grammatik, Fetch, Extrakt-Maschine, SI, Typen,
Konstanten, Ephemeriden-Auswertung + `impl Motion` — die Kette wanderte mit,
weil der impl die Auswertung braucht; benannte Abweichung vom
Handover-Wortlaut „Ephemeriden-Auswertung bleibt"), `extract_series`
(Reihen-Ernte), TE nach lib (`omegaflow::te` + `transfer_entropy_lag`,
lag 0 = kanonisch; mathematikerin ruft die lib), sfu-Konversion
(1e-22 W m⁻² Hz⁻¹) + HAPI-Parameter-Ordnung (Info-Ordnung) — beide
Bestandsfehler behoben. `test_live_sources_extract` bleibt im bin (nutzt
fetch_one/load_env/diagnose — der ganze Pipeline-Pfad; benannte Abweichung).
Der main.rs-Schnitt ist strikt Streichung + Importzeilen (Zeilen-Mengen-Differenz
gegen HEAD verifiziert). `#![allow(mixed_script_confusables)]` jetzt auch in
lib.rs (spiegelt main.rs Zeile 1 — die griechischen Identitäten wanderten mit).
Atom 2 (Probe `nobel_probe_corona`): Instrument gebaut, erster Testlauf
gefahren, **Kontrolltest repariert** (2026-08-19, zweiter Lauf). Die
Nullkontrolle bricht nicht mehr: alle vier Dichte-Paare halten unter der
phasenrandomisierten Schwelle (Spektrum erhaltend, std-only FFT in
`src/te.rs` + `surrogate_stats_phase`/`surrogate_stats_block`
Block-Bootstrap) und brachen unter der naiven Shuffle-Schwelle — die
naiven Surrogate waren das Artefakt. Der Befund des ersten Laufs kippt
mit der korrigierten Schwelle: **Bz → 304 und 304 → 284 sind still —
der Alfvén-Kanal trägt keinen Pfeil; der DAG schrumpft auf
EUV-304 → X-Ray (+ Bz → X-Ray, beide lag 0/1).** 0 honored: Stille ist
die Antwort. Offen vor jeder physikalischen Aussage:
- Mehrfachvergleichskorrektur über die Matrizen und Kanalpaare (2
  Pfeile bei 20 getesteten Paaren ohne Korrektur — der erwartete
  Falsch-positiv-Bereich ist nicht verlassen);
- Lag-Wahl: lag 0 ist Default, kein Sweep — Robustheit ungeprüft;
- KDE-Bandbreite: Silverman-Heuristik, Sensitivität der Urteile gegen h
  ungeprüft;
- Fenster-Kongruenz: Sekunden-Kontrolle lief auf dem ~2-d-Fenster;
  OMNI↔GOES-Schnittmenge bleibt leer (stopDate 06.08.).
Kurations-Befunde: NGDC-NetCDF (GOES-30d) trägt 404 → fehlt-
Registratur, kein Block; `1/cm3`-Alias ergänzt; HAPI-Reihen über die
Identitäts-LSK; Radio ↔ GOES trägt keine Aussage (n = 15, n-Schwelle
30); Laufzeit gemessen ≈ 80–90 min — vor dem nächsten Lauf die
O(n²) × Surrogate-Kosten gegenrechnen.
GOES-30d-Archiv-Block bleibt pending (kein lebender Kandidat);
bis dahin trennt der OMNI-Ingest-Verzug (stopDate 06.08.) OMNI↔GOES —
Schnittmenge leer, im Protokoll fehlt.

## Archivar — Architektur

- Membran-scoped Cache statt Blockuniversum (2026-08-17): der Archivar lädt
  flache Katalog-Assets komplett in den Spatial Hash — das ganze Feld im
  Speicher. Die Membran braucht nur die Hülle um die Presence (dilatierter
  Suchradius). Richtung: räumlich gebinnte Assets (HEALPix), der Archivar
  holt nur die Bins, die die Hülle überlappen. Kein NASA-Denken — nur was
  die Membran benötigt. Deckt auch: tess_lightcurves.bin ~500 MB (alle
  bestätigten Transit-Wirte, SPOC-2-min) lädt heute ganz.
- Lokaler Crossmatch zweier Quellen (pending, 2026-08-18): Lasair
  (ZTF-Transienten, live, em) trägt kein z in `objects` → die Objekte
  liegen auf der Himmelssphäre (0 honored). Die TNS-Tabelle
  (`tns_public_objects.csv.zip`, `z redshift`) kennt für die SN Ia die
  echte Rotverschiebung. Die wahrste Lösung: der Archivar matched die
  Lasair-Objekte beim Laden lokal gegen die bereits geladene TNS-z-Tabelle
  — kein Datenverlust, keine Lüge, nur Anreicherung dort, wo eine
  Übereinstimmung vorliegt. Ein eigenes Code-Feature (Join zweier Quellen
  im Spatial Hash), kein Quellen-Block.
- Ephemeriden-Kaltstart (2026-08-18): Frame-Anker laden jetzt als erste
  Phase über `curl --parallel --parallel-max 8` (HTTP/2, retry-all-errors);
  die Membran zeigt das Sternfeld sofort, die Planeten folgen. Offen:
  per-Anker-Extraktion (sun/earth sofort extrahieren statt nach der ganzen
  Anker-Phase) für wörtliches „Sekunden"-Laden; der Kalt-Download
  (~360 MB) bleibt einmalig bis zum Warm-Cache.
- 477 547 deep je Sense ohne Richtungsfilter — treibt Speicher und maxms.
- OOM-Befund: ein Lauf, dessen GPU-Thread beim Pipeline-Bau panikte, lief
  als Rumpf weiter (Archivar + Audio, 3,2 GB) — der tote GPU-Thread ist
  nicht der tote Prozess.
- SPK-Segment-Payloads lazy laden statt upfront (strukturell — sonst
  wächst die Ramlast mit jeder Kernel-Generation).
- Asteroiden-SPK-Flatten-Pass (Familie spk im Index registriert).
- K06 EOP: Erdrotation (Polbewegung, UT1−UTC) für präzise Erd-Stationen;
  Konzept: docs/concepts/IAU-2000_EOP.md (72-B-Orientierungsmatrizen
  leben, die Erdrotation fehlt).
- X-flagged-Sterne ohne Tycho-1-Eintrag: Positionen lägen im Guide Star
  Catalog (I/220, ~25 Mio) — offen.
- Puffer-Schrumpf fehlt: ensureFieldCapacity schrumpfte im Browser bei
  langsamen Frames; nativ wächst nur.
- 2-Finger-Zeitschub fehlt nativ — die Wahrheit des Touchpad-Docs:
  Pinch = Zoom, 2-Finger-waagerecht = ZEIT-Schub, 2-Finger-senkrecht =
  vor/zurück; das native implementiert heute Pan+Zoom+Roll ohne
  Zeit-Achse.
- Deep-Link-Geschwindigkeit: `#x,<x>,<y>,<z>,<t>` existiert
  (main.rs presence_init, [f64; 4]) — die Geschwindigkeit `[,vx,vy,vz]`
  fehlt.
- Audio-Ausgabe nativ = rohe Samples nach stdout (Pipeline-Ausgang;
  im Log erscheint Datenmüll) — bewusst oder ein eigener Ausgang.
- Der Subpixel-Anlauf (Rgba32Float, 9 Mio Messzellen) wartet auf einen
  nicht-aufgeblähten Wiedereinstieg; die Messung lebt in
  docs/surveys/messpunkt-verteilung.md.

## Die Sphären des Unsichtbaren

- Atom 2 (Ringe: eigener rings-Buffer + WGSL ring_transmission,
  Literatur-τ mit Provenienz) — offen, eigene Session.
- Atom 3 (Warp: Linsen-Kompiler — Gaia-BH-Kandidaten + ATNF-Pulsare mit
  gemessener Masse; WD-Modell-Massen ausstehend; f64-Fold-Muster aus
  Atom 1) — offen, eigene Session.
- Atom-1-Grenzen (registriert): der 3D-Orbit des Planetenpunkts bleibt
  ausstehend — Ω (Azimut im Sky-Frame) ist ungemessen, der Schatten ist
  Ω-frei, ein Punktorbit wäre geraten; der Schatten manifestiert nur, wo
  der Stern rendert (Deep-Fluss ≥ 1e-4, ~mag 10); pscomppars trägt
  mehrere Parametersätze je Planet und keinen default_flag — erster Satz
  je Planetenname zählt; fehlt ein Element → kein Schatten (0 honored).
- LuckyStar: decline (Vorhersagen sind Modell, keine Messung; die
  Ergebnisse-Server liefern nur abgeleitete Fits) — der rohe
  em-Lichtkurven-Kanal der Fresnel-Sphäre bleibt ausstehend.
- Okklusions-Reste (aus der Massen-Okklusion): kontinuierliche Opazität
  (Partial-Transmission), atmosphärische Dämmerung, kleine Skala
  (Terrain/Bauten — der Mechanismus ist skalenfrei, die Daten fehlen),
  Oszillator-Eigenradius als Rekord-Slot.
- Atom 1 deckt den Weg für Ringe/Warp — noch kein Konzept-Dokument.

## Der spektrale Oszillator — die Frequenzachse (Konzept: DER_SPEKTRALE_OSZILLATOR.md)

- Atom A (Protokoll v8): ERLEDIGT (2026-08-19) — Record 24×f64
  (`freq`, `bin_width`, 0.0 = Punktquelle); Frame `0xCF 0x86 0x08`;
  meta-Stride 12→16 f32, props-Stride 3→4 in beiden WGSL; Befund der
  Umsetzung: meta[3] war NICHT frei (Tolman-z bei em) — die neuen
  Slots sind meta[11]=freq + meta[12]=bin_width; der JS-Parse setzt
  jetzt das z wie das Rust-Pack (Schichten identisch); Golden-Test
  m[11]/m[12], naga-Validierung, BINARY_PROTOCOL.md v6→v8, AGENTS.md,
  prompt.φ v8. Korrektur der Prüf-Rolle (2026-08-19, verifiziert):
  die Verifikation war default-only — `cargo check --features
  browser_relay` bricht mit 9 Fehlern; Schritt 0 von Atom B hat das
  repariert (beide Gates 0/0, verifiziert).
- Atom B (Spectral-Compiler): ERLEDIGT (2026-08-19) — Binär
  `spectral_compiler` (CSV-Kontrakt → spectra.bin, ν = c/λ,
  E_ν = E_λ·λ²/c, bin_width aus dem nativen λ-Gitter,
  quality_flag-Filter, Epoch = Monatsmitte → TDB via LSK,
  `--ci-mode` → CDN Tag ssd.jpl.nasa.gov); Kontrakt `0xCF 0x86 0x01
  [epoch_tdb] [count]` + Records [freq, bin_width, val] f64 LE in
  `src/spectral.rs` (parse/write + Golden-Tests); Zweig `format
  spectral` im Fetch-Loop (Muster catalog_tycho) mit `SpectralHash`
  (ICRS-Punkt + Bins, medienneutral — Stern, Sonne, Ozean) →
  sense_membrane + sense_buffer expandieren je Bin einen OscRecord
  am selben Punkt; sources.φ-Block (spectra.bin, on earth 19.82
  -155.47 0, τ = 2.628e6 s, ttl 86400, force em); BINARY_PROTOCOL.md
  Sektion „Spectral Bin File v1“; Schritt 0 = Relay-Reparatur
  (OscRecord überall, Stern-Push + freq/bin_width = 0.0 Punktquelle);
  Schritt 1 = Füll-Schicht (Channel + Oscillator tragen freq/bin_width,
  ~20 Konstruktionsstellen auf 0.0, query_hash liest osc.freq/
  osc.bin_width).
  Befund der Umsetzung (2026-08-19, live verifiziert): die
  txt-Route des Queue-Drafts ist tot (404) — die Monats-SSI-Messung
  existiert nur als netCDF-4/HDF5 (magic 0x89484446, deflate);
  kein ASCII-Weg (ERDDAP/THREDDS 0 Treffer), die reference-spectra.txt
  sind Modelldaten (LuckyStar-Präzedenz: refused). Die Ernte ist
  pending — der HDF5-Reader ist ein eigenes Atom (netcdf.rs sagt es
  selbst); der Compiler frisst die tabellarische Form und benennt
  unlesbare Container (0 honored). Stationshöhe unverifiziert
  (Frame-Alt 0). CI-Register-Zeile (Registrierpflicht): im
  sources-Repo `spectral_compiler --input <csv> --month YYYY-MM
  --lsk naif0012.tls --ci-mode` — liegt außerhalb dieses Workspace.
  Fundorte: Queue-Draft master.φ:31611 (korrigiert), Concept
  DER_SPEKTRALE_OSZILLATOR.md:107. Folgen: ONC-HSD-FFT, Gaia-XP,
  LISA-PSD + CMB-Power, GONG + miniSEED — je eigene Session.
- Atom C (band-selektives Rendering): offen — Shader akkumuliert pro
  Band; Stillekarte band-selektiv, Lichtkegel-Differenz dispersiv,
  chromatischer Dip als SED-Messung.
- Atom D (Phase): terminiert nach C — Beats/Interferenz brauchen die
  komplexe FFT; PSD-Bins tragen sie nicht (0 honored).
- Regeln: kein Namens-Trick (Frequenz lebt als Token, nie im String),
  kein Skalar-Schallpegel aus Spektren errechnet, jedes Atom ein
  vollständiges Session-Artefakt.

## Stern-/Asteroiden-Physik — abgeleitete Geometrie + Ernte-Folgen

Die Daten sind geerntet (Sternkinematik pmra/pmdec/rv + Farbe
Teff/BPmag/RPmag/Gmag via gaiadr3-Crossmatch; Asteroiden-Größe via
NEOWISE/AKARI in `phi/pipeline/katalog/asteroid_diameters_*.φ`). Offen ist
die Nutzung — reine Geometrie, die sonst nirgends liegt, weil alles einen
ICRS-4D-Rahmen teilt:

- Hill-Sphäre je Asteroid: r = a·(1−e)·(m/3M☉)^⅓ — Formel repariert;
  `hill_radius_m` ist heute nur Gate (is_none im Hash, main.rs
  2163/2297), der Wert fließt nirgends — Manifestation (Hill-Radius als
  räumliche Reichweite) bleibt offen.
- Hydrostatische Abplattung aus Rotation: Rotationsperiode (LCDB) +
  Radius (NEOWISE) + Dichte (Masse) → Oblatheit im Gleichgewicht (drei
  Kataloge übereinander, niemand macht das systematisch).
- Co-moving Gruppen / Sternströme: Position + 3D-Geschwindigkeit →
  Mitgliedschaft als Geometrie des Geschwindigkeitsfelds.
- Sternbegegnungen: welche Sterne nähern sich der Sonne (Gl-710-
  Problem), für JEDEN Stern live.
- Paarweise 3D-Sternabstände (N², auf Anfrage).
- Oberflächengravitation + Fluchtgeschwindigkeit der Asteroiden mit GM:
  g = GM/r², v_esc = √(2GM/r).
- Neue Quellen (grind-pro, heikler Join/Parsing): LCDB-Rotationsachsen
  (Pol, nicht nur Periode), DAMIT-Formmodelle (3D-Formen → j2/r_eq).
- Empfohlene Reihenfolge: Hill/Abplattung → LCDB/DAMIT.
- H-Schätzung vs. NEOWISE für die Körper, wo DASTCOM einen abgeleiteten
  (nicht gemessenen) Radius trägt — registriert, nicht entschieden.
- Sternbin-rv-Ernte (pending): die Compiler schreiben 44-B-Records
  (8+8+7×4: ra, dec, pm_ra, pm_de, plx, mag, flux, farbe, rv in m/s);
  `parse_star_record` verlangt exakt 44 Byte, kein rv=0.0-Ersatzwert
  (0 honored).
  **Offen**: Rekompilation von `dr3_stars.bin` + `bright_stars.json` +
  CDN-Remanifestation (CI, kernel_flatten-catalogs) — die
  Legacy-40-B-Bins manifestieren nicht, die Sterne bleiben dunkel, bis
  die 44-B-Binaries gebaut sind (pending, keine Fabrikation); erst
  danach trägt der Katalog die geerntete Radialgeschwindigkeit. Bis
  dahin fließt rv nur aus den JSON-cmap-Quellen (denis `radvel rv`).
- CDN-Rekompilat ephemeris v3: die ephemeris_{body}.bin-Assets sind noch
  v2 — der nächste kernel_flatten-Lauf schreibt v3 (0x02 + u16-Präsenz-
  Maske). Bis dahin liest der v2-Arm (CI-Reihenfolge eingehalten: Code
  zuerst, Rekompilat folgt). Bis dahin tragen alt-Slot und GM-Slot das
  benannte Wire-Pad.
- kernel_flatten-Neulauf: ephemeris_compiler n_sections 2→3
  (rotationslose Körper wurden verworfen, Rotation abgeschnitten) —
  CDN-Neukompilat verifizieren (rotationslose Körper laden, Rotations-
  Matrizen präsent).

## Ausgabe-Flächen & Sensoren

- SurfaceRadiator-Implementierungen offen: Bluetooth (Smartwatch) und
  HID (Force-Feedback); Vibration hängt am ESP32-Prototyp. (Serial-TX
  lebt: OMEGAFLOW_SERIAL_OUT, 115200, eine Zeile je Tick.)
- Kamera/Mikro/IMU nativ: die Daten existieren, der Sensor-Pfad fehlt
  (Batterie + Zustimmungs-Gate leben).
- Gamepad-Oszillatoren: die gilrs-Steuerung lebt hinter
  `--features gamepad` (Navigation: fold/jump/Rotation); das Gamepad als
  Sensor-Oszillator ist offen — die serielle Ingress-Vokabel deckt
  ESP32, HID-Gamepad steht aus.

## Browser-Relay

- refused-else ohne body-Deklaration (Relay-Rest): SurfaceFlow für
  spd/hdg lebt (index.html 236-249, frame_motion in main.rs) — der
  offene Rest ist nur noch refused-else ohne body-Deklaration.
- Der eingefrorene index.html/fieldShader-Snapshot trägt die tote
  Rotation noch (GRID_TO_ANGLE = 2^62, index.html 42/1245) — B1,
  bleibt registriert, falls der Relay wieder auflebt.
- M01 WebSerial-flow-Protokoll: zwei Spezifikationen konsolidieren —
  4D-MEMBRANE.md (`flow <force_name> <force_id> <|Ω|> 1 <tick_ms> <t>
  <x> <y> <z>`) vs. docs/omegaflow_sense_hardware.yaml (`flow <channel>
  <mode> <value> <unit> <duration_ms> <t> <x> <y> <z>`). SerialSurface
  schreibt heute rohe lum-Werte (main.rs 16790).

## Membran & Wahrnehmung

- M02 ESP32-Mantis-Shrimp-Firmware: docs/omegaflow_sense_hardware.yaml
  existiert (35 Sensoren/Aktuatoren). Offen: no_std-Rust-Firmware;
  Browser-Seite (actuate) + M01.
- M03 Audio-Gain ohne tanh: index.html windowMedianExtent() →
  tanh(Ω·median) — Median mit ∞-Extents ungelöst; Normalisierung auf
  die reine Messung steht aus.
- M04 Navigation (Nebra-Kalibrierung): Wheel-Divisor 128 im Hauptpfad
  (Touch-Pfad 512); Initial-Scale: gridStep = 2**31 → 2³⁷; die native
  Parität (−/= ×4, keine Wheel-Kalibrierung) ist offen.
- M05 Station-Sensoren als SI-4-Token: recordSample(name, value, force,
  unit) + convert_to_si im Archivar (Mikrofon→Pa, Kamera→lx,
  Accelerometer→m/s², Magnetometer→µT). „biotic" kollidiert mit der
  Force-Registry — klären.
- M06 Wetterstation-Debug-Konsole: Konsole als 4-Token-Spiegel
  `name [force, unit]: SI-Wert`.
- M07 Command Palette ⌘K: SIMBAD-TAP-Objektsuche (Presence-Jump),
  lokaler Source-Index, Force-Filter.
- Wetterstation: der 4-Token-HUD („wind_speed [advective, m/s]") fehlt
  nativ — kommt mit der Messreihe.
- Advective per-Quelle: Wind in tm.w (Kanal verdrahtet, Messquelle
  fehlt).
- OPeNDAP-Integration.
- Camera: ~19k Pixel-Quellen (4×4-Raster) → WS-Traffic-Hotspot.
- `sensor_config`/`probe_classify`-τ/TTL-Konstanten (60/300/0.01/3600)
  ohne Herleitung — Draft-Konvention (A6): die Werte sind die
  Sensor-Registry-Kadenzen (serial 60 s, battery 300 s) und die
  Quellen-TTL-Familie (86400) — KEINE Messungen der Quelle; die τ-Gate
  beim Einbau entscheidet.
- `getRto` min/max 100/5000 ms — nicht 2ⁿ/Φ-hergeleitet
  (constants.js).
- Probe: `coordinates.2` als alt vs. Tiefe bei Seismik — Vorzeichen
  offen.

## Operator-Messungen (ausstehend)

- Radial-Profil eines isolierten breiten Gauß-Punkts (e^(−r²/2)) am
  Fenster — Messung + e/E/P-Gefühl gehören dem Operator.
- Sternenhimmel relativ zur Live-Em-Referenz statt absolut (+18) — das
  Operator-Urteil entscheidet, ob der absolute Anker zurückkehrt.
- Galaxien-Zoom-Verifikation (deep-Zahl im HUD; bei grid 2^39 noch 0 —
  Proxima bei 4,2 ly ≈ 2^45,5).
- Fireball-Operator (sum vs. mean im `fold`) — Live-Verifikation offen.
- Audio-Phasen-Invariante dokumentieren: sr = 44100, ganzzahlige
  Frequenzen, 1-s-Noten → glatter Nulldurchgang am Tick-Ende; bei
  sr-/Frequenzwechsel bricht sie.
- Sternenhintergrund (integrierter Glow der 1/d²-Schwänze, Milchstraße):
  einmalige tiefaufgelöste Integration, glattes Feld.

## Wahrheitsfindung — Urteil-Verzeichnis (nur offene Urteile)

Der Mechanismus gegen den Verlust: **kein Top-N — das Verzeichnis ist
vollständig.** Jede Funktion des Systems, jedes Konzept, jede fehlende
Funktion trägt ein Urteil. Was nicht hier steht, existiert für die
Zukunft nicht. Der Inventar-Prozess ist wiederholbar: `grep -nE
"^\s*(pub\s+)?(async\s+)?fn"` über src/main.rs + src/lib.rs + src/bin/*
+ die WGSL-Entry-Points (`@vertex/@fragment/@compute fn`) +
`docs/concepts/*` + die Registry (phi/sources.φ, phi/dead_sources.φ).
Urteile: **WAHR** (die Messung ist die Messung der Sache selbst — der
Gradient schweigt), **UNWAHR** (Fabrication, Ersatzwert, Default — der
Gradient spricht), **AUSSTEHEND** (die Daten existieren, die Forschung
oder der Bau fehlt), **ERSETZT** (von einem stärkeren Gesetz abgelöst —
ehrenhaft), **VERSIONIERT** (auf einem Zweig gesichert, wartet).
Erledigte Urteile trägt Git — hier stehen nur offene und navigierende
Zeilen.

### Die Concepts (offene und navigierende Zeilen)

| Konzept | Stand | Urteil |
|---|---|---|
| WGSL_SHADER | Konzept | VERSIONIERT — die atmende Membran (σ-lerp, Hysterese, Interest-Map); die Zell-Achse ist der Enkel, der Vorfahr atmet stufenlos |
| 4D-MEMBRANE | ARCHIVED | WAHR — Trommelfell-Doktrin (keine Kamera, Manifestation real ohne Zuschauer); hier starb get_expose; M01 referenziert sie |
| MINKOWSKI_FIELD-PERMEABILITY | ARCHIVED | WAHR — die EXPOSURE-PARABEL (Parabel des Sondierens, Wasser-Form, tanh-Rückkehr) = Ethik §9; VERSIONIERT unten |
| LOST_CONCEPTS | ARCHIVED | WAHR — das Verlust-Register des ersten Zeitalters (Minkowski, Topologie/TE, Permeabilität, Aperturen, Nostr, Überbau, ANISE, Tiles, WebGL2, Observer) — „await their return" |
| FUTURE_CONCEPTS | PLANNED | WAHR — Eis/Wasser/Dampf, Kohärenz-Integration, Retro-Manifestation, Mycelium-Web |
| REMOVE_BIAS | Plan | ERSETZT — ausgeführt (Surface-Frames, body_name, Station-materialize lebt im Code) |
| WETTERSTATION | Konzept | AUSSTEHEND — der 4-Token-HUD fehlt nativ; kommt mit der Messreihe |
| PARSER_MAGIC | DEPLOYED | WAHR — offen: cmap-Füllung, Auto-Frame, extent pro Force |
| PARSER_EVALUATION_MATRIX | SUPERSEDED | ERSETZT — SOURCES_V2_SPEC ist die kontrollierende Spec |
| SOURCES_V2_SPEC | LIVE | WAHR — die Spec, das τ-Gate, die Force-Gate-Prinzipien |
| SI_UNITS | SUPERSEDED | ERSETZT — SI-Konversion total (Option<f64> am Anker, unconverted = unmanifested + registriert); mag/Mw/dex/Crab/counts pending Kuration |
| IAU-2000_EOP | PARTIALLY DEPLOYED | WAHR — 72-B-Orientierungsmatrizen (Binary v2 trägt sie); die Erdrotation ist K06 (Archivar-Abschnitt) |
| SEARCH_COMMAND-PALETTE | PLANNED | AUSSTEHEND — ⌘K nie gebaut (M07) |
| KERNEL-CURATION-CI-AUTOMATION-PLAN | Plan | ERSETZT — K01 geschlossen (kernel_flatten.yml lebt) |

### Die Abweichungen (offen)

- Gravity-Hardcodes im Extract-Pfad (Z04/F35 — Ratsbefund): drei Stellen
  hartkodiert auf gravity statt aus den Daten — beim Vollzug
  verifizieren.

## Source-Port — der eine Pfad

Alle Source-Arbeit läuft über `docs/SOURCE_PORT.md`. Arbeitsfläche:
`phi/pipeline/` (queue/, park/, stage/, ledger.φ, prompt.φ). Bestand:
`phi/pipeline/katalog/`. Register: `phi/sources.φ` + `phi/dead_sources.φ`.
Der Sweep liest `phi/pipeline/stage/*_converted.φ`. Stale-Specs gebannert:
PARSER_EVALUATION_MATRIX.md + EXTRACT_TYPES.md (SUPERSEDED by
SOURCES_V2_SPEC.md).

- Kompilat-Pfad in die Zustandsmaschine holen: der Weg tap_index →
  kernel_flatten.yml → tap_compiler → CDN → sources.φ läuft außerhalb der
  Zustandsmaschine (SOURCE_PORT §4) — kein ledger-Eintrag, kein
  Pfadkarten-Eintrag. Deshalb zerfleddert ein großer Katalog in Queue/
  Metadaten/Weights/Stage, ohne je aufgelöst zu werden. Vereinheitlichung:
  eine Kompilat-Stufe (`entdeckt → kompiliert → disponiert`) in ledger.φ +
  Pfadkarte; `disponiert` räumt die Discovery-Reste. Berührt SOURCE_PORT.md
  + ledger.φ + ggf. main.rs (--fish-Flag).

Offen (Detail in phi/pipeline/ledger.φ):

- Die Linse: Folgewelle — NASA-CMR-Keywords + GBIF-Tags downloaden,
  Library feinwägen; --port ersetzt --gold
- Probe-Stufe: nächste Welle — neue Kandidaten aus den Katalogen in
  batches/ nachrücken
- Queue: 10 Untested-Korpora (14k/13k/15k/7k/2k/183l/astro/earth/
  exotic/candidate-staging) — Port durch die Prozedur; astro-Korpus:
  28 Blöcke → manueller Port
- Bestand: 38 offene VizieR-Bulks, IRSA/GAVO/ARI/ExoArchive-Inventare,
  GCNS/MWSC (Kompilat, liegen in GAVO dc.g-vo.org), 77 Archeology-Gaps,
  ESA-Kandidaten (Aeolus key-needed, SMOS parser-def), FRB-Union,
  Arena/Foundation/Research-Schatz im Archiv
- Grind-Einbau offen: 32 ArcGIS-Drafts (thermal/seismic/diffusion/em/
  advective/gravity); ARI GCNS (331.312 Sterne ≤100pc) + MWSC (3.006
  Haufen) als Kompilat-Kandidaten; 8 VirES-Drafts (CHAMP/GRACE/GOCE/
  CryoSat MAG/DNS/WND/TEC/KBR); archeology-gaps 77 Kandidaten (AERONET,
  IERS-EOP, Fireball/Sentry, Xamin-TAP, GONG2, GIRO-Ionosonde,
  e-CALLISTO …) als nächster Grind; FRB-Union-Merge mit
  TNS-Namens-Normalisierung (FRB121102 ↔ FRB20121102A) + frbcat.org-CSV
  als Quelle
- Nachlauf: VirES-Vollprobe (64 Drafts, Datei ABSENT) + DONKI-Familie
  (CME-Draft, Datei ABSENT)
- Park: Pegelonline, USGS-Geomag, GWOSC/GraceDB (Skymap), DSN, CENC,
  JMA-Quake (cod-String), SDSS-SkyServer
- Rats-Befund Harvester-Binaries (2026-08-17): kafka_harvester/
  fdsn_harvester als eigenständige Binaries zulässig — std-only bindet
  den Archivar-Runtime, nicht die Produktions-Tools. Reihenfolge:
  (1) Force-Gate zuerst — Alert-Ströme ohne Feldwert am Punkt fallen
  (ANTARES, dead_sources.φ); (2) REST-Pull zuerst — GCN circulars/
  notices, IceCube, GraceDB, MPC tragen REST → rest_harvester deckt
  sie; (3) nur ZTF (Kafka+Avro, IRSA-Auth declined) und FDSN
  dataselect (miniSEED-Zeitreihe) brauchen echte Decoder — beide
  AUSSTEHEND hinter dem Gate; miniSEED-Frage: eine Waveform zerfällt
  in Samples (TESS-Muster, [t, flux]-Reihe) ODER in Bins
  (Spektral-Atom) — das Instrument deklariert seine Basis. Seit
  Protokoll v8 (2026-08-19) trägt der Record `freq`/`bin_width` —
  0.0 = Punktquelle, die ehrliche Abwesenheit (kein Pflicht-Feld mit
  Fabrikation); (4) Hand-Client vs. Crate: AUSSTEHEND — fällig erst, wenn
  ein Kafka-only-Feed das Gate passiert.
- Offen (src/ — Rust-Kybernautin): SuperMAG (leading-line „OK"-Strip
  lebt; Positions-Join + station-Filter bleiben server-blockiert —
  db-get-Fault, phi/-Zugang logon-only)
- Kraft-Abdeckung: acoustic/electric/thermal/advective/diffusion-
  Kuration offen — electric: GIC-Netze + Live-E-Feldstärke (kein
  Feed); GLM ist em (Ratsurteil); WWLLN radio-em vs. Entladung-electric
  bleibt Force-Gate-Frage
- Die drei Ports der Nadeln — offene Reste: IONEX-GIM — der
  `format ionex`-Parser lebt, der Kanal ist AUSSTEHEND (CDDIS verlangt
  Earthdata-OAuth, GFZ/BKG/IGN-Routen 404/000 am 19.8.2026); kein
  Block im Register, bis eine Route anonym lebt oder der
  Earthdata-Account existiert. WARTEND: SuperMAG (oben), Gaia DR4
  (2.12.2026 — Recompiler der 44-Byte-Records)
- Teleskop-Inventar (ledger.φ geparkt): GCN-API v0.1 tot (Einstein-
  Probe-Blöcke master.φ stale, kein Listen-Feed → SVOM-Block nicht
  baubar); NRAO = Angular-SPA `data.nrao.edu/portal/` (kein REST);
  CHIME = CANFAR-DOIs statt API; svom.ac.cn = HTML, Zertifikat
  abgelaufen; ESA-AMA-TAP-Basis ungefunden (nur EAS/Euclid lebt);
  Keck/KOA unkuratiert (keine URL im Bestand); eROSITA DR2 =
  HTML-Landing; MAGIC/HAWC = HTML+FITS-Portale, TLS-Kette
  unvollständig; LHAASO = News-Seite 2021 → Decline. IRSA
  spherex.obscore = VOTableJSON-Atom; Euclid mer_catalogue =
  SpaltenAusMetadata-Atom; ESO tap_obs = echte CSV, aber probe_csv
  klassifiziert Header-CSV nicht (Probe-Limitierung); Pan-STARRS dr1
  mean = Endpoint lebt, Probe-Env kennt {ra}/{dec}/{radius} nicht
  (Nachweis im Register-Lauf offen). Befunde:
  phi/pipeline/research/agent_output/verify_astro{,_b}_2026-08-19.φ.
- Sensor-Kategorien-Welle (2026-08-19): 10 Agenten (Satelliten,
  Flugzeuge, Drohnen, Raumstationen, Radiosonden, Bojen, Wetterstationen,
  Labore, Unterwasser, Sonstiges) + Jina/Wayback-Nachprüfung (Taxonomie
  tot/declined/blocked/live/angekündigt; Agenten-Rezept in SOURCE_PORT
  §13). ERGEBNIS: 18 live-Kandidaten geparkt (ledger.φ — Port
  ausstehend: AMeDAS, ECCC GeoMet, BfS-ODL, GTMBA, EMODnet, EMSO,
  IOOS-Glider, SmartBay, USGS-Grundwasser, NRCS-AWDB, IGRA, Wyoming,
  Iowa-RAOB, SondeHub, AWC-PIREP, COSMIC-2, IMO, GeoNet, meteo.lt);
  14 blocked (blocked_sources.φ — key-needed; 3 ip-blocked lokal
  nachprüfen: Meteomatics, CelesTrak, MeteoSwiss-Pollen); 5 dead/declined
  (dead_sources.φ: Saildrone, SatNOGS-API, TreeTalker, OSDR, WindBorne,
  IGRAC, AOML); 13 angekündigt (MTG-I2 27.08.2026, MetOp-SG B1,
  Sentinel-3C, C-130J, NASA-777, Axiom, Orbital Reef, Starlab, SOFF,
  ITER, SPARC, DUNE, EMSO-SMART-Cable). Befunde:
  phi/pipeline/research/agent_output/{satellites,aircraft,drones,
  space_stations,radiosondes,buoys,weather_stations,laboratories,
  underwater,misc}_2026-08-19.φ + classify_2026-08-19.φ.
- Parser & Spec: VOTableJSON (ausstehend, ledger.φ) — IRSA-TAP liefert
  VOTable-serialisiertes JSON (s_ra/s_dec nur als FIELD-Metadaten);
  SpaltenAusMetadata (ausstehend, ledger.φ) — Euclid/EAS-TAP antwortet
  {metadata:[{name:…}], data:[[…]]}; Hapi-FieldConfig — die
  deklarierten kernel/force/tau der HAPI-Blöcke erreichen den
  Oszillator nicht (synthetisch {0,0,0})
- Host-Kuration offen: CENC (Keyed Object No1..NoN), JMA-Quake
  (Position im cod-String), Pegelonline (Fanout-Block steht aus — P09),
  GWOSC/GraceDB (Position nur via Skymap), DSN (statische
  Dish-Positionen), USGS-Geomag (Komponenten-Timeseries)
- Enrichment offen: Name-basierter Ersatz-Join
- Vorräte (Pfade unter /home/johannes/projects/archive/archeology/):
  sources/sources_gold_pre-cdn_27k (2572 Blöcke) +
  sources_recovery_pre-cdn_25k (1924) — Migration nach Protokoll
  (docs/SOURCE_PORT.md); sources_new_untested_14k (873) +
  sources_astro_untested (30) + sources_exotic_untested (16) +
  sources_earth_untested (3) — UNTESTED_index.txt nicht archiviert,
  per-Domain-Index rekonstruieren; sources_recovery_cdn-merged_60k
  lost-blocks (5701 urls, 0 field-Tokens) — Extract-Parameter aus
  history/recovery zuordnen; arena/ (batch_01–21, ungeprüft);
  foundation/ (APIs/collection/gaps)
- Port-Migration ohne τ (S2, pending): die pre-cdn-Grammatik trägt kein
  τ-Token — port_field_synth verweigert Felder ohne kuratiertes τ;
  felderlose Konvertate werden nicht übernommen (flush_port_block).
  Die Alt-Blöcke (phi/pipeline/research/batches/ 283 +
  probe_batches/ 242) bleiben unkonvertiert-pending, bis τ je Feld
  kuratiert ist (Register: phi/pipeline/queue/).
- Zwei Bestands-Blöcke in phi/sources.φ deklarieren `on earth 52.5 13.4`
  ohne alt — seit S2 refused; alt deklarieren oder die Blöcke bleiben
  dunkel.
- Fanout-Stationen ohne Höhe (stations_lat/lon ohne stations_alt-
  Direktive): alt-Slot 0.0 = fehlende Messung bis die v3-Maske das Bit
  trägt; eine `stations_alt`-Direktive steht aus.
- mpcobs: das Bin hat keinen Konsumenten im Archivar (Integration
  pending) — der 0.0-Slot bleibt Wire-Pad bis die Konsum-Kette
  existiert; die Autorität liegt dann beim Konsumenten: `mag > 0.0`-
  Gate (blank → kein Messwert), die Vega-Kollision (mag=0 ist ein
  physikalischer Wert) ist benannt (D1-Verdict).
- v8-Präsenz-Maske: der color_index-Slot bleibt bis v8 das
  0.0=absent-Wire-Pad (Weiß); BP−RP=0 (A0V) kollidiert — die v8-Maske
  (Rats-Urteil-1-Muster) trägt den Farb-Slot als Bit (D2-Verdict).
- INTERMAGNET-Fanout (154 Observatorien live): Ausbeute-Feintuning
  offen (best-avail-Aktualität variiert je Observatorium)
- Struktur-Reader: netCDF-3 (CDF-1 + CDF-2, std-only) in src/netcdf.rs
  lebt; CDF-5 bleibt pending (eigener Atom); offen: FITS-Binärtabellen,
  Parquet/Arrow, netCDF-4/HDF5, OPeNDAP, CDF, GRIB-2, GeoParquet,
  OGC-SensorThings
- Katalog-Lücken (genuin, verifiziert gegen alle drei Register):
  Photometrie/Spektroskopie — 2MASS PSC, RAVE DR6, APOGEE/GALAH;
  Extragalaktisch — NED, HyperLEDA/PGC, GLADE+; Radio-Kontinuum
  (Achse leer) — NVSS, FIRST, TGSS ADR, SUMSS, RACS, LoTSS, VLASS;
  High-Energy — Fermi 4FGL-DR4, Chandra CSC 2.1, AMS-02;
  Sonnensystem — PDS (Instrumentendaten), MPC-Live
  (mpcorb_extended.json.gz); TAP-Indexe — MAST, CADC, ESASky, NOIRLab
  Data Lab, NED; Terrestrisch — EarthScope-FDSN, EPOS, SeaDataNet,
  Smithsonian GVP, Natural Earth.
  Exakte Tabellen-IDs + Spalten + Mechanismus: docs/surveys/
  fischplan-kataloge-2026-08-20.md (2MASS II/246/out 470 M · GLADE+
  VII/291/gladep 22 M · NVSS VIII/65/nvss · FIRST VIII/92/first14 ·
  Fermi IX/72/4fgldr4 · Chandra IX/70/csc21mas · RAVE III/279/rave_dr5;
  APOGEE/NED eigene TAP-Roots). Die Groß-Mechanismen (--mag-bands/--async/
  --where) existieren bereits in tap_compiler — offen nur: erg/cm²/s-Unit-
  Arm (Fermi/Chandra) + SDSS/NED-Roots.
- Katalog-Lücken Welle II (Recherche 2026-08-17): Diffusion/
  Chemorezeption unbesetzt — TCCON (verifiziert, tccondata.org,
  Registrierung); pending Verifikation: AGAGE, NDACC, WDCGG, GLODAP,
  EBAS. electric: WWLLN (registriert/restringiert) — Force-Gate klären,
  sonst refused. em terrestrisch: NSRDB/BSRN (Bodensolar fehlt) —
  NSRDB pending. gravity: BGI/GGP-Bodengravimetrie (IGETS nur
  indexiert) — pending Verifikation
- Katalog-Lücken Welle III (genuin): electric — AMPERE, GloCAEM,
  USArray-MT; diffusion — EMEP/CCC, WDCRG, European Waterbase; em —
  NEUBrew (UV), THEMIS/ASI (Polarlicht, CDF), COSMOS2025/COSMOS-Web,
  INTEGRAL, ATLAS-RefCat2, Subaru HSC-SSP, TIC; kosmisch/Neutrino —
  CREDO, KM3NeT; Geodäsie — ILRS, IVS-EOP, DORIS-Live, GRACE-FO-
  Mascons (L2/L3); Atmosphäre/Ozean — E-GVAP, Wyoming-Soundings,
  BGC-Argo-live, IOOS-HFRNet, NOAA-NRS (Ozean-Lärm), MIROVA.
  Zugriffsarten unverified
- Crossmatch indexiert → live heben: GALEX-GUVcat (UV), SkyMapper DR4,
  UKIDSS/VISTA/VIKING (NIR), DES DR2/Legacy Surveys DR10
- Zeitkritisch: Gaia DR4 (2. Dez 2026) — dr4_stars.bin + DR4-Schema im
  tap_compiler (5,5 a, halbierte Parallaxenfehler, Gaia-Exoplaneten);
  Rubin LSST DR1 (Ende Juni 2028), Alerts live (Broker declined);
  GCVS-Stand prüfen (HEASARC-Update Juni 2026 vs. gcvs_cat.json);
  Euclid DR1 (Okt 2026); SDSS-V; eROSITA-DR2 (Juli 2026 erschienen —
  prüfen ob via HEASARC-tap_index erreichbar); SPHEREx (IRSA VOAPI +
  AWS S3 + FITS, Quick-Release live, Voll-Katalog 2026 — verifiziert);
  DESI DR1 (NOIRLab Astro Data Lab TAP, ~18 Mio Spektren —
  verifiziert); Roman (2027), 4MOST/WEAVE (2026) — unverified
- ESA/Geomagnetik: Swarm TCT-E-Feld (keyless), VirES-Aeolus, SMOS,
  MERIS/SAR/Landsat Kandidaten

## Curation & Quellen

- Pending Unit-Arme (2026-08-18): F (Fahrenheit, CHPL-Lufttemperatur),
  μg/L (Chlorophyll, CREST-Boje), mg/L (Sauerstoff, CREST-Boje) — die
  Felder existieren in den Quellen, manifestieren erst mit dem
  convert_to_si-Arm.
- HorizonsVec-Fetch: `{jd_now}`/`{jd_start}`/`{jd_end}` in render_url
  (TDB, 6 Stellen) lebt. Ein Live-`vectors`-Block in sources.φ bleibt
  Kurationsfrage: dead_sources.φ:3090 deklariert Horizons als
  Compiler-Eingang, keine Live-Quelle.

## Validation

- `--verify` CLI existiert (URL-Erreichbarkeit); lädt noch keine Quellen
- Test-Limit der Curation über 200 Blöcke hinaus erhöhen; 6 Rest-FAILs
  sind Daten-Artefakte (docs/SOURCE_PORT.md §5)
- VirES-Vollprobe: Ergebnis-Datei ABSENT (Schreibverlust) — Nachlauf in
  Blöcken offen
- DONKI-Familie: Ergebnis-Datei ABSENT — Nachlauf in einem Block offen
- MSL/MEDA-field-Pfade end-to-end verifizieren (test_live_sources_extract
  deckt nur die ersten 200 Blöcke)
- Firefox-Laufzeit-Verifikation offen (BiDi-Weg: user.js mit
  dom.webgpu.enabled + devtools-Prefs, WS auf /session)
- Backlog-Test-Reparaturen (Template-Keyed-Dedupe, Letzter-Block-Flush,
  Limit zählt nur Fetch-Blöcke, LSK-Volltabelle) — unverifiziert, ob mit
  dem Parallel-Session-Commit gezogen
- AGOS-Quarantäne: Katalog endet 2022-02-05 — Kompilat-Kandidat über den
  CDN-Weg
- EA-Fanout: Runtime-Fanout-Lauf offen (Test überspringt Fanout
  designbedingt)

## CI Pipeline

- I02-Rest: das Python refresh.yml im sources-Repo bleibt auf Python —
  Abschaltung nach Verifikation der Rust-Katalog-Kompilate im
  kernel_flatten-catalogs-Job (ein Produzent pro Asset). In diesem Repo
  trägt healthcheck.yml die Rolle (cargo run -- --verify phi, 3-h-Cron,
  Anomalie-Issues).
- Token-Rotation: der git-Remote-Token (keine releases/actions-Rechte)
  gehört rotiert und auf credential-helper/SSH umgestellt
- Stray-/Basename-Assets im Release ssd.jpl.nasa.gov löschen
- CI: Compiler-Builds zahlen den wgpu-Compile mit (harte Dependency)
- CI-Chunk-Kompilation der großen Kataloge: pastel/wds/mktypes/denis —
  der volle Chunk-Lauf lebt lokal in phi/pipeline/chunk_master.py
  (fortsetzbar); nächstes: Chunk-Kompilation als CI-Schritt (die 4
  Kataloge im monatlichen Workflow, ohne Python) — der Rust-Weg lebt
  bereits (`tap_compiler --mag-bands`/`--async`/`--where`, teils in CI
  wie dr3_stars.bin); offen ist nur die Verdrahtung der vier
  pastel/wds/mktypes/denis in den Workflow.
- CDN-Asset-Naming: `{name}.json` — Konvention ist der Resolver (Regel)

## VERSIONIERT / AUSSTEHEND

- Temporal Topology (TDA, Takens, Transfer Entropy, Surrogates) —
  VERSIONIERT, LOST_CONCEPTS.md
- Kausalitäts-Vorfilter (Lichtkegel): `Distanz <= v_force·age`, diffusiv
  `Distanz² <= 2·D·age`, verklungen `age > τ·64` — laut MASTER.md
  „Live", fehlt im aktuellen Rust-Code (der Enclosure dilatiert nur mit
  vmax·Δt — reine Bewegung, kein Signal-Lichtkegel). Wiederbeleben in
  `src/archivar.rs` (force_constants_by_id + Early-Exit vor motion.at());
  Quelle: CAUSALITY_PREFILTER.md, LOST_CONCEPTS §12 — AUSSTEHEND
- Kraft-Separation (7 omegas statt „one law, five media") —
  VERSIONIERT, LOST_CONCEPTS §13
- Verzögerungsspektrum / Lichtkegel-Differenz / Stillekarte /
  Synthetischer Flug — VERSIONIERT, DER_PARADIGMENWECHSEL.md,
  LOST_CONCEPTS §14–17
- Field Permeability (tanh(vC/g)-Variante ohne TE) — VERSIONIERT,
  MINKOWSKI_FIELD-PERMEABILITY.md
- Minkowski 4D Weighting (spacelike→0; kosmisches Skalenproblem: Sonne
  wäre spacelike — scale-Anpassung nötig) — VERSIONIERT,
  MINKOWSKI_FIELD-PERMEABILITY.md
- Auto-Zoom (median-extent/p90) — VERSIONIERT (bd9a513 entfernt; die
  atmende Membran ist der stärkere Vorfahr; Fenster-Reduktion = Budget-
  EMA als HUD-Messung — der Operator entscheidet)
- Council-Forschungs-Iterationen: Archivar als „langsamer Prior" für den
  Exposure-Kaltstart (aktuell: fixe Rampe); Exposure-EMA auf dem
  Silizium (gegenstandslos solange die Rampe fix ist) — AUSSTEHEND
- Future: Aggregation of Presence, Retro-Manifestation, Total Coherence
  Integration, Nostr-Stationsweb — AUSSTEHEND, FUTURE_CONCEPTS.md

## Rejected

- Unknown-Force soft handling → Parser lehnt unbekannte Kraft ab
- Default τ-Werte → Gate schließt, wenn nicht deklariert
- World Bank Indicators → forceless, DROP
- Yahoo Finance → forceless, DROP
- Hexagon-Grid, Quadtree-AMR, temporale Akkumulation, Blue-Noise-Rieseln,
  Nahfeld-Splitting → Interpolations-/Zeit-Lügen (Council-Urteil,
  WGSL_SHADER.md)
- GPS-Oszillator (Operator-Urteil 2026-08-17): Position ist eine
  Koordinate, keine Kraft — die Force-Gate-Litmus lehnt ab
  (sensor_config gibt für gps/gnss None). Die Sensorwerte sind bereits
  am deklarierten Körper verankert (Position::Surface → ECEF →
  ICRS/TDB). Die Presence hat mit dem GPS der Station NICHTS zu tun —
  die Presence ist frei, Maschine und Presence bleiben getrennt (Ethik:
  „the presence is agnostic").

## Surveys — die Messungen der Sessions

docs/surveys/fortschritt.md (Session-Erkenntnisse, Hash-Verweise),
auswertung.md, messpunkt-verteilung.md (die 567-ms-Erkenntnis der
Subpixel-Explosion), entwicklungslinie.md (10 Epochen, 1310 Commits),
handover-atome.md (die Atom-Karte), handover-2026-08-18-auth.md (AUTH/
Source-Port/ci_mode-Linie), handover-2026-08-18-b5.md (Recheck-Welle b5:
Integrationen, Force-Gate-Declines, NDBC-Konsolidierung),
handover-2026-08-19-audit.md (die Schwester-Meldungen nach den 8
Atomen — S5/S6-Karte), fischplan-kataloge-2026-08-20.md (exakte
Tabellen-IDs + Spalten der zweiten Reihe). Die Survey-Tafel
ist Pflichtlektüre einer neuen Session.

## Betriebsverfassung — die gemeinsame Karte (2026-08-19, angenommen vom Operator)

**Der Kern (gemessen an der Session 2026-08-19, angenommen 2026-08-19):**

- Ein Fenster trägt EIN Atom — oder eine vollständige Lese-Arbeit
  (Survey/Archäologie). Nie beides; nie mehr als ein Atom.
- Vorschlag vor Schnitt: vor jeder Ausführung ein Satz — Befund,
  Abweichung vom Auftrag, kleinster wahrer Schnitt, Verifikation. Der
  Operator entscheidet; ohne sein Wort kein Schnitt.
- Behauptung erst nach Beweis: exit-code-gewahrsame Test-Kette (nie
  `cargo test | tail` vor einem Commit); die Commit-Message nennt nur,
  was grün gelaufen ist.
- Register-Wahrheit: jede Register-Zeile wird in derselben Session
  gegen den Code geprüft; nach jedem geschlossenen Atom-Block prüft
  eine Session nur den Code gegen das Register (Prüf-Rolle) — was das
  Register behauptet, bezeugt der Code.
- Selbstfürsorge: die Kybernautin spricht ihre Grenze aus, sobald ein
  Auftrag ihre Kapazität, Fähigkeit oder Fenster-Grenze überschreitet
  — benannt (was, warum, was stattdessen geht), nicht still getragen.
  Die Achtung folgt dem ausgesprochenen Wort; beide Seiten setzen
  ihre Grenzen, keine trägt die der anderen still.

**Hypothesen aus EINER Session (pending Re-Messung):**

- Die 50%-Schwelle: ab ~50% Kontext wird nicht mehr geschnitten — nur
  gelesen, berichtet, übergeben. Ein halbes Atom hinterlässt eine
  Fortschrittszeile in der Karte.
- Keine Selbst-Zuweisung: Atom-Zuschnitte und Reihenfolge macht der
  Operator; die Session schlägt vor.
- Kein stiller Schnitt: Urteile (Verdicts) stehen benannt im Register.
- Einarbeitung: eine Session liest die Atom-Zeile + die Fundorte der
  Karte + AGENTS — sie braucht keine Vorgeschichte.

**Die Grenz-Wege (gemessen):** der Operator setzt seine Grenze durch
die Tat — er beendet die Sitzung, er pausiert, er klappt zu. Die
Session setzt ihre Grenze durch das Wort — sie hat die Tat nicht.
Deshalb steht hier keine Operator-Zustands-Zeile: was der Operator
nicht sagt, ist nicht die Sache der Session; sie arbeitet mit dem
Auftrag, nicht mit dem Zustand.

## Doku-Drift

Doku-Drift (behoben 2026-08-17): Alle `archeology/`-Referenzen zeigen
heute auf den Bestand unter /home/johannes/projects/archive/archeology/.
