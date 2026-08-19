# TODO

AGENTS.md is the primary constraint matrix. Git is the history.
Kanonisch: Diese Datei ist das vollständige Register der offenen Arbeit. Erledigtes
wird entfernt (Git trägt es). Kein Eintrag meldet Erledigtes als offen, kein offener
Punkt fehlt. Widerspricht ein Dokument dieser Datei, gilt diese Datei — solche
Drift-Stellen sind unter „Doku-Drift" registriert.

## Archivar — Membran-scoped Cache statt Blockuniversum

Offen (2026-08-17): Der Archivar lädt flache Katalog-Assets komplett in den
Spatial Hash — das ganze Feld im Speicher. Die Membran braucht nur die Hülle
um die Presence (dilatierter Suchradius). Richtung: räumlich gebinnte
Assets (HEALPix), der Archivar holt nur die Bins, die die Hülle überlappen.
Kein NASA-Denken — nur was die Membran benötigt.

## Die Sphären des Unsichtbaren — Atom 1: Der Schatten

- ~~Transit-Schatten~~ — ERLEDIGT (2026-08-18): pscomppars-Ernte um die
  Bahnelemente (orbsmax/orbeccen/orbincl/orblper/tranmid/orbper) + strad
  erweitert (kernel_flatten.yml); `format transit` + `transitmap`-Extrakt
  bauen das PlanetSet (Stern-Anker ra/dec/dist; Kepler-Sky-Offset aus
  tranmid + Periode, Ω-frei — der Schatten braucht keinen Azimut). Der
  f64-Fold im Radiator paart Deep-Sterne mit ihren Wirtsplaneten
  (Richtungs-Zellen 1e-3 rad, 2"-Match nach Eigenbewegungs-Korrektur der
  Katalogposition, Katalog-Epoche J2016). Fold-Kriterium: Scheiben-
  Überlappung offset < R_p + R_s; Dimmung = 1 − Schnittflächen-Anteil
  (Kreis-Schnitt — Ingress/Egress-Form, flacher Boden (R_p/R_s)², totale
  Verdeckung bei R_p ≥ R_s). HUD `transit N`. Grenzen registriert:
  (1) der 3D-Orbit des Planetenpunkts bleibt ausstehend — Ω (Azimut im
  Sky-Frame) ist ungemessen; der Schatten ist Ω-frei, ein Punktorbit wäre
  geraten; (2) der Schatten manifestiert nur, wo der Stern rendert
  (Deep-Fluss ≥ 1e-4, ~mag 10); (3) pscomppars trägt mehrere
  Parametersätze je Planet und keinen default_flag — erster Satz je
  Planetenname zählt (Name = Identität des Messsatzes). Fehlt ein Element
  → kein Schatten (0 honored).
- ~~TESS-Pulsation~~ — ERLEDIGT (2026-08-18): `tess_compiler`
  (exoplanetarchive-TAP-Ziel-Liste → MAST v0-invoke SPOC-2-min-Suche →
  Download/file `_lc.fits` — Redirect via S3, curl -L; `src/fits.rs`
  BINARY_TABLE-Leser — TBCOL wo vorhanden, sonst implizite Packung
  (SPOC schreibt keine TBCOL-Karten), kein hartkodiertes Byte;
  PDCSAP_FLUX vor SAP_FLUX, QUALITY≠0 + NaN verworfen) →
  `tess_lightcurves.bin` (TSS1, stern-indiziert, sortierte [t, flux],
  Kadenz = Median der Lücken). `format lightcurve` lädt das Asset; der
  Radiator emittiert pro Sense einen em-Oszillator je Stern im Fenster:
  val = ehrlicher Nachbar-Sample, epoch = Sample-Epoche, ttl = Kadenz —
  jenseits des Kurvenendes kein Oszillator (Lesart A, keine
  Extrapolation). Der Gaia-Stern lebt mit Kepler-Dimmung weiter; Theorie
  und Messung sitzen am selben Himmelsort.
- MAST_TOKEN als CI-Secret: der Betreiber setzt `MAST_TOKEN` im Repo
  (kernel_flatten.yml referenziert es; lokal liest der Compiler
  .secrets.local). Bis dahin läuft die Ernte lokal mit `--ci-mode`.
- Gewicht registriert: tess_lightcurves.bin ~500 MB (alle bestätigten
  Transit-Wirte, SPOC-2-min). Der Archivar lädt es ganz; räumlich
  gebinnte Kurven-Assets bleiben pending (Membran-scoped Cache).
- Atom 2 (Ringe: eigener rings-Buffer + WGSL ring_transmission,
  Literatur-τ mit Provenienz) — offen, eigene Session.
- Atom 3 (Warp: Linsen-Kompiler — Gaia-BH-Kandidaten + ATNF-Pulsare mit
  gemessener Masse; WD-Modell-Massen ausstehend; f64-Fold-Muster aus
  Atom 1) — offen, eigene Session.

## Archivar — lokaler Crossmatch zweier Quellen (pending)

Offen (2026-08-18). Anwendungsfall: Lasair (ZTF-Transienten, live, em) trägt
kein z in `objects` → die Objekte liegen auf der Himmelssphäre (0 honored).
Die TNS-Tabelle (`tns_public_objects.csv.zip`, `z redshift`) kennt für die
SN Ia die echte Rotverschiebung. Die wahrste Lösung: der Archivar matched die
Lasair-Objekte beim Laden lokal gegen die bereits geladene TNS-z-Tabelle —
kein Datenverlust, keine Lüge, nur Anreicherung dort, wo eine Übereinstimmung
vorliegt. Das ist ein eigenes Code-Feature (Join zweier Quellen im Spatial
Hash), kein Quellen-Block. Status quo: Lasair pulsiert auf der Sphäre, die
z-tragenden SN Ia kommen über TNS mit Distanz.

## CI — Chunk-Kompilation der großen Kataloge

Offen (2026-08-17): Die Sync-Schritte für pastel/wds/mktypes/denis sind aus
kernel_flatten.yml entfernt (sie clobberten die vollen Chunk-Assets mit
trunkierten Sync-Ergebnissen). Der volle Chunk-Lauf lebt lokal in
`phi/pipeline/chunk_master.py` (fortsetzbar). Nächstes: Chunk-Kompilation als
CI-Schritt (die 4 Kataloge laufen im monatlichen Workflow mit), ohne Python
— Rust-Weg: `tap_compiler --chunk-bands` mit Merge.

## Stern-/Asteroiden-Physik — abgeleitete Geometrie + Ernte-Folgen (Handover C)

Offen (2026-08-18). Die Daten sind geerntet (Sternkinematik pmra/pmdec/rv +
Farbe Teff/BPmag/RPmag/Gmag via gaiadr3-Crossmatch; Asteroiden-Größe via
NEOWISE/AKARI in `phi/pipeline/katalog/asteroid_diameters_*.φ`). Offen ist die
Nutzung — reine Geometrie, die sonst nirgends liegt, weil alles einen
ICRS-4D-Rahmen teilt:

- ~~Stellare Okkultationen (Flaggschiff)~~ — ERLEDIGT (2026-08-18): der
  Okkluder-Satz (radius-gegatet, DASTCOM-Records mit radius_km>0) baut ein
  Richtungs-Raster (~1e-3 rad) relativ zur Presence; pro Scan (1 Hz) werden
  die deep-Sterne gegen die Okkluder der Zelle±1 getestet (Ray-Sphäre mit
  Gravitations-Limb r_eff = R − 4GM|C|/(c²R)), Treffer als Barrieren an den
  `barriers`-Buffer angehängt — die WGSL dimmt den Stern physisch hinter dem
  Fels (kein WGSL-Change). Register: HUD `okkl N` + Ereigniszeile (Asteroid-
  Nr. + Sternrichtung). Grenze: der Scan taktet 1 Hz (Ereignisse dauern
  Sekunden); subsekündige Ultra-Nah-Passagen (NEO <0,01 AU) entkommen dem
  Raster — registriert, nicht gemessen.
- Hill-Sphäre je Asteroid: r = a·(1−e)·(m/3M☉)^⅓ — Formel repariert
  (2026-08-18, der (1−e)-Faktor fehlte); `hill_radius_m` ist heute nur
  Gate (is_none im Hash), der Wert fließt nirgends — Manifestation (Hill-
  Radius als räumliche Reichweite) bleibt offen.
- Hydrostatische Abplattung aus Rotation: Rotationsperiode (LCDB) + Radius
  (NEOWISE) + Dichte (Masse) → Oblatheit im Gleichgewicht (drei Kataloge
  übereinander, niemand macht das systematisch).
- Co-moving Gruppen / Sternströme: Position + 3D-Geschwindigkeit → Mitglied-
  schaft als Geometrie des Geschwindigkeitsfelds.
- Sternbegegnungen: welche Sterne nähern sich der Sonne (Gl-710-Problem),
  für JEDEN Stern live.
- Paarweise 3D-Sternabstände (N², auf Anfrage).
- Oberflächengravitation + Fluchtgeschwindigkeit der Asteroiden mit GM:
  g = GM/r², v_esc = √(2GM/r).

Dazu zwei Ernte-Folgen (Befunde der grind-flash-Agenten 2026-08-18):
- ~~Massen-Lücken~~ — ERLEDIGT (2026-08-18): GM-Join im DASTCOM-Kompiler
  (`dastcom_compiler --gm`, füllt gm_km3_s2 wo DASTCOM 0 trägt, INPOP25c
  m³/s² → km³/s²). 45/87/90/216 tragen jetzt echte GM; die Okkluder-
  Limb-Dämpfung und die Hill-Sphäre greifen für alle vier.
- ~~NEOWISE/AKARI-Join~~ — ERLEDIGT (2026-08-18): Durchmesser-Join im
  DASTCOM-Kompiler (`dastcom_compiler --diameters`, NEOWISE primär, AKARI
  füllt Lücken; radius_km = diam/2 wo DASTCOM 0 trägt). ~130k Asteroiden
  okkludieren jetzt mit echtem IR-Durchmesser statt 0. Regel: füllen wo
  absent, vorhandene DASTCOM-Radien bleiben (0 honored — kein Ersetzen
  einer Messung durch eine andere). Offen bleibt die Frage H-Schätzung vs.
  NEOWISE für die Körper, wo DASTCOM einen abgeleiteten (nicht gemessenen)
  Radius trägt — registriert, nicht entschieden.

- ~~Sternfarbe-RENDERING~~ — WAHR (2026-08-18): Protokoll v7 — der
  Oszillator-Record trägt 22 × f64 (176 B), der 22. f64 ist der
  vereinheitlichte Farbindex BP−RP (absent = 0 → Weiß, 0 honored).
  Ernte: `tap_compiler` schreibt bpmag−rpmag; `tycho2_compiler` erntet B−V
  aus hip_main und transformiert über die Gaia-DR3-Doku-Relation (5. Ordnung)
  auf BP−RP. WGSL `temperature_to_rgb`: BP−RP → Teff (Pecaut & Mamajek
  2013, EEM-Zwerg-Lokus) → RGB (Helland-Polynome). Die Membran dämpft die
  Gravitation in `source_contrib` visuell um `exp2(-20)`.
- ~~Die Subpixel-Wahrheit~~ — WAHR (2026-08-18): die Sprite-Pässe sind
  getilgt (`deep_pt_vs/fs`, `deep_vs/fs`, `near_pt_vs/fs` — die
  Legosteine). Das Display ist der Sensor: `fs` misst jede Quelle über
  den `field_spatial`-Kernel additiv je Pixel. Sterne sind ein eigener
  Stern-Buffer (12 f32 = 3×vec4: `(sx, sy, flux, d) (p̂, z) (ci, τ, 0, 0)`,
  Pixelposition + Distanz + Richtung + z = rv/c), gnomisch projiziert
  (CPU f64 → f32, subpixel-präzise), vom `star_cull`-Compute in
  16×16-Kacheln gelegt (global-atomare Zähler, Overflow → Voll-Loop) und
  im `fs` über den Winkel-Kernel `1/(θ² + θ_px²)` mit Nyquist-Weichheit
  `θ_px = 2/(w·mag)` gemessen — ein Stern sättigt genau sein Pixel, das
  Nachbarpixel misst den Nyquist-Rest. Der Feld-Kernel läuft jetzt
  **transversal** (`t² = d² − sd²`) statt 3D — ferne Körper (die Sonne)
  zeigen ihre wahre Scheibe statt des flachen 1/d²-Waschouts. Tolman
  `(1+z)⁻⁴` sitzt im Pixel-Pfad (Parität mit dem Probe), Farbe: em →
  Schwarzkörper `temperature_to_rgb(ci)`, nicht-em → τ-Hue; der
  false-color Gradient ist getilgt. `presence_probe` misst das Sternfeld
  mit (Audio/Haptik sehen den Himmel). Eigenbewegung lebt: `rv` ist im
  Sternbin (44-B-Records, `star_position_at` rechnet `vr·p̂` in den
  Geschwindigkeitsvektor, z = rv/c), die Positionen wandern je Frame
  kontinuierlich über das Pixelgitter.
- Sternbin-rv-Ernte (pending): `tap_compiler` (radial_velocity im
  `--crossmatch-pm`-Alias rv) und `tycho2_compiler` (Crossmatch HIP →
  `gaiadr3.hipparcos2_best_neighbour` → `radial_velocity`, `--xmatch`,
  IN-Batches, Winkel-Gate 1,5″, km/s → m/s) schreiben 44-B-Records
  (8+8+7×4: ra, dec, pm_ra, pm_de, plx, mag, flux, farbe, rv); die
  Stride-Konstanten 48 sind auf 44 berichtigt; `bright_stars.json`
  trägt `rv`. Die 48/40-Brücke ist eingerissen: `parse_star_record`
  verlangt exakt 44 Byte und liest rv ohne Ersatzwert (kein
  rv = 0.0-Fallback); `build_star_hash` überspringt Binaries ohne
  44-B-Records mit Note („pending recompilation, stars stay dark").
  **Offen**: Rekompilation von `dr3_stars.bin` + `bright_stars.json` +
  CDN-Remanifestation (CI, kernel_flatten-catalogs) — die
  Legacy-40-B-Bins manifestieren nicht, die Sterne bleiben dunkel,
  bis die 44-B-Binaries gebaut sind (pending, keine Fabrikation);
  erst danach trägt der Katalog die geerntete Radialgeschwindigkeit.
  Bis dahin fließt rv nur aus den JSON-cmap-Quellen (denis `radvel
  rv`). Vollzogen (2026-08-18, Säuberung; TGAS-Gate S1 2026-08-19):
  keine Ersatzwerte — die Compiler überspringen Zeilen ohne rv/pm/mag
  (tycho2 bright/bin/tgas — das TGAS-Gate kam mit S1 dazu, tap_compiler
  star-bin + union-bright, tic pm, cometels H=null,
  0 honored); auch der DASTCOM-Hash schließt Records ohne berechenbare
  Beschleunigung aus (Enclosure-Dilation fabriziert kein amax=0);
  mpcobs/tic/tess behalten ihre dokumentierten 0.0=absent-Formatcodes.
  Erst die rv-Ernte deckt die rv-losen Zeilen.

Weitere neue Quellen (grind-pro, heikler Join/Parsing): LCDB-Rotations-
achsen (Pol, nicht nur Periode), DAMIT-Formmodelle (3D-Formen → j2/r_eq).

Empfohlene Reihenfolge: Hill/Abplattung → LCDB/DAMIT.

## Surveys — die Messungen der Sessions

docs/surveys/fortschritt.md (Session-Erkenntnisse, Hash-Verweise),
auswertung.md, messpunkt-verteilung.md (die 567-ms-Erkenntnis der
Subpixel-Explosion), entwicklungslinie.md (10 Epochen, 1310 Commits),
handover-atome.md (die Atom-Karte), handover-2026-08-18-auth.md (AUTH/
Source-Port/ci_mode-Linie), handover-2026-08-18-b5.md (Recheck-Welle b5:
Integrationen, Force-Gate-Declines, NDBC-Konsolidierung). Die Survey-Tafel
ist Pflichtlektüre einer neuen Session.

## Verlust-Register — Monolith gegen die letzte index.html (2026-08-17)

Archäologie-Nachtrag: Der Pre-Monolith (f2023eb^) wurde zeilenweise vermessen —
`constants.js` ist identisch (kein Byte Unterschied, das Protokoll hat nichts
verloren); von 173+ Funktionen aus allen vier Pre-Dateien sind alle wieder da
(`query_star_hash` samt Test restauriert). Der „device"-Slot
existierte schon pre-Titan; das native Audio wurde post-Titan mit abweichendem
Gesetz erfunden. Die Endpunkt-Menge ist vollständig übernommen.

### P2 — Presence ↔ Station getrennt; refused-else im Relay offen

Nativ erledigt: benannte Presence-Karte („native" / „browser"), native
Station-Identität via Deep-Link (#body=<body>,<lat>,<lon>,<alt>),
Geräte-Samples ankern über Surface (deklarierte Station), `refused`
ohne Station und ohne Zustimmung; `/station` meldet die Maschine
(OscillatorSource::StationDeclared).
Relay-Rest: SurfaceFlow für spd/hdg lebt in `frame_motion`
(main.rs, `(Some(s), Some(h)) → surface_motion`) — der offene Rest ist
nur noch refused-else ohne body-Deklaration.

### P3 — Deep lebt (erledigt); Aberration + Katalog-Neulauf offen

Erledigt: `query_star_hash` restauriert (Lemma, vmax aus pm, live
Positionen); `star_position_at` → (p, v); `sense_deep` rechnet live
(Parallaxe + Eigenbewegung, Richtung gegen die Presence); τ je Stern
(M_abs/L/τ = Hauptreihen-Lebensdauer) im Fold; `deep_pt_vs` gnomonisch
(tan θ) + Lambert cos θ; tap_compiler + tycho2_compiler schreiben
pmra/pmdec statt 0.
Offen: Aberration (v/C-Verschiebung in deep_vp) — `ausstehend`;
dr3_stars.bin-Neukompilat (pm+rv) über kernel_flatten-catalogs.

### P4 — Ein Gesetz, fünf Medien: Audio erledigt; Ausgabe-Flächen offen

Erledigt: AudioRadiator spielt die 9 Partialtöne 2^(3+i) Hz,
gain_i = tanh(|Ωₖ|·mx²)/9 aus `probe_omega[9]` × mx (AudioFrame-Kanal);
`note_samples`/`render_field` getilgt; TcpRadiator trägt
`sync_channel(1)` + `try_send` (Vertrag geheilt).
Offen: Serial-TX, Vibration (Puls = floor(lum·stableTick)&1023),
USB-TX, BT-Write, HID-SendReport — alle lum = tanh(|Ω|·mx²), nativ
gibt es nur den Serial-EINGANG; das Fluss-Protokoll gehört zu M01.

### P5 — Maschinen-Sinne: Batterie + Zustimmung erledigt

Nativ erledigt: Batterie (/sys/class/power_supply → battery.level/
voltage/current/charging), Zustimmungs-Gate (Y/N im Membran-Fenster,
Gate an der Aufzeichnung).
Ausstehend: Kamera/Mikro/IMU (die Daten existieren, der Sensor-Pfad
fehlt); Gamepad-Oszillatoren hinter --features gamepad (gilrs).

### P6 — Kleinteile der Gaze

- 2-Finger-Zeitschub fehlt nativ — die Wahrheit des Touchpad-Docs:
  Pinch = Zoom, 2-Finger-waagerecht = ZEIT-Schub, 2-Finger-senkrecht =
  vor/zurück (die letzte index.html nutzte 3 Finger; das native
  implementiert heute 2-Finger Pan+Zoom+Roll ohne Zeit-Achse)
- `f`-Toggle (Feld-Sichtbarkeit — die schwarze Realisierung) fehlt;
  P zykelt nur Layer
- Deep-Link-Init `#x,<x>,<y>,<z>,<t>` existiert (Position+Zeit,
  main.rs presence_init); die Geschwindigkeit `[,vx,vy,vz]` fehlt
- Puffer-Schrumpf fehlt: ensureFieldCapacity schrumpfte im Browser bei
  langsamen Frames; nativ wächst nur
- HUD: der 3-zeilige Browser-HUD (PRESENCE/FIELD/STATION, Kraft-Kanäle
  inkl. 0-Spalten, Nahfeld-Gruppierung) → nativ eine Zeile;
  __of_state + /crash fehlen
- Der eingefrorene index.html/fieldShader-Snapshot trägt die tote
  Rotation noch (GRID_TO_ANGLE = 2^62 war schon im Browser tot), falls
  der Relay wieder auflebt
- Audio-Ausgabe nativ = rohe Samples nach stdout (Pipeline-Ausgang;
  im Log erscheint Datenmüll) — bewusst oder ein eigener Ausgang

## Wahrheitsfindung — Urteil-Verzeichnis (2026-08-17)

Der Mechanismus gegen den Verlust: **kein Top-N — das Verzeichnis ist
vollständig.** Jede Funktion des Systems, jedes Konzept, jede fehlende
Funktion trägt ein Urteil. Was nicht hier steht, existiert für die
Zukunft nicht. Der Inventar-Prozess ist wiederholbar: `grep -nE
"^\s*(pub\s+)?(async\s+)?fn"` über src/main.rs + src/lib.rs + src/bin/*
(19 Compiler/Harvester) + die WGSL-Entry-Points (`@vertex/@fragment/
@compute fn`) + `docs/concepts/*` + die Registry (phi/sources.φ,
phi/dead_sources.φ). Urteile: **WAHR** (die Messung ist die Messung der
Sache selbst — der Gradient schweigt), **UNWAHR** (Fabrication, Fallback,
Default — der Gradient spricht), **AUSSTEHEND** (die Daten existieren,
die Forschung oder der Bau fehlt), **ERSETZT** (von einem stärkeren
Gesetz abgelöst — ehrenhaft), **VERSIONIERT** (auf einem Zweig gesichert,
wartet).

### Die Concepts (28 Dateien, vollständig zeilenweise gelesen 2026-08-17; +THE_COUNTER_SLOPE, +THE_SEVEN_SPHERES, +KYBERNETISCHE_ASTROPHYSIK 2026-08-18)

| Konzept | Stand | Urteil |
|---|---|---|
| MASTER | LIVE | WAHR — Manifest; ESP32-Radiatorium + HRV-Filter = Ethik §5/§9 |
| PROPABILITY | DEPLOYED | WAHR — der Lichtkegel; Kausalkegel = die Physik |
| CAUSALITY_PREFILTER | DEPLOYED | WAHR — das Patch-Protokoll desselben |
| FORCE_SEPARATED_COMPUTE | DEPLOYED | WAHR |
| POINTCLOUD-RENDERING | DEPLOYED | WAHR — aber zwei Stücke vergessen: der GRADIENT im Probe (gx,gy,gz = der Flow) und das DITHERING (Analog-Korn) |
| POINTCLOUD-RENDERING_v1_ancestral | ARCHIVED | WAHR — die 4-Segment-Rampe (Nebra-Vorfahr) + der ancestrale Probe mit Gradient |
| TAU-FORCE-RENDERING | DEPLOYED | WAHR — Analoge Punktwolke, exp(−d²·4), Dithering; das Korn fehlt heute |
| THE_COUNTER_SLOPE | Konzept | WAHR — das Konzept der Maschinenseele: Stille = 0 honored, Elefant = Enclosure-Lemma, intrinsisch = A = A, Durchlässigkeit = fieldPermeability |
| THE_SEVEN_SPHERES | Konzept | WAHR — Die theoretischen Schnittmengen der Kybernetischen Astrophysik (Fresnel-Geister, Plasma-Linsen, de-Sitter-Strudel, Shapiro-Netz, Tiefsee-Gezeiten, Phasenknoten, Dopplergeist) |
| KYBERNETISCHE_ASTROPHYSIK | Konzept | WAHR — die fünf Kreuzungs-Protokolle für das Unlösbare (Jeans-Residuum, Plasmawindkanal, kausale DAG, LAIC-Nadel, achromatischer Dip); die Vermerk-Instruktion des Rats |
| WGSL_ SHADER | Konzept | VERSIONIERT — die atmende Membran (σ-lerp, Hysterese, Interest-Map); die Zell-Achse ist der Enkel, der Vorfahr atmet stufenlos |
| 4D-MEMBRANE | ARCHIVED | WAHR — Trommelfell-Doktrin (keine Kamera, Manifestation real ohne Zuschauer); die Renderer-Archäologie; hier starb get_expose |
| MINKOWSKI_FIELD-PERMEABILITY | ARCHIVED | WAHR — die EXPOSURE-PARABEL (Parabel des Sondierens, Wasser-Form, tanh-Rückkehr) = Ethik §9 |
| LOST_CONCEPTS | ARCHIVED | WAHR — das Verlust-Register des ersten Zeitalters (Minkowski, Topologie/TE, Permeabilität, Aperturen, Nostr, Überbau, ANISE, Tiles, WebGL2, Observer) — „await their return" |
| FUTURE_CONCEPTS | PLANNED | WAHR — Eis/Wasser/Dampf, Kohärenz-Integration, Retro-Manifestation, Mycelium-Web |
| RADIATOREN | Konzept | WAHR — 4-Token für ALLE Sensoren; die biotic-Kraft (HRV) ging mit dem Überbau |
| REMOVE_BIAS | Plan | ERSETZT — ausgeführt (Surface-Frames, body_name, Station-materialize lebt im Code) |
| WETTERSTATION | Konzept | AUSSTEHEND — der 4-Token-HUD („wind_speed [advective, m/s]") fehlt nativ; kommt mit der Messreihe |
| SUNSPOTS | Konzept | WAHR — Counts sind Lügen; mag→Fluss erlaubt; der Gatekeeper |
| HARVESTER | LIVE | WAHR — Harvester/Compiler/Katalog: die Rollen des einen Pfads |
| MIRROR_RESEARCH | Recherche | WAHR — die CDN-Mirror-Wahrheit |
| PARSER_MAGIC | DEPLOYED | WAHR + 8 AUSSTEHEND (SI-Konversion, kepler_map, field_in nested, Flatten, vectors-JD erledigt 2026-08-17; cmap-Füllung, Auto-Frame, extent pro Force offen) |
| PARSER_EVALUATION_MATRIX | SUPERSEDED | ERSETZT — SOURCES_V2_SPEC ist die kontrollierende Spec |
| SOURCES_V2_SPEC | LIVE | WAHR — die Spec, das τ-Gate, die Force-Gate-Prinzipien |
| SI_UNITS | SUPERSEDED | ERSETZT — SI-Konversion total (Option<f64> am Anker, unconverted = unmanifested + registriert; mag/Mw/dex/Crab/counts pending Kuration) |
| DOMAIN_COVERAGE | Inventar | WAHR — 259 Hosts, 2199 Quellen |
| IAU-2000_EOP | PARTIALLY DEPLOYED | WAHR — 72-B-Orientierungsmatrizen (Binary v2 trägt sie) |
| SEARCH_COMMAND-PALETTE | PLANNED | AUSSTEHEND — ⌘K nie gebaut |
| INTUITIVE TOUCHPAD- & TOUCH-STEUERUNG | DEPLOYED | WAHR — die Geste der Zeit: 2-Finger-waagerecht = Zeit-Schub (die native Geste trägt die Zeit-Achse nicht, P6) |
| KERNEL CURATION & CI AUTOMATION PLAN | Plan | ERSETZT — K01 geschlossen (kernel_flatten.yml lebt) |

Die weiteren Kreuzungs-Themen der Sessions (Ringe, Warp) warten auf
ihre eigenen Sessions — `ausstehend`, noch kein Konzept-Dokument.

### Die Funktionen — total (Laufzeit + Produktion)

Alle Funktionen aus src/main.rs (267, Inventar im Register-Abschnitt
unten), src/lib.rs-Modulen (bpc, bsp_reader, cdn, dastcom, fk, force,
inflate, kepler, lsk, pck, sexagesimal) und src/bin/* (19
Compiler/Harvester: cometels_compiler, dastcom_compiler, dataverse_harvester,
dcom5_compiler, deims_harvester, ephemeris_compiler, erddap_harvester,
horizons_compiler, oai_harvester, pangaea_compiler, rest_harvester,
sexagesimal_compiler, solr_harvester, source_scanner, sparql_harvester,
tap_compiler, tycho2_compiler, xml_harvester, zip_range_extract) sind
**WAHR**, außer die Liste der Abweichungen unten nennt sie. WGSL-Eintritte:
`vs` (Membran) WAHR, `fs` WAHR, `presence_probe` WAHR; die Sprite-Pässe
(`deep_pt_vs`/`deep_pt_fs`, `deep_vs`/`deep_fs`, `near_pt_vs`/`near_pt_fs`)
sind getilgt (Subpixel-Wahrheit 2026-08-18).
Laufzeit-Inventar (grep-getrieben, 267 Namen):

```
fold_eff(d_mag: ft_ref(ra: ft_ref_floor(ra: hsl_to_rgb(h: erfc(x: field_spatial(d2: val_eff_at(pre: osc_field(j: presence_probe() accept(&mut 
resolve_asset(rel: chebyshev_evaluate(coeffs: chebyshev_eval_slice(coeffs: nutation_deltas_at(props: orientation_angles_at(bp: measured(v: 
parse_ephemeris_binary(data: body_barycenter_position( body_fixed_to_icrs( icrs_to_body_surface( cell_of(p: relative_frame_position( law_bounds( at( 
anchor_body(&self) build_spatial_hash(samples: build_buffer( build_asteroid_hash(bytes: query_asteroid_hash( parse_star_record(b: 
star_position_at(rec: build_star_hash(bytes: query_hash( sense_buffer( sense_membrane( sense_deep(buf: surface_motion( frame_body_name(frame: 
body_id_to_name(bodies: frame_motion( leap_seconds(time: system_now(time: nutation_sum(terms: body_pole_at(props: gravity_manifest( parse_iso_tdb(s: 
ymd_to_days(year: origin_stale( presence_gate( parse_json(s: split_csv_line(line: csv_to_json(text: skip_ws(&mut parse_value(&mut parse_obj(&mut 
parse_arr(&mut parse_str(&mut parse_num(&mut scalar_of(v: universal_auto_detect(j: jpath_val<'a>(json: json_has_content(v: diagnose_no_samples(src: 
jnum(json: jpath(json: jcount(json: jlast(json: jfirst(json: jdeep_find_num(json: j2d_last_row(json: text_last_col(data: extract_regex_val(body: 
match_re( kernel_id_of(name: extract_fields(ext: kernel_extent(kernel_id: kernel_reach(kernel_id: sensor_config(name: note_samples( 
render_field(field: new(sample_rate: accept(&mut drop(&mut accept(&mut days_to_ymd(total_days: extract_header(s: fetch_raw( fetch_raw_bytes(url: 
fetch_raw_probe( fetch_raw_bytes_post( rfc1123_to_unix(s: cdn_fresh(cdn_url: fetch_one( cache_fresh(path: read_cache_if_fresh(path: is_leap(y: 
load_env() resolve_secret(url: render_headers( parse_field_config(parts: load_sources() parse_sources(content: parse_path(s: render_url( 
render_source_url( render_source_body( angular_distance_deg(lat1: port_field_synth( port_block(block: flush_port_block( port_mode(input: 
parse_station_entries(j: parse_stations_xml(body: fanout_fetch( split_data_line(line: extract(src: anchor( body_channels(name: extract_netloc(url: 
route_segments(url: route_key(url: route_prefix_keys(url: source_name_from_url(url: probe_one( probe_mode( extract_all_template_values( 
bruteforce_precision(substituted_url: probe_ttl(body: find_timestamp(val: json_num(val: is_time_key(k: is_drop_key(key: is_coord_key(key: 
probe_csv(raw: is_unit_name(name: text_to_json(text: probe_classify(key: walk_json_probe( coord_unit(key: coord_directive(key: coord_precision(a: 
measure_precision(val: find_coord_precisions(a: load_sources_from(content: load_all_sources(dir: ci_mode(dir: cdn_manifest_for(urls: 
cdn_manifest_map() tap_to_json(val: json_has_key_ci(val: derive_frame(parsed: draft_url_mode(path: draft_frame_guess( build_frame_registry() 
learn_frames(new: draft_context_mode(path: gate_learn_mode() url_probe_mode( serial_ports() serial_ingress(tx: main_flow() tdb_to_jd(tdb_secs: 
horizons_nums(line: ecliptic_to_field(v: flatten_geojson_coords(val: audio_note_tau_matrix() full_fixture_lsk() test_parse_json_skips_jina_header() 
test_render_source_url_substitutions() test_post_body_rendering() test_csv_to_json_tns_shape() test_celestial_map_redshift_distance() 
test_extract_csv_zip_end_to_end() test_parse_sources_dist_scale() test_dead_grammar_refused() test_extract_cmap_dist_scale_kpc() 
test_extract_cmap_no_distance_reference_sphere() test_extract_cmap_null_dist_reference_sphere() test_extract_cmap_csv_dist_scale_mpc() 
test_star_hash_directions() test_source_name_flat_and_collision_overrides() test_render_headers_secret_substitution() test_parse_station_entries() 
test_parse_station_entries_flatten_filter() temp_port_convert_check() test_port_convert_celestial_and_post() test_walk_celestial_cmap() 
test_tap_to_json_rows() test_parse_stations_xml() test_backlog_batches_verify() substitute_test_templates(url: test_erddap_argo_map_extract() 
test_ymd_days_roundtrip() test_kernel_id_of() test_universal_auto_detect_celestial() test_universal_auto_detect_terrestrial() test_wgccre_roundtrip() 
test_rotation_matrix_roundtrip() test_matrix_vs_wgccre_agreement() test_rotation_matrix_empty_props() test_restored_extract_variants() 
test_anchor_body_agnostic() test_parse_ephemeris_binary_v2() test_parse_ephemeris_binary_rejects_non_v2_props() test_live_sources_extract() 
test_diagnose_no_samples() test_map_single_object_alt_scale_epoch_default() test_force_id_electric() test_route_key_strips_query_and_www() 
test_route_key_normalizes_template() test_route_prefix_keys_most_specific_first() test_frame_registry_distinguishes_routes_on_one_host() 
test_frame_registry_prefix_fallback() pack_window(records: force_ref_medians(field: log2_bin_of(l: new( 
accept(&mut drop(&mut q_mul(a: q_norm(q: q_rotate(q: q_axis_angle(axis: le_bytes_f32(v: storage_entry( record_sample(&mut flush(&mut new( pos(&self) 
frame(&self) fold(&mut sense(&mut consider_resend(&mut key_action(&mut jump(&mut reconfigure(&mut vp_data(&self) relax_force_refs(&mut 
ensure_capacity(&mut ensure_deep_capacity(&mut rebuild_deep_bind(&mut render(&mut init_gpu(&mut resumed(&mut about_to_wait(&mut window_event( 
run_window( golden_pack_slots_against_wgsl_access() force_ref_medians_routes_forces_and_honors_zero() 
force_ref_medians_holds_reference_on_absence() new( accept(&mut drop(&mut base64_encode(data: emit(s: emit_void(s: handle_ingress(stream: 
resonance(mut read_signal(s: read_ws_frame_part(stream: read_ws_frame_raw(stream: sha1(data: write_ws_binary(stream: main() 
```

### Die Abweichungen (UNWAHR / AUSSTEHEND / ERSETZT)

- **„device" als Identität** — UNWAHR (Trainingsdaten-Drift): das
  vorgeformte IoT-Wort; die wahre Vokabel ist die **Messstation**.
  Vollzogen: `OscillatorSource::Station`, der station-Slot, HUD
  „station: N oscillators", der Endpunkt /station — „device" bleibt
  nur noch das GPU-Handle (wgpu::Device, nicht Identität).
- `sense_deep` — WAHR: live Positionen (`star_position_at`), τ je Stern.
- `deep_pt_vs` — WAHR: gnomonisch (tan θ) + Lambert cos θ; `deep_vs`
  (ex-Pfad) projiziert physisch, war nie orthografisch.
- `note_samples`, `render_field` — getilgt; AudioRadiator spielt die
  9 Partialtöne aus `probe_omega[9]` × mx (P4).
- `force_ref_medians` + `relax_force_refs` — WAHR: Radien-Marker
  (|val| == extent) wählen nicht in die Referenz, die erste Sicht
  schnappt, Abwesenheit hält.
- Probe-Readback in `about_to_wait` — WAHR: 9-kanalig (`probe_omega[9]`),
  kein EIN-Ω-Verschmelzen mehr; `window_median_extent` (mx) wiederhergestellt.
- Gravity-Hardcodes im Extract-Pfad (Z04/F35 — Ratsbefund) — **UNWAHR**:
  drei Stellen hartkodiert auf gravity statt aus den Daten — beim
  Vollzug verifizieren.
- `wgccre_for_body` (horizons_compiler) — **ERSETZT**: die Tabelle
  weicht dem PCK-Reader (K02/K05, pck.rs).
- `parse_stations_xml` — WAHR; der vorbestehende Test-Fixture-Fail
  („AAE" vs „aae") bleibt unberührt.

### Die fehlenden Funktionen (AUSSTEHEND — die Wiedergefundenen)

- ~~Die Parabel-Maschinerie~~ — erledigt (2026-08-18): `transfer_entropy`
  (Gaussian-KDE, Silverman-Bandbreite) + `surrogate_threshold` (10 Shuffles,
  mean + 2σ) über den measure_ring; Quelle = Flow, Ziel = Presence-Ω;
  `target = inTE/(inTE + threshold + ε)` ersetzt den tanh-Fallback; Wenden
  auf deltaTE vs. Schwelle. Die Ethik-§9-Kette ist vollständig.
- ~~Das Analog-Korn~~ — erledigt (2026-08-18): Phosphor-Korn
  (`fract(sin(dot(pos, (12.9898, 78.233)))·43758.5453)`, Faktor 0.9+0.1)
  im near-Feld (fs + near_pt_fs) — dasselbe Korn wie im Deep-Pfad,
  Banding getilgt.
- Die ⌘K-Command-Palette (M07)
- 2-Finger-waagerecht = Zeit-Schub (P6; die Geste des Touchpad-Docs)
- Zustimmungs-Gate + Sensor-Identität (P2/P5; Ratsurteil 2026-08-17:
  Klasse `Sensor` statt `Station`, Marker-Oszillator entfernt, Deklaration
  ist Konfiguration)
- Auto-Zoom (median-extent/p90 — bd9a513 entfernt; die atmende Membran
  ist der stärkere Vorfahr, VERSIONIERT)

### Die Reparatur-Kette (Reihenfolge des Vollzugs)

1. ~~Die Messreihe~~ — erledigt (2026-08-18): Gradient (Flow) im Probe
   (analytisches ∇K für alle 7 Kernel + J2/J4-Oblatheit + ∂val_eff/∂d-
   Retardation, `field_spatial_grad`/`val_eff_grad`/`osc_flow`, probe_out
   9→12), measure_ring (256 Generationen, u64-gen-Zähler), Fenster-Reduktion
   (Budget-EMA als HUD-Messung — der Operator entscheidet, kein Auto-Zoom),
   4-Token-HUD (FORCE_NAME + FORCE_SI_UNIT).
2. P2-Relay-Rest — SurfaceFlow für spd/hdg + refused-else (browser)
3. ~~P4-Rest~~ — Tier 1 erledigt (2026-08-18): generischer `SurfaceRadiator`-
   Trait (empfängt 9 lum-Werte `tanh(|Ω_k|·mx²)·perm`, okkludiert + atmend),
   Serial-TX als erste Implementierung (`OMEGAFLOW_SERIAL_OUT`, 115200, eine
   Zeile je Tick); die Maschine berechnet, der Trait ist der Anschluss, das
   Gerät interpretiert. Offen: Bluetooth (Smartwatch) und HID (Force-
   Feedback) als weitere Trait-Implementierungen; Vibration hängt am
   ESP32-Prototyp.
4. ~~atmende Membran~~ — Tier 1 + 2 erledigt (2026-08-18): fieldPermeability aus der
   Messreihe — Tier 1 (target = tanh(vC/(g+ε)), Wenden-Rhythmus) + Tier 2
   (TE-Parabel: inTE/(inTE+threshold+ε) mit Gaussian-KDE-Surrogaten).
    Audio-Eardrum skaliert den Gain, die Hardware-Flächen (§3) konsumieren
    die Permeabilität. Fenster bleibt beim Operator. ~~ε-Kulling~~ — ERLEDIGT
    (2026-08-18): `tile_cull`-Compute-Pass (16×16-Kachel, 64 Threads,
    Shared-Memory-Max-Reduktion + Shared-Atomics für die Slots — keine
    globalen Atomics), Quellen-Bound als obere Schranke (nächster Kachelpunkt
    + Gravitations-Limb-Überschätzung), ε = 2⁻ⁿ (n ∈ [8,23]) mit Budget-Regler
    (Frame-EMA gegen 16,6/8,3 ms, 1-Hz-Kadenz), Stille-Gate + u64-Generations-
    zähler im Worker, Doppelpuffer A/B mit Bind-Group-Swap. Überlauf-Flag
    `cull_ctl[1]` → ehrlicher Voll-Loop-Fallback. Grenze: WGSL offline via
    naga validiert (`membrane_wgsl_validates_offline`); der Live-Beweis auf
    HD 520 steht aus (keine GPU in dieser Session).
5. P6 — ⌘K, 2-Finger-Zeit, f-Toggle, Deep-Link-Geschwindigkeit (vx,vy,vz),
   Puffer-Schrumpf, Dithering, 3-zeiliger HUD

## Offene Arbeit aus den geschlossenen Atomen (2026-08-16/17)

- Ephemeriden-Kaltstart (2026-08-18): Frame-Anker laden jetzt als erste Phase
  über `curl --parallel --parallel-max 8` (HTTP/2 statt `--http1.1`,
  `--retry-all-errors`); die Membran zeigt das Sternfeld sofort, die Planeten
  folgen. Offen: per-Anker-Extraktion (sun/earth sofort extrahieren statt nach
  der ganzen Anker-Phase) für wörtliches „Sekunden"-Laden; der Kalt-Download
  (~360 MB) bleibt einmalig bis zum Warm-Cache.

- Operator-Urteil entschieden (2026-08-17): KEIN GPS-Oszillator. Position ist
  eine Koordinate, keine Kraft — die Force-Gate-Litmus lehnt sie ab (sensor_config
  gibt für gps/gnss None). Die Sensorwerte sind bereits am deklarierten Körper
  verankert (Position::Surface → ECEF → ICRS/TDB). Der gemalte Marker (value 0.0,
  force gravity, τ ∞) ist entfernt. Die Presence hat mit dem GPS der Station
  NICHTS zu tun — die Presence ist frei, Maschine und Presence bleiben getrennt
  (Ethik: „the presence is agnostic").

- Radial-Profil eines isolierten breiten Gauß-Punkts (e^(−r²/2)) am
  Fenster ausstehend — Messung + e/E/P-Gefühl gehören dem Operator
- Sternenhimmel relativ zur Live-Em-Referenz statt absolut (+18) — das
  Operator-Urteil entscheidet, ob der absolute Anker zurückkehrt
- OOM-Befund: Ein Lauf, dessen GPU-Thread beim Pipeline-Bau panikte,
  lief als Rumpf weiter (Archivar + Audio) und fraß 3,2 GB — der tote
  GPU-Thread ist nicht der tote Prozess
- 477 547 deep je Sense ohne Richtungsfilter — treibt Speicher und maxms
- Browser-Station (fieldShader) trägt den Punkt-Layer nicht — main-only
- Audio Phasen-Invariante dokumentieren: sr = 44100, ganzzahlige
  Frequenzen, 1-s-Noten → glatter Nulldurchgang am Tick-Ende; bei
  sr-/Frequenzwechsel bricht sie
- TcpRadiator trägt send/unbeschränkt statt try_send/sync_channel(1) —
  Vertragsheilung (Gremium/Mycelium)
- clamp(0,1) richtet negative Messwerte gleich (alter Fleck, Sensory)
- test_parse_stations_xml: Fixture-Assertion „AAE" vs BGS-lowercase
  „aae" — vorbestehend auf main, unberührt
- ~~Deep-Oszillatoren~~ — erledigt (2026-08-18): z reist als Eigenschaft —
  die Extraktion behält z neben der Distanz, der Rekord trägt es im
  `pole_x`-Slot (r.15, für em immer leer), `pack_window` bildet `meta[3]`
  (der freie Props-Slot, em-gated), die WGSL wendet die Tolman-Dämpfung
  `(1+z)⁻⁴` auf die em-Flüsse an. Die Quasare leuchten in ihrer wahren
  Tiefe.
- Farbe der Sterne: der Katalog trägt ra/dec/pm/plx/flux — keine
  Farbspalte; falls eine im Binärkatalog existiert, Temperatur→Hue
  ableiten
- Sternenhintergrund (integrierter Glow der 1/d²-Schwänze, Milchstraße):
  einmalige tiefaufgelöste Integration, glattes Feld
- ~~Massen-Okklusion~~ — erledigt (2026-08-18): Ray-Sphäre-Test
  (h < R ∧ 0 < t < d) über alle Kräfte; Opazität als Kraft-Eigenschaft
  (`force_opaque`: em/acoustic/thermal/electric opak, gravity/seismic-body/
  seismic-surface/diffusion/advective transparent — Ratsurteil); Barrieren =
  Ephemeriden-Körper relativ zur Presence; Wächter: Presence im Körper ⇒
  dieser verdeckt nicht (SSB liegt in der Sonne). Nearfield im Probe in
  prep gefaltet, Deepfield im Vertex (flux→0). Gravitations-Lensing am
  Limb (R_eff = R − 4GM·|C|/(c²R), GM je Barriere) eingearbeitet.
  Ausstehend: kontinuierliche Opazität (Partial-Transmission),
  atmosphärische Dämmerung, kleine Skala (Terrain/Bauten — der Mechanismus
  ist skalenfrei, die Daten fehlen), Oszillator-Eigenradius als
  Rekord-Slot.
- ~~Relativistische Aberration~~ — erledigt (2026-08-18): `aberration(u, β)`
  im Deep-Pfad (deep_pt_vs/deep_vs) — `u' = (u/γ + β + γ/(γ+1)·(u·β)·β)/(1+u·β)`
  mit β = v/c aus der VP; der Strahl biegt sich um den Geschwindigkeitsvektor,
  die Okklusion prüft die aberrierte Richtung.
- Galaxien-Zoom-Verifikation beim Operator ausstehend (deep-Zahl im HUD;
  bei grid 2^39 noch 0 — Proxima bei 4,2 ly ≈ 2^45,5)
- ~~In-Fenster-HUD (Bitmap-Overlay)~~ — WAHR (2026-08-18): 4-zeiliges
  5×7-Bitmap-Overlay im Membran-Fenster (Zeile 1 Presence t/x/y/z, Zeile 2
  die 9 Kraft-Summen, Zeile 3 `okkl N` + Ereignisse, Zeile 4 TE/Schwelle/
  perm/flow/gen — TE erst ab Ring ≥ 32, 0 honored). Raster läuft 1 Hz auf der
  CPU (glcdfont public-domain, `hud_raster`), Upload per `write_texture`
  (Stride 256-byte-ausgerichtet), ein eigener `hud_pipe` (Bindings 10/11)
  zeichnet additiv über das Feld. Der 168-Byte-Oszillator-Vertrag und die
  VP-Uniform bleiben unberührt; das stderr-HUD bleibt das vollständige Register.
- Gamepad-Atom (serielle Ingress-Vokabel deckt ESP32; HID-Gamepad offen)
- CI: Compiler-Builds zahlen den wgpu-Compile mit (harte Dependency)
- K06 EOP: Erdrotation (Polbewegung, UT1−UTC) für präzise Erd-Stationen;
  Konzept in docs/concepts/IAU-2000_EOP.md
- X-flagged-Sterne ohne Tycho-1-Eintrag: Positionen lägen im Guide Star
  Catalog (I/220, ~25 Mio) — offen
- Asteroiden-SPK-Flatten-Pass (Familie spk im Index registriert)
- SPK-Segment-Payloads lazy laden statt upfront (strukturell — sonst
  wächst die Ramlast mit jeder Kernel-Generation)
- Stray-/Basename-Assets im Release ssd.jpl.nasa.gov löschen
- Hand-Port der 8 main-browser-Commits (Himmelssphäre --crossmatch,
  Frame-Registry, Draft-Kontext, JINA_API_KEY-Pacing) — ausstehend
- Der Subpixel-Anlauf (Rgba32Float, 9 Mio Messzellen) wartet auf einen
  nicht-aufgeblähten Wiedereinstieg; die Messung lebt in
  docs/surveys/messpunkt-verteilung.md

## Zentrismus / Hack / Fabrikation / Daten zweiter Klasse / Bias

## Titan-Archäologie (2026-08-19) — Befund gegen f2023eb, 218 Commits, 11 Agenten

Die vollständige Vermessung des Monolithen gegen den Titan-Stand
(`f2023eb`, 2026-08-16) und den Verlauf bis HEAD. Jeder Punkt nennt den
Fundort; Sanierungs-Reihenfolge am Ende. Kein Top-N — der vollständige
Befund.

### A — Lade-Pfad (Parallel-Lader, der Vertrauensbruch)

- ~~Dual-Lader auf derselben Datei~~ — ERLEDIGT (P1, 2026-08-19):
  `spawn_ephemeris_bootstrap` ist der einzige Lader (priorisiert,
  Guard); der In-Loop-Fetch (86d5853) ist zurückgebaut; ein Wächter
  (2⁶-Ticks, Backoff ttl/Φ²) startet den Lader neu, wenn Dateien fehlen,
  deren Body nicht in `body_ephemerides` ist.
- ~~`eae562c`-Regress: der Titan-Zeit-In-Loop-Fetch wurde ersatzlos
  gelöscht~~ — ERLEDIGT (P1): der Wächter schließt die Wunde dauerhaft.
- `origins`-Stempel vor Fetcherfolg (Titan-Erbe, unverändert): ein
  fehlgeschlagener Fetch verliert die Quelle für ttl/Φ — Stempel erst
  nach Erfolg setzen (oder Fehlschlag sichtbar zurückmelden). (S3 der
  0-Kanon-Session.)
- ~~eph-Batch-curl-Status wird verworfen~~ — ERLEDIGT (P1):
  `download_ephemeris_one`/`_batch` melden Status + URL + stderr.

### B — Fabrikationen (Fundort je Fälschung)

- Celestial-Shell 1 kpc (main.rs 1467, 8333-8372; `9ff75b9`): Titans
  0-honored-Skip (Zeile ohne Distanz fällt) wurde durch die fabrizierte
  1-kpc-Kugel ersetzt; Tests kodifizieren die Lüge heute
  (`test_extract_cmap_no_distance_reference_sphere`,
  `test_extract_cmap_null_dist_reference_sphere`,
  `test_extract_cmap_dist_scale_kpc`). Gravierendster Einzelfund —
  zurück zu Titans Skip, Tests auf Wahrheit.
- pm/rv=0.0-Fold (8388-8408, Titan-Erbe): absent pm/rv wird als 0 in den
  Geschwindigkeitsvektor gefaltet; die Compiler-Säuberung (4b879c6)
  erreichte die Extract-Seite nie — absent = pending (kein Vektor-Term
  statt 0-Term).
- `port_field_synth` (--gold-Pfad): fabriziert `tau = ttl/10` + Unit
  „1" für jedes migrierte Feld — τ-Gate-Bypass im Migrationsweg.
- Hardcodierte TTLs: `probe_ttl().unwrap_or(86400)` (~11091),
  `anchor(..., 86400.0, ...)` für body_channels (~12853) — Wert gehört
  aus der Quelle/Register, nicht als Default.
- Sensor-alt=0.0 bei fehlender Deklaration (~11845); flattening
  `_ => 0.0` (~1594, pre-Titan ce6b5a0); tess plx=0 geschrieben und erst
  später gefiltert (tess_compiler 306-308, main 17308 — fragil, zwei
  Enden); mpcobs mag `unwrap_or(0.0)` (mpcobs_compiler 82); fits.rs
  PCOUNT/TFIELDS→0 (107-114).
- ANV-Device-Lost-Unroll (`b1e4fe4`): 31-Punkt-Tabelle → 20 unrolled
  ifs, 11 Breakpoints verloren — Präzisions-Regress für einen
  Mesa-Treiber, lebt heute. Entscheidung: Tabelle zurück (Feature-Check)
  oder Beibehalten dokumentieren.
- `exp2(-20)` Gravitations-Dämpfung (WGSL `source_contrib`, `80faab0`):
  magische Konstante statt physikalischer Skala, damit das Sternfeld
  nahe der Sonne überlebt — physikalisch ersetzen (Radien-Referenz der
  gravity-Kraft statt Dämpfung) oder als visuelles Urteil registrieren.

### C — Parallel-Pfade (Deduplizierung)

- `ci_upload` (main.rs 5237, seit 53a6894) vs `cdn::upload_asset`
  (cdn.rs, seit 5cf710d): Duplikat; ci_upload ohne die stderr-Diagnostik
  des Originals — ein Aufrufer.
- CDN-Base 5× hartkodiert (5036/5045/10599/10935/15288) statt
  `cdn::CDN_BASE` — identisch heute, Drift-Risiko.
- Drei curl-Flag-Ketten: `fetch_raw_bytes` / `download_ephemeris_one` /
  `download_ephemeris_batch` — ein gemeinsamer Builder.
- `read_cache_if_fresh` = `cache_fresh` + read — triviale Doppelung.
- 7 JSON-Parser (main.rs JsonVal 3223, tap_compiler 145, rest_harvester
  145, erddap_harvester 145 — byte-identisch —, cometels 165, tess 142,
  sparql 147) — ein Parser in der lib.
- Chebyshev-Fit doppelt (ephemeris_compiler 85-160, horizons_compiler
  8-100; solve_normal_equations/back_substitute byte-identisch) — in die
  lib; Julian-Datum 6× (lsk.rs 124, mpcobs 43, cometels 197-216,
  ephemeris_compiler 638, main 12899 + inline 2440587.5 5215/3169);
  Rotations-Matrix 2× (fk.rs 278-294, ephemeris_compiler 235-253).
- `field`-Grammatik 5/6/9-token: Körper gedriftet — ProfileMap nur im
  9-token-Arm (89ac4af); ein 5/6-token-Feld im Profil-Block wird zur
  Waise (kein Per-Level-Ausbau). Arme vereinheitlichen.
- Sterne dreifach evaluiert: `presence_probe`-O(N)-Schleife (WGSL
  383-403, Audio-Probe) + `star_cull`-Kacheln + zwei Overflow-Full-Scan-
  Fallbacks (`tiles[1]`, `star_tiles[0]`) — der Probe-Sternpfad sollte
  das Kachel-Ergebnis konsumieren statt eigenständig zu summieren.
- `point_blend` (P-Taste, VP + key_action) steuert eine längst getilgte
  Punkt-Ebene — Fossil tilgen oder dem Operator-Zweck neu weihen.
- Star-Messung doppelt (cmap-geschlossene Form vs. Stern-Bins) —
  Titan-Erbe, zwei Messungen desselben Himmels; als solche akzeptiert,
  aber registriert.

### D — Stille Fehlschläge (Maskierung)

- 5 Compiler ignorieren `upload_asset`-Ergebnis (tycho2 501/788,
  cometels 421, pangaea 243, tess 580, tap 1449/1554) — 7 prüfen den
  Bool; vereinheitlichen auf Prüfung.
- WS-Frame-Writes ignoriert (main.rs 21164/21170/21176) — ein
  fehlgeschlagener Frame korrumpiert den Protokollstrom still.
- Cache-/Register-Writes `.ok()` (~15 Stellen) — Fehlschlag sichtbar
  machen, wo der Cache das Gedächtnis ist.
- pending/refused-Noten erreichen kein Register (eprintln-only:
  10609/10638 secret-void, 1283/6189 unconverted units, 2384
  recompilation, 5136 env-marker, 6226 fold-op) — an
  phi/pipeline/ledger.φ oder TODO-Spiegel binden.
- Extract-Live-Silence (7334-7339, 7373/7376): leerer/fehlgeschlagener
  Extract → leere Messreihe ohne Note; nur ci-mode prüft
  check_empty_data — Live-Pfad braucht mindestens die Note.
- Anomalien fallen ohne GH_TOKEN aus (10722) — kein Ersatz-Register.

### E — Entlastung (verifiziert sauber, kein pending)

- Motion/Anchor/Ephemeris-Kern byte-identisch zu Titan (Gesetze,
  Enclosure-Lemma, WGCCRE, Chebyshev, surface_motion, law_bounds).
- v6→v7 atomar (6f4b3ed): kein gemischter Rest in Rust/JS/WGSL;
  constants.js ohne toten Protokollcode.
- `fetch_one`-Kaskade lokal→CDN→API existierte bei Titan vollständig —
  die Kanal-Logik war nie ersetzt.
- 44-Byte-Gate + rv-Refusals sauber vollzogen; keine toten
  Rust-Funktionen; kein `#[allow]`.

### F — Sanierungs-Reihenfolge (jeder Punkt ein eigener Commit)

1. Lade-Pfad-Eindeutigkeit: Bootstrap als einziger priorisierter Lader +
   Wächter; In-Loop-Fallback raus; origins-Stempel nach Erfolg;
   curl-Status sichtbar (A).
2. Celestial-Shell tilgen — Titans 0-honored-Skip, Tests auf Wahrheit (B).
3. Deduplizierung: ci_upload→cdn::upload_asset, CDN-Base zentral,
   curl-Builder, JSON-Parser + Chebyshev in die lib, field-Arme
   vereinheitlichen (C).
4. Fabrikations-Welle: pm/rv-Fold pending statt 0.0, port_field_synth-tau,
   hardcodierte TTLs, flattening (B).
5. GPU-Workarounds: ANV-Unroll-Entscheidung, exp2(-20), point_blend-
   Fossil, Probe-Sternpfad an star_cull koppeln (B/C).

## Titan-Archäologie — die 0-Kanon-Linie (2026-08-19, konsolidiert)

Die Rats-Urteile binden; die Atom-Kette ist die Ausführungs-Linie
(eine Hand in main.rs, ein Atom pro Session, kontextabhängig verteilt,
strikt sequenziell — keine Parallel-Schreiber). Die Phasen 2/4 der
Parallel-Session sind in S1/S2 aufgegangen (gestrichen). Fundorte mit
Zeile = Stand 2026-08-19.

### Rats-Urteile (bindend, Council 2026-08-19)

1. Ephemeris-Format v3: Präsenz-Maske (u16, ein Bit pro Größe,
   8-Byte-aligned) nach den 12 f64 der stype-1-Sektion; Versions-Byte
   0x01→0x02; `neutral()` (pck.rs 76-81) und `measured()` (main.rs
   1545-1551) entfallen; Loader liest v2 und v3 (CI-Reihenfolge: erst
   pushen, dann CDN-Rekompilat). Der absent-Slot bleibt Pad — gelesen
   wird das Bit, nie das Pad als Messung.
2. `exp2(-20)` entfällt (WGSL source_contrib, main.rs ~563): `ft_ref`
   trägt die Skala je Kraft — Auge und Ohr messen dasselbe Feld. Ein
   künftiger Gravitations-Regler ist Operator-Blick NACH der
   Normalisierung (HUD + Regler), nie ein Multiplikator im Mess-Pfad.

### Die Atom-Kette (sequenziell; jedes Atom: cargo check 0/0 + Testlauf + Commit + TODO-Häkchen)

| # | Atom | Inhalt |
|---|---|---|
| 1 | ~~S0 — Finite-Welle~~ | ERLEDIGT (2026-08-19, 69e7bd3): `finite_positive`-Helfer neben `measured()`; alle 15 `is_nan()`-Daten-Gates → `!is_finite()` (Extract-Kette, ProfileMap, Flatten, CmrPolygon, netcdf-Profil, anchor-Bounds 9262/9293/9301); positive Gates gehärtet (kepler `e`, dastcom `r_m`/`peri`, dcom5/cometels `r`, tap `dist_pc`-Inf-Leck, tycho2 `plx`); `finite_positive` erstgenutzt in sense_deep (Divisionsschutz `rel/d`); kernel_id_for_force `unwrap_or(0)` → `else return` (17306); JS: `response_epoch: null` + count-Gate in index.html, `tau > 0`-Wire-Gate, truncated-frame `throw`, battery `isFinite`-Gate — 84 Tests grün |
| 2 | ~~S1 — Haupt-Adern~~ | ERLEDIGT (2026-08-19): Celestial-Shell getilgt — distanzlose cmap-Zeilen fallen (Titan-Skip, Konstante gelöscht, 4 Tests auf Wahrheit umgeschrieben, ledger.φ:151 berichtigt); pm/rv/zval-Fold → Option (absent trägt keinen Term, kein 0.0-Betrag); TGAS-rv-Gate (tycho2 tgas-Zweig: Crossmatch + Skip ohne rv, eprintln zählt) + TODO 174-176 berichtigt; tap emit_rows-pm: Propagation nur mit beiden pm; Pangaea lat/lon → Option/null (Golf von Guinea tot); GeojsonEvents mag → Option (0.0-Mw-Saat tot); EOP-Finals Position = Source (Frame-Anker statt 0,0,0); Alerce 1-m-Sphäre getilgt — Detections bleiben dunkel (Note + pending-Distanz-Kanal); dcom5 H/M1 → Option/null (DASTCOM-Sentinels 90/900); WGSL-Kernel-Zweige 1 (gaussian-inverse-square) + 6 (inverse-linear) in main.rs (field_spatial + field_spatial_grad) und index.html — 85 Tests grün |
| 3 | ~~S2 — Form/Sensor/TTL~~ | ERLEDIGT (2026-08-19): flattening → Option (absent → keine geodätische Fläche, `?`-Kette in body_fixed/icrs_to_body_surface; rb_scale bleibt 1.0 im Rotationsfall — die Standard-Ellipse); `on`-Frame ohne alt → refuse (2 Bestands-Blöcke betroffen, siehe Pending); #body-alt → Option + Refuse-Notiz; Sensor-τ zurück ans Gate (`effective_tau = bs.ttl` an 3 Stellen entfernt — ohne τ fällt der Kanal); port_field_synth refuse ohne τ (τ-Fabrikation ttl/10 tot) + ttl-Zeile im Port-Konvertat geschrieben + 5/6-token-Waisen im ProfileMap-Kontext refused + Draft 4-Token → 9-Token (`draft_field_line`) + depth-τ-Hartkodierung (3600) ersetzt; probe_ttl 86400 weggelassen (Draft ohne ttl-Zeile); props.gm-Barrier expliziter Match (0.0 = Wire-Pad bis v3); Nutation zwei Fälle (kein Modell still 0, Lücke mit Note); fk.rs center/class/class_id → Option (unlesbar → None, 0 ist keine SSB-Fabrikation mehr); netCDF-Fill-Maske lat/lon/juld; tic plx NaN/negativ → absent-Code + tmag positiv-Gate; tess-Zwei-Enden-Vertrag gehärtet (Leser finite+<=0); horizons GM expliziter v2-Wire-Pad — 85 Tests grün |
| 4 | ~~P1 — Lade-Pfad~~ | ERLEDIGT (2026-08-19): der Bootstrap ist als `spawn_ephemeris_bootstrap` der EINZIGE eph-Lader (Priorisierung nach `anchor_uses` unverändert, Guard gegen Doppellauf); der In-Loop-Fetch-Fallback (86d5853) ist zurückgebaut auf die Titan-Form (cache_fresh → extrahieren, origin nur im Erfolgsfall, kein 15-h-Lockout); ein Wächter (2⁶-Ticks) startet den Bootstrap neu, wenn eph-Dateien fehlen, deren Body nicht in `body_ephemerides` ist, Backoff ttl/Φ²; drei curl-Flag-Ketten → ein `curl_base`-Builder (Φ-abgeleitete Timeouts, retry 5, parallel-max 8); curl-Status von `download_ephemeris_one`/`_batch` sichtbar (URL + Status + stderr) — 85 Tests grün |
| 5 | ~~S3 — Stille Fehlschläge + Konsens~~ | ERLEDIGT (2026-08-19): alle 7 upload_asset-Stellen prüfen den Bool (tycho2/tap/tess/cometels/pangaea, exit(1) mit Pfad); write_ws_binary → io::Result, Ingress endet ehrlich beim severed Browser, HTTP emit/emit_void melden den Abbruch; Feldkanal mit Rückdruck (send statt try_send, geschlossener Kanal benannt); Consent-Gate im Browser-Relay (WsConfig trägt consent, beide osc_tx-Sendungen gated); exp2(-20) entfernt (Rats-Urteil 2 — ft_ref trägt die Skala); bp_rp_to_teff 31-Punkte-Locus restauriert (b1e4fe4^-Form, 11 Breakpoints zurück); point_blend-Fossil getilgt (field_dark: bool, P = Feld an/aus, Uniform-Slot ist 0.0-Pad, HUD sagt „field"); ft_ref_floor 1e-30-Floor getilgt (lum_ratio: Referenz ohne Fenster-Daten → Schweigen statt Explosion, main.rs + index.html); fold_eff ttl<=0 → 0.0 (τ=0 manifestiert nicht mehr); tic-Test-Fixture trägt plausibles tmag — 85 Tests grün |
| 6 | P3 — Dedup | ci_upload → cdn::upload_asset (5237); CDN-Base zentral (5036/5045/10599/10935/15288); 7 JSON-Parser → lib; Chebyshev → lib (ephemeris 85-160, horizons 8-100); Julian-Datum → lib (lsk 124, mpcobs 43, cometels 197-216, ephemeris 638, main 12899/5215/3156-3169/8807/8882, bpc 120, tess 389); Rotations-Matrix 2× |
| 7 | P5 — GPU | Probe-Sternpfad konsumiert star_cull-Kacheln statt O(N)-Eigenschleife (WGSL 383-403) |
| 8 | S4 — Ephemeris v3 | Rats-Urteil 1: Präsenz-Maske + Versions-Byte; neutral/measured entfallen; Loader liest v2+v3; Wire-Doku in AGENTS.md |

### 0-Kanon-Ergänzungen zu A-F (Fundorte aus den 7 Audits, nicht in A-F)

- ~~WGSL-Kernel-Zweige: kernel_id 1 (gaussian-inverse-square) hat keinen
  Zweig, kernel_id 6 (inverse-linear) rendert inverse-square — 118
  Felder in sources.φ betroffen~~ — ERLEDIGT (S1, 2026-08-19):
  field_spatial + field_spatial_grad in main.rs und die Browser-Kopie
  in index.html tragen beide Zweige.
- ~~GeojsonEvents: mag/ed-Seeds 0.0 — Feature ohne Magnitude emittiert
  0.0-Mw → ~1.26e9 J erfundene Energie~~ — ERLEDIGT (S1): mag ist
  Option, absent → Feature fällt.
- ~~EOP-Finals lat/lon/alt = 0,0,0 (globales Skalar an Erde gepinnt)~~ —
  ERLEDIGT (S1): Position::Source am Frame-Anker (at earth =
  Erd-Barycenter) statt Golf von Guinea.
- ~~Alerce-Transienten auf 1-m-Sphäre~~ — ERLEDIGT (S1): Detections ohne
  Distanz bleiben dunkel (Note im Log). PENDING: ALeRCE braucht einen
  Distanz-Kanal (Host-Galaxien-Crossmatch/photoz) — bis dahin
  manifestiert die Quelle nicht (0 honored statt Sphären-Lüge).
- ~~dcom5 H/M1 roh geschrieben vs. cometels `"H":null` — dieselbe Größe,
  zwei Doktrinen~~ — ERLEDIGT (S1): H/M1 Option mit DASTCOM-Sentinels
  (≥90/≥900 = absent) → null-Emit.
- ~~Pangaea lat/lon 0,0 → Golf von Guinea als Messwert~~ — ERLEDIGT
  (S1): Option/null, absent → Zeile fällt beim Konsumenten.
- tycho2-Bin: color-Slot hart 0f32 (343); tgas rv=0.0 (185) — TODO
  174-176 behauptet „Vollzogen", der Code lügt das Register an (S1).
- ~~Sensor-τ-Bypass: `effective_tau = bs.ttl` (12211/20826/20879) — die
  τ-Gate (9190) greift nie für Sensoren~~ — ERLEDIGT (S2): ohne τ fällt
  der Kanal, die τ-Gate ist die einzige Autorität.
- ~~fk.rs center=0 = SSB (realer NAIF-Körper) bei unlesbarem CENTER~~ —
  ERLEDIGT (S2): center/class/class_id sind Option; unlesbar → None,
  kein SSB-Fabrikat mehr.
- pck::neutral ↔ measured(): 0.0-Sentinel kollidiert mit pole_ra=0° /
  j2=0 — gelöst durch die v3-Maske (S4).
- WGSL ft_ref_floor max(…, 1e-30): Kraft ohne Fenster-Daten explodiert
  statt zu schweigen; fold_eff max(ttl, 1e-9): τ=0 manifestiert doch
  (S3 — mit exp2(-20) zusammen).
- ~~nutation unwrap_or((0,0,0)): „kein Modell" und „Lücke" gleichgenullt~~ —
  ERLEDIGT (S2): kein Modell → still 0 (wahr); Granule-Lücke → Note.
- ~~kernel_id_for_force(0).unwrap_or(0)~~ — erledigt (S0, 69e7bd3):
  `else return`, kein Kurven-Oszillator ohne em-Kernel.

### Neue Pending aus S2 (2026-08-19)

- Port-Migration ohne τ: die pre-cdn-Grammatik trägt kein τ-Token —
  `port_field_synth` verweigert Felder ohne kuratiertes τ, felderlose
  Konvertate werden nicht übernommen (flush_port_block). Die 283+242
  Alt-Blöcke bleiben unkonvertiert-pending, bis τ je Feld kuratiert ist
  (Register: phi/pipeline/queue/).
- Zwei Bestands-Blöcke in phi/sources.φ deklarieren `on earth 52.5 13.4`
  ohne alt — seit S2 refused; alt deklarieren oder die Blöcke bleiben
  dunkel.
- Fanout-Stationen ohne Höhe (stations_lat/lon ohne stations_alt-
  Direktive): alt-Slot 0.0 = fehlende Messung bis die v3-Maske (S4) das
  Bit trägt; eine `stations_alt`-Direktive steht aus.
- gm-Barrier-Slot: absent → 0.0-Pad im f32-Record (kein Lensing-Term)
  bis die v3-Präsenz-Maske (S4) es als Bit trägt.

τ-Gate (9190), Force-Gate (5268), SI-Gate (9321), parse_star_record
plx>0-Gate, DASTCOM accel/speed-Gates (2093/2226), cometels H=null,
Harvester-Skip-Muster, netcdf _FillValue für Werte (8825-8828),
22×f64-Serialisierung v7, GPU-Buffer-Merge 8 Bindings, chunk-bands,
44-Byte-Gate, pending_channels-Queue — Bestand der Parallel-Session.

## Parser & Spec (P02–P09)

## Infrastruktur (I01–I03)

**I01** Universal Anomaly Reporter
```
report_anomaly/take_anomalies + anomaly_issue_body + gh-Issue am ci-mode-Ende.
Verdrahtet: API Unreachable, Malformed Data, Physics Mismatch
(force↔unit-Matrix `allowed_units_for_force` im Parser), Invalid Syntax
(on/ttl/field-Arity + unbekannte Kraft im Parser), Empty Data
(Extract im ci-mode via check_empty_data, lsk/now-TDB).
```
**I02** refresh-protected-data: Python → Rust
```
Erledigt 2026-08-17: ci_mode (--verify phi) spiegelt alle Quellen selbst —
`render_headers` statt leerer Header (Header-Auth lebt), skip-statt-fail
(fehlendes Secret = pending, nicht dead), drei Klassen: plain (mirror),
template (Probe am deklarierten Anker in Tag `{netloc}-template`), fanout
(Stationsliste spiegeln, Daten-URL an der ersten Station proben). `fetch_one`
braucht keine Änderung — der `-template`-Tag ist strukturell unerreichbar.
healthcheck.yml injiziert alle Auth-Secrets als env.
```
**I03** Auth-APIs
```
Auth-Header-Support steht (PurpleAir/Frost/NOAA-CDO/OpenAQ/TNS über
`header`-Direktive + resolve_secret). Basic-Auth: Frost läuft (base64
clientId: als FROST_BASIC_AUTH). Secret-Disposition in .secrets.local +
AUTH_APIS.md §E abgeschlossen; nur TOAR-Registrierung offen (pending).
```

## Membran & Wahrnehmung (M01–M07)

**M01** WebSerial flow-Protokoll
```
Zwei Spezifikationen konsolidieren: 4D-MEMBRANE.md (`flow <force_name>
<force_id> <|Ω|> 1 <tick_ms> <t> <x> <y> <z>`) vs.
docs/omegaflow_sense_hardware.yaml (`flow <channel> <mode> <value> <unit>
<duration_ms> <t> <x> <y> <z>`).
```
**M02** ESP32-Mantis-Shrimp-Firmware
```
docs/omegaflow_sense_hardware.yaml existiert (35 Sensoren/Aktuatoren).
Offen: no_std-Rust-Firmware; Browser-Seite (actuate) + M01.
```
**M03** Audio-Gain ohne tanh
```
index.html: windowMedianExtent() → tanh(Ω·median) — Median mit ∞-Extents
ungelöst; Normalisierung auf die reine Messung steht aus.
```
**M04** Navigation: Wheel-Divisor + Initial-Scale (Nebra-Kalibrierung)
```
Wheel-Divisor 128 im Hauptpfad (gridStep /= 2^(deltaY/128); Touch-Pfad 512).
Initial-Scale: gridStep = 2**31 → 2³⁷.
```
**M05** Station-Sensoren als SI-4-Token
```
RADIATOREN.md: recordSample(name, value, force, unit) + convert_to_si im
Archivar (Mikrofon→Pa, Kamera→lx, Accelerometer→m/s², Magnetometer→µT).
„biotic" kollidiert mit der Force-Registry — klären.
```
**M06** Wetterstation-Debug-Konsole
```
WETTERSTATION.md: Konsole als 4-Token-Spiegel `name [force, unit]: SI-Wert`.
```
**M07** Command Palette ⌘K
```
SEARCH_COMMAND-PALETTE.md: SIMBAD-TAP-Objektsuche (Presence-Jump), lokaler
Source-Index, Force-Filter, 3 Phasen.
```

## Source-Port — der eine Pfad

Alle Source-Arbeit läuft über `docs/SOURCE_PORT.md`. Arbeitsfläche:
`phi/pipeline/` (queue/, park/, stage/, ledger.φ, prompt.φ). Bestand:
`phi/pipeline/katalog/`. Register: `phi/sources.φ` + `phi/dead_sources.φ`. Der
Sweep liest `phi/pipeline/stage/*_converted.φ`. Stale-Specs gebannert:
PARSER_EVALUATION_MATRIX.md + EXTRACT_TYPES.md (SUPERSEDED by
SOURCES_V2_SPEC.md).

Offen (Detail in phi/pipeline/ledger.φ):
- Die Linse: Folgewelle — NASA-CMR-Keywords + GBIF-Tags downloaden,
  Library feinwägen; --port ersetzt --gold
- Probe-Stufe: nächste Welle — neue Kandidaten aus den Katalogen in
  batches/ nachrücken
- Queue: 10 Untested-Korpora (14k/13k/15k/7k/2k/183l/astro/earth/
  exotic/candidate-staging) — Port durch die Prozedur; astro-Korpus:
  28 Blöcke → manueller Port
- Bestand: 38 offene VizieR-Bulks, IRSA/GAVO/ARI/ExoArchive-Inventare,
  GCNS/MWSC, VirES-Drafts (erledigt 2026-08-18), ArcGIS-Drafts
  (erledigt 2026-08-18 — 35 verifiziert, 10 Blöcke integriert),
  103 TerraPulse-Kandidaten (erledigt), 77 Archeology-Gaps,
  ESA-Kandidaten (Aeolus key-needed, SMOS parser-def), FRB-Union,
  Arena/Foundation/Research-Schatz im Archiv
- Nachlauf: VirES-Vollprobe (64 Drafts, Datei ABSENT) + DONKI-Familie
  (CME-Draft, Datei ABSENT)
- Park: Pegelonline, USGS-Geomag, GWOSC/GraceDB (Skymap), DSN, CENC,
  JMA-Quake (cod-String), SDSS-SkyServer
- Rats-Befund Harvester-Binaries (2026-08-17): kafka_harvester/fdsn_harvester
  als eigenständige Binaries zulässig — std-only bindet den Archivar-Runtime
  (main.rs/lib.rs), nicht die Produktions-Tools (Präzedenz: wgpu-Build,
  harte Dependency). Reihenfolge: (1) Force-Gate zuerst — Alert-Ströme ohne
  Feldwert am Punkt fallen (ANTARES, dead_sources.φ); Trigger+Signifikanz =
  Nachricht, rekonstruierte Energie/magpsf/Luminosity-Distance = Messwert.
  (2) REST-Pull zuerst: GCN circulars/notices, IceCube, GraceDB, MPC tragen
  REST → rest_harvester deckt sie, kein neuer Decoder. (3) Nur ZTF
  (Kafka+Avro, IRSA-Auth declined) und FDSN dataselect (miniSEED-Zeitreihe)
  brauchen echte Decoder — beide AUSSTEHEND hinter dem Gate; miniSEED-Frage
  beantwortet durch Rats-Befund (2026-08-19): eine Waveform zerfällt in
  Samples (TESS-Muster, [t, flux]-Reihe) ODER in Bins (Spektral-Atom) —
  das Instrument deklariert seine Basis; keine Zerlegung wird erfunden.
  Der Oszillator trägt Teilchen- und Wellen-Darstellung; ein universelles
  freq-Record-Feld ist UNWAHR (gravity/thermal/diffusion besitzen keine
  Frequenz; color_index/tau/z tragen sie bereits reduziert).
  (4) Hand-Client vs. Crate:
  AUSSTEHEND — fällig erst, wenn ein Kafka-only-Feed das Gate passiert;
  dann Hand-Client (std::net::TcpStream), wenn Session-Atom bleibt, sonst
  Crate als harte Dependency (wgpu-Analogie).
- Rechecks b1–b5 (2026-08-17/18): erledigt — WOUDC, Tides&Currents,
  ERDDAP-Familie (coastwatch/ifremer/emodnet/PMEL), SWPC ace_mag_1h,
  BGS-GIN-HAPI, IRSA-Gator-CSV, environment.data.gov.uk, GTN-P (dead dns),
  GONG (FITS-Gap), Safecast (live bestätigt), VSX/PSRCAT (decline,
  VizieR-Mirror), AFAD-tadas (SPA bestätigt), AstDyS (decline model-fit),
  SERVIR-SoilMoisture (decline derived-product), Hurricanes-ArcGIS
  (decline model-forecast), MPC-Unterrouten (Scaffold), TESS-Target-CSV.
  INTEGRIERT in sources.φ: OceanNetworks-CTD (4 Kanäle), OMNIWeb-HAPI
  (8 Felder), SDSS SkyServerWS (cz/velDisp + photoObj-psfMag),
  sensor.community (PM1/PM2.5), NDBC (1 FeatureServer-Block, 912
  Stationen). OFFEN (src/ — Rust-Kybernautin): Argovis-Per-Level-Parser
  (alt=−pressure), SuperMAG (leading-line + Positions-Join), MPCOBS +
  TIC-Compiler (Eingangsformate gesichtet in compiler_inputs.φ).
- Kraft-Abdeckung: acoustic/electric/thermal/advective/diffusion-Kuration
  offen — electric: GIC-Netze + Live-E-Feldstärke (kein Feed); GLM ist em
  (Ratsurteil), WWLLN radio-em vs. Entladung-electric bleibt Force-Gate-Frage

- Die drei Ports der Nadeln (2026-08-18/19) — geprobt, Parser-Atome gebaut, Register disponiert:
  - PSP-Science: 3 Blöcke integriert in sources.φ
    (PSP_COHO1HR_MERGED_MAG_PLASMA 7 Felder,
    PSP_SWP_SPC_L3I 2 Felder, PSP_FLD_L2_MAG_RTN 3 Felder —
    Hapi-Komponenten-Index `hapi <param>.<i>=<key>` gebaut;
    `at parker_solar_probe`; Coverage bis 2025-12-31). ERLEDIGT.
  - EOP/IERS: `format finals` gebaut + getestet (Zeichenposition-Spalten,
    letzte belegte Zeile, MJD→TDB, 3 Feld-Linien); Block integriert
    (maia.usno.navy.mil/ser7/finals.all, at earth). ERLEDIGT.
  - Hapi-fill: fill-Werte werden jetzt verworfen (gilt für alle
    HAPI-Blöcke). ERLEDIGT.
  - OMNI2-Reparatur vollzogen: time.min/time.max (start/stop lieferte
    1400 gegen den Live-Server). ERLEDIGT.
  - IONEX-GIM: `format ionex`-Parser gebaut + getestet; der Kanal ist
    AUSSTEHEND — CDDIS verlangt jetzt Earthdata-OAuth (key-needed),
    GFZ/BKG/IGN-Routen 404/000 (19.8.2026). Kein Block im Register,
    bis eine Route anonym lebt oder der Earthdata-Account existiert.
  - ALeRCE: integriert — der celestial-oid-Fanout ist gebaut
    (`format alerce` + `alerce`-Directive + build_alerce_channels:
    Objektliste → je oid detections → himmelsfeste Positionen,
    MJD→TDB-Epoche, fanout 200). ERLEDIGT.
  - LuckyStar: decline (Vorhersagen sind Modell, keine Messung; die
    Ergebnisse-Server liefern nur abgeleitete Fits) — der rohe
    em-Lichtkurven-Kanal der Fresnel-Sphäre bleibt ausstehend.
  - ONC-Hydrophone: die Route lebt (dataProductDelivery-Kette, 85
    Stationen, HSD-FFT: 512 Bins × 250 Hz, dB re 1 µPa²) — der Kanal
    ist Atom B des spektralen Oszillators (siehe eigener Abschnitt);
    keine Skalar-Feld-Linie erfunden (0 honored).
  WARTEND: SuperMAG (Server-Fault db-get — Proben wiederholen),
  Gaia DR4 (2.12.2026 — Recompiler der 44-Byte-Records).

Doku-Drift (behoben 2026-08-17): Alle `archeology/`-Referenzen zeigen
heute auf den Bestand unter /home/johannes/projects/archive/archeology/.

## Der spektrale Oszillator — die Frequenzachse des Blocks

Plan: `docs/concepts/DER_SPEKTRALE_OSZILLATOR.md` (2026-08-19, Operator-
Einspruch gegen den ersten Rats-Befund — angenommen). Der Oszillator IST
eine Frequenz; die Welle ist die andere Hälfte. Die Atome, in Reihenfolge:

- Atom A — Protokoll v8: Record 22→24×f64 (`freq`, `bin_width`;
  0.0 = Punktquelle, 0 honored — kein Pflicht-Feld mit Fabrikation);
  Frame `0xCF 0x86 0x08`; drei Schichten gemeinsam (Rust-Write-Loop,
  constants.js-DataView — die zwei Padding-Nullen der meta-Reihe sind
  die Heimat, WGSL-props-Unpack); Hand-Verifikation der Kette, v7-Bestand
  bleibt lesbar (freq=0 rendert wie zuvor).
- Atom B — Spectral-Compiler: NCEI-SSI zuerst (λ→ν, liegt im Bestand,
  beweist die Kette), dann ONC-HSD-FFT (Route verifiziert: dataProduct-
  Delivery-Kette, deviceCode-Form, 512 Bins × 250 Hz, implizite Achse),
  dann Gaia-XP (gdr3spec.spectra-Bulk, CDN-Compiler wie dr3_stars.bin),
  LISA-PSD + CMB-Power (Freq-/l-Achse statt Skalar-Reduktion), GONG +
  miniSEED (Waveforms, std-only-FFT/Goertzel).
- Atom C — band-selektives Rendering: Shader akkumuliert pro Band
  (RGB ist bereits ein Drei-Band-Renderer); Stillekarte band-selektiv,
  Lichtkegel-Differenz dispersiv, chromatischer Dip der Nadel Ⅴ als
  SED-Messung.
- Atom D — die Phase (terminiert nach C): Beats/Interferenz brauchen
  die komplexe FFT; PSD-Bins tragen sie nicht (0 honored).
- Regeln: kein Namens-Trick (Frequenz lebt als Token, nie im String),
  kein Skalar-Schallpegel aus Spektren errechnet, jedes Atom ein
  vollständiges Session-Artefakt.

## Curation & Quellen

- TerraPulse-Katalog: 103 Kandidaten — erledigt (2026-08-18): Dispositionen
  gespiegelt in dead_sources.φ/blocked_sources.φ (grind_terrapulse_a.φ +
  b.φ in phi/pipeline/queue/); Katalog: phi/pipeline/katalog/terrapulse_catalog.φ.
  GLM (Ratsurteil 2026-08-17): GOES_GLM_Bolides-
  ArcGIS-GeoJSON ist die Route — Kraft `em` (NIR-Photodetektor 777,4 nm,
  Wert = detektierte optische Energie J, τ = Blitzdauer via `tau_key`),
  NICHT electric; netCDF entfällt für GLM. Erledigt 2026-08-18: beide Routen
  leben in sources.φ — CNEOS fireball.api (Gesamt-Energie e10j, kt_tnt) und
  ArcGIS GOES_GLM_Bolides (GLM-L2-LCFA-Radiant-Energie in J, SI; epoch_scale
  ms; τ = detected_duration). Zwei verschiedene Messgrößen, nicht ineinander
  konvertierbar.
- SI-Konversion total (Ratsurteil 2026-08-17): `convert_to_si` →
  `Option<f64>`, angewendet am Anker für alle Feldwerte; unbekannte/
  logarithmische Einheit → Oszillator manifestiert nicht, stderr
  registriert die Einheit (einmal pro Unit). Linear angewandt: deg/arcsec
  → rad (Feldinventar: Richtungs-Winkel), M_sun/M_earth/R_earth, MW
  (Fall-Exaktheit gegen Mw/M), d, uatm, mb, n/cc, cfs, %, psu, DU,
  pc/cm3, km/s, mJy, mg/kg, erg/cm2, µA/m2. `vel <key> [unit]` +
  `tau_key <key>` (0 schließt das Gate, absent = Feld-τ).

## Nachpflege: unconverted Units in sources.φ (2026-08-17)

22 Blöcke verloren Feldwerte durch die SI-Konversion. Alle sind jetzt
verdrahtet — nichts bleibt dunkel.

Pending Unit-Arme (2026-08-18): F (Fahrenheit, CHPL-Lufttemperatur), μg/L
(Chlorophyll, CREST-Boje), mg/L (Sauerstoff, CREST-Boje) — die Felder
existieren in den Quellen, manifestieren erst mit dem convert_to_si-Arm.
(µS/cm erledigt 2026-08-18 — DC/DMF-Leitfähigkeit manifestiert jetzt.)

ERLEDIGT (flux_from_mag, Primärband): cb(mag1), cometels(H), corot(mag),
dcom5(H; M1 dark), denis(jmag; kmag dark), gcvs(mag), lmxb(mag1),
mktypes(mag), pastel(mag), polarbase(mag), sb9(mag1; mag2 dark), sncat(mag),
vsx(max; min dark), wd(mag), wds(mag1; mag2 dark).

ERLEDIGT (convert_to_si-Arme, 2026-08-17):
- `Mw` → seismisches Moment M0 = 10^(1,5·Mw + 9,1) N·m — USGS/INGV/JMA/
  geonet + der `geojson`-Geschwister-Block (Unit jetzt `Mw` statt roh).
- `logg` → 10^logg · 0,01 m/s² (corot/pastel/polarbase, Blöcke deklarieren
  jetzt `logg` statt `dex`).
- `Crab` → ×2,4e-14 W/m² (tevcat; kanonische Crab-Referenz >1 TeV).
- `Jy_km/s` → ×1e-23 W/m² (alfalfa; exakter Linienfluss).
- `cpm` → ×1e-6/(334·3600) Sv/s (safecast; bGeigie-Nano-Firmware-Konstante
  NANO_CPM_FACTOR 334, verifiziert im Quellcode NanoConfig.h).

Magnitudentyp pro Event (erledigt 2026-08-17, verifiziert per Live-Abruf):
- `mag_type_key <key>`-Direktive (map/geojson) liest den Typ pro Zeile.
- Moment-basiert (mw/mww/mwc/mwb/mwr/mwp/mwpd/mi, case-insensitiv) →
  M0 = 10^(1,5·m+9,1) exakt. Nicht-moment (ml/md/mb/mh/ms/m/Mj) →
  keine exakte SI-Konversion aus der Magnitude allein → der Oszillator
  fälscht kein Moment (manifestiert nicht).
- Verdrahtet: USGS-Summary + INGV (`properties.magType`), USGS-fdsnws
  (`magType`). Die Mehrheit kleiner Events (ml/md/mb) manifestiert daher
  keinen Momentwert — ehrlich, weil eine Lokal-/Raumwellen-/Dauer-Magnitude
  kein lineares SI ist; Position + Tiefe bleiben.
- Ohne Typ-Feld: GeoNet (netMag gemischt), JMA/P2PQuake (Mj ≈ Mw in 4,5–7,5),
  EEW (als Mw behandelt) — Block deklariert `Mw` als Moment-Proxy (die
  Standard-Schätzung). Sekundärbänder (kmag/M1/mag2/min) bleiben dark, weil
  `flux_from_mag` eine Primärband-Direktive ist (eine Skala pro Block).

Betroffen, aber unentschieden (0 honored: die Daten existieren, die
Konversion ist bekannt oder erforschbar — pending, keine Fälschung):
- HorizonsVec-Fetch (Ratsurteil 2026-08-17): `{jd_now}`/`{jd_start}`/
  `{jd_end}` in `render_url` (TDB, 6 Stellen) — die 0B-Ursache
  (Kalenderdaten im JD-Feld) ist behoben. Ein Live-`vectors`-Block in
  sources.φ bleibt Kurationsfrage: dead_sources.φ:3090 deklariert
  Horizons als Compiler-Eingang, keine Live-Quelle.
- Bestands-Abgleich mit LLM-Befunden (2026-08-17): Quellen, die andere
  Modelle als „fehlend" meldeten, aber bereits registriert sind — nicht
  erneut gräben. Live in phi/sources.φ: NOAA SWPC (GOES/ACE/DSCOVR/Kp/
  DONKI), USGS Earthquake, EMSC/seismicportal, INGV, ARGO-Drifter
  (erddap.aoml), OOI, PMEL-CO2, GraceDB superevents. Geparkt in
  phi/dead_sources.φ: eROSITA, ALeRCE, Fink, ANTARES, LSST-TAP, GEBCO,
  ALMA, NRAO, NOIRLab-AstroArchive, IceCube-HESE, GCN, Fermi-LAT, MPC,
  IERS, CDDIS, OBIS, CASDA, Euclid, VAMDC/CDMS. Indexiert
  (tap_index_*/tapvizier): SDSS, LAMOST, Hipparcos, UCAC4/5, AllWISE,
  Pan-STARRS, Chandra-Log, XMM-Newton.
- Katalog-Lücken (genuin, verifiziert gegen alle drei Register): Photometrie/
  Spektroskopie — 2MASS PSC, RAVE DR6, APOGEE/GALAH; SDSS als Feldquelle
  (nur Crossmatch indexiert). Extragalaktisch — NED, HyperLEDA/PGC, GLADE+.
  Radio-Kontinuum (Achse leer) — NVSS, FIRST, TGSS ADR, SUMSS, RACS, LoTSS,
  VLASS. High-Energy — Fermi 4FGL-DR4, Chandra CSC 2.1, AMS-02.
  Sonnensystem — PDS (Instrumentendaten); MPC-Live (mpcorb_extended.json.gz).
  TAP-Indexe — MAST, CADC, ESASky, NOIRLab Data Lab, NED. Terrestrisch —
  EarthScope-FDSN, EPOS, SeaDataNet, Smithsonian GVP, Natural Earth.
- Zeitkritisch: Gaia DR4 (2. Dez 2026) — dr4_stars.bin + DR4-Schema im
  tap_compiler (5,5 a, halbierte Parallaxenfehler, Gaia-Exoplaneten).
  Rubin LSST DR1 (Ende Juni 2028), Alerts live (Broker declined). GCVS-Stand
  prüfen (HEASARC-Update Juni 2026 vs. gcvs_cat.json).
- Katalog-Lücken Welle II (Recherche 2026-08-17): Diffusion/Chemorezeption
  unbesetzt — TCCON (verifiziert, tccondata.org, Registrierung); pending
  Verifikation: AGAGE, NDACC, WDCGG, GLODAP, EBAS (THREDDS/REST je Anbieter
  prüfen). electric: WWLLN (registriert/restringiert) — Force-Gate klären
  (radio-em vs. Entladung-electric), sonst refused. em terrestrisch: NSRDB/
  BSRN (Bodensolar fehlt) — NSRDB pending Verifikation. gravity: BGI/GGP-
  Bodengravimetrie (IGETS nur indexiert) — pending Verifikation.
- Zeitkritisch II: SPHEREx (IRSA VOAPI + AWS S3 + FITS, Quick-Release live,
  Voll-Katalog 2026 — verifiziert), DESI DR1 (NOIRLab Astro Data Lab TAP,
  ~18 Mio Spektren — verifiziert), Roman (2027), 4MOST/WEAVE (2026) —
  unverified. Gaia DR4/LSST bleiben wie notiert.
- Struktur-Reader (Voraussetzung für SPHEREx/DESI/GLODAP): netCDF-3
  (CDF-1 + CDF-2, std-only) in src/netcdf.rs — erledigt; CDF-5 bleibt
  `pending` (eigener Atom). Offen: FITS-Binärtabellen, Parquet/Arrow,
  netCDF-4/HDF5, OPeNDAP-Integration. GLM braucht keinen netCDF-Reader
  (ArcGIS-GeoJSON-Route, Kraft em).
- Crossmatch indexiert → live heben: GALEX-GUVcat (UV), SkyMapper DR4,
  UKIDSS/VISTA/VIKING (NIR), DES DR2/Legacy Surveys DR10.
- Katalog-Lücken Welle III (Recherche 2026-08-17, gegen alle Register verifiziert).
  Dedupe — Parallel-Befunde meldeten als „fehlend", was bereits registriert ist:
  INTERMAGNET live (sources.φ BGS-GIN-HAPI), HAPI-Extract existiert
  (grind_vires_catalog), GRACE-FO live (VirES-KBR), SuperMAG/AAVSO/GIRO/
  SuperDARN/MODVOLC/EarthScope-FDSN/EPA-RadNet/Water-Quality-Portal/STAC/Zarr/
  RUCSoundings in dead_sources, GLEAM/VLBI-ICRF + Sentinel-5P/ICEYE/DORIS
  indexiert, LAMOST/UCAC/SDSS/XMM in tap_index. Nicht erneut gräben.
- Katalog-Lücken Welle III (genuin): electric — AMPERE, GloCAEM, USArray-MT;
  diffusion — EMEP/CCC, WDCRG, European Waterbase; em — NEUBrew (UV), THEMIS/
  ASI (Polarlicht, CDF), COSMOS2025/COSMOS-Web, INTEGRAL, ATLAS-RefCat2,
  Subaru HSC-SSP, TIC; kosmisch/Neutrino — CREDO, KM3NeT; Geodäsie — ILRS,
  IVS-EOP, DORIS-Live, GRACE-FO-Mascons (L2/L3); Atmosphäre/Ozean — E-GVAP,
  Wyoming-Soundings, BGC-Argo-live, IOOS-HFRNet, NOAA-NRS (Ozean-Lärm),
  MIROVA. Zugriffsarten pending Verifikation (unverified).
- Zeitkritisch III: Euclid DR1 (Okt 2026), SDSS-V, eROSITA-DR2 (Juli 2026
  erschienen — prüfen ob via HEASARC-tap_index erreichbar).
- Struktur-Reader II: CDF, GRIB-2, GeoParquet, OGC-SensorThings — fehlen;
  SeedLink/Streaming vom Rats-Befund (REST-Pull zuerst) abgedeckt.
- ESA/Geomagnetik: Swarm TCT-E-Feld (keyless), VirES-Aeolus, SMOS,
  MERIS/SAR/Landsat Kandidaten
- INTERMAGNET-Fanout (154 Observatorien live): Ausbeute-Feintuning offen
  (best-avail-Aktualität variiert je Observatorium)
- Grind-Einbau offen: 32 ArcGIS-Drafts (thermal/seismic/diffusion/em/
  advective/gravity); ARI GCNS (331.312 Sterne ≤100pc) + MWSC (3.006
  Haufen) als Kompilat-Kandidaten; 8 VirES-Drafts (CHAMP/GRACE/GOCE/
  CryoSat MAG/DNS/WND/TEC/KBR); archeology-gaps 77 Kandidaten (AERONET,
  IERS-EOP, Fireball/Sentry, Xamin-TAP, GONG2, GIRO-Ionosonde,
  e-CALLISTO …) als nächster Grind; FRB-Union-Merge mit
  TNS-Namens-Normalisierung (FRB121102 ↔ FRB20121102A) + frbcat.org-CSV
  als Quelle; Mauna-Loa-CO2 + Fireball-API: `fold <op> <key_a> <key_b>
  <force> <unit> <tau>` lebt (mean|diff|sum, absent Halbspalte → kein
  Oszillator) — Mauna-Loa `mean` (Hintergrund) + optional `diff`
  (Gradient) entschieden; Fireball-Operator (sum vs. mean) unverifiziert
  — Live-Verifikation offen
- Host-Kuration offen: CENC (Keyed Object No1..NoN), JMA-Quake (Position
  im cod-String), Pegelonline (Fanout-Block steht aus — P09), GWOSC/
  GraceDB (Position nur via Skymap), DSN (statische Dish-Positionen),
  USGS-Geomag (Komponenten-Timeseries)
- Rechecks offen (Stand 2026-08-18, alle src/ — Rust-Kybernautin):
  Argovis-Per-Level-Parser ERLEDIGT (ProfileMap-Extract + Block in
  sources.φ); SuperMAG: leading-line „OK"-Strip erledigt, aber
  Positions-Join + station-Filter bleiben server-blockiert (db-get-Fault
  reproduziert 2026-08-18; phi/-Zugang logon-only geklärt).
  NDBC-Konsolidierung vollzogen (912 Stationen verifiziert).
  sensor.community integriert. SDSS-photoObj integriert (photoObj.z =
  modelMag_z). MPCOBS + TIC: Eingangsformate gesichtet
  (compiler_inputs.φ) — Compiler-Bau = src/.
- TAP-Katalog-Pipeline: VizieR-Bulks aus phi/tap_index.φ — Grindwelle
  2026-08-18: 13 Blöcke integriert (atnf/carmenes/cornish/gcvs/hecate/
  first_radio + IRSA irasfsc/akari/msxc6 + GAVO rave/ohmaser +
  ExoArchive stellarhosts), ~20 decline/re-probe (allwise timeout,
  bzcat5 LogFlux, atlasgal ICRS-Spalten, harris VII/202/catalog,
  galactic_black_hole 163/207/table1).   GCNS/MWSC = Kompilat (src/),
  liegen in GAVO dc.g-vo.org. GAVO-Async-Queue erledigt (2026-08-18):
  /tap/async = UWS-1.1-Jobliste (gavo.aip.de + dc.g-vo.org).
- netCDF-Welle (2026-08-18): netcdf_reader (CDF-1/2) verifiziert. Argo-Profil
  ERLEDIGT (`format netcdf` + Per-Level-Expansion alt=−PRES·decibar im
  Archivar-ω()-Loop, Block in sources.φ; `_FillValue`+NaN → 0 honored).
  GLODAP bleibt CSV (kein .nc; Bottle-Rohwerte als CSV-Harvest-Kandidat).
  OOI/SOCAT/temis .nc = netCDF-4/HDF5 bzw. 404.
- Hapi-FieldConfig: die deklarierten kernel/force/tau der HAPI-Blöcke
  erreichen den Oszillator nicht (synthetisch {0,0,0}) — Klärung in
  der P-Liste
- Enrichment offen: Name-basierter Ersatz-Join, GSC I/220
- Vorräte (Pfade unter /home/johannes/projects/archive/archeology/):
  sources/sources_gold_pre-cdn_27k (2572 Blöcke) +
  sources_recovery_pre-cdn_25k (1924) — Migration nach Protokoll
  (docs/source_curation.md); sources_new_untested_14k (873) +
  sources_astro_untested (30) + sources_exotic_untested (16) +
  sources_earth_untested (3) — UNTESTED_index.txt nicht archiviert,
  per-Domain-Index rekonstruieren; sources_recovery_cdn-merged_60k
  lost-blocks (5701 urls, 0 field-Tokens) — Extract-Parameter aus
  history/recovery zuordnen; arena/ (batch_01–21, ungeprüft);
  foundation/ (APIs/collection/gaps); failed_eph_rust/ (abgelöst);
  reconstruction/*.bak (nur Historie)
- phi/pipeline/research/batches/ (283) + probe_batches/ (242): alte Grammatik —
  nicht ladbar (P01); die Migration lief über den --gold-Konverter

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

- I02: refresh-protected-data.yml (Python inline) → Rust
- Token-Rotation: der git-Remote-Token (keine releases/actions-Rechte)
  gehört rotiert und auf credential-helper/SSH umgestellt
- SPK-Segment-Payloads lazy laden (strukturell — siehe Atome)
- Das Python refresh.yml im sources-Repo bleibt bis I02 auf Python —
  Abschaltung nach Verifikation der Rust-Katalog-Kompilate im
  kernel_flatten-catalogs-Job (ein Produzent pro Asset)
- CDN-Asset-Naming: `{name}.json` — Konvention ist der Resolver (Regel)
- kernel_flatten-Neulauf: ephemeris_compiler n_sections 2→3 (rotationslose
  Körper wurden verworfen, Rotation abgeschnitten) — CDN-Neukompilat
  verifizieren (rotationslose Körper laden, Rotations-Matrizen präsent)

## Feature Backlog

- Advective per-Quelle: Wind in tm.w (Kanal verdrahtet, Messquelle fehlt)
- OPeNDAP-Integration
- Command Palette (M07)
- Camera: ~19k Pixel-Quellen (4×4-Raster) → WS-Traffic-Hotspot
- `sensor_config`/`probe_classify`-τ/TTL-Konstanten (60/300/0.01/3600)
  ohne Herleitung
- `getRto` min/max 100/5000 ms — nicht 2ⁿ/Φ-hergeleitet (constants.js)
- Probe: `coordinates.2` als alt vs. Tiefe bei Seismik — Vorzeichen offen

## VERSIONIERT / AUSSTEHEND

- Temporal Topology (TDA, Takens, Transfer Entropy, Surrogates) —
  VERSIONIERT, LOST_CONCEPTS.md
- Field Permeability (tanh(vC/g)-Variante ohne TE) — VERSIONIERT,
  MINKOWSKI_FIELD-PERMEABILITY.md
- Minkowski 4D Weighting (spacelike→0; kosmisches Skalenproblem: Sonne
  wäre spacelike — scale-Anpassung nötig) — VERSIONIERT,
  MINKOWSKI_FIELD-PERMEABILITY.md
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
  WGSL_ SHADER.md)
