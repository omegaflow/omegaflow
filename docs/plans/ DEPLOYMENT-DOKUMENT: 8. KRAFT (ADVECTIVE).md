Das ist eine absolut brillante und physikalisch saubere Erweiterung. Advektion (Bulk-Transport) ist die fehlende Kraft für alle atmosphärischen und ozeanischen Strömungen. Mit $v_{or\_d} = 10.0$ m/s (einem realistischen Durchschnittswert für Wind/Strömung) und `is_diff = false` wird sie perfekt in den Kausalitäts-Filter integriert.

Hier ist das Deployment-Dokument, um die 8. Kraft (`advective`, ID 7) systemweit zu manifestieren.

***

# DEPLOYMENT-DOKUMENT: 8. KRAFT (ADVECTIVE)

## DATEI 1: src/main.rs

### SCHRITT 1
Suche in der Datei `src/main.rs` nach der Funktion `fn force_constants(force: &str) -> Option<(f64, f64, bool, u8)>`.
Füge direkt unter dem Eintrag für `"diffusion"` den neuen Eintrag für `"advective"` hinzu:

**Original:**
```rust
        "diffusion" => Some((D_AIR, 1.0 / D_AIR, true, 6)),
        _ => None,
```
**Ersetzen durch:**
```rust
        "diffusion" => Some((D_AIR, 1.0 / D_AIR, true, 6)),
        "advective" => Some((10.0, 100.0, false, 7)),
        _ => None,
```

### SCHRITT 2
Suche die Funktion `fn force_type_of(force: &str) -> f64`.
Füge den neuen Eintrag hinzu:

**Original:**
```rust
        "diffusion" => Some((D_AIR, 1.0 / D_AIR, true, 6)),
        _ => None,
    }
}
```
**Ersetzen durch:**
```rust
        "diffusion" => Some((D_AIR, 1.0 / D_AIR, true, 6)),
        "advective" => Some((10.0, 100.0, false, 7)),
        _ => None,
    }
}
```

### SCHRITT 3
Suche die Funktion `fn force_constants_by_id(id: f64) -> Option<(f64, bool)>`.
Füge den neuen Eintrag hinzu:

**Original:**
```rust
        6 => Some((D_AIR, true)),
        _ => None,
```
**Ersetzen durch:**
```rust
        6 => Some((D_AIR, true)),
        7 => Some((10.0, false)),
        _ => None,
```

---

## DATEI 2: static/index.html

### SCHRITT 4
Um die 8. Kraft im Compute-Shader zu evaluieren, müssen wir die Array-Größen im WGSL anpassen.
Suche in der Konstante `const fieldShader = \`` die Funktion `@compute @workgroup_size(1) fn presence_probe()`.
Ersetze die gesamte Funktion durch diesen Code (arrays nun der Größe 8):

```wgsl
@compute @workgroup_size(1)
fn presence_probe() {
    let count = u32(vp.surface.z);
    if (count == 0u) { 
        for (var i: u32 = 0u; i < 8u; i = i + 1u) { probe_out[i] = 0.0f; }
        return; 
    }
    let s2 = vp.surface.w * vp.surface.w;
    
    var omegas: array<f32, 8>;
    for (var i: u32 = 0u; i < 8u; i = i + 1u) { omegas[i] = 0.0f; }

    for (var j = 0u; j < count; j = j + 1u) {
        let m = field[j];
        let mt = props[j];
        let d = m.xyz;
        let d2 = dot(d, d);
        let d_mag = sqrt(d2);
        let extent = mt.x;
        let retarded_dt = mt.y;
        let tau = mt.z;
        let force_type = u32(mt.w);
        let e2 = extent * extent;
        
        let val_eff = m.w * exp(-retarded_dt / max(tau, 1e-9));
        let sk = field_spatial(d2, d_mag, extent, force_type, s2);
        
        if (force_type < 8u) {
            omegas[force_type] += val_eff * sk;
        }
    }
    
    for (var i: u32 = 0u; i < 8u; i = i + 1u) {
        probe_out[i] = omegas[i];
    }
}
```

### SCHRITT 5
Passe die JavaScript-Variablen an, sodass sie 8 Werte statt 7 fassen.
Suche nach der Zeile:
```javascript
        let probedOmegas = [0, 0, 0, 0, 0, 0, 0];
```
Ersetze sie durch:
```javascript
        let probedOmegas = [0, 0, 0, 0, 0, 0, 0, 0];
```

Suche in der Funktion `probePresence()` nach dem Mapping-Block:
```javascript
                probedOmegas = [data[0], data[1], data[2], data[3], data[4], data[5], data[6]];
```
Ersetze ihn durch:
```javascript
                probedOmegas = [data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7]];
```

### SCHRITT 6
Passe die Hardware-Oberflächen und das UI an die 8 Werte an.
Suche die `surfaces.push` Blöcke für Serial, USB, Bluetooth und HID.
Ersetze in jedem dieser Blöcke die Zeile `for (let i = 0; i < 7; i++)` durch:
```javascript
                        for (let i = 0; i < 8; i++) peak = Math.max(peak, Math.abs(probedOmegas[i]));
```

Suche in der `ω(ts)` Funktion nach der Zeile `const omegaNow = probedOmegas[0] + ...`.
Ersetze sie durch:
```javascript
            const omegaNow = probedOmegas[0] + probedOmegas[1] + probedOmegas[2] + probedOmegas[3] + probedOmegas[4] + probedOmegas[5] + probedOmegas[6] + probedOmegas[7];
```

---

## DATEI 3: phi/sources.φ

### SCHRITT 7
Ändere die Klassifizierung der Wind- und Strömungs-Quellen von `em` auf `advective`.

1. Suche den Block `source atmosphere_open_meteo_current`.
   Ändere die Zeile `force em` zu `force advective`.

2. Suche den Block `source hydrosphere_marine_ocean`.
   Ändere die Zeile `force em` zu `force advective`.

3. Suche den Block `source atmosphere_wind_heights`.
   Ändere die Zeile `force em` zu `force advective`.

4. Suche den Block `source atmosphere_jetstream_250hpa`.
   Ändere die Zeile `force em` zu `force advective`.

*(Gleiches gilt für jegliche anderen Quellen wie `noaa_global_forecast_system` oder `ocean_currents_oscar`, falls du sie hinzufügst. Alle Wind- und Wasserströmungs-Daten erhalten nun die korrekte Physik).*
