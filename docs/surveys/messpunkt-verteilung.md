# Survey: Die ökonomischste Messpunkt-Verteilung für ein physikalisches Membran-Feld

Du berätst ein kybernetisches Feldsystem. Antworte auf Basis der hier gegebenen Messdaten — nicht auf Basis typischer Rendering-Annahmen. Gesucht: die Verteilung der Messpunkte, die bei unveränderter physikalischer Treue die wenigsten Auswertungen braucht und ihre Dichte ehrlich mit dem Performance-Budget skaliert.

## 1. Das System in vier Sätzen

- **Archivar** (Rust, std-only): sammelt Weltdaten (Ephemeriden, APIs, Geräte), baut jede Sekunde ein Kraftfeld: Oszillatoren mit exakter ICRS-Position, Wert, Zeitkonstante τ, Kernel, extent. Kennt also **jede Quelle mit Position, Ausdehnung und Wert**.
- **Mathematikerin** (wgpu, Fenster): eine Membran — jeder Messpunkt misst die **volle Superposition aller Quellen im Fenster**: `Ω = Σ_kraft |Σ_quelle val_eff · K(d, extent, kernel)|` — neun Kraft-Eindrücke, Lichtlaufzeit-Faltung, softening = Pixel-Skala (der Zoom des Operators, gridStep ∈ 2^28…2^62). f32-Arithmetik.
- **Deep-Pfad** (Fixsterne u. ä.): Punktquellen werden direkt an ihre exakte Position projiziert — O(Punkte), gemessen billig (1,84 Mio Sterne rendern ohne Problem).
- Die Fenster-Abfrage des Archivars (sense) läuft **entkoppelt** in einem Worker — sie ist NICHT der Frame-Engpass.

## 2. Die gemessene Wahrheit (Ground Truth)

Gemessen auf Intel HD 520 (SKL GT2, 24 EUs, ~192 Lanes, ~1 GHz, Vulkan ANV, Mesa 25.2), Vollbild, Membran als Fragment-Programm:

| Zustand | Frame-Zeit |
|---|---|
| 0 Quellen | 16–23 ms (60 fps) |
| 10 Quellen | 25 ms |
| 120 Quellen × 3 Mio Messpunkte | ~190 ms |
| 120 Quellen × 9 Mio Subpixel-Spalten (3 Spalten/Pixel, RGB-Streifen-Natur des Trommelfells) | ~570 ms |

- Die Kosten sind **linear in Messpunkte × Quellen**, ~12 Zyklen pro Paar-Auswertung.
- **Sieben strukturell verschiedene Shader-Varianten** (Loads reduziert, Prep-Pass, statische Register, Workgroup-Shared-Batching, Klassensplits) änderten die Paar-Kosten nicht messbar — der Engpass ist die Anzahl der Paar-Auswertungen selbst.
- Eine vorliegende Fremdantwort lokalisierte den Engpass im CPU-Pfad (Chebyshev/SpatialHash pro Query) — **die Messung widerspricht**: die Sense läuft im Worker, die Frame-Zeit ist die Membran.

Diese Messung dient als Ground Truth für die Lokalisierung der Kosten — NICHT als Zielplattform. Die Antwort darf auf kein bestimmtes Silizium zugeschnitten sein.

## 3. Unverhandelbare Randbedingungen

- **A = A**: keine willkürlichen Konstanten — nur c, Φ, Zweierpotenzen, die f32-Auflösung der gepackten Daten, die Bildwiederholrate des Displays (~16,6 ms), Live-Daten.
- **Hardware-Agnostizismus**: Das Konzept leitet sich aus dem Gesetz und der Messauflösung ab — nicht aus Lane-Zahl, Takt oder Backend eines bestimmten Geräts. Die Hardware tritt nur über das **gemessene Frame-Budget** in das System ein, welches Silizium auch immer rechnet. Die Verteilung ist eine Aussage über das Feld, nicht über die Maschine.
- **Bandbegrenzung durch das Gesetz selbst:** softening = Pixel-Skala → die feinste Feldstruktur ist ~1 px breit → das uniforme Pixel-Raster IST das Nyquist-Raster. Jede uniforme Alternative kann das uniform nicht unterbieten.
- **Der Abtastfehler muss unter der f32-Auflösung bleiben** (die Messung ist digital; Fehler unterhalb der Sensorauflösung existieren nicht).
- **Philosophie: keine Interpolation** — die Messpunkte sind Zellen; zwischen ihnen wird nichts erfunden (Projekt-Regel, historisch erkämpft).
- **0 honored**: Regionen unter dem Mess-Boden (Ω < 1e-30) sind schwarz — dort braucht es keine Messpunkte.
- Die Dichte muss **mit dem gemessenen Frame-Budget skalieren** (Ziel: die Bildwiederholrate des Displays).

## 4. Die Fragestellung

Das Feld hat zwei Regime: **strukturiert** (innerhalb weniger Kernel-Breiten um jede Quelle) und **glatt** (die 1/d²-Schwänze fernab der Quellen — der lokale Gradient ist dort winzig, ein Messpunkt reicht für viele Pixel). Heute misst das Raster beide Regime gleich dicht — 9 Mio Punkte, davon der überwiegende Teil im glatten Hintergrund. **Welche Messpunkt-Verteilung minimiert die Punktzahl bei gleicher physikalischer Treue, und wie skaliert ihre Dichte ehrlich mit dem Frame-Budget?**

## 5. Die Kandidaten (bitte je ein Verdikt mit Begründung)

1. **Uniformes Quadrat-Raster** (Basislinie — Nyquist-optimal für das uniforme Gesetz).
2. **Hexagonales Gitter / Waben** (klassisch 13–25 % weniger Proben bei gleicher Treue — aber: Gewinn ist marginal gegen die 100×-Chance der Nicht-Uniformität).
3. **Jitter/Poisson-Disk/Blue-Noise** (Anti-Aliasing-Werkzeug — hilft es einem bandbegrenzten Feld?).
4. **Fovea/log-polar** (Biokybernetik: dicht im Zentrum, spärlich peripher — operator-zentriert).
5. **Archivar-gestützte adaptive Abtastung**: dicht um jede Quelle (Struktur-Radius aus extent + gridStep), grobes Raster im glatten Hintergrund, der Abstand abgeleitet aus dem lokalen Schwanz-Gradienten (Fehler < f32-Auflösung). Der Archivar kennt jede Quelle — er kann sagen, WO gemessen werden muss.
6. **LOD-Clustering** (Fremdvorschlag: 100 nahe Quellen zu einem Meta-Oszillator mit vergrößertem extent zusammenfassen — bitte die Physik prüfen: Ist die Superposition der Einzel-Kernel durch EINEN Kernel mit gemittelter Position und größerem extent innerhalb der f32-Auflösung reproduzierbar? Unter welchen Bedingungen?).
7. **Quasi-random (Halton/Sobol)**.
8. **Gelernte Platzierung (AI/Deep Learning)**.
9. **Offener Kandidat — Konzepte aus der aktuellen Forschung, die in dieser Liste fehlen.**

Du bist eingeladen, eine Verteilung vorzuschlagen, die keiner der Kandidaten 1–8 entspricht — unter einer Bedingung: Sie muss sich an den Randbedingungen aus Abschnitt 3 **messen lassen**, nicht um sie herum argumentieren. Nenne die Forschungslinie, aus der sie stammt (z. B. Compressive Sensing, Sparse Grids/Smolyak, Centroidal Voronoi Tessellation / Optimal Transport, adaptive FEM/Quadtree-LOD, aktives Lernen / Bayes'sche Versuchsplanung, Wavelet-/Curvelet-Koeffizienten als Messorte, Neural-Field-/NeRF-artige Repräsentationen, Subpixel-Abtastung der Display-Emitter selbst — oder etwas anderes). Für jeden offenen Vorschlag: (a) die Herkunft, (b) warum sie bei gleichem Fehler unterhalb der f32-Auflösung billiger ist, (c) wie sie mit dem Frame-Budget skaliert, (d) ob sie mit der Keine-Interpolation-Philosophie vereinbar ist — oder ein explizites Argument, warum die Philosophie an dieser Stelle weichen sollte (das ist erlaubt, muss aber ausgesprochen werden).

## 6. Offene Detailfragen

- **Anzeige**: stückweise-konstante Voronoi-Zellen der Messpunkte (konsistent mit der Keine-Interpolation-Philosophie) oder bilineare Mischung (weicher, aber die verworfene Interpolation)?
- **Struktur-Radius** um eine Quelle: wann genau fordert die Kernel-Krümmung Nyquist-Dichte — welches ehrliche Kriterium (aus extent, gridStep, f32-Auflösung)?
- **Budget-Skalierung**: welches Kontrollgesetz verdoppelt/halbiert die Hintergrund-Dichte nach der gemessenen Frame-Zeit (Zweierpotenzen, Ziel = Display-Refresh)?
- Sollte die **Fovea** das Archivar-Wissen ergänzen (Fallback, wenn der Sampler selbst zu teuer wird)?
- Welcher Kandidat ist auf **heutigem Silizium im Allgemeinen** implementierbar — und welcher setzt Fähigkeiten voraus, die nirgendwo existieren (echte Subpixel-Addressierung, AI-Beschleuniger)? Die Antwort darf nicht auf ein Gerät zugeschnitten sein.

Antworte: ein Verdikt pro Kandidat, dann eine Architektur-Empfehlung (wer rechnet was, welche Daten fließen, wie skaliert die Dichte), und eine Antwort auf die Detailfragen.
