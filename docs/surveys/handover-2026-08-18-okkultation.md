# Handover — Stellare Okkultationen (2026-08-18)

Selbsttragend. Einstieg: `AGENTS.md` (Constraint-Matrix), `TODO.md`
(das Register), `docs/SOURCE_PORT.md` (Source-Arbeit). Branch `main`.

## Was diese Session getan hat (der Arc)

1. **Der Frosch gegessen** — stellare Okkultationen live, das Flaggschiff
   der Handover-C-Kette. Die Messung, die kein Katalog speichert: der Stern
   dunkelt hinter dem Fels. Vermessung ergab: DASTCOM trägt `radius_km` +
   `gm_km3_s2`, aber `build_asteroid_hash` verwirft alle Records ohne GM —
   die größten Okkluder (Kleopatra u.a.) gingen verloren. Der Rat entschied:
   der Okkluder-Satz ist **radius-gegatet**, getrennt vom Gravitations-Hash.
2. **Okklusions-Pfad** (main.rs): `build_occluder_set` (radius_km>0 +
   gültige Bahn) → `OccIndex`-Richtungs-Raster (~1e-3 rad) relativ zur
   Presence (Rebuild bei t-Drift 60 s / center-Drift 1e8 m) → Scan 1 Hz:
   deep-Sterne gegen die Okkluder der Zelle±1, Ray-Sphäre mit Gravitations-
   Limb (`r_eff = R − 4GM|C|/(c²R)`), Treffer als Barrieren an den
   bestehenden `barriers`-Buffer angehängt — die WGSL dimmt den Stern
   physisch. KEIN WGSL-Change, KEIN Protokoll-Change. Register: HUD
   `okkl N` + Ereigniszeile (Asteroid-Nr. + Sternrichtung).
3. **Hill-Sphäre repariert**: `hill_radius_m` fehlte der `(1−e)`-Faktor
   (das TODO trug ihn, der Code nicht). + Exzentrizitäts-Test. Befund: der
   Wert fließt nur als `is_none`-Gate im Hash — keine Mitgliedschafts-
   Änderung; die Manifestation (Hill-Radius als räumliche Reichweite) bleibt
   offen.
4. **Register repariert** (TODO.md): P6-Drift berichtigt (Deep-Link-Init
   `#x,<x>,<y>,<z>,<t>` existiert in `main.rs presence_init`, nur die
   Geschwindigkeit `[,vx,vy,vz]` fehlt); „Empfohlene Reihenfolge" neu —
   Okkultation ist erledigt, Farbe bleibt `pending` (Daten geerntet, nicht
   gemalt — die Wahrheit steht).

## Die Grenzen (registriert, nicht verschwiegen)

- Der Scan taktet 1 Hz (Ereignisse dauern Sekunden). Subsekündige
  Ultra-Nah-Passagen (NEO <0,01 AU) entkommen dem Raster. Höhere Kadenz =
  eigene Arbeit.
- `dr3_stars.bin` trägt keine Sternnamen — die Ereigniszeile nennt die
  Richtung (ra/dec), keinen Namen.
- Die 4 GM-Lücken (45 Eugenia, 87 Sylvia, 90 Antiope, 216 Kleopatra)
  okkludieren korrekt (radius>0, gm=0 → r_eff=R, 0 honored). Als
  Gravitations-Oszillatoren warten sie weiter auf die INPOP25c-GM-Integration
  (Quelle von b5 geerntet, Integration offen).

## Verifikation

`cargo check` 0 Fehler / 0 Warnungen. `cargo test --bin omegaflow`: 76 grün
(inkl. `occ_hits_ray_geometry`, `occ_dir_cell_maps_and_wraps`).
`cargo test --lib dastcom`: 6 grün (inkl. `hill_radius_scales_with_eccentricity`).

## Warnungen für die nächste Session

- Zwei Renderer-Module: `mod archivar` (Fetch/Parse/Kepler; `state_at`,
  `C_LIGHT`, `J2000_EPOCH`) vs `mod mathematikerin` (wgpu, Sense-Thread;
  eigenes `const C`). Die Okklusion lebt in `mathematikerin`; wer sie
  erweitert, importiert `state_at` + `J2000_EPOCH` (jetzt `pub`) aus
  `archivar`.
- Eine parallele Session (b5) war während dieser Arbeit live in `main.rs` /
  `TODO.md`. Vor jedem Commit `git status` prüfen und nur eigene Dateien
  stagen — nie die Arbeitsfläche einer anderen Session.
