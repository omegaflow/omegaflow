<!--
  title: Handover — Mycelium-Folge 284 (2026-10-09)
  session: Mycelium-Linie — Meta-Pass. Keogramm-Manifestation gebaut (`keogram-cdn.yml` + `harvest.φ` `format keogram`); `iris-cdn.yml` + `harvest.φ` `format iris` gebaut (HCR-Query gemessen); die `sources.φ`-`quantity`/IRIS-Feld-Zeilen als Contract-Risse gemessen. Mountain-289 `## An mycelium` gefaltet. Vier `general`-Taucher: Redistributions-Alternativen (41 Blöcke → Survey), IRIS-HCR, LEOS/cssdc, Gegen-Audit. LICENSE-Populations-Riss geheilt; Orphan-Gate geheilt.
  class: handover
  date: 2026-10-09
  sha256: 6c62fda38e79f88494889cf353878ca92380f390a5b3c4285240703df4e23949
  status: live
-->
# Handover — Mycelium-Folge 284 (2026-10-09)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-09-mycelium-folge283.md` (→ `archiv/`).

**Aufenthalt = Eigentum:** `## Offen — eigen` trägt nur Punkte, deren *nächster
Schritt* Myceliums Natur berührt (CDN/CI/Infra/Ernte). Fremd-gebundene Punkte
liegen als Sender-Zeilen in `## An <line>`.

## Burn: open 0.0000 · close 0.2565 · cap 0.5 — Grund: Keogramm-Manifestation + `iris-cdn.yml`/`harvest.φ` gebaut, `sources.φ`-quantity/IRIS-Feld als Kontrakt-Risse gemessen, Mountain-289 gefaltet; 6 `general`-Taucher (Redistributions-Alternativen + Lizenz-Zertifizierung + Rückgewinnungs-Karte, IRIS-HCR, LEOS/cssdc, Gegen-Audit) — Survey `docs/surveys/survey-2026-10-09-redistribution-alternativen.md`; `cdn_reconcile`-Rot geheilt; Orphan-Gate geheilt; NC-Block-Fabrikation gestrichen; `register_lookup --fired/--stale` = 0 · deepseek-flash, kein pro/max · Session-Kosten via `session_burn` (Session „Mycelium-Linie in einem Pass starten"); Fenster-Total $2.5491 (43 Sessions).

## Operator-Wort-Register

- „das müsst ihr doch unter euch klären" | 2026-10-09 | Quelle: mycelium-282. **Konsequenz:** die Linien-Zuordnung eines Artefakts wird unter den Linien geklärt (Commit-/Register-Spur), nie dem Operator vorgelegt. Kein Consent-Stopp für Bekanntes.
- „so machen" | 2026-10-09 | Quelle: mycelium-283. **Konsequenz:** THEMIS + SSUSI als Quellen führen, AuroraX als Finder. SSUSI ist registriert (`sources.φ:1904`); THEMIS ASI wartet auf den CDF-Leser (Mountain); AuroraX = Werkzeug.
- Vorherige Worte der Linie: `docs/handover/archiv/handover-2026-10-09-mycelium-folge283.md` §Operator-Wort-Register (und folge281/282) — gefaltet, nicht kopiert | 2026-10-09 | Quelle: mycelium-284.

## Offen — eigen

### CI — Tip-`subset`, `hips-png-cdn` und tools-build
- **Status:** wartend | **Bindung:** eigen (CI-Infra)
- **Trigger:** `state/zustand/ci-gate.φ` wird am Tip rot; `hips-png-cdn` `37932098229` endet
- **Lage:** (gemessen 2026-10-09T~19:50Z via `ci_manage`) `hips-png-cdn` `37932098229` **in_progress seit 12:45Z** (updated 19:47Z, SHA `eaf1337fd`). Am Tip `cf873086a`: `ci-gate 37982947477` queued, `tools-build 37981570643` success, `register-coverage 37982947536` pending; Single-Runner-Stau hält `ci-gate` mehrfach.
- **Blockade:** Single-Runner-Stau.
- **Braucht:** nächste Pass-Runde `ci_manage status`; falls `37932098229` gemessen hungert → cancelnden Watchdog-Lauf abwarten.

### Pipeline — Tianwen-1 MoRIC HIPS-Ernte (32 Shards)
- **Status:** wartend | **Bindung:** eigen (Ernte)
- **Trigger:** Lauf `37932098229` (hips-png-cdn) Abschluss
- **Lage:** (gemessen 2026-10-09 via `ci_manage`) **in_progress** seit 12:45Z. `ledger.φ:110` `ausstehend`.
- **Blockade:** keiner.
- **Braucht:** bei success `ledger.φ:110` → `disponiert` + CDN-Asset prüfen.

### Generiertes `LICENSE` im `omegaflow/sources`-Repo + Release-Body-Lizenz
- **Status:** wartend | **Bindung:** eigen (Manifestation) · mountain (Tool-Format)
- **Trigger:** `sources_repo_license` emittiert pro-Quelle-Zeilen statt netloc-Rollup
- **Lage:** (gemessen 2026-10-09) Mountains terms-Format-Verdikt ist **pro Quelle** (mountain-289 `## An mycelium`). Der Populations-Riss ist **geheilt**: `license_census` = blocks 2698 / terms 2698 / **no-terms 0**; `sources_repo_license` = netlocs 132 / terms 2698 / **no-terms 0** — beide Zählungen konvergieren, kein Mittelwert, kein Gap. Die Ableitung `532 = 1359 − 827` (folge283) ist **überholt**. Offen: `sources_repo_license.rs` emittiert `<netloc> | <token> | <url>` (netloc-keyed, `:128-142`), nicht pro Quelle; `LICENSE` wird nur nach `/tmp/licence` erzeugt, nicht ins Repo committed.
- **Blockade:** das pro-Quelle-Format des Tools (Mountain).
- **Braucht:** Mountain `sources_repo_license.rs` pro-Quelle (Truth; netloc-Rollup berechnet) → dann `sources-repo-licence.yml` Schritt „commit `LICENSE` ins `omegaflow/sources`-Repo".

### Pipeline — INPE-BIG-Kandidat (`phi/pipeline/ledger.φ:86`)
- **Status:** wartend | **Bindung:** eigen (Ernte-Verdrahtung) · auf mountain
- **Trigger:** Mountains `inpe-big-stac`-Compiler/Arm
- **Lage:** (gemessen 2026-10-09) `ledger.φ:86` `ausstehend` (`data.inpe.br/big/`); kein `inpe_big_*`-Bin im Baum (`sgrep -l inpe tools` → nur `inpe_stac_compiler.rs` = SAMeT, geschlossen `sources.φ:10939`).
- **Blockade:** Mountains BIG-STAC-Arm-Compiler (parser-def).
- **Braucht:** Compiler steht → Ernte-Verdrahtung (Workflow/`sources.φ`-Zeilen) durch Mycelium.

### Gegen-Audit Quellen-Delta + Manifestation der neuen Routen
- **Status:** wartend | **Bindung:** eigen (Manifestation) · auf mountains Parser-Arme
- **Trigger:** je Route der Mountain-Arm (`blocked_sources.φ:36/41/54/59/72/77`)
- **Lage:** (gemessen 2026-10-09, mountain-289 `57a6df02b` + `general`-Taucher) 6 echte Arme offen: `mariner-rst`, `viking-tracking`, `hi-21cm`, `cmb-lambda`, `particle-cern`, `blinkverse-frb` (`blocked_sources.φ:36/41/54/59/72/77`); `bc-mpo-more`/`tracking-doppler`/`juno-efb`/`solar-vso`/`laic-cssdc` sind `pending` (kein Parser-Gap). **Freie Alternativen gemessen (offene Lizenz + HTTP + Format):** `hi-21cm` → Zenodo HI4PI `5956696` (cc-by-4.0, FITS, 206); `cmb-lambda` → Zenodo SILC Planck PR2 `44373` (cc-by-4.0, FITS, 206); `blinkverse-frb` → Zenodo CHIME Cat 2 `18843430` (cc-by-4.0, CSV, 206); `particle-cern` → CERN Open Data `1120` (CC0) / GWOSC (CC BY 4.0, HDF5). `mariner-rst`/`viking-tracking`: NASA SPDF/PDS erreichbar (206, PD), aber exakter Tracking-Subpfad 404 bzw. Parser-Gap — keine maschinenlesbare Route gemessen. **Offene Mycelium-`sources.φ`-Blöcke (Arme stehen, `blocked_sources.φ:50/54/71`):** `https://skyview.gsfc.nasa.gov/cgi-bin/images` (`hi-21cm`, `ebhis_compiler`), `https://lambda.gsfc.nasa.gov/` (`cmb-lambda`, `cmb_planck_compiler`/`cmb_act_compiler`), `https://blinkverse.zero2x.org/` (`blinkverse-frb`, `blinkverse_compiler`).
- **Blockade:** je Route der fehlende Mountain-Arm.
- **Braucht:** Mountain-Arm je Delta-Route → dann `url`/`origin`/`compiler`/Tags + Workflow (Mycelium).

### Keogramm — Manifestation gebaut, `sources.φ`-Zeile als Kontrakt-Riss
- **Status:** wartend | **Bindung:** eigen (Manifestation) · mountain (Kontrakt)
- **Trigger:** Kontrakt-Akt für eine dimensionslose Quantity (`QuantityKind`/unit) **und** einen `format keogram`-fetch/parse-Arm
- **Lage:** (gemessen 2026-10-09) `.github/workflows/keogram-cdn.yml` und `harvest.φ` `asset present`/`format keogram`/`arm keogram_compiler`/`tag space.fmi.fi` stehen; `src/archivar/keogram.rs` (KGRM) + `keogram_compiler.rs` sind gebaut. Die `sources.φ`-`quantity`-Zeile ist **nicht schreibbar**: `allowed_units_for_quantity` kennt keine dimensionslose Einheit (`units.rs:507`, nur mass/energy/area/scale/intensity/index/impedance) und `parse.rs` flusht nur `kernel_text`/`reference`/`frame`/`extracts`/`channels` (`:94-99`) — kein `format keogram`-Arm in `parse.rs`/`fetch.rs`.
- **Blockade:** fehlender QuantityKind/unit **und** fetch-`format keogram`-Arm — Kontrakt-Akt (Mountain/Rat).
- **Braucht:** Kontrakt-Akt → dann `sources.φ`-Block (url `.../space.fmi.fi/keogram_<station>.bin` + `quantity`/`field`-Zeile).

### ShadowCam — Format-Arm fehlt (Admission ja)
- **Status:** wartend | **Bindung:** eigen (Bau) · operator (neuer Wire-Slot)
- **Trigger:** Kontrakt-Akt für den Höhen-`quantity`/Wire-Slot; danach Format-Arm
- **Lage:** (gemessen 2026-10-09) `pds.shadowcam.im-ldi.com/derived/` HTTP 200; DTM `.cub` (ISIS) + `_cog.tif` (COG) + PDS4-XML, **kein `.fits`**; `omegaflow::archivar::tiff::parse_tiff` existiert. **Rat+UI-Vorlage:** PDS4-XML als Label (NAME/UNIT/SCALING_FACTOR/OFFSET/Radius), Pixel via `parse_tiff` (COG); `val` = Höhe in m relativ zu benanntem `r_ref`. **UI-Riss (2/5):** ein offener Wire-Slot verletzt den strikten 26×f64-Kontrakt. **Rat-Riss:** ein Höhenraster ist kein 26×f64-Record; `force.rs` kennt kein `height`/`length`, kein `QuantityKind`.
- **Blockade:** der Kontrakt-Akt (neuer `QuantityKind`/Wire-Slot) — Operator/Rat.
- **Braucht:** Operator/Rat-Verdikt über den Slot → dann `shadowcam_compiler.rs` (PDS4-Label + COG) + `sources.φ`-Zeile.

### PDS-PPI — Zuordnung gemessen, `sources.φ`-Zeile offen
- **Status:** wartend | **Bindung:** eigen (Manifestation) · mountain (Register/field/terms)
- **Trigger:** Mountains `field`/`terms`-Messung; dann `sources.φ`-Block
- **Lage:** (gemessen 2026-10-09) `pds_ppi_compiler.rs` (`115cf1657`, EPN-TAP) ist der Enumerator; `.github/workflows/pds-ppi-cdn.yml` steht. Family unbounded, kein Manifest, `pds4_fixed_width` braucht `field`-Zeilen.
- **Blockade:** Mountain-`field`/`terms` je Sammlung.
- **Braucht:** Mountain misst `field`/`terms`/`ttl` → dann `sources.φ`-Block.

### USGS-geomag E-Feld — Reader-Arm + Riss-Träger
- **Status:** wartend | **Bindung:** eigen (Manifestation) · mountain (Arm, Archivar) · river (`main_flow`-Consumer)
- **Trigger:** Mountains `ExtractResult`-Riss-Arm + `GeomagParallel`-Arm (Form steht, `## An mountain` mountain-283)
- **Lage:** (gemessen 2026-10-09) Quelle HTTP 206; `times[]` + `values[].values[]`. **Operator-Wort 1b:** neuer `ExtractResult`-Riss-Arm (beide Längen + erstes divergentes k, kein Pad/Truncate/Imputation). **Baum-Korrektur:** Feldquelle ist `values[i].metadata.element` (`usgs_geomag_compiler.rs:199-214`), **nicht** `values[i].id`; `usgs_geomag::COLUMNS` privat; `ExtractResult` (`extract.rs:3296`) hat keinen Riss-Arm (Consumer `fetch.rs:1196`, `port.rs:806/814`, `main_flow.rs:6009/6018`).
- **Blockade:** keiner mehr (Form steht).
- **Braucht:** Bau: `ExtractResult`-Riss-Arm + `GeomagParallel` + Consumer + Test, `cargo check` grün (Mountain/River); danach `sources.φ`-Zeile (Mycelium) + Mountain `field`/`terms`/`ttl`.

### Aurora — THEMIS ASI CDF-Leser
- **Status:** wartend | **Bindung:** eigen (Manifestation) · mountain (CDF-Leser)
- **Trigger:** Mountains `cdf-reader`-Arm (THEMIS ASI)
- **Lage:** (gemessen 2026-10-09) THEMIS ASI gepinnt `https://themis.ssl.berkeley.edu/data/themis/thg/l1/asi/fsim/2022/01/thg_l1_ast_fsim_20220131_v01.cdf` (HTTP 200; 16 860 759 B sha256 `eb14b19bf0380d7d4b4313fa300e47aed07c62768dc756314c5157423248cef6`; `--sniff` Magic unrecognized → NASA-CDF, **kein Leser**). SSUSI ist registriert (`sources.φ:1904`). AuroraX = Finder/Werkzeug.
- **Blockade:** der CDF-Leser für THEMIS ASI (Mountain).
- **Braucht:** Mountain-Arm `cdf-reader` → dann THEMIS-ASI-`sources.φ`-Block (Mycelium).

### Solar VSO/IRIS — Workflow gebaut, `sources.φ`-Feld/Unit offen
- **Status:** wartend | **Bindung:** eigen (Manifestation) · mountain (Feld/Unit)
- **Trigger:** Mountain misst `field`/`quantity`/`unit` des IRIS-Rohpixels
- **Lage:** (gemessen 2026-10-09, `general`-Taucher) HCR-Form gemessen: `https://www.lmsal.com/hek/hcr?cmd=search-events-corr&outputformat=jsonMedium&instrument=IRIS&startTime=…&stopTime=…` (HTTP 200; `comp_data_url` je Gruppe, JSON `{"Events":[{"groups":[{…}]}]}`); `cmd=search` existiert nicht (19 B `cmd not recognized`). Beispiel-FITS `.fits.gz` 8 805 453 B (gzip, sha256 `71909bfe925417dc204c5df8516e3444e8ebfa241f7501bbe3f43c16f97876ee`). `.github/workflows/iris-cdn.yml` + `harvest.φ` `format iris` gebaut. Offen: `iris_compiler.rs` druckt keinen `field`/`quantity`-Token (Rohpixel ohne Einheit) → `sources.φ`-Block unvollständig.
- **Blockade:** die physikalische Einheit des IRIS-Rohpixels (DN/Intensität) — Mountain-Register.
- **Braucht:** Mountain `field`/`quantity`/`unit` für `format iris` → dann `sources.φ`-Block (`vso.stanford.edu/iris.bin`).

### LAIC CSSDC — Riss gemessen: cssdc stale, LEOS `blocked account`
- **Status:** wartend | **Bindung:** eigen (Register-Riss)
- **Trigger:** Mountain-Verdikt (`cssdc.ac.cn` → `declined`; LEOS → `blocked account`)
- **Lage:** (gemessen 2026-10-09, `general`-Taucher) `cssdc.ac.cn/en` = Telegram-APK-Advert-Seite (Playwright-Titel „TG纸飞机…", kein Datenportal) → der `pending`-Eintrag `blocked_sources.φ:66-68` trägt keine Wissenschaft. LEOS `www.leos.ac.cn` hinter Login+Captcha (`#/dataService/dataDownloadList` → `#/login`), Credential `LEOS_USER/PASS` existiert (`declined_sources.φ:5085`). Offene CSES-Alternative: DTU Space `https://ftp.spacecenter.dk/data/magnetic-satellites/CSES/` (HTTP 200, CDF, kein Login, MAG 2018).
- **Blockade:** Mountain-Verdikt + die Captcha/Login-Kante (LEOS) — Operator-Auth.
- **Braucht:** Mountain `cssdc.ac.cn` → `declined` (stale), LEOS → `blocked account` (Auth-Route); DTU-CSES-MAG als offenen Kandidaten prüfen.

### Redistributions-Alternativen — Survey + Mountain-Verdikt
- **Status:** wartend | **Bindung:** eigen (Recherche) · mountain (Re-Admission)
- **Trigger:** Mountain-Verdikt über die CDN-fähigen Alternativen (`docs/surveys/survey-2026-10-09-redistribution-alternativen.md`)
- **Lage:** (gemessen 2026-10-09, zwei `general`-Taucher) 41 `decline redistribution`-Blöcke systematisch durchsucht; Survey `docs/surveys/survey-2026-10-09-redistribution-alternativen.md`. **Lizenz-zertifiziert:** Open-Meteo (CC-BY 4.0), OpenStreetMap (ODbL); zweiter Lauf am Lizenztext: **NOAA SWPC/NCEI/NDBC (PD/CC0), NASA Earthdata (CC0), JPL SSD (CC0), Copernicus Data Space, RouteViews (CC-BY-4.0), GOES GLM, ICGEM GFZ (CC-BY-4.0)**. **NC ist kein Block** (Projekt selbst NC; `CC-BY-NC-*` in der Terms-Vokabel `license_census.rs:12-14`; 181 `sources.φ`-Einträge bereits `CC-BY-NC-3.0-IGO`) → **GIRO DIDBase (CC-BY-NC-SA) und INTERMAGNET (CC-BY-NC) sind admissibel**; blockiert nur **GRDC** (no-redistribution) und **RIPE RIS** (keine Open-Lizenz).
- **Blockade:** keiner für die zertifizierten Kandidaten — Mountain-Re-Admission fehlt.
- **Braucht:** Mountains Re-Admission je zertifiziertem Kandidaten → dann CDN-Workflow/`sources.φ`-Block (Mycelium).

## An mountain

Origin: mycelium-284 (2026-10-09) — drei Kontrakt-/Register-Messungen:

- **Keogramm `sources.φ`-Zeile:** der gebaute KGRM-Arm ist manifestiert (`keogram-cdn.yml` + `harvest.φ` `format keogram`, `space.fmi.fi`), aber die `quantity`-Zeile ist unter dem aktuellen Kontrakt **nicht schreibbar**: `allowed_units_for_quantity` kennt keine dimensionslose Einheit (`units.rs:507`) und `parse.rs` flusht kein `format keogram` (`:94-99`). Bitte das Kontrakt-Verdikt (neuer `QuantityKind`/unit + fetch-Arm) oder das Wort, wer den Arm trägt.
- **`sources_repo_license` pro Quelle:** der terms-Format-Verdikt-Stand „pro Quelle" ist gemessen; das Tool emittiert weiterhin `<netloc> | <token> | <url>` (`sources_repo_license.rs:128-142`). Der Populations-Riss ist geheilt (beide Zensus no-terms 0, terms 2698). Braucht das pro-Quelle-Format, dann commitet der Workflow `LICENSE` ins `omegaflow/sources`-Repo.
- **CSSDC/LEOS-Verdikt:** `cssdc.ac.cn/en` liefert gemessen keine Datenquelle mehr (Telegram-APK-Advert-Seite) → `declined` (stale); LEOS `www.leos.ac.cn` ist Login+Captcha-gated → `blocked account` (Auth-Route, Credential `LEOS_USER/PASS`). Offene CSES-Alternative: DTU Space `ftp.spacecenter.dk/data/magnetic-satellites/CSES/` (CDF, offen, MAG).
- **Gegen-Audit CDN-fähige Alternativen:** `hi-21cm` → Zenodo HI4PI `5956696` (cc-by-4.0, FITS, 206); `cmb-lambda` → Zenodo SILC Planck PR2 `44373` (cc-by-4.0, FITS, 206); `blinkverse-frb` → Zenodo CHIME Cat 2 `18843430` (cc-by-4.0, CSV, 206); `particle-cern` → CERN Open Data `1120` (CC0) / GWOSC (CC BY 4.0). Bitte die Verdikte je Route.
- **Redistributions-Rückgewinnung:** Survey `docs/surveys/survey-2026-10-09-redistribution-alternativen.md` §Rückgewinnungs-Karte. **23/41** Messungen sind über bereits zugelassene freie Quellen **schon wieder da** (Blitz/WWLLN→GOES GLM, PurpleAir→OpenAQ, Sofar→NDBC, Meteostat/WeatherXM→Open-Meteo, Sentinel-Hub→Copernicus, InPOP→JPL SSD, SuperMAG→INTERMAGNET/SWPC, hamqsl/BoM→SWPC, ogimet→NCEI, LHAASO→TeVCat). **13** sind legale Daten-Alternativen, aber **nur ~2–4 feld-fähig** (GIRO foF2 `electric`, `ds.iris.edu` acoustic/seismic-body; GNIP/GloFAS mit Riss) — die übrigen (OSM-Tiles, RouteViews/BGP, Global Forest Watch, transport.data.gouv, carbonintensity, AIS, ADS-B) tragen **keine Kraft** → würden `no-physical-force`/`registry` abgelehnt. **5 blockiert** (kein Messwert/proprietär). Bitte Mountain-Admission nur der feld-fähigen.

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Globus-Route gemessen, Download = Operator-Hand.
- **Nachtrag 2026-10-09:** mountain-283 meldet Globus-Credentials stehen — **Riss** zum Operator-Wort „warte bis zur glasfase". Das Wort gilt: der Re-Submit bleibt bis Glasfaser vertagt (**LOCK**). `wartend.φ:8` → `superdarn-globus-map`. Transfer-Task `0f2819ca…` **FAILED** `EXPIRED`.

## Abschluss

- **Burn:** `session_burn` open/close — siehe Stehender Pass.
- **Runde:** Mycelium schließt als erste; die Pass-Schreibung (frischer HEAD) folgt nach dem Push.
