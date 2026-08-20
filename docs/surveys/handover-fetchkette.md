# Übergabe — Atom: Die tote Fetch-Kette („body: 0 sources, api: 0 sources, 0 recs")

## Symptom

Live-Boot: `omegaflow v1.0.0 | φ v7 | body: 0 sources, 0 samples | api: 0 sources, 0 samples` und das Fenster trägt `0 recs` — dauerhaft schwarz. Die Boot-Zeile zählt Samples im Spatial-Hash (archivar.rs ~10700), d.h. die Fetch/Anker-Kette hat nichts produziert. Die Zeitbasis bootet korrekt (t ≈ 840490825 = TDB ✓ — der LSK-Atom steht). **Nicht** Schuld: Parser, Rust↔WGSL-Protokoll (pack_window ↔ Shader ist verifiziert ausgerichtet), LSK (eingebettet).

## Befund (Git-Archeologie, vier gestaffelte Regressionen)

1. **`d0c17a6` (2026-08-09):** Economy-Gate in der ω-Schleife — `r = max kernel_extent(force, kernel, body_props, tau)` + neu `if r == 0.0 { continue; }` (archivar.rs ~16102). Vorher feuerte der Fetch immer (r = ∞).
2. **`0259974` (2026-08-15):** kernel 0 (inverse-square = em/gravity-Default) fällt von `INFINITY` auf `0.0` — alle em-Quellen: r = 0.
3. **`d1c1777` (2026-08-11):** der Rust-`ephemeris_compiler` schreibt stype-2-Kernel-Parameter als `[0.0; 5]` (Zeile ~461) — die Python-Ära schrieb die echten Medium-Geschwindigkeiten. Damit liefern auch Kernel 1/3/5 (advective/diffusion/thermal) `0·τ = 0` → **jede nicht-gravitative Quelle wird nie gefetcht** („api: 0").
4. **`af43b04` (2026-08-11):** die Boot-Presence (SSB, ∞-Reichweite) wurde entfernt; `presence_gate` (seit `56195ba`) + `2a19cba` (Parser verwirft Körper ohne stype-2) → „body: 0".

Sichtbarkeits-Ende: das Feld lebte zuletzt nur über den netcdf-Bypass (`a20e84a`, 08-20 05:00, 4022 samples — letzter dokumentierter nicht-leerer Boot); **Atom 6 (`8f57a25`, SpatialHash-Umbau) kappte den Bypass, Atom 8 (`e918bda`) setzte die Diode** → seither schwarz.

## Auftrag

1. **Gate-Reparatur:** `r` = physikalische Reichweite statt Kernel-extent — em → `c·ttl·64` (Signalkegel über die Sample-Lebensdauer), übrige Kräfte → ihr Ausbreitungsmaß; der `r == 0.0`-Skip entfällt für Anker, die im Presence-Fenster liegen. Ort: archivar.rs ~16096–16140, `kernel_extent`/`kernel_reach`.
2. **Compiler-Reparatur:** stype-2 schreibt wieder echte Per-Body-Medium-Konstanten. **Offene Frage:** die Datenquelle — woher nahm der Python-Compiler die Werte (PCK? Archeologie unter `/home/johannes/projects/archive/`)?
3. **Headless-Beweise vor dem Boot:** vorhanden: „sense_membrane liefert die Sonne bei floor 0" (Zellpfad ohne Floor-Gate). Neu: Test, der die Fetch-Dispatch-Gate-Logik mit einer em-Quelle durchläuft und behauptet, dass sie fetchen darf.
4. **Gates:** cargo check 0/0 (vier Feature-Kombis), cargo test — **erst dann der Live-Boot als letztes Gate** (body > 0, api > 0, recs > 0).

## Offene Entscheidungen

- Presence-Seed: soll die Presence-Map beim Boot leer bleiben (das Fenster meldet sich Frame 1 selbst — der erste Fetch-Pass läuft dann leer und wiederholt sich per ttl) oder ein benannter Boot-Anker zurück (der entfernte SSB-∞-Eintrag)?
- Der Wert der Medium-Konstanten je Körper: aus PCK/Ephemeriden-Pipeline rekonstruieren oder als eigener Unter-Atom?

## Nicht anfassen

Keine Hotfixes, keine Fallbacks; der eingebettete LSK („Zeitbasis = Programm-Identität") bleibt; die laufende Arbeit der Operatorin (gong/rpw, fits.rs, kernel_flatten.yml) bleibt unangetastet.
