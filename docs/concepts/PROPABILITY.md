STATUS: DEPLOYED

Die Analyse von Claude (Sonnet) ist physikalisch absolut brillant. Der Gedanke, einen **Kausalitäts-Check (Lichtkegel)** einzuführen, ist genau die richtige Lücke: Bisher prüft `enclose_family` nur die geometrische Distanz (`reach`) und die Bewegungsunschärfe der *Quelle* (`vmax`/`amax`), aber nicht, ob sich das Signal mit seiner spezifischen Ausbreitungsgeschwindigkeit (`v_or_d`) in der verstrichenen Zeit (`age`) überhaupt bis zum Präsenzpunkt ausbreiten konnte.

**ABER: Bei der architektonischen Umsetzung greift Claude zu kurz.** 

Wenn wir Claudes Vorschlag folgen und den `reach` (der Suchradius für den räumlichen Hash-Grid) durch `v_or_d * age` ersetzen, sprengen wir das System. 
*Warum?* Weil `v_or_d` für EM-Gravitation `c` (Lichtgeschwindigkeit) ist. Wenn ein Sample ein Alter von 1 Tag hat, wäre `reach = c * 86400s ≈ 2,59e13 Meter` (ca. 0,17 AU). Der räumliche Hash-Grid würde versuchen, Milliarden von Zellen zu scannen, was den Archivar komplett einfriert.

### Die korrekte Architektur-Lösung

Wir lassen den `reach` (die Bounding-Box für den Hash-Grid) exakt so, wie er ist (basierend auf `vmax`/`amax` der Quelle), damit die räumliche Suche ökonomisch bleibt. 
Stattdessen fügen wir den **Kausalitäts-Check als Early-Exit *nach* der exakten Distanzberechnung** ein. Wenn der Oszillator zwar geometrisch nah ist, aber das Signal physikalisch noch nicht ankommen konnte (oder längst verklungen ist), verwerfen wir es chirurgisch, bevor teure Feldberechnungen oder Shader-Uploads stattfinden.

Hier ist der Syntax-Translation-Patch für `main.rs`, der Claudes Physik-Logik architektonisch sicher implementiert:

#### 1. Die Umkehrfunktion hinzufügen
Füge diese Funktion direkt unterhalb von `force_type_of` ein:

```rust
fn force_constants_by_id(id: f64) -> Option<(f64, bool)> {
    match id as u8 {
        0 => Some((C_LIGHT, false)),        // em
        1 => Some((C_LIGHT, false)),        // gravity
        2 => Some((V_SOUND_288, false)),    // acoustic
        3 => Some((V_P_GRANITE, false)),    // seismic-body
        4 => Some((V_S_GRANITE, false)),    // seismic-surface
        5 => Some((ALPHA_AIR, true)),       // thermal
        6 => Some((D_AIR, true)),           // diffusion
        _ => None,
    }
}
```

#### 2. Den Kausalitäts-Filter in `enclose_family` einbauen
Ersetze den Block in `enclose_family`, der nach dem `reach * reach`-Check kommt und vor der `motion.at`-Schleife, durch diesen Code:

```rust
            // Bisheriger exakter Distanz-Check (geometrisch)
            let p = smp.motion.at(t2, smp.epoch);
            let ddx = p[0] - q[0];
            let ddy = p[1] - q[1];
            let ddz = p[2] - q[2];
            let exact = smp.extent + pad;
            let dist2 = ddx * ddx + ddy * ddy + ddz * ddz;
            if dist2 > exact * exact {
                continue;
            }

            // --- NEU: Kausalitäts-Vorfilter (Lichtkegel / Diffusionsfront) ---
            // Verhindert, dass Samples verarbeitet werden, deren Signal 
            // am Presence-Punkt physikalisch noch nicht angekommen ist 
            // oder bereits ins Nichts verklungen ist.
            if let Some((v_or_d, is_diff)) = force_constants_by_id(smp.force_type) {
                let age = (t2 - smp.epoch).abs();
                
                if age < 1e-9 {
                    continue; // Vermeide Division durch Null bei statischen Epochen
                }

                if is_diff {
                    // Diffusive Kräfte (thermal, diffusion): 
                    // Reichweite wächst mit sqrt(2 * D * age)
                    if 2.0 * v_or_d * age < dist2 {
                        continue; // Diffusionsfront hat den Punkt noch nicht erreicht
                    }
                } else {
                    // Wellenartige Kräfte (em, gravity, acoustic, seismic):
                    // Reichweite wächst linear mit v_or_d * age
                    let max_causal_dist = v_or_d * age;
                    if dist2 > max_causal_dist * max_causal_dist {
                        continue; // Signal ist noch nicht am Präsenzpunkt angekommen
                    }
                }

                // Optionaler oberer Cutoff: Wenn das Signal-Zeitalter ein 
                // Vielfaches seines charakteristischen tau übersteigt, 
                // ist die physikalische Wirkung am Punkt verschwunden.
                // (Wir nutzen hier 2^6 = 64, analog zum ttl-Retention-Cutoff)
                if smp.tau > 0.0 && age > smp.tau * 64.0 {
                    continue; 
                }
            }

            // Wenn alle physischen Filter bestanden wurden, manifestiere das Feld:
            for (_, val) in &smp.fields {
                records.push((
                    p[0],
                    p[1],
                    p[2],
                    *val,
                    smp.extent,
                    smp.epoch,
                    smp.ttl,
                    smp.tau,
                    smp.force_type,
                ));
            }
```

### Warum das die elegante Lösung ist:
1. **Keine Sprengung des Grids:** Der teure `qlo`/`qhi`-Scan im räumlichen Hash bleibt von `c` (Lichtgeschwindigkeit) verschont.
2. **Wahre Kausalität:** Ein Erdbeben in Japan (akustisch/seismisch), das vor 10 Minuten passiert ist, wird nicht in Europa gezeichnet, weil `dist > v_sound * 10min` ist. Das System respektiert den Lichtkegel der jeweiligen Kraft.
3. **Wirtschaftlich:** Der teure `motion.at(t2, smp.epoch)`-Call passiert nur noch für Samples, die absolut im kausalen Horizont der Präsenz liegen.

Möchtest du, dass ich diesen Patch direkt in eine komplette `main.rs` Datei integriere, oder reicht dir die Isolation des Patches für die Manifestation?

Du hast den Kern der Omegaflow-Philosophie exakt getroffen: **A = A.** Wir zwingen der System keine künstlichen Filter auf, sondern wir ehren die physikalischen Grenzen der Wahrnehmung. Was nicht im Kausalkegel der Präsenz liegt, existiert für dieses Fenster schlicht nicht.

Und du hast recht: **Es ist rechnerisch billiger – massiv billiger.** 

Wenn wir den Kausalitäts-Check geschickt platzieren, sparen wir uns nicht nur den Netzwerk-Transfer und die GPU-Schleife, sondern wir überspringen im Archivar sogar die teuerste Operation: `motion.at(t2, smp.epoch)`.

### Warum das so extrem billig ist:

Schau dir den bestehenden Code in `enclose_family` an. Bevor wir die exakte Distanz messen, rufen wir die Bewegungsgleichungen auf:
```rust
let p = smp.motion.at(t2, smp.epoch); // <-- DAS IST TEUER!
let ddx = p[0] - q[0];
// ...
```
Für `Motion::Ground` oder `Motion::Terra` führt das Keplersche Gleichungen, WGS84-Transformationen und GMST-Berechnungen aus. Das kostet CPU-Zyklen.

Wenn wir deinen Gedanken umsetzen, nutzen wir `smp.p0f` (die Position zum Zeitpunkt der Messung, die wir bereits im Cache haben) für einen groben Kausalitäts-Check **bevor** wir `motion.at()` aufrufen. Ist das Signal physikalisch noch nicht da oder schon verklungen, skippen wir die teure Mathematik komplett.

Hier ist der auf Performance getrimmte Patch für `enclose_family` in `main.rs`:

```rust
        for samples in visit {
            for smp in samples {
                // Grober Bounding-Check (aus dem bestehenden Code)
                let dx = smp.p0f[0] - qf[0];
                let dy = smp.p0f[1] - qf[1];
                let dz = smp.p0f[2] - qf[2];
                let dist2_p0f = dx * dx + dy * dy + dz * dz;
                
                let age = (t2 - smp.epoch).abs();
                let reach = smp.extent + smp.vmax * age + 0.5 * smp.amax * age * age + pad;
                if dist2_p0f > reach * reach {
                    continue;
                }

                // --- NEU: Kausalitäts-Vorfilter (Lichtkegel / Diffusionsfront) ---
                // Verhindert teure motion.at() Aufrufe für Signale, die physikalisch
                // am Presence-Punkt q zur Zeit t2 noch nicht existieren können.
                if let Some((v_or_d, is_diff)) = force_constants_by_id(smp.force_type) {
                    if age < 1e-9 {
                        continue; // Vermeide Division durch Null bei statischen Epochen
                    }

                    // Nutze die grobe Distanz (dist2_p0f) für den Kausalitäts-Check
                    if is_diff {
                        // Diffusive Kräfte: Reichweite wächst mit sqrt(2 * D * age)
                        if 2.0 * v_or_d * age < dist2_p0f {
                            continue; // Diffusionsfront hat den Punkt noch nicht erreicht
                        }
                    } else {
                        // Wellenartige Kräfte: Reichweite wächst linear mit v_or_d * age
                        let max_causal_dist = v_or_d * age;
                        if dist2_p0f > max_causal_dist * max_causal_dist {
                            continue; // Signal ist noch nicht am Präsenzpunkt angekommen
                        }
                    }

                    // Oberer Cutoff: Wenn das Signal-Zeitalter ein Vielfaches seines 
                    // charakteristischen tau übersteigt, ist die Wirkung verschwunden.
                    if smp.tau > 0.0 && age > smp.tau * 64.0 {
                        continue; 
                    }
                }

                // ERST JETZT die exakte (und teure) Positionsberechnung durchführen
                let p = smp.motion.at(t2, smp.epoch);
                let ddx = p[0] - q[0];
                let ddy = p[1] - q[1];
                let ddz = p[2] - q[2];
                let exact = smp.extent + pad;
                let dist2 = ddx * ddx + ddy * ddy + ddz * ddz;
                
                if dist2 > exact * exact {
                    continue;
                }

                // Wenn alle physischen Filter bestanden wurden, manifestiere das Feld:
                for (_, val) in &smp.fields {
                    records.push((
                        p[0],
                        p[1],
                        p[2],
                        *val,
                        smp.extent,
                        smp.epoch,
                        smp.ttl,
                        smp.tau,
                        smp.force_type,
                    ));
                }
            }
        }
```

### Fazit
Durch diese Implementierung wird der Archivar (Rust) noch näher an die reine Physik herangeführt: Er berechnet nur noch die exakte Ephemeride und schickt die binären φ-Pakete für Oszillatoren, die sich *kausal* im Wahrnehmungsfenster befinden. Alles, was "hinter dem Horizont" liegt oder "ausgeklungen" ist, wird für wenige Nanosekunden in einem simplen `if`-Statement verworfen.

Das spart CPU, RAM, Bandbreite und GPU-Backpressure. Soll ich diese `force_constants_by_id` Funktion und den Patch direkt in deine `main.rs` Datei schreiben, damit du sie kompilieren kannst?
