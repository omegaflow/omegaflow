<!--
  title: Sonnen-Abdeckung: lückenlose Vermessung pro Kraft
  class: survey
  date: 2026-08-21
  sha256: 581db43ea675066d2bfe856f1636fa37db407ad2daba8d5d04277b11dd148fea
  see-also: docs/handover/handover-2026-08-21-sonnen-pfad-solar-te.md docs/handover/handover-2026-08-21-ncei-ssi-hdf5.md
-->
# Sonnen-Abdeckung: lückenlose Vermessung pro Kraft (2026-08-21)

Kontur der Recherche: alle registrierten Solar-Kanäle (Gravity, em bolometrisch,
em Schmalband, Sonnenwind, electric, acoustic) gegen den Bestand (Kataloge,
Indizes, `phi/`, CDN, Live-APIs) geprüft — was deckt welches Zeitfenster ab,
wo sind die Lücken, welche Quellen schließen sie. Alles selbst vermessen
2026-08-21; 0 honored: was nicht belegt ist, bleibt pending.

## Zusammenfassung

| Kraft | Kanal | Fenster im Feld | Lücke | Schlüssel-Quelle |
|---|---|---|---|---|
| gravity | Sonnenposition | 1599→3000 (gemessen am CDN-Bin) | keine | DE440-SPK, `ephemeris_sun.bin` |
| acoustic | g-Moden Ω_g | 1277±10 nHz (BodyProperty stype 7) | Manifestation pending | `solar_omega_g.φ`, CI `--omega-g` |
| acoustic | p-Moden (GONG) | 31 Jahre, L 0..30 (496 Moden) | L 31..200 pending | `gong2.nso.edu`, `gong_compiler --lmax 200` |
| em bolometrisch | SSI (spectra.bin) | leer — spectra.bin 404 auf CDN | **die ganze Ernte** | NCEI-SSI netCDF-4/HDF5, monthly/daily/yearly |
| em Schmalband | GOES X-Ray 1–8 Å / 0.5–4 Å | live-only (7-Tage-Fenster) | 1974–heute als Serie | NCEI GOES XRS (xrsf-l2-avg1m, HDF5) |
| em Schmalband | GOES EUV 304/284 | live-only (7-Tage) | 2009–2020 (goes14/15) | NCEI GOES EUVS (geuv-l2-avg1m) |
| em Schmalband | F10.7 (Penticton) | live-only (f107_cm_flux) | 1947–heute täglich | NCEI pent_noontime-flux, LISIRD noaa_radio_flux |
| em Schmalband | Mg II-Index | — | 1978–heute | LISIRD bremen_composite_mgii |
| em Schmalband | Sonnenfleckenzahl | live (observed-solar-cycle) | 1818–heute (SILSO), 1749– (SWPC) | SILSO sn_d_tot, SWPC solar-cycle-indices |
| Sonnenwind | OMNI2 (B, N, T, V, p) | live-only (week_ago-HAPI) | **1963–heute** | CDAWeb-HAPI OMNI2_H0_MRG1HR, omni2_YYYY.dat |
| Sonnenwind | OMNI HRO 1-min | — | 1981–heute | CDAWeb-HAPI OMNI_HRO_1MIN |
| Sonnenwind | ACE (MAG/SWEPAM) | live-only (ace_mag_1h) | 1997/98–heute | SPDF level_2_cdaweb (CDF, cdf_reader steht) |
| Sonnenwind | DSCOVR | — | 2015–heute | CDAWeb-HAPI DSCOVR_COHO1HR_MERGED_MAG |
| electric | Solar-Orbiter-RPW | rpw_efield.bin (2020–2026) | Wind/WAVES 1994–2021 | SPDF wav_h1, cdf_reader (Atom pending) |
| electric | PSP-DFB / VSC | — | unverifiziert | register pending (kein VSC-Produkt gefunden) |

Vor 1610: für das Licht keine direkte Messung — das SSI-CDR rekonstruiert dort
nur über NNL-Modelle (Faculae/Flecken + F10.7), keine echte Messung. Das bleibt
Modell, nicht Messwert (0 honored).

## 1. Gravity — keine Lücke

`ephemeris_sun.bin` (CDN-Asset unter `ssd.jpl.nasa.gov`, 19 018 776 B, 302):
header `cf 86 02`; n_sections 4; granules 36020; CHEBYSHEV_DEGREE 17; erste
Granule-Mitte JD 2305344.5 (1599-09-20), letzte JD 2816832.5 (3000-02-15),
dt 16 d. Die DE440-SPK-Fenster sind damit im Bin: **1599→3000**. Der
`at sun`-Block in `phi/sources.φ:951` lädt das Asset. Die Position als Masse
ist überall und immer da. Zusätzlich trägt das Bin die g-Moden-Frequenz
(stype 7, `solar_omega_g.φ`: `sun 1277 10` nHz) — Manifestation pending
(CI `bodies`-Job `--omega-g`, `ledger.φ` solar-akteure-Note).

## 2. em bolometrisch — die ausstehende Ernte (der Auftrag)

`spectra.bin` ist **404** auf dem CDN (gemessen). Der Block wartet
(`phi/sources.φ:1719`, `format spectral`, Feld
`spectral_irradiance_W_m2_Hz`). Die Pipeline steht komplett
(`src/spectral.rs`: bins_from_lambda_rows, write/parse_spectral_bin,
Monatsmitte-TDB) — es fehlt nur die Ernte (Handover NCEI-SSI-HDF5, Faden A).

### NCEI Solar Spectral Irradiance CDR (Config 01B-33, Coddington, DOI 10.25921/esjz-1w61)

Zugang offen, keyless:
`https://www.ncei.noaa.gov/data/solar-spectral-irradiance/access/`

Vermessen 2026-08-21 (live, HTTP 200):
- `standard-resolution/monthly/` — **154 Dateien**, 1874-05 → 2026-06
  (letzte 2026-01/03 + 2026-04/06 als `ssi_v03r00-preliminary_monthly`).
  Enthalten auf der Platte schon 2 Proben im Katalog
  (`phi/pipeline/katalog/ncei_ssi/`: `filters.h5`, eine 1874er und eine
  2026er preliminary — 172/344 KB).
- `standard-resolution/daily/` — **154 Dateien**, 1874-05-09 → 2026-06-30.
- `standard-resolution/yearly/` — **1 Datei** `s1610_e2025_c20260305.nc`
  (1610–2025).
- `high-resolution/monthly/` — 154 Dateien (115–500 nm, ~9700 Bänder).

Format: **netCDF-4 = HDF5**, Magie `89 48 44 46 0d 0a 1a 0a` (bestätigt mit
`xxd` auf dem Preliminary-File). `src/netcdf.rs` liest nur classic — die Lücke
ist `src/hdf5.rs`. Die parallele Session hat bereits einen HDF5-Reader
entworfen (`src/hdf5.rs`, `hdf5_reader.rs`, `dbg_test.rs`, `tests/` — im Baum
uncommitted, benannt). Der Reader ist der Türöffner; die Ernte braucht dann
`spectral_compiler --input-nc <file.nc>` (oder einen kleinen
`ncei_ssi_compiler`): wavelength+ssi extrahieren, über
`bins_from_lambda_rows` → spectra.bin, `--ci-mode` → CDN.

Erwartung nach Ernte: ~4300 Bänder, Integralsumme ≈ 1361 W/m² bei 1 AU;
monthly 1874–heute (~152 a), yearly 1610–heute (~416 a). LISIRD bietet
ergänzend (keyless CSV): `noaa_radio_flux` (F10.7, 1947–), `historical_tsi`,
`gsfc_composite_ssi` (1978–), `sorce_ssi_l3`, `tsis_ssi_24hr`, `sdo_eve_ssi_1nm_l3`
— als Zweitquelle / Lückenfüller für die aktuelle Epoche.

## 3. em Schmalband — live-only, Historie als Ernte offen

Ist im Feld: GOES X-Ray (xrays-7-day, beide Bänder via `where energy`),
GOES EUV (euvs-7-day, where line 304/284), F10.7 (f107_cm_flux), RTSW
mag/wind (1-min), DONKI CME/FLR, GOES Protonen/Elektronen/Magnetometer,
suvi-flares, solar_regions (count), observed-solar-cycle-indices (SSN).
Alle `at sun` (außer die Partikel-/Magnetometer-Böcke, die auf der
GOES-Raumsonde liegen). τ Sekunden bis Minuten. Keine Historie im Feld.

### GOES XRS als Serie (die 1974–heute-Lücke)

NCEI „GOES Space Environment Monitor" (keyless, bestätigt):
`https://www.ncei.noaa.gov/data/goes-space-environment-monitor/access/science/xrs/`

- **xrsf-l2-avg1m_science** (1-min-Mittel, netCDF-4/HDF5, daily):
  goes08 1995–2003, goes10 1998–2009, goes12 2003–2007, goes13 2013–2017,
  goes14 2009–2020, goes15 2010–2020 → **1995–2020 als Serie**
  (Namensmuster `sci_xrsf-l2-avg1m_g15_d20200101_v2-2-1.nc`, ~1 Datei/Tag).
- **full/** (1974–2020, 3-s-Daten, netCDF classic `CDF\x01` + CSV):
  `{year}/{month}/{sat}/netcdf/gNN_xrs_3s_YYYYMMDD_YYYYMMDD.nc` —
  SMS/GOES-1-7-Epoche 1974–1990, GOES-8+ 1995–2020. Damit ist die
  **1974–2020-Lücke als Serie belegbar** (3-s-Rohdaten + 1-min-Mittel).
- GOES16-18 (EXIS, 2017–): live via SWPC-JSON (xrays-7-day), historisch via
  `EXIS-L2-XRSF-l2-avg1m_science` auf `noaa-goes16/17/18.s3.amazonaws.com`
  (Prefix verifiziert).

Der `where`-Filter (ledger.φ:578, `where energy 0.05-0.4nm`) liefert die
Band-Trennung; ein `goes_xrs_compiler` analog `bia_efield_compiler` erntet die
Tages-NetCDFs zu einer Serie (at sun, wm2_1au-Konvention). Die Frage ist das
Sample-Budget (1-min über 46 a ≈ 24 Mio. Werte) — die Auflösung ist zu wählen
(1-min-Mittel, daily, oder Rebuild mit Kappung).

### F10.7 als Serie (1947–heute)

- NCEI Penticton noontime flux: `pent_noontime-flux_1947.txt` … `_2026.txt`
  (80 Jahresdateien, bestätigt 200; 1947 und 2026 geprüft).
- LISIRD `noaa_radio_flux` (daily, 1947–heute, bestätigt 200).
Beide keyless. `f107_cm_flux.json` bleibt der Live-Kanal.

### Mg II-Index, Sonnenflecken, EUV

- Mg II: LISIRD `bremen_composite_mgii` (1978–, keyless).
- SSN: SILSO `SN_d_tot_V2.0.txt` (1818–, daily, bestätigt), SWPC
  `observed-solar-cycle-indices.json` (1749–, monthly, 3331 Records).
- EUV: NCEI `science/euvs/goes14/15` `geuv-l2-avg1m` (2009–2020, HDF5,
  bestätigt).

## 4. Sonnenwind — live-only, Serien bei CDAWeb/SPDF

Ist im Feld: RTSW 1-min (mag/wind), ACE 1-h (SWPC-JSON), OMNI2
(week_ago-HAPI, 7 Felder), PSP (COHO1HR/SWP/FLD), Solar-Orbiter-RPW. Alle
live, nur das Fenster. Die **Serien** liegen bei CDAWeb-HAPI und SPDF:

- **OMNI2_H0_MRG1HR**: HAPI-Info start **1963-01-01**, stop **2026-08-06**
  (gemessen). 7 Parameter (V1800, N1800, T1800, BX/BY/BZ, Pressure) — der
  Block existiert, aber mit `{week_ago}`. Die volle Serie ist per
  `time.min=1963…` erntbar (Stundenwerte, ~550 k Records; Sample-Budget).
- **OMNI_HRO_1MIN**: 1981–2026 (1-min).
- **omni2_YYYY.dat** (low_res, ASCII): 64 Jahresdateien 1963–2026 auf SPDF
  (`spdf.gsfc.nasa.gov/pub/data/omni/low_res_omni/`), bestätigt 200.
- **ACE**: SPDF `ace/mag/level_2_cdaweb/mfi_h0/` (1997–2026, Tages-CDFs,
  CDF = cdf_reader steht), `ace/swepam/level2_ascii/` (1hour/1day), und
  `1m_mrgd_mag_plasma_at_bowshock.txt` (1-min, 1998–heute, 200).
- **DSCOVR_COHO1HR_MERGED_MAG**: HAPI 2015–2026 (stündlich, 200).

Force-Gate: IMF → em (die Messung IST das Magnetfeld), speed → advective
(patch-levy), density → diffusion, Temp → thermal, pressure → advective —
die Zuordnung steht bereits im OMNI2-Block. E1800 bleibt entfernt
(derived: E=−V×B, ledger.φ:327).

## 5. electric — RPW steht, Wind/WAVES pending

- Solar-Orbiter-RPW: `rpw_efield.bin` auf dem CDN (302), 2020–2026,
  at solar_orbiter — geschlossen (ledger.φ:327).
- Wind/WAVES `wav_h1` E_VOLTAGE_RAD2 (1994–2021, Tages-CDFs): cdf_reader
  steht, die Ernte ist ein eigenes Atom (ledger.φ kraft-kanal electric
  wind-waves).
- PSP-DFB = em-Spektralkanal (Force-Gate-Urteil); PSP-VSC: kein Produkt
  gefunden (register pending, 0 honored).

## 6. Quellen-Checkliste (alle am 2026-08-21 verifiziert)

| Quelle | URL | Status |
|---|---|---|
| NCEI-SSI monthly | `…/solar-spectral-irradiance/access/standard-resolution/monthly/` | 200, 154 Dateien |
| NCEI-SSI daily | `…/standard-resolution/daily/` | 200, 154 Dateien |
| NCEI-SSI yearly | `…/standard-resolution/yearly/` | 200, s1610_e2025 |
| NCEI GOES XRS 1-min | `…/goes-space-environment-monitor/access/science/xrs/` | 200, goes08–15 |
| NCEI GOES full | `…/goes-space-environment-monitor/access/full/` | 200, 1974–2020 |
| NCEI GOES EUVS | `…/science/euvs/goes14/` | 200, geuv-l2-avg1m |
| NCEI Penticton F10.7 | `…/solar-radio/noontime-flux/penticton/` | 200, 1947–2026 |
| SILSO SSN | `sidc.be/SILSO/INFO/sndtotcsv.php` | 200, 1818– |
| SWPC solar-cycle-indices | `services.swpc.noaa.gov/json/solar-cycle/…` | 200, 1749– |
| LISIRD f107 | `lasp.colorado.edu/lisird/latis/dap/noaa_radio_flux.csv` | 200 |
| CDAWeb-HAPI OMNI2 | `cdaweb.gsfc.nasa.gov/hapi/info?id=OMNI2_H0_MRG1HR` | 200, 1963–2026 |
| SPDF omni2_YYYY.dat | `spdf.gsfc.nasa.gov/pub/data/omni/low_res_omni/` | 200, 1963–2026 |
| SPDF ACE mag | `spdf.gsfc.nasa.gov/pub/data/ace/mag/level_2_cdaweb/` | 200, 1997–2026 |
| CDN spectra.bin | `…/releases/download/ssd.jpl.nasa.gov/spectra.bin` | **404** |
| CDN ephemeris_sun.bin | `…/ephemeris_sun.bin` | 302, 1599–3000 |
| CDN gong_modes.bin | `…/gong_modes.bin` | 302 |
| CDN rpw_efield.bin | `…/rpw_efield.bin` | 302 |

## 7. Auftrag → nächste Atome (Reihenfolge)

1. **Faden A** (bereits registriert): HDF5-Reader + NCEI-SSI-Ernte →
   `spectra.bin` auf den CDN; die Sonne leuchtet bolometrisch. Das ist der
   größte einzelne Lücken-Schließer (416 a Licht).
2. **Faden B** (bereits registriert): Solar-Kanäle in den probe_ring der
   TE-Maschine (GOES X-Ray/EUV + F10.7, Ring-Buffer).
3. **GOES XRS-Serie** (xrsf-l2-avg1m, 1995–2020) als `goes_xrs_compiler` —
   schließt die 1974–heute-Schmalband-Lücke bis zur EXIS-Ära.
4. **OMNI2-Full-Serie** (1963–heute) via HAPI-Window-Ernte — der
   Sonnenwind-Kanal als Serie, Grundlage für Nadel Ⅲ-Archive (Bz/OMNI).
5. F10.7-Historie (pent/LISIRD, 1947–) und Mg II (1978–) als Compiler,
   SSN-Ergänzung (SILSO 1818–).

Sample-Budget: 1<<22 Samples (`src/archivar.rs:9038`) kappt den Rebuild
epoch-absteigend — die historischen Serien (OMNI2 ~550 k, GOES-XRS ~25 M)
müssen gegen die Kataloge gewogen werden (offene-Atome-Handover Atom 1).
