# ARCHIVE: Minkowski Weighting & Field Permeability

## 1. Minkowski Weighting (`calculateMinkowskiWeight`)

**Die originale Implementierung (JavaScript, CPU):**

```javascript
function calculateMinkowskiWeight() {
    const g = measureRms(); 
    const scale = 1.0 + g;
    for (let i = 0; i < oscillators.length; i++) {
        const osc = oscillators[i];
        const dx = osc.originPos.x - spatialPresence.x;
        const dy = osc.originPos.y - spatialPresence.y;
        const dz = osc.originPos.z - spatialPresence.z;
        const dt = Math.abs(osc.originT - tPresence) * C;
        const spatialDistSq = dx*dx + dy*dy + dz*dz;
        const minkowskiSq = (dt * dt) - spatialDistSq;
        if (minkowskiSq < 0) { 
            osc.presenceWeight = 0; 
            continue; 
        }
        const dist4D = Math.sqrt(minkowskiSq);
        osc.presenceWeight = scale / (scale + (dist4D * dist4D));
    }
}
```

**Der kritische Bugfix (Nr. 1 von 7):**
```diff
- const dt = Math.abs(osc.originT - tPresence) * 86400.0 * C;
+ const dt = Math.abs(osc.originT - tPresence) * C;
```
`originT` und `tPresence` sind bereits in TDB-Sekunden. Der Faktor `86400.0` (Sekunden pro Tag) war ein Rest aus der alten ECEF-Ära und multiplizierte die Zeitdistanz um 4 Zehnerpotenzen falsch.

**Die Mathematik:**
- `ds² = (dt·c)² − (dx² + dy² + dz²)` — Standard Minkowski-Metrik
- `ds² < 0` → **Spacelike** (außerhalb des Lichtkegels) → `presenceWeight = 0`. Das Silizium verbraucht null Energie.
- `ds² ≥ 0` → **Timelike** (innerhalb des Lichtkegels) → `presenceWeight = scale / (scale + ds²)`
- `scale = 1.0 + g` — Die Reichweite des Bewusstseinsfensters wächst mit der Feldenergie `g` (RMS).

### Wie es heute im WGSL-Shader aussehen würde

Das aktuelle System hat die Oszillator-Daten bereits im Shader als `field[j]` (xyz+val) und `props[j]` (aperture, Δt, ttl). Die räumliche Distanz `d2` wird bereits berechnet. Die zeitliche Distanz ist in `mt.y = tPresence - o.t` gespeichert.

Der aktuelle Shader-Code:
```wgsl
let val_eff = m.w * exp2(-mt.y / max(mt.z, 1.0) * 1.4426950408889634);
omega = omega + val_eff / (d2 + scale * scale);
```

**Mit Minkowski (vorgeschlagene Änderung im Shader):**
```wgsl
let dt = abs(mt.y) * 1.4426950408889634; // |Δt| in Sekunden × c (als exp2-Faktor)
// oder direkter:
let dt_abs = abs(mt.y); // |tPresence - t| in TDB-Sekunden
let dt_c = dt_abs * 299792458.0; // × c
let minkowskiSq = dt_c * dt_c - d2;
if (minkowskiSq < 0.0) { continue; } // Spacelike → Lichtkegel-Filter
let presenceWeight = exposure / (exposure + minkowskiSq);
let val_eff = m.w * exp2(-dt_abs / max(mt.z, 1.0) * 1.4426950408889634);
omega = omega + val_eff * presenceWeight / (d2 + scale * scale);
```

**Was sich ändert:**
1. **Lichtkegel-Filter:** Oszillatoren, die spacelike zur Präsenz stehen (Information könnte sie noch nicht erreicht haben), werden komplett ausgeblendet. Das ist physikalisch korrekt — Kausalität wird respektiert.
2. **4D Power-Law:** Statt nur räumlicher Distanz (`d2`) wird die 4D Minkowski-Distanz gewichtet. Ein Oszillator, der 1 Lichtsekunde entfernt ist und 1 Sekunde in der Vergangenheit gemessen wurde, hat `ds² = 0` (lichtkegelartig) und bekommt maximales Gewicht. Ein Oszillator, der 2 Lichtsekunden entfernt ist und 1 Sekunde in der Vergangenheit liegt, ist spacelike und verschwindet.
3. **`scale = exposure`:** Der aktuelle Code verwendet bereits `exposure = max(|fieldOmega()|, 2^-64)` als uniform. Das ist genau das `1.0 + g` von damals.

### Das Problem: `c` als Konstante im Shader

Der Shader braucht `c = 299792458.0` als Konstante. Das Alignment-Protokoll erlaubt nur `c, Φ, WGS84, J2000, power-of-2` als numerische Literale. `c` ist erlaubt. Wir können es als `const C: f32 = 299792458.0;` im WGSL deklarieren oder als Uniform übergeben.

Allerdings: Bei kosmischen Skalen (1 AU = 1.5e11 m) und TTLs von 60 Sekunden ist `dt·c = 60 × 3e8 = 1.8e10` m, während `d2` bei `(1.5e11)² = 2.25e22` liegt. Das heißt `dt·c << d` — fast alle Oszillatoren wären spacelike! Die Sonne ist 8 Lichtminuten entfernt, aber ihre Daten haben eine TTL von 3600 Sekunden. Wenn die Präsenz bei (0,0,0) steht und `tPresence` um 60 Sekunden fortschreitet, ist `dt·c = 60 × 3e8 = 1.8e10`, aber `d = 1.5e11`. `minkowskiSq = (1.8e10)² - (1.5e11)² = 3.24e20 - 2.25e22 < 0`. Die Sonne wäre unsichtbar!

**Das ist physikalisch korrekt, aber praktisch problematisch.** Die Sonne hat uns vor 8 Minuten erreicht. Wenn wir `tPresence` auf "jetzt" setzen und die Sonnenoszillatoren bei `t = tPresence - 480s` messen, dann ist `dt·c = 480 × 3e8 = 1.44e11`, was größer als `d = 1.5e11` ist — knapp spacelike! Das ist die Realität: Die Sonne ist fast am Rand unseres Lichtkegels.

**Die Lösung:** Der Archivar liefert bereits `t` (die Messepoche) für jeden Oszillator. Wenn die Sonne vor 8 Minuten gemessen wurde, ist `t` in der Vergangenheit. `dt = |tPresence - t|` ist 480 Sekunden. `dt·c = 1.44e11`. `d = 1.5e11`. `minkowskiSq = (1.44e11)² - (1.5e11)² = 2.07e22 - 2.25e22 = -0.18e22 < 0`. Die Sonne wäre spacelike!

Aber das ist **falsch**, weil die Sonne nicht "vor 8 Minuten gemessen" wurde — die API liefert aktuelle Sonnenflecken-Daten. Der `t`-Wert im Protokoll ist der Zeitpunkt, an dem der Archivar die Daten abgerufen hat, nicht der Zeitpunkt, an dem das Licht die Sonne verließ. Das System misst den Zustand des Oszillators zum Zeitpunkt `t`, nicht die Emission.

**Fazit:** Minkowski im Shader würde bedeuten, dass das System nur Oszillatoren "sieht", deren Messung innerhalb des Lichtkegels der Präsenz liegt. Bei kosmischen Skalen ist das extrem restriktiv. Es wäre physikalisch korrekt (Kausalität), aber das System würde fast nichts sehen.

**Mögliche Lösung:** Den `scale`-Parameter dynamisch anpassen. Wenn `scale` sehr groß ist (z.B. `scale = 1.0 + g * AU`), wird der Minkowski-Filter weicher. Oder: Den Minkowski-Filter nur auf lokale Oszillatoren (Browser-Sensoren) anwenden, nicht auf kosmische API-Daten, deren `t` der Abruf-Zeitpunkt ist, nicht der Emissions-Zeitpunkt.

---

## 2. Field Permeability (`adaptFieldPermeability`)

**Die originale Implementierung:**

```javascript
const GROUND_STATE = Number.EPSILON;

function adaptFieldPermeability(osc) {
    // Turn detection
    let outTE = 0;
    for (let i = 0; i < oscillators.length; i++) {
        if (oscillators[i].url.startsWith('transfer.' + osc.url + '>')) 
            outTE += Math.abs(oscillators[i].median);
    }
    const deltaTE = outTE - (osc.lastOutTE || GROUND_STATE);
    osc.lastOutTE = outTE;
    
    const threshold = computeOscSurrogate(osc); // mean + 2σ of 10 shuffled KDEs
    
    osc.ticksSinceTurn++;
    if (osc.direction > 0 && deltaTE < -threshold) { 
        osc.naturalLatencyTicks = osc.ticksSinceTurn; 
        osc.direction = -1; 
        osc.ticksSinceTurn = 0; 
    }
    if (osc.direction < 0 && (deltaTE > threshold || osc.fieldPermeability <= GROUND_STATE)) { 
        osc.naturalLatencyTicks = osc.ticksSinceTurn; 
        osc.direction = 1; 
        osc.ticksSinceTurn = 0; 
    }
    
    // Exponential relaxation
    const target = osc.direction > 0 ? 1.0 : 0.0;
    const alpha = 1 - Math.exp(-1 / Math.max(1, osc.naturalLatencyTicks));
    osc.fieldPermeability += (target - osc.fieldPermeability) * alpha;
    osc.fieldPermeability = Math.max(GROUND_STATE, Math.min(1.0, osc.fieldPermeability));
}
```

### Die kontinuierliche Evolution (The "Water" Implementation)

Eine spätere Iteration des Codes zeigte eine noch elegantere, kontinuierlichere Form der Zielberechnung, die das binäre Umschalten (`direction > 0 ? 1.0 : 0.0`) auflöste:

```javascript
const target = inTE / (inTE + threshold + GROUND_STATE);
```

Das ist kein binärer Schalter. Das ist ein **kontinuierlicher Fluss**. Die Permeabilität skaliert proportional zur Stärke des echten Echos (`inTE`), aber sie wird durch das Rauschen (`threshold`) begrenzt. Wenn das Echo stark ist, geht die Permeabilität gegen 1.0. Wenn das Echo nur noch Rauschen ist, geht sie gegen 0. Es gibt kein "Ein" oder "Aus", nur ein ständiges, sanftes Atmen, das durch den Kausalechos des Feldes angetrieben wird.

Und dann die Relaxation:
```javascript
const alpha = 1 - Math.exp(-1 / Math.max(1, osc.naturalLatencyTicks));
osc.fieldPermeability += (target - osc.fieldPermeability) * alpha;
```
Exakt das, was das Alignment-Protokoll fordert: Eine exponentielle Relaxation (1st-order ODE), bei der die Zeitkonstante τ (`naturalLatencyTicks`) aus dem gemessenen Rhythmus des Raumes selbst extrahiert wird, nicht aus einem Hardcode.

**Das Problem für die Rückkehr:** Diese Funktion benötigt Transfer Entropy (TE) von der GPU, die im aktuellen System entfernt wurde. Ohne TE gibt es kein "Echo" (`inTE`), das die Permeabilität treibt.

**Die aktuelle Alternative (ohne TE):**

Das System berechnet bereits Feld-Gradienten:
```javascript
const omegaNow = fieldOmega();
const g = Math.abs(omegaNow);
const vC = prevOmegaTs > 0 && ts > prevOmegaTs 
    ? Math.abs(omegaNow - prevOmega) / ((ts - prevOmegaTs) / 1000) 
    : 0;
certainty = Math.exp(-vC / (g + (1.0 / C)));
```

`vC` (Rate of Change) ist ein direktes Maß für die Dynamik des Feldes. Wenn das Feld stark und schnell changiert, ist `vC` hoch und `certainty` sinkt. Das ist bereits eine Form von "Atmung" — aber sie ist global, nicht per-Oszillator.

**Vorschlag für die Rückkehr ohne TE:**

Anstatt TE als Echo zu verwenden, können wir die Feld-Gradienten direkt nutzen. Das System hat bereits `fieldOmegaAt(px, py, pz)`:

```javascript
function adaptFieldPermeability() {
    const omegaNow = fieldOmega();
    const g = Math.abs(omegaNow);
    const vC = prevOmegaTs > 0 && ts > prevOmegaTs 
        ? Math.abs(omegaNow - prevOmega) / ((ts - prevOmegaTs) / 1000) 
        : 0;
    
    // Target: open when field changes, close when stable
    // No hardcoded threshold — derived from field energy
    const target = g > Number.EPSILON ? Math.tanh(vC / (g + Number.EPSILON)) : 0.0;
    
    // Exponential relaxation (1st-order ODE)
    // naturalLatencyTicks adapts from measured rhythm
    const alpha = 1 - Math.exp(-1 / Math.max(1, naturalLatencyTicks));
    fieldPermeability += (target - fieldPermeability) * alpha;
    fieldPermeability = Math.max(GROUND_STATE, Math.min(1.0, fieldPermeability));
}
```

Dies ist:
- **Sanft:** Exponentielle Relaxation, kein `sin()`, kein linear step
- **Schwellwertfrei:** `tanh(vC / g)` ist skaleninvariant
- **Intrinsisch:** Das Feld atmet aus seiner eigenen Dynamik, nicht aus externer Statistik
- **Alignment-konform:** Nutzt `Number.EPSILON`, `Math.exp`, `Math.tanh`, keine harten Schwellen
