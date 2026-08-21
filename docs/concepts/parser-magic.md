<!--
  title: Parser Magic
  class: concept
  sha256: 137c72928b0afc087bcf22f5b66937554dccff7e60c5728928d707569a333d33
-->
# Parser Magic

STATUS: DEPLOYED (sections 1-11 of Present) / PARTIALLY DEPLOYED (Missing items 1-2, 5-8, 11-13)

---

## Present — What the 5399-line std-only Rust parser does

**1. Zero-Imports Purity:** HTTP-Server, WebSocket-Server, JSON-Parser, SHA1, Base64, Kalender — alles handgeschrieben mit reiner std-lib. Kein serde, kein regex, kein tokio, kein chrono.

**2. Hand-Written Regex Engine** (line 1140-1471, ~330 lines): Backtracking-Matcher mit `\d \s \w \D \S \W`-Escapes, Quantifiern `+ * ?`, Wildcard `.`, Zeichenklassen `[...]` und Capture-Gruppe für numerische Extraktion aus HTML/Text-Antworten.

**3. Celestial Mechanics** (line 30-230): Kepler-Gleichung per Newton-Raphson (5 Iterationen), GMST für Erdrotation, WGS84-Ellipsoid, geodetic → ECEF → ECI → ekliptikal → ICRS (baryzentrisch). `iau2000_to_icrs` — vereinfachte Mars-Eigenrotation mit fixer Tageslänge (88642,66 s).

**4. Schema-Sniffing** (`universal_auto_detect`, line 852-950): Erkennt automatisch Sternkataloge (ra/dec + optional plx/pmra/pmdec/radvel → CelestialMap) und Tracking-Daten (lat/lon + optional vel/trk/vr → Map). Keine Config nötig, reine Struktur-Heuristik.

**5. ~40-Variable Template DSL** (line 2873-3017): `{today}`, `{yesterday}`, `{hour_ago}`, `{week_ago}`, `{grid}` (4×4-Punktegitter), `{lat_min}/{lat_max}`, `{unix_now_plus_3600}`, `{SECRET_NAME}` — jede Quelle definiert ihr eigenes Datumsformat/BBox/Grid ohne API-spezifischen Code. Seit 2026-08-17 auch `{jd_now}`/`{jd_start}`/`{jd_end}` (TDB, 6 Stellen).

**6. Physics-Driven Spatial Partitioning** (line 252-305): `law_bounds` schätzt v und a per finiter Differenzen, skaliert mit Φ als Sicherheitsmarge. Zellengröße aus `rmax + vmax·cadence + 0.5·amax·cadence²`, aufgerundet zur nächsten Zweierpotenz.

**7. Wave Propagation Constants** (line 316-328):
```
0 => C_LIGHT              (EM)
2 => V_SOUND_288          (acoustic)
3 => V_P_GRANITE          (seismic-body P-wave)
4 => V_S_GRANITE          (seismic-body S-wave)
5 => ALPHA_AIR            (thermal diffusivity)
6 => D_AIR                (diffusion coefficient)
```

**8. Jacobson/Karels RTT in JS** (`constants.js`): Klassischer TCP-Adaptive-Timeout-Algorithmus für WebSocket-Retry-Timing.

**9. Quellentreue url-first Parsing**: Block-Trennung über Leerzeilen. `url` als Anker, `flush!()` bei nächstem `url`. Kein `source <name>`-Anker mehr.

**10. Extract-Types**: Field, Last, Count, GeojsonEvents, Path, Map, CelestialMap, Rows. Seit 2026-08-17: `fold <op> <key_a> <key_b> <force> <unit> <tau>` (mean|diff|sum), `tau_key <key>` (per-row τ, 0 schließt das Gate), `vel <key> [unit]`.

**11. Body-Agnostic Media Constants**: `v_sound`, `v_seismic_p/s`, `alpha_thermal`, `d_diffusion`, `v_advective` per BodyProperties aus Ephemeris-Binary stype==2.

**12. SI-Konversion total (2026-08-17)**: `convert_to_si` → `Option<f64>` am Anker; unbekannte/logarithmische Einheit → Oszillator manifestiert nicht (stderr registriert). `deg`/`arcsec` → rad, M_sun/M_earth/R_earth, MW (Fall-exakt gegen Mw/M), d, uatm, mb, n/cc, cfs, %, psu, DU, pc/cm3.

---

## Missing — 8 parser gaps

**1. Auto-Frame aus `lat_key`/`lon_key`** — Sources mit `lat_key`/`lon_key` und `map` aber ohne `lat`/`lon` werden refused weil `has_data_position` nur für Map/GeojsonEvents/CelestialMap gilt. Wenn `lat_key` gesetzt, Frame=Data setzen.

**2. `extent` pro Force-Type verbessern** — EM-Sample mit `C_LIGHT * tau` als extent ist gigantisch. Gravity-Bodies bräuchten `extent = body_radius`, nicht `c * τ`.

**3. ~~`kepler_map` Parsing~~** — ERLEDIGT 2026-08-17: Key-Direktiven a/e/i/om/w/ma/epoch/qr/tp verdrahtet, MPC q→a + tp→M, Solver `src/kepler.rs::elements_to_icrs_state`.

**4. ~~`vectors` / Horizons Text Parser~~** — ERLEDIGT 2026-08-17: `{jd_now}`/`{jd_start}`/`{jd_end}` (TDB) — die Kalenderdaten-im-JD-Feld-Ursache ist behoben. Ein Live-`vectors`-Block bleibt Kurationsfrage.

**5. `cmap` Celestial Map Parsing** — RA/Dec (deg), Parallaxe (mas → Distanz), Proper Motion (mas/yr → 6D State), Radialgeschwindigkeit. Keys im SourceConfig vorhanden, Parser füllt sie nicht.

**6. `window` / Temporal Bounding** — Kein `from`/`until` direkt im Config. Nur `{today}` im URL-Template.

**7. Constant `lat_key`/`lon_key` Detection** — `lat_key 48.1` sollte als Konstante erkannt werden, nicht als Spaltenname.

**8. `map` als Frame-Indikator** — Wenn `map` gesetzt UND `lat_key`/`lon_key` existieren, sollte Frame=Data automatisch folgen.

**9. ~~`field_in` Nested Support~~** — ERLEDIGT 2026-08-17: `field_in` wird im Parser refused+registriert; der `--gold`-Port migriert zu `field`, nested Pfade (dot + Array-Index) laufen über jpath.

**10. ~~`Flatten` Extract-Type~~** — ERLEDIGT 2026-08-17: generischer Flattener — geom leer → Zeilen-Koordinaten, geom ohne `coordinates`-Kind → Wert selbst als Koordinaten-Array, mehrstufig rekursiv.

**11. Unbekannte Force → Fallback** — Wenn `force_constants` `None` zurückgibt, wird Source refused. Sanfter Fallback wäre möglich, ist aber per AGENTS.md abgelehnt.

**12. Kategorie/Group Inheritance** — Keine Vererbung von Defaults von übergeordneten Gruppen im φ-Namespace.

**13. Extent Zero → Default** — Samples mit `extent=0` werden vom Enclosure-Filter nicht gefunden. Minimaler Default-Extent nötig.
