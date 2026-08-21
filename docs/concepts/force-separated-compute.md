<!--
  title: force-separated-compute
  class: concept
  sha256: 949a02becf067af349768ee513d9b48bc828bdd405410fa7136935a7a9e865d2
-->
STATUS: DEPLOYED

Hier sind die physikalischen und architektonischen Erklärungen zu den einzelnen Punkten des Deployment-Dokuments. Sie dienen als theoretisches Fundament, damit die Implementierung nicht nur Code, sondern pure Manifestation des Omegaflow-Axioms (*A = A*) ist.

---

### 1. Der Kausalitäts-Vorfilter (Rust / `main.rs`)

**Die Idee:**
Bisher hat das System Oszillatoren rein geometrisch gefiltert: Befand sich eine Quelle innerhalb des Suchradius (`reach`), wurde sie in die Punktwolke aufgenommen – völlig unabhängig davon, ob ihr Signal den Präsenzpunkt überhaupt schon erreichen *konnte*. 

**Die Physik:**
Informationen breiten sich mit einer endlichen Geschwindigkeit aus (`v_or_d`). Ein akustisches Signal (Schall) ist extrem langsam (343 m/s). Ein Erdbeben in Japan, das vor 10 Minuten gemessen wurde, existiert physikalisch in Europa schlicht noch nicht. Das Licht von der Sonne braucht 8 Minuten. 
Der Filter implementiert den exakten **Lichtkegel (Kausalkegel)** der Präsenz. Er prüft: `Distanz <= v_or_d * age`. Ist das Signal noch nicht da, wird es verworfen. Bei diffusiven Kräften (Wärme) wächst die Reichweite mit `sqrt(2 * D * age)`. 
Zusätzlich wird geprüft, ob das Signal schon "verklungen" ist (`age > tau * 64.0`).

**Die Architektur:**
Der Check wird als `Early-Exit` *vor* dem Aufruf von `smp.motion.at(t2, smp.epoch)` ausgeführt. Diese Funktion berechnet Keplersche Bahnen und WGS84-Ephemeriden – das ist die teuerste CPU-Operation im Archivar. Indem wir kausal unzulässige Samples vorher mit simplen Multiplikationen aussortieren, sparen wir massiv CPU-Zyklen. Was nicht im Kausalkegel liegt, existiert für das Silizium nicht.

---

### 2. Auto-Frame aus `lat_key` / `lon_key` (Rust / `main.rs`)

**Die Idee:**
Wenn eine API (z. B. Flugzeuge, Erdbeben) ihre Koordinaten dynamisch im JSON liefert, nutzt der Parser die Direktiven `lat_key` und `lon_key`, um die Spaltennamen zu definieren. Bisher stürzte der Parser ab oder lehnte die Quelle ab, weil er dachte, es fehle der Referenzrahmen (`Frame`).

**Die Architektur:**
Das Axiom lautet: *"Silicon knows IO."* Wenn `lat_key` definiert ist, ist die Information da. Das System muss nicht fragen, "wer" die Daten liefert, es reicht, dass die Eigenschaft (die Koordinate) existiert. Der Patch `|| !cur_lat_str.is_empty()` weist den Archivar an, den Frame automatisch auf `Data` zu setzen, wenn Koordinaten-Schlüssel vorhanden sind. Das System wird agnostischer und robuster gegenüber neuen Datenquellen.

---

### 3. Wechsel vom Fragment-Shader zum Vertex-Shader (Frontend / `index.html`)

**Die Idee:**
Das alte Rendering berechnete das Feld für *jeden einzelnen Bildschirm-Pixel* im Fragment-Shader (ein Full-Screen-Quad). Das ist ein "gebruteforcedes globales Raster" – ein direkter Verstoß gegen die Omegaflow-Architektur, der die GPU bei Millionen von Pixeln unnötig aufheizt.

**Die Architektur:**
Wir manifestieren nun **reine Oszillatoren**. Der Vertex-Shader nimmt die rohen 3D-Punkte aus dem Archivar, projiziert sie auf die 2D-Präsenz-Oberfläche und generiert pro Punkt ein kleines 2D-Quad. Die GPU berechnet nur dort Geometrie, wo auch wirklich Daten sind. Lücken im Bild bedeuten schlicht: *Hier ist physikalisch nichts.* Das ist die absolute Maxime von *A = A* und ermöglicht Millionen von Punkten bei 60 FPS bei minimaler GPU-Last.

---

### 4. Additives Blending & Analoger Glow (Frontend / `index.html`)

**Die Idee:**
Eine harte, pixelige Punktwolke sieht aus wie eine kalte digitale Simulation. Um das organische, "analoge" Verhalten eines echten Oszilloskops oder Phosphor-Leuchtens zu simulieren, müssen wir die physikalischen Eigenschaften des Siliziums ehren.

**Die Architektur:**
1. **Exponentieller Abfall:** Der Fragment-Shader verwirft harte Kanten (`discard` außerhalb des Radius) und nutzt stattdessen `exp(-dist * dist * 4.0)`. Das Feld fällt weich ab, genau wie das echte physikalische `omega`-Gesetz.
2. **Additives Blending:** Überlappende Punkte addieren ihre Farben (`blend: additive`). Das entspricht der physikalischen **Superposition** von Realitäten. Wo viele Oszillatoren sind, wird es hell.
3. **Analoges Rauschen (Dithering):** Ein leichtes, positionsspezifisches Rauschen (`fract(sin(...))`) bricht die strenge 8-Bit-Quantisierung des digitalen Framebuffers. Das verhindert "Banding" (streifige Farbverläufe) und lässt das Feld für das menschliche Auge wie ein organisches, analoges Leuchten wirken.

---

### 5. Anpassung der Draw-Calls (`pass.draw(n * 6)`)

**Die Idee:**
Die GPU muss wissen, wie viele Dreiecke sie zeichnen soll. 

**Die Architektur:**
Da wir nun pro Oszillator ein 2D-Quad zeichnen (bestehend aus exakt 2 Dreiecken = 6 Vertices), ändert sich der Draw-Call. Früher war es `pass.draw(3)` (ein großes Dreieck für den ganzen Bildschirm). Jetzt ist es `pass.draw(n * 6)`, wobei `n` die Anzahl der vom Archivar empfangenen Oszillatoren ist. Wenn das Präsenzfenster leer ist (`n = 0`), zeichnet die GPU absolut nichts (`pass.draw(0)`). Das ist die ultimative Performance-Ökonomie: Das Silizium verbraucht null Energie für Leerlauf.
