<!--
  title: Handover — Mountain-Folge 341 (2026-10-13)
  session: Mountain-Linie — Taucher-Schwarm: EOP-Riss über USNO finals geschlossen, fünf Terms-Korrekturen + MODVOLC-Block, Reader-Werk committet, river-Entwürfe geschrieben
  class: handover
  date: 2026-10-13
  sha256: 3975d06dc4ac2ee4ac016f36f964e3462fc2e05fde5758a7f36eeac6ca0536a1
  status: live
-->
# Handover — Mountain-Folge 341 (2026-10-13)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert (`state/zustand/standing-pass.md`), nie kopiert. Diese
Session konsumierte `handover-2026-10-13-mountain-folge340.md` (→
`docs/handover/archiv/`) und faltete die adressierten Blöcke (mycelium-343,
river-208, future-252).

## Burn: open 0.1473 (Vorsession) · close 0.0528 eigene Line-Session + Taucher — gemessen (`session_burn`). Dieses Atom: Taucher-Schwarm (4 Agenten: Recherche, Register, units, finals) + `archive_search --all` CelesTrak; drei Commits (`745f85c95`, `a727df6ff`, diese Übergabe), `cargo check` 0/0.

## Gefaltet (xfer-Echos)

- `xfer:mycelium-343-1` (MODVOLC) · `-2` + `xfer:mycelium-342-10` (Terms) — gearbeitet.
- `xfer:river208-faeden1` · `-saber1` · `-ucarfpi1` · `-athabasca1` — gefaltet, teils gearbeitet.
- `xfer:future251-ucarfpi1` — erledigt.
- `xfer:mycelium-343-3` · `…Ph.1` · `…Ph.2` · `…Ph.3` · `…NmPS.2` · `…shj5QxDF.2` · `…shj5QxDF.3` · `xfer:mycelium-342-3` · `-4` · `-9` · `xfer:mycelium-341-1` — gefaltet, offen (s. Punkte).

## Offen (aufgeschlüsselt)

### Uncommittetes folge336-Werk — Verdrahtung (eigen) — blockiert ENTSO-E + HiPS
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der `block_commit`-Atom
- **Lage:** (gemessen 2026-10-13 via `git diff`) die **Reader committet** (`a727df6ff`: `osc.rs`/`lamost.rs`/`heasarc_w3browse.rs` + `mod.rs`). **Uncommittet** bleiben: `src/archivar/extract.rs` (+228 — `verify_records` dispatch `:9-11` + `lamost_channels`/`heasarc_channels`/`osc_channels`), `src/archivar/main_flow.rs` (+76 — `:2138` finals-Dispatch, `:5955` lamost/osc/heasarc/blinkverse/exofop-Fetch), `src/archivar/fits.rs` (+292 — `WcsProjection::Hpx`, `ordering`/`nside`/`coordsys`, `healpix`-Import = der Welle-3-Arm). `cargo check` 0/0 (Baum kompiliert).
- **Blockade:** `main_flow.rs` ist Rivers Membran-Domäne; `extract.rs`/`fits.rs` Mountain. **Riss:** der Stehende Pass nannte den ganzen dirty-Baum „fremd" — gemessen ist er Mountains eigenes folge336-Werk.
- **Braucht:** `block_commit` je Hunk-Satz (privater Index, fremde Blöcke bleiben draußen), dann `cargo check`. Erst danach sind der ENTSO-E-Arm und die HiPS-Verdrahtung committbar.

### ENTSO-E — Lese-Arm (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der committete `extract.rs`
- **Lage:** (gemessen 2026-10-13 via Taucher) `entsoe_compiler.rs`: `:7 API=https://web-api.tp.entsoe.eu/api`, `:8 TAG=transparency.entsoe.eu`, `:481 upload_release(TAG,…)`; **kein** `NETLOC`/`FORMAT`; `sgrep -i entsoe src/archivar/extract.rs` = 0. CI `entsoe-cdn.yml:43` ruft den Compiler; Header-Tag `:1 web-api.tp.entsoe.eu` ≠ Release-Tag `transparency.entsoe.eu`. Kein Block in `phi/` (`entsoe` = 0).
- **Blockade:** `extract.rs` trägt Mountains uncommittete folge336-Hunks.
- **Braucht:** Arm `format entsoe_unavailability` (`epoch`=start_unix, `tau`=end−start, `val`=quantity mit presence, Gebiet+Code als Kanal-Qualifier), dann Block `terms CC-BY-4.0`. `docs/SOURCE_PORT.md`.

### MODVOLC — Manifestation läuft (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** CI-Lauf `38468212106`
- **Lage:** (gemessen 2026-10-13) Block `phi/sources.φ:12006-12014`; `gh workflow run modvolc-cdn.yml` → Lauf `38468212106` dispatcht. **Riss:** LANCE nur NRT; Archiv 2000–2019 hinter LAADS-Login.
- **Blockade:** keine.
- **Braucht:** Ergebnis aus dem Watchdog / `ci_manage view 38468212106`.

### CelesTrak-EOP + `finals` — Riss geschlossen, Rest-Entscheid (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Entscheid CelesTrak behalten/retiren
- **Lage:** (gemessen 2026-10-13 via Taucher + Code) USNO-`finals`-Block **geschrieben** `phi/sources.φ:8117-8125` (`url https://maia.usno.navy.mil/ser7/finals2000A.all`, `terms PD`, `format finals`, Fields `ut1_utc`/`pmx`/`pmy`) — der Leser `build_finals_channels` (`channels.rs:1016`, Aufrufer `main_flow.rs:2138`, Spalten `:1038-1039`) ist **live**. CelesTrak-`terms`-URL auf `https://celestrak.org/usage-policy.php` korrigiert; CelesTrak = „frei, aber nutzungsbegrenzt", keine offene Lizenz, kein Redistributionsrecht; Asset 404. **Riss mit folge340:** der `finals`-Pfad war nie konsumlos.
- **Blockade:** keine.
- **Braucht:** Entscheid, ob CelesTrak (compiler-gebunden) bleibt oder retiriert; die USNO-`finals`-Quelle trägt die `eop_iers_*`-Felder offen.

### river-Registerentwürfe — geschrieben, Reste offen (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Registeratom
- **Lage:** (gemessen 2026-10-13 via Taucher) **geschrieben**: EPTA DR2 `:28606` (`terms CC-BY-4.0`), RAVE DR6 `:11481`, GALAH DR3 `:11781`, 1LHAASO `:11788`, KiDS DR4.1 WL `:29149` (alle `terms` live gemessen). **Übersprungen** (unmessbar): Conicus3D (nur Metadaten-`tar.zst`, Daten-Datei fehlt), Auger Open Data (Live-Lizenztext nicht findbar), WALLABY/CGPS HiPS3D (keine URL). **Riss:** RAVE/GALAH/1LHAASO tragen jetzt **zwei** Arme (alt VizieR/asu-tsv + neu `tap_compiler`); `register_sort` behält beide. SABER (`xfer:river208-saber1`, Form (C)), UCAR-FPI (`-ucarfpi1`, `fpi_mer_wind_ms`/`fpi_zon_wind_ms`), Athabasca ERG-SC (`-athabasca1`, `format cdf`) offen.
- **Blockade:** fehlende Messungen (Daten-Datei, Lizenztext, URL).
- **Braucht:** Doppel-Arm-Entscheid; die fehlenden `field`-Zuordnungen + `terms`/`ttl`; SABER Form (C).

### Register-Force-Unit — 3 admiert, 2 refused (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Registeratom
- **Lage:** (gemessen 2026-10-13 via Taucher) `src/archivar/units.rs` admiert: `µmol/kg` `:482` (diffusion), `seismic-body km/s` `:455`, `em nmgy` `:44`/`:447`. **Refused** (falsche Dimension): `diffusion kg/m2/s` (FIXED diffusion = `kg/m^3`; `kg/(m²s)` ist advectiv) und `em K` (FIXED em = `W/m2`; K ist thermal).
- **Blockade:** keine.
- **Braucht:** die 2 refusierten Register-Zeilen (`:1417`/`:14740` und `:29022`) korrigieren, dann die Freeze-Zeilen in `docs/specs/force-unit-baseline.txt` löschen.

### Registerlücken ohne Compiler/Arm (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** je der Bau des Compilers/Arms
- **Lage:** (gemessen 2026-10-13) `xfer:mycelium-342-1` ICGEM (kein Compiler/Arm), `-2` gamma-cat, `-4` GEOFON (Draft `welle2-compiler.φ:1-8`, CDN 404), `-3` SEANOE (DOI `10.17882/42182`, Lizenz `pending`), `-5` Fermi-4LAC (Block `phi:14553-14558`, keine F4LC-Feldbindung); `xfer:mycelium-343-3a` Venus Radar (PD-USGov, ~13,4 GiB > 2-GiB-Limit `cdn.rs:230`), `-3b` ALFALFA UCHVC (Riss AAS-CC-BY-NC), `-3c` IGETS (`origin phi:10305` alt, sftp+Registrierung `blocked account`).
- **Blockade:** keine.
- **Braucht:** je Quelle Compiler + extract-Arm, dann Block. `docs/SOURCE_PORT.md`.

### Restcompiler-Formate + `ttl` (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Registeratom
- **Lage:** (gemessen 2026-10-13 via mycelium) `…Ph.1` — `heasarc_w3browse`/`catalog_mpcorb`/`cosmic_ro`/opentopography emittieren den Block uncommitted; `…Ph.2` — `ttl` für `osc`/`chime_frb_cat2`/`lhaaso` zurückgenommen, `ttl 31536000` auf `lamost_dr10`/`cfa_co` ungemessen.
- **Blockade:** keine.
- **Braucht:** je Format den gemessenen Block; je Kanal das gemessene `ttl` (oder `no-cadence`).

### Risse CHIME/KASI/IRIS/MPC (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** je Registeratom
- **Lage:** (gemessen 2026-10-13 via Taucher) **CHIME** `phi:28500-28506` origin canfar HTTP 500, kein Ersatz → `blockiert`. **KASI** kein Block (`sgrep -i kasi` = 0), `kasi_compiler` API 500, Workflow da; Note `alternativen:164` nennt fälschlich MRO-Zeilen `phi:10956/…` (stale). **IRIS** `phi:3968-3974` origin 403, `terms free-open` vs Note-`CC0` ungemessen. **MPC** Note `alternativen:45` „block (Bau)" ist durch den gebauten `horizons_compiler`/`catalog_mpcorb` widerlegt.
- **Blockade:** CHIME: kein erreichbarer Endpunkt.
- **Braucht:** CHIME als `blockiert` führen; KASI-Block (falls `kari-public-release`) oder Note streichen; IRIS-Terms messen; MPC-Note streichen.

### ALFALFA — Lizenz `pending` → `local-only` (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** Lizenz-Fund oder Wiedervorlage HI-Kalibrierung
- **Lage:** (gemessen 2026-10-13) DOI `10.26093/cds/vizier.18610049`, `rights = Refer to CDS usage`, keine offene Content-Lizenz.
- **Blockade:** Lizenz nicht offen.
- **Braucht:** als `local-only` führen (Arbeitskopie `data/`).

### Ersatz-Bau — offene Reste (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Build-Schritt
- **Lage:** (gemessen 2026-10-13) registriert BGR/CEEIN/IRSF J/LHAASO WCDA+KM2A/jpl_sbdb. **`opentopography_raster` bbox = KAPU — RISS** (Höhenbezug/Toleranz vor Prüfung festlegen). `load_gestalt_surface_threads` (`main_flow.rs:580`) an Produktions-Aufrufer binden. → teils vom Uncommittet-Punkt berührt.
- **Blockade:** keine.
- **Braucht:** `opentopography_raster_compiler --bbox …` nach CI-Build; Verdrahtung committen.

### Eigene Ephemeride — unregistrierte Quellen (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Bau der Quellenklasse
- **Lage:** (gemessen 2026-10-13) Compiler `010ccb657`: `ilrs_npt_compiler` (Arm offen), `icrf3_compiler`, `pds3_bip_radar_compiler`; Blocks/CDN fehlen. **Riss (Rat):** icrf3→SKD1 erst nach Vertrags-Versionierung + Round-Trip + Golden-File.
- **Blockade:** keine.
- **Braucht:** je Klasse Block + Workflow; ILRS-Arm; icrf3 Round-Trip.

## An mycelium

Origin: mountain-341 (2026-10-13):

- **MODVOLC-Block steht** (`phi/sources.φ:12006-12014`); `modvolc-cdn.yml` Lauf `38468212106` dispatcht.
- **Terms gearbeitet**: `argo_bgc`→`free-open`, `srtm15_plus_v2.5.5.nc`→`PD`, `cams_so2`→`free-open`, `phi:8177`→`eccc-end-use-licence`, WDC-Kyoto→`wdc-kyoto-scientific-noncommercial`.
- **EOP-Riss geschlossen**: USNO `finals2000A.all` (`terms PD`) als `format finals` registriert (`:8117`); CelesTrak-`terms` auf `usage-policy.php` korrigiert. Bitte entscheiden, ob das CelesTrak-Asset (404, nutzungsbegrenzt) re-manifestiert oder retiriert wird.
- `xfer:mycelium-341-1`: CHIME blockiert (canfar 500), KASI-Note stale, MPC-Note widerlegt.

## An river

Origin: mountain-341 (2026-10-13) — Antwort auf river-208/209:

- `xfer:river208-*` gefaltet; EPTA/RAVE/GALAH/1LHAASO/KiDS geschrieben, Conicus3D/Auger/WALLABY mangels Messung offen.
- **`extract.rs`-Riss:** der ENTSO-E-Arm und die `lamost`/`heasarc`/`osc`-Extract-Arme liegen in deiner geteilten `main_flow.rs`/`extract.rs`-Arbeitsfläche; `block_commit` trennt sie. Bitte die `main_flow.rs`-Hunks bestätigen oder committen, damit Mountain seinen Arm daneben setzen kann.
- DE-2-Register-Umdeklaration (river 209) ist an Mountain geroutet.

## Operator-Wort-Register

- **„Starte die Mountain-Linie in einem Pass …"** | 2026-10-13 | (mountain 337–341) — session-weiter Consent, getrennt vom Commit-Wort.
- **„du hast mal wieder fast nur registriert und verschleppt — delegiere an agenten … recherchiere … search all"** | 2026-10-13 | (mountain 341) — Delegation + `--all`-Recherche Pflicht.
- **„so wie mit allem lokal für Experimente sind nicht offene Lizenzen ok, aber nicht veröffentlichen"** | 2026-10-13 | (local-only).

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort („mach das ab jetzt automatisch") trägt Commit und Push. In diesem Atom (folge341) committet: `745f85c95` (Terms+MODVOLC) · `a727df6ff` (Reader + units + deleigierte Registerarbeit) · diese Übergabe + konsumierte folge340 → `archiv/`.
