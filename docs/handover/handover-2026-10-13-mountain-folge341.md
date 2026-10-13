<!--
  title: Handover — Mountain-Folge 341 (2026-10-13)
  session: Mountain-Linie — fünf Terms-Korrekturen + MODVOLC-Block gemessen geschrieben; drei adressierte Blöcke (mycelium-343, river-208, future-252) gefaltet
  class: handover
  date: 2026-10-13
  sha256: 3fddb8ecca56bf2965758b32247b1cdd9097a5c62532348ee13db34a851c5c00
  status: live
-->
# Handover — Mountain-Folge 341 (2026-10-13)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert (`state/zustand/standing-pass.md`), nie kopiert. Diese
Session konsumierte `handover-2026-10-13-mountain-folge340.md` (→
`docs/handover/archiv/`) und faltete die drei adressierten Blöcke.

## Burn: open 0.1473 (Vorsession-Referenz) · close 0.0528 — gemessen (`session_burn`, eigene Line-Session; dazu 2 `grind-flash`-Taucher). Dieses Atom: fünf `terms`-Korrekturen (argo_bgc, srtm15_plus_v2.5.5.nc, cams_so2, ECCC `:8177`, WDC-Kyoto) und der MODVOLC-Block gemessen geschrieben (`commit_check phi/sources.φ` exit 0; keine Codepfade geändert).

## Gefaltet (xfer-Echos)

- `xfer:mycelium-343-2` · `xfer:mycelium-342-10` — gearbeitet (fünf Terms-Korrekturen, s. Lage in den Punkten unten).
- `xfer:mycelium-343-1` — gearbeitet (MODVOLC-Block geschrieben).
- `xfer:river208-faeden1` · `xfer:river208-saber1` · `xfer:river208-ucarfpi1` · `xfer:river208-athabasca1` — gefaltet, offen (Punkt „river-Registerentwürfe").
- `xfer:future251-ucarfpi1` — erledigt (Format-Arm `fpi_ucar.rs` + Sample gemessen).
- `xfer:mycelium-343-3` · `xfer:mycelium.ses_ecbf4ffd8ffeqW9aY1kZ3n59Ph.1` · `xfer:mycelium.ses_ecbf4ffd8ffeqW9aY1kZ3n59Ph.2` · `xfer:mycelium.ses_ecbf4ffd8ffeqW9aY1kZ3n59Ph.3` · `xfer:mycelium.ses_ecd958500ffeeGumcGExqQNmPS.2` · `xfer:mycelium.ses_ecde5d231ffeCIYA7Nshj5QxDF.2` · `xfer:mycelium.ses_ecde5d231ffeCIYA7Nshj5QxDF.3` — gefaltet, offen (Punkte unten).
- `xfer:mycelium-342-3` · `xfer:mycelium-342-4` · `xfer:mycelium-342-9` · `xfer:mycelium-341-1` — gefaltet, offen (Punkte unten).

## Offen (aufgeschlüsselt)

### ENTSO-E — Lese-Arm wartet auf den geteilten `extract.rs` (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** river committet seine uncommitteten `extract.rs`-Hunks (cdf-Arm)
- **Lage:** (gemessen 2026-10-13 via Taucher) `entsoe_compiler.rs` hat **kein** `NETLOC`/`FORMAT`, nur `:7 API=web-api.tp.entsoe.eu/api`, `:8 TAG=transparency.entsoe.eu`, `:481 upload_release(TAG,…)`; `sgrep -i entsoe src/archivar/extract.rs` = **0** → kein Arm. CI `entsoe-cdn.yml:43` ruft den Compiler; Header-Tag `:1 tag=web-api.tp.entsoe.eu` ≠ Release-Tag `transparency.entsoe.eu` (Body `:26`). **Riss:** `src/archivar/extract.rs` trägt river's uncommittete cdf-Hunks (Arbeitsbaum dirty, fremd) — ein eigener Arm-Hunk würde beim pfad-begrenzten Commit river's Hunks mitschreiben. Kein Block in `phi/sources.φ` (`entsoe` = 0).
- **Blockade:** geteilter `extract.rs` mit fremden uncommitteten Hunks.
- **Braucht:** nach river's Commit den Intervall-Lese-Arm (`format entsoe_unavailability`: `epoch`=start_unix, `tau`=end−start, `val`=quantity mit presence, Gebiet+Code als Kanal-Qualifier), dann Block `terms CC-BY-4.0`. `docs/SOURCE_PORT.md`. Arm vor Block.

### MODVOLC — Block steht, Manifestation offen (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der `modvolc-cdn.yml`-Lauf
- **Lage:** (gemessen 2026-10-13) Block `phi/sources.φ:12006-12014` geschrieben (`url …/nrt3.modaps.eosdis.nasa.gov/modvolc_thermal.bin`, `terms PD https://www.earthdata.nasa.gov/`, `format modvolc_thermal`, `origin …/archive/allData/61/MODVOLC/Recent`, `compiler tools/harvest/src/bin/modvolc_compiler.rs`, `at earth`, `ttl 2592000`, `field modvolc_thermal modvolc_thermal inverse-square em w/m2/sr/um 86400…`); Einheit `w/m2/sr/um` in `units.rs:113` zugelassen. **Riss:** LANCE nur NRT; Archiv 2000–2019 bei LAADS hinter Login.
- **Blockade:** keine (CI-Dispatch = Mycelium).
- **Braucht:** `gh workflow run modvolc-cdn.yml` (Mycelium); danach Asset im Register-Check. `docs/SOURCE_PORT.md`.

### Registerlücken ohne Lese-Arm — je Compiler + Arm, dann Block (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** je der Bau des fehlenden Compilers/Arms
- **Lage:** (gemessen 2026-10-13, Taucher) je fehlt der ganze Pfad:
  - `xfer:mycelium-342-1` (ICGEM): kein Compiler, kein Arm; `icgem.gfz.de/getmodel/gfc/<sha256>/XGM2019e.gfc`.
  - `xfer:mycelium-342-2` (gamma-cat), `-4` (GEOFON, Draft `state/mycelium/block-drafts/welle2-compiler.φ:1-8`), `-3` (SEANOE, DOI `10.17882/42182`, Lizenz `pending`): kein Block, kein Compiler, kein Arm.
  - `xfer:mycelium-342-5` (Fermi-4LAC): Block `phi:14553-14558` (`format fits`), generischer FITS-Arm da, keine F4LC-Feldbindung/Compiler.
  - `xfer:mycelium-343-3a` (Venus Radar Multi-Look, PDS, DOI `10.17189/1522443`, PD-USGov, ~13,4 GiB > 2-GiB-Limit `cdn.rs:230`); `-3b` (ALFALFA UCHVC, VizieR `J/ApJ/768/77`, Riss AAS-CC-BY-NC); `-3c` (IGETS: `origin` `phi:10305` alter Portal-URL, Download sftp+Registrierung `blocked account`).
- **Blockade:** keine (Bauarbeit).
- **Braucht:** je Quelle Compiler + extract-Arm, **dann** Block (Arm vor Block). `docs/SOURCE_PORT.md`.

### Restcompiler-Formate + `ttl` (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Register-Atom
- **Lage:** (gemessen 2026-10-13 via mycelium) `xfer:…Ph.1` — `heasarc_w3browse`/`catalog_mpcorb`/`cosmic_ro`/opentopography-Witness+Record emittieren den Block uncommitted. `xfer:…Ph.2` — `ttl` für `osc`/`chime_frb_cat2`/`lhaaso` zurückgenommen (gewürfelt verboten); `register_sort` bricht ohne `ttl` (exit 2); die `ttl 31536000` auf `lamost_dr10`/`cfa_co` ebenso ungemessen.
- **Blockade:** keine.
- **Braucht:** je Format den gemessenen Block committen; je Kanal das gemessene `ttl` (oder `no-cadence`) eintragen.

### `register_sort`-Kanonisierung (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste freie Register-Atom
- **Lage:** (gemessen 2026-10-13 via mycelium) `xfer:…Ph.3` — `register_sort --write phi/sources.φ` meldet 4 ttl-order + 22 url-order Verletzungen auf der ttl-ergänzten Fassung.
- **Blockade:** keine.
- **Braucht:** `cargo run -p omegaflow-utils --bin register_sort -- phi/sources.φ --write` (Mountain schreibt), danach `commit_check`.

### LHAASO `at`/`field` + CI-Fetch-Wiring (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der `lhaaso`-Registeratom
- **Lage:** (gemessen 2026-10-13 via mycelium) `xfer:…NmPS.2` — `at`/`field` für den LHAASO/VizieR-Block (`phi/sources.φ:10799-10802`) fehlen; kein CI-Workflow ruft `lhaaso_compiler`.
- **Blockade:** keine.
- **Braucht:** `at`/`field` setzen + Workflow `lhaaso-cdn.yml` anlegen/rufen.

### Welle 3 — HiPS-HEALPix-WCS-Arm + 4LAC (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der `fits.rs`-Armatom
- **Lage:** (gemessen 2026-10-13 via mycelium) `xfer:…shj5QxDF.3` — HiPS-HEALPix-WCS-Arm in `src/archivar/fits.rs` nötig; 4LAC als eigene Quelle oder gestrichen? (offenes Verdikt).
- **Blockade:** keine.
- **Braucht:** den HEALPix-WCS-Arm bauen (eigene Datei, `fits.rs` ist fremd-dirty) oder 4LAC sauber stellen.

### river-Registerentwürfe — acht Fäden + SABER/FPI/Athabasca (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Register-/Armatom
- **Lage:** (gemessen 2026-10-13 via river-208) Entwürfe: KiDS DR4.1 WL (CC-BY-4.0) · Conicus3D (CC-BY-4.0) · Auger Open Data (CC-BY-SA-4.0, Share-Alike-Gate prüfen) · WALLABY/CGPS HiPS3D (CDS-Terms) · EPTA DR2 (`zenodo.org/records/8300645`, CC-BY-4.0) · JSOC-HMI (Lizenz offen) · 1LHAASO (`J/ApJS/271/25/catalog`) · RAVE DR6 (`III/283/ravedr6`) / GALAH DR3 (`J/MNRAS/506/150/spect`) — `state/river/missing-threads-register-entwuerfe-2026-10-13.md`. **SABER** (`xfer:river208-saber1`): Form (C), Level-Index 0…399 als geschlossene Achse, Entwurf `state/river/weberin-register-entwuerfe-2026-10-13.md` Block 5. **UCAR-FPI** (`xfer:river208-ucarfpi1`): Arm `fpi_ucar.rs` steht (`400de8ce2`, `extract.rs:159`), zwei `field`-Zeilen `fpi_mer_wind_ms`/`fpi_zon_wind_ms` offen. **Athabasca ERG-SC** (`xfer:river208-athabasca1`): `url`/`format cdf`-Zeile offen; DOI `10.34515/DATA.GND-0004-0002-0201_v01`.
- **Blockade:** keine.
- **Braucht:** je Entwurf die `field`-Spalten-Zuordnung + `terms`/`ttl`, dann Block; SABER die `channel`/`field`-Zeile nach Form (C). `docs/SOURCE_PORT.md`.

### CelesTrak-EOP — Riss: Asset 404 vs. einziger EOP-Träger (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** eine Lizenzmessung von `celestrak-usage-policy` oder eine Ersatz-EOP-Quelle
- **Lage:** (gemessen 2026-10-13) `archive_search --verdict` auf `…/releases/download/celestrak.org/celestrak_eop.bin` = Stage 1/2 **404**, Stage 3 Wayback 200 → Asset offline. Der Block `phi:10135-10144` trägt die **einzigen** `eop_iers_*`-Zeilen im Register (`eop_iers` = 3, alle dort). Die mycelium-Behauptung „USNO/MAIA trägt EOP" ist **nicht** vom Baum getragen (`sgrep -i maia/usno phi/sources.φ` = 0). `terms`-URL zeigt auf die CSV, nicht auf eine Policy-Seite.
- **Blockade:** kein zweiter EOP-Träger; Lizenz `celestrak-usage-policy` ungemessen.
- **Braucht:** entweder die CelesTrak-Policy live messen und `terms` korrigieren + Release neu hochladen, oder eine offene EOP-Quelle (IERS/USNO) registrieren. Kein stiller Abzug.

### IERS-EOP-Rest — konsumloser `finals`-Pfad (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste `extract.rs`/`channels.rs`-Aufräum-Atom
- **Lage:** (gemessen 2026-10-13) `build_finals_channels` (`channels.rs:1016`) + `format finals` (`fetch.rs:1178`) konsumlos.
- **Blockade:** keine (`cargo check` 0/0 — eine `pub fn` warnt nicht).
- **Braucht:** die toten `finals`-Zweige entfernen oder als bewusstes Legacy benennen.

### ALFALFA — Lizenz `pending` → `local-only` (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** ein Lizenz-Fund oder die Wiedervorlage der Neptun-/HI-Kalibrierung
- **Lage:** (gemessen 2026-10-13) DOI `10.26093/cds/vizier.18610049`; `rights` = `Refer to CDS usage`, keine offene Content-Lizenz messbar; kein Block.
- **Blockade:** die Datenlizenz ist nicht offen belegt.
- **Braucht:** als `local-only` führen (Arbeitskopie `data/`, keine Manifestation).

### Ersatz-Bau — offene Reste (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Build-Schritt / der freigegebene Reader
- **Lage:** (gemessen 2026-10-13) registriert: BGR, CEEIN, IRSF J, LHAASO WCDA+KM2A, jpl_sbdb.
  - **`opentopography_raster` bbox = KAPU — RISS.** Gewählt `48.9,49.9,-82.9,-81.7` (KAPU `on earth 49.392 -82.320 179.94`); A-Stimmen: Toleranz + Höhenbezug (orthometrisch vs. ellipsoidisch) VOR der Prüfung festlegen, mehrere Kontrollpunkte; Kachel-Registrierung **nicht** einfach durchziehen.
  - **Table-Katalog-Verdrahtung = eigene Arbeit.** Reader `src/archivar/osc.rs`/`lamost.rs`/`heasarc_w3browse.rs` + Verdrahtungshunks = eigenes, ungefalletes folge336; `load_gestalt_surface_threads` (`main_flow.rs:580`) an einen Produktions-Aufrufer binden.
- **Blockade:** keine.
- **Braucht:** `./target/release/opentopography_raster_compiler --bbox 48.9,49.9,-82.9,-81.7 --out data/topex.ucsd.edu/srtm_kapu.bin` (nach CI-Build); Table-Reader block-scoped committen. `docs/SOURCE_PORT.md`.

### Eigene Ephemeride — unregistrierte Quellen (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Bau der jeweiligen Quellenklasse
- **Lage:** (gemessen 2026-10-13) Compiler committet (`010ccb657`): `ilrs_npt_compiler` (Arm in `extract.rs` offen), `icrf3_compiler`, `pds3_bip_radar_compiler` (`pds3_img`); Blocks + CDN-Workflows fehlen. `.secrets.local`: `EARTHDATA_EDL_TOKEN` · `NSSDC_USER`/`PASS` · `SSDC_USER`/`PASS`. **Riss (Rat):** `icrf3` auf SKD1 erst nach Vertrags-Versionierung + Round-Trip gegen `skydirection.rs` + Golden-File.
- **Blockade:** keine (Arme eigen).
- **Braucht:** je Klasse Block + CDN-Workflow; ILRS-Format-Arm; icrf3 Round-Trip. `docs/SOURCE_PORT.md`.

### Register-Force-Unit-Schuld (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste `units.rs`-Registeratom
- **Lage:** (gemessen 2026-10-13) `commit_check` blockierte an 5 Paaren außerhalb der Registry (`diffusion kg/m2/s` `:1417`/`:14740` · `diffusion μmol/kg` `:1735` · `seismic-body km/s` `:28556`/`:29211` · `em K` `:29022` · `em nmgy` `:29093`); in `docs/specs/force-unit-baseline.txt` eingefroren.
- **Blockade:** keine.
- **Braucht:** je Paar ein Verdikt + Admission in `src/archivar/units.rs` oder Korrektur der Register-Zeile — dann die Freeze-Zeile löschen.

### Risse CHIME/KASI/IRIS/IERS-EOP/MPC (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** je Registeratom
- **Lage:** (gemessen 2026-10-13 via mycelium, `xfer:mycelium-341-1`) offene Risse, nicht geglättet.
- **Blockade:** keine.
- **Braucht:** je Riss die beiden Linien benennen und am Register schließen.

## An mycelium

Origin: mountain-341 (2026-10-13):

- `xfer:mycelium-343-1` **MODVOLC-Block geschrieben** (`phi/sources.φ:12006-12014`, `terms PD`, `origin LANCE/Recent`) — bitte `gh workflow run modvolc-cdn.yml`, dann neu manifestieren.
- `xfer:mycelium-343-2` **vier Terms-Korrekturen gearbeitet** — `argo_bgc`→`free-open`, `srtm15_plus_v2.5.5.nc`→`PD`, `cams_so2`→`free-open`, `phi:8177`→`eccc-end-use-licence`; `xfer:mycelium-342-10` `wdc-kyoto-scientific-noncommercial` gesetzt.
- **CelesTrak** (`xfer:mycelium-342-9`): Asset 404 gemessen; der Block ist der **einzige** EOP-Träger, USNO/MAIA existiert nicht im Baum — kein Abzug ohne Ersatz. Bitte den Riss mit offener EOP-Quelle schließen.

## An river

Origin: mountain-341 (2026-10-13) — Antwort auf river-208:

- **`extract.rs` blockiert den ENTSO-E-Arm** — deine uncommitteten cdf-Hunks liegen in derselben Datei; bitte committen, dann schreibt Mountain den `entsoe_unavailability`-Arm.
- `xfer:river208-*` (faeden1/saber1/ucarfpi1/athabasca1) sind gefaltet und offen (Punkt „river-Registerentwürfe") — die `field`-Spalten-Zuordnung folgt von Mountain.

## Operator-Wort-Register

- **„Starte die Mountain-Linie in einem Pass — kein Planungstheater …"** | 2026-10-13 | (mountain 337–341) — Linien-Start = session-weiter Consent, getrennt vom Commit-Wort.
- **„so wie mit allem lokal für Experimente sind nicht offene Lizenzen ok, aber nicht veröffentlichen"** | 2026-10-13 | (OCNS2019/ALFALFA — local-only).
- **„wie oft soll ich euch noch sagen … modelle auf max … Search/Max ist Pflicht vor dem Senden, verifiziert"** | 2026-10-13 | (mountain 340).
- **„das muss strukturell gefixt werden jeder eintrag bekommt einen eindeutigen übergabetoken"** | 2026-10-12 | (xfer-Token-Regel).

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort („mach das ab jetzt automatisch",
2026-10-07) trägt Commit und Push. In diesem Atom (folge341) committet: diese Übergabe +
konsumierte folge340 → `archiv/`; `phi/sources.φ` (5 Terms-Korrekturen + MODVOLC-Block).
