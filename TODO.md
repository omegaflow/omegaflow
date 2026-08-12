# TODO

AGENTS.md is the primary constraint matrix. Git is the history.

## Stand — 2026-08-12 (4D-Worldline + Channel-Pipeline)

Protokoll v5 (120 byte/Osc, response_epoch, velocity), 4D Enclosure Lemma (temporal dilation via delta_t_cache), GPU velocity-propagation in build_field_grid. Body-Channel-Pipeline: body_channels() zerlegt BodyProperties in radius/rotation/gi_sq-Channels via anchor(). Probe kernel_id→force_type, 7→9 Omegas, Audio 9 Partial. Kernel-0-Physik: Softening = max(extent,gridStep)², kein künstlicher Nyquist-Floor. Navigation: Mausrad ×4, [-]/[+] Zoom ×4, Jump-Scale 2^28, Initial-Scale 2^31. StderrRadiator: source/oscillator-Counts getrennt, kompakt. HUD: 3-zeilig, UPPERCASE-Header, feste Spalten, Selbst-Dokumentation. Jump-Reparatur: last_field als Option<Buffer>. Consent: Audio außerhalb des Gates (Radiator). Agent permissions: council/explore/general edit:deny. 0 Warnings, 0 Errors.

---

### Einheiten (1)

**U01** ephemeris_compiler schrieb km-Granules — CDN-Ephemeriden neu manifestieren
```
ephemeris_compiler.rs:460 → samples_x.push(x * 1000.0);  (Fix im Code, Daten noch alt)
/jump/earth → 1.136e8 (km-Skala)   /jump/iss → 1.136e11 (m-Skala, horizons_compiler)
```
Die BSP-basierten Binaries (sun, mercury…neptune, moon, Monde) auf dem CDN tragen km;
horizons-basierte (iss, jwst, sonden, Kleinkörper) tragen m. main.rs rechnet durchgängig
in m (radius_m, alt, c). Compiler ist gefixt (×1000 vor dem Fit) — die CI muss die
ephemeris_*.bin neu kompilieren und auf `ssd.jpl.nasa.gov` re-uploaden; lokale
/tmp/omegaflow_eph_*.bin danach verwerfen. Bis dahin stehen Planeten bei 1/1000 der Distanz.

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

**F39** Zero-BodyProperties für körperlose Körper
```
horizons_compiler.rs:707 → BodyProps { a0_deg: 0.0, radius_m: 0.0, flattening: 0.0, … }
```

**F40** NAIF-ID unbekannt → "unknown"
```
ephemeris_compiler.rs:604 → _ => "unknown"
```

---

### Daten zweiter Klasse (2)

**D07** Kein Oszillator-Cap in Rust
```
index.html:534 → while (c * 32 > maxBuf) c >>= 1;
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
horizons_compiler.rs:19 → BODIES_WITH_MEDIA (23 Einträge)
ephemeris_compiler.rs:20 → BODIES_WITH_MEDIA, all_target_ids
ephemeris_compiler.rs:579 → body_id_to_name Tabelle
```

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
