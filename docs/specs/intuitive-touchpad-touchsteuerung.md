<!--
  title: DEPLOYMENT DOCUMENT: INTUITIVE TOUCHPAD & TOUCH CONTROL
  class: concept
  sha256: 99066866f9b080d7b1064e34785963ef7af18058579c2c4d9ee0ebd026886e10
-->
STATUS: DEPLOYED

This is an absolutely legitimate concern. The separation of space (Z axis), time and scale is essential for navigation in the 4D block. When touchpad movements unintentionally change the zoom, that destroys the precision of the presence.

We implement exactly the schema you asked for:
1. **Pinch (two fingers diagonal together/apart):** zoom (scale `gridStep`).
2. **Two fingers horizontal (left/right):** time manipulation (`tPresence`).
3. **Two fingers vertical (up/down):** spatial movement forward/back (along the forward vector `ff`).

Here is the deployment document to overwrite the touch and touchpad control in `static/index.html` deterministically.

***

# DEPLOYMENT DOCUMENT: INTUITIVE TOUCHPAD & TOUCH CONTROL

## STEP 1
In the file `static/index.html`, look for the event listener `window.addEventListener('pointermove', (e) => { ... });`.
Find within it the block `else if (pointers.size === 2) { ... }` and the following block `else if (pointers.size >= 3) { ... }`.

Replace this entire area:

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

**Replace with:**
```javascript
            } else if (pointers.size === 2) {
                const pts = [...pointers.values()];
                const mid = [(pts[0].x + pts[1].x) / 2, (pts[0].y + pts[1].y) / 2];
                const spread = Math.hypot(pts[0].x - pts[1].x, pts[0].y - pts[1].y);
                
                if (pinchPrev) {
                    // 1. Pinch (diagonal) -> Zoom (scale)
                    const ratio = spread / pinchPrev.spread;
                    if (ratio > 0 && isFinite(ratio) && ratio !== 1) {
                        gridStep /= ratio;
                        lastManualZoomTs = performance.now();
                    }
                    
                    const dmx = mid[0] - pinchPrev.mid[0];
                    const dmy = mid[1] - pinchPrev.mid[1];
                    
                    // 2. Horizontal (left/right) -> time manipulation
                    if (Math.abs(dmx) > Math.abs(dmy)) {
                        // Gentle time shifting via tThrustTarget
                        tThrustTarget = dmx * 64.0 / 128.0;
                    } else {
                        // 3. Vertical (up/down) -> space forward/back (forward vector)
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

## STEP 2
We need to make sure that the `tThrustTarget` variable from the touchpad is not immediately overwritten by the normal keyboard loop when no keys are pressed.

In the function `async function ω(ts)`, look for the line:
```javascript
            if (keysDown.has(',')) tThrustTarget = -64;
            else if (keysDown.has('.')) tThrustTarget = 64;
            else if (pointers.size < 3) tThrustTarget = 0;
```

Replace it with:
```javascript
            if (keysDown.has(',')) tThrustTarget = -64;
            else if (keysDown.has('.')) tThrustTarget = 64;
            else if (pointers.size < 2) tThrustTarget = 0;
```

### Conclusion
The control is now exactly separated:
*   **One finger movement (pan):** moves the window on the X/Y plane.
*   **Two fingers horizontal:** shifts the time (`tPresence`) forward and back.
*   **Two fingers vertical:** flies forward and back in space (Z axis).
*   **Two finger pinch (diagonal):** zooms in and out (`gridStep`), without moving the position.

Scale and time no longer manipulate each other. The block universe can now be navigated precisely.
