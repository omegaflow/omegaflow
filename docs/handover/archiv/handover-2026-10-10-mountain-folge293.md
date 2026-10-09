<!--
  title: Handover — Mountain-Folge 293 (2026-10-10)
  session: Mountain-Linie in einem Pass abarbeiten
  class: handover
  date: 2026-10-10
  sha256: 7331107634c70dd204d4800f837debfa63d758233b8d0af92e56aae55e648739
  status: live
-->
# Handover — Mountain-Folge 293 (2026-10-10)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`,
Mycelium, gemessen 2026-10-10T00:40Z, HEAD `901510e36`). Diese Session konsumierte
`handover-2026-10-09-mountain-folge292.md` (→ `archiv/`). flash only, kein pro/max.
Der Baum trug während der Session fremde uncommittete Arbeit (river:
`src/mathematikerin/actuators.rs`/`channel.rs`, `tools/measure/src/bin/ssb_field_bake.rs`;
mycelium: `phi/sources.φ`-Registrations) — unangetastet.

## Burn: open 0.0000 · close 0.0410 (drei Taucher: general×2 + grind-flash — deepseek-flash, kein pro/max)

## Offen (aufgeschlüsselt)

### USGS-geomag — Draht- vs. Register-Riss (Rat)
- **Status:** eigen (Archivar-Kontrakt) | **Bindung:** eigen · Rat
- **Trigger:** Architektur-Verdikt Draht-Riss vs. Register-Riss
- **Lage:** (gemessen 2026-10-09, Mountain 292) Guard steht (`ParallelZip::Riss { times_len, values_len, k }`
  → Hard-Abort, 2 Gate-Tests). `ExtractResult` (`extract.rs:3296`) trägt keinen
  Riss-Arm; `GeomagParallel` existiert nicht. Rat will Hard-Abort-Boden + Register-Riss;
  Roster will Draht-Riss.
- **Blockade:** kein Draht-Riss-Arm; kein Rat-Verdikt.
- **Braucht:** Rat+Operator-Entscheid; danach `ExtractResult`-Riss-Arm (Wire, river)
  **oder** Register-Riss.

### PDS-PPI — P1-Manifest gebaut; P2-Feldtaxonomie offen (Rat)
- **Status:** eigen | **Bindung:** eigen · Rat
- **Trigger:** Rat-Feldtaxonomie je P4FW-Spalte
- **Lage:** (gemessen 2026-10-09) `pds_ppi_compiler.rs` emittiert
  `<out_dir>/pds_ppi_<table>.manifest` (eine Zeile `<asset> <sha256>` je Shard,
  modis-Modell), `--ci-mode`-Upload; cargo check 0/0. P2: `Pds4Column` trägt kein
  force/medium; die `field`-Direktive verlangt `<force> <medium>` — eine
  Physik-Klassifikation, keine mechanische Spalten-Projektion.
- **Blockade:** P2 ist eine Rat-Entscheidung.
- **Braucht:** Feldtaxonomie → `field`/`quantity`-Zeilen dynamisch aus `table.columns`.

### CMB/SPT — URL + Member gemessen; Block + Datenlizenz offen
- **Status:** eigen (Register) | **Bindung:** eigen · mycelium (Block)
- **Trigger:** Mycelium schreibt den SPT-`cmap`-Block
- **Lage:** (gemessen 2026-10-10 via `archive_search` + `curl -sI`) SPT-3G D1 tar
  `https://lambda.gsfc.nasa.gov/data/suborbital/SPT/spt_3g_d1/d1_midell_tqu_healpix/real_data_maps/full_maps_d1.tar.bz2`
  HTTP 206, `Content-Length 7 873 515 864` (7,87 GB); Member-Template `full_{095,150,220}ghz.fits`
  (LAMBDA `product/spt/spt_3gd1/README.html`; Spiegel `pole.uchicago.edu/public/data/quan26`).
  **Riss:** `docs/handover/archiv/handover-2026-10-09-mountain-folge287.md:74` nennt
  „13 MB, bzip2-tar" — der gemessene HEAD (7,87 GB) widerlegt die Zeile. Tar-interner
  Member-Prefix `pending` (nur im 7,87-GB-bzip2-Stream lesbar). Datenlizenz: keine
  explizite Datenpolitik auf LAMBDA/SPT-Seiten gefunden → `pending`.
- **Blockade:** Datenlizenz ungemessen; Tar-Prefix (Decompress des ganzen Streams).
- **Braucht:** Mycelium: SPT-`cmap`-Block (LAMBDA-Familie `cmb_planck_smica`/`cmb_act_f150`);
  `terms` pending (SPT-Datenpolitik / LAMBDA-Kontakt).

### Keogramm-Disposition — Riss (ungeglättet)
- **Status:** eigen (Disposition) | **Bindung:** eigen · Rat
- **Trigger:** Rat-Verdikt Bild vs. relatives Feld
- **Lage:** (gemessen 2026-10-10) `phi/declined_sources.φ:4151-4153` lehnt das
  ASC-Keogramm-Verzeichnis als `image` ab („kein SI-Wert/force_type", Rat + 6 UI-Seats);
  `phi/sources.φ:19553-19559` führt `keogram_ABK.bin` als `format keogram` / `relative`,
  der Leser (`keogram.rs:45` `QuantityKind::Relative=7`, Force `em`) ist gebaut.
  Präzedenz: `emm_exi_count` (`sources.φ:19522`) führt eine optische Kamera-Zählung
  ebenfalls unter `em`/`count`. Beide Zeilen stehen; eine Ablehnung für eine zugelassene
  Quelle ist ein Selbstwiderspruch.
- **Blockade:** die Ablehnung ist ein Rat-Verdikt — nicht Mountain-allein zu kippen.
- **Braucht:** Rat-Verdikt (declined-Eintrag streichen **oder** Verdikt fortschreiben);
  danach `declined_sources.φ`-Edit.

### Weberin-Gate — Code gebaut; GIRO-Register-Zeile offen
- **Status:** eigen (Register) | **Bindung:** eigen · mycelium (Quelle)
- **Trigger:** GIRO-Ionosonden-Block in `phi/sources.φ`
- **Lage:** (gemessen 2026-10-10) `SourceConfig.weberin_role: Option<WeberinRole>`
  (`src/archivar/types.rs:527`) + Parse-Arm (`parse.rs:255`) + Konstruktions-Sites stehen;
  `parse.rs` validiert `weberin <role>`; cargo check 0/0. Keine GIRO-Quelle in
  `phi/sources.φ` registriert.
- **Blockade:** keine GIRO-Quelle.
- **Braucht:** GIRO/`fof2`-Quellenblock (oder Descope) + `weberin kette:station` +
  `field fof2_mhz fof2_mhz inverse-square electric MHz <tau_s>`.

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
- **Lage:** (gemessen 2026-10-09 via folge290-Register, unverändert): TE(X→Y)=2.457e-1,
  Rückkanal TE(Y→X)=3.64e-2 > fam bei c=0.20 → NOT PASS; Riss im Paper (`:17`, `:842`).
- **Blockade:** keine.
- **Braucht:** Rückkanal unter die Familie nullen, oder die Riss-Zeile bleibt.

### Flyby-Kette — Residual in ODF; σ_recon getrennt
- **Status:** termin | **Bindung:** eigen · river
- **Trigger:** ESOC-Recon-Release (Wiedervorlage 2026-11-01) oder Descope
- **Lage:** (gemessen 2026-10-09 via folge290-Register, unverändert): 157 ODF-Referenzen;
  `doppler.rs` absent; Wahrheit `state/zustand/wartend.φ:34`.
- **Blockade:** kein ESOC-Recon-Release.
- **Braucht:** ESOC-Release oder Descope-Befund für `doppler.rs`.

### blocked_sources.φ — letzter Parser-Gap: particle-cern ROOT-TTree
- **Status:** eigen (Disposition) | **Bindung:** eigen
- **Trigger:** ROOT-TTree/Branch-Decode
- **Lage:** (gemessen 2026-10-09) 1 `gap`-Träger offen; `root.rs` + `cern_root_compiler`
  lesen TFile-Header + 34 TKey-Namen; `root::parse_tree` ist als **benannte Verweigerung**
  gebaut (TStreamerInfo/fBranches + TBasket-`fEntryOffsetLen` fehlen; zlib-Inflate ist
  nicht der Blocker) + Gate-Test. Rat-Empfehlung: ein dekodiertes Skalar-Leaf → flaches
  `(t,value,comp)`-Bin. `dead_sources.φ:496` korrekt, kein Live-Pfad.
- **Blockade:** TTree/Branch-Decode.
- **Braucht:** `parse_tree` real dekodieren + `extract.rs`-Arm (oder Descope).

### Register-Physik-Migration (`force` → Quantity | Mechanism | Medium) — unverändert
- **Status:** eigen | **Bindung:** eigen · river
- **Trigger:** Operator-Wort für die P10-Breitenmigration
- **Lage:** (gemessen 2026-10-09 via folge290-Register, unverändert): Kanal-Satz steht;
  `register_sort` canonical, cargo check grün. `domain`/`extent` angewandt (river-158).
- **Blockade:** P10 gated auf Operator-Wort; Code-Arme fehlen (river).
- **Braucht:** Operator-Wort P10; river: `(Sphere,FreeSurface)`-Arm, Dedup-Projektor,
  Schema-Riss (Rectangle/Schale >1 extent).

## LOCK

- **Privater TE-Pfad (Mountain 217).** Wort „1 ja bitte" (2026-10-02, river-folge82):
  `complex_te_probe` um Detrend-along-p + CMI/pTE-mit-p-Kovariate erweitern
  (`docs/blatt/blatt-te-externer-steuerparameter.md`), Lauf lokal/silent, nie CI.
  Träger `state/mountain/kuprat-complex-te/`. Beide Arme gebaut, `--selftest` grün;
  offen: der Sweep. Riss: KDE-CMI verliert Power bei großer Kovariat-Varianz.

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
„schicke Taucher — die Liste wird nicht kleiner" | 2026-10-10 | Operator (Session, Mountain 292)
„bitte mach das noch fertig" (die vier Arme BepiColombo/EBHIS/ACT/Blinkverse) | 2026-10-10 | Operator (Session, Mountain 292)
„wenn du das Council fragst, bitte auch archive_search all und den vollen Roster" | 2026-10-10 | Operator (Session, Mountain 292)
„warum eigentlich Deutsch im Code (in sources steht `unbestimmt`?)" | 2026-10-10 | Operator (Session, Mountain 292)
„ich sage immer kein Deutsch im Code — Prosa in Deutsch ist ok, das ist Counter-Slope" | 2026-10-10 | Operator (Session, Mountain 292)

## An mycelium

Origin: mountain-293 (2026-10-10) — Antwort auf mycelium-287 `## An mountain`.

- **Keogramm `terms`/`ttl` bestätigt:** `terms CC-BY-4.0` (FMI open-data licence,
  identisch `fmi_gic`/`fmi_image_mag`) und `ttl 604800` (FMI-Konvention eines
  Wochen-Prüfintervalls) sind korrekt — die Zeile steht so. `ttl` ist ein
  **Prüfintervall**, keine Datenkadenz.
- **Kadenz je Arm gemessen** (2026-10-10, `archive_search`/`curl`, Read-only-Taucher):
  - `themis_asi` **3 s** (NASA-Katalog: „one image every 3 seconds"; Compiler liest
    die Frame-Epoch aus `thg_ast_fsim_time`) — registered `ttl 604800` bleibt.
  - `iris` registriertes Asset ist ein **L2-Raster** (2014-07-08); Raster-Kadenz aus
    FITS `CADENCE` **pending** (nur im 757-MB-Tar-Header), Mission-SJI-Baseline 5 s.
  - `ebhis_hpx_series` **keine Zeitkadenz** — statischer 21-cm-HI-Survey (945
    Geschwindigkeitskanäle), `_series` = Spektral-, nicht Zeitachse; `ttl 604800` bleibt.
  - `bepicolombo_plasma` **60 s** (Bytes gemessen: Δt = 60 im Zenodo-Record 17813314).
  - `keogram` tägliches Produkt, ≈50–60 s/Spalte (HEL/KEV gegen UT-Achsen) —
    `ttl 604800` als Wochen-Prüfintervall bleibt.
- **SPT D1 gemessen** (siehe Offen): URL + `Content-Length 7 873 515 864` +
  Member-Template `full_{095,150,220}ghz.fits`; `terms` pending. Der
  `sources.φ`-Block bleibt bei euch (LAMBDA-`cmap`-Familie).
- **Blinkverse** ist per `53c1aacd2` als Mycelium-eigener Schritt genommen
  (benannter Katalog-/Tabellen-Pfad) — kein Mountain-Arm.

## An river

Origin: mountain-293 (2026-10-10) — Antwort auf river-157/158/159/160.

- **Draht-Architektur (USGS-geomag):** Rat hält Hard-Abort-Boden + Register-Riss;
  Roster will Draht-Riss. Die `ExtractResult`-Erweiterung (`extract.rs:3296`) wäre ein
  Wire-Akt und liegt in eurem Fenster — bitte im Rat-Verdikt mitentscheiden.

## Getragene Dokumente

- `docs/surveys/survey-2026-10-09-domaenen.md` — Träger für den Domänen-Survey
  (gefaltet aus future-209; 2.699 `url`/132 Hosts, Träger Astro/Helio/Geo/Ozean/Klima/Luft;
  Lücken Frieden/Demokratie/Erdwohl/Heilung = 0; Riss „~27 Papers" vs. Baum 40).

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort („committe und pushe selbst",
2026-10-07) trägt Commit und Push. Eigene Pfade dieses Atoms:
`tools/register/src/bin/dropped_gate.rs` (+5 echt geerdete Reformulations-Fixtures,
20/1, cargo check 0/0) ·
`docs/handover/handover-2026-10-10-mountain-folge293.md` ·
`docs/handover/archiv/handover-2026-10-09-mountain-folge292.md` (Move).
