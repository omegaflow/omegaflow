# Survey-Auswertung — Konsens der vier Modelle (Shared-Memory-Runde)

Quellen: /home/johannes/Schreibtisch/survey/*_Shared_Memory (Sonnet, Max, GLM, Kimi, Deepseek).

## Der Konsens: Das Token stirbt, der Zähler lebt

Alle vier Modelle konvergieren unabhängig auf denselben Satz — in Max' Formulierung:

> **Der Buffer ist die Nachricht. Der Zähler ist das Protokoll. Die Stille ist die Optimierung.**

Archivar und Mathematikerin teilen denselben physischen Speicher. Damit existiert keine zweite Kopie, die divergieren könnte — ein Ω-Token, das zwei Seiten vergleicht (mit Toleranz, mit f32-Epsilon), löst ein Problem, das die Architektur nicht hat. Der GPU-Readback, den der Vergleich erzwingen würde, IST selbst Reibung. Kandidat 10 in seiner Token-Form wird verworfen — von allen vier.

**Was an seine Stelle tritt:**

1. **Ein atomarer u64-Generationszähler.** Der Archivar schreibt Quellen in den Buffer, inkrementiert danach atomar `generation`. Die Mathematikerin vergleicht eine Ganzzahl mit ihrem `generation_seen`: gleich → aus dem bestehenden Buffer rendern (kein Re-Sense, kein Vergleichs-Äquivalent, keine Toleranz — der Vergleich ist exakt, weil er ein Integer ist); verschieden → den neuen, bereits vollständig geschriebenen Buffer lesen.
2. **Das einzig verbleibende Kohärenzproblem ist die Schreib-Sichtbarkeit** (Torn Reads): Sonnet — Doppelpuffer (A/B alternierend), der Zähler wird erst nach dem vollständigen Schreiben erhöht.
3. **Stille als Zustand (0 honored).** Der Archivar schreibt NICHTS, wenn das Feld stationär ist — keine Heartbeats, keine Token, keine Stichproben. Max: Allostase als Nicht-Schreiben; Kimi: „Ein ruhendes Feld strahlt nicht. Ein ruhender Archivar sendet nicht."
4. **Die HUD-Ω löst sich auf.** Die Ω-Werte pro Kraft sind Akkumulatoren, die der Archivar beim Buffer-Bau ohnehin berechnet — sie liegen im gemeinsamen Speicher. Die HUD liest sie direkt. Kein GPU-Map-Readback, kein Token, kein Warten (beseitigt den gemessenen Map-Timeout unter Last).
5. **Budget-Regelung (Deepseek):** zwei Zahlen über die Grenze — Frame-Zeit + Messpunktzahl — der Archivar passt die Hintergrund-Zellweite an. Reibungsärmste Dichteskalierung.

## Was offen bleibt (für die frische Session)

- Die **Verteilungs-Verdikte der früheren Runden** (Council/Extension-Dateien, 9 Kandidaten) sind hier noch nicht ausgewertet — GLMs Shared-Memory-Antwort hält die adaptive Zellhierarchie/Quadtree-LOD als GPU-seitige Entscheidung fest, aber die vollständigen Begründungen liegen in den größeren Dateien.
- Die Doppelpuffer-/Generations-Architektur ist zu entwerfen und zu messen (sie ersetzt den heutigen sense-Worker-Roundtrip an den richtigen Stellen).
- Die Messpunkt-Verteilung wird erst nach der Verteilungs-Auswertung gewählt.

## Verdikt

Kandidat 10 (Feld-Token) ist als Änderungs-Detektor verworfen — die Architektur heißt: **ein Buffer, ein Zähler, sonst Stille.** Die Reibung ist das Verarbeiten; je weniger Verarbeitung, desto weniger Reibung.
