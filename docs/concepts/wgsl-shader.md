Deepseek Chat v4
# Architektur-Konzept: „Agnostische Membran“ – Kontinuierliches 4D-Feld auf 2D-Membran

## 1. Überblick & Kernidee

Das Grundproblem ist die Diskrepanz zwischen der großen Pixelanzahl (≈2 Mio.) und der kleinen Quellenanzahl (10–200). Ein Per‑Pixel‑Shader, der für jedes Pixel über alle Quellen iteriert, ist auf der Intel HD 515 bei steigender Quellenzahl nicht mehr beherrschbar (O(N·M)). Ein festes Compute‑Grid mit bilinearer Interpolation verschwendet Rechenzeit in leeren Regionen und erzeugt sichtbare Diskontinuitäten beim Skalieren.

Die Lösung besteht in einer **Trennung von Messung und Darstellung**:

- **Messung** erfolgt an einer adaptiven, regelmäßigen (hexagonalen) Punktwolke – den „Mess‑Knoten“.  
- **Darstellung** interpoliert im Fragment‑Shader zwischen den drei nächstgelegenen Knoten (baryzentrisch).  
- Die Anzahl der Mess‑Knoten wird **stufenlos** über einen Regelkreis an die aktuelle GPU‑Last angepasst.

Damit reduziert sich die Feldberechnung von O(Pixel·Quellen) auf O(Knoten·Quellen). Bei z. B. 50 000 Knoten und 200 Quellen sind das 10 Mio. Kernel‑Auswertungen pro Frame – machbar auf einer schwachen integrierten GPU. Der Fragment‑Shader liest nur noch drei Werte pro Pixel und führt eine einfache Interpolation aus.

---

## 2. Buffer‑Layout

Alle Daten liegen in GPU‑Speicher, die Aktualisierung erfolgt über WebSocket und `writeBuffer`.

| Buffer | Typ | Inhalt |
|--------|-----|--------|
| `SourceBuffer` | SSBO (readonly) | Array aller aktiven Quellen mit Position, Wert, Epoche, Tau, Extent, Kernel_ID, Kraft_Type, Velocity, Pole, J2, J4, r_eq. |
| `FieldNodeBuffer` | SSBO (read/write) | Skalarer Feldwert **Ω** für jeden aktiven Mess‑Knoten. Ein Float32 pro Knoten. Bei maximal 65 536 Knoten ≈ 256 KiB – vernachlässigbar. |
| `UniformBuffer` | Uniform | Aktuelle Zeit `t`, `grid_scale` (Gitterkonstante), sichtbarer Membran‑Ausschnitt, Anzahl aktiver Knoten, Tonemap‑Parameter (`max_lvl`). |
| `DensityBuffer` | optional, Texture/SSBO | Dichtefunktion für adaptive Verfeinerung (Backend‑Vorgabe), siehe Abschnitt 7. |

Die Positionen der Mess‑Knoten werden **nicht** explizit gespeichert. Sie ergeben sich analytisch aus dem linearen Knotenindex und der Gitterkonstante `grid_scale`. Das spart Speicher und vereinfacht die stufenlose Skalierung.

---

## 3. Compute‑Shader: Feldmessung an den Knoten

Der Compute‑Shader wird mit genau so vielen Threads gestartet, wie aktuell Mess‑Knoten aktiv sind. Jeder Thread:

1. **Berechnet seine Position** im Membran‑Koordinatensystem (z. B. normiert auf [0,1]²) aus seinem linearen Index und `grid_scale` – unter Annahme eines regelmäßigen Hexagon‑Gitters (siehe Abschnitt 6).  
2. **Summiert über alle Quellen** im `SourceBuffer`. Für jede Quelle wird die zeitabhängige Position berechnet (z. B. `pos + velocity · (time − epoch)`). Anschließend wird anhand von `Kernel_ID` und `Kraft_Type` der Beitrag zum Feldwert Ω berechnet. Gravitation (Kernel 0) nutzt z. B. `1.0 / max(d², 1.0)` (kein Softening). Die 4D‑Parameter (Tau, J2, J4, r_eq, Pole) fließen in die Kernel‑Auswertung ein.  
3. **Schreibt Ω** in `FieldNodeBuffer[gl_GlobalInvocationID.x]`.

Die Schleife über die Quellen ist im Shader unkritisch, da `source_count` klein ist. Die Komplexität O(Knoten·Quellen) wird durch die adaptive Knotenanzahl begrenzt.

**Wichtig:** Der Compute‑Shader arbeitet unabhängig von der späteren Darstellung. Er kennt weder Pixel noch Kamera. Dadurch bleibt die Feldberechnung auch bei Kamera‑Zooms oder Rotationen stabil – es werden nur die sichtbaren Knoten neu berechnet.

---

## 4. Fragment‑Shader: Interpolation & Tonemap

Der Fragment‑Shader ist bewusst extrem schlank:

1. **Pixel → Membran‑Raum**: Die Pixelkoordinate wird in das Membran‑Koordinatensystem transformiert.  
2. **Hexagon‑Interpolation**: Aus der Membran‑Koordinate werden die drei umgebenden Mess‑Knoten bestimmt. Da das Gitter regelmäßig und analytisch beschrieben ist, geschieht dies ohne Suche, allein durch mathematische Umformung (schiefe Koordinaten, Modulo‑Operationen).  
3. **Baryzentrische Interpolation**: Die drei Ω‑Werte werden mit ihren baryzentrischen Gewichten gemischt.  
4. **Nebra‑Tonemap**: `log2(|Ω| / max(lvl, 2^-64)) / 8.0 + 0.5` liefert die Luminanz.

Der Fragment‑Shader führt nur **drei Speicherzugriffe** auf `FieldNodeBuffer` durch – deutlich weniger als ein Per‑Pixel‑Ansatz mit 200 Quellen. Die Interpolation glättet die Werte zwischen den Knoten und verhindert sichtbare Diskontinuitäten.

---

## 5. Stufenlose Skalierung („flüssige“ Anpassung)

Die Messdichte wird über einen **kontinuierlichen Regelkreis** gesteuert:

- Ein gleitender Mittelwert der GPU‑Frame‑Zeit (`stableTick`) wird gemessen.  
- Ein Regler (z. B. PID) vergleicht `stableTick` mit einem Zielwert (z. B. 16 ms für 60 fps).  
- Ist die Frame‑Zeit zu hoch, wird `grid_scale` **vergrößert** → weniger Knoten → geringere Last.  
- Ist sie zu niedrig, wird `grid_scale` **verkleinert** → mehr Knoten → höhere Qualität.

Die Änderung von `grid_scale` erfolgt **nicht in diskreten Stufen**, sondern als kontinuierlicher Float‑Wert mit begrenzter Änderungsrate (z. B. maximale relative Änderung pro Frame). Dadurch verschieben sich die Mess‑Knoten bei jeder Anpassung nur minimal. Der Fragment‑Shader interpoliert zwischen den aktuellen Knoten, sodass das Bild visuell „flüssig“ bleibt – kein „Netflix‑Buffering‑Effekt“.

Optional kann der Compute‑Shader die neuen Knotenwerte mit den alten zeitlich filtern (Temporal Anti‑Aliasing), um Flimmern bei schnellen Dichteänderungen zu vermeiden.

---

## 6. Bewertung der Hexagonalen Wabenstruktur

**Mathematisch und WebGPU‑technisch ist ein hexagonales Gitter mit baryzentrischer Interpolation sehr gut umsetzbar und performant.**

- **Geometrie**: Ein Hexagon‑Gitter lässt sich als Dreiecksgitter aus gleichseitigen Dreiecken auffassen. Die Knoten liegen auf einem regelmäßigen Raster mit versetzten Reihen. Die Transformation von Pixelkoordinaten in die drei umgebenden Knoten ist rein analytisch – ähnlich wie bei einem Rechteckgitter, nur mit etwas mehr Mathematik (schiefe Koordinaten, Fallunterscheidung für die versetzten Reihen).  
- **Interpolation**: Baryzentrische Interpolation auf einem Dreieck ist im WGSL‑Fragment‑Shader trivial – nur drei Gewichte und drei `textureLoad`/`SSBO`‑Zugriffe. Im Vergleich zur bilinearen Interpolation auf Rechtecken werden sogar **ein Speicherzugriff weniger** benötigt.  
- **Qualität**: Hexagonale Gitter bieten eine bessere Isotropie und vermeiden richtungsabhängige Artefakte. Für die gleiche visuelle Qualität benötigen sie ca. 13 % weniger Knoten als ein äquivalentes Rechteckgitter. Das ist auf schwacher Hardware ein echter Gewinn.

**Herausforderungen:**

- Die Indizierung ist etwas komplexer als bei einem Rechteckgitter. Man muss im Shader die versetzte Anordnung der Knoten korrekt behandeln. Dies ist aber ein einmaliger Implementierungsaufwand und gut dokumentiert.  
- Für **adaptive Verfeinerung (AMR)** wird ein regelmäßiges Hexagon‑Gitter schnell unhandlich. Sobald man lokal unterschiedliche Dichten zulässt, bricht die analytische Pixel‑zu‑Knoten‑Zuordnung zusammen. Man müsste dann entweder eine Dichte‑Textur verwenden (die das Gitter verzerrt) oder auf ein unregelmäßiges Netz mit Suche umsteigen – beides ist aufwendig und verliert die Performance‑Vorteile.

**Fazit zur Hexagon‑Idee:** Als Basis für eine gleichmäßige, stufenlos skalierbare Messstruktur ist das Hexagon‑Gitter **empfehlenswert**. Für spätere adaptive Verfeinerung sollte man jedoch auf ein hierarchisches Gitter (Quadtree) ausweichen – oder das Hexagon‑Gitter als feinste Stufe innerhalb eines Quadtrees verwenden.

---

## 7. Backend‑Vorgabe für Messdichte (AMR)

Das Backend kann dem Frontend **mitteilen, wo die Mess‑Knoten am dichtesten sein müssen**. Dies ist eine natürliche Erweiterung und ermöglicht echtes **Adaptive Mesh Refinement (AMR)**.

**Vorschlag für die Umsetzung:**

- Das Backend sendet eine **Dichtefunktion** (z. B. als analytische Formel, als Textur oder als Liste von Regionen mit gewünschter Auflösung). Diese kann aus physikalischen Kriterien abgeleitet werden: hohe Feldgradienten, Nähe zu Quellen, schnelle zeitliche Änderungen.  
- Das Frontend nutzt diese Dichtefunktion, um die lokale Knotenplatzierung zu steuern.

**Zwei mögliche Ansätze:**

1. **Verzerrtes regelmäßiges Gitter**: Man moduliert die lokale Gitterkonstante über eine Dichte‑Map. Die Knoten bleiben topologisch regelmäßig, aber ihre Abstände variieren. Die Fragment‑Shader‑Interpolation benötigt dann eine Koordinatentransformation, die die lokale Verzerrung berücksichtigt. Das ist machbar, aber komplex und kann bei starken Dichtegradienten zu numerischen Problemen führen.

2. **Hierarchisches Gitter (Quadtree)**: Das regelmäßige Hexagon‑Gitter wird durch einen Quadtree ersetzt, dessen Blattzellen die Messknoten enthalten. Die Verfeinerung wird durch die Backend‑Dichtefunktion gesteuert: Zellen in Regionen hoher Dichte werden rekursiv unterteilt. Die Fragment‑Shader‑Interpolation wird dann bilinear (innerhalb der Rechteckzellen) oder baryzentrisch (wenn man Dreiecke pro Zelle bildet). Dieser Ansatz ist **wesentlich robuster** und für echtes AMR auf GPUs etabliert.

Da die Aufgabe explizit fragt, ob das Backend AMR vorgeben kann: **Ja, das ist sinnvoll und praktikabel.** Die Architektur sollte so entworfen werden, dass der Schritt vom regelmäßigen Hexagon‑Gitter zu einem hierarchischen Gitter ohne grundlegende Änderungen an Buffer‑Layout und Compute‑Shader möglich ist. Der Compute‑Shader bleibt derselbe (er berechnet Ω an gegebenen Knotenpositionen), nur die Knotenplatzierung und die Fragment‑Interpolation ändern sich.

---

## 8. Fazit

Das vorgestellte Konzept erfüllt alle Anforderungen:

- **Lauffähig auf schwacher GPU**: Durch die Trennung von Messung (Compute, O(Knoten·Quellen)) und Darstellung (Fragment, O(Pixel) mit nur 3 Speicherzugriffen) bleibt die Last kontrollierbar.  
- **Kontinuierlich & lückenlos**: Die baryzentrische Interpolation im Fragment‑Shader garantiert ein glattes Bild ohne sichtbare Gittergrenzen.  
- **Stufenlose Skalierung**: Ein PID‑Regler passt `grid_scale` kontinuierlich an die Frame‑Zeit an; die Knotenpositionen verschieben sich flüssig.  
- **Hexagon‑Idee**: Sehr gut geeignet als Basis‑Gitter, mathematisch und in WGSL performant umsetzbar.  
- **Backend‑AMR**: Als Erweiterung realisierbar, vorzugsweise über ein hierarchisches Quadtree‑Gitter.

Damit ist die „Agnostische Membran“ ein zukunftssicheres, performantes Architekturmuster für die kontinuierliche Visualisierung eines 4D‑Feldes auf extrem schwacher Hardware.

#####################################################################################################

Kimi Chat 2.6 

 # Architektur-Konzept: Agnostische Membran
## Kontinuierliches 4D-Physik-Feld auf schwacher GPU (Intel HD 515)

---

### 1. Das fundamentale Design-Prinzip: Evaluation ≠ Rasterisierung

Das zentrale Erkenntnisproblem ist die Verwechslung von *Feld-Auswertung* und *Bild-Synthese*. Ein Per-Pixel-Fragment-Shader, der 200 Quellen × 2M Pixel auswertet, ist auf einer Intel HD 515 (ca. 300–400 GFLOPS theoretisch, deutlich weniger effektiv, Bandbreite-limitiert) nicht haltbar. Die Lösung ist eine **dreistufige Entkopplung**:

| Stufe | Aufgabe | Wo |
|-------|---------|-----|
| **A** | Adaptive Mesh-Generierung | Compute-Shader |
| **B** | Feld-Auswertung auf Mesh-Nodes | Compute-Shader |
| **C** | Interpolation + Tonemapping | Fragment-Shader |

Der Fragment-Shader berechnet **keine Physik mehr**. Er interpoliert vorab berechnete Node-Werte und wendet die Nebra-Tonemap an. Das reduziert die physikalische Auswertung von O(N×M) auf O(N×K), wobei K die Anzahl der adaptiven Mesh-Nodes ist (typisch 5.000–40.000, nicht 2.000.000).

---

### 2. Buffer-Architektur und Speicherlayout

Die Intel HD 515 teilt den Hauptspeicher mit der CPU (Unified Memory Architecture). Das ist ein Fluch (Bandbreite) und Segen (Zero-Copy möglich). Das Layout muss Cache-Lines respektieren (64 Byte) und Bank-Konflikte im GPU-Speicher vermeiden.

#### 2.1 Quellen-Buffer (`sources`)
```
struct Source {
    position: vec3<f32>,      // 12 Byte
    value: f32,               // 4 Byte
    epoch: f32,               // 4 Byte
    tau: f32,                 // 4 Byte
    extent: f32,              // 4 Byte
    kernel_id: u32,           // 4 Byte
    kraft_type: u32,          // 4 Byte
    velocity: vec3<f32>,      // 12 Byte
    pole: vec3<f32>,          // 12 Byte
    j2: f32,                  // 4 Byte
    j4: f32,                  // 4 Byte
    r_eq: f32,                // 4 Byte
    padding: f32,             // 4 Byte → 64 Byte aligned
}
```
**Wichtig:** 64 Byte pro Quelle = exakt eine Cache-Line. Bei 200 Quellen = 12,8 KB. Das passt komplett in L2-Shared / Workgroup Memory. Der Compute-Shader lädt alle Quellen **einmalig** in `workgroup`-speicher und wiederverwendet sie für alle Nodes der Workgroup.

#### 2.2 Mesh-Node-Buffer (`mesh_nodes`)
Doppelt gepuffert (Ping-Pong) für adaptives Subdivision/Coarsening:
```
struct MeshNode {
    world_pos: vec2<f32>,     // Membran-Koordinaten (x,y)
    z_layer: f32,             // z für 3D-Projektion
    field_value: vec4<f32>,   // Ω für 4 Kanäle (oder mehr, gepackt)
    refinement: f32,          // Lokaler Detailgrad 0.0–1.0
    error_metric: f32,        // Geschätzter Interpolationsfehler
}
```
Speicherbedarf bei 20.000 Nodes: ~480 KB. Vernachlässigbar.

#### 2.3 Mesh-Topologie-Buffer (`mesh_indices`)
Index-Buffer für `drawIndexed`. Enthält die Triangulierung der adaptiven Mesh-Nodes. Bei adaptiven Quad-Trees: 3 Indizes pro Triangle, ca. 2× Node-Anzahl.

#### 2.4 Uniform-Buffer (`frame_state`)
```
struct FrameState {
    view_proj: mat4x4<f32>,
    stable_tick: f32,         // Gemessene Frame-Zeit (ms)
    target_tick: f32,         // Ziel-Framezeit (z.B. 16.67ms)
    time: f32,
    source_count: u32,
    max_depth: u32,           // Aktuelle maximale Baumtiefe
    force_flags: u32,         // Aktive Kraft-Kanäle (Bitmask)
    tonemap_level: f32,       // max(lvl, 2^-64)
}
```

---

### 3. Die Compute-Pipeline: Adaptive Feld-Auswertung

#### 3.1 Stufe A: Mesh-Refinement (Compute-Shader)

**Eingabe:** Ein grobes Seed-Mesh (z.B. 16×16 uniformes Quad-Gitter) + die Quellen-Liste.

**Pro-Node-Logik:**
1. Berechne lokale Feld-Komplexität: `complexity = max(|∇Ω|) * local_source_density`
2. Berechne geschätzten Interpolationsfehler: Wenn vier Nachbar-Nodes existieren, interpoliere den Mittelpunkt und vergleiche mit tatsächlicher Auswertung.
3. **Split-Entscheidung:** `if (complexity > threshold AND error > tolerance) → subdivide`
4. **Merge-Entscheidung:** `if (complexity < threshold_low AND children_error < tolerance) → coarsen`

**Ausgabe:** Neue Node-Liste + neue Index-Liste (Triangulierung).

**Intel HD 515 Spezifika:** 
- Workgroup-Größe: **64** (nicht 256 oder 1024). Die Intel HD 515 hat nur 24 EUs (Execution Units) mit je 7 Threads. Zu große Workgroups blockieren das Scheduling.
- Verwende `workgroupBarrier()` nach dem Laden der Quellen in Shared Memory.
- Vermeide atomare Operationen auf globalen Buffern. Nutze stattdessen **Prefix-Sum-Scan** oder eine zweiphasige Strategie: Zuerst Zählen, dann Allokieren, dann Schreiben.

#### 3.2 Stufe B: Feld-Auswertung (Compute-Shader)

Jeder Node iteriert über alle Quellen (aus Shared Memory) und akkumuliert die 9 Kraft-Kanäle.

**Optimierung für Gravitation (Kernel 0):**
```
let d2 = dot(delta, delta);
let inv_d2 = 1.0 / max(d2, 1.0);  // Kein Softening, aber numerischer Schutz
```
Da alle Nodes dieselben Quellen sehen, ist das Caching in `workgroup`-Speicher essenziell.

**Kraft-Superposition:**
Die 9 Kanäle werden in zwei `vec4<f32>` und einen `f32` gepackt, um Speicherbandbreite zu sparen. Alternativ: Auswertung on-the-fly im Fragment-Shader? **Nein.** Die Node-Anzahl ist um Faktor 50–100 kleiner als die Pixel-Anzahl. Auswertung auf Nodes ist der Sweet Spot.

---

### 4. Die Fragment-Pipeline: Interpolation ohne Lügen

Der Fragment-Shader erhält die interpolierten `world_pos` und `field_value` vom Vertex-Shader (via Varyings). 

**Kritische Erkenntnis:** WebGPU/WGSL interpoliert Varyings **perspektivisch korrekt** und linear im Clip-Space. Das ist für eine 2D-Membran, die flach in der XY-Ebene liegt, identisch mit bilinearer Interpolation im Welt-Raum. Es gibt keine "Lüge" – solange das Mesh fein genug ist.

**Nebra-Tonemap im Fragment-Shader:**
```
let omega = length(interpolated_field.xyz);  // |Ω|
let mapped = log2(omega / max(uniforms.tonemap_level, 1e-20)) / 8.0 + 0.5;
return vec4<f32>(mapped, mapped, mapped, 1.0);
```

**Visuelle Kontinuität:** Die Kontinuität wird nicht durch den Shader, sondern durch die **Mesh-Dichte** garantiert. Wo das Feld glatt ist, sind die Quads groß. Wo es krümmt (nahe Quellen), sind sie klein. Die lineare Interpolation zwischen eng benachbarten Nodes ist visuell identisch mit der analytischen Lösung.

---

### 5. Stufenlose Skalierung: Das Frame-Time-Feedback-System

Das harte Problem ist nicht das Mesh, sondern die **stufenlose Anpassung** an `stableTick`. Ein harter Switch zwischen 128×128 und 64×64 erzeugt den "Netflix-Buffering-Effekt".

**Lösung: Hysterese-gesteuerter LOD-Bias**

```
// Pseudocode für den Refinement-Compute-Shader
let tick_ratio = stable_tick / target_tick;
let target_depth = current_max_depth;

if (tick_ratio > 1.15) {
    target_depth = current_max_depth - 1;  // Coarsen
} else if (tick_ratio < 0.85 && current_max_depth < ABSOLUTE_MAX) {
    target_depth = current_max_depth + 1;  // Refine
}

// Hysterese: Änderung nur alle N Frames oder mit Exponential-Smoothing
max_depth = mix(max_depth, target_depth, 0.1);
```

**Was skaliert stufenlos?**
- Nicht die Grid-Auflösung (die ist diskret: 16×16, 32×32, etc.).
- Sondern der **Refinement-Threshold**: `error_tolerance` und `complexity_threshold` sind kontinuierliche Floats. Wenn die GPU unter Druck steigt, wird der Threshold erhöht → weniger Splits → gröbere, aber immer noch adaptive Triangulierung. Das ist visuell flüssig, da sich die Mesh-Dichte organisch verschiebt.

**Zusätzlich:** Die `target_tick` kann dynamisch sein. Wenn das Backend meldet, dass eine große Epochen-Veränderung bevorsteht (viele Quellen bewegen sich), wird das Ziel-Budget kurzzeitig auf 30 FPS (33ms) gesenkt, um Ruckler zu vermeiden.

---

### 6. Evaluation: Hexagonales Wabenmuster

| Aspekt | Bewertung |
|--------|-----------|
| **Mathematische Effizienz** | ~13% weniger Nodes für gleiche Abdeckung (isotrope Kreis-Packung). Bei adaptiven Meshes irrelevant, da die Topologie sich ohnehin verzweigt. |
| **Baryzentrische Interpolation** | **Albtraum in WGSL.** Hexagonale Zellen sind keine Simplexe. Baryzentrische Koordinaten existieren nur auf Dreiecken. Ein Hexagon müsste in 4–6 Dreiecke trianguliert werden. Die Interpolation auf dem Hexagon selbst erfordert eine nicht-lineare Gewichtsfunktion (Shepard, RBF), die im Fragment-Shader teuer ist. |
| **Nachbarschafts-Logik** | Ein Hexagon hat 6 Nachbarn (vs. 4 beim Quad). Das adaptive Splitting/Merging erfordert komplexere Bookkeeping (halbe Kanten, DCEL-Struktur). In WGSL ohne Pointer und dynamische Allokation extrem fehleranfällig. |
| **WebGPU-technisch** | Index-Generierung für Hex-Meshes ist auf der CPU einfach, auf der GPU (Compute) schwierig. Quad-Trees haben natürliche 1D- und 2D-Index-Mappings (Morton-Codes, Z-Order). |

**Fazit:** Das hexagonale Wabenmuster ist für diesen Anwendungsfall **eine akademisch interessante Sackgasse**. Der Gewinn von 13% weniger Nodes wird durch die massiv erhöhte Komplexität im Compute-Shader und die nicht-triviale Interpolation mehr als aufgezehrt. Für eine schwache GPU ist **Einfachheit = Performance**.

**Empfehlung:** Verwende einen **adaptiven Quad-Tree mit bilinearer Interpolation**. Die "Lüge" der bilinearen Interpolation verschwindet, sobald das Mesh lokal fein genug ist – was der adaptive Refinement-Shader garantiert.

---

### 7. Adaptive Mesh Refinement (AMR) via Backend-Vorgabe

Das Backend kann dem Frontend mitteilen, wo die Messdichte erhöht werden muss, ohne das komplette Mesh zu steuern.

**Mechanismus: Interest-Texture**
- Das Backend sendet eine niedrigaufgelöste `interest_map` (z.B. 64×64, R8_UNORM) via WebSocket.
- Jeder Pixel enthält einen Wert 0.0–1.0: "Wie wichtig ist diese Region?"
- Der Compute-Shader multipliziert den lokalen `complexity`-Wert mit `1.0 + interest * 4.0` vor der Split-Entscheidung.

**Vorteil:** Das Frontend behält die Kontrolle über die absolute Node-Anzahl (Frame-Time-Feedback), aber das Backend verschiebt die Prioritäten. Leere Raumregionen mit `interest = 0` werden nie fein aufgelöst, egal wie viel GPU-Budget verfügbar ist.

**Alternative:** Statt Textur kann das Backend auch 5–10 "Focus-Regions" (Bounding Boxes + Gewicht) senden. Das ist bandbreitenschonender.

---

### 8. Intel HD 515: Spezifische Optimierungen

| Problem | Lösung |
|---------|--------|
| **Bandbreite** | Quellen in `workgroup`-Speicher (12,8 KB). Nodes als `vec4`-aligned Strukturen. |
| **FP32-Overhead** | Wo möglich `f16` verwenden (Intel HD 515 Gen9 unterstützt fp16 in ALUs). `field_value` und `mesh_pos` können halbe Genauigkeit haben. |
| **Divergenz** | Alle Nodes in einer Workgroup sollten ähnliche Quellenzahl sehen. Sortiere Quellen räumlich (z.B. Morton-Order) vor dem Upload, damit Workgroups zusammenhängende Regionen abarbeiten. |
| **Overdraw** | Zeichne das Mesh als einzelnes `triangle-list` mit Backface-Culling. Kein Fullscreen-Quad. |
| **WebSocket-Upload** | Nutze `GPUQueue.writeBuffer()` mit einem staging buffer. Intel HD 515 hat zero-copy UMA – vermeide unnötige `copyBufferToBuffer`. |
| **Double Buffering** | Mesh-Nodes und Indices doppelt puffern (Ping-Pong), damit der Compute-Shader das neue Mesh in Buffer B schreibt, während Buffer A gerendert wird. |

---

### 9. Zusammenfassung der Architektur

```
┌─────────────────┐     ┌──────────────────┐     ┌─────────────────┐
│  Rust Backend   │────▶│  WebSocket       │────▶│  sources buffer │
│  (200 Quellen)  │     │  + interest_map  │     │  (GPU)          │
└─────────────────┘     └──────────────────┘     └─────────────────┘
                                                          │
                                                          ▼
┌─────────────────────────────────────────────────────────────────────┐
│  COMPUTE PASS (Adaptive Mesh Refinement)                            │
│  - Workgroup: 64 Threads                                            │
│  - Lädt Quellen → workgroup memory                                  │
│  - Evaluiert Feld-Komplexität + Backend-Interest                    │
│  - Split/Merge basierend auf error_metric + stable_tick feedback    │
│  - Schreibt: mesh_nodes (Ping-Pong), mesh_indices                   │
└─────────────────────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────────────┐
│  RENDER PASS                                                        │
│  Vertex-Shader: world_pos → clip_space                              │
│  Fragment-Shader: lineare Interpolation der field_value (Varying)   │
│                   + Nebra-Tonemap                                   │
│  Output: 2D-Membran                                                 │
└─────────────────────────────────────────────────────────────────────┘
```

**Das System skaliert stufenlos**, nicht durch harte Grid-Wechsel, sondern durch kontinuierliche Threshold-Anpassung im Refinement-Shader. Es ist lückenlos, weil das adaptiv verfeinerte Mesh die Krümmung des Feldes exakt abbildet. Es ist performant auf der Intel HD 515, weil die physikalische Auswertung auf ~10.000–30.000 Nodes statt 2.000.000 Pixeln erfolgt, mit optimaler Cache-Nutzung und divergenzfreier Workgroup-Ausführung.

#####################################################################################################

Arena Max (ANthropic)

# Architektur-Konzept: Agnostische Membran

## Das Kernproblem in einem Satz

Du brauchst eine **Lookup-Tabelle für ein kontinuierliches Feld**, deren Auflösung atmet — und einen Fragment-Shader, der diese Tabelle so liest, dass keine Pixel-Lücken sichtbar werden.

---

## 1. Die Zwei-Stufen-Architektur

Die Lösung trennt **Messung** (teuer, Compute) und **Darstellung** (billig, Fragment) vollständig.

### Stufe 1: Compute-Shader → Feld-Textur

Ein Compute-Shader evaluiert das Physik-Feld auf einem **regulären 2D-Grid** und schreibt das Ergebnis in eine `texture_storage_2d<rgba32float>`. Jeder Texel speichert die superpositionierte Feld-Stärke (oder einen Vektor aus den dominanten Kraft-Kanälen).

**Warum Textur statt Buffer?**
- Der Fragment-Shader bekommt **Hardware-bilineare Interpolation geschenkt** (`textureSample` mit `linear`-Sampler). Das ist ein einziger Textur-Fetch — schneller als jede manuelle Interpolation.
- Textur-Caches auf Intel HD 515 sind für genau diesen Zugriffsmuster optimiert.
- Kein Index-Gebastel, kein Alignment-Problem.

**Grid-Größe ist variabel:** Die Textur wird mit einer festen **maximalen** Auflösung alloziert (z.B. 256×256), aber der Compute-Dispatch berechnet nur einen **aktiven Teilbereich** `(W_active × H_active)`. Der Rest bleibt unberührt.

### Stufe 2: Fragment-Shader → Bildschirm

Ein Fullscreen-Quad-Fragment-Shader samplet die Feld-Textur mit normalisierten UV-Koordinaten. Er wendet die Nebra-Tonemap an und schreibt Luminanz.

```
UV = fragment_position.xy / screen_resolution
field_value = textureSample(field_texture, linear_sampler, UV * (active_size / max_size))
luminance = log2(abs(field_value) / max(level, 2^-64)) / 8.0 + 0.5
```

Das ist **O(1) pro Pixel**, unabhängig von Quellen-Zahl und Grid-Größe.

---

## 2. Die stufenlose Skalierung ("Atmendes Grid")

### Das Prinzip

Die `stableTick`-Frametime steuert einen **kontinuierlichen Skalierungsfaktor** `σ ∈ [0.0, 1.0]`, der die aktive Grid-Auflösung bestimmt:

```
σ_target = clamp(budget_ms / measured_compute_ms, 0.0, 1.0)
σ = lerp(σ_previous, σ_target, 0.08)      // Trägheit, kein Springen
W_active = round(σ * W_max)                // z.B. σ=0.5 → 128 von 256
H_active = round(σ * H_max)
```

**Warum das funktioniert:** Wenn die GPU 16ms braucht aber 8ms Budget hat, halbiert sich die Grid-Fläche (nicht die Kantenlänge — die Arbeit skaliert quadratisch). Die bilineare Interpolation im Fragment-Shader **füllt die fehlenden Pixel automatisch** — das Bild wird weicher, nie lückenhaft.

**Der Trägheitsterm (`lerp 0.08`)** verhindert Oszillation. Das Grid atmet wie ein Zwerchfell, nicht wie ein Presslufthammer.

### Warum kein Mip-Chain / LOD-System?

Mip-Chains sind für **statische** Texturen. Hier ändert sich der Inhalt jeden Frame. Die Kosten für `generateMipmaps` wären höher als der Gewinn. Ein einzelnes Textur-Level mit variabler Belegung ist einfacher und vorhersagbarer.

---

## 3. Buffer-Layout

### Uniform Buffer: Simulation State (~256 Bytes)

```
struct SimUniforms {
    view_origin: vec2<f32>,      // Kamera-Zentrum in Welt-Koordinaten
    view_extent: vec2<f32>,      // Sichtbarer Bereich
    active_grid: vec2<u32>,      // Aktuell berechnete Texel (W, H)
    max_grid: vec2<u32>,         // Allozierte Textur-Größe
    epoch_now: f64,              // Aktuelle Simulationszeit (64-bit!)
    channel_mask: u32,           // Bitmaske: welche der 9 Kanäle aktiv
    level_ref: f32,              // Tonemap-Referenzpegel
}
```

**Problem: f64 in WGSL.** Intel HD 515 hat kein `f64`. Die Epoch muss als **zwei `f32`** kodiert werden (high + low split, Kahan-Summation im Shader für `dt = epoch_now - source.epoch`). Das ist kritisch für die Zeitkonstante `τ`.

### Storage Buffer: Quellen-Array (~200 × 80 Bytes = 16 KB)

```
struct FieldSource {
    pos: vec3<f32>,
    val: f32,
    epoch: f32,           // Relativ zu einer Referenz-Epoche (Precision!)
    tau: f32,
    extent: f32,
    kernel_id: u32,       // 0-6
    force_type: u32,      // 0-8 → Kanal-Index
    velocity: vec3<f32>,
    pole: vec3<f32>,
    j2: f32,
    j4: f32,
    r_eq: f32,
    _pad: vec2<f32>,      // Alignment auf 16-Byte-Grenzen
}
```

**Alignment ist lebenswichtig** auf Intel. Jedes Struct-Member auf `16-Byte`-Grenzen. Lieber Padding verschwenden als einen unaligned-Load provozieren.

### Storage Texture: Feld-Textur

`texture_storage_2d<rgba32float, write>` im Compute, `texture_2d<f32>` im Fragment.

- **R:** Superpositionierter Skalar-Wert (Haupt-Luminanz)
- **G:** Gradienten-Magnitude (für spätere Visualisierung / AMR-Hinweis)
- **B:** Dominanter Kanal-Index (als Float kodiert, für Farb-Mapping)
- **A:** Temporal-Blend-Faktor (für Ghosting/Trail-Effekte)

Vier Kanäle reichen für die Darstellung. Wenn alle 9 Kraft-Kanäle **gleichzeitig separat** gebraucht werden: ein `texture_storage_2d_array` mit 9 Layern — aber das ist auf Intel HD 515 Gift. Besser: CPU-seitig vorselektieren, maximal 3-4 aktive Kanäle pro Frame.

---

## 4. Compute-Shader: Innere Schleife

### Dispatch-Struktur

```
workgroup_size(8, 8, 1)     // 64 Invocations — Intel-sweet-spot
dispatch(ceil(W_active/8), ceil(H_active/8), 1)
```

**Warum 8×8?** Intel HD 515 hat 24 EUs mit SIMD8. Workgroups >64 Invocations fragmentieren die Belegung. 8×8 passt exakt.

### Innere Schleife (Pseudocode-Logik)

```
grid_pos = global_id.xy
world_pos = view_origin + (grid_pos / active_grid) * view_extent
Ω = 0.0
for i in 0..source_count:
    src = sources[i]
    d = distance(world_pos.xy, src.pos.xy)  // 2D-Projektion auf Membran
    
    // Temporal Decay
    dt = epoch_now - src.epoch
    decay = exp(-dt / src.tau)
    
    // Kernel-Dispatch (switch statt Funktionspointer)
    contribution = evaluate_kernel(src.kernel_id, d, src) * decay * src.val
    
    // Additive Superposition
    Ω += contribution

textureStore(field_tex, grid_pos, vec4(Ω, gradient_mag, channel, 1.0))
```

**Kernel-Dispatch:** Ein `switch(kernel_id)` mit 7 Cases. Kein dynamischer Branching-Albtraum, weil innerhalb einer Workgroup typischerweise **alle Sources den gleichen Pfad nehmen** (die Schleife iteriert über Sources, nicht über Kernel-Typen). Die Divergenz ist **pro Schleiten-Iteration**, nicht pro Pixel.

**Optimierung: Tiling.** Wenn >50 Sources: Die Workgroup lädt 8-16 Sources in `var<workgroup>` Shared Memory, evaluiert alle Pixel gegen diesen Batch, dann nächster Batch. Reduziert Storage-Buffer-Reads um Faktor 8.

---

## 5. Bewertung der Hexagonal-Idee

### Mathematisch: Elegant, korrekt, 13.4% effizienter

Ein hexagonales Sampling-Gitter hat die **höchste Packungsdichte** aller regulären 2D-Gitter. Für ein isotropes Feld (das Gravitationsfelder sind) bedeutet das: gleiche Rekonstruktionsqualität mit 13.4% weniger Samples, oder bessere Qualität bei gleicher Sample-Zahl.

Baryzentrische Interpolation in der Hex-Zelle (3 Eckpunkte statt 4 beim Quad) braucht **weniger Multiply-Adds** als bilineare Interpolation.

### WebGPU-technisch: Ein Albtraum, den Du nicht willst

**Problem 1: Kein Hex-Textur-Format.** `textureSample` mit `linear`-Sampler kann nur auf regulären Grids interpolieren. Für Hex brauchst Du **manuelle Interpolation** im Fragment-Shader:
- Welt-Position → Hex-Zelle finden (2 Divisionen, Rundung, Fallunterscheidung ungerade/gerade Reihe)
- 3 nächste Nachbar-Texel bestimmen (Index-Arithmetik mit Branching)
- 3× `textureLoad` (kein Cache-kohärenter Zugriff, da versetztes Muster)
- Baryzentrische Gewichte berechnen (2 Kreuzprodukte)
- Gewichtete Summe

Das sind **~25 ALU-Ops + 3 Textur-Loads mit semi-zufälligem Zugriffsmuster** statt **1 Hardware-interpolierter Textur-Load**. Auf Intel HD 515 kostet ein `textureLoad` mit Cache-Miss ~200 Takte. Der 13% Sampling-Vorteil wird durch den 10× teureren Rekonstruktions-Schritt **vollständig aufgefressen**.

**Problem 2: Compute-Dispatch.** Das Hex-Grid hat keine natürliche Abbildung auf `workgroup_size(x,y,1)`. Jede zweite Reihe ist versetzt. Du verlierst Workgroup-Kohärenz und musst Spezial-Indexing im Compute-Shader machen.

**Problem 3: Debugging.** `textureStore` in ein Hex-Layout lässt sich nicht trivial in einem Textur-Viewer inspizieren.

### Verdikt

**Nicht machen.** Die Intel-HD-515-GPU ist **Textur-Sampler-limitiert, nicht ALU-limitiert**. Der eine kostenlose `textureSample`-Call mit Hardware-Bilinear schlägt jede manuelle Interpolation. Die 13% Grid-Effizienz sind real, aber irrelevant gegenüber dem Faktor 10 im Fragment-Shader.

**Wenn Du Sampling-Effizienz willst:** Rotiere das reguläre Grid um 26.57° (= `atan(0.5)`). Das ergibt ein "Quincunx"-Muster, das ~10% effizienter samplet als axis-aligned — und **trotzdem mit Standard-Bilinear funktioniert**, weil die Rotation in der UV-Transformation passiert, nicht im Speicherlayout.

---

## 6. Adaptive Mesh Refinement (AMR) vom Backend

### Konzept: Machbar und wertvoll, aber als Phase 2

Das Backend kennt die Quellen-Positionen und kann eine **Dichtekarte** berechnen:

```
Backend sendet: importance_map[32×32] = f32
```

Jede Zelle gibt an, wie viel Rechenzeit dort investiert werden soll (basierend auf Quellen-Dichte, Gradienten-Schätzung, Nutzer-Fokus).

### Umsetzung auf GPU-Seite

**Nicht als unstrukturiertes Mesh** (das ist auf Intel unbezahlbar). Stattdessen: **Multi-Resolution-Kacheln.**

Die Feld-Textur wird in 8×8-Kacheln unterteilt. Jede Kachel hat ein **LOD-Level** (0 = volle Auflösung, 1 = halbe, 2 = viertel). Der Compute-Dispatch berechnet:

- LOD 0: Jedes Texel der Kachel (8×8 = 64 Invocations)
- LOD 1: Jedes 2. Texel, die Zwischenwerte bleiben vom letzten Frame (temporal)
- LOD 2: Jedes 4. Texel

Die `importance_map` bestimmt das LOD pro Kachel. Der Fragment-Shader ändert sich **nicht** — er samplet immer die gleiche Textur. Die LOD-Grenzen verschwimmen durch bilineare Interpolation natürlich.

**Kosten:** Ein zusätzlicher Indirektions-Buffer (`lod_per_tile[N]`), ein leicht komplexerer Dispatch (indirect dispatch buffer, den die CPU füllt). Kein Hexagoner Wahnsinn.

### Warnung

AMR lohnt sich erst ab **>100 Sources mit räumlicher Clusterung**. Bei 10-50 gleichverteilten Sources ist ein uniformes Grid nicht langsamer, weil der Overhead der LOD-Verwaltung die eingesparte Rechenzeit auffrisst.

---

## 7. Zusammenfassung der Architektur

```
┌─────────────────────────────────────────────────────────┐
│  Rust Backend (WebSocket)                               │
│  → Filtered Sources Array (10-200 × 80B)                │
│  → [Optional Phase 2: importance_map 32×32]             │
└──────────────┬──────────────────────────────────────────┘
               │ WebSocket, ~60Hz oder on-change
               ▼
┌─────────────────────────────────────────────────────────┐
│  JavaScript / TypeScript Host                           │
│  1. Schreibe Sources → GPU Storage Buffer               │
│  2. Messe stableTick → berechne σ → setze active_grid   │
│  3. Schreibe Uniforms → GPU Uniform Buffer              │
│  4. Dispatch Compute (ceil(W_a/8), ceil(H_a/8), 1)     │
│  5. Draw Fullscreen Quad (Fragment liest Feld-Textur)   │
└──────────────┬──────────────────────────────────────────┘
               │
    ┌──────────┴──────────┐
    ▼                     ▼
┌────────────┐    ┌──────────────────┐
│  Compute   │    │  Fragment Shader  │
│  Shader    │    │                  │
│            │    │  UV → texSample  │
│  8×8 WG    │    │  → Tonemap      │
│  Sources   │──▶ │  → Luminanz     │
│  → Σ Ω    │    │                  │
│  → texStore│    │  O(1) pro Pixel  │
└────────────┘    └──────────────────┘
  Feld-Textur       Sampler(linear)
  rgba32float
  256×256 max
  W_a×H_a active
```

### Kritische Zahlen für Intel HD 515

| Parameter | Wert | Begründung |
|---|---|---|
| Max Textur | 256×256 | 1MB @ rgba32float. Mehr frisst den winzigen L3. |
| Workgroup | 8×8 | 64 Invocations = 8 SIMD8-Lanes × 8 Reihen |
| Source-Batching | 16 pro Tile-Load | Shared Memory auf Intel: 64KB pro Subslice |
| Min Grid | 32×32 | Darunter sieht man Texel-Kanten trotz Bilinear |
| Target Compute | <4ms | Lässt 12ms für Fragment + Presentation |

### Was Du **nicht** brauchst

- Keine Vertex-Buffer für ein Mesh (Fullscreen-Quad genügt, 3 Vertices ohne VB)
- Kein Hex-Grid
- Kein Mipmap-Chain
- Kein Multi-Pass für die 9 Kanäle (in einem Pass superpositionieren)
- Keinen Depth-Buffer

#####################################################################################################

Claude Sonnet 5 Arena

# Architektur-Konzept: Die "Agnostische Membran"

## 1. Grundproblem und Strategiewechsel

Das Kernproblem ist eine Fehlallokation von Rechenzeit: Ein Fragment-Shader verschwendet Zyklen auf Pixel-Präzision in Regionen, wo sich das Feld über hunderte Pixel kaum ändert (glatte Gravitationsgradienten im "leeren" Raum), während er in der Nähe von Quellen (steile `1/d²`-Gradienten) eigentlich mehr Auflösung bräuchte, als er bekommt.

Die Lösung ist eine **Entkopplung von Physik-Sampling und Pixel-Rasterung** durch ein bewegliches, adaptives Mesh, kombiniert mit **Compute-Shadern zur Vorverdichtung** der Quelldaten. Wir bauen eine dreistufige Pipeline:

```
[Rust/WebSocket] → [Stage A: Field Compaction] → [Stage B: Mesh Sampling] → [Stage C: Fragment Interpolation]
   Quellen (N)         Compute Shader              Compute Shader              Fragment Shader
```

Die zentrale Erkenntnis: **N (Quellen) und M (Mesh-Nodes) dürfen sich niemals im Fragment-Shader treffen.** Die O(N×M)-Auswertung findet ausschließlich in einem Compute-Shader auf einem kleinen, adaptiven Node-Gitter statt (typisch M = 2.000–20.000 Nodes, nicht 2 Mio Pixel). Der Fragment-Shader tut danach nur noch O(1) Interpolationsarbeit pro Pixel.

## 2. Buffer-Layout

**Buffer 1 – `SourceBuffer` (Storage, read-only, GPU)**
Struct-Array direkt aus dem WebSocket-Payload gemappt (std430-kompatibel gepackt: vec3 immer auf 16-Byte Grenzen ausrichten, um Padding-Fallen bei Kernel_ID/Kraft_Type als u32 zu vermeiden). Wird bei jedem WebSocket-Update per `writeBuffer` aktualisiert – kein Re-Alloc, feste Kapazität (z.B. 256 Slots), aktive Anzahl `N_active` als Uniform.

**Buffer 2 – `MeshNodeBuffer` (Storage, read_write)**
Das eigentliche adaptive Netz. Kein festes 128×128-Grid, sondern eine **flache Liste von Node-Deskriptoren**: `{ position: vec2<f32>, level: f32 (Ω-Wert), density_weight: f32, valid: u32 }`. Die Topologie (wer ist Nachbar von wem) wird NICHT explizit gespeichert, sondern implizit über eine reguläre logische Indexstruktur (Quadtree-Tile oder hex-Zeilen-Offset) rekonstruiert – das spart einen Adjazenz-Buffer.

**Buffer 3 – `TileControlBuffer` (Storage, vom CPU/Backend beschreibbar)**
Ein grobes Kontrollraster (z.B. 32×32 Tiles über den Screen), das pro Tile eine `refinement_level: f32` (0.0 = grob, 1.0 = maximal fein) sowie optional einen `priority_hint` vom Rust-Backend enthält. Dies ist der Hook für AMR (siehe Punkt 4).

**Buffer 4 – `Uniforms`**
`stableTick`, `viewport`, `zoom`, `time`, `globalDensityBudget` (die stufenlose Skalierungsgröße, siehe unten).

**Buffer 5 – `IndirectDrawBuffer`**
Da die Node-Anzahl pro Frame variiert (adaptive Dichte!), wird die Dreiecks-/Punktanzahl für den Renderpass nicht CPU-seitig fixiert, sondern über `drawIndirect` aus einem Compute-Pass geschrieben (Node-Count → Vertex-Count). Das ist essentiell für "stufenlos" – die GPU entscheidet selbst, wie viele Primitive sie zeichnet.

## 3. Pipeline-Stages im Detail

### Stage A: Field Compaction (Compute, 1× pro Frame, sehr leichtgewichtig)
Kein Rechen-Schritt im eigentlichen Sinn, sondern Housekeeping: Sourcen nach Relevanz für den aktuellen Viewport clippen/sortieren (Frustum-Cull auf `extent`), damit Stage B nicht 200 Quellen gegen Nodes testet, die weit außerhalb liegen. Bei 10–200 Quellen ist das primär Bandbreiten-, nicht Rechenoptimierung, aber wichtig für Vorhersagbarkeit.

### Stage B: Mesh Sampling (Compute, workgroup pro Node)
Der einzige Ort, an dem tatsächlich N×M-Physik gerechnet wird. Jede Workgroup nimmt einen Node, iteriert über die (bereits gefilterten) aktiven Quellen, akkumuliert additiv über alle 9 Kraft-Kanäle (`Ω_channel[9]` als lokales Array in Registern/Workgroup-Shared-Memory), wendet die jeweilige Kernel-Funktion (0–6) je Quelle an, superpositioniert, und schreibt am Ende den Nebra-tonemappten Luminanzwert plus die rohen Kanal-Werte (für spätere Farbkodierung/Force-Type-Visualisierung) in den `MeshNodeBuffer`.

Kostenrechnung: bei M = 5.000 Nodes und N = 200 Quellen sind das 1.000.000 Kernel-Auswertungen im Compute-Shader – das ist für eine HD 515 in einem gut parallelisierten Compute-Pass (workgroup_size 64, keine divergenten Branches wenn Kernel-Dispatch per uniform Branching statt per-invocation) deutlich verdaulicher als 2.000.000 Pixel × 200 Quellen = 400 Mio Operationen im Fragment-Shader.

### Stage C: Fragment Interpolation (Vertex+Fragment, kein Compute mehr)
Die Nodes werden als Dreiecksnetz (aus der impliziten Topologie trianguliert, siehe Kapitel 5) gerendert. Der Vertex-Shader reicht `Ω`/Luminanz als Interpolant durch, der Fragment-Shader macht **nur noch** die finale Tonemap-Kompression (log2-Skalierung) und Farbkodierung – GPU-natives lineares/baryzentrisches Interpolieren übernimmt der Rasterizer selbst, kostenlos. Das ist der Trick: WebGPUs Fixed-Function-Interpolation ist quasi umsonst, während eine manuelle bilineare Interpolation im Shader Bandbreite und ALU kostet.

## 4. Stufenlose Dichte-Adaption ("Flüssiges Netz")

Statt diskreter LOD-Sprünge (128×128 → 64×64) wird die Nodedichte über eine **kontinuierliche Kostenfunktion** gesteuert, die jeden Frame nachjustiert wird:

```
targetNodeCount(t) = clamp(targetNodeCount(t-1) * adjustFactor, MIN, MAX)
adjustFactor = smoothstep-artige Reaktion auf (stableTick / frameTimeBudget)
```

Konkret: Ein kleiner Compute-Pass (oder sogar CPU-seitig, da nur ein Skalar) vergleicht `stableTick` (gemessene Frame-Zeit, exponentiell geglättet über z.B. 8–16 Frames zur Vermeidung von Single-Frame-Spikes) gegen ein Ziel-Budget (z.B. 12ms bei 60Hz-Ziel). Die Abweichung steuert einen **kontinuierlichen Skalar `densityBudget ∈ [0,1]`**, der nicht die Grid-Auflösung direkt setzt, sondern einen Parameter für die Subdivision-Tiefe pro Tile im `TileControlBuffer`.

Der entscheidende Kniff gegen den "Netflix-Buffering-Ruckler": **Hysterese + Trägheit**. `densityBudget` bewegt sich pro Frame nur um einen kleinen Bruchteil (`lerp(current, target, 0.05)`) in Richtung des von `stableTick` geforderten Werts. Dadurch entstehen keine sichtbaren Sprünge im Mesh, sondern ein langsames "Fließen" der Nodedichte – exakt das gewünschte Flüssigkeitsverhalten. Die Node-Positionen selbst werden nicht neu gewürfelt, sondern bestehende Nodes werden bei Verdichtung lokal unterteilt (Midpoint-Split) bzw. bei Verdünnung zusammengefasst (Nachbar-Merge), sodass keine Popping-Artefakte durch komplette Neuvermaschung entstehen.

Die Subdivisionstiefe selbst wird nicht global, sondern **pro Tile** (aus `TileControlBuffer`) gerechnet: `localLevel = baseLevel(densityBudget) + gradientBoost(|∇Ω|) + backendHint`. Der Gradient-Boost sorgt dafür, dass Regionen mit steilen Feldänderungen (nahe an Quellen mit kleinem `extent`) automatisch feiner aufgelöst werden als leerer Raum – das löst das "verschwendete Rechenzeit in leeren Regionen"-Problem strukturell, nicht nur durch rohe Node-Reduktion.

## 5. Bewertung: Hexagonales Wabenmuster

**Mathematisch:** Ja, uneingeschränkt machbar. Hex-Sampling ist tatsächlich informationstheoretisch überlegen (die 13%-Zahl bezieht sich auf die Nyquist-optimale Abtastdichte für isotrope 2D-Signale – ein Vorteil, der bei radial-symmetrischen Feldern wie Gravitationspotentialen sogar besonders gut passt, da Aliasing bei radialsymmetrischen Quellen mit hexagonalem Sampling gleichmäßiger verteilt wird statt sich in Achsenrichtungen zu konzentrieren).

**WebGPU-technisch:** Hier trennt sich die Theorie von der Praxis in mehreren Punkten:

- **Rasterizer-Nutzung:** Ein Hexagon ist kein natives Primitiv. Man muss es als 4 oder 6 Dreiecke (Fan um Zentrum) triangulieren. Damit verliert man den größten Vorteil des Vorschlags – man rechnet dann wieder mit Dreiecks-Baryzentrik, nur mit mehr Dreiecken pro "logischer Zelle" als beim einfachen Quad-Grid (2 Dreiecke). Netto-Ersparnis bei Vertex-Count ist real, aber kleiner als die 13% suggerieren, weil die Fan-Triangulierung Overhead erzeugt.

- **Adjazenz-Komplexität:** Die größte praktische Hürde ist nicht das Rendern, sondern die **adaptive Verfeinerung**. Bei einem Quad-/Triangle-Grid ist Midpoint-Subdivision (1 Quad → 4 Quads, oder 1 Dreieck → 4 Dreiecke) ein seit Jahrzehnten gelöstes, GPU-freundliches Muster (vgl. Terrain-LOD-Quadtrees). Für ein Hex-Grid gibt es **kein sauberes rekursives Subdivisionsschema** — Hexagone lassen sich nicht in kleinere Hexagone gleicher Form zerlegen (das ist geometrisch unmöglich, ein Hexagon zerfällt in Dreiecke oder in eine Mischung aus kleineren Hexagonen und Dreiecken/Rauten an den Nahtstellen). Für unser Kernbedürfnis – stufenlose, lokale Verdichtung – ist das ein Show-Stopper oder zumindest ein erheblicher Mehraufwand an Spezialfall-Logik.

- **Indexierung:** Hex-Koordinatensysteme (axial/cube coordinates) sind in CPU-Code elegant, aber in WGSL-Compute-Shadern (wo man Nachbarschaft aus einem linearen Buffer-Index ableiten will) umständlicher als das triviale `row*width+col`-Schema von Quads. Jede Nachbarschaftsabfrage (für Gradient-Boost, Merge/Split) braucht mehr Verzweigungslogik.

**Urteil:** Hex-Grid ist mathematisch reizvoll, aber für ein **adaptives** Mesh auf schwacher Hardware ein Aufwand-Nutzen-Fehlschlag. Der Performance-Gewinn (13% weniger Samples bei gleicher Nyquist-Qualität) wird durch Triangulierungs-Overhead größtenteils aufgefressen, während die Implementierungskomplexität für adaptive Subdivision massiv steigt. **Empfehlung: Nicht für v1.** Stattdessen: reguläres Dreiecksgitter aus Quad-Diagonalen (nicht Quads selbst – Dreiecke geben dem Rasterizer direkt echte baryzentrische Interpolation ohne Bilinear-Krücke), mit klassischer Quadtree-artiger Midpoint-Subdivision. Das liefert 90% des Hex-Vorteils (glattere Interpolation als reines Achsen-Bilinear) bei einem Bruchteil der Komplexität. Hex-Sampling könnte man als spätere Forschungs-Iteration für ein **statisches** Hintergrund-Grid (z.B. den weit entfernten "leeren Raum"-Bereich, der ohnehin nie subdividiert wird) evaluieren, wo die Adjazenz-Probleme nicht auftreten, weil dort nie verfeinert wird.

## 6. AMR: Backend-gesteuerte Verdichtung

Ja, und das Konzept ist bereits im `TileControlBuffer` (Kapitel 2) angelegt. Das Rust-Backend kennt die Quellen exakt (Position, `extent`, Kraft-Typ, sogar prädizierte Bewegung via `velocity`) und kann daher **analytisch** vorherberechnen, wo hohe Gradienten zu erwarten sind – etwas, das das Frontend sonst erst durch Sampling entdecken müsste (reaktiv statt prädiktiv).

Konkreter Mechanismus: Bei jedem WebSocket-Update sendet das Backend zusätzlich zum `SourceBuffer` einen kompakten `RefinementHint`-Stream – z.B. eine Liste von `(tileX, tileY, priority)`-Tripeln für Tiles, die eine Quelle mit kleinem `extent` oder hohem `|val|/tau`-Verhältnis enthalten (schnelle zeitliche Änderung = potenziell scharfe visuelle Kante). Das Frontend schreibt diese Hints per `writeBuffer` in den `TileControlBuffer` und die Stage-B-Subdivisionslogik (Kapitel 4) berücksichtigt sie additiv zum lokal gemessenen Gradienten.

Wichtig für Stabilität: Der Hint ist ein **Bias, kein Diktat** – das Frontend behält die finale Kontrolle über die Gesamt-Node-Zahl (`densityBudget`-Deckel), damit ein Backend, das versehentlich "alles ist wichtig" sendet, nicht die Framerate der schwachen GPU zerstört. Das Verhältnis ist: Backend sagt *wo* (relative Priorität), Frontend entscheidet *wie viel* (absolutes Budget, `stableTick`-gesteuert). Diese Trennung macht das System resilient gegen einen "gierigen" Server und lässt es dennoch von domänenspezifischem Wissen profitieren, das rein bildbasierte Heuristiken (Gradient-basiertes Refinement allein) nicht hätte – z.B. eine Quelle, die gerade erst gespawnt wurde und noch keinen sichtbaren Gradienten erzeugt hat, aber laut `velocity` gleich in einen Bereich einlaufen wird.

## 7. Zusammenfassung der Kernentscheidungen

| Aspekt | Entscheidung | Begründung |
|---|---|---|
| N×M-Auswertung | Compute-Shader auf Mesh-Nodes, nicht Fragment-Shader | Reduziert Operationen um Faktor ~50-400x |
| Interpolation | Dreiecks-Baryzentrik via Hardware-Rasterizer | Kostenlose Fixed-Function statt manuelles Bilinear |
| Mesh-Topologie | Reguläres Dreiecksgitter mit Quadtree-Subdivision | Adjazenz trivial, Subdivision GPU-erprobt |
| Skalierung | Kontinuierlicher `densityBudget`-Skalar mit Lerp-Trägheit | Verhindert Popping, echtes "Fließen" |
| Hex-Grid | Verworfen für v1 | Subdivision geometrisch ungelöst, Aufwand > Nutzen |
| AMR | Backend liefert Priority-Hints, Frontend behält Budget-Hoheit | Domänenwissen nutzen, ohne Kontrolle abzugeben |

#####################################################################################################

GLM 5.2 Chat

Hier ist das Architektur-Konzept für die „Agnostische Membran“, optimiert für die extremen Einschränkungen einer Intel HD 515 GPU (Skylake-Architektur, extrem begrenzte ALUs, Textur-Bandbreite und Register-Datei).

---

# Architektur-Konzept: Die Agnostische Membran (WGSL)

## 1. Grundparadigma: Messung vs. Darstellung (Entkopplung)
Der Fatalste Fehler auf schwacher Hardware wäre, Physik (Messung) und Pixel-Ausgabe (Darstellung) im Fragment-Shader zu koppeln. Wir trennen dies rigoros in eine **Compute-Pass (Messung)** und eine **Fragment-Pass (Interpolation & Tonemap)**. 

Das Bild wird nicht pro-Pixel berechnet, sondern auf einem variablen Mess-Netz. Die Lücken zwischen den Messpunkten werden durch native Hardware-Interpolation im Fragment-Shader gefüllt. Da die Intel HD 515 Linear-Filterung in Hardware extrem schnell kann (kostenlos im Vergleich zu ALU-Operationen), nutzen wir das als Haupttreiber.

## 2. Buffer & Speicher-Strategie
Wir vermeiden UBOs für die Quellen, da diese Size-Limits haben und bei 200 Quellen mit vielen Floats überlaufen. Wir nutzen RoH-Buffer (Read-only Hardware):
*   `Src_Buffer` (StorageBuffer, read-only): Enthält die gefilterten Quellen (Position, val, t, tau, etc.). Format: Array of Structs, strided für Alignment.
*   `Field_Target` (Texture2D, RGBA16Float): Das Render-Target für den Compute-Shader. Speichert die 9 Kraft-Kanäle komprimiert (z.B. in 3 RGBA-Textures oder einem Array-Texture-Layer). Wichtig: *Diese Textur muss mit `linear` Sampling betrieben werden*.

## 3. Die stufenlose Skalierung (Fluid Mesh Scaling)
Ein hartes Raster ruckelt beim Skalieren, weil es Topologie-Wechsel erfordert. Die Lösung für schwache GPUs ist **Progressive Blue Noise Akkumulation** (Temporal Refinement):
Wir behalten eine *fixe* Maximal-Größe des Compute-Grids (z.B. 256x256 Punkte). In jedem Frame berechnet das Backend/WASM basierend auf `stableTick` einen Füllgrad $K \in [0.0, 1.0]$.
*   Ist die Frame-Zeit schlecht (z.B. 30ms), sinkt $K$ auf z.B. 0.2.
*   Der Compute-Shader wird nur mit `ceil(256 * K)` Workgroups pro Dimension dispatched.
*   **Der Trick der Stufenlosigkeit:** Wir nutzen kein quadratisches Streichen, sondern eine Halton- oder Blue-Noise-Sequenz. Der Compute-Shader liest aus einem statischen Look-Up-Textur (Blue Noise), *welche* Gitterpunkte in diesem Frame berechnet werden sollen. 
*   Vor dem Compute-Pass kopieren wir das `Field_Target` des letzten Frames in ein `History_Target` (Ping-Pong). Der Compute-Shader schreibt nur die $K$% neuen Werte in `Field_Target`, alle anderen Pixel behalten den Vorjahreswert. 
*   **Ergebnis:** Bei schlechter Performance verdichtet sich das Bild nicht zu Klumpen, sondern die Messpunkte "rieseln" wie Regen über den Bildschirm und glätten sich temporal. Keine Topologie-Änderung, kein Ruckeln, stufenlos anpassbar.

## 4. Compute-Shader vs. Fragment-Shader
**Compute-Shader (Die Messung):**
*   Liest die Liste der aktiven Punkte für diesen Frame (via Blue Noise LUT).
*   Für jeden aktiven Punkt: Iteriert über das `Src_Buffer`-Array (max. 200).
*   Berechnet für jede Quelle die 9 Kraft-Kanäle (Gravitation, etc.). Wir verzweigen (`switch` auf `Kernel_ID`) nicht pro Pixel, sondern gruppieren Quellen idealerweise im Backend, oder akzeptieren leichte Divergenz in der Workgroup.
*   Schreibt das Ergebnis (Superposition der Kräfte) in das `Field_Target`.

**Fragment-Shader (Die Membran):**
*   Rendert ein Fullscreen-Quad.
*   Liest aus `Field_Target` mittels `textureSample` (Lineare Interpolation).
*   Die 9 Kanäle werden addiert zu $\Omega$.
*   Wendet die Nebra-Tonemap an: `log2(abs(omega) / max(lvl, 2e-64)) / 8.0 + 0.5`.
*   Gibt die Luminanz aus.

## 5. Evaluierung der Hexagonal-Idee
**Urteil: WebGPU-technisch umsetzbar, aber mathematisch auf der Intel HD 515 ein Albtraum.**
*   *Theorie:* Hexagonales Sampling ist 13% effizienter, da es die optimale 2D-Kugelpackung ist (isotrope Frequenzantwort).
*   *Praxis in WGSL:* WebGPU (und die Intel Treiber) bieten native Hardware-Filterung (Bilinear) **nur für regelmäßige quadratische Grids**. Würdest du ein Hex-Raster verwenden, müsstest du im Fragment-Shader die 3 nächsten Nachbarn suchen und baryzentrische Interpolation per Hand (ALU) durchführen.
*   *Das Problem:* Eine manuelle baryzentrische Interpolation kostet ca. 6-8 Texture-Fetches und Divisonen pro Pixel. Bei 2 Millionen Pixeln auf einer Intel HD 515 brennt die GPU durch. Die 13% Ersparnis beim Compute werden durch den 300% höheren Aufwand beim Fragment-Rendering mehr als vernichtet.
*   *Fazit:* Verwerfen. Nutze ein quadratisches Grid. Die Hardware-Bilinear-Interpolation von WebGPU ist auf Low-End-Hardware der absolute Flaschenhals-Sparer.

## 6. Adaptive Mesh Refinement (AMR) durch das Backend
Das ist nicht nur möglich, sondern der absolut korrekte Weg für die Zukunft der Pipeline. Das Backend weiß, wo die Gravitation steil abfällt (hohe Gradienten, nahe Quellen).
*   **Konzept:** Das Backend schickt neben den Quellen ein 1D-Array von "Regionen" (Bounding Boxes) mit, die eine hohe Messdichte erfordern.
*   **Umsetzung in WebGPU:** Statt die Auflösung des Haupt-Textures zu ändern (unmöglich ohne Neu-Allokation), nutzen wir ein **Hierarchisches Textur-System (Clipmaps)**.
    *   Layer 0: 128x128 für die globale Membran.
    *   Layer 1..N: 64x64 Texturen, die in hoher Auflösung dort reingerechnet werden, wo das Backend es fordert.
*   Im Compute-Shader prüfen wir, ob der aktuelle Pixel in eine Backend-ROI fällt. Wenn ja, wird er in die feinere Textur (und eine Offset-Tabelle) geschrieben.
*   Im Fragment-Shader wird zuerst die grobe Basis-Textur gesamplet, dann geprüft, ob an dieser UV-Position eine feinere ROI-Textur existiert. Wenn ja, wird deren Wert verwendet (bzw. per Lerp überblendet).
*   Dies verhindert das "Netflix-Buffering" und nutzt die Compute-Zeit ausschließlich dort, wo die Nebra-Tonemap durch steile Gradienten sonst aliasen (flickern) würde.

## Zusammenfassung der Pipeline
1. **WS-Update:** Rust sendet Quellen (Array) und optionale AMR-Boxen.
2. **CPU/WASM:** Liest `stableTick`, berechnet $K$ (Dichte 0..1).
3. **Compute Pass 1:** Dispatcht $K$% der Punkte nach Blue-Noise-Verteilung auf 128x128 Grid. Schreibt in `Field_Target`.
4. **Fragment Pass:** Fullscreen-Quad, Samplet `Field_Target` (Bilinear), wendet Nebra-Tonemap an. (Später erweiterbar um AMR-Sampling).

#####################################################################################################

Claude Sonnet 5 High Chat 

# Architektur-Konzept: Die Agnostische Membran
### Kontinuierliche 4D-Feld-Evaluierung auf schwacher Hardware (Intel HD 515)

---

## 1. Grundprinzip: Entkopplung von Physik und Pixel

Das Kernproblem ist eine Verwechslung zweier unabhängiger Auflösungen: die **physikalische Auflösung** (wie fein muss ich das Feld abtasten, um es korrekt zu rekonstruieren?) und die **Display-Auflösung** (wie viele Pixel hat der Bildschirm?). Der Per-Pixel-Shader zwingt beide auf denselben Wert – das ist der O(N·M)-Suizid.

Die Lösung ist eine strikte **Zwei-Pass-Pipeline** mit einer dritten, unsichtbaren Domäne dazwischen:

```
Compute-Pass (Physik)  →  Node-Buffer (Ω-Feld)  →  Fragment-Pass (Optik)
   O(N × G)                  Zwischenspeicher         O(Pixel), kein N mehr
```

`G` ist die Anzahl der Messknoten (typisch 64² bis 128², *nicht* 2 Mio.). Damit sinkt die Physik-Last von ~400 Mio. Operationen/Frame (2M px × 200 Quellen) auf ~3–16 Mio. – ein Faktor von 25–130×, exakt in dem Bereich, den eine HD 515 verkraftet. Der Fragment-Shader sieht die Quellen **nie**; er kennt nur noch das bereits superpositionierte Ω-Feld und interpoliert.

---

## 2. Buffer-Layout

| Buffer | Typ | Inhalt | Update-Frequenz |
|---|---|---|---|
| `SourceBuffer` | Storage Buffer (read-only, Compute) | Array `Source[≤200]`, 16-Byte-aligned gepackt (pos, val, t, tau, extent, kernel_id, force_type, velocity, pole, J2, J4, r_eq) | pro WS-Snapshot, double-buffered (Ping-Pong), damit der Compute-Pass nie einen halb geschriebenen Frame liest |
| `SimParams` | Uniform Buffer | time, stableTick (EMA), Q (Qualitätsfaktor), Viewport, Kamera-Transform, Nebra-Tonemap-Parameter (`lvl`) | pro Frame |
| `FieldTexture` | `texture_storage_2d<rgba16float>` mit **doppelter Usage-Flag** (STORAGE_BINDING + TEXTURE_BINDING) | Ω im R-Kanal, optional Gradient/Age in G/B/A | pro Compute-Dispatch |
| `PriorityBuffer` | Storage Buffer, r32float, feste Maximalgröße | Refresh-Priorität pro Knoten (siehe 3.2) | akkumulierend, persistent über Frames |

Wichtig: `FieldTexture` wird **einmal** in Maximalgröße allokiert und nie neu erzeugt. Eine Re-Allokation (`createTexture`) ist auf Intel-iGPUs ein Stall-Kandidat – das wäre der eigentliche Grund für den "Netflix-Buffering-Effekt", den du beobachtest, nicht die Bilinearität selbst.

`rgba16float` statt `r32float`, weil 32-Bit-Float-Texturen in WebGPU nur mit optionalem Feature `float32-filterable` linear samplebar sind – auf Intel HD 515 nicht garantiert. `rgba16float` ist per Core-Spec filterfähig und liefert dir hardwarebeschleunigtes Bilinear quasi gratis (ein Sampler-Tap statt manueller Interpolationslogik im Fragment-Shader).

---

## 3. Compute-Pass: Sampling statt Raster

### 3.1 Basis-Evaluierung
Ein Thread pro Knoten, nicht pro Pixel. Jeder Thread iteriert über `SourceBuffer`, wertet die 9 Kraft-Kanäle gemäß `kernel_id`/`force_type` aus und summiert additiv zu Ω. Das ist strukturell identisch zum harten 128×128-Grid, das du schon hast – der Unterschied liegt in Abschnitt 3.2/3.3.

### 3.2 Kontinuierliche Skalierung – das "flüssige" Netz
Statt die Grid-*Auflösung* diskret zu ändern (128→96→64, was zwangsläufig poppt), bleibt die Topologie fest auf Maximalgröße. Variiert wird stattdessen, **wie oft** jeder Knoten neu berechnet wird – ein Prioritäts-Scheduler:

- Jeder Knoten bekommt eine Priorität aus zwei Termen: Nähe zu einer Quelle (abgeleitet aus deren `extent`-Feld – das ist bereits dein AMR-Signal, siehe 5.) und Volatilität (`tau`, `velocity` – schnell bewegte/kurzlebige Quellen erzwingen häufigere Updates).
- Der globale Qualitätsfaktor `Q ∈ [0,1]` wird aus `stableTick` per EMA + Hysterese-Regler (Schmitt-Trigger, kein Bang-Bang) berechnet: Q sinkt nur, wenn stableTick über z. B. 250 ms konstant über Budget liegt; steigt nur, wenn über 500 ms konstant darunter.
- Pro Frame werden die `round(Q · G_max)` Knoten mit höchster Priorität aktualisiert; alle anderen behalten ihren letzten Wert (temporales Caching). Da sich das Feld physikalisch glatt entwickelt (τ-gesteuert), ist ein 1–3 Frames alter Wert in ruhigen Regionen visuell irrelevant.
- Für nicht-ganzzahlige Budgets nutzt man stochastisches Scheduling: Ein Knoten mit gebrochenem Refresh-Intervall `K` wird mit Wahrscheinlichkeit `1/K` pro Frame aktualisiert (Hash aus Node-ID + Framezähler). Das erzeugt einen echten kontinuierlichen Erwartungswert statt Integer-Stufen – "stufenlos" im wörtlichen Sinn.

Damit gibt es **keine** Resolution-Sprünge, keine Re-Tessellation, keine Buffer-Realloc – nur eine kontinuierlich atmende Update-Rate auf fixer Geometrie.

### 3.3 Nahfeld-Korrektur (optional, für harte Singularitäten)
Gravitation ohne Softening (`1/max(d²,1)`) kann in Kamerannähe stärker variieren, als jedes Grid auflösen kann. Empfehlung: Quellen innerhalb eines kleinen Screen-Space-Radius um die Kamera separat exakt pro Pixel addieren (typ. 0–3 Quellen, kein N-Problem), während das Grid den glatten Fernfeld-Anteil aller übrigen Quellen liefert. Splitting-Verfahren, keine Grid-Verfeinerung nötig.

---

## 4. Fragment-Pass: Interpolation

Voller Screen-Quad, ein `textureSampleLevel` mit linearem Sampler auf `FieldTexture`. Das war's – die Bilinearität ist Hardware-Feature, kostet auf Intel-iGPUs praktisch nichts. Anschließend direkt die Nebra-Tonemap auf den gesampelten Ω-Wert anwenden. Kein Loop, keine Quellen-Referenz mehr im Fragment-Shader.

Die "Lücken"/"Lügen" zwischen den Stützstellen, die du beim harten 128er-Grid beobachtest, sind kein Interpolationsproblem, sondern ein **Sampling-Problem**: Bilinear kann nur so gut sein wie die Knotendichte. Löst 3.2/3.3, nicht Abschnitt 4.

---

## 5. Bewertung: Hexagonales Wabenmuster

Die 13%-Effizienzaussage ist korrekt – das hexagonale Gitter ist der informationstheoretisch optimale 2D-Quantisierer (geringster Rekonstruktionsfehler pro Sample bei gegebener Dichte). Rein rechnerisch: baryzentrische Interpolation aus drei Stützpunkten ist O(1) und für 2 Mio. Pixel kein Performance-Problem – die axiale Hex-Koordinatentransformation (Pixel → (q,r) → nächstes Dreieck → Baryzentrik) sind ~20–30 ALU-Ops, das ist gegenüber dem eigentlichen Flaschenhals (der N-Iteration, die wir bereits eliminiert haben) irrelevant.

**Das Problem ist nicht Performance, sondern Kompatibilität mit Abschnitt 3.2.** Ein Quadrat-Grid subdiviert sauber (1 Quad → 4 Quads), das ist der Standardfall für adaptive/LOD-Meshing seit Terrain-Engines der 2000er. Ein Hexagon subdiviert *nicht* sauber – an Auflösungsgrenzen entstehen zwangsläufig Pentagon-/Fünfeck-Defekte oder T-Junction-artige Nahtstellen, ein seit Jahrzehnten bekanntes, nicht-triviales Meshing-Problem. Genau das brauchst du aber für die stufenlose, ortsabhängige Dichte aus Abschnitt 3.2/5(AMR).

Fazit: Hex ist **kein Albtraum in WGSL** (die Fragment-Shader-Mathematik ist machbar und günstig), aber es ist ein **schlechtes Kosten-Nutzen-Verhältnis für dieses Projekt**: 13% konstanter Faktor gegen ein bereits vorhandenes 25–130× durch die Entkopplung – und es verkompliziert genau die Anforderung (stufenlose adaptive Dichte), die am wichtigsten ist. Empfehlung: **v1 mit Quad-Grid**, Hex als mögliche v2-Optimierung nur, falls die Dichte am Ende doch uniform bleibt (dann entfällt der Subdivisions-Konflikt).

---

## 6. Bewertung: Backend-gesteuertes AMR

Kamerabewegung/Zoom ist rein lokal am Client und darf nie auf einen WebSocket-Roundtrip warten – jede Backend-Vorgabe zur Verfeinerung würde dem Nutzer-Input hinterherhinken. Client-seitiges Refinement (Abschnitt 3.2) muss daher die Autorität für die Frame-zu-Frame-Entscheidung behalten.

Allerdings: Das Protokoll liefert mit `extent` und `tau` bereits ein implizites AMR-Signal – keine neuen Felder nötig. Sinnvoll wäre eine **seltene, grobe** Backend-Vorgabe (nicht pro Frame): bei größeren Szenenwechseln (viele neue, eng geclusterte Quellen) einen kompakten Hotspot-Hinweis mitschicken, der dem Prioritäts-Scheduler einen Kaltstart-Bias gibt, statt dass er sich erst über 1–2 Sekunden selbst einpendeln muss. Also: Backend als **langsamer Prior**, Client als **schneller Regler** – nicht umgekehrt.

---

## 7. Zusammenfassung

| Anforderung | Lösung |
|---|---|
| Schwache GPU, kein O(N·M) | 2-Pass-Pipeline: Compute auf G Knoten, Fragment nur Interpolation |
| Lückenloses Bild | Hardware-Bilinear via `rgba16float`-Texture, Nahfeld-Splitting bei Singularitäten |
| Stufenlose Skalierung | Fixe Topologie, kontinuierlicher Prioritäts-/Stochastic-Refresh-Scheduler statt Resolution-Stufen |
| Hexagonal-Wabe | Mathematisch/technisch machbar, aber konfligiert mit AMR – für v1 nicht empfohlen |
| Backend-AMR | `extent`/`tau` reichen als Signal; Backend nur als seltener Kaltstart-Hint, nie als Frame-Autorität |
