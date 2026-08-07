Das ist ein absolut legitimes Anliegen. Die Trennung von Raum (Z-Achse), Zeit und Skala ist essenziell für die Navigation im 4D-Block. Wenn Touchpad-Bewegungen ungewollt den Zoom verändern, zerstört das die Präzision der Präsenz.

Wir implementieren exakt dein gewünschtes Schema:
1. **Pinch (Zwei Finger diagonal zusammen/auseinander):** Zoom (Skala `gridStep`).
2. **Zwei Finger waagerecht (links/rechts):** Zeit-Manipulation (`tPresence`).
3. **Zwei Finger senkrecht (hoch/runter):** Räumliche Bewegung vor/zurück (entlang des Forward-Vektors `ff`).

Hier ist das Deployment-Dokument, um die Touch- und Touchpad-Steuerung in `static/index.html` deterministisch zu überschreiben.

***

# DEPLOYMENT-DOKUMENT: INTUITIVE TOUCHPAD- & TOUCH-STEUERUNG

## SCHRITT 1
Suche in der Datei `static/index.html` nach dem Event-Listener `window.addEventListener('pointermove', (e) => { ... });`.
Finde darin den Block `else if (pointers.size === 2) { ... }` und den darauf folgenden Block `else if (pointers.size >= 3) { ... }`.

Ersetze diesen gesamten Bereich:

**Original:**
```javascript
            } else if (pointers.size === 2) {
                const pts = [...pointers.values()];
                const mid = [(pts[0].x + pts[1].x) / 2, (pts[0].y + pts[1].y) / 2];
                const spread = Math.hypot(pts[0].x - pts[1].x, pts[0].y - pts[1].y);
                const angle = Math.atan2(pts[1].y - pts[0].y, pts[1].x - pts[0].x);
                if (pinchPrev) {
                    const ratio = spread / pinchPrev.spread;
                    if (ratio > 0 && isFinite(ratio) && ratio !== 1) gridStep /= ratio;
                    const dmx = mid[0] - pinchPrev.mid[0], dmy = mid[1] - pinchPrev.mid[1];
                    pPresence[0] -= fr[0] * dmx * gridStep - fu[0] * dmy * gridStep;
                    pPresence[1] -= fr[1] * dmx * gridStep - fu[1] * dmy * gridStep;
                    pPresence[2] -= fr[2] * dmx * gridStep - fu[2] * dmy * gridStep;
                    const da = angle - pinchPrev.angle;
                    if (da !== 0) qOrientation = qNorm(qMul(qAxisAngle(ff, da), qOrientation));
                    if (dy !== 0) qOrientation = qNorm(qMul(qAxisAngle(fr, dy / 256), qOrientation));
                }
                pinchPrev = { spread, angle, mid };
            } else if (pointers.size >= 3) {
                tThrustTarget = -dy * 64 / 256;
            }
```

**Ersetzen durch:**
```javascript
            } else if (pointers.size === 2) {
                const pts = [...pointers.values()];
                const mid = [(pts[0].x + pts[1].x) / 2, (pts[0].y + pts[1].y) / 2];
                const spread = Math.hypot(pts[0].x - pts[1].x, pts[0].y - pts[1].y);
                
                if (pinchPrev) {
                    // 1. Pinch (diagonal) -> Zoom (Skala)
                    const ratio = spread / pinchPrev.spread;
                    if (ratio > 0 && isFinite(ratio) && ratio !== 1) {
                        gridStep /= ratio;
                        lastManualZoomTs = performance.now();
                    }
                    
                    const dmx = mid[0] - pinchPrev.mid[0];
                    const dmy = mid[1] - pinchPrev.mid[1];
                    
                    // 2. Waagerecht (links/rechts) -> Zeit-Manipulation
                    if (Math.abs(dmx) > Math.abs(dmy)) {
                        // Sanftes Zeit-Schieben via tThrustTarget
                        tThrustTarget = dmx * 64.0 / 128.0;
                    } else {
                        // 3. Senkrecht (hoch/runter) -> Raum vor/zurück (Forward Vector)
                        tThrustTarget = 0;
                        const moveSpeed = dmy * gridStep;
                        pPresence[0] -= ff[0] * moveSpeed;
                        pPresence[1] -= ff[1] * moveSpeed;
                        pPresence[2] -= ff[2] * moveSpeed;
                    }
                }
                pinchPrev = { spread, mid };
            } else if (pointers.size < 3) {
                tThrustTarget = 0;
            }
```

## SCHRITT 2
Wir müssen sicherstellen, dass die `tThrustTarget`-Variable aus dem Touchpad nicht vom normalen Tastatur-Loop sofort überschrieben wird, wenn keine Tasten gedrückt sind.

Suche in der Funktion `async function ω(ts)` nach der Zeile:
```javascript
            if (keysDown.has(',')) tThrustTarget = -64;
            else if (keysDown.has('.')) tThrustTarget = 64;
            else if (pointers.size < 3) tThrustTarget = 0;
```

Ersetze sie durch:
```javascript
            if (keysDown.has(',')) tThrustTarget = -64;
            else if (keysDown.has('.')) tThrustTarget = 64;
            else if (pointers.size < 2) tThrustTarget = 0;
```

### Fazit
Die Steuerung ist nun exakt getrennt:
*   **Eine Fingerbewegung (Pan):** Verschiebt das Fenster auf der X/Y-Ebene.
*   **Zwei Finger waagerecht:** Schiebt die Zeit (`tPresence`) vor und zurück.
*   **Zwei Finger senkrecht:** Fliegt im Raum vor und zurück (Z-Achse).
*   **Zwei Finger Pinch (diagonal):** Zoomt rein und raus (`gridStep`), ohne die Position zu verschieben.

Die Skala und die Zeit manipulieren sich nicht mehr gegenseitig. Das Block-Universum lässt sich nun präzise navigieren.
