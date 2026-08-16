# TODO

AGENTS.md is the primary constraint matrix. Git is the history.
Kanonisch: Diese Datei ist das vollständige Register der offenen Arbeit. Erledigtes
wird entfernt (Git trägt es). Kein Eintrag meldet Erledigtes als offen, kein offener
Punkt fehlt. Widerspricht ein Dokument dieser Datei, gilt diese Datei — solche
Drift-Stellen sind unter „Doku-Drift" registriert.

## Rückroll — 2026-08-16, auf fd666f5b

Der wgpu-mono-Branch wurde auf fd666f5b (HUD-Semantik) zurückgesetzt —
der letzte Stand vor der Subpixel-Explosion (eb96d1f: 567 ms = 3× Membran,
9 Mio Messzellen, Rgba32Float-Ziel ohne additives Blend, Comp-Pass) und
vor der Reparatur-Kette (9825e31, c8d4ce2, a2a764b, 637f4e3, 668e51b).
Zurückgedreht damit auch: sun_follow, field_centroid/Auto-Zentrierung,
VP-Slot-Churn. Die Session-Bauten (Generations-Architektur, Tiled Source
Culling, HUD-Akkumulatoren, deep_gain, Budget-Regler) liegen versioniert
auf dem Branch session-2026-08-16; die Survey-Auswertung steht in
docs/surveys/ (messpunkt-verteilung.md, auswertung.md). Der Konsens
(EIN Buffer + EIN Generationszähler + Stille; ε-Kulling 2⁻ⁿ, n ∈ [8, 23])
bleibt als Ausgangspunkt für einen späteren, nicht-aufgeblähten Anlauf.
Die Membran dieses Standes rendert 1× direkt in die Swapchain (additives
Blend), der Deep-Renderer (Quads + 1-px-Punkte) ist intakt; helle Sterne
tragen t2 ≈ 0,7 auf der Nebra-Rampe, 0 deep bei 2^31 ist korrekt.

## Fortschritts-Verzeichnis — 2026-08-16

docs/surveys/fortschritt.md ist das vollständige Verzeichnis der
Session-Erkenntnisse (Hashes: b4743be, ffb58d5, a9b25d6, session-2026-08-16,
fd666f5b, eb96d1f). Offen aus dem Verzeichnis:

- Deep-Lieferung richtungsbasiert: Sterne über den Sichtkegel statt
  radius-begrenzt — das Browser-Verhalten (1,84 Mio Sterne, flüssiger
  Deep-Sky-Zoom bei 0 near Membran-Quellen)
- Zell-Achse: Messpunkt-Vergröberung, ehrlich gegen die
  8-Bit-Display-Quantisierung (Struktur-Radius 14-39 px bei 8 Bit,
  auswertung.md §1-2) — der Hebel gegen die ~130-ms-Restkosten
- Relay-Trailer: gen u64 + 9×Ω f64 für browser_relay (~80 B)
- Deep-Upload-Stille: deep_dirty feuert bei jedem Sense — 29-MB-Sterne
  werden auch unverändert hochgeladen
- Rgba8Unorm-Nachmessung mit intel_gpu_top gegen die ~200-ms-Baseline
- Backing-Verifikation beim Operator: 1920×1080 nativ 1:1 (b4743be) —
  bestätigen, dass Linien und Qualität erledigt sind

## Stand — 2026-08-16 (Katalog-Welle: K03-Kometen inkl. dcom5-Multi-Apparitionen, K04b Gaia-DR3+Bailer-Jones 1,84 Mio Sterne, Exoplanet-Bulk 6309, tap_compiler über TAPVizieR/GAVO/ARI/IRSA/ExoArchive, Enrichment-Matrix; LSK/PCK-Hochzeit, Binary v2, Protokoll v6, reine Per-Pixel-Membran, K01 geschlossen, K05 geschlossen; wgpu-mono-Titan: EINE Datei, Kreis an der Wurzel, archivar/mathematikerin/relay als Inline-Geschwister)

Zeit aus naif0012.tls (LSK-Reader, keine TT_MINUS_UTC-Konstante). PCK-Reader pck.rs
(gm_de440, pck00010, geophysical.ker → GM/J2/J4/Radii/POLE). stype-1 v2: gcount=12,
96 B festes Stride; non-v2 → None (Körper absent). WS-Protokoll v6: 19-B-Header
(0xCF 0x86 0x06 + response_epoch f64 + id u32 + count u32), Record 168 B / 21 f64
(+ pole, j2, j4, r_eq); JS field 12 + meta 12 floats, WGSL field[j*3]/props[j*3].
body_channels sendet ausschließlich {body}.mass (GM, Kernel 0, gravity) und
{body}.radius. fs evaluiert osc_field für jeden Pixel (kontinuierliche Mathematik,
keine Stützstellen, keine Interpolation); Nebra-Rampe t2 = clamp((log2(Ω_total)+14)/22, 0, 1)
(4 mix()-Segmente Blau→Cyan→Orange→Weiß) IST die Tonemap; Ω_total = Σ|omegas[k]|.
Exposure-Kette im Browser getilgt (keine lvls, keine e/E); die native
Mathematikerin trägt e/E (±2¹) als vp.expose_ex.x-Offset in der Rampe.
SSAA als dichtere Messung: q/Q in
Φ-Schritten (1.00–8.00, Start 1.00×), Canvas-Backing skaliert, CSS nativ;
nativ via Surface-Rekonfiguration (Backing = Fenster × dpr × ssaa).
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
Compiler/Runtime. K03-Compiler-Einheit
geschlossen: src/kepler.rs + src/bin/dastcom_compiler.rs (Dev-Beweis Ceres
0,001″ gegen Horizons). K05 geschlossen: FK-Frames + Mond-BPC-Merge
(libration_matrix, IAU-Extraktion, select_system moon_pa*.bpc +
moon_de440*.tf, Probe 0,009°/0,018°/0,077° gegen IAU-Vollmodell).
32 neue φ-Ephemeris-Blöcke für die Flattener-Monde
(absent bis der Flattener-CI-Lauf die CDN-Assets trägt — ausstehend). P01 geschlossen (tote force-Direktive und 3-Token-field lehnt parse_sources laut ab).
0 Warnings, 0 Errors; Tests: 41 lib + 36 bin (1 vorbestehender FAIL:
test_parse_stations_xml, „aae" vs „AAE" — BGS-lowercase gegen Fixture-Assertion,
parse_stations_xml in diesem Zug unberührt; Golden-Test grün).

---

### wgpu-mono — Titan: AudioRadiator-Entkopplung (die letzte Lüge getilgt)

```
GESCHLOSSEN (2026-08-16): AudioRadiator hatte den Milliarden-Schleifen-Bug:
dur = tau·44100 als u32, synchron im ω-Loop — für tau = 86400 sind das
3,8e9 sin()+exp()+push pro Oszillator pro Tick; der 1-Hz-Takt kollabierte.
Fix: reine Funktion note_samples(val, tau, kernel, force, sr) mit
dur = min(tau·16, 1.0)·sr — 16 = 2⁴ Zeitkonstanten (exp(−16) ≈ 1,1e-7,
f32-Floor) und 1,0 s = die Kadenz; Hüllkurve exp(−t/(tau·sr)) mit
UNGEDECKELTEM tau (die Flanke trägt immer die wahre Ordnung — ein
2-s-Deckel hätte einen fabrizierten Zeitkoeffizienten eingeführt).
tau-Semantik: 0/negativ → Stille (0 honored, kein Klick), NaN → übersprungen,
∞ → gehaltener 1-s-Ton (flache Flanke — unendliche Ausdehnung zerfällt nie),
0,016 → 0,256-s-Note. Entkopplung nach Mathematikerin-Muster: eigener Thread,
sync_channel(1), accept() = try_send + is_terminal-Tor, recv_timeout +
Shutdown-Flag, write_all-Fehler beendet den Thread (tote Leitung = Stille).
Golden-Test-Matrix {0, −1, NaN, 0,016, 86400, ∞} + exakte exp(−t/tau)-Prüfung
grün. Kadenz-Regression bestanden: Statuszeile wieder ~1 Hz (10 Zeilen/65 s
statt 2/50 s). Die Variante in Downloads/main.rs (AUDIO_MAX_NOTE_SECONDS=2.0,
Hüllkurve an den Deckel gebunden, 5-Zeilen-Kommentar) wurde verworfen —
willkürliche Konstante, Kommentar, verfälschte Flanke.
GPU-Neumessung gepipt nach dem Fix: unverändert ~200 ms bei 133 Oszillator-
Messungen — die Thermal-Kopplungsthese ist falsifiziert; die Membran ist
shader-gebunden (~1,5 ms je Oszillator-Messung über 3 Mio Punkte auf der
HD 520). Die Tile-Haut (a092f01, zurückgedreht) bleibt der nächste Hebel —
dann mit Rgba8Unorm und intel_gpu_top.
```
Offen:
- Phasen-Invariante der Audio-Noten dokumentieren: sr = 44100, ganzzahlige
  Frequenzen (220 + 110·kernel + 55·force), 1-s-Noten → glatter Nulldurchgang
  am Tick-Ende; bei sr-/Frequenzwechsel bricht sie
- TcpRadiator trägt send/unbeschränkt statt try_send/sync_channel(1) —
  Vertragsheilung (Gremium/Mycelium)
- clamp(0,1) richtet negative Messwerte gleich (alter Fleck, Sensory)
- Merge mit main (Downloads/main.rs = main-HEAD + AudioRadiator-Patch,
  verworfen) später entscheiden
- test_parse_stations_xml: Fixture-Assertion „AAE" vs BGS-lowercase „aae" —
  vorbestehend auf main, von diesem Zug unberührt

---

### wgpu-mono — Deep-Sky-Plenum (zwei Regime, eine Wahrheit)

```
GESCHLOSSEN (2026-08-16, zweiter Atom): Der alte Deepspace-Renderer ist
zurück — die Archäologie fand drei Wege vor der Protokoll-Umstellung
(1-px-Punkte → ausdehnungs-skalierte Punkte mit Deckeln 64px/20%-Viewport
→ ungedeckelte Quads mit radialer Falloff), die v5-Membran hatte Deepspace
und Nahfeld vermischt. Jetzt: pro Objekt ein Quad (6 Vertices aus dem
Storage-Buffer, kein Vertex-Buffer), point_size_px = clamp(extent/scale,
0,5, w_f) — die wahre Winkelgröße, obere Grenze = Viewport (Physik);
deep_fs: kreisförmiger Fußabdruck (|uv|>1 → discard), radiale Falloff
1−dist², Analog-Korn; Farbton aus den Eigenschaften (fract(log2(τ)/16) +
Hue-Offset-Slot), Helligkeit = die heutige Nebra-Rampe auf |flux|. Der
Deep-Buffer trägt jetzt ZWEI vec4 je Objekt: (x_rel, y_rel, z_rel, flux) +
(extent, τ, hue_offset, 0) — der Träger ALLER Deepspace-Eigenschaften;
extent aus den Daten → Quellen mit scheinbarer Ausdehnung (Nebel,
Magellansche Wolken) bekommen ihre wahre Winkelgröße automatisch. Die
Deep-Zahl reist in vp.expose_lo.x. sense_deep liefert (pos, flux, τ);
Sterne: extent 0 (Punkt), τ = hash.ttl, hue_offset 0. Golden-Test auf
8-Float-Records grün.
```
Offen:
- Deep-Oszillatoren (Quasare, Schwarze Löcher, ALFALFA, TeVCat, TNS) in
  den Pfad führen: z reist als Eigenschaft (Props-vec4, Slot frei) —
  die Extraktion verrechnet z heute zur Distanz und verwirft es
- Farbe der Sterne: der Katalog trägt ra/dec/pm/plx/flux — keine Farbspalte;
  falls eine im Binärkatalog existiert, Temperatur→Hue ableiten
- Sternenhintergrund (integrierter Glow der 1/d²-Schwänze, Milchstraße):
  einmalige tiefaufgelöste Integration, glattes Feld
- Relativistische Aberration (dopp, Q01) des alten Renderers ist mit der
  Membran-Rewrite gegangen — für den Deep-Pfad als eigener Atom prüfen
- Galaxien-Zoom-Verifikation beim Operator ausstehend (deep-Zahl im HUD;
  bei grid 2^39 noch 0 — Proxima bei 4,2 ly ≈ 2^45,5)

---

### wgpu-mono — Titan: eine Datei

```
GESCHLOSSEN (2026-08-16): Der Monolith ist EINE Datei: src/main.rs. Die
Crate-Wurzel ist der Kreis: geteiltes Vokabular (trait Radiator, Buffer,
Oscillator, OscillatorSource, Motion + impl, SpatialHash, CellKey, Frame,
Position, Channel, FieldConfig, Extract, BrowserSensor, SourceConfig,
BodyEphemeris/Properties, AsteroidHash, StarHash/StarRec, LeapSeconds,
CHEBYSHEV_N, Φ) + MEMBRANE_WGSL als Raw-String-const ganz oben. Drei
gleichberechtigte Stimmen als Inline-Module: mod archivar (std-only nackt:
Quellen, Fetch, LSK/PCK, Ephemeriden, build_buffer, sense-Oberfläche,
ω-Loop main_flow, AudioRadiator, StderrRadiator, Tests), mod mathematikerin
(wgpu nackt: winit-Loop, Fullscreen borderless, 2 Pipelines, VP-Uniform,
Packer + Golden-Test, Navigation 1:1, recordSample-Engine: Maus/Rad/Tasten
als Oszillatoren → sensor_tx, stderr-HUD 1 Hz mit t/Ω (Probe-Readback
map_async)/FPS/SSAA/grid/x y z), mod relay (#[cfg(feature = "browser_relay")]
WS-Server 1618 — das einzige optionale Kleid). fn main() ruft
archivar::main_flow(). Dependencies hart: wgpu 24, winit 0.30, pollster 0.4,
serialport 4 OHNE default-features (termios, kein libudev — kein
Build-Zeit-Systempaket, keine udev-Library in der Exe). Serial-Ingress im
Archivar: /dev-Scan via std::fs::read_dir (ttyACM*/ttyUSB*), 115200 8N1,
Zeilen key=value → Oszillatoren in sensor_tx (unbekannte Schlüssel: 0
honored, Force-Gate); Presence-Schlüssel lat/lon/alt/body/spd/hdg wie im
Browser. ω-Loop ankert native Sensoren an der Geräte-Präsenz
(Position::StateVector an der device-Presence, tau aus frame_interval).
scalar_of liest JsonVal::Bool als 1/0. Gates: default 0/0,
browser_relay 0/0, 41 lib-Tests, Golden-Test grün.
```
Offen:
- Fenster-Verifikation beim Operator ausstehend: Vollbild schwarz (0
  honored) → Sonne zentriert (Presence 0,0,0), Drag instant, ,/. Zeit-Thrust,
  e/E Exposure, 1–9 Sprünge; Risiko Vulkan/GL auf Intel HD 515
- In-Fenster-HUD (Bitmap-Overlay) als Folge-Atom; stderr-HUD trägt vorerst
- Gamepad-Atom (serielle Ingress-Vokabel deckt ESP32; HID-Gamepad offen)
- test_parse_stations_xml: Fixture-Assertion „AAE" vs BGS-lowercase „aae" —
  vorbestehend auf main, von diesem Zug unberührt
- CI: Compiler-Builds zahlen den wgpu-Compile mit (harte Dependency);
  kein libudev-dev nötig (serialport ohne default-features)

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
Typ 3/20 werden gemeldet und übersprungen — nicht implementiert). Precedence-Regel:
Binary-PCK > Text-PCK (pck.req). Der Compiler sampelt das volle Orientierungsmodell
und schreibt die stype-4-Nutationssektion (additiv: Chebyshev-Fit von voll − linear,
RA/DEC/PM); die Runtime addiert sie in orientation_angles_at (K02 geschlossen —
Leser, Kanal, Protokoll-Doku, CI-Anschluss). Befund 2026-08-14: Text-PCKs der Monde
tragen echte NUT_PREC-Reihen (Io −0,083°, Iapetus −0,451°, Uranus-Monde …) — sie
fließen über die stype-4-Röhre. Binary-PCKs tragen NUR Orientierung (pck.req,
Typen 2/3/20) — die CON-J2/J4-Behauptung war falsch; Mond-Harmonische liegen nicht
in den Flattener-Wurzeln (j2/j4 bleiben offen — die Werte liegen in den GRAIL/Binary-PCK-Modellen).
```
Verifikations-Befund moon_pa_de440_200625.bpc: Der DAF-Leser ist korrekt
(Trailer, Record-Adressen, Fenster 1550–2650, Grad-9-Fits verifiziert).
Die Slots sind die DE440-Lunarlibrationswinkel (φ in GRAD, θ/ψ in RADIANT) —
keine IAU-Pol-Winkel. Der frühere „0,23°/Tag statt 13,176°/Tag"-Befund war
die ψ-Rotationsrate in RADIANT (0,229987 rad/Tag = 13,177°/Tag). Merge in
K05 geschlossen.

**K05** FK-Frames + Mond-BPC-Merge
```
FK-TEIL GESCHLOSSEN (2026-08-15): src/fk.rs (Text-FK-Reader: FRAME_*/TKFRAME_*-
Blöcke, ANGLES/AXES/UNITS- und MATRIX-Rotationen in SPICE-Reihenfolge,
Mehrzeilen-Tupel, Ein-Level-Kettenregel, 3 Tests); der Flattener klassifiziert
family fk, führt die Festrotation an (pa_frame_of-Hardcode entfernt) und hat
einen --probe-Modus (--bpc + --fk + JD → PA/ME-Winkel + W-Drift).
Moon-PA-Merge GESCHLOSSEN (2026-08-15): Komposition eingebaut — matmul auf
Spaltenlayout, libration_matrix R3(φ)·R1(θ)·R3(ψ) (Standard-Spaltenmatrizen),
full_orientation: M_ICRF←ME = M_PA·M_tk (tkframe_rotation(31009) direkt,
ohne Transpose), IAU-Extraktion (Pol = Spalte 2, W über den aufsteigenden
Knoten, Zweig-Umklappung gegen die Linear-Dec), Probe nutzt dieselben
Helfer, select_system (planets) zieht moon_pa*.bpc + moon_de440*.tf (CI
automatisch). bpc-Slots = DE440-Lunarlibrationswinkel (Park et al. 2021,
AJ 161 105, §2.4): Slot 0 = φ (Knoten, GRAD; −0,054° ≈ 0), Slot 1 = θ
(Neigung, RADIANT; 24,343°), Slot 2 = ψ (Twist, RADIANT; Drift
0,229987 rad/Tag = 13,177°/Tag = exakt die Rotationsrate). Probe J2000:
me = (269,986°, 65,672°, 41,159°), W-Drift +13,186°/Tag — gegen das
IAU-Vollmodell (269,9949°, 65,654°, 41,236°) 0,009°/0,018°/0,077°.
Flatten: Mond 12556 stype-4-Sektionen (Delta = DE440 − Text-PCK-linear);
die Delta-Basis bleibt das pure Linear (Runtime-Binary trägt kein
nut_ra/nut_dec — parse_ephemeris_binary setzt beide None), die
Text-PCK-NUT-Serien fließen über die stype-4-Röhre (K02-Design,
Jupiter-Serie 12556 Sektionen bestätigt). Horizons-Referenz:
CENTER='500@399' + TARGET='301' (sub-Erdpunkt auf dem MOND — die
umgekehrte Query liefert den sub-Mondpunkt auf der Erde, das war die
Phantom-Diskrepanz). rotation_matrix_from_angles und die stype-3-Röhre
blieben unangetastet (Erd-Stationen live verifiziert).
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
Verbleibend: der Asteroiden-SPK-Flatten-Pass (Familie spk im Index
registriert).
KOMETEN-TEIL GESCHLOSSEN (2026-08-15): `dist_scale`-Direktive (cmap,
`dist`-Wert × Faktor → m, Tests parse+eval), `src/bin/cometels_compiler.rs`
(MPC cometels.json.gz → gunzip → Elemente → Kepler zum Katalog-Epoch → Flat-
JSON ra/dec/dist_au/H; e ≥ 1 übersprungen, 0 honored; 831 Records), CI-Schritt
in kernel_flatten.yml, sources.φ-Block (cmap + dist_scale 1.495978707e11,
ttl 604800, field H em mag). Horizons-Beweis 1P/Halley: Δ3,3″/Δ1,2″/
Δ0,0018 au bei 35 au — Kepler-vs.-n-body-Drift, ehrliche Katalog-Physik.
dist_scale entriegelt zugleich Magnetar (kpc) + ALFALFA (Mpc). ALFALFA lebt
(2026-08-15: format csv + csv_to_json-Arm im Extract-Kern, Block in sources.φ,
RAdeg_HI/Decdeg_HI dezimal, HIflux Jy·km/s, Test Mpc→m). SEXAGESIMAL-GAP
GESCHLOSSEN (2026-08-15): `src/sexagesimal.rs` (RA-Stunden/Decl-Grade → deg,
Space- und Colon-Separatoren, Signum, 5 Tests) + `src/bin/sexagesimal_compiler.rs`
(--source tevcat|magnetar → Flat-JSON ra/dec/dist/flux; CSV mit gequoteten
Komma-Feldern; TeVCat „2.0 kpc"→kpc-Parser). Dev-Beweise: TeVCat Crab
83,6329°/+22,0145° (360 Quellen, 1 Skip) und Magnetar SGR 1806-20
272,16390°/−20,41107°/8,7 kpc (31/31, 0 Skips) — exakt die Katalogwerte.
φ-Blöcke magnetar_flat.json + tevcat_flat.json (cmap, dist_scale
3,085677581e19, field flux em) + CI-Schritt in kernel_flatten.yml.
Verifikation 2026-08-15 (zweite Session): 5 Lib-Tests grün, Live-Läufe
360/1 Skip + 31/0 Skips exakt, Crab 83,6329°/+22,0145° und SGR 1806-20
272,16390°/−20,41107° — TeVCat-Flux-Einheit ist „Crab" (relativer Fluss,
1 Crab = 2,4e-8 erg/cm²/s bei 1 TeV) — Umrechnung in SI offen, Unit-Token
benennt was IST.
Das CDN-Asset cometels_flat.json entsteht beim ersten CI-Lauf (GH_TOKEN lokal
absent) — bis dahin manifestiert der Block 0 (ausstehend,
kein live-Fallback auf die .gz-Quelle möglich).
DCOM5-TEIL GESCHLOSSEN (2026-08-15): `src/bin/zip_range_extract.rs` (Zip-
Range-Extraktor: EOCD → Central Directory → Member-Payload per HTTP-Range,
curl + src/inflate.rs, 2 Tests) holt dastcom5/doc/README.txt (60 416 B) aus
dastcom5.zip (515 MB, 47 Einträge) — die früheren Offset-Rätsel („835-Header
+ 976-Null-Record ab 1811", „f64-Block nicht 8-aligned", „q=0,571 nirgends")
waren Fehl-Offsets: die Byte-Map fixiert EPOCH@16, MA@32, W@40, OM@48,
IN@56, EC@64, A@72, QR@80, TP@88 (f64), H@578, G@582, M1@586, RAD@702,
ALBEDO@722 (f32), SBNAM@760, DESIG@910, COMNAM@947 — H@578 ist nicht mal
4-aligned, der 976-B-Record ist byte-gepackt. `parse_comet_record` +
`comet_state_at` in src/dastcom.rs (2 Tests: Layout-Roundtrip,
Zwei-Körper-Radius am Epoch), `src/bin/dcom5_compiler.rs` (4972 Records,
2688 geschrieben, 2284 e ≥ 1 — 0 honored; Flat-cmap name/ra/dec/dist_au/H/M1,
Kepler zum Record-Epoch), CI-Schritt in kernel_flatten.yml, sources.φ-Block
(cmap + dist_scale, fields H + M1 em mag). Dev-Beweis 1P/Halley, 30 Records
(29 „1P" von 1633920,5 = 239 v. Chr. bis 2418800,5 = 1910 + SB441-N16):
1910-Record exakt (q 0,587208 = Literatur 0,5872, tp 2418781,68 =
1910-04-20,18, e 0,9673); M1 = 5,5 über alle Records (JPL-Total-
Magnitude), rad 5,5 km, albedo 0,04 — H = 99 (unbelegt) überall, der
M1-Kanal trägt die Messung. Das 1986-Apparitions-Record heißt im Katalog
SB441-N16 (tp 2446469,97 = 1986-02-09,47, e 0,96794, dist 28,8 au am
Epoch 1968) — der Record-Name ist, was er ist. CDN-Asset dcom5_comets.json
entsteht beim ersten CI-Lauf (ausstehend bis dahin).
```

**K04** Tycho-2-Katalog (em)
```
GESCHLOSSEN (2026-08-15): src/bin/tycho2_compiler.rs — VizieR I/259 tyc2.dat
(20 Teile, 2 430 450 Records) + suppl_1 (H-flagged, 17 588) + suppl_2
(T-flagged, 1 146; X-flagged-Main-Records ohne Position bleiben absent) mit
Hipparcos-Join I/239 hip_main.dat (Plx + Johnson-V für Suppl-Zeilen; Suppl-
Positionen J1991.25 → J2000 propagiert). 118 637 Sterne mit plx > 0 ins Bin
(36-B-Stride), 2 330 545 ohne Parallaxe 0 honored. Runtime: format
catalog_tycho + StarHash (Enclosure-Lemma, statisch auf Load-Epoch-Positionen,
vmax aus pm·d datenabgeleitet ×Φ, span-guard wie DASTCOM), Emission
10^(-0.4·mag) em, τ = ttl, extent 0 (Pixel-Scale-Softening). Test
build+query. Vega-Beweis: RA Δ0,0009″, Dec Δ0,01″, plx 128,93 vs. 130,23 mas,
dist 7,8 vs. 7,68 pc — ehrliche Katalog-Physik (Tycho-1-Reduktion). CI-Schritt
in kernel_flatten.yml. CDN-Asset tycho2_stars.bin entsteht beim ersten CI-Lauf.
Verbleibend: X-flagged-Reste — GESCHLOSSEN soweit möglich (2026-08-15):
`--tyc1 <tyc_main.dat>` (Tycho-1, I/239 — I/196 ist das HIC, nicht Tycho-1)
als dritter Positions-Fallback im tycho2_compiler (Key space-separiert,
J1991.25 → J2000): 5 618 X-flagged aufgelöst (+538 mit plx im Fallback-Bin).
103 845 X-flagged bleiben absent: GSC-only-Sterne ohne Tycho-1-Eintrag —
Positionen lägen im Guide Star Catalog (I/220, ~25 Mio) — offen.
Der Gaia-Merge ist
GESCHLOSSEN (K04b, 2026-08-15): TGAS (I/337 tgas.dat.gz, 2 057 050 Records)
als --source tgas im tycho2_compiler (Feld-Split an „|", Epoch 2015.0 →
J2000 propagiert), 2 025 673 Sterne mit plx > 0 ins Bin (72,9 MB), sources.φ-
Block tgas_stars.bin ersetzt tycho2_stars.bin, CI-Schritt gespiegelt.
Beweis HIP 13989 (HD 18560): plx 6,35 mas, G 7,991, dist 157,5 pc — DR1-
ehrlich (DR3 verfeinert auf 6,66). Befunde: Vega/Barnard absent, weil Gaia
DR1 bei G < ~3,5 sättigt bzw. pm > 3,5″/yr ausgeschlossen ist — 0 honored.
Der Tycho-2+I/239-Weg (--source tycho2) bleibt als Compiler-Modus.
SUPERSEDED am selben Tag (2026-08-15): der DR3-Merge (K04b-Welle, ARI
Heidelberg, Bailer-Jones + Hipparcos-Helle) ersetzt tgas_stars.bin als
Live-Kanal — siehe eigener Eintrag; TGAS-Modus = Historie/Fallback-Code.
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
| em | live: TNS (~20k Transienten), SWPC GOES/ACE/EUV/Kp, OpenSky-Geo-Altitude; Zeitreihen: NOAA GOES, HEASARC, PDS | K04 (Katalog); Kuration (Zeitreihen) |
| gravity | live: NOAA CO-OPS Tidenpegel (Fanout); JPL SSD komplett: DE442/441/440, Mond-Systeme, Kleinkörper, Sonden, GM-PCK, DASTCOM | K01 (Monde+CI), K02 (Binary-PCK), K03 (Katalog) |
| acoustic | live: METAR + NDBC-Buoys (Wellen); GONG/SOHO (Helioseismologie) | Kuration |
| seismic-body | USGS, IRIS, GFZ, PDS InSight/Apollo | Kuration (USGS bereits aktiv) |
| seismic-surface | live: USGS, INGV, JMA | Kuration |
| thermal | live: NOAA-CDO-Fanout (TMAX), AOML-Drifter, GML Barrow, Buoy-WTMP; DASTCOM H/Albedo + Yarkovsky-Listen; GOES-Thermal | K03 (Parameter); Kuration (Zeitreihen) |
| diffusion | live: PurpleAir, OpenAQ-pm25-Fanout, GML CH4/N2O/SF6, OOI-pCO2; NOAA SWPC, NASA OMNI | Kuration |
| advective | live: Waterservices-Rivers, OpenSky-Aircraft, Buoy-Wind; DSCOVR/SWPC (Solarwind), NOAA GFS | Kuration |
| electric | live: Swarm EFI (Vs) + FAC (IRC/FAC-Ströme) + FAST-MAGA-LR (em, NRT) via VirES-HAPI; TCT02 + TCT16 = Quarantäne (beide enden 2025-12-04, kein Live-Feed, swarm-diss nur CDF/ZIP) | GLM-Blitz (NetCDF-Reader fehlt), GIC-Netze (nicht öffentlich), Live-E-Feld-Stärke (kein Feed existiert) |

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
GESCHLOSSEN (2026-08-15): parse_sources lehnt die tote force-Direktive und den
3-Token-field laut ab (eprintln Refused — kein stilles 0). Die alte Grammatik aus
phi/port/queue/* und phi/research/batches/* migriert der --gold-Konverter
(Migration mit Lautsignal).
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
Parallelität (3er-Fenster + fanout_delay), Präsenz-Sortierung und
Präsenz-URL-Rendering (Operator-Präsenz statt Frame-Punkt als Zentrum,
{lat_min}-Bbox folgt dem Fenster) und OpenAQ-v3-Anbindung
(stations_flatten + stations_filter) sind implementiert (2026-08-15).
NOAA CDO (thermal) und OpenAQ pm25 (diffusion) leben im Fanout. Offen:
generisches Flatten über mehrere Ebenen.
```

**I04** CDN-Naming bei Param-Varianten — flacher Asset-Name fixiert
```
source_name_from_url mappt jetzt '/', '?', '&', '=' → '-' (flacher Name, kein
Slash im Asset-Namen). Vorher wurden '?'/'&'/'=' zu '/' — Query-URLs erzeugten
tiefe Pfade, gh lud nur den Basenamen hoch (Stray-Assets YES.json und
_271_d_27.json aus einem transienten Horizons-Block; plus Basename-Kollisionen
json.json/5000.json/-1.json/…). CDN-first-Konsum (fetch_one) und Mirror
(ci_mode) teilen dieselbe Funktion → Produzent/Konsument bleiben konsistent.
Template-URLs ({lat} etc.) bleiben is_fixed-gated (kein Mirror, Live-Fallback).
Offen: Stray-/Basename-Assets im Release ssd.jpl.nasa.gov löschen.
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
/home/johannes/projects/archive/archeology/ci/*.yml + secrets.template. (Der Ephemeriden-Teil der
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

### Source-Port — der eine Pfad

GESCHLOSSEN (2026-08-15, Struktur-Teil): Alle Source-Arbeit läuft über
`docs/SOURCE_PORT.md` — das selbsttragende Protokoll mit Zustandsmaschine,
Workflow-Prozedur (`--gold` → `test_backlog_batches_verify`-Sweep →
Disposition; kein neuer CLI-Modus), Referenz-Karte (bindend | nachschlagen |
SUPERSEDED) und Pfadkarte. Arbeitsfläche: `phi/port/` (queue/ 15 Korpora
getrackt, park/ Parser-Gap-Kandidaten, stage/ Konvertierungs-Ausgänge,
ledger.φ als DAS Zustands-Register, prompt.φ Port-Vorlage). Bestand:
`phi/research/`. Register: `phi/sources.φ` + `phi/dead_sources.φ`. Der
Sweep liest `phi/port/stage/*_converted.φ`; `phi/port/` ist fetch-only im
Mirror-Gate (wie research). Stale-Specs gebannert:
PARSER_EVALUATION_MATRIX.md + EXTRACT_TYPES.md (SUPERSEDED by
SOURCES_V2_SPEC.md).

Doku-Drift: Alle früheren `archeology/`-Referenzen in dieser Datei und in
docs/source_curation.md zeigen auf den Konsolidierungs-Zwischenstand vor
2026-08-15. Der Bestand liegt heute unter /home/johannes/projects/archive/
(archeology/ dorthin verschoben); die aktionierbaren Korpora sind nach
phi/port/queue/ kopiert (getrackt).

Offen (Detail in phi/port/ledger.φ, jede Zeile mit Zustand):
- Die Linse: source_scanner (src/bin/source_scanner.rs) + library.φ (746
  gewichtete b2find-Tags, Ratssitzung, Wort-Grenzen) + gate_weigh in
  src/force.rs (force_id_of/kernel_id_for_force/default_kernel_for jetzt in
  der Lib, A=A). Gewichtstabellen: phi/port/weights_*.txt. Sensor-Regel in
  SOURCES_V2_SPEC §2. Folgewelle: NASA-CMR-Keywords + GBIF-Tags downloaden,
  Library feinwägen; --port ersetzt --gold (port_mode/port_block), Test
  temp_port_convert_check liest queue/master.φ.
- Probe-Stufe: --probe mit LSK-Selbstholung, Auto-Draft (walk_json_probe),
  format free text + text_to_json (NDBC-Text), Headers gerendert,
  --fetchone für den Fetch-Vergleich. Wahrheitstest 2/7 (USGS 293 Samples,
  NDBC 7 Samples); Vergleich P1–P4 in probe_comparison.txt (Linse 57% vs
  2%). CI: .github/workflows/probe_sweep.yml (wöchentlich + manuell, 8
  Threads + 300ms Pacing, kompletter Korpus in 11m10s) probt batches/,
  lädt probe_all_survivors/probe_all_void/weights als Artefakte.
  ERSTE WELLE 2026-08-15: 4.485 Blöcke → 120 Survivors → Review: GeoNet-NZ
  + PIREP eingebaut, 68 Dispositionen (53 variant, 22 model-forecast …),
  coral + waqi geparkt (Hand-Kuration/freier Key). Nächste Welle: neue
  Kandidaten aus den Katalogen in batches/ nachrücken.
- Queue: 10 Untested-Korpora (14k/13k/15k/7k/2k/183l/astro/earth/exotic/
  candidate-staging), Gold 2572 + Recovery 1924 + 5701 Lost-Blocks (Join aus
  richest/params) — Port durch die Prozedur. Erster Port 2026-08-15: STAC
  sentinel2_l2a verifiziert mit Samples (element84, proj:centroid-Keys,
  eo:cloud_cover 28,4/10,2/19,5 — Content-Type-json-Fix + post_body-
  Template-Substitution im Sweep, fetch_raw_probe/fetch_raw_bytes_post);
  landsatlook leer unter Fixture (dokumentierter Pass); astro-Korpus:
  gemini decline + mast 404 in dead_sources.φ, 28 Blöcke → manueller Port
  (tap_compiler-Route, kepler_map, Einzelobjekt, Index-Positionen).
  --gold-Konverter trägt jetzt: post_body aus method/body, celestial
  (ra/dec/s_ra/s_dec) → cmap + at sun, benannte Keys → on earth, url-first-
  Emission, source-Split (ttl-Off-by-one behoben) — Test
  test_gold_convert_celestial_and_post.
- Bestand: 38 offene VizieR-Bulks, IRSA/GAVO/ARI/ExoArchive-Inventare,
  GCNS/MWSC, 8 VirES-Drafts, 32 ArcGIS-Drafts, 103 TerraPulse-Kandidaten,
  77 Archeology-Gaps, ESA-Kandidaten, FRB-Union, Arena/Foundation/Research-
  Schatz im Archiv.
- Nachlauf: VirES-Vollprobe (64 Drafts, Datei ABSENT) + DONKI-Familie
  (CME-Draft, Datei ABSENT).
- Park: Pegelonline, USGS-Geomag, GWOSC/GraceDB (Skymap), DSN, CENC,
  JMA-Quake (cod-String), SDSS-SkyServer.
- Rechecks: Argovis, sensor.community, environment.data.gov.uk, BGS-GIN,
  IRSA-Gator-CSV, ACTRIS, GTN-P, GONG, OceanNetworks, OMNIWeb, AstDyS,
  SuperMAG, WOUDC, AAVSO-VSX, ATNF-PSRCAT, TESS-Target-CSV,
  Tides&Currents-Datagetter, coastwatch/ifremer/emodnet-ERDDAP, PMEL,
  SERVIR/NDBC/Hurricanes-ArcGIS, AFAD tadas, MPC-Unterrouten,
  SWPC ace_mag_1h.
- Parser-Gaps: P02–P08, HapiFieldConfig, SignumFaltung, PostBodyMigration
  (method/body fällt im --gold-Konverter), GLMNetCDF; Kraft-Abdeckung
  acoustic/electric/thermal/advective/diffusion-Kuration.

### Curation & Quellen

- **TerraPulse-Katalog** (Crawl 2026-08-15, 25 Status-Seiten): 469 Quellen
  klassifiziert in `phi/research/agent_output/terrapulse_catalog.φ` —
  103 Kandidaten (NCEI-Stationsdaten/ISD, Kyoto-Dst, GFZ-Kp, USGS-Geomagnetism,
  NMDB-Neutronen, INTERMAGNET-HAPI, GWOSC-Gravitationswellen, IceCube, SILSO,
  WSPR/HamQSL, GLM-Blitz-NetCDF), 366 Decline (USDA/EPA/JRC-Inventare).
  Block-Erstellung + Verifikation offen; GLM braucht NetCDF-Reader (electric).
- **ESA/Geomagnetik-Backlog** (Linkliste 2026-08-15, ESA-Login vorhanden):
  `phi/research/agent_output/esa_geomagnetic_catalog.φ` — integriert: Swarm EFI
  (electric/diffusion/thermal), Swarm FAC (electric, IRC/FAC-Ströme), BGS-NGK (em).
  Kandidaten: Swarm TCT-E-Feld (keyless, VirES), VirES-Aeolus (advective,
  login), SMOS (diffusion, login), MERIS/SAR/Landsat (Raster, login).
  Modelle (IGRF/CHAOS/LCS/MF7/DTU-GVO) decline. ESA_USER/ESA_PASS-Platzhalter
  in .secrets.local angelegt.
- **INTERMAGNET-Fanout live** (2026-08-15, `144a990`): 154 Observatorien über
  GINServices-GetCapabilities-XML (`parse_stations_xml`-Fallback im Fanout —
  die Antwort ist XML, kein JSON). Archiv-Befund: 376 Dateien/25.009
  Vorkommen, aber nur 93 Codes je als URL-Block mit Koordinaten; die anderen
  61 existierten nur als Katalog-Einträge (intermagnet.txt ×3). Fanout:
  präsenz-sortiert, cap 40, BGS verlangt `Z`-Suffix an Zeitstempeln
  (start={week_ago}T00:00:00Z). Abgleich 2026-08-15 (intermagnet_abgleich.txt):
  live 154 / Archiv-Codes 94 / nur-archiv 2 (MCG, NAG — historische Codes).
  Offen: Ausbeute-Feintuning (best-avail-Aktualität variiert je Observatorium).
- **Re-Kuratierungs-Kampagne GESCHLOSSEN** (2026-08-15): `home_archiv_inventory.txt`
  (376 Dateien, 25.009 Vorkommen) vollständig dedupliziert
  (`home_dedupe_map.txt`: 195 unique, 22 Home-only-ohne-Repo-Gegenstück),
  --gold-konvertiert (15 gold_home_*_accepted.φ, 2.243 parse-fähige Blöcke),
  test_backlog_batches_verify-Sweep terminiert: ZERO neue Live-Quellen
  (2 neue URLs void) — der Home-Bestand ist eine Kopie der bereits
  terminierten Korpora. B2FIND (EUDAT) = Registry-decline, aber der
  tags=catalogs-Bestand ist vollständig indexiert: `b2find_catalogs_index.φ`
  (1.082 Records, 3 Seiten, alle organisation=IVOA; 995 CDS-VizieR,
  23 GAVO, Rest VO-TAP-Dienste; Provider-/Grind-Kandidaten-Blöcke).
- **Grind-Welle 2026-08-15** (Elf Buckets, alles in phi/research/agent_output/):
  · TerraPulse-103 Kandidaten vollständig geprobt (grind_terrapulse_a/b.φ):
    4 Drafts, Mauna-Loa-CO2 + Fireball-API offen (Signum-Faltung der
    Halbspären-Spalten fehlt — Parser-Gap, P-Liste).
  · ESA-Vollportal (grind_esa_full.φ): 0 neue keyless Punkte — VirES-HAPI-
    Katalog vollständig inventarisiert (174 Datasets, grind_vires_catalog.φ,
    8 Drafts CHAMP/GRACE/GOCE/CryoSat MAG/DNS/WND/TEC/KBR — Einbau offen).
  · NASA (grind_nasa.φ): DONKI-FLR + 3×FIRMS eingebaut; InSight tot
    (Mission beendet, sol_keys:[]), NeoWs decline (Orbit-Fit).
  · Host-geparkt (grind_host.φ): MSL-Mars-Wetter eingebaut; Pegelonline =
    Struktur-Befund (wartet auf Parallel-Session-Fanout); DSN/SatNOGS decline,
    CENC/JMA parser-def.
  · Rechecks (grind_rechecks.φ): PMEL-CO2 + EA-Flusspegel-Fanout eingebaut;
    SDSS-SkyServer-cmap (0.Rows) offen (Validierung).
  · B2FIND-Hints (grind_b2find.φ): 0 accepted — ICOS/TOAR key-needed,
    GEOFON quakeml-only, Rest Registry.
  · FRB (grind_frb.φ): FRBCAT1 tot (frbcat.org 000), CHIME via VizieR
    J/ApJS/257/59/table2 lebt (536, ohne z → 0 honored, tap_compiler-Route).
  · ArcGIS (grind_arcgis_index.φ + grind_arcgis_deep.φ): 440 + 620 Datasets
    indexiert, 131 + 358 Services geprobt, 32 Block-Drafts (17 + 15, thermal/
    seismic/diffusion/em/advective/gravity — Einbau offen).
  · ARI Heidelberg (grind_ari.φ): Gaia-Archiv TAP + GCNS (331.312 Sterne
    ≤100pc, dist-Parameter) + MWSC (3.006 Haufen) — Kompilat-Kandidaten
    (tap_compiler-Muster, Einbau offen); tap_index_ari.φ bleibt
    Parallel-Session-Artefakt.
  · DOMAIN_COVERAGE-Manifest (grind_domain_coverage.φ): alle 259 Hosts
    abgeglichen (22 live, 168 klassifiziert, 16 Kandidat, 52 offen geprobt)
    → gracedb/IOC/AGOS eingebaut.
  · Planetenarchive (grind_planetary.φ): MEDA-Feed lebt eingefroren
    (2024-04-27, eingebaut mit Zerfalls-Hinweis), Marsquake V14 ohne
    Live-Route (PDS-Archiv), Apollo-Seismik = Compile-Candidate (ASCII),
    LRO/Venus/Cassini = parser-def Archive, PDS-API = Registry-decline.
  · Archeology-Gaps (archeology_gaps_index.φ): 77 neue Kandidaten-URLs
    (AERONET, IERS-EOP, Fireball/Sentry, Xamin-TAP, GONG2, GIRO-Ionosonde,
    e-CALLISTO …) — nächster Grind-Brocken; MASTER.md/DOMAIN_COVERAGE.md
    als Manifeste registriert.
  · dead_sources-Migration der Grind-Dispositionen: erledigt (273 Einträge,
    mechanischer Merge aus grind_*.φ, Commit 4da86e4).
- **Hapi-FieldConfig-Befund** (2026-08-15): bei hapi-Extracts ersetzt
  `hapi_found` die field-Zeilen-Config durch synthetisch {kernel:0, force:0,
  tau:0} (main.rs ~7469) — die deklarierten kernel/force/tau der
  HAPI-Blöcke erreichen den Oszillator nicht. Bestehende Swarm/INTERMAGNET-
  Blöcke manifestieren (Kanal lebt), aber die Kraft-Labels sind tote
  Grammatik. Klärung in P-Liste.
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
    Strings, glat/glon als Float (galaktisch) — GESCHLOSSEN (Sexagesimal-Kompilat)
  · DSN `eyes.nasa.gov/dsn/data/dsn.json?t={unix}` — em; dishes{}.az/el/ws/sigs
    (Signalstärke); Positionen = statische DSN-Standortkoordinaten
  · SO2 Vulkanemission ArcGIS `services7.arcgis.com/WSiUmUhlFx4CtMBB/.../SO2datanew/
    ...f=geojson` — diffusion; GeoJSON-Punkte, properties mit VolcanoNumber/Emission
  · Schildkröten-Tracks ArcGIS `services6.arcgis.com/2DGR1sZBUvcPcd8Z/.../
    C_mydas_SSM/...f=geojson` — biotic; lat/lon/date/turtleid
  · Magnetar-Tabelle `www.physics.mcgill.ca/~pulsar/magnetar/TabO1.csv` — em cmap;
    CSV mit Period/Pdot/B/Flux/Dist — GESCHLOSSEN (Sexagesimal-Kompilat,
    RA „01 00 43.14"/Decl „-72 11 33.8" bestätigt + konvertiert)
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
  Endpoint, IRSA-Gator (TAP-Route aufgelöst 2026-08-15: --votable-Arm trägt
  das 995-Tabellen-Inventar; die Gator-CSV-Syntax bleibt offen),
  Exoplanet-Archiv TAP (aufgelöst 2026-08-15: pscomppars-Bulk live,
  Overrule der Grind-Disposition — siehe TAP-Pipeline-Eintrag), ACTRIS,
  GTN-P, GONG, OceanNetworks, OMNIWeb, AstDyS. SERVIR-SMAP/1km: Service
  entfernt (404).
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
  Geparkt (Parser-Gap, offen): Pegelonline (fanout liegt seit P09 — Block
  steht noch aus), USGS-Geomag (Komponenten-Timeseries), GWOSC/GraceDB
  (Position nur via Skymap), DSN (statische Dish-Positionen, Keyed Object),
  CENC (Keyed Object No1..NoN), JMA-Quake (Position im `cod`-String).
  Aufgelöst seitdem: TeVCat + Magnetar (Sexagesimal-Kompilat + Enrichment,
  live), ALFALFA (dist_scale Mpc, live), MPC-cometels (K03 geschlossen).
  Korrigiert zu decline (Spec §3): C_mydas_SSM (position-only, §3.9),
  PSF-Phytoplankton (Zählwerte, §3.2), SatNOGS (position-only), Active-
  Hurricanes-Sampler (Forecast-Track-Punkte, §Model), SIMBAD-TAP (VOTable
  trotz format=json).
- `phi/research/agent_output/batch_*_accepted.φ` (32 Dateien, neue Grammatik):
  **ausgeschöpft (2026-08-15)** — 629 Blöcke, 389 unique: alle klassifiziert
  (live / staging_verified.φ / staging_empty.txt). Integriert: ~48 Blöcke
  (SWPC GOES+EUV, Kp, ACE L1, OpenSky, Waterservices, CO-OPS-Tiden-Fanout
  gravity, 29 NDBC-Buoys, GML CH4/N2O/SF6, Barrow, AOML-Drifter, OOI-pCO2,
  BOM). Verbleibende Staging-Blöcke sind redundante Varianten (USGS-Historie,
  Per-Station-Tides superseded durch Fanout). Nächster Bestand:
  `archeology/sources/` — `--gold`-Konverter steht (mechanisch: force→kernel,
  tau=TTL/10, lat_key→lat, field_in→field, last_row→lastrow): 2573 Blöcke →
  1191 parse-fähig → `phi/research/agent_output/gold_converted.φ`. **Sweep
  terminiert (2026-08-15)**: 111 ok gestagt, 757 void diagnostiziert — das
  pre-CDN-Archiv liefert ZERO neue Live-Quellen (historisches Register, keine
  Live-Werte). Verifizierer-Dedupe keyed jetzt auf die substituierte URL
  (Template-Varianten kollabieren), Leerzeichen werden %20-normalisiert.
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

- TAP-Katalog-Pipeline (2026-08-15): `src/bin/tap_compiler.rs` manifestiert —
  `--index` (tap_schema.tables → phi/tap_index.φ: TAPVizieR-Inventar 64 148
  Tabellen), `--fetch-from` (ADQL TOP, FORMAT=json, --columns-Mapping → Flat-cmap),
  `--ci-mode`. Erster Bulk: FRBCAT (J/other/PASA/33.45/frbcat, 118 Zeilen,
  zHost→z, Beweis FRB 121102: DM 557, z 0,19273, Speak 0,4 Jy — exakt).
  Befund: TAPVizieR-sync kennt kein OFFSET/LIMIT (nur TOP). Paging gelöst
  (ARI, 2026-08-15): mag-Band-Adaption (--mag-bands: COUNT-Halbierung,
  Streaming pro Band) bei indizierten Mirrors; UWS-Async bleibt für
  nicht-bandbare Resultate (Queue-abhängig). Inventar-Welle
  GESCHLOSSEN (2026-08-15): IRSA (995 Tabellen → phi/tap_index_irsa.φ) und
  Exoplanet-Archiv (46 Tabellen → phi/tap_index_exoarchive.φ) inventarisiert;
  tap_compiler gewinnt dafür --index --votable (Feldnamen-Mapping),
  JSON-Top-Level-Array, \/-\b-\f-Escapes, --style mast (POST-Form),
  --cols-unquoted (Oracle-Identifier), --skip-null, HTTP-Status-Diagnostik.
  Exoplanet-Beweis-Bulk: pscomppars (sy_dist pc, pl_masse M⊕, pl_rade R⊕,
  6309 Zeilen, skip-null dist) — 51 Peg b: dist 15,4614 pc + ra/dec exakt,
  pl_masse 193,88 M⊕ (Archiv-Spalte). Overrule der Grind-Disposition
  „decline catalog (kein Live-Messwert)" (grind_nasa.φ Z.65, dead_sources
  Z.3669): seit der Katalog-Welle (K03/K04) sind statische Messwert-Kataloge
  Kanal-Spezies — Gaia-DR3-Präzedenz (1,84 Mio statische Sterne, live);
  Planeten-Masse ist das Exo-Analogon des DASTCOM-GM-Gates (gravity).
  HEASARC: tap_schema.tables antwortet
  0 Zeilen (Sync lebt, Schema leer) — die xamin-Route selbst trägt: FORMAT=
  csv/json ungestützt, FORMAT=votable liefert BINARY-Streams (kein TABLEDATA),
  FORMAT=text = Pipe-Tabelle (tap_compiler gewinnt --text + --csv +
  --votable-td dafür). xamin-Schema-Befunde: hmxbcat ohne dist-Spalte
  (historische Query veraltet), fermilpsc ohne redshift-Spalte; swiftgrb
  trägt z: 872 Records (COUNT bestätigt = Vollkatalog), 338 mit redshift —
  Beweis GRB 090423: z 8,23 + ra/dec exakt, BAT-Fluence 6,25e-7 erg/cm².
  MAST: tap-sync 404 (nginx) — MAST bleibt key-needed (dead_sources.φ).
  VizieR-Bulk-Welle (2026-08-15): BZCAT5 (VII/274/bzcat5, 3561 Zeilen,
  skip-null z) — Beweis 5BZQ J1229+0203 = 3C 273: z 0,158 (Archiv rundet,
  Literatur 0,15834), FR 54992 mJy (55 Jy — exakt die 1,4-GHz-Flussdichte),
  ra/dec exakt. CI-Schritte für bzcat5.json + swiftgrb.json in
  kernel_flatten.yml, sources.φ-Blöcke (z → Hubble, cmap). Offen: weitere
  Tabellen-Bulks aus phi/tap_index.φ (39 genutzte VizieR-Tabellen, davon
  BZCAT5 erledigt), Gaia-Archiv (Parallel-Session-Grind).
- GAVO-Grenze (2026-08-15): dc.g-vo.org/tap inventarisiert → phi/tap_index_gavo.φ
  (271 Tabellen, ~130 Schemata). tap_compiler um \uXXXX-Escapes erweitert
  (GAVO-Deskriptionen). Zahler: gedr3dist.main = Bailer-Jones-Gaia-DR3-
  Distanzen (r_med_geo pc, Join über gaia-Lite per source_id) — die
  K04b-Folge-Quelle; antares.data/antares10.data = ANTARES-Mirror (5 921
  Zeilen) — Recheck erledigt: Counts+Position → Decline (dead_sources.φ).
  GAVO-sync kennt ebenfalls kein OFFSET/LIMIT; GAVO-litewithdist ist
  unindiziert (Band-COUNTs 20s+) — der Merge lief über ARI (indiziert).
  GAVO-Async-Queue hing am 15.08. (2 Jobs PENDING) — Recheck am selben Tag:
  Probe-Job o234nz95 blieb 90 s PENDING, Queue hängt weiterhin.
- K04b-Welle GESCHLOSSEN (2026-08-15): tap_compiler gewinnt UWS-Async-Client
  (submit/phase/results), VOTable-Reader (FIELDref-Self-Closing-Fix — der
  stille None-Killer), --votable-Sync-Modus, --order, --epoch (pm-Propagation
  → J2000, pmra/pmdec-Drop), --join (Alias-Join), --mag-bands (adaptive
  Halbierung über COUNT, streaming pro Band), --star-bin (36-B-Records,
  plx = 1000/dist_pc, pm 0 — statisch), Dict-Zeilen (ARI-Shape). Der Merge:
  ARI Heidelberg (gaia.ari.uni-heidelberg.de, Sync-Cap 100k, mag-Indizes
  0,2 s/COUNT — GAVO-Queue hing, aber ARI trägt gaiaedr3.gaia_source_lite
  ⋈ gaiaedr3_complements.distances) → 1 837 214 Sterne mit Bailer-Jones-
  Distanzen (G 1,94–11,5, 66 MB Bin), ersetzt TGAS in sources.φ. Beweis
  HD 18560: BJ 177,8 pc (TGAS-DR1 sagte 157,5 — DR3 verfeinert). Laziness
  behoben (2026-08-15): --source bright im tycho2_compiler (Hipparcos V<1,94,
  J1991.25→J2000 — 45 Sterne: Vega 7,76 pc, Sirius 2,64 pc, Alpha Centauri
  1,347 pc) + --union-bright im tap_compiler → 1 837 259 Sterne in EINEM
  Kanal, kein Fallback-Block. Befunde:
  ARI-lite cutt bei G<1,94 (physikalisch: Gaia sättigt dort — geschlossen
  via Hipparcos-Union, kein Fallback); GAVO-litewithdist ist der
  vor-gejointe, aber unindizierte Zwilling; ARI inventarisiert (203
  Tabellen → phi/tap_index_ari.φ);
  GAVO-sync/ARI-sync ohne OFFSET/LIMIT — UWS-Async bleibt für
  >100k-Einzelresultate. CNS5 (5 909 Sterne, alle mit gaia_edr3_id) ⊂ DR3
  — kein separater Block, die lokale Blase manifestiert im Merge.
- FRB-Recherche (2026-08-15, „absent ist erst nach Recherche absent"): das
  Inventar liefert 2 FRB-Tabellen — FRBCAT 2016 (118, 18 z) + A&A 693/A279
  (2025, 24, 23 z, DM-Halo-Zerlegung, tausc) — beide live, Überlapp ≈ 0.
  Darüber hinaus existieren FRBCAT1 (frbcat.org, ~840 FRBs, ~50 z) und der
  CHIME-Katalog (536, ohne z) — außerhalb von VizieR. Offen: Union-Merge mit
  TNS-Namens-Normalisierung (FRB121102 ↔ FRB20121102A) + frbcat.org-CSV als
  Quelle (sexagesimal_compiler-Muster).
- TeVCat-Enrichment (2026-08-15): --join-z (BZCAT4, J/A+A/495/691/bzcat4,
  3149 Blazare, 2551 mit z) + --join-dist (Green snrcat J/A+A/612/A1, 282 SNR,
  86 mit Dist) im sexagesimal_compiler (Nächster-Nachbar im Winkelabstand).
  Befund: TeV-Zentroid vs. optische Position ~2′ (Mkn 421: 30″-Join verfehlt,
  3′-Join trifft, z=0,03). Ergebnis: 67 von 360 TeVCat-Quellen mit z, 78 mit
  dist. tap_compiler erweitert: @n-Index-Mapping (VizieR-Virtualspalten
  _RA_icrs nur via SELECT * greifbar).
- Enrichment-Matrix komplett (2026-08-15, „alle Kataloge"):
  · TGAS: --hip-Join im tgas-Modus — 160 der 31 217 plx-losen Zeilen via
    Hipparcos wiederhergestellt (Rest trägt keine HIP-Nummer — absent).
    (Historie: der Live-Kanal ist seit der K04b-Welle der DR3-Merge.)
  · TeVCat: z-Join (BZCAT4) + dist-Join (Green) + dist2-Join (B/psr/psr,
    PSRCAT-Mirror, 2536 Pulsare, 2348 mit Dist, <=30″) — Vela 0,29 kpc und
    Geminga 0,25 kpc exakt; 67 z + 79 dist von 360.
  · FRB: FRBCAT + A279 live; frbcat.org (FRBCAT1, ~840, ~50 z) von hier
    nicht erreichbar (Connect-Refused) und CHIME nicht in VizieR — als
    externe Quellen dokumentiert, Union-Merge offen.
  · cometels/DASTCOM/ALFALFA/Magnetar: komplette Distanzen im Katalog selbst.
  Offen: Name-basierter Fallback-Join, GSC I/220, Gaia-DR3-Merge.

---

### Validation

- `--verify` CLI existiert (URL-Erreichbarkeit); lädt noch keine Quellen.
- `force` und der 3-Token-`field` lehnt `parse_sources` laut ab (Refused, P01
  geschlossen); `field_in` migriert der `--gold`-Konverter, `pos` trägt keinen Arm.
- Test-Limit der Curation über 200 Blöcke hinaus erhöhen; 6 Rest-FAILs sind
  Daten-Artefakte (docs/SOURCE_PORT.md §5).
- **Kanonisierung 2026-08-15:** `phi/sources.φ` (171 Blöcke), `phi/dead_sources.φ`
  (1.056 unique) und `.secrets.local` (46 Keys) aufgeräumt und sortiert:
  Blöcke alphabetisch nach URL, kommentarfrei, Direktiven in kanonischer
  Reihenfolge, 1 Toter-Duplikat entfernt. File-Regeln jetzt in
  `docs/concepts/SOURCES_V2_SPEC.md` §1.0 — jeder Einbau folgt der Spec,
  keine Docstrings in den φ-Dateien.
- **Vollständigkeits-Audit 2026-08-15** (keine Deckel, keine Auswahl, keine
  Ranglisten — jeder Crawl-Auftrag läuft bis zur Erschöpfung):
  · B2FIND tags=catalogs: VOLLSTÄNDIG — 1.082 Records, alle indexiert
    (b2find_catalogs_index.φ).
  · EO-Gateway: VOLLSTÄNDIG — 251 Einträge (grind_eogateway_index.φ).
  · VirES-HAPI-Katalog: Inventar komplett (grind_vires_catalog.φ, 174
    Datasets); die Vollprobe (174/174 disponiert, 64 Drafts) lag als
    Agent-Ergebnis vor, die Datei ist jedoch ABSENT (Schreibverlust) —
    Nachlauf in Blöcken offen (gleiches Muster wie ArcGIS-Blöcke).
  · NASA-DONKI-Familie: 10/10 Endpoints disponiert (CME-Draft, 9 Decline),
    Ergebnis-Datei ebenfalls ABSENT — Nachlauf in einem Block offen.
  · ArcGIS: LÄUFT inkrementell in Blöcken (grind_arcgis_bNN.φ, 6 Keywords
    pro Block, alle Seiten bis Relevanz-Erschöpfung, jeder neue
    FeatureService geprobt und disponiert). Lauf 1 (22 Keywords, Seite 1)
    und Lauf 2 (47 Keywords, 2 Seiten) waren gedeckelt — die Blöcke
    übernehmen jetzt die vollständigen Paginationen.
- **Source-Gate (2026-08-15):** `test_live_sources_extract` über das ganze
  Register: 76 ok / 2 void, jede Void mit datentragender Verifikation.
  Dispositionen:
  · Safecast: OK nach LSK-Volltabelle in der Fixture (die Minimal-Fixture
    kannte prä-2017-Epochs nicht → alle Zeilen fielen — Fixture-Fix, kein
    Source-Fix).
  · DONKI-FLR: der classType=X-Filter des APIs wird NICHT angewendet
    (liefert C2.4/C4.1 mit) — der Regex extrahiert trotzdem nur X-Flares;
    heute kein X-Flare → 0 honored (historisch X1.1→1.1 verifiziert).
  · AGOS → Quarantäne: Fenster-Routen count:0, der Katalog endet 2022-02-05
    (all-quakes 17.380 Events/5,5 MB, significant 1.533 M5+) — Live-
    Ingestion eingestellt; Kompilat-Kandidat über den CDN-Weg.
  · opensky → Quarantäne: 429 bei zwei Läufen (ADSB deckt den Kanal).
  · TCT02/TCT16 → Quarantäne: swarm-diss-Crawl (grind_swarm_diss.φ) belegt:
    TCT16 existiert als CDF/ZIP, endet ebenfalls 2025-12-04 — kein
    Live-Nachfolger, kein JSON-Zugang auf swarm-diss.
  · EO-Gateway-Index (grind_eogateway_index.φ): 251 Einträge (91 Missionen,
    115 Campaigns, 45 Instrumente), API-Route
    earth.esa.int/eogateway/api/search/full_text, 6 Grind-Kandidaten
    (smos-diss/oads, scihub/dataspace, science-pds.cryosat, eumetsat-romsaf).
  · EA-Fanout: Keys gegen Live-Antworten verifiziert (notation/lat/long,
    items.value) — Runtime-Fanout-Lauf offen (Test überspringt Fanout
    designbedingt).
  · Backlog-Test-Reparaturen im Working Tree (Template-Keyed-Dedupe,
    Letzter-Block-Flush, Limit zählt nur Fetch-Blöcke, LSK-Volltabelle):
    warten auf den Abschluss der laufenden main.rs-Überarbeitung der
    Parallel-Session (deren Zwischenstand kompiliert nicht — cur_force) und
    werden mit deren Commit mitgezogen.
- **Secrets-Befund (2026-08-15):** Keys sind vollständig — `.secrets.local`
  liegt im Repo-ROOT (46 Keys inkl. NASA_API_KEY, FIRMS_MAP_KEY, ESA_USER/
  ESA_PASS) und wird via resolve_asset (CWD-relativ) geladen. DONKI-FLR und
  FIRMS fetchen 200 mit Key (manuell verifiziert). GitHub-Secrets-Parität
  hergestellt: 19 lokal-neue Keys (ESA_USER/PASS, ARBIMON, GBIF, MOVEBANK,
  PURPLEAIR, SUPERMAG_PASS, TNS, HERMES, PROXY, TRANSIT511, WILDLIFE_INSIGHTS,
  tedp_*) nach GitHub synchronisiert — 46 lokal = 46 GitHub (tedp_* werden
  von GitHub uppercased). Der frühere „marker absent"-Befund war
  Probe-Mechanik: --probe lief aus /tmp/opencode ohne die Asset-Datei im
  CWD — bei Probe-Läufen außerhalb des Repo-Roots `.secrets.local`
  verlinken. Die verbleibenden Probe-Declines der Grind-Einbau-Sektion sind
  Probe-Artefakte (synthetisierte Configs: HAPI-/CSV-/Regex-Blöcke werden
  vom Probe nicht über die deklarierten Extracts verifiziert — CSV braucht
  format-csv-Konvertierung, hapi die hapi-Arm-Extraktion, regex den
  regex-Arm; alles im Runtime-Pfad vorhanden).
- **TNS-Header-Fix (2026-08-15):** resolve_secret trennt JSON-Braces nicht von
  Marker-Braces — `tns_marker{"tns_id":4474,...}` wurde als ein Marker-Key
  gelesen → UA kollabierte zu „tns_marker" → wis-tns.org 403. Fix: Header als
  Einzel-Marker `header user-agent {TNS_UA}` (sources.φ) + `.secrets.local`-
  Eintrag `TNS_UA=tns_marker{"tns_id":197991,"type":"bot","name":"omegaflow_archivar"}`
  (registrierter Marker) und `TNS_API_KEY` gesetzt.
- **Probe-Verifikation Grind-Einbau (2026-08-15):** --probe mit LSK auf die
  Grind-Einbau-Sektion: VirES-HAPI-Syntax (id=/start=/stop= und dataset=/
  time.min= sind beide gültig — 400er waren time-outside-range der
  Datensatz-Abdeckungen), IOC-Keys sind Lat/Lon (korrigiert), gracedb-
  superevents-Struktur bestätigt, AGOS heute leer (count:0), EA-Fanout vom
  Probe-Design ausgenommen ({station}). Offen: Extract-Verifikation der
  MSL/MEDA-field-Pfade end-to-end (test_live_sources_extract deckt nur die
  ersten 200 Blöcke — Limit erhöhen, dann Grind-Einbau einschließt).
- **VirES-Lag-Befund GESCHLOSSEN (2026-08-15):** FAST-Produkte haben ~10 h Lag
  (stopDate ≈ now−10 h) — die {hour_ago}/{now}-Fenster lagen außerhalb der
  Abdeckung (400 time-outside-range). Fix: EFI/FACATMS/FAST-MAGA
  nutzen jetzt `start={yesterday}T12:00:00Z&stop={yesterday}T13:00:00Z` mit
  ttl 86400 (Fenster endet im Abdeckungsbereich, ttl faltet den Lag ehrlich).
  TCT02 bleibt Archiv-only (Abdeckung endet 2025-12-04, 400 außerhalb der Abdeckung).
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

- **Healthcheck-Befund + Fix (2026-08-15):** Der Healthcheck lief per
  `*/5`-Cron + Push-Trigger und fetchtete JEDEN Lauf alle Quellen
  (cache_fresh-Gate galt nur fürs Mirroring, nicht für research) —
  21 Runs hingen/stauten, API-Quota starb. Fix manifestiert:
  (1) ci_mode wendet den TTL-Gate (cache_fresh) auf ALLE festen URLs an
  und schreibt den Cache auch im Healthcheck-Pfad, (2) Template-URLs
  (`{lat}` etc.) werden übersprungen und gezählt (presence-gated — der
  Runtime gehört das, nicht der CI), (3) actions/cache persistiert
  /tmp/archivar_cache über Runs (Restore+Save), (4) Cron auf `0 */3`
  (TTL-ausgerichtet statt 5 min), (5) verify-Jobs ohne --release
  (Build-Minuten). Prinzip: der Healthcheck prüft, ob sich eine Quelle
  geändert hat (TTL-Ablauf → Fetch), nicht ob eine API gerade antwortet.
- **Workflow-Entkopplung (2026-08-15):** push und Healthcheck sind getrennt —
  `ci.yml` (push: cargo check + cargo test, `RUSTFLAGS: -D warnings`) vs.
  `healthcheck.yml` (nur schedule 0/3h + dispatch; cd+sources zu EINEM
  verify-Job verschmolzen — ein Cache-Key, kein Race; Mirror/Probe nie auf
  push). `pages.yml` baut die 5 Cross-Release-Builds nur noch bei Änderungen
  an src/**, static/**, Cargo.toml (paths-Filter) — Doku-Pushes kosten keine
  Runner-Minuten mehr.
- **Upload-Regel (2026-08-15, verbindlich):** CDN-Uploads laufen ausschließlich
  über die CI (`--ci-mode` in den Compilern, kernel_flatten.yml). Manuelle
  Uploads sind keine Option — Befund: der git-Remote-Token hat weder
  releases- noch actions-Rechte (404/403), die Struktur erzwingt CI. Der
  Remote-URL-Token gehört rotiert und auf credential-helper/SSH umgestellt.
- `refresh-protected-data.yml` (Python inline) → Rust (Befund oben unter Infrastruktur).
- Ephemeriden-Flatten läuft seit K01 in `kernel_flatten.yml` — seit 2026-08-15 als
  Drei-Job-Struktur: `index` (voll rekursiver --index-Crawl → phi/sources_index.φ +
  docs/reference/KERNEL_INDEX.md, Artifact statt Mid-Run-Git-Kopplung, Bot-Commit)
  + `bodies` (needs index, nutzt das Artifact: --fetch-from --systems
  planets,jupiter,saturn,mars,uranus,neptune,pluto --ci-mode → CDN-Assets
  ephemeris_{body}.bin, --clobber; Body-Manifest in sources_index.φ;
  horizons_compiler --ci-mode; dastcom_compiler --ci-mode) + `catalogs` (läuft
  parallel zum index, kein needs: alle Katalog-Kompilate, continue-on-error je
  Katalog + Issue bei Void). Cron `17 5 1 * *` (05:17 UTC, kollisionsfrei zum
  0/3h-Healthcheck) + workflow_dispatch. Failure-Issues je Job.
- **Flattener-Befund (2026-08-15):** Runs #3/#4 scheiterten beim Upload-Step
  (OMEGAFLOW_TOKEN auf dem 5000/h-API-Limit — die Push-Healthcheck-Flut hatte
  die Quota gesprengt). Run #5 (nach Quota-Reset + Queue-Clearing) kam weiter:
  SPK-Flatten + Horizons + DASTCOM luden durch (15 Monde + dastcom_asteroids.bin
  im CDN), dann kippte der Sexagesimal-Step: unquotierte `;` in den
  `--columns`-Argumenten (Bash-Split → tap_compiler bekam nur `name:Name`).
  Fix manifestiert: alle `--columns` gequotet + Monolith in bodies/catalogs
  gesplittet (Katalog-Fehler blockt nie wieder die Bodies).
- **Kernel-Generations-Befund (2026-08-15):** Der Flatten schrieb 32 SPK-Körper,
  aber 17 Monde fehlen, weil `select_system` nur die höchste Basis-Generation
  lud: sat480 (nur Barycenter 699) statt sat441 (alle Saturn-Monde), jup387xl
  (8 Monde, kein himalia) statt jup341, nep105 statt nep097. Die sechs
  Aug-11-Assets (titan/tethys/rhea/enceladus/dione/triton) sind Stale-Reste der
  sat441/nep097-Generation. Fix manifestiert: `moon_carriers`-Selektion
  (jupiter→jup365, saturn→sat441, neptune→nep097) — exklusiv, keine
  numerische Nachrang-Füllung; die Nummer ordnet nicht mehr nach Abdeckung.
  Zweiter Befund im selben Lauf: sat441 (2021) trägt 12 Saturn-Monde, aber
  NICHT atlas/epimetheus/janus/pandora/prometheus — und sat427 (2018) trägt
  sie auch nicht (verifiziert per CDN-Lauf). Die fünf laufen daher über die
  Horizons-Liste bodies_stable (610/611/615/616/617), wie himalia (506).
  Alle 17 Monde liegen seit dem Lauf 31883186409 im CDN (74 ephemeris-Assets
  gesamt).
- **Flatten-Speicher-Befund (2026-08-15):** `SpkFile::from_daf` parst alle
  Segment-Payloads upfront — sat441 (661 MB) + jup387xl (1,4 GB) + ura184
  (387 MB) + nep105 (210 MB) ≈ 3,3 GB Dateien → >7 GB Runner-RAM → Run #7
  SIGTERM (143), Run #8 „operation canceled" im Extraktions-Silentium. Fix
  manifestiert: moon_carriers exklusiv (jup365 statt jup387xl, kein
  sat480/nep105 — die System-Barycenter trägt de721 bereits). Offen
  (strukturell): Segment-Payloads lazy laden statt upfront, sonst wächst die
  Ramlast mit jeder Kernel-Generation.
- Katalog-Kompilate seit 2026-08-15 im selben Job: cometels_compiler
  (cometels_flat.json), dcom5_compiler (dcom5_comets.json),
  tycho2_compiler --source bright (bright_stars.json),
  tap_compiler Exoplanet-Bulk (pscomppars, --cols-unquoted + --skip-null dist
  → exoplanets.json), tap_compiler DR3-Merge via ARI (--mag-bands +
  --star-bin + --union-bright → dr3_stars.bin), tap_compiler FRBCAT + A279,
  sexagesimal_compiler (Magnetar + TeVCat mit BZCAT4/Green/PSRCAT-Joins).
  Die CDN-Assets entstehen beim
  ersten Lauf nach dieser Welle — bis dahin manifestieren die Blöcke 0
  (ausstehend, dokumentiert je Block).
- Quota-Befund 2026-08-15: `--verify phi` lud rekursiv die research-Batches
  (27k+ Quellen) und mirrorte jede per gh release upload — das Kontingent
  (5000/h) starb strukturell. Fix manifestiert: (1) Mirror nur für das
  kanonische Register (phi/sources.φ, `--verify phi`), (2) research-Dirs werden
  nie gemirrort (Fetch-only-Gesundheitscheck im sources-Job), (3) Frische-Gate =
  ausschließlich lokale TTL (`cache_fresh` auf /tmp/archivar_cache) — keine
  CDN/API-Metadaten-Abfrage, (4) `cdn_asset_fresh` entfernt. API-Last des
  Verify-Takts: ~0 (Uploads nur bei abgelaufener lokaler TTL der kanonischen
  Festquellen). K01-Befund: SSB-Kette muss Kernel-übergreifend klettern
  (jup365 hat (501,5), (5,0) lebt in de442) — `state_ssb_multi` fixiert;
  Beweis: 8 Jupiter-Monde mit 6.848 Granulen, Ganymed mit stype-4-Nutation.
- Das Python `refresh.yml` im sources-Repo (Kataloge/TAP/Gaia, Release v1.0) bleibt
  bis I02 auf Python — K01-Grenze. Entscheidung 2026-08-15: Abschaltung nach
  Verifikation der Rust-Katalog-Kompilate im kernel_flatten-catalogs-Job
  (ein Produzent pro Asset).
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
