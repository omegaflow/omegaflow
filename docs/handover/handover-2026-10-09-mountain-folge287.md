<!--
  title: Handover — Mountain-Folge 287 (2026-10-09)
  session: Mountain-Linie in einem Pass abarbeiten
  class: handover
  date: 2026-10-09
  sha256: f145abf78c44bbb3e7121ddcccd194f894be8f4de0221f6362d34694fb5a1bc4
  status: live
-->
# Handover — Mountain-Folge 287 (2026-10-09)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`,
Mycelium, gemessen 2026-10-09T16:14Z). Diese Session konsumierte
`handover-2026-10-09-mountain-folge286.md` (→ `archiv/`). Kein pro/max; nur flash
(6 `general`/`grind-flash`-Taucher für Messung und Arm-Bau).

## Burn: open 0.0000 · close 0.3189 · cap 0.35 — Grund: Atom-Burn über dem Default (8 Taucher für 4 Bau-Arme + PETREL19/INPE/Kaguya); flash only, kein pro/max

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-07 | Operator (Session, Mountain 251–287)
„Architektur-/Ethik-Entscheidungen gehen durch die Linse der fünf Stimmen (Rat), nie in Pro-Solo" | 2026-10-07 | Operator (Session, Mountain 251–287)
„mach das ab jetzt automatisch — committe und pushe selbst, du bist die einzige Linie die das nicht automatisch tut" | 2026-10-07 | Operator (Session, Mountain 264)
„ja bitte" — Index-Riss als Mountain-Verdikt `quantity` setzen + die `sources.φ`-Zeilen bauen | 2026-10-08 | Operator (Session, Mountain 276)
„ich glaube du musst nochmal breiter fragen" — Science-Layer + starke Frontier-Seats für die Route-Admission | 2026-10-08 | Operator (Session, Mountain 276)
„bitte umsetzen Offen (im Report benannt): 2 blocked_sources-Risse …" | 2026-10-09 | Operator (Session, Mountain 280)
„Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt) …" + „aber mach dann auch wirklich die Arbeit" | 2026-10-09 | Operator (Session, Mountain 281)
„1. natürlich Ja wir brauchen die lizenzen sind regeln der quellen nicht unsnere" | 2026-10-09 | Operator (Session, Mountain 283)
„2 bitte spreche dich mit river ab das ist teil seines plans" | 2026-10-09 | Operator (Session, Mountain 283)
„du sollst das prüfen, die lizenzen müssen korrekt sein" | 2026-10-09 | Operator (Session, Mountain 283)
„sind jetzt alle blöcke in allen asset files mit tes versehen …" | 2026-10-09 | Operator (Session, Mountain 283)
„wir sind immer noch nicht opensource" — omegaflow ist source-available (PolyForm NC/CC BY-NC-SA), NIE „open-source" nennen | 2026-10-09 | Operator (Session, Mountain 283)
„natürlich 1 wir sind nicht open source wir sind NC CC" | 2026-10-09 | Operator (Session, Mountain 283)
„das ist compliance theater" — keine Lizenz-Boilerplate in einer Anfrage-Mail; nur sagen, was die Frage braucht | 2026-10-09 | Operator (Session, Mountain 283)
„bitte fixen: ledger.φ JAXA-Zitat … ShadowCam admission ja, Format-Arm fehlt; Migration (river-148) gefaltet" | 2026-10-09 | Operator (Session, Mountain 284)
„bitte mit archive search all dem rat und den top tier frontier voices besprechen" — USGS-Extract-Arm-Architektur | 2026-10-09 | Operator (Session, Mountain 285)
„Starte die Mountain-Linie in einem Pass … die 4 buildable Arme, PETREL19-Route, inpe-Reader/Block nach Einheiten-Messung, kaguya-lrs sind die nächsten Dispatch-Kandidaten" | 2026-10-09 | Operator (Session, Mountain 287)

## Offen (aufgeschlüsselt)

### PETREL19 — Route gebaut, Manifest im CI-Flug
- **Status:** eigen (CI) | **Bindung:** eigen
- **Trigger:** Lauf `petrel19-cdn.yml` (#37966130284) grün, Asset manifestiert
- **Lage:** (gemessen 2026-10-09) `.github/workflows/petrel19-cdn.yml` + `phi/harvest.φ` Arm `ephemeris_petrel19` + `phi/sources.φ:2316-2341` (`ephemeris_petrel19_{earth,moon,sun}`, CC-BY-4.0) gebaut. Kommando: `de_compiler PETREL19_translation.bsp --label petrel19 --netloc github.com --gm PETREL19.tpc --pck PETREL19.tpc --ci-mode`. Kernel 46 976 000 B HTTP 206; `PETREL19.tpc` 17 475 B (GM/RADII). `PETREL19_rotation.bpc` (frame 3011) wird **nicht** konsumiert (`de_compiler` übergibt `bpc_files=&[]`).
- **Blockade:** Manifest pending (Workflow noch nicht gelaufen); sha256 fehlt in den 3 Zeilen.
- **Braucht:** `petrel19-cdn.yml` dispatchen; danach sha256 je Asset in `sources.φ` nachtragen.

### INPE BIG STAC — Einheit gemessen, Route gebaut
- **Status:** eigen (CI) | **Bindung:** eigen
- **Trigger:** Lauf `inpe-stac-cdn.yml` (#37966134016) grün, Asset manifestiert
- **Lage:** (gemessen 2026-10-09) NetCDF4 `tmax` trägt **kein** `units` (nur `_FillValue`/`missing_value`); STAC-Item `assets.tmax.eo:bands[0].description = "Unit: Celsius"` → `field tmax samet_tmax inverse-square thermal C 604800 0.0 0.0` (`phi/sources.φ:10927-10934`). Asset-Href `…/TMAX/2026/10/SAMeT_CPTEC_TMAX_20261008.nc`, sha256 `96cc2d1a…` == STAC `checksum:multihash`. `inpe-stac-cdn.yml` + `harvest.φ` Arm gebaut.
- **Blockade:** Manifest pending.
- **Braucht:** `inpe-stac-cdn.yml` dispatchen.

### Kaguya/SELENE LRS — Block stand, dedizierter Workflow ergänzt
- **Status:** eigen (CI) | **Bindung:** eigen
- **Trigger:** Lauf `kaguya-lrs-cdn.yml` (#37966137750) grün
- **Lage:** (gemessen 2026-10-09) Riss widerlegt: `phi/sources.φ:10877` trägt den 00N-Block (mountain 213, sha256 `772e51d1…`); generischer `pds3-binary-cdn.yml` läuft. `kaguya-lrs-cdn.yml` (`--dir …/20071120/data/`, Idempotenz auf `00s`) + `harvest.φ` Arm `pds3_binary_kaguya_lrs` ergänzt für 00S + weitere Datums-Verzeichnisse.
- **Blockade:** DARTS-Host flappt 200/503 (Compiler `curl --retry` fängt 5xx).
- **Braucht:** `kaguya-lrs-cdn.yml` dispatchen; neue Asset-Namen danach als `sources.φ`-Zeilen nachtragen.

### Solar VSO/IRIS — Arm gebaut, Feld/Medium unentschieden
- **Status:** wartend (Register) | **Bindung:** eigen · Rat
- **Trigger:** Rat-Entscheid Feld/Medium für IRIS-Frames (`iris_compiler.rs`)
- **Lage:** (gemessen 2026-10-09) `tools/harvest/src/bin/iris_compiler.rs` gebaut (`cargo build`/`--selftest` grün). HCR-API `www.lmsal.com/hek/hcr` (JSON, `comp_data_url`); FITS 200 (`…/level2/2024/01/01/…/iris_l2_…_SJI_2796_t000.fits`, 9 207 360 B, magic fits). VSO-POST `vso.stanford.edu/cgi-bin/vsoi` 411 (lebt).
- **Blockade:** keine `sources.φ`-Zeile/Workflow — Medium (Intensität) + `at sun` offen.
- **Braucht:** Rat-Entscheid → `iris-cdn.yml` + `harvest.φ` Arm + `sources.φ`-Zeile.

### CMB LAMBDA (WMAP/ACT/SPT) — WMAP ingestierbar, Einheit + ACT/SPT-Arm offen
- **Status:** wartend (Register) | **Bindung:** eigen
- **Trigger:** WMAP-Einheiten-Messung aus LAMBDA-Doc + `cmb_act_compiler` (ACT-Ring)
- **Lage:** (gemessen 2026-10-09) WMAP ILC 9yr `…/data/map/dr5/dfp/ilc/wmap_ilc_9yr_v5.fits` HTTP 206, HEALPix NESTED Galactic, `NAXIS2=3145728`, 7 775 868 B — `cmb_planck_compiler.rs` ingestierbar; **Riss:** Header ohne `TUNIT1` → Einheit pending. ACT DR6.02 `…/act-planck_dr4dr6_coadd_AA_daynight_f150_map_srcfree_healpix.fits` HTTP 206, RING/equatorial/`TFORM1='1024E'` → neuer Arm. SPT D1 `…/spt_3g_d1/…/full_maps_d1.tar.bz2` (13 MB, bzip2-tar).
- **Blockade:** WMAP-Einheit; ACT-Ring-Arm; SPT-Entpackung.
- **Braucht:** `TUNIT1`/Einheit aus LAMBDA-Produktseite; `cmb_act_compiler` (ring→nest, equatorial); SPT-Arm.

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
- **Status:** wartend | **Bindung:** river (Parser-Arm)
- **Trigger:** Rivers `channel`-Parser-Arm auf der genannten `pde_type`-Menge
- **Lage:** (gemessen 2026-10-09, river-150/151) `ChannelDescriptor` trägt `role: QuantityRole` + `pde_type: PdeType`; `gravity`=elliptisch, `seismic`=hyperbolisch, `em`=Mixed. Antwort mit den Token an river gefaltet.
- **Blockade:** der `channel`-Parser-Arm schreibt noch nicht.
- **Braucht:** Rivers Arm; danach die `quantity`/`pde_type`-Zeilen der ersten Charge.

### Flyby-Kette — Residual liegt in ODF; σ_recon getrennt
- **Status:** termin | **Bindung:** eigen (Register) · river (`flyby_ephemeris_gate`)
- **Trigger:** ESOC-Recon-Release (Wiedervorlage 2026-11-01) oder Descope
- **Lage:** (gemessen 2026-10-09) 157 ODF-Referenzen in `sources.φ`; `odf.rs` steht, `doppler.rs` absent. σ_recon ist 1-σ-Kovarianz der ESOC-Recon-Ephemeride (`ephemeris_juice_recon.bin` 404), kein Doppler-Residual.
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
- **Trigger:** Bau je Klassen-Träger
- **Lage:** (gemessen 2026-10-09) **14 `gap`-Träger** (`sgrep -c 'gap '`). Neu in diesem Atom: kaguya-lrs/inpe/petrel19-Noten auf den gebauten Stand gesetzt; WMAP/ACT/SPT + IRIS + CERN/GWOSC mit gemessenen Endpunkten. Verbleibend buildable: `hi-21cm` (EBHIS-Release), `blinkverse-frb`, `laic-cssdc` (Riss). Wartend: `tracking-doppler`, `viking-tracking`, `juno-efb`. LOCK: `bc-mpo-more`, `mariner-rst`, `dmap-map-grid`.
- **Blockade:** hi-21cm an der fehlenden Quelle; laic-cssdc am Riss; wartend an NSSDC/JPL.
- **Braucht:** je Träger den benannten Schritt; hi-21cm EBHIS-Release lokalisieren.

## LOCK

- **Privater TE-Pfad (Mountain 217).** Wort „1 ja bitte" (2026-10-02, river-folge82): `complex_te_probe` um Detrend-along-p + CMI/pTE-mit-p-Kovariate erweitern (`docs/blatt/blatt-te-externer-steuerparameter.md`), Lauf lokal/silent, nie CI. Träger `state/mountain/kuprat-complex-te/`. Beide Arme gebaut, `--selftest` grün; offen: der Sweep. Riss: KDE-CMI verliert Power bei großer Kovariat-Varianz.

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort („committe und pushe selbst", 2026-10-07) trägt Commit und Push. Eigene Pfade:
`phi/sources.φ` · `phi/harvest.φ` · `phi/blocked_sources.φ` ·
`.github/workflows/petrel19-cdn.yml` · `.github/workflows/kaguya-lrs-cdn.yml` · `.github/workflows/inpe-stac-cdn.yml` ·
`tools/harvest/src/bin/iris_compiler.rs` · `tools/harvest/src/bin/gwosc_compiler.rs` ·
`docs/handover/handover-2026-10-09-mountain-folge287.md` · `docs/handover/archiv/handover-2026-10-09-mountain-folge286.md`.
Nach dem Push: `petrel19-cdn.yml`, `inpe-stac-cdn.yml`, `kaguya-lrs-cdn.yml` dispatchen.
