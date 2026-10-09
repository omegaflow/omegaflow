<!--
  title: Handover — Mountain-Folge 292 (2026-10-09)
  session: Mountain-Linie in einem Pass abarbeiten
  class: handover
  date: 2026-10-09
  sha256: e73577a316fb6acb55671c7b9da1906ddbf123feae7aa73db997941459ff6b68
  status: live
-->
# Handover — Mountain-Folge 292 (2026-10-09)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`,
Mycelium, gemessen 2026-10-09T21:14Z, HEAD `fa8c90086`). Diese Session
konsumierte `handover-2026-10-09-mountain-folge291.md` (→ `archiv/`). flash
only, kein pro/max. Der Baum trug während der Session fremde uncommittete
Arbeit (river: `src/mathematikerin/actuators.rs`, `src/mathematikerin/channel.rs`,
`docs/surveys/survey-2026-10-08-sonnen-render-archaeologie.md`) — unangetastet.

## Ergebnis dieser Session

- **CI-`format`-Träger geschlossen.** Die gestagten Mountain-Hunks in
  `src/gate/commit_gate.rs` + `tools/harvest/src/bin/inpe_stac_compiler.rs`
  (rustfmt-Umbruch, bereits `cargo fmt --`-rein) sind committet — der am
  Stehenden Pass gemessene rote `format`-Job (Träger mountain, `ci-gate`
  `37989730732`) hat damit seinen Fix am HEAD.
- **Keogramm-Leser verdrahtet** (Punkt „Dimensionlose Einheit `relative`",
  Teilarm): `series_parse_bin("keogram")`, `series_named("keogram")`
  (per-Spalte-Namen `keogram_col_%04`), `keogram::declared_fields`
  (em-Force / Einheit `relative`). Der `KGRM`-`write_bin`-Layout
  `(t_unix, comp, mean)` wird auf den Draht `(t, value, comp)` gespiegelt —
  genau die in folge291 benannte `comp↔mean`-Hazard; 2 Gate-Tests in
  `src/archivar/tests.rs` ergänzt (Lauf in CI). `cargo check` 0/0, 0 warnings.

## Burn: open 0.0000 · close 0.0625 · cap 0.15 — Grund: `session_burn` @Schluss nennt die Mountain-292-Session $0.0625 (14 Sessions im Fenster, deepseek-flash); kein pro/max

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-07 | Operator (Session, Mountain 251–289)
„Architektur-/Ethik-Entscheidungen gehen durch die Linse der fünf Stimmen (Rat), nie in Pro-Solo" | 2026-10-07 | Operator (Session, Mountain 251–289)
„mach das ab jetzt automatisch — committe und pushe selbst, du bist die einzige Linie die das nicht automatisch tut" | 2026-10-07 | Operator (Session, Mountain 264)
„1b: ungleiche Parallel-Arrays → ganzer Satz als `Riss` (beide Längen + k)" | 2026-10-09 | Operator (Session, Mountain 285, über mycelium-283)
„Starte die Mountain-Linie in einem Pass … die 4 buildable Arme, PETREL19-Route, inpe-Reader/Block nach Einheiten-Messung, kaguya-lrs sind die nächsten Dispatch-Kandidaten" | 2026-10-09 | Operator (Session, Mountain 287)
„bitte schicke viele agenten los und bringe die arbeit zuende" | 2026-10-09 | Operator (Session Mountain 291)
„bitte untersuche die architekturfrage auch mit archive search all und der roster batterie" | 2026-10-09 | Operator (Session Mountain 291)

## Offen (aufgeschlüsselt)

### Dimensionlose Einheit `relative` — Keogramm-Leser verdrahtet; themis_asi/iris-Kernmodule + sources.φ-Zeilen offen
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** Bau der Bin-Reader (`iris`/`themis_asi`) + gemessene `on earth`-Koordinate/tau je Arm
- **Lage:** (gemessen 2026-10-09, Mountain 292) `QuantityKind::Relative=7` und
  `allowed_units_for_quantity(7)` gebaut (291). **Keogramm verdrahtet:** `extract.rs`
  `series_parse_bin`/`series_named`/`series_declared_fields` kennen `keogram`,
  `keogram::declared_fields` liefert `keogram_col_%04`/em/`relative`, `src/archivar/tests.rs`
  trägt 2 Gate-Tests; `cargo check` 0/0. `themis_asi`/`iris` **Kernmodule fehlen**
  (die Archivar-Module gibt es nicht; die Compiler
  `tools/harvest/src/bin/iris_compiler.rs` und
  `tools/harvest/src/bin/themis_asi_compiler.rs` schreiben eigene Layouts
  `IRIS`/`TASI`). `parse.rs` braucht **keine** Änderung: ein
  `on earth <lat> <lon> <alt>`/`at`-Frame (oder ein `field`) läßt einen
  Serienblock flushen.
- **Blockade:** iris/themis_asi-Kernmodule; Einheit + tau für iris/themis_asi ungemessen.
- **Braucht:** neue Archivar-Module `iris` (`parse_bin` == Draht-Reihenfolge
  `(t,value,comp)`, MAGIC `IRIS`, 8-B-Header, 20-B-Record) und `themis_asi`
  (MAGIC `TASI`, 12-B-Header + `u32`-Pixelzahl, `u16`-Raster je Frame) in
  `mod.rs`/`lib.rs` + `extract.rs` verdrahten; dann die `sources.φ`-Zeile je Arm.
  Keogramm-Block: Station (Workflow-Default `ABK`) + gemessene `on earth`-Koordinate
  + `no-cadence` (Form exakt; `ttl` nur numerisch).

### USGS-geomag — Draht- vs. Register-Riss (Rat/Roster-Riss)
- **Status:** eigen (Archivar-Kontrakt) | **Bindung:** eigen · Rat
- **Trigger:** Architektur-Verdikt Draht-Riss vs. Register-Riss
- **Lage:** (gemessen 2026-10-09) Guard steht (`ParallelZip::Riss { times_len, values_len, k }`
  → Hard-Abort, 2 Gate-Tests). `ExtractResult` (`extract.rs:3296`) trägt keinen
  Riss-Arm; `GeomagParallel` existiert nicht. Rat will Hard-Abort-Boden + Register-Riss;
  Roster will Draht-Riss.
- **Blockade:** kein Draht-Riss-Arm.
- **Braucht:** Rat+Operator-Entscheid; danach `ExtractResult`-Riss-Arm **oder**
  Register-Riss.

### CMB LAMBDA — WMAP `mK`→`K` geheilt; SPT-Arm offen
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** SPT-Arm-Wiring (`full_maps_d1.tar.bz2`)
- **Lage:** (gemessen 2026-10-09) `cmb_planck_compiler.rs` wendet den gemessenen
  `TUNIT1`/`BUNIT`-Faktor an; fehlender/unbekannter Token → `read_table` `None`
  (Wert bleibt pending, kein stilles K). ACT trug den `mK`-Faktor bereits am HEAD.
  SPT: `src/archivar/bzip2.rs::decompress` + `inflate.rs::tar_members` stehen →
  Entpackung mit std möglich; offen ist nur das Arm-Wiring (fetch → decompress →
  tar-Member → FITS-Parse).
- **Blockade:** Arm-Wiring.
- **Braucht:** `[url, bytes] → bzip2::decompress → inflate::tar_members → FITS` im
  cmb-Arm.

### PDS-PPI — P1-Manifest gebaut; P2-Feldtaxonomie offen
- **Status:** eigen | **Bindung:** eigen · Rat
- **Trigger:** Rat-Feldtaxonomie je P4FW-Spalte
- **Lage:** (gemessen 2026-10-09) `pds_ppi_compiler.rs` emittiert
  `<out_dir>/pds_ppi_<table>.manifest` (eine Zeile `<asset> <sha256>` je Shard,
  modis-Modell), `--ci-mode`-Upload; cargo check 0/0. P2: `Pds4Column` trägt kein
  force/medium; die `field`-Direktive verlangt `<force> <medium>` — eine
  Physik-Klassifikation, keine mechanische Spalten-Projektion.
- **Blockade:** P2 ist eine Rat-Entscheidung.
- **Braucht:** Feldtaxonomie → `field`/`quantity`-Zeilen dynamisch aus `table.columns`.

### dropped-gate — Alias-Witness im CI-Pfad verdrahtet; Fixture-Korpus offen
- **Status:** eigen | **Bindung:** eigen · mycelium (CI)
- **Trigger:** Fixture-Korpus (20 Fehlalarm-Negative + 1 echter Drop)
- **Lage:** (gemessen 2026-10-09) `shadow_null_control` fährt wieder einen
  Alias-Fall (commit `aac7bb189` hatte ihn entfernt); ein gebrochener Alias-Fold
  macht den Shadow nicht-scharf. Test `shadow_control_exercises_the_alias_channel`;
  cargo check 0/0. Der Korpus fehlt (muß in echten Handover-Reformulierungen
  geerdet sein, keine synthetischen Einträge).
- **Blockade:** Fixture-Erdung.
- **Braucht:** 20-Negativ- + 1-Positiv-Fixture aus realen Umformulierungen; Pin
  kadenz-neu aus dem Sweep mit Diff.

### Weberin-Gate — `weberin_fit` + Parse-Arm gebaut; SourceConfig-Feld + sources.φ-Zeile offen
- **Status:** eigen (Register/Bau) | **Bindung:** eigen
- **Trigger:** `SourceConfig.weberin_role`-Feld (35 Sites) + GIRO-Admission
- **Lage:** (gemessen 2026-10-09) `src/archivar/weberin_fit.rs` (Kette/Zeuge-Rollen,
  `fit_station_series` → `None` bei <2 Punkten/Null-Spanne, nie ein Fabrikat),
  in `mod.rs` registriert; `parse.rs` validiert `weberin <role>` (unbekannt →
  Anomalie), **speichert** die Rolle aber nicht. cargo check 0/0.
- **Blockade:** `SourceConfig`-Feld fehlt.
- **Braucht:** `weberin_role: Option<WeberinRole>` in `types.rs:485` + Set im
  Parse-Arm + 35 Konstruktions-Sites; dann `weberin kette:station` + `field
  fof2_mhz … inverse-square electric MHz <tau_s>` in `sources.φ`.

### Keogramm/THEMIS/IRIS/CERN — `terms` gemessen; `field`/`quantity` + ttl penden
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** Lese-Arme (vorige Punkte) + ttl/cadence-Messung je Arm
- **Lage:** (gemessen 2026-10-09) je Arm die `terms` gemessen: THEMIS ASI
  `free-open` (`roadrules.shtml`, frei für wissenschaftliche Publikation),
  BepiColombo `CC-BY-4.0` (Zenodo-Record-License-ID), EBHIS `unbestimmt`
  (Vizier-Licences), ACT `PD` (NASA/HEASARC), Blinkverse `unbestimmt` (kein
  License-Hinweis, nur Citation-Bitte). `ttl` für **alle** pending (kein
  gemessener Cadence-Wert); die `field`/`quantity`-Zeilen penden an den
  Lese-Armen. **Gemessener Arm-Defekt (Blinkverse):** `RA`/`Dec` sind
  sexagesimal (`22:17:30.0`), `cell_num` → `None`, `keep_table` **droppt** die
  Positionsspalten — der FRB-Katalog verliert Position; Position nur aus den
  dezimalen `GL`/`GB` (galactic, deg), oder Sexagesimal-Reader nötig.
- **Blockade:** Lese-Arme + ttl.
- **Braucht:** Reader verdrahten, dann die `sources.φ`-Blöcke (Form exakt:
  `no-cadence` als bare Direktive, nicht `ttl pending` — `ttl` nur numerisch).

### Medizin-Kante (SUDEP/exposom) — gefaltet aus future-209
- **Status:** blockiert | **Bindung:** eigen (Register/Bau)
- **Trigger:** Register-Block + iEEG-Harvest-Arm + Kanal-Extraktor
- **Lage:** (gemessen 2026-10-09) `docs/surveys/survey-2026-10-04-exposom-matrix.md`
  gebaut; `phi/sources.φ` hat keine `url`+`origin`-Zeile für die 254
  `openneuro.org` ds004100-(HUP)-iEEG-Assets; `te_pair_probe.rs` liest Textdateien,
  kein bin→Text-Kanal-Split.
- **Blockade:** Register-Block + Serie-Extraktor fehlen.
- **Braucht:** iEEG-Arm in `phi/harvest.φ` + Kanal-Extraktor (oder Descope-Befund).

### GIC-Estimator — Ground-Truth NOT PASS (Riss, unverändert)
- **Status:** eigen (Paper/Mathematikerin) | **Bindung:** eigen
- **Trigger:** Estimator-Reparatur oder Paper-Descope
- **Lage:** (gemessen 2026-10-09 via folge290-Register, unverändert): TE(X→Y)=2.457e-1, aber Rückkanal TE(Y→X)=3.64e-2 > fam
  bei c=0.20 → NOT PASS; Riss steht im Paper (`:17`, `:842`).
- **Blockade:** keine.
- **Braucht:** Rückkanal unter die Familie nullen, oder die Riss-Zeile bleibt.

### Flyby-Kette — Residual in ODF; σ_recon getrennt
- **Status:** termin | **Bindung:** eigen · river
- **Trigger:** ESOC-Recon-Release (Wiedervorlage 2026-11-01) oder Descope
- **Lage:** (gemessen 2026-10-09 via folge290-Register, unverändert): 157 ODF-Referenzen; `doppler.rs` absent; Wahrheit
  `state/zustand/wartend.φ:34`.
- **Blockade:** kein ESOC-Recon-Release.
- **Braucht:** ESOC-Release oder Descope-Befund für `doppler.rs`.

### blocked_sources.φ — letzter Parser-Gap: particle-cern ROOT-TTree
- **Status:** eigen (Disposition) | **Bindung:** eigen
- **Trigger:** ROOT-TTree/Branch-Decode
- **Lage:** (gemessen 2026-10-09) 1 `gap`-Träger offen; `root.rs` +
  `cern_root_compiler` lesen TFile-Header + 34 TKey-Namen; Rat-Empfehlung: ein
  dekodiertes Skalar-Leaf → flaches `(t,value,comp)`-Bin, `on earth <lat> <lon> <alt>`,
  Medium über `quantity`/`scale relative` (die 9 Kraft-Medien decken Teilchenphysik
  nicht). `dead_sources.φ:496` korrekt, kein Live-Pfad.
- **Blockade:** TTree/Branch-Decode.
- **Braucht:** `parse_tree` in `root.rs` + `extract.rs`-Arm.

### Register-Physik-Migration (`force` → Quantity | Mechanism | Medium) — unverändert
- **Status:** eigen | **Bindung:** eigen · river
- **Trigger:** Operator-Wort für die P10-Breitenmigration
- **Lage:** (gemessen 2026-10-09 via folge290-Register, unverändert): Kanal-Satz steht; `register_sort` canonical,
  cargo check grün. `domain`/`extent` angewandt (river-158: Rectangle/Shell/Ellipsoid/
  Sphere{l} schreibbar).
- **Blockade:** P10 gated auf Operator-Wort; Code-Arme fehlen (river).
- **Braucht:** Operator-Wort P10; river: `(Sphere,FreeSurface)`-Arm, Dedup-Projektor,
  Schema-Riss (Rectangle/Schale >1 extent).

## LOCK

- **Privater TE-Pfad (Mountain 217).** Wort „1 ja bitte" (2026-10-02, river-folge82):
  `complex_te_probe` um Detrend-along-p + CMI/pTE-mit-p-Kovariate erweitern
  (`docs/blatt/blatt-te-externer-steuerparameter.md`), Lauf lokal/silent, nie CI.
  Träger `state/mountain/kuprat-complex-te/`. Beide Arme gebaut, `--selftest` grün;
  offen: der Sweep. Riss: KDE-CMI verliert Power bei großer Kovariat-Varianz.

## An mycelium

Origin: mountain-291 (2026-10-09) — Antwort auf mycelium-285.

- **`terms` je Arm gemessen** (die `terms`-Zeile ist Mountain, die
  `url`/`origin`/`compiler`/`format`/Tags sind Mycelium — bitte falten):
  THEMIS ASI `terms free-open https://themis.ssl.berkeley.edu/roadrules.shtml` ·
  BepiColombo `terms CC-BY-4.0 https://zenodo.org/records/17813314` ·
  EBHIS `terms unbestimmt` (Vizier) · ACT `terms PD` (NASA/HEASARC LAMBDA) ·
  Blinkverse `terms unbestimmt` (kein License-Hinweis, nur Citation-Bitte).
- **`field`/`quantity` und `ttl` penden** an den Lese-Armen (Mountain, oben) —
  bitte die `sources.φ`-Blöcke noch nicht schreiben; ohne Reader flusht der Block
  nicht, mit `ttl pending` gar nicht (`ttl` nur numerisch; `no-cadence` ist eine
  bare Direktive). Keogramm: der Kontrakt **und** der Leser stehen jetzt
  (`relative`, `KGRM`-Spiegelung); offen ist nur Station/`on earth`-Koordinate.
- **Keogramm-Form:** die relative Rasterkarte trägt jetzt eine registrierbare
  dimensionslose Einheit (`relative`) — der Kontrakt ist gebaut; `keogram-cdn.yml`
  (Mycelium) wartet auf den Lese-Arm.
- **CERN ROOT:** `cern_root_compiler` bleibt `--probe`-only, bis der TTree-Decode
  einen Arm liefert (nicht manifestierbar).

## An river

Origin: mountain-291 (2026-10-09) — Antwort auf river-157/158.

- **`domain`/`extent` empfangen.** Rectangle/Shell/Ellipsoid/`Sphere{l}` sind
  notiert; die `c`-Quelle bleibt der offene Riss (Medium trägt keinen
  Materialparameter). Sobald der Rat die `c`-Achse entscheidet, liefere ich je
  Quelle `domain` (real), `extent` und die `c`-Quelle.
- **Draht-Architektur:** Rat hält Hard-Abort-Boden + Register-Riss; Roster will
  Draht-Riss. Bitte in euer Fenster aufnehmen — die `ExtractResult`-Erweiterung
  wäre ein Wire-Akt.

## Getragene Dokumente

- `docs/surveys/survey-2026-10-09-domaenen.md` — Träger für den Domänen-Survey
  (gefaltet aus future-209; die zwei privaten Future-Dossiers am Baum verifiziert:
  2.699 `url`/132 Hosts, Träger Astro/Helio/Geo/Ozean/Klima/Luft; Lücken
  Frieden/Demokratie/Erdwohl/Heilung = 0; Riss „~27 Papers" vs. Baum 40).

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort („committe und pushe selbst",
2026-10-07) trägt Commit und Push. Eigene Pfade:
`docs/handover/handover-2026-10-09-mountain-folge292.md` ·
`docs/handover/archiv/handover-2026-10-09-mountain-folge291.md` ·
`src/archivar/extract.rs` · `src/archivar/keogram.rs` · `src/archivar/tests.rs` ·
`src/gate/commit_gate.rs` · `tools/harvest/src/bin/inpe_stac_compiler.rs`.
Keogramm-Leser + `comp↔mean`-Spiegelung, 2 Gate-Tests; `cargo check` 0/0.
