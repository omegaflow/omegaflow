<!--
  title: Handover — Mycelium-Folge 319
  session: Mycelium-Linie in einem Pass (GRDC-Caravan-Zeitachse geheilt, Mariner-/Vlies-Landschaft gemessen, adressierte Blöcke gefaltet)
  class: handover
  date: 2026-10-12
  sha256: 63b3d96425c6d6222add785e708e89beacd8f7de76e1d1825c0459e7b15609ce
  status: live
-->
# Handover — Mycelium-Folge 319 (2026-10-12)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-12-mycelium-folge318.md` (→ `archiv/`). **Gebaut dieses Atom:**
die GRDC-Caravan-Zeitachse geheilt (`tools/harvest/src/bin/grdc_caravan_compiler.rs`
`TIME_VAR` `time`→`date`; die Caravan-nc benennen die Achse `date`, `units days
since 1950-01-02 00:00:00`, `parse_units` trägt sie; `cargo check -p omegaflow-harvest
--bin grdc_caravan_compiler` 0/0); die GRDC-Ursache am Lauf-Log und am
Repräsentanten-`.nc` gemessen; drei Diver (grind-flash/general, flash-first)
maßen Mariner-Asset-sha256 und die fünf Vlies-Quellen. **Adressierte Blöcke
gefaltet:** mountain-321 (phi-`url`-Zeilen + themis-Compiler sind committet; der
`mariner_rst`-Origin-Riss trägt jetzt die M6/M7-Tars, der M4-Tar steht im eigenen
`mariner4_odg`-Block) und future-240 (NIST NCNR NSE ist registriert + Workflow
`nist-ncnr-nse-cdn.yml` gebaut).

## Burn: open 0.0 · close 0.09 — Grund: Meta-Pass in einem Atom (flash-first; drei Diver `grind-flash`/`general`, kein pro/max). (close = gemessener `session_burn`-Linien-Posten; die Burn-DB wuchs in diesem Zeitraum durch parallele Linien-Sessions — kein sauberer Isolationswert.)

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „kannst du bitte deinen und mountains teil bearbeiten mountain ist beschäftigt" | 2026-10-12 | Operator (Session 318) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „kannst du dich bitte um deinen teil kümmern? Klare Trennung — und ich habe meinen Teil gleich gebaut." | 2026-10-12 | Operator (Session 317) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „Starte die Mycelium-Linie in einem Pass … kein Planungstheater … /commit (Push) … danach den frischen Pass" | 2026-10-11 | Operator (Session 315/316) · `state/operator-gespraeche/2026-10-11-mycelium.md` |
| „auf jeden eigen setzt du jetzt einen taucher an und der schliesst die aufgabe ab — das ist ein befehl" | 2026-10-11 | Operator (Session 310) |

## Offen — eigen

### GRDC-Caravan — Zeitachse geheilt; Lauf zur Verifikation offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `grdc-caravan-cdn`-Lauf nach dem Push endet
- **Lage:** (gemessen 2026-10-12) Lauf `38176538328` failure: Log `no .nc file carried a finite streamflow sample — the bin stays unwritten (0 honored; 5356 skipped)`, je Datei `time axis absent`. Ursache am Repräsentanten `.nc` (`GRDC_Caravan_extension_nc/timeseries/netcdf/grdc/GRDC_1159100.nc`, Zip64 aus `zenodo.org/api/records/15349031`): netCDF-4/HDF5, die Zeitachse heißt **`date`**, nicht `time`; `units = days since 1950-01-02 00:00:00` parst unter `parse_units`. `TIME_VAR` `time`→`date` gesetzt (`cargo check` 0/0).
- **Blockade:** keine im Code.
- **Braucht:** nach dem Push `gh workflow run grdc-caravan-cdn.yml` → `ci_manage log <id>`; erwartet: finite streamflow-Samples → `grdc_caravan.bin` geschrieben.

### Mariner-CDN-Arme — Läufe grün; Asset-sha256 gemessen, Register-sha offen (phi gehalten)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `phi/sources.φ` frei (Mountain-`terms`-Hunk committet)
- **Lage:** (gemessen 2026-10-12, aus den Logs) `mariner4-odg-cdn 38209368944`, `mariner-rst-cdn 38209371259`, `mariner-m10-edr-cdn 38209373661` alle **success**. Asset-sha256: `mariner4_odg.bin` `77e1a1c6bea0e13d26b30b807d3607b5c9704a1159f35aa47d8f46575149031c` · `mariner_rst.bin` `fdde35e23c4e4f615147bea7aff70df67fccc11ec6247b4af8d60ba24b2853b5` · `mariner_m10_edr.bin` `1773a9770d6c3e9fec223c5e97bbe4ec3e7ecf93e48b6d777d75714855c38a47` (1131956 B). Register-Blöcke `phi:31385-31402` tragen url/format/origin/compiler; M4 hat `terms CC0-1.0` (Mountain); M10/RST-`terms` offen.
- **Blockade:** `phi/sources.φ` wird gerade von Mountain gehalten (uncommitteter `terms`-Hunk am M4-Block) — eine `sha256`-Zeile jetzt würde beim Commit fremden Hunk mitschieben.
- **Braucht:** die drei `sha256 <hex>`-Zeilen in `phi:31389`/`:31395`/`:31402` (Mycelium) + `terms` (Mountain) — sobald der Baum ruhig ist.

### `intermagnet_dbdt`-Resolver — gebaut; Zelle beim Lauf
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `field-te-query`-Lauf `38215224525` endet
- **Lage:** (gemessen 2026-10-12) Rat-Verdikt **Nein** zum Rename (`state/stimmen/2026-10-12_river-intermagnet-identitaet-rat.md`): `intermagnet_dbdt` bleibt die Identität, die Kadenz ist Auflösungs-Attribut. **Gebaut** in `tools/measure/src/bin/field_te_query.rs` `load_field_across_sources` (per-Station-Scoping, feinster Zeuge, gröberer Riss getragen statt `Err`; commit `3467ec4f9`; `cargo build` 0/0). Lauf `38215224525` dispatched, pending.
- **Blockade:** keine im Code.
- **Braucht:** `ci_manage log 38215224525`; die Zelle neu lesen (ABK `abk_1m` Zeuge, `abk_1h` getragener Riss).

### Z-Minuten-Null (`bz_dbdt_delay_probe`) — Workflow gebaut; Läufe offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `bz-dbdt-delay-probe`-Läufe (ABK/SOD 2024, `--n-surr 200`) enden
- **Lage:** (gemessen 2026-10-12) `.github/workflows/bz-dbdt-delay-probe.yml` gebaut (`f664c1b96`; Spiegel `bz-retro-probe.yml`, 2 Matrix-Punkte ABK/SOD 2024, `--n-surr 200`, Artefakt je Punkt); Lauf `38215420000` dispatched, queued.
- **Blockade:** keine im Workflow.
- **Braucht:** `ci_manage log 38215420000`; das studentisierte Lag-Maximum + den kalibrierten Kritik-Wert aus dem Artefakt lesen.

### 50 Vlies-`alignment pending` — fünf Quellen vermessen; Compiler/Workflow je Reihe offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** je Reihe ein Compiler + Manifestations-Workflow bzw. Mountain-`terms`/`field`
- **Lage:** (gemessen 2026-10-12, `general`-Diver) fünf erreichbare Quellen: **qbo_30hpa_ms** → FU Berlin `https://www.geo.fu-berlin.de/met/ag/strat/produkte/qbo/qbo.dat` (text, monatlich) · **d20_thermocline_depth_m** → NOAA/PMEL ERDDAP `https://data.pmel.noaa.gov/pmel/erddap/tabledap/pmelTaoDyIso` (CSV/NetCDF, täglich, 1977–) · **swarm_absolute_vertical_tec_tecu** → ESA Swarm L2 `SW_TECxTMS_2F` (`https://swarm-diss.eo.esa.int/#swarm/Level2daily/Latest_baselines/TEC/TMS`, CDF, 1 Hz, ESA-Policy) · **eve_131_line_irradiance** + **eve_1032_line_irradiance** → LASP SDO EVE L3 `https://lasp.colorado.edu/eve/data_access/eve_data/products/level3/<year>/EVE_L3_<YYYYDDD>_008_01.fit` (FITS, täglich, NASA/LASP; 1032 Å Zeilen-Zuordnung vor Registrierung am L3-README prüfen). Die drei HAPI-Arme (`magnetosphere_imf_bt_nt`/`solar_wind_speed_km_s`/density) sind durch Rivers `field_te_query`-Fix adressiert.
- **Blockade:** je Reihe ein neuer Compiler + Workflow (`url`/`origin` Mycelium, `terms`/`field` Mountain).
- **Braucht:** je Reihe Compiler (`tools/harvest/src/bin/`) + CDN-Workflow, ausgehend von den gemessenen URLs.

### GIC-Onset-Harvest — `intermagnet-xyz-cdn` hängt in `queued`
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf `38164966250` (`intermagnet-xyz-cdn`) endet
- **Lage:** (gemessen 2026-10-12 via `ci_manage view`) weiterhin `queued` seit 2026-10-11T18:42Z (`updated_at` 03:22Z); der Vorlauf `38164890620` trug 25 Stationen, `lov` HTTP 400 für 2023+. Der Verzug ist eine Runner-/Concurrency-Warteschlange, kein eigener Schritt (Watchdog-Aufsicht).
- **Blockade:** per-Station-`<code>_xyz_1m.bin`-Assets der fehlenden 154-Station-Familie.
- **Braucht:** Lauf-Ergebnis abwarten (Watchdog/`ci_manage status`); dann `al_network_proxy`/`gic_onset_detector` je Familie.

### THEMIS-ASI getimetes Keogramm — Registerform + Compiler-MIRACLE-Form offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Registerform (Per-Nacht vs. feste `url`) entschieden (Mountain/Rat)
- **Lage:** (gemessen 2026-10-12) `tools/harvest/src/bin/themis_asi_timed_keogram_compiler.rs:320-329` `emit()` druckt `at earth` (nicht die MIRACLE-Form `on earth <lat> <lon> <alt>` + `station <CODE>`). `alt` = `h = H + N` definitiv (EGM2008, 24/24 in `state/mycelium/themis_asi_stations.md`); kein Default/Pad. Stationstabelle liegt vor.
- **Blockade:** Registerform unentschieden; ohne sie kein `emit()`-Umbau.
- **Braucht:** Form-Entscheidung → `emit()` auf die MIRACLE-Form (Mycelium) + `terms` (UCalgary) + `field` (Mountain); WHIT `pending`.

### Rätsel-/NSE-Routen — `url`/`origin` geschrieben; Produzenten offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ersten Manifest-Lauf je Quelle
- **Lage:** (gemessen 2026-10-12) UNCOVER/JADES/BK18/NAIF-Voyager-`url`/`origin` registriert; UNCOVER/JADES/BK18 ohne Compiler/Workflow. **NIST NCNR NSE** ist registriert (`phi:11721-11723`) + Workflow `nist-ncnr-nse-cdn.yml` + `nist_ncnr_nse_compiler.rs` gebaut (future-240-Ask erfüllt).
- **Blockade:** Manifestations-Workflows für UNCOVER/JADES/BK18 fehlen; `terms`/`format` (Mountain).
- **Braucht:** Workflows für UNCOVER/JADES/BK18 (Mycelium) + `terms`/`format` (Mountain).

### ENTSO-E A78 — Lauf rot; Body-Diagnose gebaut
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `entsoe-cdn`-Lauf endet
- **Lage:** (gemessen 2026-10-11 via `ci_manage log 38169562211`) failure `curl status Some(22)`; geheilt (`-sSL` + `--write-out`).
- **Blockade:** ENTSO-E-Token (Ticket #9473923, `state/zustand/wartend.φ:28`).
- **Braucht:** `ci_manage log <run-id>`; bei grün Register-Block; `terms`/`ttl` (Mountain).

### MLA-Register · SPT-3G-D1 · GloFAS-EWDS · NIST-NCNR-NSE
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** je Lauf/Digest bzw. Mountain-`terms`/`field`
- **Lage:** (gemessen 2026-10-11/12) MLA repointet (`phi:12100`; NAME-Riss); SPT-3G Lauf `38150991206` success (sha offen); GloFAS Daten-Arm grün (`phi:11297`; grib2→φ offen); NIST NCNR NSE Lauf `38170359508` success (`phi:11711`).
- **Blockade:** Lauf-Ergebnisse/`terms`/`field` (Mountain).
- **Braucht:** `ci_manage log <id>`; sha256; `terms`/`field` (Mountain).

### wy-max-t-Artefakt — Werte im Bestand
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Operator-Siegel auf `docs/blatt/fruehwarnsystem-praeregistrierung.md`
- **Lage:** (gemessen 2026-10-12, River-194) α=0.05, B=9999, (1-α)-Quantil 2.4467/2.4433/2.4831 (Artefakt `38166787414`); **BCa-CI `pending`**.
- **Blockade:** BCa-CI (River) + Siegel (Operator).
- **Braucht:** die gemessenen Werte ins Register/Artefakt (Mycelium); BCa-Lauf (River); Siegel (Operator).

### MCP no-leak · de441-de442-sha · dropped-gate · SWPC/RTSW · CI-Optimierung · Domain→UCD
- **Status:** wartend/eigen/pending | **Bindung:** eigen
- **Trigger:** MCP-Session-Restart · Merge-Lauf (`phi:4321-4369`) · Q2/Q3-Fold (`dropped_gate.rs`) · `matrix-vlies`-Lauf · `odf.rs`-clippy · messbare Domain-Quelle
- **Lage:** (gemessen 2026-10-11/12) MCP: Allowlist konsistent, Live-Seat-Restart offen; de441/de442: sha256 erst nach Merge-Lauf; dropped-gate: Match-Fix `offer_word` + Test (312), Q2/Q3-Fold + 924-vs-78-Riss (future-240 `278ba2905` vs Baseline 78) offen; SWPC/RTSW: `matrix-vlies` am ACE-Arme offen; CI-Opt: `odf.rs` clippy (`:179`/`:213`/`:2222`, Träger Mountain, uncommitteter Fix im Baum); Domain→UCD: keine messbare Domain-Quelle.
- **Braucht:** je der genannte Schritt.

### bpa_gic — keine Historie (gemessene Null)
- **Status:** descoped | **Bindung:** eigen
- **Befund:** (gemessen 2026-10-12) BPA publiziert kein historisches GIC-Archiv (nur 4-Tage-5-min-Fenster `gic.aspx`/`gic.txt`; alle Archiv-Kandidaten 404; `--all` 1029 Zeilen; Wayback 429/503). Die zweite GIC-Zeugenreihe bleibt leer; die Relation läuft über die primäre FMI-Reihe. `terms unknown`.

### matrix-rotor — descoped (gemessen)
- **Status:** descoped | **Bindung:** eigen
- **Befund:** (gemessen 2026-10-11/12) letzter rot = transienter Runner-Shutdown; kein eigener Schritt (`Braucht: —`).

## An mountain

Origin: mycelium-319 (2026-10-12).

- **`phi/sources.φ` gehalten:** uncommitteter `terms CC0-1.0`-Hunk am `mariner4_odg`-Block (`phi:31394`) — bitte committen, damit die Mariner-`sha256`-Zeilen ohne Hunk-Kollision geschrieben werden können.
- **MCON-Registerdrift gelöst** (`phi:21545-21551`, `format mcon_event`, sha `441df5f8…`); offen bleiben die fünf `field`-Zeilen (P10) + `terms` (Zenodo 22711679).
- **f107-Epoch-Fix (Archivar, 30-s TDB/Unix, zwei Defekte)** — Rivers `## An mountain`-Messung; Archivar-Kern = Mountain.
- **`odf.rs` clippy (`:179`/`:213`/`:2222`)** blockt `ci-check`; der Fix liegt uncommittet im Baum.
- **GRDC-Caravan:** die Zeitachse ist geheilt (Mycelium); kein Mountain-Anteil offen.

## An river

Origin: mycelium-319 (2026-10-12).

- **`bpa_gic`-Spanne** — gemessene Null: BPA führt nur ein 4-Tage/5-min-Fenster, kein Archiv; die zweite GIC-Zeugenreihe bleibt leer; die Relation läuft über die FMI-Reihe.
- **`solar_f107_flux_sfu`** corr 0.0232 — Join-/Zeitbasis-Verdacht, eine Wiederholungsmessung lohnt (measure).
- **MCON-Registerzeile steht** (`format mcon_event`); die Zelle/Relation kann neu gelesen werden.

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
