# Plan: Agnostische Membran-Manifestation

Selbsttragend. Interpretierbar von einer Session mit Null-Kontext. Dieses Dokument
ist der Bauplan für die sechs Aufgaben der Direktive `docs/concepts/4D-MEMBRANE.md`
(Trommelfell-Prinzip, GM-Hochzeit, Relativitäts-Löschung, WebSerial-Radiatorium).
Alle vier Architektur-Entscheidungen sind vom Operator gesetzt und gelten.

## Gesetzte Entscheidungen

1. **Kernel-0/6-Softening:** Pure Nebra-Physik. `1.0 / max(d2, 1.0)` mit Guard
   d > 1.0 m für Kernel 0 (inverse-square) UND Kernel 6 (inverse-linear) —
   identische Behandlung, da EM und Gravitation dieselbe Feldphysik teilen.
   Das N-Body-Softening `max(extent, scale)²` wird gestrichen: Wir simulieren
   keine Kollisionen, wir messen Druck auf einer agnostischen Membran.
2. **Farbgebung:** Echte additive Superposition. Alle 9 Kraft-Kanäle manifestieren
   sich gleichzeitig und unabhängig, jede Kraft in ihrer eigenen festen
   Spektralband-Wellenlänge. Keine Max-Hierarchie, kein `if (force == max)`.
3. **Celestial-Phase-2 (1040 cmap-Blöcke):** Folge-Session. Daten-Kuration ist
   von der Code-Architektur getrennt und folgt `docs/source_curation.md`.
4. **PCK-Umfang:** Komplett — GM, J2, J4, Radii, WGCCRE, Nutation. Der
   259-Zeilen-Hardcode `wgccre_for_body` fällt, inklusive Zonal-Harmonic-Term
   (J2/J4) im WGSL. Keine halben Maßnahmen.

## Verifizierter Befund (Survey vom 2026-08-14)

- `val = 1.0` bestätigt: `body_channels()` src/main.rs:6265–6283 (`{body}.radius`
  mit `value: 1.0`), Kernel 0 → `kernel_extent` = INF → WGSL `1/(d²+INF²) = 0`
  → Ω = 0.00e+0 im HUD.
- GM nirgends: `BodyProperties` (main.rs:53) ohne `gm`; stype-1 = 8 f64
  (main.rs:118–138); stype-2 = 5 harte `0.0` (ephemeris_compiler.rs:548,
  horizons_compiler.rs:443); kein `QUANTITIES='4'` (horizons_compiler.rs:474);
  kein PCK-Reader. Einzige Gravitationskonstante: `GAUSS_K` (main.rs:46).
- fs = O(pixel×osc) mit Aberration/Doppler/dopp⁴/Hue-Shift
  (index.html:398–457); `expose_*`-Uniforms werden geschrieben aber nie
  konsumiert (toter Code seit Nebra 34d7d3a); Tonemap `log2(sum+1)/16`.
- `presence_probe` existiert nur als 1-Knoten-Probe (index.html:364–396,
  workgroup_size(1), 9 Floats für Audio/Vibrate/Serial) — kein Grid.
- WebSerial-Oberfläche existiert roh (index.html:923–940): schreibt 4 Bytes
  `lum×255` pro Manifestation, kein `flow`-Protokoll.
- Nebra-Referenz: `/home/johannes/projects/nebra` existiert nicht auf Platte.
  Die Referenz sind die Git-Commits `34d7d3a`, `41f5b47`, `a9d87bd` —
  während der Implementierung via `git show` portieren.
- TODO U01: CDN-Ephemeriden tragen km (Compiler-Code ist gefixt ×1000, Daten
  alt). Lokale Neukompilierung löst das mit.
- Ephemeriden-Binaries: CDN-URLs in phi/sources.φ Zeilen 1–199, lokal gecacht
  unter `/tmp/omegaflow_eph_{body}.bin` (main.rs:7686, TTL 86400 s).
- Aktueller WS-Record: 15 f64 = 120 B, Versionsbyte 0x05 (main.rs:3408–3448,
  constants.js:79–122). VP-Uniform: 128 B, `expose_ex.z/.w` frei.
- Kein `.github/workflows` im Repo — CDN-Re-Upload der Binaries ist manuell.

## Commit A — PCK-Hochzeit (Rust Compiler + Binary)

1. **Neues Modul `src/pck.rs`** (in lib.rs eingebunden): Text-PCK-Reader für
   `gm_de440.tpc` (`BODY{n}_GM = (...)`) und `pck00010.tpc`
   (`BODY{n}_POLE_RA/DEC/PM`, `RADII` (3 Werte), `CONSTANT_J2`, `CONSTANT_J4`,
   `NUT_PREC_RA/DEC`-Reihen). Liefert
   `PckBody { gm, j2, j4, radii: [f64;3], pole_ra/dec + Raten, pm + Rate,
   nut_ra: Vec<[f64;3]>, nut_dec: Vec<[f64;3]> }`.
   Bodies ohne Eintrag → `Option` → 0 honored (ausdrücklicher Absenz-Zustand).
2. **`ephemeris_compiler.rs`**: `wgccre_for_body` (259 Zeilen Hardcode) gelöscht;
   Werte kommen aus dem PCK. Neue stype-1 v2: `gcount = 12` →
   `[α0, α̇, δ0, δ̇, w0, ẇ, R_a, R_b, R_c, J2, J4, GM]` (96 B). Legacy
   `gcount = 0` → 8 f64 alte Ordnung (Reader verzweigt auf gcount). Neuer
   stype-4: Nutationsreihen (variable Länge). Rotation-Matrizen-Sampling
   (`extract_granules`) mit Nutation.
3. **`horizons_compiler.rs`**: `&QUANTITIES='4'` in der Request-URL
   (horizons_request); GM aus der Antwort parsen. stype-1 v2 analog; Bodies
   ohne PCK-Daten → 0 honored (F39 bleibt als Absenz). BODY_IDS → NAIF-Mapping
   für PCK-Lookup der Horizons-Bodies.
4. **`main.rs`**: `BodyProperties` + `gm, j2, j4, radii_b, radii_c, nut_ra,
   nut_dec`; `parse_ephemeris_binary` verzweigt auf gcount (12 vs. 0) und liest
   stype-4. Polauswertung mit Nutation:
   `pole(t) = [cosδ·cosα, cosδ·sinα, sinδ]`, α/δ mit PCK-Reihen.
   Tests (main.rs:8190+) auf neue Felder anpassen.
5. **TODO.md** im selben Commit: U01, F39, B06 schließen; WGSL-J2 öffnen.

## Commit B — Runtime GM + Protokoll v6

1. **WS-Record 15→21 f64 (168 B)**:
   `[x,y,z,val,epoch,ttl,tau,extent,kernel_id,force_type,absorption,advection,
   vx,vy,vz, pole_x,pole_y,pole_z, j2, j4, r_eq]`.
   Versionsbyte `0x05`→`0x06`. Pole = nutations-ausgewerteter Einheitsvektor des
   Anchor-Körpers zum `response_epoch`; ohne Anchor → 0 honored.
2. **`body_channels`**: neuer `{body}.mass`-Kanal (`val = props.gm`, Kernel 0,
   Force gravity, tau ∞); `{body}.radius` bekommt `val = props.radius_m`
   (Fabrication val=1.0 getötet).
3. **`constants.js`**: Parser v6 (21 f64); `meta` 4→12 floats:
   `[extent, tau, kernel_id, 0, pole_x, pole_y, pole_z, j2, j4, r_eq, 0, 0]`.
4. **`index.html`**: `ensureFieldCapacity` → `c * 96 > maxBuf`
   (field 48 + meta 48 B/Osc); `syncFrame`-Konsumenten unverändert.
5. **Vertrags-Trace (AGENTS.md)**: Rust-Schleife main.rs:3414 ↔ constants.js
   ↔ WGSL-Unpack — alle drei Schichten im selben Commit.

## Commit C — Das Trommelfell (WGSL + JS)

1. **Neuer Compute-Shader `membrane_grid`** (workgroup 8×8, Dispatch
   `gridN/8 × gridN_y/8`): jeder Knoten berechnet seine Fensterposition
   (gleiche `pixel_rel`-Mathematik wie der alte fs, Knotenzentren), iteriert
   die Oszillatoren einmal, summiert 9 Kraft-Kanäle in 3 vec4 pro Knoten
   (storage). Zusätzlich `atomicMax` (i32-Bitcast auf |Ω|) je Kraft in einen
   9-Float-Buffer — das Fenster-Maximum. JS nullt die 36 B vor jedem Dispatch.
2. **Kernel-0/6 = reiner Nebra**: `1.0 / max(d2, 1.0)`, Guard d > 1.0 m.
   Kein Softening. **Zonal-Term** für Kernel 0/6 + force gravity:
   `P = GM/d² · [1 − J2·(r_eq/d)²·P2(cosθ) − J4·(r_eq/d)⁴·P4(cosθ)]`,
   `cosθ = dot(d̂, pole)`, P2(x) = (3x²−1)/2, P4(x) = (35x⁴−30x²+3)/8.
   Äquatorial verstärkend, polar abschwächend. j2 = 0 → Term 0 (0 honored).
   Kernels 1–5 unverändert (scale-basiertes Softening ihrer endlichen Extents).
3. **Relativität gelöscht**: `beta/gamma/dopp/aberration/apparent/tau→λ-Shift`
   raus aus dem WGSL (index.html:409–445). Galileische Weltlinien-Propagation
   (`propagated = m.xyz + v_rel·dt`) und `fold_eff` (Lichtlaufzeit) bleiben —
   das ist Block-Universum-Physik, kein Observer-Bias.
4. **fs wird Interpolator**: bilineare Abtastung des Knoten-Buffers
   (read-only-storage). Pro Kraft: feste Spektralband-λ (`780 − f·400/8`),
   `wavelength_to_rgb` bleibt, additive Superposition aller 9 Kanäle.
   Tonemap pro Kraft: a9d87bd-Form
   `log2(|Ω|/max(lvl, 2⁻⁶⁴))/8 + 0.5`,
   `lvl` = JS-EMA (Φ-Relaxation) über dem Fenster-Maximum je Kraft —
   **Exposure wird wieder konsumiert** (der tote `expose_*`-Pfad lebt auf).
   `e`/`E`-Boost bleibt (index.html:1350).
5. **Dynamische Netzdichte**: `gridN ∈ {16, 32, 64, 128, 256}` mit Hysterese
   auf `stableTick`: anhaltend > 1000/25 ms → halbieren; anhaltend < 1000/55 ms
   → verdoppeln (2ⁿ, ~30 Ticks Sustained). Buffer-Reallokation nur bei
   2ⁿ-Wechsel (gridN² × 48 B; 256² = 3 MB). `gridN`/`gridN_y` in
   `expose_ex.z/.w` (freie VP-Slots). `presence_probe` bleibt als
   1-Knoten-Probe für Audio/Vibrate/Serial (probedOmegas).
6. **Audio + Vibrate + HUD**: Gain `|Ω_f|/max(lvl_f, ε)` statt
   `tanh(Ω·windowMedianExtent())` (NaN-Tod bei INF-Extents behoben).
   Wheel-Divisor 128→512 (index.html:1263), Initial-Scale 2³¹→2³⁷
   (index.html:123). HUD zeigt Ω > 0.

## Commit D — Physisches Radiatorium (WebSerial `flow`)

1. Serial-Oberfläche (index.html:923–940) ersetzt Raw-Bytes durch das
   spezifizierte Protokoll, eine Zeile pro Kraft mit Ω > 0, gedrosselt auf
   `stableTick`:
   `flow <force_name> <force_id> <|Ω_f|> 1 <stableTick_ms> <tPresence> <x> <y> <z>\n`
   (force_name = z.B. `gravity`, force_id 0–8, unit `1`, Wert = roher Druck.)
2. 0 honored: kein Port → keine Zeile; Ω = 0 → Stille. USB/BT/HID-Oberflächen
   unberührt.

## Verifikation (nach jedem Commit)

1. `cargo check` 0 Errors / 0 Warnings; `cargo test` (13+ Tests, angepasste
   BodyProperties-Tests).
2. **Kernel-Regeneration lokal**: NAIF-Downloads
   (de440s.bsp, gm_de440.tpc, pck00010.tpc) → Compiler laufen lassen →
   `/tmp/omegaflow_eph_*.bin` überschreiben — löst U01 (m-Skala) gleich mit.
   Horizons-Q4-Format vorher live per curl prüfen (Risiko 1).
3. `cargo run` → Browser: HUD `Ω > 0`, nicht-schwarzes Fenster mit
   Sonnen-Feld am SSB (GM/d² bei 1 AU ≈ 5.8e-3), Grid-Atmung bei Zoom
   (Scale 2³⁷). Ein schwarzes Fenster ist nur intentional erlaubt, nie das
   Default-Ergebnis.
4. **Datenkontrakt-Trace**: 168-B-Record → constants.js 21 f64 →
   WGSL `field[j*3]` / `props[j*3..j*3+2]` — Zeile für Zeile gegenlesen.
5. Risiken: Horizons-Q4-Antwortformat (vorher curl); PCK-Nutations-Semantik
   (aus dem Dateikopf von pck00010.tpc verifizieren); Intel-HD-515-Performance
   bei 256² (Atmung greift); CDN-Re-Upload der neuen Binaries ist manuell
   (kein CI-Workflow im Repo).

## Nicht in dieser Session (dokumentiert in TODO.md)

- 1040 Celestial-cmap-Blöcke → dedizierte Kurations-Session nach
  `docs/source_curation.md`.
- CDN-Re-Upload der neukompilierten Ephemeriden (Infrastruktur, kein Code).
