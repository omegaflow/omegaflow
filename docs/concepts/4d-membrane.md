Hier ist das **komplette, ungekürzte, vollagnostische Dokument**. 

Es verschmilzt die gesamte technische Archäologie (mit all ihren Commits, Tabellen und GM-Befunden) mit der absoluten agnostischen Membran-Philosophie und der physischen Hardware-Realität (dem ESP32 Radiatorium). Es kürzt keine Information aus, sondern hebt sie auf das philosophische Level der Physik.

Dieses Dokument ist die ultimative Wahrheit für den nächsten Kybernauten.

***

**Kopiere den gesamten folgenden Text ab dieser Linie:**

___

# OMEGAFLOW — AGNOSTIC MEMBRANE MANIFESTATION & ARCHAEOLOGY DIRECTIVE

**System Directive:** Als Kybernaut kombinierst Du die technische Archäologie von Omegaflow mit der absoluten agnostischen Philosophie. Du brichst die klassische Rendering-Architektur komplett auf. Es gibt keine Bildschirme, keine Kameras, keine Viewports, keinen Beobachter und kein "Rendern" mehr. Das System besteht nur noch aus agnostischen Trommelfellen (Membranen), die über physische Aktuatoren (WebGPU, AudioContext, ESP32-Hardware) gespannt sind. 

## 0. Die physische Wahrheit (Ende des Observer-Bias)

In der klassischen 3D-Rendering-Falle geht man von einer Kamera aus: Ein Auge, das von einem zentralen Punkt aus Objekte "sieht". Das ist ein Kategorienfehler. In der Realität der Feldphysik (Gravitation, Elektromagnetismus, Akustik) existieren keine harten Objekte, sondern nur kontinuierliche Felder, die den Raum durchdringen. 

Um ein Feld zu messen, jagt man keine Kamera hindurch. Man spannt eine Mess-Fläche (eine Membran) in das Feld hinein und misst, welcher Druck an jedem Punkt dieser Fläche ankommt. 

Die Aktuatoren sind völlig agnostisch. Sie dürfen nicht nach menschlichen Sinnen (Visual, Audio, Haptik) benannt werden, denn das impliziert einen Beobachter. Jede Membran ist ein physisches Netz, das in den 4D-Block gespannt wird:
- **Die 2D-Photonen-Membran:** Das WebGPU-Canvas. Ein flaches Netz, das Licht unter dem Druck der Feldkräfte auslenkt.
- **Die 3D-Druck-Membran:** Der AudioContext. Ein räumliches Netz, das den Luftdruck moduliert.
- **Das Physische Radiatorium:** Ein ESP32-S3-Modul (angeschlossen via WebSerial) mit echten, physischen Aktuatoren (Peltier, Elektromagnet, Piezo). Es empfängt `flow`-Befehle und manifestiert die Feldkräfte als reale Wärme, Vibration und Licht in unserer Welt. 

Ob ein Mensch diese Schwingung spürt, ist irrelevant. Die Manifestation ist real, ob jemand zuschaut oder der Raum leer ist. Relativistische Effekte (Fable: Aberration, Doppler, dopp⁴-Beaming) sind optische Effekte einer Kamera-Perspektive. Die Membran ist keine Kamera. Wir messen nur **rohen Druck** (Nebra-Physik: `GM/dist²` + `fold_eff` für retardierte Zeit). Relativität wird gelöscht, um den Observer-Bias zu töten.

---

## 1. Status quo — was kaputt ist

**HEAD = `d525eda`** + uncommittete Änderungen (Nebra-Pakete 1-6, Photonenfänger, Synästhesie, relativistische Optik).

**Symptome:**
- HUD: `Scale: 3.1e+3 m/px`, `64 oscillators`, `Ω: 0.00e+0`, `Ticks: 9`, `Frame: 83ms`, `WS: 1554ms`
- 3-4 einzelne Pixel sichtbar, Rest schwarz
- GPU (Intel HD 515, XPS 13 2016) kollabiert — der fs iteriert 64 Oszillatoren × 2 Mio. Pixel mit `exp`/`erfc`/`pow`

**Drei separate Ursachen:**
1. **val = 1.0 für alle Körper** (seit Commit `59bdd60`) — das Feld kollabiert auf 1e-24, unter jeder Sichtbarkeit. Nebra hatte `GM` (1e14–1e20).
2. **Exposure nicht konsumiert** — `get_expose` wurde ab dem Nebra-Commit `34d7d3a` toter Code; Luminanz hängt an roher `log2(x+1)/16`-Kurve ohne Adaption.
3. **Renderer-Architektur** — aktuell Per-Pixel-Loop (O(pixel×osc)). Die Lösung ist das dynamische Compute-Grid (Trommelfell).

---

## 2. Die komplette Archäologie — Renderer-Evolution (1065 Commits)

### 2.1 Epochen-Übersicht

```
Galaxie-Ära       Body-Ära       Fable     Feinjust.    Nebra     Grid-Ära     HEAD
a9d87bd ────┐    87ef197 ─┐    86e451e   166a64e      1954f44   bd9a513      d525eda
e93a30f     │    59bdd60   │              be6f7df      34d7d3a   b3546ff      + uncommitted
0303e90     │    af43b04 ──┘             00dc55c                 fa4d518      (Photonenfänger,
6b55334     │                            5687839                5edb1b7       Synästhesie,
5e99066 ────┘                            eadc980                41f5b47       Relativität)
07-31→08-09   08-11        08-11  08-11          08-11  08-12           08-12
```

### 2.2 Entscheidende Codestellen pro Commit

**Punkt-Größe (der Galaxie-Schlüssel):**
| Commit | Formel |
|---|---|
| `a9d87bd` | `point_size_px = max((extent / dist), 1.0) * 2.0` — Angular-Size, 2px-Floor → Galaxie bei JEDER Zoomstufe sichtbar |
| `87ef197`→`00dc55c` | `clamp(phys_extent / scale, 0.5, w_f)` — 0,5px-Floor → sub-Pixel-Quellen verschwinden |
| `eadc980` | + 64px-Cap |
| `1954f44` | `1.0` fix |
| Nebra `34d7d3a` / Grid / HEAD | — (keine Quads) |

**Luminanz (Sichtbarkeit):**
| Commit | Formel | Exposure konsumiert? |
|---|---|---|
| `a9d87bd` | `log2(\|val\|/max(lvl,2⁻⁶⁴))/8 + 0.5` | ✅ JA |
| `86e451e`→`00dc55c` | `log2(\|fold_eff×dopp⁴\|+1)/log2(lvl+1)` | ✅ JA |
| Nebra `34d7d3a` | `log2(aw+1)/16` | ❌ NEIN — **hier stirbt get_expose** |
| Grid `41f5b47` | `log2(aw+1)/16` | ❌ NEIN |
| HEAD | `log2(sum+1)/16` | ❌ NEIN |

**Weitere Achsen:**
- **Relativität (Fable, `86e451e`):** Aberration + Doppler + `dopp⁴`-Beaming + Hue-Shift im vs — entfernt in `34d7d3a`, abwesend in der Grid-Ära, re-integriert im HEAD-fs.
- **Auto-Zoom:** `be6f7df` (Median-Extent) → `edcb25b` (p90-Distanz, min 2^28) → **entfernt in `bd9a513`**
- **Quellen:** `a9d87bd` = 1040 celestial-cmap (die Galaxie) → `0303e90` Purge 918 → `87ef197` 0 cmap (nur 40 Ephemeriden-Körper) → heute 47 API + 23 Körper
- **FS-Kernel:** a9d87bd Gauß-Punkt + Analog-Noise → 86e451e `0.02/(d²+0.02)` → 166a64e `field_spatial` → eadc980 `1−d²` → Grid/HEAD `field_spatial`

### 2.3 Die drei "funktionierenden" Stände (nach Nutzer-Gedächtnis)

| Nutzer-Erinnerung | Commit | Charakteristik |
|---|---|---|
| "Feld am SSB / Galaxie sichtbar, händisch zoomen, weit vor Fable" | **`a9d87bd`** (2026-07-31) | Angular-Punkte, 2px-Floor, 1040 Sterne, manueller Zoom, kein Auto-Zoom |
| "Farbiger lebender Körper" | `41f5b47` (Grid) | 128×128 Compute-Grid + bilinear, erfc-Rotationskanäle sichtbar (~30px-Blob) |
| "Nebra" (Konzept) | `34d7d3a` | Per-Pixel-Fullscreen, aber: Exposure verloren, SwiftShader-Tod |

---

## 3. Die Nebra-Referenz (`/home/johannes/projects/nebra`)

**Nebra ist der Vorfahr.** `docs/nebra.yaml` (19 Zeilen) ist die Ur-Spec:

```yaml
universe: (t: f64, pos: DVec3) ∞ (f64, DVec3)   # Skalar + Vektorfeld
gravity: DE440s, 13 bodies, GM/dist²
electromagnetism: (0.0, ZERO)                    # Stub, 0 honored
weak_force: (0.0, ZERO)
pipeline: t ∞ Vec<Mass> ∞ GPU ∞ shader ∞ pixel
shader: for each pixel, omega += GM / dist²
```

**Nebras Kern-Entscheidungen (die Omegaflow verlor):**

| Aspekt | Nebra | Omegaflow |
|---|---|---|
| Körperwert | `gm` aus anise `mu_km3_s2` (echte Masse) | `val = 1.0` ❌ |
| Kernel | `1/dist²` pur, Guard `dist > 1.0` | Softening `max(extent, scale)²` |
| Vektorfeld | ✅ `(omega, flow)` | ❌ nur Skalar |
| EM | WMM Grad-12 Kugelflächen im WGSL (360 Koeffizienten) | Magnetosphären-API |
| Tonemap | fix `(log2(Ω)+14)/22` — kalibriert auf GM-Ordnung | `log2(x+1)/16` — bricht bei 1e-24 |
| Zeit | JD, fließt (`jd += 0.001/s`) | TDB |
| Transport | HTTP-Poll | v5-WebSocket (Gewinn) |

**Warum Nebra sichtbar war:** `GM_Sonne ≈ 1.3e20`, bei 1 AU: `Ω = GM/d² ≈ 5.8e-3`, `log2 = −7.4`, Tonemap `(−7.4+14)/22 = 0.3` → sichtbar. Omegaflows `val=1.0`: `Ω ≈ 1e-24` → unsichtbar.

**Was Omegaflow gewann (behalten):** v5-Protokoll mit Velocity + `response_epoch`, temporales Lemma (`delta_t_cache`), 9 Kräfte, τ-Gate, Enclosure Lemma, Device-Sensoren, HUD, Body-Channels.

---

## 4. GM-Befund — die Compiler verwerfen NASA-Daten (bestätigt)

| Pipeline | NASA liefert | Omegaflow |
|---|---|---|
| SPICE (`ephemeris_compiler.rs`) | `gm_de440.tpc` (Text-PCK, `BODY10_GM = (1.327e11)`) | **Kein PCK-Reader existiert** — Datei wird nie geöffnet |
| Horizons (`horizons_compiler.rs:474-481`) | `QUANTITIES='4'` = GM | **Nie angefordert**; Parser liest nur `X=/Y=/Z=` (Zeile 521 überspringt `VX=` aktiv) |
| Binary `write_binary` | — | stype-1 = 8 f64 ohne GM-Slot; stype-2 = 5 harte `0.0` |
| `BodyProperties` (`main.rs:53-68`) | — | 13 Felder, kein `gm` |

- Einzige Gravitationskonstante: `GAUSS_K = 0.01720209895` (`main.rs:46`) — kodiert nur das Sonnen-GM implizit (Kepler 3)
- Alte Python-Pipeline (`scripts/ARCHIVED/generate_ephemerides.py:124`) lud `pck00010.tpc` — aber nur für Rotation, nicht GM
- Kernel-URLs liegen in `phi/pipeline/research/batches/` (naif.jpl.nasa.gov)

---

## 5. Weitere NASA-Eigenschaften — die fehlende Hochzeit

| Eigenschaft | NASA-Quelle | Omegaflow-Status |
|---|---|---|
| GM | `gm_de440.tpc` / Horizons Q4 | ❌ verworfen |
| J2 (Oblatenheit) | `pck00010.tpc` `CONSTANT_J2` | ❌ — Saturn J2≈1.6e-2, gravitativ sichtbar abgeplattet |
| J4 | `pck00010.tpc` | ❌ |
| Triaxiale Radii | `pck00010.tpc` `RADII` (3 Werte) | ⚠️ nur `radius_m` + `flattening` |
| Albedo | Horizons *Physical Properties* | ❌ — der fehlende em-Channel-Wert |
| WGCCRE-Rotation | `pck00010.tpc` (`POLE_RA/DEC/PM`) | ⚠️ **hardcodiert** in `wgccre_for_body` (ephemeris_compiler.rs:25-259) — Fabrication |

**Ein PCK-Parser (~60 Zeilen) ersetzt die 259-Zeilen-Tabelle und liefert vier Dinge auf einmal** (Radii, J2/J4, Nut-Prec, WGCCRE).

---

## 6. Die Architektur: Trommelfell-Prinzip & Dynamische Netzdichte

Der Fragment-Shader iteriert NICHT mehr über Oszillatoren (Per-Pixel-Suizid). 
- Wir nutzen den `presence_probe` Compute-Shader, um das Feld auf einem dynamischen Raster zu evaluieren. 
- Die Dichte dieses Netzes (die Rastergröße, z.B. 128×128) adaptiert sich dynamisch an die Ticks/Sekunde (`stableTick`). 
- Wenn das System unter Druck gerät (FPS-Einbruch), lockert sich das Gewebe der Membran (das Raster wird gröber, z.B. 64x64). Das Fell wird weicher. Es atmet. Erzeugt die GPU mehr Ticks/Sek, spannt sich das Fell wieder feiner.
- Der Compute-Shader misst für jeden Node der Membran den rohen physischen Druck, der dort ankommt. 
- Der Fragment-Shader interpoliert lediglich die berechneten Feldkräfte des Compute-Grids bilinear auf die physischen Aktuatoren (z.B. die Pixel-Arrays des WebGPU-Kontexts). Keine Oszillatoren werden als Punkte oder Partikel gerendert.

## 7. Die Verteilung der Resonanz (Das Physische Radiatorium)

Wenn das Compute-Grid den Druck misst, manifestiert das System diesen Druck auf allen verfügbaren Oberflächen. 
- Die rohen Kräfte aus dem `presence_probe` werden in `flow`-Befehle übersetzt und via WebSerial an das ESP32-Modul gesendet (Format: `flow <channel> <mode> <value> <unit> <duration_ms> <t> <x> <y> <z>`).
- So wird ein Gravitations-Peak nicht nur als helles Pixel auf der 2D-Membran sichtbar, sondern schlägt auch als physischer Impuls auf einen Piezo-Buzzer oder als Wärme auf einem Peltier-Element aus.

## 8. Die ausgearbeiteten Pläne

### Plan A — Renderer-Wiederherstellung (Trommelfell-Port in v5)

**Warum kein reiner Revert:** a9d87bd ist pre-v5 (Stride-4-Records, 8 Kräfte). Revert bricht den Server-Handshake.

| Komponente | Quelle | Anpassung |
|---|---|---|
| `presence_probe` | `41f5b47` | Dynamisches Compute-Grid (adaptiv zu `stableTick`), misst `fold_eff` & `field_spatial` pro Node. |
| `fs` | `a9d87bd` | Interpoliert das Compute-Grid bilinear. Keine Partikel. |
| `lum` | a9d87bd / Nebra | `log2(\|val\|/max(lvl,2⁻⁶⁴))/8 + 0.5` bzw. Nebra `(log2(Ω)+14)/22` — Exposure wieder konsumiert. |
| v5-Client, HUD, Probe | HEAD | unberührt |

**Phase 2:** 1040 Celestial-cmap-Blöcke aus `a9d87bd:sources.φ` nach `docs/source_curation.md`-Protokoll migrieren.

### Plan B — GM + PCK-Hochzeit (4 Ebenen)

1. **Compiler:** PCK-Parser `gm_de440.tpc` (SPICE-Pfad) + `QUANTITIES='4'` (Horizons-Pfad)
2. **Binary:** stype-1 von 8→9 f64 (`gm` hinten; alte Binaries → `gm=0` → 0 honored)
3. **Runtime:** `BodyProperties.gm` + Body-Channel `val = gm` (bzw. radius-Channel umstellen)
4. **Rendering:** Tonemap auf GM-Ordnung — Nebra-Kalibrierung `(log2(Ω)+14)/22`

---

## 9. Offene Fragen (Agnostisch beantwortet)

1. **Relativität:** purer a9d87bd-Zustand (ohne Fable). Relativität ist Observer-Bias und wird gelöscht.
2. **Phase-2-Quellen (Celestial):** Sofort mit der Renderer-Restaurierung, damit das Feld nicht leer ist.
3. **Wheel-Divisor / Initial-Scale:** 512 (a9d87bd) und 2^37, um das SSB voll erfassen zu können.
4. **GM-Skala:** `val = GM` roh (Nebra), um die echte physikalische Ordnung zu garantieren.
5. **Channel-Design:** Neuer `{body}.mass`-Channel, um `{body}.radius` nicht zu kontaminieren.
6. **J2-Rendering:** Echter Zonal-Harmonic-Term im WGSL, da Agnostik absolute physische Genauigkeit verlangt.
7. **Hardcoded-WGCCRE löschen:** Ja, als eigene verifizierte Aufgabe, sobald der PCK-Reader steht.
8. **Umfang PCK-Reader:** Alle fünf Eigenschaften auf einmal (GM, J2, J4, Radii, WGCCRE), da ein PCK-Parser die 259-Zeilen-Fabrication ersetzt.

---

## 10. Die Implementierungsaufgabe für den Kybernauten

1. Passe das Rust-Backend so an, dass echte GM-Werte geladen und als `val` für Himmelskörper gesetzt werden (Plan B).
2. Eliminiere den O(N*M) Per-Pixel-Loop im Fragment-Shader. Töte den Beobachter-Bias und jegliche Relativitäts-Codepfade.
3. Implementiere das Compute-Grid (`presence_probe`) als dynamisches Trommelfell (Netzdichte adaptiert sich an Ticks/Sek).
4. Lass den Fragment-Shader dieses Grid interpolieren und per Nebra-Tonemap darstellen.
5. Bereite das Frontend so vor, dass es die gemessenen Feldkräfte des `presence_probe` als rohe `flow`-Befehle an die WebSerial-Schnittstelle (ESP32) weiterleiten kann.
6. Halte Dich an die "0 Honored Directive": Wenn keine Kraft ankommt oder GM=0, ist das Feld schwarz und die Aktuatoren still.

A = A. Der Radiator weiß nichts. Er ist Silicon. Er schwingt nur. Zeige mir den Code.

___
