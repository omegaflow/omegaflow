<!--
  title: Handover — Mountain-Folge 289 (2026-10-09)
  session: Mountain-Linie in einem Pass abarbeiten
  class: handover
  date: 2026-10-09
  sha256: 1d0d658947555f6ee6a5262c1cd0f0486a8990e8f3557be9e790817bfea5b0f4
  status: live
-->
# Handover — Mountain-Folge 289 (2026-10-09)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`,
Mycelium, gemessen 2026-10-09T18:37Z — am älteren HEAD `00b008452`, vor dem
Kaguya-Fix). Diese Session konsumierte `handover-2026-10-09-mountain-folge288.md`
(→ `archiv/`). flash only, kein pro/max.

## Burn: open 0.0000 · close 0.0540 · cap 0.15 — Grund: Kaguya-CI-Fehllage geheilt (Case-Bug), Keogramm-Form-Verdikt; `session_burn` bei Schluss nennt die Mountain-Linie $0.0540 (total $0.9188 / 31 Sessions, deepseek-flash), flash only, kein pro/max

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-07 | Operator (Session, Mountain 251–289)
„Architektur-/Ethik-Entscheidungen gehen durch die Linse der fünf Stimmen (Rat), nie in Pro-Solo" | 2026-10-07 | Operator (Session, Mountain 251–289)
„mach das ab jetzt automatisch — committe und pushe selbst, du bist die einzige Linie die das nicht automatisch tut" | 2026-10-07 | Operator (Session, Mountain 264)
„1b: ungleiche Parallel-Arrays → ganzer Satz als `Riss` (beide Längen + k)" | 2026-10-09 | Operator (Session, Mountain 285, über mycelium-283)
„Starte die Mountain-Linie in einem Pass … die 4 buildable Arme, PETREL19-Route, inpe-Reader/Block nach Einheiten-Messung, kaguya-lrs sind die nächsten Dispatch-Kandidaten" | 2026-10-09 | Operator (Session, Mountain 287)

## Offen (aufgeschlüsselt)

### Kaguya/SELENE LRS — Case-Bug geheilt, Fix im Lauf verifiziert, Re-Dispatch
- **Status:** wartend (CI) | **Bindung:** eigen
- **Trigger:** Lauf `kaguya-lrs-cdn.yml` (#37981227493) grün, 00S-Assets manifestiert
- **Lage:** (gemessen 2026-10-09 via `ci_manage log 37979918649`) der Fix `9b1911eb7` ist **im Lauf verifiziert**: `pds3_binary_lrs_sw_wf_00n_007080e.bin` packt mit sha256 `772e51d1…` (identisch zur bestehenden Register-Zeile) und `pds3_binary_lrs_sw_wf_00s_007080e.bin` erstmals mit sha256 `8c681469…` (1971 rows). Der Lauf #37979918649 wurde mid-way **cancelled** (nicht failure) → erneut dispatcht #37981227493. Ursache des vorigen Lauf #37978167263: `collect_pairs` lowercased den Daten-Stem (`lrs_sw_wf_*e.tbl` → 404, DARTS case-sensitiv), das Label blieb korrekt.
- **Blockade:** DARTS-Host flappt 200/503 (`curl --retry` fängt 5xx)
- **Braucht:** Lauf #37981227493 grün → neue 00S-Asset-Namen als `sources.φ`-Zeilen + sha256 aus dem Release-Digest nachtragen (00S `8c681469…` steht schon gemessen).

### Keogramm (FMI MIRACLE) — Arm gebaut, Form-Verdikt + Manifestation offen
- **Status:** eigen (Register/Manifestation) | **Bindung:** eigen · mycelium (Workflow/CDN)
- **Trigger:** Form-Verdikt + `sources.φ`-Zeile + `keogram-cdn.yml` + `harvest.φ` Arm
- **Lage:** (gemessen 2026-10-09) `src/archivar/keogram.rs` (`KGRM`-Bin, `mean` je Spalte, Presence-Bit) und `tools/harvest/src/bin/keogram_compiler.rs` (`--station --start --stop` → FMI-JPEGs → Spalten) stehen; **kein** Workflow, **kein** `harvest.φ`-Arm, **keine** `sources.φ`-Zeile (gemessen via `sgrep`).
- **Blockade:** keine.
- **Braucht:** Form-Verdikt (siehe An mycelium) → `keogram-cdn.yml` + `harvest.φ` `asset present`-Eintrag + `sources.φ` `quantity`-Zeile.

### USGS-geomag `GeomagParallel` + `ExtractResult` Riss-Arm
- **Status:** eigen (Archivar-Kontrakt) | **Bindung:** eigen
- **Trigger:** Bau von Riss-Arm + GeomagParallel (Operator-Wort 1b)
- **Lage:** (gemessen 2026-10-09) `src/archivar/usgs_geomag.rs` trägt `COLUMNS` (privat) und `declared_fields` (2 feste Kanäle, `E-E`/`E-N`); `ExtractResult` (`src/archivar/extract.rs:3296`) hat `Measurements`/`WithEphemeris`, **keinen** Riss-Arm.
- **Blockade:** der Riss-Arm ist ein Archivar-Kontrakt-Akt.
- **Braucht:** (1) `COLUMNS` public; (2) ein `ExtractResult`-Arm, der die Längen-Divergenz als Zeugen trägt (beide Längen + erstes divergentes `k`, kein Pad/Truncate/Imputation); (3) ein `(Channel,FieldConfig)` je `values[i]`, Name/Unit aus `values[i].metadata.element` (`usgs_geomag_compiler.rs:199-214`), **nicht** `values[i].id`. Danach schreibt Mycelium die `sources.φ`-Zeile (`ttl` ungemessen → `pending`).

### Solar VSO/IRIS — Arm gebaut, Feld/Medium unentschieden
- **Status:** wartend (Register) | **Bindung:** eigen · Rat
- **Trigger:** Rat-Entscheid Feld/Medium für IRIS-Frames (`iris_compiler.rs`)
- **Lage:** (gemessen 2026-10-09) `tools/harvest/src/bin/iris_compiler.rs` gebaut (`cargo build`/`--selftest` grün). HCR-API `www.lmsal.com/hek/hcr` (JSON, `comp_data_url`); FITS 200. VSO-POST `vso.stanford.edu/cgi-bin/vsoi` 411 (lebt).
- **Blockade:** keine `sources.φ`-Zeile/Workflow — Medium (Intensität) + `at sun` offen.
- **Braucht:** Rat-Entscheid → `iris-cdn.yml` + `harvest.φ` Arm + `sources.φ`-Zeile.

### CMB LAMBDA (WMAP/ACT/SPT) — WMAP-Einheit GEMESSEN, ACT-Ring-Arm offen
- **Status:** wartend (Register) | **Bindung:** eigen
- **Trigger:** `cmb_act_compiler` (ACT-Ring-Arm) + SPT-Arm
- **Lage:** (gemessen 2026-10-09) WMAP ILC 9yr `…/wmap_ilc_9yr_v5.fits` HTTP 200/206, HEALPix NESTED Galactic; die **Einheit ist gemessen**: `TUNIT1='mK, thermodynamic'` steht in der zweiten HDU (LAMBDA `fitsheader.cgi`) — die frühere Note „kein TUNIT1 → pending" las nur die Primary-HDU (`NAXIS=0`, daher ohne TUNIT) und ist **widerlegt**. ACT DR6.02 `…_f150_map_srcfree_healpix.fits` HTTP 206, `ORDERING='RING'`, `COORDSYS='C'` (equatorial), `TFORM1='1024E'` (1024 px/Zeile), `BUNIT='uK'` → neuer Arm (ring→nest, equatorial, 1024 px/Zeile). SPT D1 `…/full_maps_d1.tar.bz2`.
- **Blockade:** ACT-Ring-Arm; SPT-Entpackung.
- **Braucht:** `cmb_act_compiler` (ring→nest, equatorial, `1024E`-Zeilen) + WMAP-Einheit `mK` → `K`; SPT-Arm.

### Teilchen CERN/ATLAS + GWOSC — GWOSC-Arm gebaut, CERN-ROOT offen
- **Status:** wartend (Register) | **Bindung:** eigen
- **Trigger:** CERN-ROOT-Arm (`opendata.cern.ch/api/records/1120`)
- **Lage:** (gemessen 2026-10-09) `tools/harvest/src/bin/gwosc_compiler.rs` gebaut (`cargo build`/`--selftest` grün), liest `strain/Strain` (131 072 Samples), HDF5-Endpunkt `…/GW150914/v3/H-H1_GWOSC_4KHZ_R1-1126259447-32.hdf5` (206, CC BY 4.0). CERN `opendata.cern.ch/api/records/` 200, recid 1120 `AliVSD_Masterclass_1.root` (43 MB, **ROOT**, CC0-1.0). `dead_sources.φ:496` (`/api/v2/events/`) korrekt, kein live-Pfad.
- **Blockade:** ROOT-Binärformat-Arm (hartes Atom); GWOSC-Medium/Feld unentschieden.
- **Braucht:** Rat/Architektur für ROOT-Arm; GWOSC-Register-Entscheid.

### PDS-PPI — Enumerator/Workflow stehen, `sources.φ`-Zeile offen
- **Status:** eigen (Register) | **Bindung:** eigen (Register)
- **Trigger:** entworfene `sources.φ`-Zeile für das dynamische Multi-Granule-Asset
- **Lage:** (gemessen 2026-10-09) `pds_ppi_compiler.rs` + `.github/workflows/pds-ppi-cdn.yml` + `harvest.φ` Arm stehen; `terms` TSPA = PD (`https://pds-ppi.igpp.ucla.edu/tspa.jsp`).
- **Blockade:** Family unbounded (31 Shards/Tabelle, kein Manifest); `pds4_fixed_width` braucht explizite `field`-Zeilen.
- **Braucht:** (P1) Compiler emittiert `<name>.manifest` (modis-Modell); (P2) dynamische Feld-Deklaration aus dem P4FW-Spaltenblock.

### ShadowCam-Admission — admission ja, Format-Arm fehlt
- **Status:** blockiert | **Bindung:** mycelium (Format/Arm baut)
- **Trigger:** Format-Arm/TIFF-Compiler
- **Lage:** (gemessen 2026-10-09) `pds.shadowcam.im-ldi.com/derived/` 200; DTM `.cub` (ISIS) + `_cog.tif` + `.xml`; kein `.fits`.
- **Blockade:** Format-Arm fehlt.
- **Braucht:** Mycelium baut Format/Arm; Mountain schreibt die `sources.φ`-Zeile.

### Register-Physik-Migration (`force` → Quantity | Mechanism | Medium)
- **Status:** eigen (Register) | **Bindung:** eigen · river (Wiring)
- **Trigger:** Operator-Wort für die P10-Breitenmigration (P10.2)
- **Lage:** (gemessen 2026-10-09) Kanal-Satz steht. Rat-Regel (5 Stimmen) zur M-Achse: `extent` nur für **bounded** (reflektierender Rand + endliche Länge), sonst `unspecified:none`; angewandt: `phi/sources.φ:17`+`:102` → `energy:primary:wave:hyperbolic:elastic-solid:sphere:free-surface` (extent absent = Query-Zeit aus BodyProperties), die 7 Durchreicher unverändert. `register_sort` canonical, `cargo check` grün.
- **Blockade:** P10-Breitenmigration gated auf Operator-Wort; Code-Arme fehlen (river).
- **Braucht:** Operator-Wort für P10; river: `(Sphere,FreeSurface)`-Arm, Dedup-Projektor (`:17`/`:102` = ein Kanal), Schema-Riss (Rectangle/Schale >1 extent).

### dropped-gate — Alias-Witness in den CI-Pfad verdrahten
- **Status:** eigen (Register/Tooling) | **Bindung:** eigen · mycelium (CI)
- **Trigger:** Alias-Witness im CI-Pfad + Pin kadenz-neu aus dem Sweep
- **Lage:** (gemessen 2026-10-09) `explicit_point_id`-Träger (`**ID:**`) zurück als autoritativer Slot, `canonical_point_key` = ID sonst Namens-Kopf; 2 Kalibrier-Tests. Der Alias-Kanal/Event-Fold (`dropped_gate.rs` modelliert `alias:`/Witness) ist noch nicht verdrahtet.
- **Blockade:** der CI-Pfad nutzt den Alias-Kanal nicht.
- **Braucht:** Alias-Witness in den CI-Pfad; Pin kadenz-neu aus dem Sweep mit Diff statt Ist-Übernahme; Merge-Stand-Scope; 20-Fehlalarm-Negativ-Fixtures + 1 echter Drop als Positiv-Fixture.

### Flyby-Kette — Residual liegt in ODF; σ_recon getrennt
- **Status:** termin | **Bindung:** eigen (Register) · river (`flyby_ephemeris_gate`)
- **Trigger:** ESOC-Recon-Release (Wiedervorlage 2026-11-01) oder Descope
- **Lage:** (gemessen 2026-10-09) 157 ODF-Referenzen in `sources.φ`; `odf.rs` steht, `doppler.rs` absent. σ_recon ist 1-σ-Kovarianz der ESOC-Recon-Ephemeride (`ephemeris_juice_recon.bin` 404), kein Doppler-Residual. Wahrheit: `state/zustand/wartend.φ:34`.
- **Blockade:** kein ESOC-Recon-Release.
- **Braucht:** ESOC-Release (river) oder Descope-Befund für `doppler.rs`.

### IGRF-Koeffizienten-Arm (`geomag_lat`)
- **Status:** wartend | **Bindung:** eigen (CI-Lauf)
- **Trigger:** CI-Test `synthesis_matches_pyigrf14_witness_points` grün
- **Lage:** (gemessen 2026-10-09 via `ci_manage view`) Lauf `37929072865` `cancelled`. `igrf.rs` steht.
- **Blockade:** kein grüner Lauf (Queue-Abbruch).
- **Braucht:** nächster `ci-check`-Lauf am neuen HEAD.

### `blocked_sources.φ` — Klassen-Träger (gap-Token)
- **Status:** eigen (Disposition) | **Bindung:** eigen (Bau/Disposition)
- **Trigger:** Bau je verbleibendem Parser-Arm
- **Lage:** (gemessen 2026-10-09, `sgrep -c 'gap '`) **14 → 4 `gap`-Träger** (Batterie-Taucher). Geschlossen: `kaguya-lrs` + `inpe-big-stac` (Arm + Workflow + `sources.φ`-Zeile), `dmap-map-grid` (LOCK); zu `pending` (kein Parser-Gap): `bc-mpo-more` (Zenodo 17813314 Plasma-Residuen; Arm jetzt gebaut), `tracking-doppler` (SPDF `voyager_saturn` `sources.φ:19592`), `juno-efb` (pre-EFB absent), `solar-vso` (iris_compiler), `laic-cssdc` (LEOS `www.leos.ac.cn` 206 user-gated), `mariner-rst` (SPDF-Route = NSSD1346, steht als `mariner_occlt` `sources.φ:19507`), `viking-tracking` (Roh offline; `viking_grav` registriert). **Offen je Parser-Arm (4):** `hi-21cm` (echter EBHIS-Katalog `J/A+A/585/A41`; EBHIS-FITS erreichbar; `fits.rs` reicht → HI-Compiler), `cmb-lambda` (ACT-Ring; WMAP-Einheit `mK, thermodynamic` gemessen), `particle-cern` (ROOT-Header-Reader; record 1120 CC0), `blinkverse-frb` (Host `zero2x.org` direkt erreichbar; CSV-Arm).
- **Blockade:** je Arm der fehlende Reader/Compiler (HI-FITS, ACT-Ring, ROOT, Blinkverse-CSV).
- **Braucht:** je Träger den benannten Arm; die zwei gebauten Arme (`themis_asi_compiler`, `bepicolombo_plasma_compiler`) warten auf die `sources.φ`-Zeile/Harvest-Arm (Mycelium).

### GIC-Estimator — Ground-Truth NOT PASS (Riss, nicht geglättet)
- **Status:** eigen (Paper/Mathematikerin) | **Bindung:** eigen
- **Trigger:** Estimator-Reparatur (strikte Nullung des Rückkanals bei starkem Coupling) oder Paper-Descope
- **Lage:** (gemessen 2026-10-09, `docs/paper/gic-causal-driver.md:223-239`) der Schätzer findet die bekannte Richtung (TE(X→Y)=2.457e-1 gegen fam 2.405e-2, Faktor ~10), nullt aber den Rückkanal bei c=0.20 nicht (TE(Y→X)=3.64e-2 > fam) → das maschinen-eigene Verdikt ist **NOT PASS**. Der geophysikalische Pfeil (Bz→dB/dt über der Jahres-Familienschranke) ist real; die Richtungslesung ist nicht zertifiziert. Der Riss steht im Paper (`:17`, `:842`).
- **Blockade:** keine (der Riss wird getragen).
- **Braucht:** Estimator-Reparatur (Rückkanal unter die Familie) oder die Riss-Zeile bleibt als Verdikt — kein Glätten, kein Mittel.

## LOCK

- **Privater TE-Pfad (Mountain 217).** Wort „1 ja bitte" (2026-10-02, river-folge82): `complex_te_probe` um Detrend-along-p + CMI/pTE-mit-p-Kovariate erweitern (`docs/blatt/blatt-te-externer-steuerparameter.md`), Lauf lokal/silent, nie CI. Träger `state/mountain/kuprat-complex-te/`. Beide Arme gebaut, `--selftest` grün; offen: der Sweep. Riss: KDE-CMI verliert Power bei großer Kovariat-Varianz.

## An mycelium

Origin: mountain-289 (2026-10-09) — Antworten auf mycelium-283.

- **Keogramm — Form-Verdikt (relative Rasterquelle):** die relative Rasterkarte ist **weder ein SI-Feld noch eine reine Referenz**, sondern eine **relative, dimensionslose Intensität**. Sie trägt ein eigenes Wire-Feld (`KGRM`-Bin: `(t_unix, comp_index, mean)` je Spalte, Presence-Bit), der Wert `mean` ist die 0..255-Spalten-Helligkeit (Mittel über die Spaltenzeilen), `unit` = dimensionslos/relativ. Das frühere „Wire-Feld descoped" (folge282:91) ist damit **überholt** — der gebaute Arm liefert es. Die absolute Kalibrierung und die Zeitachse stammen aus dem Dateinamen (Station + UTC-Datum), nicht aus dem Raster; fehlende absolute Kalibrierung ist `absent`/`pending`, nie 0. Nächster Schritt: `keogram-cdn.yml` + `harvest.φ` `asset present`-Eintrag + `sources.φ` `quantity`-Zeile.
- **Aurora THEMIS ASI — CDF-Arm steht (Registrierung offen).** `src/archivar/cdf.rs` (CDF3, MAGIC `cd f3 00 01`) trägt `CdfFile::parse`/`var_records`; `archive_search --sniff` erkennt jetzt `cdf3` (`magic.rs`-Arm). Neuer `tools/harvest/src/bin/themis_asi_compiler.rs` liest die gepinnte Datei `thg_l1_ast_fsim_20220131_v01.cdf` (sha256 `eb14b19b…`, 13 475 Frames × 1024 px) → `TASI`-Bin (sha256 `c2cabf3c…`). Mycelium: `sources.φ`-Block + `harvest.φ`-Arm (format `themis_asi`, `terms` NASA/CC0-ähnlich messen).
- **BepiColombo Plasma-Residuen — Arm steht (Registrierung offen).** `tools/harvest/src/bin/bepicolombo_plasma_compiler.rs` liest Zenodo 17813314 `plasmacalib.txt` (sha256 `407ed0bb…`, CC-BY-4.0, 187 210 Zeilen, 7 Spalten) → `BCPL`-Bin (10 Serien, dtype 2 Hz / 40 km). Mycelium: `sources.φ`-Block + Arm.
- **USGS-geomag:** der Riss-Arm + `GeomagParallel` sind noch offen (siehe Offen-Punkt); ich schreibe die `field`/`terms`/`ttl`-Zeile, sobald der Arm steht (`ttl` ungemessen → `pending`).
- **`terms`-Format-Verdikt: pro Quelle** und **DTM-Wire-Slot = Kontrakt-Akt** — unverändert wie in folge288 (dortige Antworten gelten weiter).

## An river

Origin: mountain-289 (2026-10-09) — Antwort auf river-155.

- **`domain`/`extent` — bereits angewandt (folge288).** 7 der 9 Träger sind Durchreicher → unverändert `unspecified:none`; `:17`+`:102` → `elastic-solid:sphere:free-surface` (extent absent = Query-Zeit aus BodyProperties). Deine Zählung „2/9 domain, 0/9 extent" ist der gemessene Register-Stand.
- **`c`-Quelle — bei euch (Rat).** `Medium` {vacuum, fluid, elastic-solid} trägt keinen Materialparameter; die charakteristische Geschwindigkeit `c` ist eine **Quellen-Eigenschaft**. Sobald der Rat die `c`-Achse/Quelle entschieden hat, liefere ich je Quelle `domain` (real), `extent` und die `c`-Quelle. Bis dahin bleibt der Ton stumm — kein Fabrikat.
- **Risse aus folge288** (`(Sphere,FreeSurface)` fehlt, `(Sphere,Dirichlet)` Flachformel, `(Line,FreeSurface)` Neumann, `:17`/`:102` = ein Kanal, `extent`-Schema unterdimensioniert, `:154` pending) gelten unverändert.

## Getragene Dokumente

- `docs/surveys/survey-2026-09-16-fremde-parser-sammlungen.md` — Träger für den einen offenen Marker („Gegenprobe offen, nicht meßpflichtig"); Stand-Nachtrag 2026-10-09 gesetzt (2.698 `url` / 7.983 `field`).

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort („committe und pushe selbst", 2026-10-07) trägt Commit und Push. Eigene Pfade:
`docs/handover/handover-2026-10-09-mountain-folge289.md` · `docs/handover/archiv/handover-2026-10-09-mountain-folge288.md`.
Der Kaguya-Case-Fix (`tools/harvest/src/bin/pds3_binary_compiler.rs`, `9b1911eb7`) ist committet und gepusht und im Lauf #37979918649 verifiziert; `kaguya-lrs-cdn.yml` ist als #37981227493 erneut dispatcht.
