# TODO

AGENTS.md is the primary constraint matrix. Git is the history.

## Stand — 2026-08-14 (LSK/PCK-Hochzeit, Binary v2, Protokoll v6)

LSK-Kernel naif0012.tls als Zeit-Quelle (TT_MINUS_UTC-Konstante gelöscht; Zeit absent ohne LSK). PCK-Reader pck.rs (gm_de440.tpc, pck00010.tpc: POLE/PM/RADII/J2/J4/NUT). stype-1 v2: gcount=12, 96 B festes Stride (α0, α̇, δ0, δ̇, w0, ẇ, R_a, R_b, R_c, J2, J4, GM), non-v2 → None. Compiler schreiben v2 aus PckBody (ephemeris, NAIF-Mapping 1→199 etc.) bzw. Horizons QUANTITIES='1,2,4' GM-Parse. F39 gelöscht. body_channels: {body}.radius = radius_m, {body}.mass = gm (Kernel 0, gravity, tau ∞, nur wenn gemessen). WS-Protokoll v6: 21 f64 (168 B) + pole/j2/j4/r_eq. body_pole_at mit Nutation, triaxiales Ellipsoid (radii_b/radii_c). Lokale v2-Binaries unter /tmp/omegaflow_eph_*.bin (28 Körper). 0 Warnings, 0 Errors.

---

### Einheiten (1)

**U01** CDN-Ephemeriden neu manifestieren (v2 + m-Skala)
```
Compiler gefixt (×1000, stype-1 v2 gcount=12); 28 Binaries lokal neu kompiliert und
unter /tmp/omegaflow_eph_*.bin verifiziert. Der CDN-Release trägt noch gcount=0-Legacy
(km) → Runtime gibt None → Körper absent (0 honored) bis Re-Upload.
```

**U02** Monde (io…triton) ohne Ephemeriden
```
de440s.bsp trägt keine Mond-Segmente → jup365/sat441/mar097/ura111/nep095.bsp
herunterladen, mit ephemeris_compiler kompilieren (v2), nach /tmp/omegaflow_eph_*.bin.
Bis dahin: Monde absent (0 honored).
```

**U03** J2/J4 für weitere Körper: nur Erde hat Text-Quelle
```
geophysical.ker (auch live auf NAIF) trägt BODY399_J2/J4 → Erde hat echte
Harmonische (j2=1.082616e-3 im v6-Record verifiziert). Weitere Körper: Werte liegen
nur in binären .bpc-Kerneln (earth_*.bpc, moon_pa_*.bpc) → Binary-PCK-Reader
(DAF-Segment-Typ 2) als nächste Quelle. Bis dahin: j2/j4 = 0.0 neutral (0 honored).
```
(Scanner-Fix für nackte Werte + Kommentarzeilen mit '='-Suche ist drin.)

---

### Zentrismus (3)

**Z03** Sonne (NAIF 10) spezialbehandelt im Ephemeriden-Compiler
```
ephemeris_compiler.rs:658 → if granules.is_empty() && target_id != 10 {
```

**Z04** Ephemeris-Kanäle hardcoded auf gravity
```
main.rs:4637 → kernel: 0, force: 1, tau: 86400.0 * 365.0,
```

**Z08** Device-Daten verworfen ohne Körper-Ephemeriden
```
main.rs:2954 → if let (Some(lat), Some(lon), Some(alt)) = (st_lat, st_lon, st_alt) {
```

---

### Hack (2)

**H11** Hot-Path-Clones: `all.push(s.clone())` jeden Tick
```
main.rs:7245 → all.push(s.clone());
```

**H12** `query_hash` klont Zellen-Referenzen
```
main.rs:643 → out.push(samples);
```

---

### Fabrikation (4)

**F33** Geschwindigkeit `[0.0, 0.0, 0.0]` — CelestialPolygon
```
main.rs:5161 → v: [0.0, 0.0, 0.0],
```

**F35** State-Vector hardcoded gravity
```
main.rs:4637 → kernel: 0, force: 1, tau: 86400.0 * 365.0,
```

**F40** NAIF-ID unbekannt → "unknown"
```
ephemeris_compiler.rs:604 → _ => "unknown"
```

---

### Daten zweiter Klasse (2)

**D07** Kein Oszillator-Cap in Rust
```
index.html:551 → while (c * 96 > maxBuf) c >>= 1;
```

**D08** `/device` scannt nur Device-Quellen
```
main.rs:2746 → if matches!(osc.source, OscillatorSource::Device)
```

### Bias (2)

**B05** ISS bekommt spezielles Datenfenster
```
horizons_compiler.rs:727 → let months = if *name == "iss" { 0.9 } else { 1.0 };
```

**B06** Hardcodierte Body-Listen in Compilern
```
BODIES_WITH_MEDIA in beiden Compilern gelöscht (stype-2 immer, absent = 0.0).
Verbleibend: horizons_compiler.rs wgccre_for_body-Zwillingstabelle (→ PCK-Hochzeit),
ephemeris_compiler.rs all_target_ids + body_id_to_name Tabelle.
```

---

## Membran (Stand 2026-08-14: reine Nebra-Thermik)

- ~~Trommelfell: Compute-Grid~~ — **getilgt**. Rückkehr zum Nebra-Prinzip: der fs
  evaluiert `osc_field` für JEDEN Pixel (kontinuierliche Mathematik, keine Stützstellen,
  keine Interpolation — Raum-Lüge und Zeit-Lüge verworfen, Council-Urteil über alle
  sechs Berater-Positionen in docs/concepts/WGSL_ SHADER.md).
- ~~Exposure-Kette~~ — **getilgt**: atomicMax, readForceMax, kernelExpose, e/E-Tasten,
  expose_lo/hi/ex sind tot. Die Nebra-Rampe (tiefes Blau → Cyan → Orange → Weiß,
  4 mix()-Segmente, Original aus /home/johannes/projects/nebra) IST die Tonemap:
  `t2 = clamp((log2(Ω_total) + 14.0) / 22.0, 0, 1)` mit Ω_total = Σ|omegas[k]|.
  Fixe GM-Ordnungs-Kalibrierung: Sonne bei 1 AU → t2 ≈ 0.30.
- `stableTick` ist reiner HUD-Messwert — kein Regler verkleinert das Feld. Der fallende
  Tick ist die ehrliche Kapazitätsmessung des Siliziums.
- presence_probe bleibt für Audio/Haptik/Serial (Sonde am Präsenzpunkt).

### Verbleibend
- WebSerial flow-Protokoll: `flow <force> <id> <|Ω|> 1 <tick_ms> <t> <x> <y> <z>`.
- Audio-Gain auf lvl-Basis statt tanh(Ω·medianExtent).
- Wheel-Divisor 128→512, Initial-Scale 2³¹→2³⁷ (Nebra-Kalibrierung).
- Horizons-Zwillingstabelle löschen (PCK-Hochzeit); stype-4-Nutation.
- Forschungs-Iteration (Council): Backend als „langsamer Prior" für Exposure-Kaltstart
  (falls je wieder eine Anpassung gewünscht wird — aktuell: fixe Rampe).

---

## Residual

### Source Curation
- `archeology/sources/sources_gold_359-domains.φ` — 27K lines, 359 domains, OLD grammar
- EVERY source block needs migration: `force em` → `field <key> <name> <kernel> <force> <unit>`
- Arena batches (`archeology/arena/`) contain API proposals not yet in φ files
- `archeology/sources/sources_new_untested_14k_new-unchecked.φ` — 873 unchecked blocks
- `archeology/sources/sources_new_untested_candidate-staging.φ` — staging candidates
- `archeology/foundation/gaps.md` — domain coverage gaps
- `archeology/foundation/collection.md` — curated collection state

### Code Hygiene
- Tau-Gate inkonsistent: 5-token/6-token prüfen `v > 0.0`, 9-token prüft nicht
- Kamera: ~19k Pixel-Oszillatoren (4×4-Raster) → WS-Traffic-Hotspot
- `sensor_config`/`probe_classify`-τ/TTL-Konstanten ohne Herleitung (60/300/0.01/3600)
- `getRto` min/max 100/5000 ms — nicht 2ⁿ/Φ-hergeleitet (constants.js)
- Probe: `coordinates.2` als `alt … km` — bei Seismik-Feeds ist es Tiefe (−km); Vorzeichen-Heuristik offen
- CDN-Asset-Naming `{name}.json` (clobber) vs AGENTS-Konvention `{prefix}_{iso8601}.json`

### Probe-Skalierung (300k URLs → kuratierte sources.φ)
- URL-Listen-Ingest: eine URL pro Zeile → Default-Blöcke (ttl-Schätzung, Frame-Frage)
- Rate-Limit pro Netloc (ttl/Φ), Blocklist für 403/404-Domains
- Zweit-Fetch nach Δt → beobachtete Änderungsrate → ttl
- `vel`-Unit-Konvertierung (m/s fest), `z`-Redshift-Key für cmap
- UNCERTAIN-Felder: Review-Pass (Kybernaut) → Force/Unit-Zuweisung → Registry wächst

### Validation
- `--verify` CLI exists (tests URL reachability), no sources loaded yet
- Old sources: `pos` and `field_in` tokens → parser ignores via `_ => {}`

### CI Pipeline
- `mirror-cdn.yml`: cron disabled
- `generate-ephemerides.yml`: Python/spiceypy → needs Rust rewrite
- `refresh-protected-data.yml`: Python inline scripts → needs Rust rewrite

### Feature Backlog
- Advective per-Oszillator: wind speed in `tm.w` (channel wired, data source needed)
- OPeNDAP Integration
- New Extract Types: Kepler, HorizonsVec, Flatten
- `field_in` nested support
- Command Palette (⌘K)
- Minkowski 4D Weighting

### Deferred
- Temporal Topology (TDA, Takens, Transfer Entropy)
- Field Permeability

### Rejected
- Unknown-Force soft fallback → parser rejects unknown force
- Default τ values → gate closes if not declared
- World Bank Indicators → forceless, DROP
- Yahoo Finance → forceless, DROP
