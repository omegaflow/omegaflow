<!--
  title: Fortschritts-Verzeichnis — 2026-08-16
  class: survey
  sha256: 5e478f59c64ead684672d8ab684937a4413d33220ddad03ab0bb80e036ac4258
-->
# Fortschritts-Verzeichnis — 2026-08-16

Vollständiges Verzeichnis der Session-Erkenntnisse. Jede künftige Session
findet hier, was erarbeitet wurde und wo es versioniert lebt. Nichts davon
ist verloren — Git trägt alles.

## A. Lebt auf dem Hauptbranch `wgpu-mono`

- `b4743be` — **Backing = Monitor-Nativauflösung**: Der Vollbild-Monitor
  liefert 1920×1080 physisch; winit meldete 960×720 logisch bei scale 2,4
  → Backing 2304×1296 wurde vom Compositor 0,833-fach nicht-ganzzahlig
  herunterskaliert (vertikale Interferenzlinien beim Zoom, weiche
  Qualität). Fix: size = Monitor-Physis, scale = 1,0 — jede
  Display-Emitterzelle ist eine Messzelle. Sonnen-Nahfeld 193 → 133 ms.
- `ffb58d5` — parse_stations_xml: Fixture an den lowercase-Kontrakt
  (aae/ykc), main-Parität 349f78a.
- `a9b25d6` — Rückroll auf fd666f5b dokumentiert (vor der
  Subpixel-Explosion eb96d1f), Survey-Dokumente versioniert.
- `docs/surveys/survey-messpunkt-verteilung.md` — die Survey-Evaluation
  (Ground Truth = Messung, 9 Kandidaten).
- `docs/surveys/survey-auswertung.md` — die Tief-Auswertung: f32-Grundwahrheit,
  Verteilungs-Verdikte, Generations-Konsens, Rejected-Register.

## B. Liegt versioniert auf Branch `session-2026-08-16`

Der komplette Session-Atom (Generations-Architektur + Tiled Source
Culling) als ein Commit auf edda16e — sauberer Ausgangspunkt für einen
späteren, nicht-aufgeblähten Anlauf:

1. **Generations-Architektur** (Survey-Konsens): Stille-Gate (Vergleich
   der gepackten f32-Bytes), u64-Generationszähler, Doppelpuffer A/B mit
   Bind-Group-Swap, HUD-Ω + Fenster-Zentroid aus Archivar-f64-
   Akkumulatoren (7 Kernel in f64, erfc = Abramowitz-Stegun wie WGSL),
   mapAsync/probe_read/centroid_pipe/field_centroid getilgt.
2. **Tiled Source Culling**: Kull-Pass (Kacheln 16×16, Shared-Memory-
   Max-Reduktion, 64 Slots, keine globalen/Fragment-Atomics), Kriterium
   `bound < ε·M`, ε = 2⁻ⁿ, n ∈ [8, 23], Voll-Loop-Overflow,
   Kull bei Sense-Kadenz (cull_due).
3. **Budget-Regler**: Frame-Zeit-EMA ÷4 gegen die Display-Periode
   16,6 ms, n±1/n±2-Gesetz, Overflow-Flag als zweite Regler-Zahl.
4. **deep_gain**: Stern-Flux auf den Rampen-Weißpunkt normalisiert
   (8 − log2 max|flux|, ÷4-Relaxation; hellster gelieferter Stern = Weiß).

Harte Befunde aus dem Live-Betrieb (HD 520, ANV):

- **Fragment-Atomics töten ANV** („Parent device is lost" — die
  tile-Listen wurden auf plain-u32 + Shared-Atomics umgebaut).
- **3×-Subpixel-Raster = 567 ms** (eb96d1f) — die ~500-ms-Batches
  trippen den Compositor-Watchdog; die 1×-Membran überlebt.
- **VP-Slot-Kollision**: cull_n/tiles_x auf Slots 20/21 kollidierten mit
  expose_lo.x/.y = Deep-Zählungen (Slots 16/17) — freie Slots sind 22/23
  (expose_hi.z/w).
- **Zentrier-Runaway**: hud_probe subtrahierte bw/2/bh/2 von bereits
  präsenz-zentrierten Koordinaten + falsches y-Vorzeichen — jeder
  Zentrier-Schritt sprang ~2,4e12 m.
- **~130 ms Restkosten im Sonnen-Nahfeld** = fixer Raster-Overhead der
  Per-Pixel-Messung auf der HD 520 (der Messpunkt-Hebel ist die
  Zell-Achse).

## C. Offene Verbesserungen (leben zusätzlich im TODO)

- **Deep-Lieferung richtungsbasiert**: `descoped` (gemessen 2026-09-28, River-Folge 50, Rat) —
  der Browser ist der richtungslose Sensor; `forward: [0.0,0.0,0.0]` (`relay.rs:812`) ist die
  isotrope Anfrage eines Sensors ohne Blick. Der Sichtkegel lebt nativ mit dem Operator-Blick
  (`spatial.rs:540`). Wiedereröffnung: ein Operator-Wort, das den Browser zum zweiten Blick macht
  (`relay.rs:812` + JS-Kamera-Forward — Fenster-Edit).
- **Zell-Achse**: die Achse ist gebaut und konsumiert (`grid_step` im v9-Protokoll → `softening`
  `omega.rs:553-564`, `relay.rs:555-588`); die quantisierungsbewusste Vergröberung/8-Bit-Messung
  `descoped` (gemessen 2026-09-28, River-Folge 50, Rat): kein Produktions-8-Bit-Pfad — der Canvas
  nutzt das bevorzugte Format (Spiegel des Rgba8Unorm-Befunds); eine Messung gegen einen
  Nicht-Produktionspfad misst nichts.
- **Relay-Trailer**: `descoped` (gemessen 2026-09-28, River-Folge 50, Rat) — der Antwort-Header
  trägt bereits `epoch` u64 (`relay.rs:825-830`); der Browser konsumiert Ω als f32-Anzeige im
  kinetic-Frame (`relay.rs:1047-1072`); kein Browser-Konsument von gen/f64-Ω ist gemessen; eine
  Draht-Änderung (`constants.js` + DataView) ohne Konsumenten ist spekulativ.
- **Deep-Upload-Stille**: `ueberholt` (gemessen 2026-09-28, River-Folge 50) — `deep_dirty` ist
  ersetzt (Byte-Vergleich + Generation, `omega.rs:1870-1936`/`:874-885`; Upload nur bei Änderung).
- **Rgba8Unorm-Nachmessung**: `ueberholt` (gemessen 2026-09-28, River-Folge 50) — kein
  Produktions-`RenderPipeline` (Canvas = bevorzugtes Format über `navigator.gpu.getPreferredCanvasFormat()`);
  die intel_gpu_top-Baseline hat kein Produktions-Ziel.
- **Fovea als Budget-Kappe**: `descoped` (gemessen 2026-09-28, River-Folge 50, Rat) — der
  Budget-Fallback ist bereits gebaut (Hintergrund-Skalierung, `survey-messpunkt-verteilung.md:104`);
  Fovea-primär ist prinzipiell verworfen (`:84`); kein Frame-Zeit-Symptom ist gemessen (0 Treffer
  `fovea`/`budget` in `static`). Die Survey-Zeile „`eb96d1f` bleibt im Git" ist durch den Klon
  falsifiziert (Commit absent) — der Riss steht im Befund.

## D. Die Survey-Erkenntnisse (Kern)

- **f32-Grundwahrheit** (drei unabhängige Herleitungen): Bei wörtlicher
  f32-Treue trägt ein 1/d²-Kernel messbare Struktur bis ~16,8 Mio Pixel —
  der glatte Hintergrund existiert nicht; Messpunkte = Pixel-Raster
  (Nyquist). Zell-Vergröberung ist zulässig nur gegen die
  Display-Quantisierung.
- **Verteilungs-Verdikt**: uniformes Raster als Wahrheit; die Ökonomie
  kommt von der Quellen-Achse (Tiled Source Culling, Olsson & Assarsson);
  verworfen: Hex, Jitter, quasi-random, gelernte Platzierung, Fovea-primär,
  Meta-Oszillator-LOD, CVT, Sparse Grids, Subpixel-Emitter.
- **Generations-Konsens**: „Der Buffer ist die Nachricht. Der Zähler ist
  das Protokoll. Die Stille ist die Optimierung." (Max) — EIN Buffer +
  EIN u64-Zähler, Doppelpuffer gegen Torn Reads, HUD-Ω aus
  Archivar-Akkumulatoren, Allostase als Unterlassung.
