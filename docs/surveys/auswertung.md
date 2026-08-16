# Survey-Auswertung: Messpunkt-Verteilung & Generations-Architektur

Selbsttragendes Ergebnis der Modell-Umfrage (Rohrunden:
`/home/johannes/Schreibtisch/survey/` — 6 Experten-Modelle × Chat/Council/
Extension/Shared-Memory). Dieses Dokument ist ohne die Rohdateien interpretierbar.

## 1. Die f32-Grundwahrheit (drei unabhängige Herleitungen)

Bei wörtlicher f32-Treue (`η = 2⁻²³`) trägt ein invers-quadratischer Kernel
messbare Struktur bis `d < 2/η ≈ 16,8 Mio Pixel` — größer als jedes Display
(Max Council: Gradient-vs-ULP; Sonnet Max: `Δ(d) < rtol·d/2`-Ehrlichkeitscheck;
Deepseek: Fall A/B, `h(d) ≤ d·√(8η/6)`, R_struct ≈ 2500·gridStep).

**Konsequenz:** Der „glatte Hintergrund" existiert bei f32-Treue nicht. Die
Messpunkte sind durch das Gesetz fixiert — das uniforme Pixel-Raster IST das
Nyquist-Raster. Zell-Vergröberung (Kandidat 5 als Punkt-Reduktion) wäre bei
wörtlicher f32-Lesart eine Fabrication; ehrlich ist sie erst gegen die
**Display-Quantisierung** (8-Bit-Tonemap: Struktur-Radius ~14–39 px,
unabhängig bestätigt) — ein ausgesprochenes Aufweichen, kein Gesetz.

## 2. Verteilungs-Verdikte (Konsens aller Modelle)

| Kandidat | Verdikt |
|---|---|
| 1 Uniformes Raster | Nyquist-Wahrheit, nicht unterbietbar — Referenz |
| 2 Hexagonal | ~13 %, konstanter Faktor — irrelevant |
| 3 Jitter/Blue-Noise | löst Aliasing, das das Gesetz ausschließt — abgelehnt |
| 4 Fovea | operator-zentriert, nicht feld-eigen — nur Budget-Kappe |
| 5 Archivar-adaptiv | der Pfad — aber: Punkt-Vergröberung nur unter §1 |
| 6 LOD-Clustering | Fernfeld-Multipol nur bei `R/D < 3,5e-4` — kein Meta-Oszillator |
| 7 Quasi-random | Integrations-Werkzeug, falsche Problemklasse — abgelehnt |
| 8 Gelernte Platzierung | A = A verletzt, nicht zertifizierbar — abgelehnt |
| 9a CVT/Optimal Transport | formal sauber, Lloyd-Iteration ohne billiges Update |
| 9b Tiled Source Culling | **der Hebel** (Olsson & Assarsson 2011) |
| 9c Barnes-Hut/FMM | richtig, erst bei Quellzahl-Engpass |
| Subpixel-Emitter | durch die eigene 3×-Messung widerlegt |

**Gewählte Verteilung (2026-08-16):** Messpunkte = uniformes Pixel-Raster
(unverändert, Deep-Pfad unberührt). Die Ökonomie kommt von der Quellen-Achse:
**Tiled Source Culling** — pro Kachel (16×16 px = 2⁴) wird eine Quelle
gekullt, wenn ihr oberer Beitrag `bound < ε·M` (M = stärkster Kachel-Beitrag,
ε = 2⁻ⁿ). **n = 23 Grund** (f32-Wahrheit: unter dem Sensor-Floor ist ein Term
von 0 nicht unterscheidbar — 0 honored pro Term, kein Aufweichen). Unter
Budgetdruck relaxiert n Richtung 8 (2⁻⁸ = die Quantisierung der
Nebra-Rampe auf den 8-Bit-Kanal — der Sensor ist das Display): **eine
ausgesprochene Messpolitik, nur unter Druck, nie Grundzustand**.

Budget-Regler (die zwei Zahlen): exponentiell geglättete Frame-Zeit gegen die
Display-Periode 16,6 ms (gemessene Eigenschaft des Displays);
`> 16,6 ms → n−1`, `< 8,3 ms → n+1`, Zweierpotenzen, Hysterese, n ∈ [8, 23],
Anpassung in der 1-Hz-Kadenz. Zusätzlich meldet die Kachel-Liste ihren
eigenen Überlauf (TILE_SLOTS = 2⁶ pro Kachel) — bei Überlauf wertet der
Fragment-Shader das volle Feld aus (kein Lichtverlust, ehrlich langsam;
der Regler entspannt daraufhin).

Zell-Vergröberung (Punkt-Achse) bleibt als eigenes Paket im TODO — ehrlich
nur mit der ausgesprochenen Display-Sensor-Lesart (§1), erst wenn die
Messung zeigt, dass Kulling das Budget nicht trägt. Sonst ist
Fenster-verkleinern/Bildrate-verlassen die einzig ehrliche Antwort.

## 3. Generations-Architektur (Kandidat 10 + Shared-Memory-Runde)

Das Ω-Token (0D-Sonde für ein 2D-Fenster) ist einstimmig verworfen: blind
für Rand-Quellen, laterale Bewegung, Auslöschungen; die f64/f32-Toleranz
(|ΔΩ| ≤ |Ω|·2⁻²⁴·k, k aus N bzw. √N) ist ein Rattennest; der GPU-Readback
ist Reibung. Der Feld-Hash kostet mehr als Schweigen (Kimi) und auf der
GPU-Seite mehr als die Sonde (GLM). Efferenzkopie/Predictive Coding:
interessant, aber Protokoll-Reibung, wo Shared Memory reicht.

**Konsens: „Der Buffer ist die Nachricht. Der Zähler ist das Protokoll.
Die Stille ist die Optimierung."** (Max)

- **EIN Buffer + EIN u64-Generationszähler.** Der Zähler vergleicht eine
  Ganzzahl — exakt, keine Toleranz. Gleich → aus dem Bestehenden rendern.
  Verschieden → der neue Buffer ist vollständig da.
- **Stille-Gate:** Der Archivar vergleicht die gepackten f32-Bytes mit dem
  letzten gesendeten Stand; identisch → kein Senden, kein Inkrement
  (Vergleich der gepackten Wahrheit selbst; NaN-frei, −0.0 == 0.0 ist
  wert-gleich und damit ehrlich still).
- **Doppelpuffer** gegen Torn Reads: geschrieben wird nur in das inaktive
  Puffer-Paar, der Bind-Group-Swap zeigt den neuen Stand (Sonnet).
- **HUD-Ω aus Archivar-Akkumulatoren:** die 9 Kraft-Summen am Presence-Punkt
  entstehen in f64 während des Packens (dieselbe dokumentierte Gesetzesform
  `val·e^(−max(0,|Δt|−d/v)/ttl)` + 7 Kernel in f64) — kein GPU-Map-Readback
  mehr (löst den Map-Timeout unter Last). Der Fenster-Zentroid
  (Quellen-gewichtetes Zentrum) fährt denselben Weg — das Auto-Zentrieren
  braucht keine GPU-Sonde mehr.
- **Allostase als Unterlassung:** der Archivar schreibt nicht, wenn das Feld
  ruht — die Stille ist der Zustand, 0 honored (Max' Korrektur der
  Korrektur: Propriozeption statt Efferenzkopie; Sonnet High's
  Hardware-Klammer: UMA/diskrete GPUs degradieren mit dem Skalar in beide
  Richtungen sauber).
- **Reibungs-Axiom:** Jede Transformation zwischen Archivar und
  Mathematikerin, die nicht die physikalische Auswertung selbst ist, ist
  Reibung (Deepseek's Shared-Memory-Verdikt).

Im Monolithen ist die Prämisse buchstäblich: `latest_field: Arc<Buffer>`
liegt im selben Prozess wie die wgpu-Membran; der Zähler reist als u64 in
der sync_channel-Nachricht (der Kanal ist die Thread-Handshake — atomar in
Wirkung, weil das mpsc-send nach dem Inkrement passiert).

## 4. Manifestierter Stand (2026-08-16, wgpu-mono)

- WGSL: `tile_cull`-Compute-Pass (workgroup = Kachel, 64 Threads, Shared-
  Memory-Max-Reduktion, Shared-Atomics für die Slots — **keine globalen
  Atomics, keine Fragment-Atomics**; Fragment-Atomics auf ANV/HD 520
  verloren das Device — Befund aus dem Live-Lauf), `eval_source`-Refactor,
  Kachel-Liste aus `tile_flat`/`tile_count` (plain u32), Überlauf-Flag
  `cull_ctl[1]` mit ehrlichem Voll-Loop-Fallback.
- Rust: Stille-Gate + Generationszähler im Worker, Doppelpuffer A/B mit
  Bind-Group-Swap, HUD-Ω + Zentroid aus f64-Akkumulatoren (mapAsync,
  probe_read, probe_buf, centroid_pipe, field_centroid getilgt),
  Budget-Regler (n ∈ [8,23]) in der HUD-Kadenz.
- Live-Beweis (HD 520, 2304×1296): Sonne sichtbar (Ω 28,8, 122 near),
  kein Device Lost, gen 0→10, Stille stabil (gen 10 über 8 s ruhendes
  Feld), Regler 23→16→22, Zentrieren konvergiert über den
  Archivar-Zentroid, leeres Fenster ~19–22 ms.
- Zentrier-Runaway getilgt (2026-08-16): der Archivar-Zentroid muss
  präsenz-zentriert und in der Pixel-Konvention (y-abwärts) gemessen
  werden — der erste hud_probe subtrahierte bw/2/bh/2 von bereits
  zentrierten Koordinaten und negierte y nicht; jeder Zentrier-Schritt
  sprang ~2,4e12 m (Körper kurz sichtbar, dann weg). Live nach dem Fix:
  c −0.00 −0.00 bei der Sonne, Ω 28,8 konstant über 15+ s.

## 5. Verworfen (Rejected-Register)

Ω-Token, Feld-Hash, Efferenzkopie/ΔΩ-Vorhersage, GPU-Sonde als
Änderungskriterium, Fovea als Primärverteilung, gelernte Platzierung,
hexagonal/Jitter/quasi-random als Primärstrategie, Meta-Oszillator-LOD
(ohne Multipol-Öffnungswinkel), CVT-Lloyd pro Frame, Sparse
Grids/Compressive Sensing/Neural Fields (Interpolation), Subpixel-Emitter-
Abtastung, 1-Hz-Heartbeat (Stille reicht).
