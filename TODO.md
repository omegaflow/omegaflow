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
  OMNI↔GOES-Schnittmenge bleibt leer (stopDate 06.08.);
- nobel_probe_corona v2 (Multi-Force-TE): der Probe läuft auf der
  skalaren Pairwise-TE (`transfer_entropy_lag`) — die bedingte
  Multi-Force-TE (alle Kräfte im Phasenraum, DAG über alle Paare
  und Verzögerungen) ist pending;
- Desktop-Fork (GTX 970): der Lauf mit 30-Jahres-Daten braucht die
  GPU (1664 CUDA-Cores) — die O(n²) × Surrogate-Kosten gegenrechnen
  (~80–90 min gemessen);
- 90-Tage-Archive für den Lauf (Bz/GOES/GONG): GONG steht (31
  Jahre); Bz/GOES hängen am GOES-30d-Archiv-Block (pending, unten)
  und am OMNI-Ingest-Verzug.
Kurations-Befunde: NGDC-NetCDF (GOES-30d) trägt 404 → fehlt-
Registratur, kein Block; `1/cm3`-Alias ergänzt; HAPI-Reihen über die
Identitäts-LSK; Radio ↔ GOES trägt keine Aussage (n = 15, n-Schwelle
30); Laufzeit gemessen ≈ 80–90 min — vor dem nächsten Lauf die
O(n²) × Surrogate-Kosten gegenrechnen.
GOES-30d-Archiv-Block bleibt pending (kein lebender Kandidat);
bis dahin trennt der OMNI-Ingest-Verzug (stopDate 06.08.) OMNI↔GOES —
Schnittmenge leer, im Protokoll fehlt.
Solar-Akteure — pending (2026-08-20): der g-Moden-Oszillator
selbst (0 honored — einzeln nie gemessen); der CDAWeb-Live-Block
(SOLO_L2_RPW-TDS-SURV-STAT, SN_RMS_E V/m, 16-s-Kadenz) — der
Publikations-Lag (~5 Monate, stopDate 2026-03-25) lässt das
{hour_ago}-Fenster heute leer (0 honored), sobald die NASA
erweitert, fließt der Kanal. Pending: Berkeley-VSC-Feld
(Baum-Forschung: kein VSC-Produkt auf research.ssl.berkeley.edu/
data/psp/data/sci/fields/l2|l3, kein HAPI-Dataset mehr —
PSP_FLD_L2_VSC 1406; AEB@0/@1 = HK), Wind/WAVES wav_h1
E_VOLTAGE_RAD2 (1994–2021, Tages-CDFs — der Parser steht, die
Ernte ist ein eigenes Atom), GONG L 31..200 (CI --lmax 200) +
mparam-Eigenfrequenzen für freq/bin_width, GOLF (Medoc 000),
PSP-DFB = em-Spektralkanal (Force-Gate-Urteil, kein
electric-Katalysator), TRACERS-SPK (das SOC publiziert noch keine
SPICE-Kernels — solar_akteure_probe.φ), Huffman/Adaptive-CDF
(parser-gap in src/cdf.rs). CI: kernel_flatten sun job (gong
monthly, rpw einmalig mit Asset-Guard, bodies job trägt
--omega-g). Befunde selbsttragend: phi/pipeline/research/
agent_output/solar_akteure_probe.φ.
ZeilenFilter implementiert (2026-08-20): `where <key> <value>` an
first/last — die Extract-Varianten tragen Option<(String,String)>,
row_matches (String exakt, Nummer numerisch), jfirst_where/jlast_where;
extract UND extract_series teilen das Prädikat; malformed where + where
auf field werden laut refused. euvs-7-day-Block zurück mit
solar_euv_flux_304/284_wm2 (where line 304/284 — der mgii_index-Record
ist ausgeschlossen, kein W/m2-Label auf dem Index), xrays trägt
`where energy 0.05-0.4nm` (Band deklariert statt positionsabhängig);
der nobel probe erntet X-Ray/EUV über extract_series aus dem Block
(harvest_block) — der hardcodierte Filter ist tot. Live-Befund:
n = 10078 (X-Ray) / 10024 (EUV-304/284), Kadenz 60 s.
Faden-B-Einheit (2026-08-21): der xrays-Block trägt jetzt beide
Bänder (zweites last, where energy 0.1-0.8nm — die Datei liefert
beide, das lange Band 1–8 Å war unerntet); F10.7 (Penticton) neu
eingetragen: f107_cm_flux.json, first flux where frequency 2800,
sfu, τ=3600 (Prozesswissen — Stunden-Skala, nicht ttl/10); die
RTSW-1m-Dateien sind absteigend sortiert — mag/wind von last auf
first gestellt (last trug den ~24 h alten Record). Force-Gate je
Kanal geführt: em für X-Ray/EUV/F10.7 (die Messung IST Strahlung),
Bz/Bt/Plasma über ihre Feld-Signatur wie eingetragen. Live-Befund:
test_live_sources_extract führt alle fünf Solar-Blöcke ohne void,
nobel probe unverändert (X-Ray n=10078). Offen: der Fluss der
Kanäle in den probe_ring der GPU (src/mathematikerin.rs:1410) —
der kausale Pfeil F10.7 ↔ X-Ray wartet auf diese Einheit.
A=A-Reparatur (2026-08-21): der solar-radio-flux-Block ist aus dem
Register entfernt — path 0.details.0.flux war eine Chimäre (je Fetch
eine andere Station/Frequenz, Frame on earth 0 0 0 fabriziert) →
blocked_sources.φ parser-def nested-filter (where filtert nur die
oberste Array-Ebene, kein Filter auf details[].frequency). Die
2695-MHz-Reihe erntet der nobel probe weiter (harvest_radio); F10.7
trägt f107_cm_flux.json.
Luminositäts-Atom — benannte Grenzen (pending): die Ankerung
modelliert Isotropie (die Röntgenemission ist richtungsabhängig —
abgeleitet, nicht gemessen); die Energie-Bänder der Partikel-Dateien
sind positionsabhängig (where-Filter pending); mag-Felder (alerce
u.a.) konvertieren nicht (logarithmische Achse — pre-existing skip
am Anker, pending).
Zeitbasis-Atom (2026-08-20): `src/kernels/naif0012.tls` ist die
Geburtsurkunde der Zeit — eingebettet (include_str!), der Zeit-Arc wird
bei Konstruktion mit der Tabelle gefüllt, der Boot hängt nie am Netz;
das stille LSK-Gate ist gestorben (nur der vergiftete Mutex ruft laut
die Verweigerung: „the time base is absent — the process refuses to
fabricate a dead field"). Tests: die eingebettete Tabelle parst,
leap_at(now) == 37 s, system_now liefert ohne Fetch; sense_membrane
liefert das Sonnen-Sample bei floor 0 (der Zellpfad trägt kein
Floor-Gate — die Sonne bootet die Referenz, die unbounded-Sterne
folgen). Pflicht: bei einer angekündigten Schaltsekunde wird die Datei
im selben Commit erneuert; der Runtime-Fetch (sources.φ:1514)
überschreibt die Tabelle im Speicher und deckt die Zwischenzeit.
Pending: freq/bin_width-Träger im Block-Grammar (die Band-Slots bleiben
0.0 = Punktquelle, 0 honored), where-Auto-Draft im Probe, p/cm3 des
Probe-Klassifikators (unkonvertiert, kein Block nutzt es).
Atom 10 (2026-08-20): die TE-Maschine lebt wieder — der native
Echo-Pfad (Permeabilitäts-Schleife) läuft auf Takens-eingebetteten
Zuständen (`topological_te_phase`, dim 3, order 3). `find_mi_lag`:
2×2-Midpoint-Histogramm, erstes lokales Minimum ab lag 3, rs/Φ-Schranke —
kein Minimum → keine TE. `embed_series` bleibt vorwärts (das
Geometrie-Instrument); die TE-Bedingung ist rückwärts gespiegelt
(x_t, x_{t−τ}, x_{t−2τ}) — Registerzeile: der Vorwärts-Zustand trägt die
Zukunft in der Bedingung (Leakage, falsche Stille); die Geometrie ist
richtungsagnostisch. Silverman auf die Varianz der eingebetteten
Vektoren skaliert (σ² = mean ‖z−z̄‖², ein isotroper Kernel je Faktor).
Surrogat-Integrität: jedes phasenrandomisierte/Block-Bootstrap-Surrogat
trägt seine eigene MI-Suche und eigene Einbettung (kein τ →
übersprungen, nie 0.0; < 2 gültige → keine Aussage); die x-Seite bleibt
fixiert. `permutation_entropy` → Option<f64> (Registerzeile: 0.0 ist
null-echt — vollständig geordnete Reihe; null Motive ist fehlt; Ties →
Fenster übersprungen, nie lexikografisch gebrochen; Sortieren der Motive
= O(n log n)). PE-Gate: 2⁴-Ring der eigenen PE-Geschichte des Treibers,
bewaffnet ab 2³, Sprung ⇔ |pe − mean| > 2·sd → Richtungs-Entscheidung und
TE-Ziel gehalten, Atem über den Selbstmessungs-Zweig; die Baseline ist
Live-Daten — durch ein anhaltendes Regime adaptiert sie (der Übergang
feuert). Die naive Shuffle-Null der Schleife ist ERSETZT
(broken-null-control.md bleibt gültig: der skalare Pfad
`transfer_entropy_lag` ist unangetastet, der Probe behält seine
lag-Matrizen). Der historische degenerierte Guard (2τ ≥ rs → τ = 1)
fiel — ein fabrizierter Default; Fenster zu klein → keine TE (0
honored). Offen (registriert): der multivariate Silverman-Exponent (−1/(d+4)) als
Dichte-konsistente Form (der Unterschied wird von der Surrogat-Schwelle
absorbiert); das PE-Delay der Pipeline läuft auf 1 (der historische
Shader-Rhythmus, benannt — die Standalone-Funktion trägt delay offen).
Atom 11 (2026-08-20): te_compute läuft — der topologische TE-Kern rechnet
im WGSL-Compute-Shader (ein Thread je Serie: xs + ys + 10 Surrogate;
MI-Suche, Silverman-Bandbreiten, vierfache KDE-Summen, PE). Die
MI-Schranke max_lag = n/Φ kommt als Uniform von der CPU (die f64-Formel,
keine f32-Division); die phasenrandomisierten Surrogate bleiben CPU-f64-FFT
(byte-identisch zum broken-null-control-Register, Upload 12 KB je Tick);
mean + 2σ reduziert die CPU in f64 (die historische Grenze); src/te.rs
bleibt die kanonische CPU-Referenz samt Tests (der skalare Pfad ist
unangetastet). Verpasst die Readback die Deadline, erntet der nächste
Tick die ausstehende Map (eine Map im Flug, kein Doppel-Map, kein
Livelock — der Verdikt trägt dann die Daten des vorigen Ticks) und der
benannte Zustand wechselt nur an der Grenze (te verdict present /
readback pending / real series invalid / fewer than two surrogates).
Offen (registriert): das Zeilen-parallele Re-Shape (ein
Thread je t statt serieller m²-Schleife — trägt das Ringwachstum); der
WGSL-FFT der Surrogate (f32-Null) als benannte Alternative; der
f32-EPS-Floor der MI-Entropie weicht vom f64-Wert ab (die
Surrogat-Schwelle absorbiert).

OMEGAFLOW_HIDDEN (2026-08-21): hidden = fensterlos, klanglos,
regungslos. Das Fenster läuft unsichtbar (with_visible(false), kein
Fokus-Griff); das silent-Flag im NativeApp torbt die ω-Loop-Sends an
acoustic/seismic (src/mathematikerin.rs); AcousticOscillator und
SeismicOscillator werden nicht gestartet (kein PCM auf stdout, keine
Serial-Vibration) und der Relay-Radiator nicht gebaut (src/archivar.rs).
Der Operator wird nie ungefragt penetriert — visuell, akustisch, taktil,
relay — nicht; der ω-Loop läuft dabei exakt wie produktiv, nur still.
Der Render-Pfad ist geteilt (src/mathematikerin.rs render): der
Probe-Compute-Pass läuft immer, Surface/Render-Pass/present nur sichtbar —
so füllt sich der Probe-Ring auch headless mit ~1 Hz, und die
Maschinenzeile `φ window:` (stderr, 1 Hz) trägt te/thr/tau/pe/state/focus/keys
als lesbaren Zwilling des HUD.

Nicht-em-Farbtod (2026-08-21): die Membran ist keine Kamera — nur em
(Licht) trägt Farbe (`color_lut_rgb`); die anderen 8 Kräfte krümmen das
Feld (lum, Transferentropie), sie leuchten nicht. `hsl_to_rgb` ist tot;
der Objekt-Bias, der den Schirm gelb färbte, fällt.

Asteroiden-Langbogen (2026-08-21): ceres + vesta tragen den
sb441-n16-Langbogen (8001 v. Chr.–9000 n. Chr., JD −1200525,5–5008242,5,
256-Tage-Raster, Grad 17, 24.253 Granulen je Körper, Roundtrip ≤ 9,4 m;
GM aus gm_Horizons.pck; de441.bsp ist der Sonnenträger — de442.bsp trägt
nur ~1100 Jahre, gemessen). Offen: sb441-n373 (14,13 GiB) — der
Split-Weg (spk_split, Stream, nie full-load; Handover
handover-2026-08-21-n373-split-langbogen.md, Schritt 0 vermessen:
373 = 343 + 30 KBO laut IOM-Tabelle 1, Eris 2136199/Haumea 2136108/
Makemake 2136472 bestätigt drin, Apophis/Bennu/Encke bestätigt nicht
drin, n373s = Kurzfenster 1550–2650 n. Chr., GM aus IOM-Tabelle 1 —
gm_Horizons trägt abweichende TNO-Nummern); die 14 weiteren n16-Körper
ohne sources.φ-Block; die 12 Körper ohne SPK-Segment tragen die
12-Monats-Fenster weiter (fehlt, nicht null).

HDF5-Erntekarte (2026-08-21, Handover
handover-2026-08-21-hdf5-fits-ernte.md): recherchiert — HDF5 gehört fast
ausschließlich zur Erdbeobachtung + SSI + Gravitationswellen. Drei
MUSS-Quellen (Auth-bereit, `EARTHDATA_EDL_TOKEN` liegt vor): Black Marble
VNP46A1/VJ146A1 (em, Nachtradianz, LAADS; Suomi-NPP endet 1. Nov. 2026),
VIIRS/MODIS Surface Reflectance VNP09/MOD09 (em, der echte Blue-Marble-
Kanal), LIGO GWOSC Strain (gravity, HDF5, offen). Galaxie/Universum messen
in FITS, nicht HDF5; deren HDF5 (IllustrisTNG/EAGLE/CAMELS/FLAMINGO) ist
Simulation → decline no-physical-force (Eintrag mit verifizierter URL in
der Ernte-Session). Ernte hängt am HDF5-Reader (paralleles Atom).

FITS-Reader (2026-08-21, Handover handover-2026-08-21-hdf5-fits-ernte.md):
pure Rust, std-only, Gold-Standard wie netcdf.rs/cdf.rs/hdf5.rs — der
fehlende Schlüssel für Galaxie/Universum-Messungen (Gaia, SDSS, 2MASS,
Pan-STARRS, DES, Planck, JWST). Header 80-Byte-Cards, HDU-Extensions,
BINTABLE, BSCALE/BZERO, WCS. Crates nur als Format-Referenz.

Fetch-Ketten-Atom (2026-08-20, Handover handover-fetchkette.md): die vier
Regressionen sind geschlossen — und drei echte Pipeline-Bugs lagen darunter.
Gate-Reparatur: der Fetch-Dispatch rechnet die physische Reichweite
`dispatch_reach` = signal_reach(force, advection, ttl·64) — das eine
Ausbreitungsgesetz der Query-Gates, körper-unabhängig, kein
Ephemeriden-Rennen; der `r == 0`-Skip ist tot (der Anker im
Presence-Fenster trägt die Fenster-Reichweite selbst); eine Quelle ohne
Ausbreitungsgesetz wird benannt verweigert. `extract_fields` liest
ProfileMap. Compiler-Reparatur: `omegaflow::media` trägt die MEDIA-Tabelle
der Python-Ära (23 Körper, 5 Slots vs/vp/vsseis/ath/dd; die 6. Spalte vad
starb schon 93b6f8e — Advektion ist per-Feld); beide Compiler schreiben
die echten Werte, körperlose Sonden tragen die Pad [0.0; 5].
Boot-Reihenfolge: `anchor_uses` (Oszillator-Zahl je Frame-Körper, kein
Hardcode) teilt Download-Bootstrap UND Load-Gate; Sonden/Monde warten, bis
alle Anker-Körper im Archiv stehen — Vertragstest:
test_anchor_bodies_have_ephemeris_sources. Presence ab Tick 1: der
Boot-Send lebt in main_flow (Ruhezustand SSB, Reichweite = Pixelmaßstab),
der fabrizierte ∞-Anker bleibt tot; `OMEGAFLOW_HEADLESS=1` fährt den
Archivar ohne Membran (kein Fenster, kein GPU). Die drei gefundenen
Pipeline-Bugs: (1) `body_barycenter_position` + Rotationssuche waren
lineare Scans über 36020 Granulate — mit 147k wartenden Kanälen im
Pending-Drain hing der ω-Loop Minuten pro Tick (das schwarze Feld);
jetzt binäre Suche per partition_point (die Granulat-Abstände sind
unregelmäßig 32/26/… — ein naiver O(1)-Index war falsch, der Test
test_barycenter_lookup_finds_irregular_granule trägt das). (2) Der
netcdf-Zweig holte die URL ungerendert ({week_ago}-Platzhalter gingen roh
an curl — Argo 400) — render_source_url läuft jetzt auch dort. (3) curl
ohne `-g` (Globoff): die gerenderten `box=[[…]]`-URLs starben mit exit 3 —
-g jetzt in allen curl-Basen. CDN-Gate: `cdn_fresh` prüft gegen
`ttl.max(CI_REFRESH_S=300)` — die CI-Kadenz als Boden, sonst ist die
Beschleunigungsschicht für ttl-60-Quellen tot. Fenster-Fokus:
`.with_active(true)` + focus_window-Nachforderung über die ersten 60
Frames (WMs ignorieren Anfragen vor dem Mapping) + ESC über logische UND
physische Taste. Gates: cargo check 0/0 (vier Kombis), cargo test 213/213
lib + Bins; Live-Boot headless: body 5 / 1,7M Samples, api 72 Quellen /
106.707 Samples — die Kette fließt wieder. Offen (registriert): die
Solar-CDN-Assets sind veraltet (Manifestator hat sie noch nicht erneuert)
und SWPC live ist langsam — bis zum Asset-Refresh fließen sie gedrosselt;
der ESC-Bug braucht den
Repro vom Operator, falls er nach der Fokus-Reparatur bleibt.
Fetch-Sturm-Reparatur (offene-atome Atom 2, 2026-08-21) — ERLEDIGT: die
Fetch-Überlappung ist geschlossen — `begin_fetch`/`settle_fetch`
(In-Flight-Guard: ein laufender Fetch blockt die Neu-Dispatch; vorher
feuerte jeder Tick die laufenden Fetches erneut — der dastcom-Read-Void
kehrte sogar sendelos zurück = unbegrenzter Re-Dispatch, jetzt sendet er);
der 2ⁿ-Void-Backoff kühlt tote Quellen exponentiell (Kappe 2⁴ → max
ttl/Φ·16); gezählt werden nur Netz-Voids (`fetch_ok` im FetchResult) —
write/read/extract-Voids zählen nicht (0 honored: fehlt ≠ null-echt, die
Diagnose nennt das Gesetz: fetch void → ttl/Φ·2ⁿ, write/read/extract →
ttl/Φ); Fetch-Budget 2³ je Tick (max 8 in-flight im Live-Zyklus, die
Parallelität des curl-Bootstrap). Gates: cargo check 0/0 (vier Kombis),
238 lib-Tests grün (2 hdf5-Fehler der Parallel-Session benannt),
Hidden-Lauf 150 s stabil (29 api / 309k Samples, dastcom 1× geladen). Schwarzes
Fenster trotz vollem Feld (Operator-Befund, 2026-08-20): die Kette
lieferte 23 042 recs ans Fenster, aber der Fragment-Shader rechnete die
Pixel-Luminanz ohne die scale²-Kompensation — der Meterraum-Kernel
(1/(d²+scale²), scale = grid 2³¹) drückte jede Sample-Helligkeit auf
~1/scale² gegen die Rohwert-Referenz → rgb ≈ 0, schwarz, während der
Probe (Rohsummen) Kraftwerte zeigte und omega_total die Kompensation
bereits trug (×scale² am Ende). Reparatur: die rgb-Luminanz trägt die
gleiche scale²-Kompensation (FIELD_WGSL, eine Zeile) — die Feld-Splats
leuchten wieder gegen ihre Referenz. Die Refs der Luminanz (Median der
Rohwerte) bleiben unangetastet. Gates: cargo check 0/0 (vier Kombis),
cargo test 218/218 inkl. naga-Validierung.

## Archivar — Architektur

- Dreiteilung — WAHR (2026-08-20, Ratsurteil 5×WAHR, Schnitt ausgeführt):
  main.rs (16 344 Zeilen) aufgelöst. Ein Archivar (omegaflow::archivar, ~16,4k:
  Feld-Structs + Radiator-Trait, ω-Loop, spatial cache, CI-Modes, sense_*,
  Radiatoren, Tests, Parser build_curve_set/build_planet_set + take_*),
  eine Mathematikerin (omegaflow::mathematikerin: FIELD_WGSL + HUD + Fenster +
  Emitter), relay (Kind, cfg browser_relay), main = Einzeiler. Der Doppelname
  (omegaflow::archivar × mod archivar) ist gestorben; das verbotene Wort
  „membrane" fiel zweimal (MEMBRANE_WGSL → FIELD_WGSL,
  membrane_wgsl_validates_offline → field_wgsl_validates_offline). Gates grün:
  cargo check 0/0 (default + browser_relay + gamepad), cargo test 159/159 lib
  + alle Bins, die drei Datenvertrag-Tests unter neuem Namen, Live-Boot
  (Intel HD 520 Adapter). main.rs-Diff = 16 343 Streichungen + 2-Zeilen-Trunk
  (im Vorschlag benannt). Cargo.toml unberührt. Bestand: sense_membrane heißt
  weiter sense_membrane (Funktionsname, außerhalb der Rename-Reichweite) und
  ein eprintln trägt „membrane window" — benannt, nicht verschluckt.
- wgpu/winit in der lib — bewusst getragen (2026-08-20, Operator-Entscheidung):
  omegaflow::archivar bleibt std-only (Modul-Ebene); die Crate trägt wgpu/winit.
  Jeder Bin-Kaltbuild zahlt den GPU-Baum einmal (inkrementell danach), kein
  Runtime-Preis. Die std-only-Grenze ist eine Etage gesunken — registriert.
- feature-gate `gpu` — eigenes Atom, pending: `pub mod mathematikerin` als
  #[cfg(feature="gpu")] + Co-Gate der main_flow-Verdrahtung (crate::
  mathematikerin::-Stellen PresenceFrame/EMOscillator/KineticRadiator)
  + Feature-Propagation zum Default-Bin — kein Ein-Zeilen-cfg, ein Faden
  durch die ω-Loop.
- Membran-scoped Cache statt Blockuniversum (2026-08-17): der Archivar lädt
  flache Katalog-Assets komplett in den Spatial Hash — das ganze Feld im
  Speicher. Die Membran braucht nur die Hülle um die Presence (dilatierter
  Suchradius). Korrektur 2026-08-20 (Prüf-Rolle gegen LOST_CONCEPTS §9 +
  ERAEN): „HEALPix"-Tiling war Drift — Geospatial-Tiles wurden entfernt
  („No meshes. No grids. Every raw point makes us truer"), ersetzt durch das
  Enclosure Lemma. Die Wahrheit ist der 3D-Spatial-Hash `(i64,i64,i64)` des
  Lemma selbst (Kimi K3, ERAEN §63): Fang = Chunking entlang DIESER Zellen,
  keine 2D-Himmelsprojektion,   kein neuer Raster. Der verlorene Pendant war
  §12 Causality-Prefilter (Signal-Lichtkegel) — WIEDERBELEBT 2026-08-20:
  `signal_reach` aus dem force_type (Wellen 0/1/2/3/4/8: c, 343, 6000,
  3000, Advektion, 1.0; diffusiv 5/6: √(2·D·age), D = 0.3/0.05), Cut vor
  `motion.at()` in query_hash + query_asteroid_hash (`age > ttl·2⁶` oder
  Distanz am Anker > reach + extent + pad; unbekannte Kraft → refused,
  kein Default). Das Lemma dilatiert jetzt mit Bewegung UND Signal.
  Deckt auch: tess_lightcurves.bin ~500 MB lädt heute ganz.
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
- OOM-Befund: ein Lauf, dessen GPU-Thread beim Pipeline-Bau panikte, lief
  als Rumpf weiter (Archivar + Audio, 3,2 GB) — der tote GPU-Thread ist
  nicht der tote Prozess.
- SPK-Segment-Payloads lazy laden statt upfront (strukturell — sonst
  wächst die Ramlast mit jeder Kernel-Generation).
- Asteroiden-SPK-Flatten-Pass (Familie spk im Index registriert).
- K06 EOP: Erdrotation (Polbewegung, UT1−UTC) für präzise Erd-Stationen;
  Konzept: docs/concepts/iau-2000-eop.md (72-B-Orientierungsmatrizen
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
  (src/mathematikerin.rs:3859, [f64; 4]) — die Geschwindigkeit `[,vx,vy,vz]`
  fehlt.
- Audio-Ausgabe nativ = rohe Samples nach stdout (Pipeline-Ausgang;
  im Log erscheint Datenmüll) — bewusst oder ein eigener Ausgang.
- Der Subpixel-Anlauf (Rgba32Float, 9 Mio Messzellen) wartet auf einen
  nicht-aufgeblähten Wiedereinstieg; die Messung lebt in
  docs/surveys/messpunkt-verteilung.md.
- Bewegungs-Vereinheitlichung Atom 4 (2026-08-20, nach Atom 3): die
  Asteroiden-Route starb — AsteroidHash + query_asteroid_hash + p0-Vorräte
  sind getötet; Asteroiden sind gewöhnliche Samples im inertialen
  SpatialHash mit motion: Motion::Kepler { rec: Arc<AsteroidRec> }
  (GM- und Radius-Sample teilen den Record via Arc; source = Body wie die
  Planeten-Kanäle, der Wiederaufbau speist sie je Zyklus aus
  archive.asteroid_samples — die Body-Retention-Sperre trägt sie ohne
  Verdopplung). build_asteroid_samples(bytes, ttl) liefert
  (Vec<Sample>, Option<Arc<OccluderSet>>) — der cadence-Parameter aus der
  Atom-Vorgabe hatte keine Rolle in der Funktion und blieb draußen
  (0/0-Gate); anchor_p0/vmax/amax via law_bounds (per-Sample statt
  global). Abweichung zum alten Record: Radius-Sample trägt kernel_id 1
  (Atom-Vorgabe; der alte query_asteroid_hash schrieb 0). sense_* findet
  Asteroiden via query_hash(&buf.inertial). Offen (Atom 5):
  StarHash + query_star_hash sterben — Sterne werden Samples mit
  Motion::Spherical { rec }.
- Bewegungs-Vereinheitlichung Atom 5 (2026-08-20, nach Atom 4): StarHash +
  query_star_hash + die browser_relay-Zell-Vorräte (cell_size, p0-Vorrat,
  build_epoch-Parameter) sind getötet — Sterne sind gewöhnliche Samples im
  inertialen SpatialHash mit motion: Motion::Spherical { rec: Arc<StarRec> }
  (Anweisung „Arc wie bei den Asteroiden"; der frühere TODO-Wortlaut trug
  „{ rec }"). build_star_samples(bytes) liefert (Vec<Sample>, Vec<StarRec>)
  — epoch 0.0 (J2000), ttl = tau, val = flux, force 0 (em), kernel 0,
  extent 0, source = Body; anchor_p0/vmax/amax via law_bounds; der
  Signalkegel-Gate trägt die Sterne ohne Eingriff (reach ≥ pad deckt jedes
  Fenster-Stern, c·age liefert den Dilatations-Rand; der cadence-Parameter
  der Vorgabe blieb draußen — build_star_samples braucht ihn nicht, 0/0).
  Erweiterung: Sample trägt neu color_index (Wire-Slot 21) — der generische
  query_hash-Pfad schrieb dort 0.0, query_star_hash trug die BP−RP-Farbe;
  ohne das Feld wäre die Browser-Sternfarbe eine Fabrikation gewesen
  (0 honored; API-Samples tragen 0.0 = weiß, abwesend). MAX_SAMPLES
  1<<21 → 1<<22: gültige Stern-Records (1,19 Mio von 1,67 Mio — 476 661
  tragen keine positive Parallaxe und bleiben wie bisher dunkel) +
  Asteroiden (1,56 Mio Records) überschreiten die alte Kappe; ohne
  Erhöhung kippten die ältesten Samples — genau die Sterne (epoch 0.0).
  Buffer.star_records:
  Arc<Vec<StarRec>> blieb für sense_deep + transit_factors (gestorben in
  Atom 8, 2026-08-20), der Wiederaufbau
  speist die Stern-Samples je Zyklus aus archive.star_samples
  (Body-Retention wie Asteroiden). Benannt: das native Fenster trägt die
  Sterne jetzt über &buf.cache (sense_membrane) — vorher nur der
  Browser via query_star_hash; das Bin trägt 145 744
  Parallaxen-Artefakte (f32-MAX/Denormals — parse-gültig, landen in
  fernen Zellen, nie abgefragt; Bestand aus der StarHash-Ära).

- Atom 6 (2026-08-20, nach Atom 5): `bodies` ist tot — das Universum ist
  ein Hash. Buffer trägt einen einzigen universellen SpatialHash `cache`
  in absoluten ICRS-Koordinaten. Gestorben: die HashMap<String, SpatialHash>
  je Körper, das Feld `inertial`, die relative-Frame-Subtraktion
  `relative_frame_position` (law_bounds rechnet p0/v/a jetzt absolut via
  motion.at — ein Bodensensor der Erde ankert bei ~1.5e11 m vom SSB,
  i64-Zellindizes tragen das), der anchor-Parameter von query_hash
  (qf = center), sense_buffer (der browser_relay-Pfad ruft sense_membrane —
  beide Körper waren zeilenidentisch), die bodies-Schleifen in
  sense_membrane, StderrRadiator, relay /station und /field. Die
  Anchor-Semantik der Körper (gravity_manifest via anchor_body — gestorben
  in Atom 7, 2026-08-20) und body_barycenter_position bleiben — der
  Eintrag des Körpers ist die Motion, nicht der Cache. Registriert
  (keine Fabrikation, offene
  Eigenschaft): die Zellgröße hängt am span aller Samples — die Sterne
  (~1e19 m) heben cell_size auf ~1e16 m, das Sonnensystem teilt wenige
  Zellen; die Broadphase läuft über den in_box-Fallback und den
  dist2_anchor_p0-Filter (Enclosure bleibt konservativ, kein Sample geht
  verloren; die feine Zellauflösung der alten Körper-Hashes existiert
  nicht mehr). Auflösung 2026-08-20 (Atom 8): die Sterne verlassen den
  bounded-Teil (extent ∞) — die Zellgröße schrumpft zurück auf
  Sonnensystem-Maß. 0/0-Gate in allen vier Feature-Kombinationen
  (default, browser_relay, gamepad, beide), 165 Tests grün.
- Atom 7 (2026-08-20, nach Atom 6): `gravity_manifest` ist tot — die Form
  gehört zum Anker, nicht zur Messung. Gestorben: die Funktion
  gravity_manifest, body_pole_at, der `.radius`-Kanal von body_channels
  (ein Planet ist nur noch seine Masse GM) und die Multipol-Zweige
  (J2/J4/Abplattung) in beiden WGSL-Feldern (nativ FIELD_WGSL:
  osc_field/osc_flow/source_bound/source_contrib; Browser fieldShader:
  osc_field). Das Gravitationsfeld ist ein reines Feldgesetz (1/d² bzw.
  1/d je Kernel) — das Feld trägt keine Form. Die Wire-Slots pole_x/y/z,
  j2, j4, r_eq bleiben im 24×f64-Protokoll (Datenvertrag unverändert,
  meta-Pack unverändert) und tragen für force_type 1 konsequent 0.0
  (0 honored); pole_x trägt weiter das Tolman-z für em. Der
  kernel_extent-Gravitationspfad (extent = radius_m) bleibt — die
  Reichweite des Ankers, keine Form. Offen (späteres Atom): die
  Okklusion stirbt zugunsten der Feld-Absorption. — ERLEDIGT in Atom 8
  (2026-08-20): die Okklusion ist tot, die Feld-Absorption ist pending
  (siehe Atom 8). Gates: cargo check
  0/0 (vier Feature-Kombinationen), cargo test 165/165 lib + alle
  Bins, naga-Validierung grün. Der Schnitt landete in zwei Commits:
  die Rust-Hälfte (archivar.rs: query_hash, gravity_manifest,
  body_channels) trug der vorangegangene Commit (benannt „Atom 6",
  8f57a25) — der WGSL-/Register-Schluss folgt separat.

- Atom 8 (2026-08-20, nach Atom 7) — „Die Vereinheitlichung des
  Sensoriums (Der Schall entscheidet, nicht das Trommelfell)": die
  Objekt-Render-Pfade sind tot. Gestorben: die Okklusion (WGSL
  occlusion() + barriers-Buffer, Ephemeriden-Radius-Barrieren +
  Asteroiden-Barrieren, OccIndex/OccReport/occ_dir_cell/occ_hits_ray/
  occulting_barriers, HUD „okkl"), die Stern-Sprites (sense_deep,
  pack_stars, star_contrib/star_assign/star_cull, star_vbuf/star_count,
  deep-Zahl im HUD), die 2D-Gitter (tile_cull, source_bound,
  tiles/star_tiles-Buffer, 16×16-Kacheln, cull_n) und die
  Transit-Scheibengeometrie (Extract::TransitMap + „transitmap" +
  tname/tra/…-Schlüssel, build_planet_set, transit_factors,
  disc_intersection_fraction, der sources.φ-Block „format transit").
  Der Fragment-Shader fs iteriert linear über field/props via
  source_contrib — eine kurze Liste; die VP-Uniform trägt kein
  star_count/barrier_count mehr (9×vec4f = 36 f32; expose_ex.x =
  Belichtungs-Offset, .y = Response-Epoche). Sterne sind gewöhnliche
  Punktquellen-Samples im unbounded-Pfad des SpatialHash (extent = ∞ als
  Reichweite-Marker; wire_extent schreibt 0.0 auf den Draht — die
  Punktquelle hat keine Ausdehnung, dort ist 0 die Wahrheit). Die Diode
  (query_hash, unbounded-Pfad): Stufe 1 val-Gate vor motion.at
  (|val|·(1+z)⁻⁴ < ft_ref·2^(−expose_offset) → skip; ft_ref == 0 →
  dunkle Diode — der Shader zeigt ohnehin lum 0), Stufe 2 Quergate nach
  motion.at (t² = d² − (fwd·(p−center))²; val_eff/(t²+scale²) <
  ft_ref·2^(−off)/scale² → skip; der Fold spiegelt fold_eff). Der Schall
  trifft das Trommelfell quer — die radiale d²-Formel des Auftrags war
  ein Physikfehler (kein Stern hätte je die Schwelle passiert) und ist
  benannt; die gewählte Form ist die Operator-Entscheidung (Val-Domäne +
  Quergate). Folge-Wahrheiten: nahe der Sonne passiert kein Stern (die
  Diode, die die Sonne hört, hört keinen Stern); im tiefen Raum
  relaxiert die Live-Referenz (ft_ref), achsennahe helle Sterne
  erscheinen als weiche Glows beim Blick-Sweep. Der Browser-Relay trägt
  die dunkle Diode (floor = [0;9]) — Ausgabe identisch zu vorher; das
  Browser-Sternfeld bleibt pending (eigene Belichtungsrampe). CurveSet
  (tess_lightcurves, emit_curves) lebt. Der Lichtkegel-Horizont
  (signal_reach, c·age ≈ 8 kpc) begrenzt jetzt auch die Sterne —
  registriert (vorher sense_deep ohne Lichtkegel). Pending:
  Feld-Absorption (Transits/Okklusion als Feld-Eigenschaft — der
  absorption-Slot lebt im Protokoll). Gates: cargo check 0/0 (vier
  Feature-Kombinationen), cargo test 162/162 lib, naga-Validierung.
  Sprach-Durchgang (im selben Commit, Operator-Weisung 2026-08-20): die
  deutsche Sprache ist aus dem Code gewandert — „sterne/planeten" →
  stars/planets, alle verbliebenen deutschen Diagnostik-Zeilen,
  Kommentare, CLI-Dokumente (--summarize-Vorlage, Nobel-Probe-Bericht)
  und Bezeichner (Pfeil/fenster/extrahiere/sekunden_*) sind übersetzt;
  AGENTS-Sprachdoktrin erfüllt. Nur Eigennamen tragen Deutsch.
  Semantischer Feinschliff (Operator-Weisung): `SampleSource::Body` →
  `SampleSource::Ephemeris` (die Herkunft ist das Ephemeriden-Kompilat,
  nicht ein Körper — BodyEphemeris/BodyProperties bleiben Eigennamen der
   Datensätze); alle erklärenden Docstrings starben — es bleiben nur
   Physik-Herleitungen, 0-honored-Gesetze, Pending-Register und
   Byte-Layout-Kontrakte (53 Zeilen).

- Atom 9 (2026-08-20, nach Atom 8) — „Die synästhetischen Aktuatoren
  (Der Geräte-Bias stirbt)": die Ausgabe-Surfaces tragen keine
  Gerätenamen mehr — ein Aktuator ist ein Oszillator, den das Feld
  anregt, und übersetzt das ganze 4D-Feld in seine eigene Dimension.
  Gestorben: `AudioRadiator` (der Synthesizer: phase-Array, feste
  Frequenzen 2^(3+i) Hz, tanh-Gain, 44.1-kHz-Pacing,
  AUDIO_SAMPLE_RATE/AUDIO_BUFFER_SAMPLES), `SerialSurface` (der
  Debug-Log: formatierte lum-Textzeilen), `MathematikerinRadiator`
  (der Gerätename für das Fenster), die Frame-Typen `AudioFrame
  { omega, mx, permeability }` + `SurfaceFrame { lum }` und
  `window_median_extent` (kein Konsument mehr — 2 Tests zogen mit).
  Geboren: ein `PresenceFrame { omega: [f32; 9] }` — die neun
  Kraftwerte der Presence, wie sie der GPU-Probe aus Superposition
  und retardierter Laufzeit rechnet — auf zwei Kanälen
  (acoustic_tx/seismic_tx). `AcousticOscillator` (acoustic): die
  zeitliche Σω-Folge wird roh als f32-LE-PCM an stdout geschrieben —
  ein Frame = ein Sample, die Abtastrate ist die Kadenz des Feldes
  selbst (~1 Hz Probe); keine künstliche Oszillation, keine feste
  Frequenz — das Feld IST die Welle. `SeismicOscillator` (seismisch,
  Trait `KineticRadiator::vibrate`): Σω als rohe f32-LE-Intensität
  (4 B/Frame, fester Stride) direkt an den seriellen Port — die
  Vibration ist die Summe der Kräfte an der Presence. `EMOscillator`
  (em): das Fenster übersetzt das gesamte 4D-Feld (alle 9 Kräfte) in
  eine 2D-Verteilung von em-Emissionen — em via `color_lut_rgb`
  (BP−RP→Teff→RGB; `temperature_to_rgb` starb in Atom 8, keine
  Auferstehung), die anderen 8 Kräfte via `hsl_to_rgb`-False-Color.
  Konsequenz benannt: `field_permeability` (die
  Transfer-Entropie-Maschine) verliert mit dem Synthesizer ihre
  Strahlungs-Konsumenten — die Aktuatoren schreiben roh, kein
  Scaling; die TE-Maschine läuft als HUD-Messung (`perm`) weiter, die
  Strahlungs-Bindung (Permeability → Aktuator) ist pending und kehrt
  zurück, wenn Atom 10 (Takens/MI) die Maschine neu gebaut hat — ein
  Trommelfell wird nicht an ein Stethoskop angeschlossen, das gleich
  zerlegt wird (Operator-Entscheidung). Der `AcousticOscillator` zog
  von archivar.rs in die Mathematikerin — alle drei Oszillatoren
  wohnen bei der, die das Feld rechnet. Gates: cargo check 0/0
  (vier Feature-Kombinationen), cargo test 160/160 lib + alle Bins
  (2 gefallen gegenüber Atom 8), naga-Validierung, Live-Boot.

## Die Sphären des Unsichtbaren

- Atom 2 (Ringe: eigener rings-Buffer + WGSL ring_transmission,
  Literatur-τ mit Provenienz) — offen, eigene Session.
- Atom 3 (Warp: Linsen-Kompiler — Gaia-BH-Kandidaten + ATNF-Pulsare mit
  gemessener Masse; WD-Modell-Massen ausstehend; f64-Fold-Muster aus
  Atom 1) — offen, eigene Session.
- Atom-1-Grenzen (registriert): der 3D-Orbit des Planetenpunkts bleibt
  ausstehend — Ω (Azimut im Sky-Frame) ist ungemessen, der Schatten ist
  Ω-frei, ein Punktorbit wäre geraten; der Transit-Schatten ist seit
  Atom 8 (2026-08-20) tot — die Rückkehr läuft über die
  Feld-Absorption (pending, oben); pscomppars trägt
  mehrere Parametersätze je Planet und keinen default_flag — erster Satz
  je Planetenname zählt; fehlt ein Element → kein Schatten (0 honored).
- LuckyStar: decline (Vorhersagen sind Modell, keine Messung; die
  Ergebnisse-Server liefern nur abgeleitete Fits) — der rohe
  em-Lichtkurven-Kanal der Fresnel-Sphäre bleibt ausstehend.
- Okklusions-Reste → Feld-Absorption (pending, Atom 8, 2026-08-20):
  kontinuierliche Opazität
  (Partial-Transmission), atmosphärische Dämmerung, kleine Skala
  (Terrain/Bauten — der Mechanismus ist skalenfrei, die Daten fehlen),
  Oszillator-Eigenradius als Rekord-Slot, Transits als Feld-Dämpfung.
  Die geometrische Okklusion (Ephemeriden-Barrieren) starb in Atom 8;
  der absorption-Slot lebt im Protokoll — das Atom ist die Manifestation.
- Atom 1 deckt den Weg für Ringe/Warp — noch kein Konzept-Dokument.

## Der spektrale Oszillator — die Frequenzachse (Konzept: der-spektrale-oszillator.md)

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
  (SampleRecord überall, Stern-Push + freq/bin_width = 0.0 Punktquelle);
  Schritt 1 = Füll-Schicht (Channel + Sample tragen freq/bin_width,
  ~20 Konstruktionsstellen auf 0.0, query_hash liest sample.freq/
  sample.bin_width).
  Befund der Umsetzung (2026-08-19, live verifiziert): die
  txt-Route des Queue-Drafts ist tot (404) — die Monats-SSI-Messung
  existiert nur als netCDF-4/HDF5 (magic 0x89484446, deflate);
  kein ASCII-Weg (ERDDAP/THREDDS 0 Treffer), die reference-spectra.txt
  sind Modelldaten (LuckyStar-Präzedenz: refused). Die Ernte ist
  ERLEDIGT (2026-08-21) — `src/hdf5.rs` liest den Container in reinem
  Rust (Superblock v0-v3, Object-Header v1/v2, Fractal-Heap,
  B-Tree v1/v2, Chunk-Indexe, Filter deflate/shuffle/fletcher32/
  scaleoffset, Jenkins-lookup3-Prüfsummen, getestet gegen die echten
  2026er + 1874er NCEI-Dateien); `spectral_compiler --input-nc
  <file.nc> --month YYYY-MM` erntet (wavelength + SSI + time →
  Bänder, E_ν = E_λ·λ²/c mit dem 1e9-nm→m-Faktor — der Atom-B-Code
  trug die Einheit verdeckt als Lücke, die Ernte hat sie benannt und
  geschlossen), das CDN trägt spectra.bin (2026-06, Integral ≈
  1362,17 W/m² bei 1 AU; Asset ersetzt den 404). Named: prä-1972
  Epochen verweigert der LSK-Pfad (1874er-Datei liest, das Epoch
  bleibt void — 0 honored); shared messages + huge fractal-heap
  objects + virtuelle Datasets sind benannte Leser-Lücken;
  SSI_UNC trägt der Kontrakt nicht. Stationshöhe unverifiziert
  (Frame-Alt 0). CI-Register-Zeile (Registrierpflicht): im
  sources-Repo `spectral_compiler --input-nc <datei.nc> --month
  YYYY-MM --lsk naif0012.tls --ci-mode` — liegt außerhalb dieses
  Workspace.
  Fundorte: Queue-Draft master.φ:31611 (korrigiert), Concept
  der-spektrale-oszillator.md:107. Folgen: ONC-HSD-FFT, Gaia-XP,
  LISA-PSD + CMB-Power, miniSEED — je eigene Session. GONG ist
  erledigt (2026-08-20, gong_compiler: mrvmt L 0..30 geerntet;
  L 31..200 + mparam-Frequenzen pending).
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
  `hill_radius_m` ist heute nur Gate (is_none im Hash,
  src/archivar.rs:9134/9268), der Wert fließt nirgends — Manifestation (Hill-Radius als
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
  spd/hdg lebt (index.html 236-249, frame_motion in
  src/archivar.rs:10114) — der
  offene Rest ist nur noch refused-else ohne body-Deklaration.
- Der eingefrorene index.html/fieldShader-Snapshot trägt die tote
  Rotation noch (GRID_TO_ANGLE = 2^62, index.html 42/1245) — B1,
  bleibt registriert, falls der Relay wieder auflebt.
- M01 WebSerial-flow-Protokoll: zwei Spezifikationen konsolidieren —
  4d-membrane.md (`flow <force_name> <force_id> <|Ω|> 1 <tick_ms> <t>
  <x> <y> <z>`) vs. docs/omegaflow_sense_hardware.yaml (`flow <channel>
  <mode> <value> <unit> <duration_ms> <t> <x> <y> <z>`). SeismicOscillator
  schreibt heute die rohe f32-Σω-Intensität (4 B/Frame) an den Port
  (src/mathematikerin.rs, SeismicOscillator).

## Membran & Wahrnehmung

- Device-Lost-Befund + Farb-LUT (2026-08-20): Mesa 25.2.8 (ANV/Vulkan,
  apt-Upgrade 18.08., 25.0.7 → 25.2.8) verlor das Device beim Kompilieren des
  Feld-Fragment-Shaders (create_render_pipeline → Parent device is lost;
  der pre-cut-Binary starb identisch — der Dreiteilungs-Schnitt ist unschuldig).
  Bisektion im Test-Modus (jede Schicht einzeln gegen den Treiber gebaut):
  Okklusion lebt, Stern-Tiles leben, Tile-Cull lebt, omega-Akkumulation lebt,
  hsl_to_rgb lebt — `temperature_to_rgb` im dynamischen Loop tötet (die
  31-Stützstellen-Interpolation Pecaut-Mamajek + Helland-Polynome, inlined in
  den Loop-Body, überfordern den gen9-Compiler). (Die drei ersten
  Bisektions-Zeugen starben in Atom 8 — der Befund selbst bleibt wahr.) Fix: die Wahrheit wanderte in
  den Archivar — `omegaflow::spectral::color_lut_rgba` (256 Bins, Rgba32Float,
  Rand-Bins = exakte Locus-Klemmen, Bins dazwischen = Zentren; f64) als
  LUT-Textur (Binding 9+12, Nearest, NonFiltering); WGSL sampelt
  `color_lut_rgb` (weiß bei ci==0 wie zuvor); die drei WGSL-Funktionen starben
  — eine Quelle, kein Duplikat. Cargo.toml unberührt. Gates: 161/161 Tests
  (2 neue LUT-Tests in spectral.rs), cargo check 0/0 (alle Features),
  Live-Lauf unter Mesa 25.2.8 ohne Panik. Benannt: Mesa 25.0.7 schluckte das
  Konstrukt, 25.2.8 ist strenger; der OOM-Befund oben (GPU-Thread-Panik beim
  Pipeline-Bau) deckt sich mit diesem Tod — ob identisch, trägt die nächste
  Prüf-Rolle; ein Upstream-Bericht an Mesa/wgpu ist ein eigenes Atom.
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
- Sternenhimmel relativ zur Live-Em-Referenz (ft_ref) statt absolut —
  seit Atom 8 (2026-08-20) ist die Diode exakt relativ zur Live-Referenz;
  die Operator-Messung bleibt ausstehend: Wie atmet der Glow beim
  Übergang Sonnen-Nähe → tiefer Raum, wann erscheinen die achsennahen
  Sterne im Blick-Sweep.
- Galaxien-Zoom-Verifikation: der alte deep-Zähler starb mit Atom 8 —
  offen ist das Operator-Gefühl für die Glows im tiefen Raum (Proxima
  bei 4,2 ly ≈ 2^45,5); keine tiefaufgelöste Vorab-Integration.
- Fireball-Operator (sum vs. mean im `fold`) — Live-Verifikation offen.
- Audio-Phasen-Invariante dokumentieren: sr = 44100, ganzzahlige
  Frequenzen, 1-s-Noten → glatter Nulldurchgang am Tick-Ende; bei
  sr-/Frequenzwechsel bricht sie.
- Sternenhintergrund (integrierter Glow der 1/d²-Schwänze, Milchstraße):
  seit Atom 8 ist der Glow die lebende Summe der Diode — die Messung
  gehört dem Operator (kein vorab integriertes Feld).

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
parser-evaluation-matrix.md + EXTRACT_TYPES.md (SUPERSEDED by
sources-v2-spec.md).

- Kompilat-Pfad in die Zustandsmaschine holen: der Weg tap_index →
  kernel_flatten.yml → tap_compiler → CDN → sources.φ läuft außerhalb der
  Zustandsmaschine (SOURCE_PORT §4) — kein ledger-Eintrag, kein
  Pfadkarten-Eintrag. Deshalb zerfleddert ein großer Katalog in Queue/
  Metadaten/Weights/Stage, ohne je aufgelöst zu werden. Vereinheitlichung:
  eine Kompilat-Stufe (`entdeckt → kompiliert → disponiert`) in ledger.φ +
  Pfadkarte; `disponiert` räumt die Discovery-Reste. Berührt SOURCE_PORT.md
  + ledger.φ + ggf. main.rs (--fish-Flag).

Offen (Detail in phi/pipeline/ledger.φ):

- Solar-Akteure-Folgen: der CDAWeb-Live-Block steht
  (SOLO_L2_RPW-TDS-SURV-STAT, SN_RMS_E V/m — der Katalogschlüssel
  war /hapi/catalog, nicht /hapi/capabilities); der Publikations-Lag
  (~5 Monate, stopDate 2026-03-25) lässt das {hour_ago}-Fenster
  heute leer (0 honored). Die 2022-2026-Lücke ist GESCHLOSSEN
  (cdf_reader-Atom: LIRA-Ernte 2022-11-25→2025-12-31, 200544
  Records, 347692 total, CDN hochgeladen — der Befund „AMDA ≠ LIRA
  ~50 %, CDAWeb == LIRA" ist registriert, Naht 2022-11-24T23:55 |
  11-25T00:05 benannt). Pending: Berkeley-VSC (Baum-Forschung: kein
  VSC-Produkt auf dem live-Baum, HAPI-Dataset 1406), Wind/WAVES
  wav_h1 E_VOLTAGE_RAD2 (Parser steht — Ernte als eigenes Atom),
  GONG L 31..200 + mparam (Eigenfrequenz/Linienbreite →
  freq/bin_width); GOLF-Zeitreihen (Medoc curl 000); der
  kernel_flatten sun job trägt gong/rpw-CI
- Die Linse: Folgewelle — NASA-CMR-Keywords + GBIF-Tags downloaden,
  Library feinwägen; --port ersetzt --gold. 9. Fassung 2026-08-20
  geschliffen (NOAA-NODD-Befund: sea-ice/cors/gnss=em, crowdsourced
  bathymetry=acoustic, Stationsklima ghcn/gsod/isd=thermal — die Linse
  zieht die Buckets jetzt selbst, GDP-Drifter +52)
- S3-Harvester: xml_harvester löst den ListBucketResult-Namespace nicht
  (0 records, getestet 2026-08-20) — die NOAA-NODD-S3-Buckets
  (sea-ice/GDP-Drifter/cors/bathymetry) sind geparkt (ledger.φ
  parser-gap); braucht Namespace-Handling oder einen s3_harvester
- Probe-Stufe: nächste Welle — neue Kandidaten aus den Katalogen in
  batches/ nachrücken
- Queue: 10 Untested-Korpora (14k/13k/15k/7k/2k/183l/astro/earth/
  exotic/candidate-staging) — Port durch die Prozedur; astro-Korpus:
  28 Blöcke → manueller Port
- Bestand: 38 offene VizieR-Bulks, IRSA/GAVO/ARI/ExoArchive-Inventare,
  GCNS/MWSC (Kompilat, liegen in GAVO dc.g-vo.org), 77 Archeology-Gaps,
  ESA-Kandidaten (Aeolus key-needed, SMOS parser-def), FRB-Union,
  Arena/Foundation/Research-Schatz im Archiv
- Harvest-Runde 2026-08-20: TAP-Indizes ESO (59) + CADC (21) + MAST (15) +
  Chandra (11) und ERDDAP BCO-DMO + NOAA/PMEL (je 1.000) geerntet —
  Tür-Kataloge, SI-Extraktion pro Tabelle/Dataset folgt (ledger.φ erledigt +
  index.φ index). Dataverse lokal geerntet: Harvard 88.741 + Borealis 24.063
  (Linse: 455/176 positive — Ozeanchemie, Thermal, CO₂, AOD = Probe-Kandidaten
  der nächsten Welle); UNC 401 auth-needed (blocked key-needed). OAI arXiv:
  1.300 Records, dann Abbruch am skip-Timeout (oai_harvester ohne
  Timeout-Flag — fixe Länge nötig oder --set-Partitionierung). Dead: SDSS-TAP
  (alle Routen 404, 2026-08-20 — CasJobs bleibt). Parser-gap: MGDS
  (marine-geo.org liefert data_set-XML als Attribute, xml_harvester liest
  Kind-Elemente). Recherche: EPN-TAP-Endpoint (VESPA). Harvester-Fixes:
  tap_compiler --index folgt Redirects (curl -L, Chandra 303),
  dataverse_harvester root+Slash (war 000). Linse-Fix: source_scanner als
  Release-Binary laufen (Debug ~6× langsamer, 88k Records in 1,5 min statt
  timeout). Probe-Batch gebaut: queue/grind_dataverse.φ (136 Blöcke, Harvard
  90 + Borealis 46, Gewicht ≥ 16, Dataverse-API je DOI live verifiziert) —
  nächste Probe-Welle.
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
  Photometrie/Spektroskopie — 2MASS PSC (pending, s.u.), RAVE DR6,
  APOGEE/GALAH; Extragalaktisch — NED (Root verifiziert, Chunks pending),
  HyperLEDA/PGC; Radio-Kontinuum (Achse leer) — TGSS ADR, SUMSS, RACS,
  LoTSS, VLASS (NVSS/FIRST erledigt — Workflow + Blöcke);
  High-Energy — Fermi 4FGL-DR4 (Unit-Arm pending), AMS-02 (Chandra
  CSC 2.1 erledigt); Sonnensystem — PDS (Instrumentendaten), MPC-Live
  (mpcorb_extended.json.gz); TAP-Indexe — MAST, CADC, ESASky, NOIRLab
  Data Lab, NED; Terrestrisch — EarthScope-FDSN, EPOS, SeaDataNet,
  Smithsonian GVP, Natural Earth.
  Exakte Tabellen-IDs + Spalten + Mechanismus:
  /home/johannes/projects/archive/handover/handover-2026-08-20-fischplan-kataloge.md
  + handover-2026-08-20-chunk-kataloge.md (archiviert).
  RAVE (III/279/rave_dr5): 472845 Zeilen kompiliert (24 RA-Slices à
  15°, rv-Gate HRV), Asset + sources.φ-Block leben — der --async+JOIN-
  Weg hing auf VizieR PENDING (gemessen: >600 s in 2 CI-Läufen,
  >3600 s lokal). GLADE+ ist pending: Spalten live verifiziert, aber
  drei gemessene Blocker — Schrittboden-Kappung des --mag-bands-
  Banders, 2-GB-Release-Limit, MAX_SAMPLES 4.19 M (chunk-plan, archiviert).
- VizieR-async-Befund: --async + gaiadr3-JOIN hängt PENDING — UWS-Jobs
  sind IP-gebunden: stirbt der Runner, verwaist der Job. RA-Slices
  sind der Weg für Crossmatch-Kompilate.
- health-Label-Befund: das Label fehlte auf omegaflow/omegaflow —
  alle `gh issue create --label health` waren stumm (kein einziges
  flatten-Issue im Register). Der Prepare-Schritt legt das Label jetzt
  an (kernel_flatten.yml).
- ω-Loop-Fetch-Sturm (Befund 2026-08-21): der Live-Source-Zyklus fischt
  ~200 Quellen kontinuierlich mit 4 Retries × 23 s und ttl/Φ-Backoff —
  ein unbegrenzter Churn, der die Heimleitung bei jedem Membran-Lauf
  sättigt (Neustart erzwungen). Budget-Messungen brauchen einen
  begrenzten/drosselbaren Lauf statt des vollen Membran-Churns — der
  Sturm selbst ist ein eigener Reparatur-Gegenstand (Retry-Exponent,
  Pausen pro Quelle).
- Sample-Budget des Feldes (kritisch, eigenes Atom): die Summe aller
  Katalog-Blöcke (Sterne 1.19 M + Asteroiden 1.56 M + NVSS 1.8 M +
  FIRST 1.1 M + Chandra 0.4 M + vier Chunks 1.4 M + …) liegt über
  MAX_SAMPLES (1<<22, src/archivar.rs:9038) — der Rebuild hält die
  jüngsten Samples und wirft die ältesten (epoch 0.0 = Sterne +
  Kataloge, archivar.rs:15543). Welcher Anteil der epoch-0.0-Samples
  überlebt, ist ungemessen — die Messung ist die Vorbedingung für jeden
  weiteren Katalog-Block (Commit 2 des chunk-plans).
- Pfeiler-Registratur Nadel V (Farbe, kritisch): die JSON-Kataloge
  (denis/wds/pastel/mktypes/rave) ernten bpmag/rpmag, aber color_index
  manifestiert nur der star-bin-Pfad (dr3_stars.bin, bright_stars) —
  cmap-Farb-Schlüssel oder Compiler-bp_rp-Alias als eigenes Atom.
  2MASS J−K hängt am Bulk-Kompilator-Atom (s.u.).
- Pfeiler-Registratur Nadel V (Frequenzachse, kritisch): NVSS/FIRST
  tragen 1.4 GHz nicht in freq/bin_width (kein Compiler-Flag) — pending.
- Nadel-I-Befund NED: Root https://ned.ipac.caltech.edu/tap/sync +
  NEDTAP.objdir (ra, dec, z, prefname, type_key, n_spectra) live
  verifiziert (2026-08-20); sync-COUNT läuft in den 60-s-Timeout
  (Server: async) — async-Slice-Counts messen, dann RA-Slice-Chunk-
  Schritt (eigenes Atom).
- 2MASS-Befund: sync-COUNT auf II/246/out > 60 s (gemessen 2026-08-20) —
  der --mag-bands-Bander ist für 470 M nicht CI-tragfähig; Bulk-Route
  (cdsarc-ftp) braucht einen Kompilator (eigenes Atom), ein erklärter
  heller Schnitt bleibt Kurationsfrage.
- Chandra-Drift benannt: der Block trägt erg/cm2, CSC-Fluxb ist
  physikalisch erg/cm²/s — gehört zum Unit-Arm, Block-Label prüfen.
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
- Register-Datenqualität (2026-08-21, abgeschlossen): Gate-Konsistenz
  wiederhergestellt — `extract_fields` trägt `GeojsonEvents` (zwei
  Force-Felder kernel 0 / seismic-body, die Spiegel des Extracts),
  der Hidden-Lauf trägt keine „no field lines"-Zeile mehr, der
  fdsnws-Block fetched wieder (leeres Fenster am Golf von Guinea =
  Wahrheit). Der Live-Test ist Instrument: echte Systemzeit, vier
  void-Klassen (key/drift/ruhig/kaputt), `--reverify` schreibt
  phi/pipeline/stage/recheck_live.φ, healthcheck.yml trägt den
  3-h-Cron-Job (Artifact + Drift-Issue). phi/pipeline/refusal_ledger.φ
  sammelt die Laufzeit-Refusals (je Quelle+Klasse ein Eintrag).
  Hinweis: die Code-Einheiten A–C gingen mit d9d2c72 (Fetch-Sturm-
  Reparatur der Parallel-Session, die das ganze archivar.rs
  übernahm) in Git — die Register-Commits 62dae65/e97844c tragen
  healthcheck/recheck_live/refusal_ledger/TODO. Handover archiviert.

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
- CI-Chunk-Kompilation der großen Kataloge: der chunk_catalogs-Job
  (kernel_flatten.yml) verdrahtet RAVE/pastel/wds/mktypes/denis als
  Bash-RA-Slices (CI-Replikat von phi/pipeline/chunk_master.py, ohne
  Python). Zwei Dispatch-Läufe wurden extern abgebrochen; die
  Kompilate wurden lokal nachgeholt — alle fünf Assets liegen valide
  auf dem CDN, sources.φ- und ledger-Einträge leben (Commit 2,
  chunk-plan, archiviert). Offen: ein voller grüner
  Lauf (Verifikation des Slice-Schritts) + die MAX_SAMPLES-
  Budget-Messung. GLADE+ bleibt draußen (drei gemessene Blocker,
  s. Katalog-Lücken). Der JSON-mag-bands-Bracket-Bug und das
  WHERE-Quoting (tap_compiler.rs) sind behoben — die Fixe tragen Git.
- CDN-Asset-Naming: `{name}.json` — Konvention ist der Resolver (Regel)

## VERSIONIERT / AUSSTEHEND

- Temporal Topology (TDA, Takens, Transfer Entropy, Surrogates) —
  VERSIONIERT, lost-concepts.md
- Kraft-Separation (7 omegas statt „one law, five media") —
  VERSIONIERT, LOST_CONCEPTS §13
- Verzögerungsspektrum / Lichtkegel-Differenz / Stillekarte /
  Synthetischer Flug — VERSIONIERT, der-paradigmenwechsel.md,
  LOST_CONCEPTS §14–17
- Field Permeability (tanh(vC/g)-Variante ohne TE) — VERSIONIERT,
  minkowski-field-permeability.md
- Minkowski 4D Weighting (spacelike→0; kosmisches Skalenproblem: Sonne
  wäre spacelike — scale-Anpassung nötig) — VERSIONIERT,
  minkowski-field-permeability.md
- Auto-Zoom (median-extent/p90) — VERSIONIERT (bd9a513 entfernt; die
  atmende Membran ist der stärkere Vorfahr; Fenster-Reduktion = Budget-
  EMA als HUD-Messung — der Operator entscheidet)
- Council-Forschungs-Iterationen: Archivar als „langsamer Prior" für den
  Exposure-Kaltstart (aktuell: fixe Rampe); Exposure-EMA auf dem
  Silizium (gegenstandslos solange die Rampe fix ist) — AUSSTEHEND
- Future: Aggregation of Presence, Retro-Manifestation, Total Coherence
  Integration, Nostr-Stationsweb — AUSSTEHEND, future-concepts.md

## Rejected

- Unknown-Force soft handling → Parser lehnt unbekannte Kraft ab
- Default τ-Werte → Gate schließt, wenn nicht deklariert
- World Bank Indicators → forceless, DROP
- Yahoo Finance → forceless, DROP
- Hexagon-Grid, Quadtree-AMR, temporale Akkumulation, Blue-Noise-Rieseln,
  Nahfeld-Splitting → Interpolations-/Zeit-Lügen (Council-Urteil,
  wgsl-shader.md)
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
