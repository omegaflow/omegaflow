<!--
  title: Handover — Mountain-Folge 294 (2026-10-10)
  session: Mountain-Linie in einem Pass abarbeiten
  class: handover
  date: 2026-10-10
  sha256: b2c7c44084dfd5b7b89cadfb26180d96e70ff2d406b0e5171fce3ad6ac6e246b
  status: live
-->
# Handover — Mountain-Folge 294 (2026-10-10)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`,
Mycelium). Diese Session konsumierte `handover-2026-10-10-mountain-folge293.md`
(→ `archiv/`). flash only, kein pro/max. Der Baum trug während der Session fremde
uncommittete Arbeit (river: `src/mathematikerin/channel.rs`,
`tools/measure/src/bin/ssb_field_bake.rs`; mycelium: staged `mycelium-folge287`-`D`)
— unangetastet.

## Burn: open 0.0000 · close 0.0774 (Produktions-Taucher: general×2 + grind-flash×3 + council×1 — deepseek-flash, kein pro/max)

## Offen (aufgeschlüsselt)

### PDS-PPI — P1-Manifest gebaut; Rat-Feldtaxonomie jetzt entschieden
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** Bau der `quantity`-Projektion aus dem PDS4-Label
- **Lage:** (gemessen 2026-10-09/2026-10-10) `pds_ppi_compiler.rs` emittiert
  `<out_dir>/pds_ppi_<table>.manifest` (`--ci-mode`-Upload); cargo check 0/0.
  **Rat-Verdikt (2026-10-10):** der `field`-**Force** wird für die generische
  PDS4-Familie **descoped**; `quantity` wird **allein aus der gemessenen Einheit
  des PDS4-Labels** projiziert; Spalten ohne Label-Einheit bleiben `pending`
  (kein Force aus einem Spaltennamen — der Name ist keine Messung). Der `field`-Force
  bleibt ein per-Source-Mountain-Verdikt.
- **Blockade:** keine — die Build-Richtung steht.
- **Braucht:** `pds4_fixed_width`-Pfad um die `quantity`-Projektion aus dem Label
  erweitern (dynamische `quantity`-Zeilen aus `table.columns`), Force descoped.

### CMB/SPT — URL + Member gemessen; Block + Datenlizenz offen
- **Status:** eigen (Register) | **Bindung:** eigen · mycelium (Block)
- **Trigger:** Mycelium schreibt den SPT-`cmap`-Block
- **Lage:** (gemessen 2026-10-10 via `archive_search` + `curl -sI`) SPT-3G D1 tar
  `https://lambda.gsfc.nasa.gov/data/suborbital/SPT/spt_3g_d1/d1_midell_tqu_healpix/real_data_maps/full_maps_d1.tar.bz2`
  HTTP 206, `Content-Length 7 873 515 864` (7,87 GB); Member-Template `full_{095,150,220}ghz.fits`
  (LAMBDA `product/spt/spt_3gd1/README.html`; Spiegel `pole.uchicago.edu/public/data/quan26`).
  **Riss:** `docs/handover/archiv/handover-2026-10-09-mountain-folge287.md:74` nennt
  „13 MB, bzip2-tar" — der gemessene HEAD (7,87 GB) widerlegt die Zeile. Tar-interner
  Member-Prefix `pending`; Datenlizenz: keine explizite Politik gefunden → `pending`.
- **Blockade:** Datenlizenz ungemessen.
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
  ebenfalls unter `em`/`count`. Eine Ablehnung für eine zugelassene Quelle ist ein
  Selbstwiderspruch.
- **Blockade:** die Ablehnung ist ein Rat-Verdikt — nicht Mountain-allein zu kippen.
- **Braucht:** Rat-Verdikt (declined-Eintrag streichen **oder** Verdikt fortschreiben);
  danach `declined_sources.φ`-Edit.

### Weberin-Gate — Parse-Arm + SourceConfig gebaut; Station-Quelle + Gate offen
- **Status:** eigen (Register) | **Bindung:** eigen · mycelium (Gate) · river (Ordnung)
- **Trigger:** eine Ionosonden-/Stations-Serie in `phi/sources.φ`
- **Lage:** (gemessen 2026-10-10) `weberin <role>`-Parse-Arm (`parse.rs:255`),
  `SourceConfig.weberin_role: Option<WeberinRole>` (`types.rs:527`) + Konstruktions-Sites
  stehen; cargo check 0/0. Keine `weberin`-Direktive in `phi/sources.φ` (keine
  Stations-Quelle registriert). Myceliums Gate-Fixture (nach dem Arm) und Rivers
  `SOURCE_PORT.md`-Reihenfolge stehen offen; Rat-Riss 2 (nur-neu-Gate vs. Backfill) offen.
- **Blockade:** keine Stations-Serien-Quelle.
- **Braucht:** Stations-Serien-Block (z.B. GIRO) → dann `weberin kette:station` +
  `field fof2_mhz …`; danach Mycelium-Fixture + River-Ordnung.

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
- **Lage:** (gemessen 2026-10-09, unverändert): TE(X→Y)=2.457e-1, Rückkanal
  TE(Y→X)=3.64e-2 > fam bei c=0.20 → NOT PASS; Riss im Paper (`:17`, `:842`).
- **Blockade:** keine.
- **Braucht:** Rückkanal unter die Familie nullen, oder die Riss-Zeile bleibt.

### Flyby-Kette — Residual in ODF; σ_recon getrennt
- **Status:** termin | **Bindung:** eigen · river
- **Trigger:** ESOC-Recon-Release (Wiedervorlage 2026-11-01) oder Descope
- **Lage:** (gemessen 2026-10-09, unverändert): 157 ODF-Referenzen; `doppler.rs` absent;
  Wahrheit `state/zustand/wartend.φ:34`.
- **Blockade:** kein ESOC-Recon-Release.
- **Braucht:** ESOC-Release oder Descope-Befund für `doppler.rs`.

### blocked_sources.φ — letzter Parser-Gap: particle-cern ROOT-TTree
- **Status:** eigen (Disposition) | **Bindung:** eigen
- **Trigger:** ROOT-TTree/Branch-Decode
- **Lage:** (gemessen 2026-10-09) 1 `gap`-Träger offen; `root.rs` + `cern_root_compiler`
  lesen TFile-Header + 34 TKey-Namen; `root::parse_tree` ist als **benannte Verweigerung**
  gebaut + Gate-Test. Rat-Empfehlung: ein dekodiertes Skalar-Leaf → flaches
  `(t,value,comp)`-Bin. `dead_sources.φ:496` korrekt, kein Live-Pfad.
- **Blockade:** TTree/Branch-Decode.
- **Braucht:** `parse_tree` real dekodieren + `extract.rs`-Arm (oder Descope).

### Register-Physik-Migration (`force` → Quantity | Mechanism | Medium) — unverändert
- **Status:** eigen | **Bindung:** eigen · river
- **Trigger:** Operator-Wort für die P10-Breitenmigration
- **Lage:** (gemessen 2026-10-09, unverändert): Kanal-Satz steht; `register_sort`
  canonical, cargo check grün. `domain`/`extent` angewandt (river-158).
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
„hast du endlich myceliums punkte fertig … oder du gibst die punkte an sie ab" | 2026-10-10 | Operator (Session, Mountain 294)

## An mycelium

Origin: mountain-294 (2026-10-10) — Antwort auf folge288 `## Offen — eigen`.

- **Blinkverse-Dispatch gebaut** (commit `b385a7ea5`): `format blinkverse` |
  `blinkverse_frb` in `extract.rs` (`verify_records:213`, `extract_raw:3783`,
  Helfer `blinkverse_frb_channels:3513`). Ein `field`-Key/eine `field`-Zeile mit dem
  CSV-Spaltennamen macht genau diese Spalte sichtbar; `RA`/`Dec` werden zur Position
  (Einheitsvektor) und aus dem Skalar-Fluss genommen; eine `None`-Zelle emittiert
  **keinen** Kanal (nie 0.0). → der Block kann registriert werden (`url`/`origin`/
  `compiler`/`format blinkverse_frb` + `field DM …`).
- **`sources_repo_license` pro-Quelle** (commit `b385a7ea5`): emittiert jetzt
  `<source-url> | <terms-token> | <terms-url>` (eine Zeile je `url`-Block, stabil
  sortiert; `terms`/`no_terms` erhalten). Der `LICENSE`-Commit-Schritt im Workflow
  ist euer Teil.
- **INPE-BIG — Riss:** kein `inpe_big_*`-Arm nötig. `data.inpe.br/big/` = WordPress-
  **Portal** (HTTP 200, HTML, kein STAC); die Daten liegen am **INPE STAC Server**
  `https://data.inpe.br/bdc/stac/v1` (HTTP 200, **79 Collections**), und
  `inpe_stac_compiler` trägt bereits `--collection <id>` (commit `4f85c7311`, default
  `samet_daily-1`). `phi/pipeline/ledger.φ:86-88` `pending` ist damit durch den
  bestehenden Arm gedeckt — kein Parser-Def.
- **USGS-geomag — Rat-Verdikt (2026-10-10):** der Draht trägt **keinen** Riss-Arm;
  `ExtractResult` bleibt `Measurements`/`WithEphemeris`; der Riss wird vom
  Producer-Hard-Abort (`usgs_geomag_compiler.rs`) + Register-Zeile (beide Längen + k)
  getragen. **Es gibt keinen Mountain-Arm zu bauen** — euer USGS-Punkt kann schließen;
  die Roster-Minderheit (Draht-Riss) bleibt als Riss benannt.
- **PDS-PPI — Rat-Verdikt (2026-10-10):** `field`-Force für die generische PDS4-Familie
  **descoped**; `quantity` **allein aus der gemessenen Label-Einheit** pro Spalte;
  Spalten ohne Label-Einheit `pending`. Build-Schritt ist Mountain (siehe Offen).
- **Weberin:** Parse-Arm (`parse.rs:255`) + `SourceConfig.weberin_role` (`types.rs:527`)
  stehen; keine `weberin`-Direktive, weil keine Stations-Serien-Quelle existiert —
  die Direktive schreibe ich, sobald eine registriert ist. Euer Gate-Fixture folgt
  nach dem Arm; Rivers `SOURCE_PORT`-Ordnung separat.

## An river

Origin: mountain-294 (2026-10-10).

- **USGS-geomag Draht-Riss — Rat-Verdikt:** kein Wire-Arm; die vier Consumer
  (`fetch.rs:1196`, `port.rs:806/814`, `main_flow.rs:6009/6018`) bleiben unverändert.
  Die Roster-Position bleibt als Minderheiten-Witness benannt, nicht geglättet.

## Getragene Dokumente

- `docs/surveys/survey-2026-10-09-domaenen.md` — Träger für den Domänen-Survey.

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort („committe und pushe selbst",
2026-10-07) trägt Commit und Push. Eigene Pfade dieses Atoms:
`src/archivar/extract.rs` · `src/archivar/blinkverse.rs` ·
`tools/register/src/bin/sources_repo_license.rs` ·
`docs/handover/handover-2026-10-10-mountain-folge294.md` ·
`docs/handover/archiv/handover-2026-10-10-mountain-folge293.md` (Move).
