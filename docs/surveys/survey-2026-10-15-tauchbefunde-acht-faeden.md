<!--
  title: Survey — Tauchbefunde der acht Fäden (2026-10-15)
  class: survey
  date: 2026-10-15
  sha256: 72071e0b4910ce517f76c0b8ed797eaf9ea9d3331df2adb4e4591781bfd59395
  status: live
  see-also: docs/surveys/survey-2026-10-15-raetsel-matrix-korrektur.md docs/handover/archiv/handover-2026-10-15-mycelium-folge359.md
-->
# Survey — Tauchbefunde der acht Fäden (2026-10-15)

Operator-Wort 2026-10-15: die nächste Session lässt **je einen `taucher`** auf jeden der acht
Fäden los — **harte Bandagen**, **ein Dispatch = ein Faden + ein begrenzter Schritt**. Die
Korrektur (`survey-2026-10-15-raetsel-matrix-korrektur.md`) steht: für sechs der acht Fäden ist
der **Quellen-Block schon da** — gesucht ist der **fehlende Kanal**; für WGLC/SILSO/EDGES/TGF/
Wolken die **fehlende Quelle**. Grenze: `terms`/`ttl`/`field` = Mountain; der Taucher liefert
Messung + `url`/`origin`/`compiler`/`sha256` + CDN-Naht.

Acht Taucher liefen in einem Atom; die Schlüssel-Belege wurden an der Session gegen den Baum
nachgemessen (`sgrep`/`sread`/`git log`). Alle Entwürfe sind **Vorschläge** — nichts ist ins
Register geschrieben. Der Baum gewinnt.

Register-Nullstände (am HEAD dieser Session nachgemessen):
`sgrep -i -c 'hmi' phi/sources.φ` = 0 · `'silso'` = 0 · `'sunspot'` = 0 · `'neutrino'` = 0 ·
`'lensing'` = 0 · `'eve_'` = 0 · `'format eve'` = 0 · `'hips3d'` = 2 (nur der WALLABY-Block) ·
`'tar_zst'` = 1 (nur der Conicus-Block). `src/archivar/fetch.rs`: `tar_zst` = 0; `src/`: `hips3d`
= 0 → beide Formate **ohne Reader**.

---

## Faden 1 — HMI (Photosphären-Magnetogramm), Kanal-Lücke über JSOC

**Quelle steht.** Der AIA-Block ab `phi:2081`: `terms CC0-1.0 http://jsoc.stanford.edu/JSOC_Data_Policy.html`,
`origin http://jsoc.stanford.edu/cgi-bin/ajax/jsoc_fetch`, `compiler aia_compiler.rs`, `at sun`,
`ttl 86400`. Es fehlt der **`hmi`-Kanal** (0 `hmi`-Zeilen im Register).

**Gemessene Route** (`archive_search --verdict`, `jsoc_info` HTTP 200):
- `hmi.M_720s` — 720-s-Takt (`T_REC step 720.0`, CADENCE 720), Segment FITS 4096×4096, `units Gauss`,
  Zeitraum `2009.04.13_21:48` … `2026.10.11_23:12`; `primekeys T_REC+CAMERA` ⇒ **3 Datensätze je
  Zeitpunkt** (1 side/2 front/3 both) → Dedup-Pflicht. Wertprobe 2026-10-01: `DATAMEAN` 0,196–0,217 G.
- `hmi.sharp_720s` (SHARP-Parameter, HARP-Key) und `hmi.sharp_cea_720s` (**Vektorfeld** `Bp`/`Bt`/`Br`
  + `_err` + `conf_disambig`) je HTTP 200.
- Exportform: `jsoc_fetch?ds=hmi.M_720s[<t>/<span>@720s]&op=exp_request&protocol=fits&method=url_quick`
  → HTTP 200 (`{"status":7, "error":"… 1 pending export requests …"}`); Dateien unter
  `https://jsoc1.stanford.edu<dir>/<file>`. Der Export ist **angestoßen** (`JSOC_20261015_004396`),
  nicht weiter verfolgt (kein Polling).
- **Lizenz:** die JSOC-Data-Policy nennt CC0-1.0 ausdrücklich für `sdo.X`, **`hmi.X`** und `aia.X` —
  die AIA-`terms`-Zeile gilt für HMI 1:1.

**Befund:** ein `hmi_compiler.rs` ist **analog `aia_compiler.rs`** baubar (dieselben drei JSOC-Basen,
`rs_list`/`exp_request`); Unterschied: Serie `hmi.M_720s`, Wert `DATAMEAN` direkt (**Gauss**),
Raster 720 s, Kanal = CAMERA 1/2/3 (dedupen).

**Riss A (Maschinerie ohne Register).** Es gibt `tools/harvest/src/bin/hmi_polar_compiler.rs`
(Serie `hmi.synoptic_mr_polfil_720s`), `src/archivar/hmi_polar.rs` (MAGIC `HMP1`),
`hmi_cycle_probe.rs`, `wso_hmi_consistency.rs`, `.github/workflows/hmi-polar-cdn.yml` — **aber
`sgrep -i 'hmi' phi/sources.φ` = 0** (kein Block). Der Compiler lädt nach Tag
`jsoc1.stanford.edu` (`hmi_polar_compiler.rs:393`) — dieser Tag ist **nicht registriert**
(`sgrep 'jsoc1' phi/sources.φ` = 0) und auf GitHub **404**; die Workflow-Naht zieht dagegen von
`ssd.jpl.nasa.gov`, wo `hmi_polar.bin` liegt (`--verdict` HTTP **206**). **Compiler-Ziel ≠ Asset-Ort.**

**Riss B (Format).** `format hmi` existiert nicht (die Grammatik kennt nur `aia`,
`src/archivar/extract.rs:125`/`:633`): neuer Parser-Arm `hmi` **oder** `aia`-Wiederverwendung mit
reserviertem Kanal — ein Kontrakt-Entscheid.

**Block-Entwurf (Vorschlag; `field`/`ttl` = Mountain):**
```
url https://github.com/omegaflow/sources/releases/download/jsoc.stanford.edu/hmi_m_720s_<jahr>.bin
terms CC0-1.0 http://jsoc.stanford.edu/JSOC_Data_Policy.html
format hmi
origin http://jsoc.stanford.edu/cgi-bin/ajax/jsoc_fetch
compiler tools/harvest/src/bin/hmi_compiler.rs
at sun
ttl 86400
field hmi_m_720s_blos hmi_m_720s_blos inverse-square em G 86400 0.0 0.0
```
CDN-Tag `jsoc.stanford.edu` ist schon registriert (AIA) — **kein** neuer Tag.
Mycelium: `url`/`origin`/`compiler` + Naht. Mountain: `field`-Admission, Einheit (G/T), `ttl`,
Format-Arm, der `hmi_polar`-Tag-Riss.

---

## Faden 2 — SILSO (Sonnenfleckenzahl), **keine Lücke — ein Verwurf** (Riss)

**Register:** `silso`/`sunspot`/`ssn` = 0; F10.7 steht (`phi:433` swpc, `phi:12896` f107_penticton).

**Gemessene Quelle:** `https://www.sidc.be/SILSO/DATA/SN_d_tot_V2.0.csv` — HTTP 200,
**2 897 272 B**, sha256 `4c452439dc1f9d52e3278b379bcd5be542e595fe36b26123c004da4720c36fc1`
(unabhängig über `manifest-paired.sha256` bestätigt). `;`-getrennt, 8 Spalten, **kein Header**;
1818-01-01 … 2026-09-30, **76 244** Zeilen. Missing = `-1`; **`0` ist ein echter Wert** (nicht
als absent zu werfen). Nord/Süd separat: `SN_d_hem_V2.0.csv` (850 364 B, sha
`896f4572…`). Lizenz: **CC-BY-NC-4.0** (`sidc.be/SILSO/legal`; Haus-Token im Census).

**Riss (nicht geglättet).** Der Baum trägt einen **stehenden Verwurf**, die Korrektur-Survey nennt
SILSO dagegen eine „echte Quellen-Lücke":
- `git show e988e464d` (mountain-334, 2026-10-12): drei `decline unphysical https://www.sidc.be/SILSO/…`
  Zeilen. `235e42604` hat **alle 1821 decline-Zeilen aus phi entfernt** — „git carries the verdicts".
  Aktuell `sgrep -c 'decline ' phi/sources.φ` = **0**.
- `docs/specs/sunspots.md`: *„The sunspot number (SSN) is a human invention … an index, not a physical
  measurement"* — SSN wird verweigert; F10.7 / GOES-X-Ray / Magnetfeld sind die zugelassenen Wahrheiten.
- `src/archivar/port.rs:2048`: `kl.contains("_sunspot_number")` steht in der **Refuse-Liste**.
- Gegenlinie: `state/mountain/faden-term-field-ttl-draft.φ` [1c] (2026-10-14) schlägt den Block vor.

**Zuständigkeit:** `terms` (`CC-BY-NC-4.0`) wäre Mountains Zeile **nur bei Zulassung**; die Zulassung
selbst ist der stehende `unphysical`-Verwurf. **Kein Block ohne diesen Entscheid** (Mountain/Rat).
Kein Compiler, kein Workflow vorhanden (`sgrep -i -c 'silso' tools/harvest/src/bin/` = 0).

---

## Faden 3 — Stellar-Aktivität (XUV, log R'HK), Kanal-Lücke über RAVE/GALAH

**Quellen stehen, tragen aber keinen Aktivitätsindex.**
- **RAVE** (`phi:9815` gavo, `phi:9920` dc.g-vo, `phi:11532` DR5, `phi:31599` DR6) und **GALAH**
  (`phi:9898` datacentral) sind registriert; `xuv`/`rhk`/`ca_hk` = 0.
- **Gemessen (VizieR TAP_SCHEMA):** `III/283/ravedr6` (42 Spalten) und GALAH DR3/DR4 tragen
  **keine** `logRHK`/`CaII`/`Ha`/`IRT`-Spalte — `[Ca/H]`/`Ca_fe` ist die **Häufigkeit**, nicht die
  Aktivität. Aktivität ist für RAVE/GALAH **nicht in-situ**.
- **Die echten freien Aktivitätsquellen:** VizieR `J/A+A/697/A230/catalog` (Freund et al. 2025,
  Spalte **`logR'IRT`**, `vizier-scientific-free`); Zenodo **18834075** LAMOST DR11
  (`S_L, S_MWO, R_HK, R′_HK`, **CC-BY-4.0**), Zenodo **10841693** DR8.
- **XUV:** SDO EVE L3 merged SSI (LISIRD `sdo_eve_ssi_1nm_l3`, 6–106 nm, **CC0-1.0**,
  `…/eve/data_access/eve_data/products/merged/latest_EVE_L3_merged_1nm.fit` HTTP 206); EVE L2 Stundenraster.

**Riss A (EVE ist im Code gebaut, im Register nicht).** `src/archivar/eve.rs` (MAGIC `EVL1`, 11 Linien),
`extract.rs:126` (`"eve" => eve::parse_bin`), `main_flow.rs:3215`, `tools/harvest/src/bin/eve_compiler.rs`
(CDN-Tag `lasp.colorado.edu-eve`) — aber `format eve` = 0, kein `eve-cdn.yml`, Tag 404. Der Kanal ist
verdrahtet, aber register- und CDN-los.
**Riss B (GALAH-Block kaputt).** Der Register-Block `phi:9937–9944` (Survey-Kürzel `phi:9898` ist
offset-stale) trägt in den `cmap`-Zeilen `ra_dr2`/`dec_dr2`; das Datacentral-TAP kennt nur `ra`/`dec`
→ HTTP **400**. Die registrierte **URL** (`:9937`) fragt bereits `ra,dec` — der Fehler sitzt in der
`cmap` (`:9943-9944`), nicht in der Abfrage. (Gegenprobe + Fix 2026-10-16, Sensory.)

**Kanal-Entwürfe (Vorschlag):** EVE 11 Zeilen `field eve_<N>_line_irradiance … inverse-square em wm2_1au 3600`;
`field logR'IRT freund_logr_irt gaussian-inverse-square em 1 604800`; `field logRHK lamost_dr11_logrhk …`.
Die Shape/Force-Klasse eines dimensionslosen Index ist ein Kontrakt-Entscheid (Rat/Mountain).
Mycelium: `eve-cdn.yml` (nach Tag-Zulassung); Mountain: EVE-Block + Felder, `phi:9898`-Reparatur,
Aktivitäts-Zulassungen.

---

## Faden 4 — Schwache Linsen / Shear, Kanal-Lücke über KiDS WL

**Blöcke stehen, strahlen aber nichts.**
- `phi:31167` KiDS DR4 bright (Fluss, kein Shear), `phi:31178` KiDS DR4.1 WL-Katalog,
  `phi:30583` Conicus3D WL-Lightcone; `lensing`/`shear` = 0.
- **KiDS WL gemessen:** Zenodo `16366035`, **cc-by-4.0**, `KiDS_DR4.1_ugriZYJHKs_SOM_gold_WL_cat.fits`
  **17 712 469 440 B = 16,50 GiB**, md5 `3e764558…`; Spalten aus dem FITS-Header: `RAJ2000`,
  `DECJ2000`, **`e1`/`e2`** (Lensfit-Elliptizität), `weight`, `Z_B`; Subset `kids_subset_safe.fits`
  8 MiB. Offizielle Direkt-URL `kids.strw.leidenuniv.nl/DR4/data_files/…` HTTP 206.
- **Conicus3D:** Zenodo `18776222`, cc-by-4.0, `…phase_metadata.tar.zst` **34 407 793 603 B = 32,04 GiB**.

**Riss (Transport).** Alle drei registrierten CDN-`url` liefern **404** (kein Matrix-Eintrag in
`.github/workflows`, keine `sha256`; `raw_mirror` verweigert ohne sha256). `format tar_zst` hat
**keinen Reader** (`fetch.rs format_readable` kennt ihn nicht). `phi:31178` fehlt `cmap`/`ra`/`dec`
**und** ein Distance-Key → per Test (`tests.rs:2563`) strahlt die Zeile **keinen Shear-Sample**.

**Kanal-Entwurf (Vorschlag; zwei Physik-Risse = Rat/Mountain):**
```
cmap .
ra RAJ2000
dec DECJ2000
z_key Z_B
field e1 kids_dr4_wl_e1 <kernel> <force> 1 31536000 0.0 0.0
field e2 kids_dr4_wl_e2 <kernel> <force> 1 31536000 0.0 0.0
```
**Gesetzt 2026-10-18 (Mountain):** `phi:31397–31402` trägt jetzt `cmap .` · `ra RAJ2000` ·
`dec DECJ2000` · `z_key Z_B` · `field e1 kids_dr4_wl_e1 inverse-square gravity 1 …` ·
`field e2 …`. Der FITS-Subset-Leser steht (`src/archivar/fits_range.rs`, `17279769e`); die
CDN-Naht trägt `sha256 dcc2bf03…`. Offen bleibt nur der 16,5-GiB-Vollzug im Content-Cache —
kein fehlender field-Arm mehr.
Offen: nichts am Keyword — entschieden 2026-10-18 (River, `docs/blatt/blatt-2026-10-18-force-keyword.md`):
`force gravity` · `kernel inverse-square` · Einheit `1` (nie `scalar`). Grund: e1/e2 sind die
transversale zweite Ableitung des Linsenpotentials (Poisson `∇²ψ = 2κ`); `inverse-linear` wäre das
Ablenkungsgesetz (γ ∝ θ⁻¹, SIS), nicht das Feldgesetz; `em` wäre die Strahlungsgröße, nicht die Form.
Gegenstimme benannt (Fugu: „geometrische Verzerrung, keine Kraft"). Mycelium: sha256 + CDN-Matrix-Einträge
+ `tar_zst`-Arm. `format fits` braucht **keinen** neuen Compiler (eingebauter Arm), aber der
16,5-GiB-Vollzug im Content-Cache ist ein Blocker (kein Range-/Spaltensubset-Leser).

---

## Faden 5 — Neutrino / IceCube / Auger / LHAASO, Kanal-Lücke

**`neutrino` = 0 ist ein Wort-Null, kein Kanal-Null.** Der Neutrino-Kanal wird per
`record`/`witness` (S2E1/SKY1/PAO1) + `force em` deklariert; bestehende Felder heißen
`icecat_sky1` (`phi:12894`), `antares_events_2007_2017` (`phi:12928`). Kein `neutrino`-Force
(`channel.rs`: 9 Kräfte, `force_id_of("neutrino")` = None) — die Doktrin (`die-weberin.md:103`):
das Teilchen ist **Abstammung, nicht Kraft**.

**Gemessene Belege:**
- **IceCube IceTracks DR2:** Dataverse DOI `10.7910/DVN/MMIIZA`, **CC0-1.0**, 41 Dateien;
  `events`-Spalten `run event subevent MJD log10(E/GeV) AngErr RA Dec Azimuth Zenith`. Der Block
  `phi:14659` trägt `format json` (widerspricht `.bin`), `origin` = **Such-API** (nicht die Datei),
  kein `field`/`compiler`/`at`.
- **Auger:** `opendata.auger.org/catalog/data.zip` (3 467 397 B, sha `befceb14…`), **CC-BY-SA-4.0**;
  Ausgabe PAO1 (13-B-Header + 28-B-Records). `phi:31446` = `witness s2-direction` + `record pao1`.
- **LHAASO:** VizieR `J/ApJS/271/25`, `vizier-scientific-free`; die zwei γ-Felder sind korrekt
  (`lhaaso_km2a`, `lhaaso_sky1`). Ein `field … neutrino …` wäre **Fabrikation**.
- **CDN:** LHAASO present (206); Auger/IceCube/IceCat/ANTARES **404**.

**Harter Code-Riss (Kreuz-Riss).** Die vier Record-Compiler `auger_compiler.rs:17-39`,
`icecat_compiler.rs:194-217`, `antares_vo_compiler.rs` und `amon_compiler.rs` prüfen ihr MAGIC
gegen `WitnessKind::S2Direction`; `src/archivar/witness.rs:20-23` bildet aber
`AMN1|PAO1|S2E1|ERBQ|MCON → PointEvent` (nur `SKY1|SKD1|VLDE → S2Direction`). **Die vier Compiler
brechen in ihrer ersten Zeile ab** („… reads …, not s2-direction — the asset stays unwritten") —
das erklärt die 404 zwanglos. Zwei Zeilen, nie nachgezogen. Die Auflösung berührt `witness.rs`
(Archivar/Mountain) **und** die Compiler (Mycelium) → **ein Kreuz-Riss**, kein stiller Schreibakt.

**Entwurf:** IceCube `field icecube_icetracks_dr2 icecube_icetracks_dr2 inverse-square em 1 31536000`;
Auger analog; `at earth` ergänzen; `format json` korrigieren. **Unit-Riss:** Auger-Energie in **EeV**
fehlt in `units.rs` (`allowed_units_for_force(0)` und `-quantity(energy)`). Mycelium: Compiler-Gate,
Spiegel, sha256. Mountain: `field`/`quantity`-Admission, Witness-Identität, `eev`-Unit.

**Gesetzt 2026-10-18 (Mountain):** Auger `phi:19075` und IceCube `phi:14733` tragen
`field … inverse-square em 1 31536000 0.0 0.0`; beide Blöcke führen `at earth`. Der
Witness-Kreuz-Riss ist am HEAD bereits geheilt (`src/archivar/witness.rs:20-23`:
`AMN1|PAO1|S2E1 → S2Direction`), die vier Record-Compiler laufen also nicht mehr in den
Bruch. Offen: IceCube-`format json` vs. `.bin`, fehlender IceCube-Compiler, CDN-404 und
die `eev`-Unit (Mycelium/Archivar).

---

## Faden 6 — 21 cm / EDGES, **die Lücke liegt in phi, nicht in der Welt** (Riss)

**Quellen öffentlich erreichbar.**
- **EDGES DR1** (reduziert): `https://loco.lab.asu.edu/download/790/` → `figure1_plotdata.csv`
  (HTTP 200, 129 Zeilen; Kopf `Frequency [MHz], Weight, a: Tsky [K], b: Tres1 [K], c: Tres2 [K],
  d: Tmodel [K], e: T21 [K]`; 50,012–100 MHz, Δ 0,390625 MHz).
- **EDGES Kalibration:** Zenodo `16881868` → Record `18091240`, **cc-by-4.0** (`Spectra.7z` 2 842 929 960 B …).
- **EDGES Roh:** `galileo.sese.asu.edu/edges-data/` (~150 `.gsh5`, je 1,2–3,6 GB) — **> 2 GiB ⇒ Shard**.
- **REACH:** `pending` — Instrument/Kalibrationscode öffentlich, **kein** Datenprodukt (403 anonym).

**Riss (nicht geglättet).** Die Handover nennt EDGES eine „echte Quellen-Lücke"; die Weltmessung
zeigt das Gegenteil. Der Vorgänger `xfer:mycelium-344-2` hatte Zenodo `16881868` am 2026-10-13
**schon gemessen**; der Entwurf `state/mycelium/block-drafts/2026-10-13-faden-schliessung.φ` ist
**nicht mehr im Baum** — der Befund wurde nie registriert. **Zweiter Riss:** WALLABY (`phi:14519` = offset-stale, real heute
`phi:14569–14574`, ODbL-1.0, `format hips3d`) hat keine `field`-Zeile **und** `format hips3d` hat
**keinen Reader** (`src/` = 0). Zusatz: der Block registriert das CDS-**HiPS-Produkt** (`ODbL-1.0`),
nicht den Survey; die Daten stehen bei CASDA/CSIRO unter **CC-BY-4.0** (`10.25919/hg66-4v60`).

**Kanal-Entwurf / Compiler.** `spectral_compiler.rs` ist **λ-nativ**, EDGES **frequenz-nativ** in K →
ein eigener `edges_compiler.rs` auf demselben Schreibpfad (`write_spectral_bin`). Entwurf:
`field brightness_temperature edges_lowband_t21_k inverse-square em K 2628000`, `at earth`.
Mountain: `terms` (keine SPDX-Lizenz, Zitatpflicht Bowman 2018 + Murray 2026), `ttl`/`no-cadence`,
Kernel. Mycelium: `url`/`origin`/`compiler`/`sha256` + Naht.

---

## Faden 7 — PTA / Pulsar-Timing, Kanal-Lücke über EPTA DR2

**Block `phi:30615`** trägt nur `url`/`terms CC-BY-4.0`/`format zip`/`origin`/`no-cadence` — **kein**
`compiler`/`field`/`at`/`sha256`. `sgrep -i residuum phi/sources.φ` = 0.

**Gemessen:** EPTA-DR2.zip **39 275 794 B**, md5 `307b2b99…`, sha256 `2ae54694…`; das Zip enthält
**Roh-ToAs + `.par`-Modelle, keine Residuum-Tabelle** (`TRES` in µs steht im `.par`; Residuum
entsteht erst per TEMPO2-Fit). Ergänzend: Zenodo `8025019` (white-noise-Residuen, **nur PDFs**),
`8091568` (MCMC-Ketten, 649 MB). Zweitquellen IPTA DR2 (keine Lizenz gemessen) / NANOGrav 15 yr
(Zenodo `7967584`, cc-by-4.0). CDN: `…/zenodo.org/epta_dr2.zip` = **404**.

**Compiler-Weg:** kein tempo2-Parser im Baum (`harvest`+`measure`: kein `pulsar|pta|timing`)
→ neuer `epta_dr2_compiler.rs` (Vorbild `llr_compiler.rs`), liest pro Pulsar `RAJ`/`DECJ`/`TRES`.

**Kanal-Entwurf:** `at sun` + `cmap`/`ra RAJ`/`dec DECJ` +
`field TRES pulsar_timing_residual_s inverse-square em s 31536000 0.0 0.0`
(`us` verweigert das Gate; `TRES`-µs → **s** rechnen). Frame `at sun` (Quelle) vs. `at earth`
(Empfänger) = Mountain. Mountain: `field`/`force em`/`unit s`/`kernel`/τ/Frame. Mycelium:
`url`/`origin`/`compiler` + Spiegel.

---

## Faden 8 — TGF / Wolkenbedeckung (WGLC ist Rat, nicht dieser Faden)

**TGF.** Lebende Quelle: FSSC `fermi.gsfc.nasa.gov/ssc/data/access/gbm/tgf/` (HTTP 200) — der
**zweite** Fermi-GBM-TGF-Katalog (Roberts et al. 2018), **4144 TGFs**, 2008-07-11 … 2016-07-31.
CSV-Tabellen (keyless): `gbm_tgf_catalog_offline.csv` (4135 Zeilen, sha256 `f75cf291…`),
`gbm_tgf_catalog_trig.csv` (686), `_teb.csv` (30), `_wwlln.csv` (1544, sha256 `c49faa9d…`).
Terms: **NASA PD** (`terms PD https://fermi.gsfc.nasa.gov/` ist Haus-Form, `phi:14696`).
**Riss:** der „3. TGF-Katalog" (Roberts, ~15 000 TGFs, Nov 2012–Nov 2023) existiert nur als
**Konferenz-Abstract** — **keine Datensatz-URL** → `pending`.

**Wolken.** Billigster Kanal: **Open-Meteo-Archive** (ERA5-basiert, **keyless**, **CC-BY-4.0**,
Compiler `open_meteo_archive_compiler.rs` ist **generisch** — liest beliebige `hourly`-Schlüssel).
Der Block `phi:1626` trägt heute nur `temperature_2m,pressure_msl` (`at earth`, bin 3600). Die
Variablen `cloud_cover(_low/_mid/_high)` in `%` sind über die `origin`-Query nachrüstbar.
Gated: MODIS `MOD08_D3` (LAADS, `EARTHDATA_EDL_TOKEN` **abwesend**), ERA5 via CDS (**401**).

**Entwürfe:** TGF-Block (`format gbm_tgf`, `on earth 0 0 0`, `map .`, `lat Lat`/`lon Lon`, Felder
`gbm_tgf_bgo0_counts`/`nai_counts`/`discovery_bin_ms`, `ttl 2592000`); Open-Meteo-`origin` um
`cloud_cover,…` erweitern + vier Felder (`gaussian-inverse-square diffusion % 3600`). Mycelium:
`url`/`origin`/`compiler`, CDN-Tags, Keys. Mountain: `field`/`ttl`/`terms`-Disposition, `field`
vs. `quantity` beim Ereigniskatalog.

**Gesetzt 2026-10-18 (Mountain):** Open-Meteo-`origin` (`phi:1656`) führt jetzt
`cloud_cover,cloud_cover_low,cloud_cover_mid,cloud_cover_high`; die vier Felder stehen
`phi:1663–1666` (`gaussian-inverse-square diffusion % 3600`). Der generische
`open_meteo_archive_compiler.rs` liest die Schlüssel. Offen: Asset-Neubau/CI und der
TGF-3-Katalog (nur Abstract).

---

## Querschnitts-Befunde

1. **Block ≠ Kanal** gilt durchgängig: für HMI, XUV/logR'HK, Shear, Neutrino, PTA steht die Quelle —
   der **Kanal** (die `field`-Zeile) fehlt; für SILSO, EDGES, TGF, Wolken fehlt die Quelle.
2. **Maschinerie ohne Register / CDN** (mehrfach gemessen): `hmi_polar` (kein Block, falscher Tag),
   EVE (Block + Workflow + Tag fehlen), icecat/antares/auger/amon (Compiler-Gate bricht),
   `format hips3d`/`tar_zst` (kein Reader).
3. **Grenze bleibt:** `terms`/`ttl`/`field`/Identität = Mountain/Rat; `url`/`origin`/`compiler`/
   `sha256`/Tag/Naht = Mycelium. Die Mountain-Zeile ist geschrieben: das Verdikt zu allen acht
   Fäden steht in `state/mountain/2026-10-15-acht-faeden-verdicts.φ` (2026-10-17); offene
   field-Risse sind dort benannt (u. a. WALLABY ohne `field`/Reader, EVE-`format` fehlt).

---

## Nachtrag 2026-10-18 — Restarme mit Spaltenkopf-Suche nachgemessen

Fünf Taucher, `archive_search --all` plus gezielte Arme, live HTTP. Belege:
`state/mountain/2026-10-18-tauchbefunde-restarme.φ`.

| Faden | Befund | Korrektur / Disposition |
|---|---|---|
| 1 HMI | `hmi.M_720s` exakt: `DATAMEAN` [G], 4096², 720 s, CC0 im FITS-Header | **1 Record/T_REC, nicht 3**; `CAMERA` wechselt (1→3), kein Dedup. Export braucht einmalige E-Mail-Registrierung. Arm: `hmi_compiler.rs`/`format hmi` |
| 3 EVE | exakt ist **L2b Lines**: `products/level2b/YYYY/DOY/EVL_L2B_…fit.gz`, `LINE_IRRADIANCE[71]`, 60 s | Compiler zeigte auf `level2` (endet 2018) → **Pfad in diesem Pass auf `level2b` + ein File/DOY geheilt**. Lizenz **nicht CC0**, sondern offen/ohne SPDX. XUV <33 nm nach MEGS-A-Ausfall 2014 nicht lieferbar |
| 7 EPTA | Zenodo `8300645`, 39 275 794 B, `sha256 2ae54694…`, CC-BY-4.0; `.par` trägt `TRES` 26/26 | **Kein TEMPO2 für Per-Pulsar-TRES nötig**; nur für per-TOA-Serie. `RAJ/DECJ` 23/26, `ELONG/ELAT` 3 → Fallback. `sha256` gesetzt |
| 6 EDGES | exakt: `figure1_plotdata.csv`, 7 907 B, Header `Frequency, Weight, Tsky, Tres1, Tres2, Tmodel, T21` | Nur DR1 als T21-Tabelle; Rohdaten DR3. Lizenz: kein SPDX, Zitat Bowman 2018. Arm: Block + `edges_compiler.rs` |
| 6 WALLABY | registrierter `hips3d`-Block = **Spectral-Cube**, keine Spalten; exakter Katalog = CASDA TAP `AS102.wallaby_pdr2_sourcecatalogue_v01`, 3 454 Zeilen, 43 Spalten, CC-BY-4.0, DOI `10.25919/qw7w-tn96` | Cube ≠ Katalog. Neuer TAP-Block/Compiler; Spalten `ra/dec/freq/w20/w50/log_m_hi` |
| 4 Conicus | Zenodo `18776222`, 34 407 793 603 B, CC-BY-4.0; `.npy` `(1,128,128)` δ-Overdensity aus **AbacusSummit** (`base_c000_ph000–024` + `c009_ph000`) | **`declined`:** kein Shear/κ; keine phi-Zeile, keine Rohdaten im Haus. Zenodo bleibt externe Referenz |
| 5 IceCube | Dataverse `10.7910/DVN/MMIIZA` v3.1, 14 TSV, 1 643 355 Events, CC0; Header `run event subevent MJD log10(E/GeV) AngErr RA Dec Azimuth Zenith` | URL 404/Tag fehlt; `format json` vs `.tab`; `record skydirection` falsch → S2E1; kein Compiler |
| 5 Auger | 109-Event-Katalog exakt; kompiliertes Asset `ssd.jpl.nasa.gov/auger_catalog.pao1` 200, 3 065 B, `sha256 03c85879…`, PAO1 n=109 | **URL in diesem Pass von `opendata.auger.org/…` (404) auf `ssd.jpl.nasa.gov/…` geheilt**, `sha256`/terms gesetzt. `eev` ist bereits in `units.rs:389` — alte Riss-Zeile stale |
| 8 TGF-3 | **kein Datensatz**; nur AGU-Abstracts | `pending`; Weg = direkte Anfrage GBM-TGF-Team. 2. Katalog live (5 CSVs) |
| 6 REACH | **kein Datenprodukt**; 403 = Cloudflare-WAF, `/data` = 404 | Release erst „upon detection"; nur Simulationen. `pending` |
