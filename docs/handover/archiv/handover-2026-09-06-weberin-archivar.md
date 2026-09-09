<!--
  title: Handover — Weberin Schritt 1: Motion::Kepler baryzentrisch (Archivar-Zeilenplan)
  class: handover
  date: 2026-09-06
  sha256: 491c18e5be8c95f253b6e46ddda3a314f88744e2c95e5db88d0015e66dd44f90
  status: archived
  see-also: docs/concepts/die-weberin.md docs/concepts/archivar-mathematikerin.md docs/surveys/survey-2026-09-06-codestruktur.md
-->
# Handover — Weberin Schritt 1: Motion::Kepler baryzentrisch

Selbsttragend. Diese Übergabe trägt den zeilenweisen Archivar-Plan für den
ersten Schritt der Weberin: die eine heliozentrische Sonderwurst
(`Motion::Kepler` ohne Körper-Anker) wird baryzentrisch und ehrt die
`at <body>`-Eintragung agnostisch. Der Plan ist gemessen, nicht geraten —
jede Zeile wurde gegen den Baum gelesen.

## Der Befund (gemessen)

- `Motion::Kepler { rec }` (types.rs:25) ist die einzige nicht-selbstauflösende
  Bewegungs-Variante: `state_at` liefert heliozentrisch (Sonnen-Ursprung),
  während `Surface`/`Barycenter` sich über `body_name` selbst baryzentrisch
  auflösen.
- `build_asteroid_samples(bytes, ttl)` (spatial.rs:166) rechnet den Anker mit
  **leerer** Ephemeris (spatial.rs:167) — deshalb konnte der Sonnen-Offset nie
  addiert werden (geboren 2026-08-20, `8f57a25d`, nie eine Entfernung).
- Die Eintragung `at sun` (phi/sources.φ:1201) ist deklarativ korrekt, wird vom
  dastcom-Pfad aber nie gelesen (main_flow.rs:1043 übergibt keinen Frame).
- Keine weitere Sonderwurst: alle anderen Format-Zweige ehren den Frame oder
  sind ICRS (Sterne/Kosmologie, SSB-Ursprung).

## Code-Regeln (bindend)

Keine Docstrings, keine Kommentare, kein `unwrap`/`unwrap_or(0.0)`,
kein `_ => 0`, kein `max(1)`, kein `#[derive(Default)]`.
`cargo check` = 0 Fehler UND 0 Warnungen. `?` statt `unwrap`.

## Der Zeilenplan

### 1. src/archivar/types.rs (Z.25–27)
```rust
// ALT
    Kepler {
        rec: Arc<AsteroidRec>,
    },
// NEU
    Kepler {
        rec: Arc<AsteroidRec>,
        body_name: String,
    },
```

### 2. src/archivar/motion.rs
Z.339–345 — `Motion::Kepler::at`:
```rust
// ALT
            Motion::Kepler { rec } => {
                let t_jd = t / 86400.0 + J2000_EPOCH;
                match state_at(rec, t_jd) {
                    Some((p, _)) => finite_pos(p),
                    None => None,
                }
            },
// NEU
            Motion::Kepler { rec, body_name } => {
                let t_jd = t / 86400.0 + J2000_EPOCH;
                let (p, _) = state_at(rec, t_jd)?;
                let anchor = body_barycenter_position(body_name, t, eph)?;
                finite_pos([p[0] + anchor[0], p[1] + anchor[1], p[2] + anchor[2]])
            },
```
Z.350–358 — `anchor_body`:
```rust
// ALT
            Motion::Surface { body_name, .. } | Motion::Barycenter { body_name, .. } => Some(body_name),
            Motion::Linear { .. } | Motion::Kepler { .. } | Motion::Spherical { .. } => None,
// NEU
            Motion::Surface { body_name, .. } | Motion::Barycenter { body_name, .. } | Motion::Kepler { body_name, .. } => Some(body_name),
            Motion::Linear { .. } | Motion::Spherical { .. } => None,
```

### 3. src/archivar/spatial.rs (Z.166–188)
```rust
// ALT
pub fn build_asteroid_samples(bytes: &[u8], ttl: u64) -> Vec<Sample> {
    let eph: HashMap<String, BodyEphemeris> = HashMap::new();
// NEU
pub fn build_asteroid_samples(
    bytes: &[u8],
    ttl: u64,
    body_name: String,
    eph: &HashMap<String, BodyEphemeris>,
) -> Vec<Sample> {
```
```rust
// ALT Z.181–186
        let motion = Motion::Kepler {
            rec: Arc::new(rec.clone()),
        };
        let Some((anchor_vmax, anchor_amax, anchor_p0)) =
            law_bounds(&motion, epoch_secs, 0.0, &eph)
// NEU
        let motion = Motion::Kepler {
            rec: Arc::new(rec.clone()),
            body_name: body_name.clone(),
        };
        let Some((anchor_vmax, anchor_amax, anchor_p0)) =
            law_bounds(&motion, epoch_secs, 0.0, eph)
```

### 4. src/archivar/main_flow.rs (dastcom-Zweig, Z.992 ff.)
Tor (Sonne geladen, sonst vertagen):
```rust
// ALT
            if archive.sources[i].format == "catalog_dastcom" {
// NEU
            if archive.sources[i].format == "catalog_dastcom" {
                if !archive.body_ephemerides.contains_key("sun") {
                    continue;
                }
```
Snapshot + Frame-Körpername (nach `let src_ttl = src_clone.ttl;`, vor `thread::spawn`):
```rust
                let eph_arc = archive.body_ephemerides.clone();
                let body_name = frame_body_name(&src_clone.frame);
```
Aufruf (Z.1043):
```rust
// ALT
                    let samples = build_asteroid_samples(&bytes, src_ttl);
// NEU
                    let samples = build_asteroid_samples(&bytes, src_ttl, body_name, &eph_arc);
```

### 5. src/archivar/tests.rs
- Z.1529, 1554 — `Motion::Kepler { rec: … }` → Feld `body_name: "sun".to_string()`.
- Z.1596, 1599 — Muster `Motion::Kepler { rec: rec_gm }` → `{ rec: rec_gm, .. }`.
- Z.1562–1608 — Test `test_build_asteroid_samples_gm_radius_and_query`:
  - `build_asteroid_samples(&bin, 86400)` → mit `"sun".to_string()` + synthetischer
    Sonnen-`BodyEphemeris` (Muster `tests.rs:2882`/`3023`).
  - Z.1604: `anchor_p0[0] ≈ au_m` → `anchor_p0[0] ≈ au_m + sun_ssb[0]`.
  - Z.1607 ff.: `build_buffer`/Query brauchen dieselbe Sonnen-Map.

## Verifikation
`cargo check` (0 Fehler/0 Warnungen), `cargo test` grün — insb.
`test_build_asteroid_samples_gm_radius_and_query`.

## Danach (eigene Zeilenpläne, wenn erreicht)
- Schritt 2 — ICRS-Blöcke ehrlich deklarieren (statt ignoriertem `at sun`).
- Schritt 3–4 — Weberin-Maschine (mathematikerin).
- Schritt 5–6 — Schwelle kalibrieren, DASTCOM→MPC.
