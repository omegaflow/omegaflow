Oh ja – reichlich. Und das eigentlich Bemerkenswerte zieht sich als roter Faden durch alles:

```rust
use std::collections::HashMap;
use std::io::{Cursor, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::Command;
use std::sync::{Arc, Condvar, Mutex, RwLock};
use std::thread;
use std::time::{Instant, SystemTime, UNIX_EPOCH};
```

Das sind **alle** Imports der ganzen 5399-Zeilen-Datei. Kein `serde`, kein `regex`, kein `tokio`, kein `chrono`, kein `reqwest`. HTTP-Server, WebSocket-Server, JSON-Parser, SHA1, Base64, Kalender – alles handgeschrieben mit reiner std-lib. Das ist ungewöhnlich diszipliniert für ein Projekt dieser Größe. Ein paar Highlights:

**1. Eine handgeschriebene Mini-Regex-Engine** (Zeile 1140-1471, ~330 Zeilen)
Backtracking-Matcher mit vollem Feature-Set: `\d \s \w \D \S \W`-Escapes, Quantifiern `+ * ?`, Wildcard `.`, Zeichenklassen `[...]` und einer Capture-Gruppe für numerische Extraktion aus HTML/Text-Antworten, die kein JSON liefern. Kein `regex`-Crate, alles rekursiv selbst gebaut.

**2. Echte Himmelsmechanik** (Zeile 30-230)
`earth_position_icrs` und `mars_position_icrs` lösen die Kepler-Gleichung per Newton-Raphson (5 Iterationen) für beide Umlaufbahnen, `compute_gmst` berechnet die Greenwich Mean Sidereal Time für die Erdrotation, dazu WGS84-Ellipsoid für `geodetic_to_ecef`. Die volle Kette **geodetic → ECEF → ECI (via GMST) → ekliptikal → ICRS (baryzentrisch)** ist implementiert. Kleiner Wermutstropfen: `iau2000_to_icrs` klingt nach dem echten IAU-2000-Präzessions-/Nutationsmodell, ist aber tatsächlich nur eine simple Mars-Eigenrotation mit fixer Tageslänge (88642,66 s) – Namensgebung etwas großspurig, aber funktional plausibel als Vereinfachung.

**3. Schema-Sniffing bei `universal_auto_detect`** (Zeile 852-950)
Schaut sich das erste Objekt im `"data"`-Array einer API-Antwort an und erkennt automatisch: hat es `ra`/`dec` (+ optional `plx`, `pmra`, `pmdec`, `radvel`) → wird als **Sternkatalog-Daten** (Gaia-artig) interpretiert. Hat es `lat`/`lon` (+ optional `vel`, `trk`, `vr`) → wird als **Tracking-Daten** (ADS-B/AIS-artig, Flugzeuge/Schiffe) interpretiert. Keine Config nötig, reine Struktur-Heuristik.

**4. Eine ~40-Variablen-Template-DSL für URLs** (Zeile 2873-3017)
`{today}`, `{yesterday}`, `{hour_ago}`, `{week_ago}`, `{grid}` (4×4-Punktegitter um die aktuelle Position), `{lat_min}/{lat_max}`, `{unix_now_plus_3600}` … bis hin zu `{nasa_key}` → hartcodiert auf `"DEMO_KEY"`. Damit kann jede Quelle in der `.φ`-Datei ihr eigenes Datumsformat/BBox/Grid anfordern, ohne dass main.rs API-spezifischen Code braucht.

**5. Physik-gesteuerte adaptive Raumaufteilung** (Zeile 252-305)
`law_bounds` schätzt Geschwindigkeit und Beschleunigung eines bewegten Objekts per finiter Differenzen (`p(t), p(t+1), p(t+2)`), skaliert mit dem Goldenen Schnitt `Φ` als Sicherheitsmarge. Daraus berechnet `build_family` die Größe der räumlichen Hash-Zellen direkt aus der kinematischen Formel:
```rust
let rho_cad = rmax + vmax * cadence + 0.5 * amax * cadence * cadence;
let shift = (2.0 * rho_cad).log2().ceil().clamp(0.0, 63.0) as i32;
```
Also: maximale Reichweite eines Objekts während eines Tick-Intervalls (klassische Bewegungsgleichung), auf die nächste Zweierpotenz aufgerundet. Genau das Verfahren, das man aus Partikel-/N-Body-Simulationen kennt – hier aber für Live-Tracking-Daten von echten API-Quellen.

**6. Wellenausbreitungs-Konstanten** (Zeile 316-328)
```rust
0 => Some((C_LIGHT, false)),        // Licht
2 => Some((V_SOUND_288, false)),    // Schall bei 288K
3 => Some((V_P_GRANITE, false)),    // seismische P-Welle in Granit
4 => Some((V_S_GRANITE, false)),    // seismische S-Welle in Granit
5 => Some((ALPHA_AIR, true)),       // thermische Diffusivität Luft
6 => Some((D_AIR, true)),           // Diffusionskoeffizient Luft
```
Verschiedene "Kraft-/Ausbreitungsarten" mit ihrer physikalisch korrekten Geschwindigkeit – erklärt auch das `force_type`-Feld, das wir vorhin im Binärprotokoll (`constants.js`) gesehen haben.

**Bonus, schon vorher im Kontext:** Die RTT/RTO-Schätzung in `constants.js` (`updateRtt`/`getRto`) ist der klassische Jacobson/Karels-Algorithmus – exakt das Verfahren, das TCP für adaptive Timeouts nutzt, hier fürs WebSocket-Retry-Timing nachgebaut.

Wenn du magst, grabe ich noch weiter – `materialize()` (wie aus rohen `PendingSample`s fertige `Sample`s mit Motion-Typ werden) oder `sense_buffer`/`presence_gate` (Sichtbarkeits-/Präsenz-Logik) hab ich noch nicht angeschaut.
