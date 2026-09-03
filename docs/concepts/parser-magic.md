<!--
  title: Parser Magic
  class: concept
  sha256: a799e131d648dde46cc4037022c04f3d3ccd823ad92f928b9279e8ca3a0c0b4a
-->
# Parser Magic

STATUS: DEPLOYED (sections 1-11 of Present) / PARTIALLY DEPLOYED (Missing items 1-2, 5-8, 11-13)

---

## Present — What the 5399-line std-only Rust parser does

**1. Zero-Imports Purity:** HTTP server, WebSocket server, JSON parser, SHA1, Base64, calendar — all hand-written with pure std-lib. No serde, no regex, no tokio, no chrono.

**2. Hand-Written Regex Engine** (line 1140-1471, ~330 lines): backtracking matcher with `\d \s \w \D \S \W` escapes, quantifiers `+ * ?`, wildcard `.`, character classes `[...]` and a capture group for numeric extraction from HTML/text responses.

**3. Celestial Mechanics** (line 30-230): Kepler equation via Newton-Raphson (5 iterations), GMST for Earth rotation, WGS84 ellipsoid, geodetic → ECEF → ECI → ecliptic → ICRS (barycentric). `iau2000_to_icrs` — simplified Mars rotation with fixed day length (88642,66 s).

**4. Schema-Sniffing** (`universal_auto_detect`, line 852-950): detects star catalogs automatically (ra/dec + optional plx/pmra/pmdec/radvel → CelestialMap) and tracking data (lat/lon + optional vel/trk/vr → Map). No config needed, pure structural heuristics.

**5. ~40-Variable Template DSL** (line 2873-3017): `{today}`, `{yesterday}`, `{hour_ago}`, `{week_ago}`, `{grid}` (4×4 point grid), `{lat_min}/{lat_max}`, `{unix_now_plus_3600}`, `{SECRET_NAME}` — every source defines its own date format/BBox/grid without API-specific code. Since 2026-08-17 also `{jd_now}`/`{jd_start}`/`{jd_end}` (TDB, 6 decimal places).

**6. Physics-Driven Spatial Partitioning** (line 252-305): `law_bounds` estimates v and a via finite differences, scaled with Φ as the safety margin. Cell size from `rmax + vmax·cadence + 0.5·amax·cadence²`, rounded up to the next power of two.

**7. Wave Propagation Constants** (line 316-328):
```
0 => C_LIGHT              (EM)
2 => V_SOUND_288          (acoustic)
3 => V_P_GRANITE          (seismic-body P-wave)
4 => V_S_GRANITE          (seismic-body S-wave)
5 => ALPHA_AIR            (thermal diffusivity)
6 => D_AIR                (diffusion coefficient)
```

**8. Jacobson/Karels RTT in JS** (`constants.js`): the classic TCP adaptive timeout algorithm for WebSocket retry timing.

**9. Source-faithful url-first parsing**: block separation via blank lines. `url` as anchor, `flush!()` at the next `url`. No more `source <name>` anchor.

**10. Extract-Types**: Field, Last, Count, GeojsonEvents, Path, Map, CelestialMap, Rows. Since 2026-08-17: `fold <op> <key_a> <key_b> <force> <unit> <tau>` (mean|diff|sum), `tau_key <key>` (per-row τ, 0 closes the gate), `vel <key> [unit]`.

**11. Body-Agnostic Media Constants**: `v_sound`, `v_seismic_p/s`, `alpha_thermal`, `d_diffusion`, `v_advective` per BodyProperties from the ephemeris binary stype==2.

**12. SI conversion total (2026-08-17)**: `convert_to_si` → `Option<f64>` at the anchor; unknown/logarithmic unit → the oscillator does not manifest (registered on stderr). `deg`/`arcsec` → rad, M_sun/M_earth/R_earth, MW (case-exact against Mw/M), d, uatm, mb, n/cc, cfs, %, psu, DU, pc/cm3.

---

## Missing — 8 parser gaps

**1. Auto-frame from `lat_key`/`lon_key`** — sources with `lat_key`/`lon_key` and `map` but without `lat`/`lon` are refused because `has_data_position` applies only to Map/GeojsonEvents/CelestialMap. When `lat_key` is set, set Frame=Data.

**2. Improve `extent` per force type** — an EM sample with `C_LIGHT * tau` as extent is gigantic. Gravity bodies would need `extent = body_radius`, not `c * τ`.

**3. ~~`kepler_map` parsing~~** — DONE 2026-08-17: key directives a/e/i/om/w/ma/epoch/qr/tp wired, MPC q→a + tp→M, solver `src/kepler.rs::elements_to_icrs_state`.

**4. ~~`vectors` / Horizons text parser~~** — DONE 2026-08-17: `{jd_now}`/`{jd_start}`/`{jd_end}` (TDB) — the calendar-date-in-JD-field cause is healed. A live `vectors` block remains a curation question.

**5. `cmap` Celestial Map Parsing** — RA/Dec (deg), parallax (mas → distance), proper motion (mas/yr → 6D state), radial velocity. Keys present in SourceConfig, the parser does not fill them.

**6. `window` / Temporal Bounding** — no `from`/`until` directly in the config. Only `{today}` in the URL template.

**7. Constant `lat_key`/`lon_key` Detection** — `lat_key 48.1` is recognized as a constant, not as a column name.

**8. `map` as frame indicator** — when `map` is set AND `lat_key`/`lon_key` exist, Frame=Data follows automatically.

**9. ~~`field_in` nested support~~** — DONE 2026-08-17: `field_in` is refused+registered in the parser; the `--gold` port migrates to `field`, nested paths (dot + array index) run via jpath.

**10. ~~`Flatten` Extract-Type~~** — DONE 2026-08-17: generic flattener — geom empty → row coordinates, geom without a `coordinates` child → the value itself as coordinate array, multi-level recursive.

**11. Unknown force → alternate path** — when `force_constants` returns `None`, the source is refused. A gentle alternate path would be possible, but is rejected per AGENTS.md.

**12. Category/group inheritance** — no inheritance of defaults from parent groups in the φ namespace.

**13. Extent zero → standing value** — samples with `extent=0` are not found by the Enclosure filter. A minimal standing extent is needed.
