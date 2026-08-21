# Entwicklungslinie — omegaflow von Anbeginn

Archäologie 2026-08-17. Die Linie des Systems vom ersten Commit bis heute:
was entstand, was verging, und was der Zeit und den Session-Constraints
geopfert wurde. Quellen: Git-Historie main (1310 Commits, 2026-06-05 →
2026-08-17), TODO.md (Rückroll, Eine Version, Fortschritts-Verzeichnis,
Stand, wgpu-mono-Blöcke), docs/surveys/, /home/johannes/projects/archive/
(PROJECT_GUIDE, session_logs, pre_cdn_history, omegaflow_archeology.zip).

## Epochen-Tafel

### Epoche 0 — Die Appelle (vor 2026-06-05, vor dem Git)
Es gab eine „appeal"-App, deren Kern nach Rust migriert wurde
(„feat: appeal pure rust core stack migration"). Über die Zeit davor ist
im Repo nichts belegt — das Archiv (session_logs, archeology.zip) ist
unausgewertet. 0 honored: was nicht vermessen ist, ist nicht behauptet.

### Epoche 1 — Rust-Kern und Verfassung (2026-06-05/06)
Entstanden: EpistemicState, MeasurementState, LossState,
ProvenanceMetadata, Relationality; die „Constitution" v3.3.0
(docs/prompts §1–§30); config.yaml-Regeln; die nebra-pilot-Crate;
das reine-Rust-Kern-Mantra.
Verloren: die konstitutionelle Dokument-Architektur — mit v3.0
(„A = A − universe(jd, pos) → (omega, flow)") weicht sie dem einen
Prinzip. Die Verfassungs-Textmauer stirbt an der A=A-Axt.

### Epoche 2 — A = A, das erste Gesetz (2026-06-09 bis 12)
Entstanden: `universe(jd, pos) → (omega, flow)`; GPU-Evaluator
(GM/dist² je Pixel); WMM-Erdmagnetfeld im GPU („A ∞ A: electromagnetism
flows"); EGM96-Geoid + dynamisches DEM; Terrain-Cache;
omegaflow-wasm; Ephemeriden-Umbau (state.rs → ephemerides/terrain/
magnetic); „no comments. no docstrings. no compliance theater."
(2026-06-12) — der Nacktheits-Eid, der bis heute gilt.

### Epoche 3 — Der Browser als Station (2026-06-11 bis 07-01)
Entstanden — die reichste Epoche, die spätere Maßstab:
- index.html/constants.js: die Browser-Station mit WebGPU-Membran,
  Presence-Weltlinie (p + v·(t−t0)), Gaze-Quaternion, Deep-Link
  #x,y,z,t[,vx,vy,vz], /jump/<body>, windowQueries (center + 4 Ecken)
- Station ≠ Presence: Geolocation (body/lat/lon/alt → Surface),
  Consent-Gate, Batterie, IMU ×6, Kamera (Pixel-Oszillatoren),
  Mikrofon (FFT-Bins), Gamepads als Oszillatoren, Serial/USB/BT/HID
- Ein Gesetz, fünf Medien: die Probe-Ωₖ je Kraft am Presence-Punkt
  treiben Audio (9 Partialtöne), Vibration, Serial/USB/BT/HID-Ausgänge
  — alle durch tanh(|Ωₖ|·mx²) mit mx = windowMedianExtent
- RTT-adaptives Transportprotokoll, Reconnect mit Backoff,
  Buffer-Schrumpf bei langsamen Frames, 3-Finger-Zeitschub
- /resonance, /crash-Diagnostik, der 3-zeilige HUD
Verloren: siehe Register — die meiste Browser-Pracht ist beim nativen
Umbau nicht mitgegangen (P2–P6).

### Epoche 4 — is(t,x,y,z) und die Reise (2026-06-22 bis 07-01)
Entstanden: das .is-Format („is(t, x, y, z) — pure points, pure flow,
live resonance"), fly.io-Deployment.
Verloren: das .is-Binärformat — ersetzt durch das φ-Protokoll (v6,
0xCF 0x86 0x06, Record 168 B). Der PROJECT_GUIDE im Archiv dokumentiert
die Zeit.

### Epoche 5 — phi und der Rat (2026-07-01 bis 15)
Entstanden: φ/ → phi/ (ASCII-Pfadfix); sources.φ; das gremium mit
5 Stimmen (der Council-Vorläufer: „breathing MA (asymmetric parabol +
consolidation), continuous curiosity"); +287 Quellen (Währungen, ETFs,
Noosphäre: Wikipedia/arXiv, Spurengase, Pollen); der AST-Parser und die
„agnostic kybernetic architecture".
Verloren: das „legacy immune system" und der „biological overhead"
(2026-07-15) — eine ganze biologische Metaphern-Schicht wurde
abgestreift. Ein Paradigma-Schnitt: die kybernetische Sprache ersetzt
die organische.

### Epoche 6 — Quellen-Explosion und Sternhimmel (2026-07-17 bis 08-02)
Entstanden: 23 Orbit-Traces, 16 seismic-body, 14 thermal Quellen,
Saturn-Ringe; per-force spatial kernels mit Lichtlaufzeit-Retardation
(aperture/res → force/extent/tau); exposureBoost e/E („schwache
Himmelskörper sichtbar"); der SYNTHETISCHE Sternhimmel im Fragment-
Shader (WGSL-Hash — vor dem echten Katalog!); NASA FIRMS (119k
Hotspots), Esri, CO-OPS, nearest-station-Resolver; der Hybrid-LOD
Star Renderer mit Magnitude-to-Flux; Phi-basierte Exposure.
Verloren: der synthetische Sternhimmel — überflüssig, als der echte
Katalog kam (0 honored: die gemessene Wahrheit ersetzt die gemalte);
deviceorientation-Listener (getilgt, WGSL select() statt ternary).

### Epoche 7 — CDN und Katalog-Wellen (2026-08-04 bis 14)
Entstanden: der Archivar als Manifestator des CDN (ci-mode,
{netloc}/{name}.json, naming convention als Resolver);
verify_sources.py (8 Arbeiter); K03-Kometen (dcom5-Multi-Apparitionen,
Kepler); K04b Gaia DR3 + Bailer-Jones 1,84 Mio Sterne; Exoplaneten-
Bulk 6309; der tap_compiler über TAPVizieR/GAVO/ARI/IRSA/ExoArchive;
LSK/PCK-Hochzeit; Binary v2; Protokoll v6; TNS-Vollkatalog (~20k
Rotverschiebungs-Transienten); std-only DEFLATE (inflate.rs — ein
Zip-Entpacker aus dem Standalone-Stack!); body-orientation CDN
(stype-3 Rotationsmatrizen aus SPICE); arena (132 live-verifizierte
Kandidaten).
Verloren: query_star_hash — die Lemma-Sternabfrage (Zell-Dilatation,
Parallaxe, Eigenbewegung, star_position_at): die Sterne waren VOLLE
Membran-Oszillatoren. Die Deep-Schicht (eingefrorene Richtungen)
ersetzte sie (P3). Am 2026-08-07: 143 Commits an einem Tag — die
Crossmatch-Welle, der CDN-Umbau.

### Epoche 8 — wgpu-mono (2026-08-14 bis 16)
Entstanden: Titan „EINE Datei" (der Monolith: Kreis an der Crate-Wurzel,
archivar/mathematikerin/relay als Inline-Geschwister); Deep-Sky-Plenum
(zwei Regime, eine Wahrheit — der Deep-Renderer mit ausdehnungs-
skalierten Quads); Titan-Audio (der Milliarden-Schleifen-Bug getilgt:
dur = τ·44100 → 3,8e9 sin() je Oszillator je Tick); Atom-1 Hybrid
(near_pt-Gauß-Punkt-Layer, Per-Kraft-Referenzen, e/E ×2/÷Φ, P-Zyklus);
Gaze-Drehung 1:1 (die tote Rotation, die schon im Browser tot war).
Verloren/zurückgerollt: sun_follow, field_centroid/Auto-Zentrierung,
VP-Slot-Churn (Rückroll auf fd666f5b); die Subpixel-Explosion
(eb96d1f: 567 ms, 9 Mio Messzellen, Rgba32Float — Session-Experiment,
dessen MESSUNG in messpunkt-verteilung.md lebt); Generations-
Architektur, Tiled Source Culling, HUD-Akkumulatoren, deep_gain,
Budget-Regler — versioniert auf session-2026-08-16 (nicht verloren,
wartend); main-browser eingefroren (8 Commits: Himmelssphäre
--crossmatch, Frame-Registry, derive_frame, frame_learned.φ, TAP-
Zeilen-Mapping, cmap-Draft, --draft-context, JINA_API_KEY-Pacing —
Hand-Port als eigene Session offen); relativistische Aberration —
mit der Membran-Rewrite gegangen (eigener Atom offen).

### Epoche 9 — Verlust-Bewusstsein (2026-08-17)
Entstanden: das Verlust-Register P1–P6 (der Monolith gegen die letzte
index.html); der Archäologie-Nachtrag (Pre-Monolith f2023eb^:
constants.js identisch, von 173+ Funktionen fehlen genau zwei);
die Entwicklungslinie (dieses Dokument). Die Session-Grenze ist die
Erkenntnis: was die Sessions vergessen, hält jetzt das Register.

## Das Verlorene — die vergangenen Features

| Feature | Epoche | Schicksal | Status |
|---|---|---|---|
| Verfassung §1–§30, config.yaml | E1 | von A=A abgelöst | ersetzt |
| .is-Binärformat | E4 | von Protokoll v6 abgelöst | ersetzt |
| immune system / biological overhead | E5 | abgestreift (Paradigma) | getilgt |
| synthetischer Sternhimmel (WGSL-Hash) | E6 | vom echten Katalog abgelöst | ersetzt |
| deviceorientation-Listener | E6 | getilgt | getilgt |
| query_star_hash + Lemma-Sterne | E7 | von eingefrorenen Deep-Richtungen ersetzt | P3 |
| sun_follow, field_centroid | E8 | Rückroll | wartet |
| Subpixel-Explosion (Rgba32Float) | E8 | Rückroll (Messung lebt) | wartet |
| Generations-Architektur, Tiled Culling, deep_gain, Budget | E8 | session-2026-08-16-Zweig | versioniert |
| main-browser 8 Commits (Himmelssphäre, Frame-Registry, JINA) | E8 | main-browser eingefroren | Hand-Port offen |
| Relativistische Aberration | E8 | mit der Membran-Rewrite | eigener Atom |
| Browser-Exposure-Kette (lvls, e/E ±1) | E3 | getilgt | durch Atom-1 ersetzt |
| 5-Medien-Gesetz + windowMedianExtent + Ausgabe-Flächen | E3 | nie in den nativen Pfad | P4 |
| Presence ↔ Device-Trennung, Station-Identität | E3 | nie im nativen Pfad | P2 |
| Maschinen-Sinne (IMU/Kamera/Mikro/Batterie/Gamepads) | E3 | nie im nativen Pfad | P5 |
| Consent-Gate | E3 | nie im nativen Pfad | P5 |
| 3-Finger-Zeitschub, f-Toggle, Deep-Link-Init, HUD | E3 | nicht übernommen | P6 |

## Befund zur Frage: „Ist Genialität der Zeit und den Session-Constraints geopfert worden?"

Ja — an drei Stellen messbar:
1. **Die Rückroll (E8)**: sun_follow und field_centroid — der lebende
   Blick, der der Sonne folgt — wurden mit dem Subpixel-Experiment
   zurückgerollt. Sie sind kein Konzeptfehler, sie waren Kollateralschaden
   einer Session, die am Rgba32Float-Experiment verbrannte.
2. **Der Monolith (E7/E8)**: Die Lemma-Sterne, die 5-Medien-Physik, die
   Station-Identität, die Maschinen-Sinne — der native Umbau hat das
   Browser-Erbe nicht portiert, sondern ersetzt. Die häufigste Verlust-
   Ursache war nicht Entscheidung, sondern Nicht-Wissen: niemand hielt
   die index.html als Maßstab (jetzt: das Register).
3. **main-browser (E8)**: acht ausgereifte Commits (Himmelssphäre,
   Frame-Registry, JINA-Pacing) froren ein, weil die Konsolidierung die
   Session kostete.

Aber der Befund ist keine Niederlage:
- Nichts davon ist unwiederbringlich. session-2026-08-16, main-browser,
  die Surveys, die Archeology-Zips und das Register sind die Saat —
  P1–P6 und die offenen Atome sind die Erntepläne.
- Die größten Verluste sind Paradigma-Schnitte mit Wiederkehr-Garantie:
  das „immune system" war eine Sprache; die Verfassung war ein Dokument;
  A=A und 0 honored sind stärker als beide. Die Rose-Formel
  (0.02/(r²+0.02)) wurde verworfen, weil sie der 1/d²-Kernel selbst war —
  die Disziplin tötete die schöne Lüge.
- Die Entwicklungslinie ist keine Gerade: sie ist eine Spirale. Jede
  Rückroll ist eine Messung, keine Niederlage; jede Welle (Katalog,
  CDN, wgpu-mono) hat die nächste Epoche genährt. Die Session-Constraints
  haben Genialität gekostet — aber das System hat sie als Messpunkte
  aufgehoben: fürtschritt.md, auswertung.md, messpunkt-verteilung.md
  sind die Schatzkammer der 567-ms-Erkenntnis.
- Die Regel für die Zukunft steht in Epoche 9: **was eine Session
  verlässt, wird registriert, bevor es geht.** Das Verlust-Register ist
  das Gedächtnis, das die Session-Grenze überwindet.
