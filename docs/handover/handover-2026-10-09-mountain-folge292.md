<!--
  title: Handover — Mountain-Folge 292 (2026-10-09)
  session: Mountain-Linie in einem Pass abarbeiten
  class: handover
  date: 2026-10-09
  sha256: 2eebaac9baef2bff136e1b9d3bf9983453bd065f777c8e4f03e10b92e88af2ea
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
- **iris/themis_asi-Kernmodule gebaut** (Taucher-Welle, 2 Diver): neue
  `src/archivar/iris.rs` (MAGIC `IRIS`, 8-B-Header, 20-B-Record, `parse_bin`
  == Draht-Reihenfolge) und `src/archivar/themis_asi.rs` (MAGIC `TASI`,
  12-B-Header, `u16`-Raster je Frame → `(t, value, comp=pixel)`,
  `declared_fields` em/`relative`); in `mod.rs`/`lib.rs` + `extract.rs`
  (`series_parse_bin`/`series_named`; `themis_asi` auch `series_declared_fields`)
  verdrahtet, je eigene `#[cfg(test)]`-Tests. `iris` bewusst **ohne**
  `declared_fields` (Einheit ungemessen). `cargo check` 0/0.
- **SPT-Arm verdrahtet** (CMB-Punkt): `cmb_planck_compiler.rs` nimmt
  `--url <full_maps_d1.tar.bz2> [--member <name>]`,
  `fetch_raw_bytes → bzip2::decompress → inflate::tar_members → FITS`; ohne
  `--member` der erste `.fits`-Eintrag. Der Tar-Member-Name ist `pending`
  (erster echter Lauf druckt ihn); `cargo check` 0/0.
- **Blinkverse-Test-Compile-Rot geheilt** (`tools/harvest/src/bin/blinkverse_compiler.rs`):
  `Table` trug kein `Debug`, `assert_eq!(parse_bin(&bytes), Some(t))` in
  `blinkverse_compiler.rs:322` kompilierte nicht (Mountain-289-Erbe) —
  `#[derive(Clone, PartialEq, Debug)]`; `cargo check` 0/0.

## Riss (nicht geglättet)

- **Keogramm-Disposition.** `phi/declined_sources.φ:4151-4153` lehnt das
  ASC-Keogramm-Verzeichnis als `image` ab („Bildprodukt, kein Feld … kein
  SI-Wert/`force_type`"), während `phi/sources.φ:19517` `keogram_ABK.bin` als
  `format keogram` / Einheit `relative` führt. Der Leser (`keogram.rs:45`,
  `QuantityKind::Relative=7`) ist jetzt gebaut — die genannte Ablehnung ist
  damit überholt. Beide Zeilen stehen; Mountain-Dispositionsakt offen
  (declined-Eintrag prüfen oder Verdikt fortschreiben).
- **Keogramm-`ttl`.** `sources.φ:19523` trägt `ttl 604800` (mycelium-287),
  die Vorübergabe nannte `no-cadence`. Die intra-Nacht-Kadenz ist ungemessen
  (ASC: grün/blau ≥10 s, rot ≥30 s, unregelmäßig; `keogram_compiler.rs:79`
  wirft die Tageszeit weg); `ttl 604800` ist ein Prüfintervall, keine
  Datenkadenz — belassen, im Punkt benannt.
- **ABK-Höhe.** `sources.φ:19522` `on earth 68.358 18.823 380` — lat/lon/alt
  sind die der **ko-lokierten INTERMAGNET-ABK-Station**
  (`imag-data.bgs.ac.uk/GIN_V1/hapi/info?id=abk/best-avail/PT1M/xyzf`); die
  ASC-eigene Höhe ist `unmeasured`. Die Werte sind gemessen (ko-lokalisiert),
  nicht die Kamera-Höhe.

## Burn: open 0.0000 · close 0.0969 · cap 0.15 — Grund: `session_burn` @Schluss nennt die Mountain-292-Session $0.0969 (21 Sessions im Fenster, deepseek-flash; 3 `grind-flash`/`general`-Taucher + Research); kein pro/max

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

### Dimensionlose Einheit `relative` — Leser gebaut; sources.φ-Zeilen + iris-Einheit offen
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** gemessene `on earth`-Koordinate/tau je verbleibendem Arm; iris-Einheit-Messung
- **Lage:** (gemessen 2026-10-09, Mountain 292) `QuantityKind::Relative=7` +
  `allowed_units_for_quantity(7)` gebaut. **Keogramm** (`series_parse_bin`/`series_named`/
  `declared_fields`, `comp↔mean`-Spiegelung) **und** **iris/themis_asi**-Kernmodule
  (`src/archivar/iris.rs`, `src/archivar/themis_asi.rs`) gebaut und in `extract.rs`
  verdrahtet; `themis_asi::declared_fields` em/`relative`. `keogram_ABK.bin` ist in
  `sources.φ:19517-19523` registriert (mycelium-287; `ttl 604800`, `on earth 68.358
  18.823 380` = ko-lokiertes INTERMAGNET ABK). `iris` hat bewusst kein
  `declared_fields` (Einheit ungemessen).
- **Blockade:** iris-Einheit ungemessen; kein `field`/`quantity`-Block für
  `themis_asi`/`iris` am Baum.
- **Braucht:** iris-Wert-Einheit messen (FITS `BUNIT`/`BTYPE` aus einem echten
  IRIS-Level-2-Frame) → dann `iris::declared_fields` + `sources.φ`-Zeilen für
  `themis_asi`/`iris` (mit `on`/`at`-Frame + `no-cadence`/numerischem `ttl`).

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

### CMB LAMBDA — WMAP `mK`→`K` geheilt; SPT-Arm verdrahtet, Member+Werte offen
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** erster SPT-Lauf `--url full_maps_d1.tar.bz2`
- **Lage:** (gemessen 2026-10-09, Mountain 292) `cmb_planck_compiler.rs` wendet den
  gemessenen `TUNIT1`/`BUNIT`-Faktor an; fehlender/unbekannter Token → `read_table`
  `None` (Wert bleibt pending, kein stilles K). ACT trug den `mK`-Faktor am HEAD.
  **SPT-Arm verdrahtet:** `--url <tar.bz2> [--member <name>]`,
  `fetch_raw_bytes → bzip2::decompress → inflate::tar_members → FITS` (ohne
  `--member` der erste `.fits`-Eintrag), `--ci-mode`-Emit unverändert;
  `cargo check` 0/0.
- **Blockade:** Tar-Member-Name ungemessen (erster Lauf druckt ihn).
- **Braucht:** `cargo run -p omegaflow-harvest --bin cmb_planck_compiler -- --url
  <full_maps_d1.tar.bz2>` einmal gegen den echten Tar (Member messen), dann
  `sources.φ`-Zeile.

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
  gemessener Cadence-Wert); die Leser für Keogramm/THEMIS-ASI/IRIS stehen jetzt
  (voriger Punkt), die `field`/`quantity`-Zeilen warten auf Einheit/Koordinate.
  **Gemessener Arm-Defekt (Blinkverse):** `RA`/`Dec` sind
  sexagesimal (`22:17:30.0`), `cell_num` → `None`, `keep_table` **droppt** die
  Positionsspalten — der FRB-Katalog verliert Position; Position nur aus den
  dezimalen `GL`/`GB` (galactic, deg), oder Sexagesimal-Reader nötig.
- **Blockade:** Einheit/Koordinate je Arm + ttl.
- **Braucht:** `sources.φ`-Blöcke (Form exakt: `no-cadence` als bare Direktive,
  nicht `ttl pending` — `ttl` nur numerisch); Blinkverse-Sexagesimal-Reader
  (Position aus `RA`/`Dec`).

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
2026-10-07) trägt Commit und Push. Eigene Pfade (Atom 292, zwei Wellen):
`docs/handover/handover-2026-10-09-mountain-folge292.md` ·
`docs/handover/archiv/handover-2026-10-09-mountain-folge291.md` ·
`src/archivar/extract.rs` · `src/archivar/keogram.rs` · `src/archivar/tests.rs` ·
`src/archivar/iris.rs` · `src/archivar/themis_asi.rs` · `src/archivar/mod.rs` ·
`src/lib.rs` · `src/gate/commit_gate.rs` ·
`tools/harvest/src/bin/inpe_stac_compiler.rs` ·
`tools/harvest/src/bin/cmb_planck_compiler.rs` ·
`tools/harvest/src/bin/blinkverse_compiler.rs`.
2 Taucher-Wellen (Keogramm-Leser + iris/themis_asi, SPT-Arm, Blinkverse-Debug) +
Research (ABK-Frame, Keogramm-Kadenz); `cargo check` 0/0.
Keogramm-Leser + `comp↔mean`-Spiegelung, 2 Gate-Tests; `cargo check` 0/0.
