<!--
  title: Handover — Mountain-Folge 296 (2026-10-10)
  session: Mountain-Linie in einem Pass abarbeiten
  class: handover
  date: 2026-10-10
  sha256: 4ffb9bd27ce8339568ec4f2bf011021d68bf8feeae576814489ce57fd7adf5ab
  status: live
-->
# Handover — Mountain-Folge 296 (2026-10-10)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`). Diese
Session konsumierte `handover-2026-10-10-mountain-folge295.md` (→ `archiv/`).
flash only, kein pro/max. Der Baum trug fremde uncommittete Arbeit
(`tools/measure/src/bin/ssb_field_bake.rs`) — unangetastet.

## Burn: open 0.0000 · close 0.0438 — gemessen via session_burn (Linien-Session $0.0438; Taucher-Burn — grind-flash×2 + general×2 — noch nicht in der Session-Zeile aggregiert)

## Offen (aufgeschlüsselt)

### P10 Register-Physik — `gravity` migriert; `last`-Arm + Regime-Achse bei river
- **Status:** eigen | **Bindung:** river
- **Trigger:** die nächste Gruppe (`seismic`)
- **Lage:** (gemessen 2026-10-10) `tools/register/src/bin/p10_gravity_migrate.rs`
  hat 100 `field … gravity` → `quantity … geometry`, 5 → `quantity … source-parameter`
  umgetaggt; `register_sort` kanonisch (2708), `cargo check` 0/0. **Riss:** die 4
  `last`-Zeilen (`hydrosphere_river_stage_m`, `hydrosphere_tide_water_level_m`,
  `gracefo_kbr_range_rate_m_s`, `gracefo_kbr_range_accl_m_s2`) brauchen einen
  `last … <kind>`-Arm; river-161 hat `parse.rs` ohne diesen Arm committet.
- **Blockade:** `last`-Quantity-Arm fehlt.
- **Braucht:** river: `last`-Quantity-Arm in `parse.rs`; dann die 4 Zeilen migrieren.
  Danach Gruppe `seismic` (dann `acoustic`/`diffusion`, `em` zuletzt).

### P10 Regime-Achse — `<regime>`-Token benannt verweigert; kein Descriptor-Slot
- **Status:** eigen | **Bindung:** river
- **Trigger:** eine `Regime`-Achse im `ChannelDescriptor`
- **Lage:** (gemessen 2026-10-10, unverändert river-161) der optionale `<regime>`-Token
  wird bei `field` benannt verweigert (`parse.rs:1249`), nicht still verworfen. Der
  Deskriptor braucht Conserved + Boundary + Regime (quasi-statisch/strahlend).
- **Blockade:** die Regime-Achse existiert nicht.
- **Braucht:** river: `Regime`-Achse + Token-Auflösung.

### CMB/SPT — `terms` gemessen (LAMBDA no-restrictions); Block = Mycelium
- **Status:** eigen (Register) | **Bindung:** mycelium (Block)
- **Trigger:** Mycelium schreibt den SPT-`cmap`-Block
- **Lage:** (gemessen 2026-10-10) LAMBDA/AWS-Registry „no restrictions"; NASA-weit CC0
  (`science.data.nasa.gov/about/license`); LAMBDA `contact/` erbittet Acknowledgement
  (keine Bedingung).
- **Blockade:** keine (Mountain-Seite geschlossen).
- **Braucht:** Mycelium: SPT-`cmap`-Block (`terms CC0-1.0 https://lambda.gsfc.nasa.gov/contact/`).

### IRIS — Rohpixel, kein Feld; BUNIT-Riss benannt
- **Status:** eigen (Register) | **Bindung:** mycelium (Block)
- **Trigger:** Rat-Verdikt zum `count`-Token / sources.φ-Block
- **Lage:** (gemessen 2026-10-10) `iris_compiler.rs` liest kein BUNIT; `src/archivar/iris.rs:6`
  `UNIT="count"` ist eine Baum-Konstante, nicht die Quelle (BUNIT `Corrected DN`). Riss:
  mycelium-288/289 lesen „BUNIT count gemessen" — das ist die Konstante. Rohe DN ist kein
  physikalisches Feld → keine `field`/`quantity`-Zeile; der `count`-Token ist zu beheben.
- **Blockade:** kein Descriptor für rohe DN.
- **Braucht:** Mycelium: IRIS sources.φ-Zeile + Workflow; die `UNIT`-Konstante klären.

### GIRO DIDBase — Compiler gebaut; Manifestation = Mycelium
- **Status:** eigen | **Bindung:** mycelium (Block)
- **Trigger:** Mycelium schreibt Workflow + sources.φ-Block
- **Lage:** (gemessen 2026-10-10) Compiler gebaut und grün: `src/archivar/giro_fastchar.rs`
  (MAGIC `GIFC`) + `tools/harvest/src/bin/giro_fastchar_compiler.rs`; Endpoint
  `https://lgdc.uml.edu/fastchar/getbest` (HTTP 200, `text/plain`, kein Key); realer Lauf
  `--station JR055 --start 2012-07-02T21:00:00Z --stop 2012-07-03T03:00:00Z` → 24 Records,
  Roundtrip 608 B; Lizenz CC BY-NC-SA 4.0 (`giro.uml.edu/didbase/RulesOfTheRoad.html` 206);
  `cargo check` 0/0, Bin baut.
- **Blockade:** keine (Mountain-Seite).
- **Braucht:** Mycelium: `giro-fastchar-cdn.yml` + harvest.φ-Arm (`format giro_fastchar`)
  + sources.φ-Block (`url github.com/omegaflow/sources/releases/download/lgdc.uml.edu/giro_fastchar.bin`,
  `terms CC-BY-NC-SA-4.0 https://giro.uml.edu/didbase/RulesOfTheRoad.html`,
  `origin https://lgdc.uml.edu/fastchar/getbest`, `compiler tools/harvest/src/bin/giro_fastchar_compiler.rs`,
  `at earth`, `ttl`, `field atmosphere_ionosonde_fof2_mhz … mhz`).

### Keogramm ABK — Quelle saisonal; Workflow-Ziel falsch (Mycelium)
- **Status:** eigen | **Bindung:** mycelium
- **Trigger:** Mycelium-Workflow-Fix
- **Lage:** (gemessen 2026-10-10) FMI `ABK.2604` letzte echte Nacht 2026-04-21 (2605/2606
  leer, 2607–2610 404); SGO `emCCD_ABK` Index lebt (jüngste echte Nacht 2026-04-21);
  `keogram-cdn 37995952959` rot auf `ABK.2610` (404). SGO-Keogramm-JPG ist Bildprodukt
  (declined), der ABK-Index läuft über FMI.
- **Blockade:** keine.
- **Braucht:** Mycelium: `keogram-cdn.yml` auf die jüngste verfügbare ABK-Nacht zielen.

### blocked_sources.φ — particle-cern ROOT-TTree: Gap gemessen (Object-Streamer)
- **Status:** eigen (Disposition) | **Bindung:** eigen
- **Trigger:** ROOT-TTree/Object-Streamer-Decode
- **Lage:** (gemessen 2026-10-10) Der Gap ist der ROOT-**Object-Streamer**
  (`TStreamerInfo`/`TBuffer` + `TBranch`/`TLeaf`/`TBasket`; `root.rs:222` benannte
  Verweigerung), **nicht** die Kompression: zlib steht (`archivar::inflate`, `hdf4.rs:887`).
  Header/TKey/`--probe` (`cern_root_compiler`) ist der erreichbare Teil und steht. 1
  abhängige Quelle (`blocked_sources.φ:64`), 0 Consumer; Rat (folge291): die 9 Kraft-Medien
  decken Teilchenphysik nicht. Riss: das gap-Token `particle-cern` mischt die ATLAS-URL
  (`opendata.atlas.cern`) mit dem ALICE-Sample (`AliVSD_Masterclass_6.root`). `note` auf
  den gemessenen Gap aufgefrischt.
- **Blockade:** TTree/Branch-Decode (Object-Streamer fehlt global); nur flash verfügbar.
- **Braucht:** Object-Streamer bauen (`TBuffer` + version-pinned 53006 `TBranch`/`TLeaf`/
  `TBasket`) + `extract.rs`-Arm — hartes Atom; oder Operator/Council-Verdikt, ob der
  `parser-def`-Zustand bleibt (Feststellung: `blocked` ist korrekt, das Format ist
  unlesbar; die Decline-Vokabel hat kein Token „Parser außer Scope").

### GIC-Estimator — finite-sample-Artefakt; Riss im Paper getragen
- **Status:** eigen (Paper/Mathematikerin) | **Bindung:** eigen
- **Trigger:** Estimator-Härtung (CI-Probe) oder Thread-Descope
- **Lage:** (gemessen 2026-10-10) `te_ground_truth.rs:17-19` ist streng unidirektional →
  der wahre Rückkanal ist 0; `TE(Y→X)=3.64e-2 > fam 2.405e-2` ist ein **finite-sample
  Artefakt** bei starkem Coupling (n-abhängig, `docs/paper/gic-causal-driver.md:241-251`),
  kein physikalischer Riss. Das Paper trägt den Riss (`:17`/`:842`) und den offenen Thread
  (`:706-711`/`:821-823`); der Silverman-KDE-Schätzer ist die kanonische Referenz.
- **Blockade:** kein Härtungs-Arm gemessen; `fam` im Ground-Truth-Bin ist noch das
  Round-Maximum, nicht das studentisierte Westfall–Young-max-T.
- **Braucht:** CI-Probe `cargo run -p omegaflow-measure --bin te_bias_n_probe -- --estimator embedded`
  (c=0.2, n=10000) → nullt der embedded/KSG-Arm den Rückkanal, Ground-Truth auf den
  embedded-Arm umstellen; sonst bleibt der Riss (kein `pass`-Edit ohne Messung).

### Flyby-Kette — Residual in ODF; σ_recon getrennt
- **Status:** termin | **Bindung:** termin:2026-11-01
- **Trigger:** ESOC-Recon-Release (oder Descope)
- **Lage:** (gemessen 2026-10-09, unverändert) 157 ODF-Referenzen; `doppler.rs` absent;
  Wahrheit `state/zustand/wartend.φ:34`.
- **Blockade:** kein ESOC-Recon-Release.
- **Braucht:** ESOC-Release oder Descope-Befund für `doppler.rs`.

### PDS-PPI — `quantity`-Projektion gebaut; Block = Mycelium
- **Status:** eigen | **Bindung:** mycelium (Block)
- **Trigger:** Mycelium schreibt den sources.φ-Block
- **Lage:** (gemessen 2026-10-10) `units.rs` + `pds_ppi_compiler.rs` emittieren je Spalte
  eine `quantity`-Zeile aus dem gemessenen PDS4-`<unit>`; Spalten ohne Label-Einheit
  bleiben `pending`.
- **Blockade:** keine.
- **Braucht:** Mycelium: PDS-PPI-Block (manifest + `quantity`-Zeilen).

## LOCK

- **Privater TE-Pfad (Mountain 217).** Wort „1 ja bitte" (2026-10-02, river-folge82):
  `complex_te_probe` um Detrend-along-p + CMI/pTE-mit-p-Kovariate erweitern; Lauf
  lokal/silent, nie CI. Träger `state/mountain/kuprat-complex-te/`. Beide Arme gebaut,
  `--selftest` grün; offen: der Sweep. Riss: KDE-CMI verliert Power bei großer
  Kovariat-Varianz.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„warum schreibst du hier erst in die blocked sources anstatt direkt an den finalen ort — das ist einfach nur faules compliance theater" | 2026-10-10 | Operator (Session, Mountain 295)
„Offen bleibt in P10: der optionale `<regime>`-Token (wird derzeit benannt verweigert — keine Descriptor-Achse dafür), die Regime-Achse selbst, und die Zeilen-Migration in phi/sources.φ (Mountain)." | 2026-10-10 | Operator (Session, Mountain 295)
„Starte die Mountain-Linie in einem Pass — kein Planungstheater … kein Consent-Stopp für Bekanntes" | 2026-10-07 | Operator (Session, Mountain 251–290)
„mach das ab jetzt automatisch — committe und pushe selbst" | 2026-10-07 | Operator (Session, Mountain 264)

## An mycelium

Origin: mountain-296 (2026-10-10).

- **GIRO DIDBase — Manifestation.** Der Compiler steht (`src/archivar/giro_fastchar.rs`,
  `tools/harvest/src/bin/giro_fastchar_compiler.rs`), gemessen grün (JR055 → 24 Records).
  Der sources.φ-Block wird allein von euch geschrieben (Manifestations-Direktiven). Block:
  `url …/lgdc.uml.edu/giro_fastchar.bin`, `terms CC-BY-NC-SA-4.0
  https://giro.uml.edu/didbase/RulesOfTheRoad.html`, `origin https://lgdc.uml.edu/fastchar/getbest`,
  `compiler tools/harvest/src/bin/giro_fastchar_compiler.rs`, `at earth`, `ttl`,
  `field atmosphere_ionosonde_fof2_mhz`; dazu `giro-fastchar-cdn.yml` + `format giro_fastchar`-Arm.
- **CMB/SPT** — terms gemessen (CC0/no-restrictions, LAMBDA/AWS): der SPT-`cmap`-Block
  kann geschrieben werden (`terms CC0-1.0 https://lambda.gsfc.nasa.gov/contact/`).
- **Medizin-Kante (iEEG)** — Mountain-Seite geschlossen: `eeglab::eeg_from_bin` akzeptiert
  `Samples::Double`. Offen bei euch: `phi/harvest.φ`-iEEG-Arm (`--dataset ds004100`,
  `^sub-HUP…_ieeg\.bin$`) + `sources.φ`-Block.
- **Keogramm** — `keogram-cdn.yml` zielt auf `ABK.2610` (404); FMI endet `ABK.2604`
  (2026-04-21), SGO-Index lebt. Auf die jüngste verfügbare Nacht zielen.

## An river

Origin: mountain-296 (2026-10-10).

- **P10 `gravity`-Migration** ist gelaufen (`field` → `quantity geometry`/`source-parameter`,
  100+5 Zeilen). **Riss:** 4 `last`-Zeilen (river_stage/tide_water/range_rate/range_accl)
  trägt der Klassifikator als geometry, aber der `last`-Arm kennt nur eine force — es
  fehlt ein `last … <kind> …`-Arm (`parse.rs`). Bis dahin bleiben sie `gravity`.
- Der optionale `<regime>`-Token + die `Regime`-Achse bleiben bei euch (P10 braucht den
  vollständigen Deskriptor: Conserved + Boundary + Regime).

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort („committe und pushe selbst",
2026-10-07) trägt Commit und Push. Eigene Pfade dieses Atoms:
`src/archivar/giro_fastchar.rs` · `src/archivar/mod.rs` ·
`tools/harvest/src/bin/giro_fastchar_compiler.rs` · `phi/blocked_sources.φ` ·
`docs/handover/handover-2026-10-10-mountain-folge296.md` ·
`docs/handover/archiv/handover-2026-10-10-mountain-folge295.md` (Move).
