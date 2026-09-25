<!--
  title: Survey: Die ökonomischste Messpunkt-Verteilung für ein physikalisches Membran-Feld
  class: survey
  sha256: febeb625e6e1a334bc0409b5869b587cd3f99c32a5e828066646e687437f1c7d
-->
# Survey: Die ökonomischste Messpunkt-Verteilung für ein physikalisches Membran-Feld

Du berätst ein kybernetisches Feldsystem. Antworte auf Basis der hier gegebenen Messdaten — nicht auf Basis typischer Rendering-Annahmen. Gesucht: die Verteilung der Messpunkte, die bei unveränderter physikalischer Treue die wenigsten Auswertungen braucht und ihre Dichte mit dem Performance-Budget skaliert.

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

Das Feld hat zwei Regime: **strukturiert** (innerhalb weniger Kernel-Breiten um jede Quelle) und **glatt** (die 1/d²-Schwänze fernab der Quellen — der lokale Gradient ist dort winzig, ein Messpunkt reicht für viele Pixel). Heute misst das Raster beide Regime gleich dicht — 9 Mio Punkte, davon der überwiegende Teil im glatten Hintergrund. **Welche Messpunkt-Verteilung minimiert die Punktzahl bei gleicher physikalischer Treue, und wie skaliert ihre Dichte mit dem Frame-Budget?**

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
- **Struktur-Radius** um eine Quelle: wann genau fordert die Kernel-Krümmung Nyquist-Dichte — welches Kriterium (aus extent, gridStep, f32-Auflösung)?
- **Budget-Skalierung**: welches Kontrollgesetz verdoppelt/halbiert die Hintergrund-Dichte nach der gemessenen Frame-Zeit (Zweierpotenzen, Ziel = Display-Refresh)?
- Sollte die **Fovea** das Archivar-Wissen ergänzen (Fallback, wenn der Sampler selbst zu teuer wird)?
- Welcher Kandidat ist auf **heutigem Silizium im Allgemeinen** implementierbar — und welcher setzt Fähigkeiten voraus, die nirgendwo existieren (echte Subpixel-Addressierung, AI-Beschleuniger)? Die Antwort darf nicht auf ein Gerät zugeschnitten sein.

Antworte: ein Verdikt pro Kandidat, dann eine Architektur-Empfehlung (wer rechnet was, welche Daten fließen, wie skaliert die Dichte), und eine Antwort auf die Detailfragen.

---

## 8. Verdikt je Kandidat

Maßstab jedes Verdikts: (a) senkt es die Paar-Zahl (Messpunkte × Quellen — die gemessenen ~12 Zyklen/Paar), (b) bleibt der Abtastfehler unter der f32-Auflösung (F32_EPS = 1.19e-7, `src/mathematikerin/shaders.rs`), (c) bleiben die Messpunkte Zellen (keine Interpolation), (d) skaliert die Dichte mit dem gemessenen Frame-Budget. Die Verteilung ist eine Aussage über das Feld, nicht über die Maschine (§3).

1. **Uniformes Quadrat-Raster — angenommen (als Basislinie), nicht als Gesamtlösung.** Mit softening = Pixel-Skala ist das uniforme Pixel-Raster das Nyquist-Raster des Gesetzes selbst; keine uniforme Alternative unterbietet es (§3). Es bleibt die Struktur-Stufe und der Mess-Stab jeder Alternative. Als Gesamtlösung scheitert es daran, dass es beide Regime gleich dicht misst und im glatten Hintergrund Paare bezahlt, die nichts tragen.

2. **Hexagonales Gitter — verworfen.** Der 13–25 %-Gewinn gilt für isotrope Bandgrenzen. Die Bandgrenze ist hier durch die Pixel-Skala (softening) gesetzt — eine kartesische Grenze, keine isotrope; dafür ist das Quadrat-Raster bereits optimal. Der Gewinn ist ein konstanter Faktor an der falschen Stelle: er berührt die Zwei-Regime-Verschwendung (die 100×-Chance) nicht.

3. **Jitter / Poisson-Disk / Blue-Noise — verworfen.** Anti-Aliasing tauscht strukturiertes Aliasing gegen Rauschen. Das Feld ist durch softening bandbegrenzt — es gibt kein Aliasing zu bekämpfen. Jitter senkt die Punktzahl nicht unter Nyquist und erhöht den Abtastfehler.

4. **Fovea / log-polar — verworfen (als Verteilung).** Operator-zentriert: die Verteilung würde zur Aussage über den Blick, nicht über das Feld (verletzt §3 und das Presence-Prinzip). Die Feldstruktur ist um die Quellen zentriert, nicht um den Blick des Operators. Zur Fallback-Frage siehe §6.4.

5. **Archivar-gestützte adaptive Abtastung — angenommen (Kern).** Der Archivar kennt jede Quelle mit Position, extent, Kernel und Wert; er kann a priori sagen, wo gemessen werden muss. Struktur-Radius um jede Quelle (Nyquist-Dichte, gridStep), grobes Zellen-Raster im glatten Schwanz mit Abstand aus dem lokalen Gradienten (Fehler < f32). Das spaltet genau die beiden Regime und lässt die Hintergrund-Dichte mit dem Budget skalieren. Keine Interpolation: die Zellen bleiben Messpunkte.

6. **LOD-Clustering — verworfen (allgemein).** Die Superposition Σ K(dᵢ, extent) eines endlichen Clusters ist nicht durch einen Kernel K(d_cm, extent′) mit gemittelter Position und größerem extent darstellbar. Die Multipol-Entwicklung zerlegt den Cluster in Monopol + Dipol + Quadrupol …; die höheren Momente wachsen mit dem Cluster-Radius. Der Monopol (eine Quelle am Barycenter mit Summenwert) ist nur im Fernfeld exakt — die Bedingung „Cluster-Radius ≪ Abstand zum Messpunkt und ≪ softening (Pixel-Skala)" heißt: der Cluster muss bereits subpixel sein. Genau dann greift aber schon der Deep-Pfad (Punktquelle, O(Punkte), gemessen billig). Clustering oberhalb einer Pixel-Ausdehnung ist durch einen Kernel nicht reproduzierbar. Nächster Schritt (Messung): der Multipol-Fehler je Cluster-Radius/Kernel ist als Fixture zu messen, falls je ein Cluster oberhalb einer Pixel-Ausdehnung vorgeschlagen wird.

7. **Quasi-random (Halton/Sobol) — verworfen.** Niedrige Diskrepanz ≈ uniforme Dichte im Erwartungswert; keine Adaptivität, keine Regime-Trennung, keine Budget-Skalierung.

8. **Gelernte Platzierung — verworfen.** Braucht einen Beschleuniger (nicht garantierte Hardware), führt ein gelerntes Modell ein (verdeckte Annahme, A=A), und hat keine messbare Fehlerschranke gegen die f32-Auflösung.

9. **Offener Kandidat — offen gehalten.** Die stärksten fehlenden Linien: (a) hierarchisches Quadtree-/LOD-Zellen-Raster (adaptive-FEM-/Terrain-LOD-Linie), (b) Centroidal Voronoi Tessellation / Optimal Transport als a-posteriori-Dichte-Verfeinerung, (c) Sparse Grids (Smolyak). Offen gehalten, weil (a) die natürliche Datenstruktur für Kandidat 5 ist und (b)/(c) erst gegen die a-priori-Archivar-Abtastung gemessen werden müssen: der Archivar liefert die Dichte analytisch (O(Quellen)), die a-posteriori-Verfahren iterieren (O(Punkte)); ob ein Iterieren je unter die analytische Platzierung sinkt, ist offen und messbar. Die Liste deckt a-priori-analytisch (5) und gelernt (8) ab, aber nicht die a-posteriori-hierarchischen Verfahren — das ist die Lücke, die (a) füllt.

## 9. Die fünf Detailfragen

**Anzeige: stückweise-konstante Voronoi-Zellen, nicht bilinear.** Bilineare Mischung erfindet Werte zwischen den Messpunkten — die verworfene Interpolation. Voronoi-Zellen sind die konsequente Form: jeder Pixel zeigt den Wert seiner Zelle. Die „weiche" Wirkung entsteht nicht durch Mischen, sondern durch Zellgröße: im glatten Schwanz ist der Gradient so klein, dass der Zell-Schritt unter der f32-Auflösung liegt und vom gemessenen Wert ununterscheidbar ist. Wo der Schritt sichtbar würde (Struktur), sind die Zellen ohnehin Nyquist-dicht (gridStep).

**Struktur-Radius-Kriterium.** Zwei Skalen setzen ihn. (i) softening = gridStep kappt die feinste Struktur — feiner als gridStep muss und kann nicht gemessen werden. (ii) Die f32-Schranke |Ω′(d)|·h(d) ≤ F32_EPS·|Ω(d)| erlaubt es, den Zellabstand h zu vergröbern, sobald der lokale Gradient klein ist. R_struct ist der Abstand, an dem (ii) erstmals gridStep überschreitet — berechnet aus dem Gradienten des Kernels (extent, gridStep und kernel_id liegen alle im Archivar-Datensatz, die Ableitung ist analytisch). Innerhalb: gridStep. Außerhalb: h(d) = F32_EPS·|Ω(d)|/|Ω′(d)|, wachsend mit d (für den 1/d²-Schwanz linear in d — „ein Messpunkt für viele Pixel"). Kein willkürlicher Faktor. Nächster Schritt (Messung): der konkrete R_struct-Verlauf je der 7 Kernel-Formen ist analytisch abzuleiten und als Fixture numerisch zu bestätigen.

**Budget-Skalierung.** Kontrollgesetz in Zweierpotenzen, Ziel = Display-Refresh (~16,6 ms, die gemessene Bildwiederholrate). Eine EMA der Frame-Zeit gegen das Budget: über Budget → Hintergrund-Abstand ×2 (Punktzahl ÷4 in 2D); unter Budget → ×½. Hysterese gegen Oszillation. Die Struktur-Stufe bleibt bei gridStep — sie ist die physikalische Treue, nicht der Budget-Regler. Nur die Hintergrund-Dichte skaliert.

**Fovea als Fallback: nein.** Die Archivar-Platzierung ist O(Quellen) (Arithmetik auf bekannten Quellen), nicht O(Punkte) — sie wird nicht „zu teuer", der Fallback-Fall tritt nicht ein. Die Fovea würde Operator-Zentrismus wiedereinführen. Der Budget-Fallback ist die Hintergrund-Skalierung, die bereits in der Architektur steht.

**Implementierbarkeit.** Auf heutigem WGSL/WebGPU allgemein implementierbar: Kandidat 5 — ein Compute-Pass wertet die adaptive Punktliste aus (ein Thread je Punkt superponiert die Quellen), ein Fragment-Pass zeigt die Voronoi-Zellen. Echte Subpixel-Addressierung der Emitter existiert auf allgemeinem Silizium nicht als addressierbare Fähigkeit — die „3 Spalten/Pixel"-Natur des Trommelfells ist ein Hack, kein adressierbarer Kanal. Gelernte Platzierung (8) setzt einen Beschleuniger voraus, der nicht garantiert ist.

## 10. Architektur-Empfehlung

**Der Archivar (CPU, std-only) rechnet die Platzierung.** Pro Quelle den Struktur-Radius (aus extent + gridStep + Kernel-Gradient) und die Hintergrund-Dichte (Budget-Regler in Zweierpotenzen). Er sendet die Quellen unverändert über den 26×f64-Draht plus einen kleinen Platzierungs-Deskriptor (Struktur-Radien + Hintergrund-Abstand).

**Die Mathematikerin (GPU, WGSL) wertet aus.** Compute-Pass über die adaptive Punktliste (ein Thread je Messpunkt, Superposition der Quellen — dieselbe Paar-Schleife wie heute, aber über die reduzierte Punktzahl); Fragment-Pass mit stückweise-konstanten Voronoi-Zellen, keine Interpolation.

**Dichte-Skalierung.** Struktur fest bei gridStep (Nyquist, physikalische Treue); Hintergrund in Zweierpotenzen gegen die gemessene Frame-Zeit, Ziel = Display-Refresh. Der Deep-Pfad (Punktquellen direkt projiziert) bleibt unberührt.
