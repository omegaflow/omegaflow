# Handover: omegaflow — Hybrid, Zell-Achse und der Rest der Survey-Tafel

Selbsttragende Karte für eine frische Session. Stand: Branch `main`, HEAD `47475f3`.

## Erster Schritt der neuen Session (Arbeitsbaum aufräumen)

```bash
cd /home/johannes/projects/omegaflow
git status                      # uncommittete Gauß-Stern-Edits (verworfenes Experiment)
git checkout -- src/main.rs     # die verwerfen — sie werden durch Atom 1 ersetzt
cargo check                     # muss 0/0 sein
```

`cargo check` 0/0, 47 Tests grün (Stand HEAD).

**Lesepflicht zu Beginn** (die Karte): `docs/surveys/auswertung.md` (f32-Grundwahrheit,
Verteilungs-Verdikte, Generations-Konsens, Rejected-Register),
`docs/surveys/fortschritt.md` (Verzeichnis mit Hashes),
`docs/surveys/messpunkt-verteilung.md`, `AGENTS.md`, `TODO.md`.

**Sicherungs-Branches:** `session-2026-08-16` (Generation + Kulling + Budget +
HUD-Akkumulatoren + deep_gain als Code), `backup-7d0ac9f`, `parallel-welle-2026-08-16`.

---

## Atom 1 — Hybrid-Rendering: Membran + Quellen-Punkt-Layer (der aktuelle Rat-Plan)

**Ziel:** Die zwei Wahrheiten des Blocks zugleich. (1) Die **Membran** = das
kontinuierliche Feld Ω(x) = Σ val_eff·K — bleibt. (2) Der **Punkt-Layer** =
je Oszillator ein kompakter Gauß-Punkt mit τ-Farbton, |val|-Helligkeit,
e^(−r²/2)-Transparenz.

**Warum nicht die Rose-Formel:** `0.02/(r²+0.02)` ist der 1/d²-Kernel selbst —
sie würde das Feld doppelt zählen. Der Punkt zeigt das **Wesen** (kompakter
Marker), die Membran das **Geflecht**.

**Schritte:**

1. **WGSL** (in `MEMBRANE_WGSL`): `near_pt_vs`/`near_pt_fs`:
   - Quad ±2ⁿ px, Falloff **e^(−r²/2)** (die exponentielle Relaxation selbst,
     σ = 1), α = Falloff.
   - Größe `clamp(extent/scale, 0.5, 16)` — **Zweierpotenzen**, abgeleitet aus
     der gemessenen Ausdehnung, KEIN Hand-Regler.
   - **τ-Farbton**: `hue = fract(log2(τ)/16)`, `hsl_to_rgb(hue, 1.0, lum)` —
     volles Farbspektrum aus der Zeitlichkeit.
   - `lum = clamp(log2(|val_eff|/referenz) / 8 + 0.5, 0, 1)` gegen die
     Pro-Kraft-Referenz.
2. **Rust**: Pipeline + Bind-Group (VP + `field` + `props`) +
   `pass.draw(n·6)` **nach** der Membran, additiv (`one/one`).
3. **Pro-Kraft-Exposure** (ersetzt das hartkodierte `+14`/`+18`): 8 Kanäle —
   bei res-Empfang den **Median von |val| je Kraft** bestimmen,
   **exponentiell relaxieren**, in die freien VP-Slots (`expose_lo`/`expose_hi`).
   Tasten `e`/`E`: **`E` = ×2 (ein Stopp rauf), `Shift+E` = ÷Φ (feiner Trim
   runter)** — die Rückkehr Richtung Referenz übernimmt die Relaxation, nicht
   die Tastatur. Default-Offset 4.
4. **Punkte/Feld-Regler**: neuer Uniform-Slot, Blend des Punkt-Layers
   (0 = nur Feld, 1 = beides, 2 = nur Quellen), Taste `P` in Φ/2ⁿ-Schritten,
   **Default 1**.
5. **Konstanten-Säuberung**: alle Werte aus Φ, 2ⁿ, c, Pogson-Faktor
   (0,2/Magnitude) hergeleitet — keine willkürlichen Zahlen. Sterngröße
   `exp2(4 − 0.2·|val|)` behalten (4 = log2(16 px), 0,2 = Pogson in 2ⁿ).
   ssaa bleibt Φ (Q).
6. **Subpixel: NICHT anfassen** — Rat-Urteil + Survey-Widerlegung
   (`auswertung.md`: „Subpixel-Emitter — durch die eigene 3×-Messung
   widerlegt"). Ehrliche minimale Einheit = Pixel; Auflösung = `ssaa`.
7. **Verifikation**: `cargo check` 0/0, `cargo test` grün, Screenshot +
   Pixel-Analyse (`xwd` + `ffmpeg -f rawvideo -pix_fmt rgb24` + python:
   Komponenten zählen, Radial-Profil eines Punktes prüfen — weicher
   e^(−r²/2)-Schwanz, kein hartes Quadrat). Commit + Push, TODO im selben
   Commit.

---

## Atom 2 — Zell-Achse (der Messpunkt-Hebel, endlich gebaut)

**Warum:** Die Per-Pixel-Membran kostet ~130 ms (116 Quellen × 2 Mio Pixel) —
der Grund für alle Leistungsprobleme. Die Zell-Achse vergröbert die
Messpunkte ehrlich gegen die Display-Quantisierung (auswertung.md §1–2).

**Design (aus der Survey):**
- **Struktur-Zellen** (1×1 px) um jede Quelle im Struktur-Radius
  (~14–39 px bei 8-Bit-Sensor) — dort bleibt das Feld scharf.
- **Hintergrund-Zellen** (2^k × 2^k px) im glatten Feld — dort wird nur je
  Zelle ausgewertet.
- Die **Archivarin** kennt jede Quelle und kann sagen, WO gemessen werden
  muss — sie liefert den Zell-Plan; die Mathematikerin wertet pro Zelle aus.
- **Keine Interpolation** (stückweise-konstante Zellen = ehrliche Messung).
- **Budget**: zwei Zahlen (Frame-Zeit + Messpunktzahl) → Zellweite,
  exponentielle Relaxation.
- Ziel: Membran von ~130 ms auf ein Budget, das danach **VR-Stereo erlaubt**
  (Atom 6).

**Reihenfolge:** erst Atom 1 fertig + gepusht, dann Zell-Achse als eigener,
kompletter Session-Atom (Rust-Zell-Plan + Compute-Pass + Anzeige).

---

## Atom 3 — Generations-Architektur + Stille (Port von `session-2026-08-16`)

Der Survey-Konsens: **„Der Buffer ist die Nachricht. Der Zähler ist das
Protokoll. Die Stille ist die Optimierung."** Der Code liegt fertig auf dem
Branch — portieren:
- **Stille-Gate**: Arbeiter vergleicht die gepackten f32-Bytes mit dem
  letzten Stand — identisch → kein Senden (memcmp der Wahrheit selbst).
- **u64-Generationszähler** in der res-Nachricht; Upload nur bei Wechsel.
- **Doppelpuffer A/B** mit Bind-Group-Swap gegen Torn Reads.
- **HUD-Ω + Fenster-Zentroid aus Archivarin-f64-Akkumulatoren** (`hud_probe`:
  dokumentierte Gesetzesform + 7 Kernel in f64 + erfc A&S) —
  `mapAsync`/`probe_read`/`centroid_pipe`/`field_centroid` getilgt (der
  Map-Timeout unter Last verschwindet strukturell).
- **Allostase als Unterlassung**: der Archivar schreibt nicht, wenn das Feld
  ruht (0 honored).

---

## Atom 4 — Tiled Source Culling + Budget-Regler (Port von `session-2026-08-16`)

- **Kull-Pass** (Compute, Kacheln 16×16 = 2⁴): Shared-Memory-Max-Reduktion,
  **KEINE globalen/Fragment-Atomics** (harter Live-Befund: Fragment-Atomics
  verloren das Device auf ANV/HD 520), Kachel-Listen als plain u32
  (`tile_flat`/`tile_count`, 64 Slots), Überlauf → ehrlicher Voll-Loop.
- **Kriterium** `bound < ε·M`, ε = 2⁻ⁿ, **n = 23 Grund** (f32-Wahrheit),
  Relaxation bis n = 8 unter Budgetdruck (Display-Wahrheit — ausgesprochene
  Messpolitik).
- **Kull bei Sense-Kadenz** (die Listen sind so frisch wie das Feld, das sie
  gattern).
- **Budget-Regler**: Frame-Zeit-EMA ÷4 gegen die Display-Periode 16,6 ms;
  n±1 (n±2 bei Überlauf/Schwerlast), n ∈ [8, 23].

---

## Atom 5 — deep_gain, Deep-Upload-Stille, Relay-Trailer

- **deep_gain**: der Stern-Weißpunkt normalisiert (8 − log2 max|flux| des
  gelieferten Satzes, ÷4-Relaxation) — ersetzt jeden hartkodierten
  Stern-Offset. Fährt im freien VP-Slot.
- **Deep-Upload-Stille**: `deep_dirty` feuert heute bei jedem Sense — die
  gepackten Deep-Bytes vergleichen und nur bei Änderung hochladen.
- **Relay-Trailer**: gen u64 + 9×Ω f64 (~80 B) für `browser_relay`;
  `constants.js`-Parser um den Trailer erweitern.

---

## Atom 6 — VR/OpenXR (Quest) — nach der Zell-Achse

- `vr = ["dep:openxr"]` als Feature-Gate (wie `gamepad`), Session öffnen,
  **Kopf-Pose = Blick-Quaternion** (die Präsenz bleibt die freie Weltlinie —
  der Kopf ist nur ihr Blick), **Stereo-Deep zuerst** (billig), Membran-Stereo
  erst, wenn die Zell-Achse das Budget trägt. Controller analog Gamepad.

---

## Rejected (nicht anfassen — dokumentiert)

Subpixel-Emitter (Survey-Widerlegung), Fovea als Primärverteilung, Ω-Token,
Feld-Hash, Efferenzkopie, gelernte Platzierung, hex/jitter/quasi-random,
Meta-Oszillator-LOD, CVT, Sparse Grids, 1-Hz-Heartbeat.
Siehe `auswertung.md` §5 + TODO-Rejected.

---

**Reihenfolge der Umsetzung:** Atom 1 (diese Session) → Atom 2 → Atom 5 →
Atom 3 + 4 → Atom 6. Jeder Atom = eine vollständige Session mit Verifikation.
