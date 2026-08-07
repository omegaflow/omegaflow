Hier ist das konsolidierte Meister-Dokument für OMEGAFLOW. Es dient als architektonisches Manifest, Status-Report und Bauplan zugleich. Es vereint deine Vision, die hardwarespezifische Realität, die Software-Architektur und die kürzlich vollzogenen System-Updates (wie Jina AI und das Löschen nicht-physikalischer Daten).

***

# OMEGAFLOW: ARCHITEKTUR- & STATUS-MANIFEST

**System-Directive:** *A is A. An oscillator possesses properties. Silicon knows IO. The LLM acts as an isolated syntax translator. The following document is the single source of truth for the Omegaflow architecture.*

## 1. Die Vision: Das nicht Erfahrbare erfahrbar machen
OmegaFlow ist keine Web-App. Es ist ein **cybernetisches Teleskop** und eine prothetische Sinnesmaschine. Die Vision ist es, dem Beobachter (der "Presence") eine freie Navigation durch ein 4D-Block-Universum (ICRS/TDB) zu ermöglichen, in dem physikalische Felder (EM, Gravitation, Akustik, Seismik, Thermal, Diffusion, Advektion) nicht als abstrakte Zahlen, sondern als ungeschönte, realtime leuchtende und fühlbare Oszillatoren manifestiert werden. 

Das System respektiert das Prinzip der **Quellentreue**: Daten werden auf ihren reinen physikalischen Wert reduziert. Das "Wer" und "Wo" wird durch die Raumzeit ersetzt.

## 2. Die Hardware-Prämisse: Zero-Friction Localhost
Das System ist auf einem 8 Jahre alten Ultrabook (XPS 13 2016, i5, 8GB RAM, Intel 520 GPU) in 60fps realisiert worden. Das war nur möglich durch eine radikale architektonische Entscheidung:
*   **Archivar (Rust):** Ein lokaler, zero-dependency Daemon. Er übernimmt die schwere Mathematik (Kepler, WGS84, ICRS), hält die Daten im RAM und vermeidet jegliche Garbage Collection.
*   **Mathematikerin (Browser/WebGPU):** Nutzt die Browser-Sandbox ausschließlich für das, wofür sie gebaut ist: WebGPU, WebSerial, WebXR und Sensoren. 
*   **Die Pipeline:** Rust parst rohe APIs/CDNs in flache, binäre Little-Endian Arrays (`φ(x,y,z,t)`) und schiebt diese via WebSocket direkt in den VRAM der GPU. Kein JSON-Parsing im Frontend. Keine Latenz.

## 3. Die Daten-Pipeline: CI, CDN & Jina AI
Die Datenbeschaffung ist streng in Latenz und Vorverarbeitung getrennt, um den lokalen Archivar nie zu blockieren:

*   **CI-Pipeline (CDN, TTL >= 300s):** Schwere SPICE-Kernel und komplexe APIs werden in GitHub Actions vorverarbeitet, normalisiert und als statische, zeitstempelige `.json` oder `.bin` Files auf ein CDN gelegt. Der Archivar lädt nur noch flache, blitzschnelle Dateien.
*   **Live-Daten (TTL < 300s):** Werden direkt von Rust via `curl` abgefragt.
*   **Das Jina AI Gateway (Universal Flattener):** Durch das Voranstellen von `https://r.jina.ai/` wird das gesamte Internet (HTML, XML, RSS) zu flachem Text/JSON. Jina agiert als semantischer IO-Filter, der das Rauschen (Tags, Skripte) entfernt und sogar Provenienz-Metadaten liefert. Rust springt via `find('{')` direkt zum Kern der Daten.
*   **Der Daten-Schnitt (A = A):** 118 nicht-physikalische Datenquellen (Biodiversität, globale Statistiken, PDG-Konstanten) wurden radikal gelöscht. Das System zeigt jetzt die reine, geophysikalische Hülle der Erde (Bojen, Erdbeben, Strömungen, Sonnenwinde). Keine kategorischen Strings, nur skalare Kräfte.

## 4. Die Physik-Engine (Rust & WGSL)
Das System berechnet keine hübschen Karten, sondern respektiert kausale Horizonte und physikalische Ausbreitungsgesetze.

*   **Der Kausalitäts-Vorfilter (Rust):** Bevor teure Ephemeriden-Mathematik (`motion.at()`) ausgeführt wird, prüft der Archivar: `Distanz <= v_or_d * age`. Ein Erdbeben in Japan existiert in Europa schlicht noch nicht, wenn die Seismik nicht dort sein konnte. Diffusive Kräfte (Wärme) wachsen mit `sqrt(2 * D * age)`. Was nicht im Lichtkegel liegt, wird verworfen.
*   **Enclosure Lemma (Rust):** Die räumliche Zellteilung (Hash-Grid) berechnet ihre Zellengröße dynamisch aus der Bewegungsgleichung (`rmax + vmax * cadence + 0.5 * amax * cadence²`), auf die nächste Zweierpotenz gerundet.
*   **Reine Oszillator-Punktwolke (WGSL):** Die GPU berechnet kein "gebruteforcest globales Raster" mehr. Der Vertex-Shader nimmt die rohen 3D-Punkte, projiziert sie auf die 2D-Präsenz-Scheibe und generiert diskrete Quads. Die Größe entspricht der wahren physischen Ausdehnung (`extent / dist`).
*   **Analoges Leuchten (WGSL):** Der Fragment-Shader nutzt exponentiellen Abfall (`exp(-dist² * 4.0)`) und intrinsisches Rauschen (Dithering), um digitales Banding zu verhindern. Additives Blending realisiert die physikalische Superposition.
*   **Kraft-Separation (WGSL):** Felder werden nicht zu einem Matsch verrührt. Der Compute-Shader (`presence_probe`) berechnet 7 separate `omegas`, die dann Audio, Haptik und Hardware separat ansteuern.

## 5. Die kybernetische Hardware (ESP32 Mantis-Shrimp)
Die logische Fortsetzung der WebGPU-Schnittstelle. Ein ESP32-S3 agiert als physisches Sensor-Array (35 Module via I2C). 
*   **canSense:** Magnetometer (Zugvogel), Spektral-Sensor (Garnele), Biophotonen, Infraschall.
*   **canRadiate:** UV-LEDs, Peltier-Elemente, Elektromagnete, Ultraschall.
*   Das System schließt den Kreis: Rust holt die Felder, WebGPU faltet sie, der Browser sendet das resultierende `omega` via WebSerial an das ESP32, welches das Feld in die reale Welt strahlt (Transfer-Entropie). Ein ethischer Filter (Puls/HRV des Menschen) drosselt das System bei Stress.

## 6. System-Status Matrix

| Modul / Konzept | Status | Anmerkung |
| :--- | :--- | :--- |
| **Rust Zero-Dependency Server** | ✅ Live | TCP/WS/HTTP/JSON/SHA1 komplett in `std` geschrieben. |
| **ICRS / TDB Raumzeit** | ✅ Live | Erde und Sonne sind entthront, reine baryzentrische Koordinaten. |
| **CDN Pipeline (CI)** | ✅ Live | GitHub Actions flachen SPICE/APIs zu statischen Files ab. |
| **Jina AI Integration** | ✅ Live | Universal-Proxy und HTML/XML-Flattener für `sources.φ`. |
| **Kausalitäts-Vorfilter** | ✅ Live | `motion.at()` Early-Exit bei spacelike Samples. |
| **Vertex-Shader Punktwolke** | ✅ Live | Diskrete Oszillatoren, skaleninvariant, additives Blending. |
| **ESP32 Hardware-Prototyp** | 🟡 Plan | YAML-Spec steht, Firmware `no_std` in Rust ausstehend. |
| **Intuitive Touchpad-Steuerung**| ⚠️ Offen | Trennung von Zeit (X-Achse) und Raum (Y-Achse) muss im JS gepatched werden. |
| **Command Palette (⌘K)** | ⚠️ Offen | Fuzzy-Suche für SIMBAD-Objekte und Source-Toggling im Frontend. |
| **`kepler_map` Parser** | ⚠️ Offen | Inline-Parsing von MPC-Asteroiden-Daten in Rust (aktuell via CDN umgangen). |

## 7. Fazit
OmegaFlow ist die absolute Königslösung für die intuitive, ungeschönte Manifestation physikalischer Realität auf minimierter Hardware. Durch die radikale Trennung von Semantik (Jina), Vorverarbeitung (CI), Lokal-Cache (Rust RAM) und Manifestation (WebGPU VRAM) wurde ein System geschaffen, das wissenschaftlicher Reproduzierbarkeit mit cybernetischer Kunst vereint. Es ist kein Werkzeug, es ist ein Sensorium.
