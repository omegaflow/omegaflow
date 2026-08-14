# TODO

AGENTS.md is the primary constraint matrix. Git is the history.
Kanonisch: Diese Datei ist das vollständige Register der offenen Arbeit. Erledigtes
wird entfernt (Git trägt es). Kein Eintrag meldet Erledigtes als offen, kein offener
Punkt fehlt. Widerspricht ein Dokument dieser Datei, gilt diese Datei — solche
Drift-Stellen sind unter „Doku-Drift" registriert.

## Stand — 2026-08-14 (LSK/PCK-Hochzeit, Binary v2, Protokoll v6, reine Per-Pixel-Membran)

Zeit aus naif0012.tls (LSK-Reader, keine TT_MINUS_UTC-Konstante). PCK-Reader pck.rs
(gm_de440, pck00010, geophysical.ker → GM/J2/J4/Radii/POLE). stype-1 v2: gcount=12,
96 B festes Stride; non-v2 → None (Körper absent). WS-Protokoll v6: 19-B-Header
(0xCF 0x86 0x06 + response_epoch f64 + id u32 + count u32), Record 168 B / 21 f64
(+ pole, j2, j4, r_eq); JS field 12 + meta 12 floats, WGSL field[j*3]/props[j*3].
body_channels sendet ausschließlich {body}.mass (GM, Kernel 0, gravity) und
{body}.radius. fs evaluiert osc_field für jeden Pixel (kontinuierliche Mathematik,
keine Stützstellen, keine Interpolation); Nebra-Rampe t2 = clamp((log2(Ω_total)+14)/22, 0, 1)
(4 mix()-Segmente Blau→Cyan→Orange→Weiß) IST die Tonemap; Ω_total = Σ|omegas[k]|.
Exposure-Kette getilgt (keine lvls, keine e/E). SSAA als dichtere Messung: q/Q in
Φ-Schritten (1.00–8.00, Start 1.00×), Canvas-Backing skaliert, CSS nativ.
28 lokale v2-Binaries unter /tmp/omegaflow_eph_*.bin (Meter, verifiziert).
0 Warnings, 0 Errors; Tests: 5 lib + 20 bin.

---

### Einheiten & Ephemeriden (3)

**U01** CDN-Ephemeriden neu manifestieren (v2 + m-Skala)
```
Compiler schreiben v2 (gcount=12, ×1000); 28 Binaries lokal kompiliert und verifiziert.
Der CDN-Release trägt noch gcount=0-Legacy (km) → Runtime gibt None → Körper absent
(0 honored) bis Re-Upload. Kein CI-Workflow aktiv (siehe I02).
```

**U02** Monde (io…triton) ohne Ephemeriden
```
de440s.bsp trägt keine Mond-Segmente → jup365/sat441/mar097/ura111/nep095.bsp
herunterladen, mit ephemeris_compiler kompilieren (v2), nach /tmp/omegaflow_eph_*.bin.
Bis dahin: Monde absent (0 honored). Deckt sich mit I03 (Kernel-Flattener).
```

**U03** J2/J4 für weitere Körper: nur Erde hat Text-Quelle
```
geophysical.ker trägt BODY399_J2/J4 → Erde hat echte Harmonische (j2=1.082616e-3
im v6-Record verifiziert). Weitere Körper: Werte liegen nur in binären .bpc-Kerneln
→ Binary-PCK-Reader (DAF-Segment-Typ 2; daf.rs existiert als Basis). Bis dahin:
j2/j4 = 0.0 neutral (0 honored).
```

---

### Zentrismus (3)

**Z03** Sonne (NAIF 10) spezialbehandelt im Ephemeriden-Compiler
```
ephemeris_compiler.rs:457 → if granules.is_empty() && target_id != 10 {
```

**Z04** Ephemeris-Kanäle hardcoded auf gravity
```
main.rs:5255, 5336, 5381 → tau: 86400.0 * 365.0, kernel: 0, force: 1
```

**Z08** Device-Daten verworfen ohne Körper-Ephemeriden
```
main.rs /device-Pfad (~2954) → st_lat/st_lon/st_alt-Gate ohne eph
```

---

### Hack (2)

**H11** Hot-Path-Clones: `all.push(s.clone())` jeden Tick
```
main.rs:8111 → all.push(s.clone());
```

**H12** `query_hash` klont Zellen-Referenzen
```
main.rs:732 → out.push(samples);
```

---

### Fabrikation (3)

**F33** Geschwindigkeit `[0.0, 0.0, 0.0]` — CelestialPolygon
```
main.rs:5766 → v: [0.0, 0.0, 0.0],
```

**F35** State-Vector hardcoded gravity
```
main.rs:5255 → kernel: 0, force: 1, tau: 86400.0 * 365.0, (drei Stellen, siehe Z04)
```

**F40** NAIF-ID unbekannt → "unknown"
```
ephemeris_compiler.rs:350 → _ => "unknown"
```

---

### Daten zweiter Klasse (2)

**D07** Kein Oszillator-Cap in Rust
```
index.html:542 → while (c * 96 > maxBuf) c >>= 1;
```

**D08** `/device` scannt nur Device-Quellen
```
main.rs:3099 → if matches!(osc.source, OscillatorSource::Device)
```

---

### Bias (2)

**B05** ISS bekommt spezielles Datenfenster
```
horizons_compiler.rs:715 → let months = if *name == "iss" { 0.9 } else { 1.0 };
```

**B06** Hardcodierte Body-Listen in Compilern
```
Verbleibend: horizons_compiler.rs wgccre_for_body-Zwillingstabelle (→ PCK-Hochzeit,
wgccre in ephemeris_compiler ist bereits gelöscht), ephemeris_compiler.rs
all_target_ids + body_id_to_name Tabelle (→ NAIF-ID-Mapping vervollständigen, F40).
```

---

### Parser & Spec (8 — aus PARSER_MAGIC.md, PARSER_EVALUATION_MATRIX.md, SOURCES_V2_SPEC.md §10)

**P01** Tote Grammatik wird still akzeptiert
```
main.rs:4419 ("force"-Arm) und main.rs:4246 (3-Token-"field" setzt τ=0) parsen alte
Blöcke fehlerfrei → stille 0 Oszillatoren (0 honored, aber ohne sichtbares Signal).
Entscheidung: laut ablehnen (Refused) oder Migration mit Lautsignal. Betrifft
archeology/sources/* (alte force-Grammatik) und phi/research/batches/* (source-Köpfe).
```

**P02** SI-Konvertierung + Unit-Kraft-Matrix
```
convert_to_si + allowed_units_for_force (PARSER_EVALUATION_MATRIX.md); inkl.
mag → W/m² (SUNSPOTS-Rest: Sterne blieben). SOURCES_V2_SPEC §10: „units are
documentation slots" — heute roher Durchfluss.
```

**P03** `z`-Redshift-Key für cmap + per-row τ-Override + vel-Einheitenkonvertierung
```
SOURCES_V2_SPEC.md §10: z_key (Hubble-Flow), τ-Override je Zeile, vel m/s fix.
```

**P04** kepler_map-Bahnlöser fehlt
```
Extract existiert, Kepler-Gleichungs-Bahnrechnung fehlt (PARSER_MAGIC.md,
gegen Code verifiziert).
```

**P05** HorizonsVec: 0B-Fetches + falsche Timestamps
```
PARSER_MAGIC.md — Horizons-Text-Extract produziert leere/zeitversetzte Samples.
```

**P06** cmap: plx/pmra/radvel-Füllung
```
PARSER_MAGIC.md — Parallaxe/Eigenbewegung/Radialgeschwindigkeit ungenutzt.
```

**P07** extent-pro-Force
```
PARSER_MAGIC.md — gravity → body_radius statt c·τ als Extent-Herleitung.
```

**P08** `field_in` nested + Flatten-Extract
```
PARSER_MAGIC.md / EXTRACT_TYPES.md — geschachtelte Feldpfade und Flatten-Variante.
```

---

### Infrastruktur (4)

**I01** Universal Anomaly Reporter
```
PARSER_EVALUATION_MATRIX.md — GitHub-Issue via gh; Kategorien: Physics Mismatch,
API Unreachable, Empty, Malformed, Invalid.
```

**I02** CI-Dual-Mode: Rust, nicht Python
```
AGENTS.md formuliert „same binary runs in CI (CDN-write)" als Ist — real laufen
generate-ephemerides.yml + refresh-protected-data.yml in Python, mirror-cdn.yml
cron disabled. Offen: --ci-mode/CDN-Upload in Rust, Python-Exzision, mirror-cdn
aktivieren. Vorlage: archeology/ci/*.yml + archeology/ci/secrets.template.
```

**I03** CI-Kernel-Flattener (NAIF)
```
KERNEL CURATION & CI AUTOMATION PLAN: Planetary/Satellites/Asteroids/Spacecraft-Kernel
→ CDN, kernel_flatten.yml, sources_index.φ, NAIF-ID-Mapping. Deckt U02.
```

**I04** Auth-APIs
```
AUTH_APIS.md: 25 Secrets offen (Platzhalter in .secrets.local), Auth-Header-Support
im Fetch-System, Priorität-A-Quellen nach phi/sources.φ.
```

---

### Membran & Wahrnehmung (8)

**M01** WebSerial flow-Protokoll
```
Zwei Spezifikationen konsolidieren: 4D-MEMBRANE.md (`flow <force_name> <force_id>
<|Ω|> 1 <tick_ms> <t> <x> <y> <z>`) vs. docs/omegaflow_sense_hardware.yaml
(`flow <channel> <mode> <value> <unit> <duration_ms> <t> <x> <y> <z>`).
```

**M02** ESP32-Mantis-Shrimp-Firmware
```
docs/omegaflow_sense_hardware.yaml existiert (35 Sensoren/Aktuatoren, Pin-Map,
Safety-Matrix). Offen: no_std-Rust-Firmware; Browser-Seite (actuate) + M01.
```

**M03** Audio-Gain ohne tanh
```
index.html: windowMedianExtent() → tanh(Ω·median) — Median mit ∞-Extents ungelöst;
Normalisierung auf die reine Messung steht aus.
```

**M04** Navigation: Wheel-Divisor + Initial-Scale (Nebra-Kalibrierung)
```
index.html:1194 → gridStep /= 2^(deltaY/128) (Hauptpfad; Touch-Pfad nutzt 512).
Initial-Scale: `gridStep = 2**31` (index.html:121) → 2³⁷.
```

**M05** Device-Sensoren als SI-4-Token
```
RADIATOREN.md: recordSample(name, value, force, unit) + convert_to_si im Archivar
(Mikrofon→Pa, Kamera→lx, Accelerometer→m/s², Magnetometer→µT). „biotic" kollidiert
mit der Force-Registry — klären.
```

**M06** Wetterstation-Debug-Konsole
```
WETTERSTATION.md: Konsole als 4-Token-Spiegel `name [force, unit]: SI-Wert`;
Frontend-Lookup + SI-Anzeige fehlen.
```

**M07** Command Palette ⌘K
```
SEARCH_COMMAND-PALETTE.md: SIMBAD-TAP-Objektsuche (Presence-Jump), lokaler
Source-Index, Force-Filter, Fuse.js, 3 Phasen.
```

**M08** Horizons-Zwillingstabelle löschen + stype-4-Nutation
```
wgccre_for_body in horizons_compiler (siehe B06) → PCK; Nutationsreihen als
stype-4-Sektion im Binary + body_pole_at-Nutzung (nut_ra/nut_dec stehen bereit).
```

---

### Doku-Drift (6 — Wahrheitspflicht: Docs an den Code)

**D-D1** BINARY_PROTOCOL.md neu schreiben (v6)
```
Real: 19-B-Header (0xCF 0x86 0x06, response_epoch f64, id u32, count u32),
Record 168 B / 21 f64, field 12 + meta 12 floats, WGSL field[j*3]/props[j*3].
Dokumentiert: v0x02, 80 B, 10 f64, 11-B-Header. response_epoch ist nirgends dokumentiert.
```

**D-D2** AGENTS.md-Protokoll-Absatz korrigieren
```
„96 bytes, 12 × f64", „0xCF 0x86 0x04" → 168 B, 21 f64, 0x06. (0xCF 0x86 0x01 ist
nur der Ephemeriden-Binary-Header.)
```

**D-D3** FORCE_SYSTEM.md + README.md angleichen
```
9 Kräfte inkl. biotic; Verweise auf phi/forces.φ + phi/units.φ (existieren nicht)
→ SOURCES_V2_SPEC als Quelle; BodyProperties-Liste ohne gm/j2/j4/radii/nut.
```

**D-D4** Curation-Pfad-Drift
```
source_curation.md + AGENTS.md verweisen auf phi/research/pre-cdn-history/
UNTESTED_blocks.φ — Pfad existiert nicht mehr. Reale Untested-Listen:
archeology/sources/sources_*_untested_* (873 + 30 + 16 + 3 Blöcke). Index
rekonstruieren oder Docs umhängen.
```

**D-D5** AGENTS-CI-Behauptung angleichen
```
„same binary code runs in CI (CDN-write)" ist Ziel, nicht Ist → als Ziel kennzeichnen
(siehe I02) oder umsetzen.
```

**D-D6** Duplikate auflösen
```
docs/dual_mode_architecture.md = docs/plans/dual_mode_architecture.md (byte-identisch);
docs/source_curation.md = docs/plans/source_curation.md; docs/concepts/WGSL SHADER =
docs/concepts/WGSL_ SHADER.md. Je ein Exemplar behalten (AGENTS-Verweis beachten).
```

---

### Curation & Quellen

- `phi/research/agent_output/batch_*_accepted.φ` (32 Dateien, neue Grammatik):
  konvertierte Blöcke, die nie nach phi/sources.φ übernommen wurden.
- `archeology/sources/sources_gold_pre-cdn_27k_359-domains.φ` (2572 Blöcke, alte
  force-Grammatik) + `sources_recovery_pre-cdn_25k_211-domains.φ` (1924): Migration
  nach Protokoll (docs/source_curation.md); siehe P01.
- `sources_new_untested_14k_new-unchecked.φ` (873), `sources_astro_untested_*` (30),
  `sources_exotic_untested_*` (16, ohne force → Gate), `sources_earth_untested_*` (3).
- `sources_recovery_cdn-merged_60k_lost-blocks.φ` (5701 urls, 0 field-Tokens):
  Extract-Parameter aus history/recovery-Dateien zuordnen.
- `phi/research/batches/` (283) + `probe_batches/` (242): alte Grammatik bzw.
  `source`-Köpfe — nicht ladbar (P01).
- `archeology/arena/` (batch_01–21): API-Vorschläge als Fließtext, ungeprüft.
- `scripts/ARCHIVED/` + `scripts/__pycache__/`: historische Python (migrate/verify)
  — nur als Vorlage.
- `archeology/foundation/`: ALIGNMENT_PROTOCOL.md (in AGENTS eingebettet),
  APIs.md/collection.md/gaps.md (Curation-Inventar), sources.φ.spec (tote Grammatik).
- `archeology/reconstruction/*.bak`: Vor-v6-Versionen — als Data-Contract-Referenz
  brüchig, nur Historie.
- `archeology/failed_eph_rust/`: abgelöst durch src/ephemeris_compiler.rs + bsp_reader/pck.

---

### Validation

- `--verify` CLI existiert (URL-Erreichbarkeit); lädt noch keine Quellen.
- Tote Tokens (`force`, 3-Token-`field`, `field_in`, `pos`) werden laut P01 still
  geparst — Lautablehnung fehlt.
- Test-Limit der Curation über 200 Blöcke hinaus erhöhen; 6 Rest-FAILs sind
  Daten-Artefakte (docs/source_curation.md).

---

### CI Pipeline

- `mirror-cdn.yml`: cron disabled → I02.
- `generate-ephemerides.yml` (Python/spiceypy) + `refresh-protected-data.yml`
  (Python inline) → Rust-Rewrite (I02).
- CDN-Asset-Naming: `{name}.json` (clobber) vs. AGENTS-Konvention
  `{prefix}_{iso8601}.json` — Konvention ist der Resolver, Naming weicht ab.

---

### Feature Backlog

- Advective per-Quelle: Wind in tm.w (Kanal verdrahtet, Messquelle fehlt).
- OPeNDAP-Integration.
- Kepler-Bahnlöser (P04), HorizonsVec-Fix (P05), Flatten (P08), field_in nested (P08).
- Command Palette (M07).
- Minkowski 4D Weighting (spacelike→0; kosmisches Skalenproblem: Sonne wäre
  spacelike — scale-Anpassung nötig; MINKOWSKI_FIELD-PERMEABILITY.md).
- Camera: ~19k Pixel-Quellen (4×4-Raster) → WS-Traffic-Hotspot.
- `sensor_config`/`probe_classify`-τ/TTL-Konstanten (60/300/0.01/3600) ohne Herleitung.
- `getRto` min/max 100/5000 ms — nicht 2ⁿ/Φ-hergeleitet (constants.js).
- Probe: `coordinates.2` als alt vs. Tiefe bei Seismik — Vorzeichen offen.

---

### Deferred

- Temporal Topology (TDA, Takens, Transfer Entropy, Surrogates) — LOST_CONCEPTS.md.
- Field Permeability (tanh(vC/g)-Variante ohne TE) — MINKOWSKI_FIELD-PERMEABILITY.md.
- Forschungs-Iterationen (Council): Backend als „langsamer Prior" für Exposure-Kaltstart
  (aktuell: fixe Rampe, keine Anpassung); Exposure-EMA auf dem Silizium (gegenstandslos
  solange die Rampe fix ist).
- Future: Aggregation of Presence, Retro-Manifestation, Nostr-Stationsweb
  (kollidiert mit LOST_CONCEPTS-Entscheidung) — FUTURE_CONCEPTS.md.

---

### Rejected

- Unknown-Force soft fallback → Parser lehnt unbekannte Kraft ab.
- Default τ-Werte → Gate schließt, wenn nicht deklariert.
- World Bank Indicators → forceless, DROP.
- Yahoo Finance → forceless, DROP.
- Hexagon-Grid, Quadtree-AMR, temporale Akkumulation, Blue-Noise-Rieseln,
  Nahfeld-Splitting → Interpolations-/Zeit-Lügen (Council-Urteil, WGSL_ SHADER.md).
