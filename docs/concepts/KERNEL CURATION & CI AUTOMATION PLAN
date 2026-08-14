Ich verstehe Deine Sorge absolut. "For now but later" ist der Tod jeder Architektur. Wir dürfen diese NASA-Liste nicht einfach als "späteres Problem" abheften. 

Wenn wir das System auf das v6-Protokoll und das Trommelfell umgestellt haben (Päckchen 2 und 3), brauchen wir einen **scalebaren, automatisierten Prozess**, um diese Flut an Kerneln in die CI-Pipeline zu integrieren, ohne dass Du jeden Block manuell tippen musst.

Hier ist der offizielle **Master-Curation & Automation Plan**. Du kannst diesen Text als Markdown-Datei (`docs/kernel_curation_plan.md`) speichern und in der nächsten, isolierten Session dem Kybernauten geben. Er löst das Problem nicht durch "später machen", sondern durch "Automatisierung jetzt definieren".

***

### Kopiere diesen Text als Plan für die Kernel-Kuration:

___

# OMEGAFLOW — Kernel Curation & CI Automation Plan

**Status:** Architektur-Direktive für die skalierbare Integration aller NAIF SPICE Kernel.
**Prinzip:** Kein "For now". Wir bauen eine Pipeline, die automatisch tausende Körper in flattened `.bin`-Dateien auf dem CDN manifestiert, ohne manuellen Code-Input.

## 1. Die Datenquellen (Die rohe NASA-Bibliothek)
Das System unterteilt die NASA-Kernel in vier Domänen:
1. **Planetary (DE440s):** Sonne, Planeten, Mond. (Bereits integriert, wird in Päckchen 2 auf v2 aktualisiert).
2. **Satellites:** Monde von Jupiter, Saturn, Uranus, Neptun, Pluto (z.B. `jup344.bsp`, `sat454.bsp`). Dateien sind teilweise riesig (bis 1.9 GB).
3. **Asteroids & Comets:** Zehntausende Körper aus `asteroids_de441/` und Kometen wie `67p/`.
4. **Spacecraft:** Sonden wie Voyager (`vgr1.x2100.bsp`), Cassini, Parker Solar Probe.

## 2. Die CI-Pipeline (Der Flattening-Prozess)
Die Integration geschieht ausschließlich über die CI-Pipeline (z.B. GitHub Actions). Kein lokaler Download von 3 GB-Dateien auf dem XPS 13.

**Der CI-Workflow (`.github/workflows/kernel_flatten.yml`):**
1. **Fetch:** Die CI lädt die Master `.bsp`-Dateien von `ssd.jpl.nasa.gov/ftp/eph/` herunter.
2. **Compile:** Die CI nutzt den `ephemeris_compiler` (mit dem neuen `--gm` und `--pck` Flags aus Päckchen 2), um die Körper zu extrahieren. Die CI generiert für jeden Körper eine `ephemeris_{body}.bin` (v2 Format, gcount=12).
3. **Push to CDN:** Die CI pusht die generierten `.bin`-Dateien als Release-Assets zum Omegaflow GitHub-CDN.
4. **Indexing:** Die CI aktualisiert eine `sources_index.φ`, die alle verfügbaren Körper als URL-Liste enthält.

## 3. Die Lokale Symbiose (Das Enclosure Lemma)
Auf dem lokalen XPS 13 (dem Browser/Präsenzfenster) läuft nichts von dieser Schwerstarbeit. 

1. Das Frontend meldet dem Backend: "Ich schwebe bei `(x, y, z, t)` im Jupiter-System."
2. Das Rust-Backend nutzt das **Enclosure Lemma**. Es weiß, dass es für Jupiter Monde geben könnte.
3. Das Backend lädt *nur* die `ephemeris_io.bin`, `ephemeris_europa.bin` etc. vom CDN herunter (geflattet, wenige Kilobyte) und cached sie lokal in `/tmp`.
4. Taucht der Punkt auf, an dem die Voyager-Sonde ist, lädt es `ephemeris_voyager1.bin`. 

## 4. Implementierungsschritte (Für die Kurations-Session)
Dieses Päckchen wird in einer dedizierten Session gebaut, *nachdem* das v6-Protokoll und das Trommelfell (Päckchen 2 & 3) stabil laufen.

1. **`scripts/generate_sources.py` (oder Rust):** Ein Skript, das die NASA-Verzeichnisse scannt und die `sources.φ` Blöcke für `ephemeris_binary` automatisch generiert.
2. **NAIF-ID-Mapping-Tabelle:** Vervollständigung der `pck_id_of` und `body_name_of` Maps in Rust für alle Saturn/Jupiter/Uranus-Monde und Asteroiden-Gürtel-Objekte.
3. **CI-Skript:** YAML-Workflow, der den Compiler in der Cloud über die Liste iterieren lässt.

## Fazit
Wir verschieben das Problem nicht auf "later". Wir definieren jetzt den exakten Mechanismus, wie das System diese Datenflut bewältigen wird: durch CI-Automatisierung und dynamisches Enclosure-Lemma-Loading. Die Daten existieren, die Pipeline ist definiert, die lokale Hardware wird geschont.

___

### Warum das Dein "For now but later"-Problem löst:
Du hast jetzt einen **architektonischen Bauplan**. Wenn jemand (oder eine KI) fragt: "Wann integrieren wir die Jupiter-Monde?", ist die Antwort nicht "später", sondern: "Sobald Päckchen 2 & 3 fertig sind, bauen wir diese CI-Pipeline, und sie werden automatisch integriert."

Wir haben den Plan. Er ist sicher abgelegt. Jetzt kehren wir zu DeepSeek zurück und lassen uns den Code für **Päckchen 2** geben!
