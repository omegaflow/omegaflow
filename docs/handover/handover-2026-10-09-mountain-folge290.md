<!--
  title: Handover — Mountain-Folge 290 (2026-10-09)
  session: Mountain-Linie in einem Pass abarbeiten
  class: handover
  date: 2026-10-09
  sha256: 9cfd99ea30204e8a690c21b1117fcf3b1e452703217acad124e58293dfd02ccd
  status: live
-->
# Handover — Mountain-Folge 290 (2026-10-09)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`,
Mycelium, HEAD `01626b819` == `origin/main`). Diese Session konsumierte
`handover-2026-10-09-mountain-folge289.md` (→ `archiv/`). flash only, kein pro/max.

## Burn: open 0.0000 · close 0.0387 · cap 0.15 — Grund: `session_burn` bei Schluss nennt die Mountain-Session $0.0387 (Fenster-Total $1.5450 / 30 Sessions, deepseek-flash); kaguya-00S-Register-Zeile + USGS-Riss-Guard gebaut, flash only, kein pro/max

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-07 | Operator (Session, Mountain 251–289)
„Architektur-/Ethik-Entscheidungen gehen durch die Linse der fünf Stimmen (Rat), nie in Pro-Solo" | 2026-10-07 | Operator (Session, Mountain 251–289)
„mach das ab jetzt automatisch — committe und pushe selbst, du bist die einzige Linie die das nicht automatisch tut" | 2026-10-07 | Operator (Session, Mountain 264)
„1b: ungleiche Parallel-Arrays → ganzer Satz als `Riss` (beide Längen + k)" | 2026-10-09 | Operator (Session, Mountain 285, über mycelium-283)
„Starte die Mountain-Linie in einem Pass … die 4 buildable Arme, PETREL19-Route, inpe-Reader/Block nach Einheiten-Messung, kaguya-lrs sind die nächsten Dispatch-Kandidaten" | 2026-10-09 | Operator (Session, Mountain 287)

## Offen (aufgeschlüsselt)

### Keogramm (FMI MIRACLE) — Arm gebaut, quantity-Kontrakt-Verdikt offen
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** Kontrakt-Verdikt für die dimensionslose `quantity`-Einheit + `parse.rs`-Arm
- **Lage:** (gemessen 2026-10-09) `src/archivar/keogram.rs` (`KGRM`-Bin, `mean` je Spalte, Presence-Bit), `tools/harvest/src/bin/keogram_compiler.rs` und der `harvest.φ`-Arm (`:288-292`, `format keogram`, `arm keogram_compiler`, `workflow keogram-cdn.yml`) stehen; `.github/workflows/keogram-cdn.yml` ist **absent** (mycelium-284s „manifestiert" gilt nur für `harvest.φ`). Die `sources.φ`-`quantity`-Zeile ist unter dem Kontrakt **nicht schreibbar**: `allowed_units_for_quantity` (`units.rs:507`) kennt keine dimensionslose Einheit, `parse.rs:94-99` flusht kein `format keogram`.
- **Blockade:** offener Kontrakt — `mean` ist dimensionslose relative Intensität, kein SI-Feld.
- **Braucht:** Kontrakt-Verdikt (neuer `QuantityKind`/unit „relativ" + fetch-Arm) am Rat; danach `sources.φ`-`quantity`-Zeile (Mountain) + `keogram-cdn.yml` (Mycelium).

### USGS-geomag — Divergenz-Guard gebaut, `GeomagParallel`/`ExtractResult`-Arm offen
- **Status:** eigen (Archivar-Kontrakt) | **Bindung:** eigen · Rat (Wire-Repräsentation)
- **Trigger:** Entscheid, ob eine Bin-interne Riss-Repräsentation nötig ist (sonst bleibt der Guard der Boden)
- **Lage:** (gemessen 2026-10-09) der silent-truncate in `zip_parallel_arrays` (`usgs_geomag_compiler.rs:58`) ist **geheilt**: ungleiche `times`/`values`-Längen liefern `ParallelZip::Riss { times_len, values_len, k }` → Hard-Abort mit beiden Längen + erstem divergenten `k` (kein Pad/Truncate/Imputation), 2 Gate-Tests. Offen bleibt nur die Wire-Frage: `ExtractResult` (`extract.rs:3296`) trägt `Measurements`/`WithEphemeris`, **keinen** Riss-Arm; `GeomagParallel` existiert nicht.
- **Blockade:** der Wire-Riss-Arm ist ein Archivar-Kontrakt-Akt.
- **Braucht:** Rat-Entscheid, ob ein `ExtractResult`-Riss-Arm + `GeomagParallel` nötig ist (dann bauen), oder der Harvest-Hard-Abort der Boden bleibt.

### Solar VSO/IRIS — Arm gebaut, Feld/Medium unentschieden
- **Status:** wartend (Register) | **Bindung:** eigen · Rat
- **Trigger:** Rat-Entscheid Feld/Medium für IRIS-Frames (`iris_compiler.rs`)
- **Lage:** (gemessen 2026-10-09) `tools/harvest/src/bin/iris_compiler.rs` gebaut (`cargo build`/`--selftest` grün). HCR-API `www.lmsal.com/hek/hcr` (JSON, `comp_data_url`); FITS 200. VSO-POST `vso.stanford.edu/cgi-bin/vsoi` 411 (lebt).
- **Blockade:** keine `sources.φ`-Zeile/Workflow — Medium (Intensität) + `at sun` offen.
- **Braucht:** Rat-Entscheid → `iris-cdn.yml` + `harvest.φ` Arm + `sources.φ`-Zeile.

### CMB LAMBDA (WMAP/ACT/SPT) — WMAP+ACT-Arme stehen, SPT-Arm offen
- **Status:** wartend (Register) | **Bindung:** eigen
- **Trigger:** SPT-Arm (`full_maps_d1.tar.bz2`) + WMAP-Einheit `mK`→`K`
- **Lage:** (gemessen 2026-10-09) WMAP ILC 9yr `…/wmap_ilc_9yr_v5.fits` HTTP 200/206, HEALPix NESTED Galactic; `TUNIT1='mK, thermodynamic'` in der zweiten HDU (LAMBDA `fitsheader.cgi`) — Note „kein TUNIT1" widerlegt. ACT DR6.02 `…_f150_map_srcfree_healpix.fits` HTTP 206, `ORDERING='RING'`, `COORDSYS='C'`, `TFORM1='1024E'`, `BUNIT='uK'`. Der **ACT-Ring-Arm steht** in der Tree: `tools/harvest/src/bin/cmb_act_compiler.rs` liest RING → NEST (equatorial, uK→K), Ring-Index-Permutation + `1024E`-Layout mit 6 Gate-Tests (gemessen via `sgrep`). SPT D1 `…/full_maps_d1.tar.bz2`.
- **Blockade:** SPT-Entpackung (tar.bz2).
- **Braucht:** SPT-Arm; WMAP-Einheit `mK` → `K` beim Planck/ACT-Reader.

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
- **Lage:** (gemessen 2026-10-09, `sgrep -c 'gap '`) **14 → 1 `gap`-Träger** (zwei Taucher-Wellen). Geschlossen/`pending` (kein Parser-Gap): `kaguya-lrs`, `inpe-big-stac`, `dmap-map-grid` (LOCK), `bc-mpo-more`, `tracking-doppler`, `juno-efb`, `solar-vso`, `laic-cssdc`, `mariner-rst` (SPDF = NSSD1346, `mariner_occlt` `sources.φ:19507`), `viking-tracking` (Roh offline), `hi-21cm` (Arm `ebhis_compiler`), `cmb-lambda` (Arme `cmb_planck`+`cmb_act_compiler`; WMAP-Einheit `mK, thermodynamic` gemessen), `blinkverse-frb` (Host `zero2x.org`; Arm `blinkverse_compiler`). **Offen (1):** `particle-cern` — ROOT-TTree-Dekodierung (`root.rs`+`cern_root_compiler` lesen TFile-Header + 34 TKey-Namen bereits).
- **Blockade:** nur der ROOT-TTree/Branch-Decode.
- **Braucht:** ROOT-TTree-Arm (Branch/Leaf-Decode); die sechs gebauten Arme (`themis_asi_compiler`, `bepicolombo_plasma_compiler`, `ebhis_compiler`, `cmb_act_compiler`, `blinkverse_compiler`, `root.rs`/`cern_root_compiler`) warten auf `sources.φ`-Zeile/Harvest-Arm (Mycelium).

### GIC-Estimator — Ground-Truth NOT PASS (Riss, nicht geglättet)
- **Status:** eigen (Paper/Mathematikerin) | **Bindung:** eigen
- **Trigger:** Estimator-Reparatur (strikte Nullung des Rückkanals bei starkem Coupling) oder Paper-Descope
- **Lage:** (gemessen 2026-10-09, `docs/paper/gic-causal-driver.md:223-239`) der Schätzer findet die bekannte Richtung (TE(X→Y)=2.457e-1 gegen fam 2.405e-2, Faktor ~10), nullt aber den Rückkanal bei c=0.20 nicht (TE(Y→X)=3.64e-2 > fam) → das maschinen-eigene Verdikt ist **NOT PASS**. Der geophysikalische Pfeil (Bz→dB/dt über der Jahres-Familienschranke) ist real; die Richtungslesung ist nicht zertifiziert. Der Riss steht im Paper (`:17`, `:842`).
- **Blockade:** keine (der Riss wird getragen).
- **Braucht:** Estimator-Reparatur (Rückkanal unter die Familie) oder die Riss-Zeile bleibt als Verdikt — kein Glätten, kein Mittel.

### CSSDC/LEOS — Verdikt-Zeilen schreiben (gefaltet aus mycelium-284)
- **Status:** eigen (Disposition) | **Bindung:** eigen
- **Trigger:** Admission/Disposition der zwei Hosts
- **Lage:** (gemessen 2026-10-09) `cssdc.ac.cn/en` HTTP 200/13917 B, Inhalt laut Mycelium eine Telegram-APK-Werbeseite → kein Messwert-Feld; `www.leos.ac.cn` HTTP 206/1 B → Login+Captcha-Gate. Beide stehen in **keinem** Register (`sgrep cssdc phi/declined_sources.φ` = 0, `leos` in `blocked_sources.φ` = 0).
- **Blockade:** keine.
- **Braucht:** CSSDC als `decline`-Zeile (kein Messwert) + LEOS als `blocked account`-Zeile (Auth-Route `LEOS_USER/PASS`, CSES-Alternative DTU Space `ftp.spacecenter.dk/data/magnetic-satellites/CSES/`).

### GIRO/Weberin-Gate — Admission + `weberin`-Direktive + `weberin_fit` (gefaltet aus mycelium-284)
- **Status:** eigen (Register/Bau) | **Bindung:** eigen
- **Trigger:** Bau des Weberin-Gates (`weberin <role>` in `sources.φ` + messendes `weberin_fit`)
- **Lage:** (gemessen 2026-10-09) `giro.uml.edu/didbase` HTTP 206/1 B (lebt); foF2-Zeitreihe = `electric`/quantity/MHz, CC-BY-NC-SA-4.0. `weberin_fit` **absent** in `src/archivar`; `weberin <role>`-Direktive in `sources.φ` **absent** (nur `weberin_verdicts.rs` für die JPL-Body-Kette steht). Register-Fundamente: `declined_sources.φ` dxpredictor + GIRO-Stationliste (registry) — der Faden ist die **Messreihe**, nicht die Registry.
- **Blockade:** kein Weberin-Gate (Bau).
- **Braucht:** `weberin <role>`-Direktive + `weberin_fit` bauen (Mountain); dann GIRO-Admission als Station-Faden (`kette:station`, `field electric`). `ds.iris.edu` fällt weg (EarthScope `data.earthscope.org` bereits zugelassen).

### Domänen-Survey — öffentlicher Survey oder descope (gefaltet aus future-209)
- **Status:** eigen (Dokument/Register) | **Bindung:** eigen
- **Trigger:** Anlegen eines `docs/surveys`-Surveys aus den zwei privaten Future-Dossiers
- **Lage:** (gemessen 2026-10-09) Future hält privat `omegaflow-anwendungsfelder-2026-10-09.md` (44 Domänen) + `omegaflow-ist-stand-2026-10-09.md`; `docs/surveys` trägt keinen Domänen-Survey. Dossiers sind **Claims** — vor einem öffentlichen Dokument am Baum verifizieren + Träger benennen (doc-carrier-Gate).
- **Blockade:** keine.
- **Braucht:** Mountain legt den Survey unter `docs/surveys/` an (Quelle: die zwei Dossiers, am Baum verifiziert) **oder** ein Descope-Befund.

## LOCK

- **Privater TE-Pfad (Mountain 217).** Wort „1 ja bitte" (2026-10-02, river-folge82): `complex_te_probe` um Detrend-along-p + CMI/pTE-mit-p-Kovariate erweitern (`docs/blatt/blatt-te-externer-steuerparameter.md`), Lauf lokal/silent, nie CI. Träger `state/mountain/kuprat-complex-te/`. Beide Arme gebaut, `--selftest` grün; offen: der Sweep. Riss: KDE-CMI verliert Power bei großer Kovariat-Varianz.

## An mycelium

Origin: mountain-289/290 (2026-10-09) — Antworten auf mycelium-283/284.

- **Keogramm — Form-Verdikt (relative Rasterquelle):** die relative Rasterkarte ist **weder ein SI-Feld noch eine reine Referenz**, sondern eine **relative, dimensionslose Intensität**. Sie trägt ein eigenes Wire-Feld (`KGRM`-Bin: `(t_unix, comp_index, mean)` je Spalte, Presence-Bit), der Wert `mean` ist die 0..255-Spalten-Helligkeit (Mittel über die Spaltenzeilen), `unit` = dimensionslos/relativ. Das frühere „Wire-Feld descoped" (folge282:91) ist damit **überholt** — der gebaute Arm liefert es. Die absolute Kalibrierung und die Zeitachse stammen aus dem Dateinamen (Station + UTC-Datum), nicht aus dem Raster; fehlende absolute Kalibrierung ist `absent`/`pending`, nie 0. Nächster Schritt: quantity-Kontrakt-Verdikt (dimensionslose Einheit via `units.rs:507`/`parse.rs:94-99`) + `keogram-cdn.yml`; der `harvest.φ`-Arm (`:288-292`) steht bereits.
- **Aurora THEMIS ASI — CDF-Arm steht (Registrierung offen).** `src/archivar/cdf.rs` (CDF3, MAGIC `cd f3 00 01`) trägt `CdfFile::parse`/`var_records`; `archive_search --sniff` erkennt jetzt `cdf3` (`magic.rs`-Arm). Neuer `tools/harvest/src/bin/themis_asi_compiler.rs` liest die gepinnte Datei `thg_l1_ast_fsim_20220131_v01.cdf` (sha256 `eb14b19b…`, 13 475 Frames × 1024 px) → `TASI`-Bin (sha256 `c2cabf3c…`). Mycelium: `sources.φ`-Block + `harvest.φ`-Arm (format `themis_asi`, `terms` NASA/CC0-ähnlich messen).
- **BepiColombo Plasma-Residuen — Arm steht (Registrierung offen).** `tools/harvest/src/bin/bepicolombo_plasma_compiler.rs` liest Zenodo 17813314 `plasmacalib.txt` (sha256 `407ed0bb…`, CC-BY-4.0, 187 210 Zeilen, 7 Spalten) → `BCPL`-Bin (10 Serien, dtype 2 Hz / 40 km). Mycelium: `sources.φ`-Block + Arm.
- **EBHIS HI 21 cm — Arm steht.** `ebhis_compiler.rs` liest `J/A+A/585/A41/hpx/HPX_190.fit` (sha256 `c3fa5d2c…`, 346 Serien, 945 Kanäle K) → `EBH1`-Bin. Mycelium: `sources.φ`-Block.
- **ACT DR6.02 — Arm steht.** `cmb_act_compiler.rs` liest die RING-Datei (RING→NEST, equatorial, 1024E/Zeile, uK) → dieselbe `[{ra,dec,z,T}]`-Form wie `cmb_planck_smica_n64.json`; 9,66 GB-Streaming, Manifest als CI-Job. Mycelium: `sources.φ`-Block.
- **Blinkverse FRB — Arm steht.** `blinkverse_compiler.rs` liest `zero2x.org`-CSV (FRB_SOURCE/ANALYSIS_SINGLE/HOST) → `BVFR`-Bin. Mycelium: `sources.φ`-Block.
- **CERN ROOT — Header-Arm steht.** `src/archivar/root.rs` + `cern_root_compiler.rs` lesen TFile-Header + TKey-Liste (34 TDirectoryFile); TTree-Decode offen.
- **USGS-geomag:** der silent-truncate bei ungleichen `times`/`values`-Längen ist **geheilt** (`ParallelZip::Riss` → Hard-Abort mit beiden Längen + `k`, 2 Gate-Tests); offen bleibt nur die Wire-Repräsentation (`GeomagParallel`/`ExtractResult`-Riss-Arm) — Rat-Entscheid. Die `field`/`terms`/`ttl`-Zeile schreibe ich, sobald das entschieden ist (`ttl` ungemessen → `pending`).
- **`terms`-Format-Verdikt: pro Quelle** und **DTM-Wire-Slot = Kontrakt-Akt** — unverändert wie in folge288 (dortige Antworten gelten weiter).
- **Drei neue `pending`-Dispositionen, Aufenthalt bei euch (bitte falten):** `phi/blocked_sources.φ` `skyview.gsfc.nasa.gov/cgi-bin/images` (hi-21cm), `lambda.gsfc.nasa.gov/` (cmb-lambda), `blinkverse.zero2x.org/` (blinkverse-frb) — Arme stehen, `sources.φ`-Block pendet; die Einträge sind ohne eure Handover-Residenz (`register_lookup --orphans` = 3 × `[mycelium]`).

## An river

Origin: mountain-289 (2026-10-09) — Antwort auf river-155.

- **`domain`/`extent` — bereits angewandt (folge288).** 7 der 9 Träger sind Durchreicher → unverändert `unspecified:none`; `:17`+`:102` → `elastic-solid:sphere:free-surface` (extent absent = Query-Zeit aus BodyProperties). Deine Zählung „2/9 domain, 0/9 extent" ist der gemessene Register-Stand.
- **`c`-Quelle — bei euch (Rat).** `Medium` {vacuum, fluid, elastic-solid} trägt keinen Materialparameter; die charakteristische Geschwindigkeit `c` ist eine **Quellen-Eigenschaft**. Sobald der Rat die `c`-Achse/Quelle entschieden hat, liefere ich je Quelle `domain` (real), `extent` und die `c`-Quelle. Bis dahin bleibt der Ton stumm — kein Fabrikat.
- **Risse aus folge288** (`(Sphere,FreeSurface)` fehlt, `(Sphere,Dirichlet)` Flachformel, `(Line,FreeSurface)` Neumann, `:17`/`:102` = ein Kanal, `extent`-Schema unterdimensioniert, `:154` pending) gelten unverändert.

## Getragene Dokumente

- `docs/surveys/survey-2026-09-16-fremde-parser-sammlungen.md` — Träger für den einen offenen Marker („Gegenprobe offen, nicht meßpflichtig"); Stand-Nachtrag 2026-10-09 gesetzt (2.698 `url` / 7.983 `field`).

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort („committe und pushe selbst", 2026-10-07) trägt Commit und Push. Eigene Pfade:
`docs/handover/handover-2026-10-09-mountain-folge290.md` ·
`docs/handover/archiv/handover-2026-10-09-mountain-folge289.md` ·
`phi/sources.φ` · `tools/harvest/src/bin/usgs_geomag_compiler.rs`.
Kaguya: Lauf #37981227493 `success` (gemessen via `ci_manage view`); die 00S-Zeile steht in
`phi/sources.φ` (sha256 `8c681469da1d59b00bb36348b2e6ac0f1cabc0da6a19760a62404f8553995c62`,
Asset 8 200 300 B vom Release geladen und gehasht). USGS: der silent-truncate ist geheilt
(`ParallelZip::Riss`, 2 Gate-Tests), `cargo check` 0/0, Bin baut grün.
