<!--
  title: Survey — Kapitulationen & Pendings: Register-Inventur (Stand 2026-09-14)
  class: survey
  date: 2026-09-14
  sha256: d7d9c671f05cfc31a5d9b9bf718555f490db05a7e1c85bb4408fac210da9e1ca
  status: live
  see-also: phi/blocked_sources.φ phi/dead_sources.φ AGENTS.md
-->
# Kapitulationen & Pendings — Register-Inventur (Stand 2026-09-14)

Vollständige Inventur des Registers der aufgegebenen und offenen Quellen. Gemessen
am 2026-09-14 mit den Haus-Werkzeugen (`giveup_scan`, `pending_extract`,
`register_lookup`) und direkten `rg`-Zählungen über `phi/`. Kein neues Register,
kein Befund: der Survey trägt den gemessenen Bestand, die Werkzeuge bleiben die
Werkzeuge.

## Die drei Register-Dokumente + das Toolset

| Dokument | Inhalt | Einträge |
|---|---|---|
| `phi/blocked_sources.φ` | Pendings + gesperrte Quellen | **32** — 26 pending, 4 blocked, 2 reg |
| `phi/dead_sources.φ` | Kapitulationen (`decline`/`dead`) | **1218** — 808 decline, 410 dead (4883 Zeilen) |
| `phi/pipeline/refusal_ledger.φ` | abgelehnte Fetches | **76** refused |

Werkzeuge: `giveup_scan` (`cargo run -p omegaflow-utils --bin giveup_scan`),
`pending_extract` + `register_lookup` (`cargo run -p omegaflow-register --bin …`).

## 1. Pendings — `phi/blocked_sources.φ` (26)

| # | Zeile | Quelle | Zustand / nächster Schritt |
|---|---|---|---|
| 1 | 6 | Pioneer-10 ATDF S-Band-Doppler | 14-Feld-Serie, kein Skalar; kein Membran-Konsument (Tor 1) |
| 2 | 10 | NOAA MarineCadastre AIS vessel-traffic | kein Konsument (Tor 1) |
| 3 | 24 | US-CRN (CDS) | Form 403 → offener Direktdownload NCEI gemessen (HTTP 200) |
| 4 | 28 | NOAA-NODD NRS bioacoustic (spectral) | Spektrum gehalten, kein Skalar, kein Konsument |
| 5 | 32 | NOIRLab Astro Data Lab TAP | Reg. abgelehnt, TAP öffentlich; Wiedervorlage 2026-12-02 |
| 6 | 36 | COSMIC-2 GNSS-RO | Tarball-Entpackung + Ernte offen |
| 7 | 40 | GNIP Niederschlags-Isotope | kein Konsument (Tor 1) |
| 8 | 44 | ONC Hydrophon (MATLAB-v5) | Parser gebaut; Bin-Geometrie messen, dann Ernte |
| 9 | 48 | VOTable/TAP-Kataloge (18 Inventare) | 2 Fehlschläge; statische Photometrie ohne Konsument |
| 10 | 81 | ESO tap_cat | HARPS geerntet; Mehrband ohne Konsument |
| 11 | 85 | ESO tap_obs | ObsCore Discovery-Metadaten, kein Skalar-Feld |
| 12 | 89 | WFAU OSA (ATLAS DR1) | statische Photometrie, kein Konsument |
| 13 | 93 | WFAU SSA (SuperCOSMOS) | statische Photometrie, kein Konsument |
| 14 | 97 | WFAU VSA (VISTA/VVV) | Backend down + kein Konsument |
| 15 | 101 | WFAU WSA (UKIDSS) | Backend down + kein Konsument |
| 16 | 105 | VLASS (`cirada.ca`) | FITS-Reader-Gap; CADC youcat `cirada.VCSS` |
| 17 | 109 | NOAA CORS (RINEX) | Hatanaka/SBF/Trimble; `rinex 0.22.0` |
| 18 | 113 | NOAA ERI imagery | JPEG-in-TIFF-Dekoder offen |
| 19 | 117 | GK2A AMI | GSICS-Kalibrierung offen |
| 20 | 121 | GOES-16 ABI | Compiler gebaut; GSICS pending; GLM-Sibling |
| 21 | 125 | Himawari-8 AHI | Compiler gebaut; Kalibrierung (HSD-Block 5) offen |
| 22 | 137 | GDP Drifter | `.zarr` + Ernte offen |
| 23 | 141 | OCS Hydrodata | All-Survey-Ernte + `.tif`-LZW offen |
| 24 | 145 | WOD Ozeanprofile | Compiler gebaut; SOHM offen |
| 25 | 149 | NEXRAD Level II | Compiler gebaut; Feld-System-Reader + CI-Manifest offen |
| 26 | 153 | SuperDARN FITACF Direktroute | Compiler gebaut; erster CI-Manifest-Lauf offen |

Nachzug 2026-09-17: Die Zeilen 3 (US-CRN, `uscrn_hourly.bin`), 4 (NRS,
`nrs_audio_series.bin`), 6 (COSMIC-2, `cosmic_ro_temp.bin`), 16 (VLASS,
`vlass_tap_component.bin`/`vlass_tap_source.bin`), 17 (CORS,
`cors_1lsu_2024001.bin`), 21 (Himawari, `himawari_ahi_counts.bin`), 22
(GDP-Drifter, `gdp_drifter.bin`), 24 (WOD, `noaa_wod_2000_drb.bin`), 25
(NEXRAD, `nexrad_level2.bin`), 26 (SuperDARN, `superdarn_fitacf.bin`) sind
aufgelöst — die Assets stehen in `phi/sources.φ` (gemessen 2026-09-17). Die
Tabelle selbst gibt den Register-Stand 2026-09-14.

## 2. Gesperrt + Register-Verweise — `phi/blocked_sources.φ` (6)

| Zeile | Klasse | Quelle |
|---|---|---|
| 14 | `blocked key` | AQS Data Mart API (EPA) — key-pflichtig |
| 19 | `blocked account` | Babamul (Caltech LSST-Broker) — HTTP 401 Auth-Wall |
| 129 | `blocked parser-def las-laz` | NOAA NOS Coastal Lidar — LASzip-Chunk-Dekoder offen |
| 133 | `blocked parser-def las-laz` | USGS 3DEP EPT — LASzip-Chunk-Dekoder offen |

Nachzug 2026-09-17: Der LASzip-Chunk-Dekoder ist gebaut (`src/archivar/las/laszip.rs`,
`LazDecoder`/`has_laszip_vlr`, exportiert in `src/archivar/las/mod.rs:580–581`) —
beide `blocked parser-def las-laz`-Zeilen sind aufgelöst.
| 17, 22 | `reg` | AQS-API-Doku, Babamul-Signup |

## 3. Kapitulationen — `phi/dead_sources.φ` (1218)

4883 Zeilen; 1218 Keyword-Einträge. **157 davon tragen keine Klasse** (Keyword mit
trailing space, z. B. `:501` `decline `, `:2561` `dead `) — eine Parser-Lücke, kein
Wert. Klassifiziert: **1061** (654 decline + 407 dead).

### dead (410 gesamt; 407 klassifiziert, 3 ohne Klasse)

| Klasse | Anzahl | Klasse | Anzahl |
|---|---|---|---|
| 404 | 255 | 400 | 7 |
| dns-unresolved | 59 | transport | 5 |
| redirect-maintenance | 25 | dns | 5 |
| 5xx | 15 | ssl | 3 |
| timeout | 11 | scheme-unsupported | 3 |
| unreachable | 10 | 401 | 2 |
| | | tls-reset, origin-down, login-wall, absent, 503, 502, 333 | je 1 |

### decline (808 gesamt; 654 klassifiziert über 112 Klassen, 154 ohne Klasse)

**≥ 10:** no-physical-force 73 · variant 62 · model-forecast 58 · registry 41 ·
registry/katalog 34 · catalog 28 · redistribution 22 · superseded-by-integrated 21 ·
presence-catalog 16 · static 10 · position-only 10 · infrastructure 10

**9:** derived-orbit-fit · derived · alerts · aggregate

**8:** imagery

**7:** terrain · schedule · molecular · method · literature · derived-satellite-index

**6:** superseded · redundant · model · health-stats · duplicate · derived-product

**5:** table-not-found · stats · schema · presence · path · imagery-tiles · computation

**4:** randomness · model-reanalysis · commercial

**3:** unit-arm · superseded-by-ephemeris · simulation · kein-feld · docs · derived-model

**2:** station-registry · publisher · no-measurement · model-computation ·
metadata-registry · metadata-portal · kein-latlon · duplicate-eop · counts ·
catalog-registry · catalog-duplicate · archive-only · archive-catalog ·
aggregate-index · abgeleitet-tiefe

**1 (52):** synthetic-product · superseded-by-woudc · superseded-by-skyServerWS ·
superseded-by-pris · superseded-by-openaq · superseded-by-hapi · superseded-by-cencoos ·
superseded-by-box-route · superseded-by-api.woudc.org · static-catalog · static-archive ·
stale-feed · single-event · significance-registry · satellite-scene-catalog ·
registry-premium-gzip · registry-list · reference · redistributed-synop ·
product-file-index · probe-timeout · position-column-unverified · paleo-catalog ·
own-infrastructure · orbital-catalog · no-public-api · no-json-redundant ·
no-input-signal · model-fit · metadata-sparql · metadata-sos · legacy · kompilat ·
image-survey · image · html-page · html · form · flux-column-unverified ·
file-inventory · duplicate-omni · duplicate-intermagnet · direction-only ·
derived-radar-product · derived-crossmatch · columns · ckan-catalog · atlas ·
analysis-dataset · aggregate-statistics · aggregate-indices · aggregated-index

## 4. Refusals — `phi/pipeline/refusal_ledger.φ` (76)

76 `refused`-Zeilen (abgelehnte Fetches mit Zeitstempel, Kanal `extract-void`/`fetch-void`
und URL) — die Laufzeit-Kapitulation, getrennt vom kuratierten `dead_sources.φ`.

## 5. `giveup_scan` — 22 Wörter, 4394 Fundstellen

| Klasse | Fundstellen |
|---|---|
| declined | 3190 |
| descoped | 524 |
| gated | 373 |
| honest-face | 177 |
| unbuilt | 55 |
| unpublished | 29 |
| no-access | 26 |
| paywall | 19 |
| request-only | 18 |
| not-public | 11 |

Top-Dateien: `phi/dead_sources.φ` 868 · `phi/pipeline/catalog/terrapulse_catalog.φ` 369 ·
`phi/pipeline/catalog/copernicus_disposition.φ` 304 · `phi/pipeline/queue/grind_vires_full.φ` 194 ·
`phi/pipeline/catalog/noaa_nodd_disposition.φ` 161.

## 6. `pending_extract` — opencode.db

1 Treffer: Ernte-Session — „Passwort gültig, aber Profil nicht aktiviert (pending)".

## Methode

- `rg -c '^decline'` / `'^dead'` → 808 / 410; `rg -c '^(decline|dead)\s*$'` → 157 klassenlos.
- `rg -o '^(decline|dead)\s+\S+' … | awk '{print $1,$2}' | sort | uniq -c | sort -rn` → Klassen-Histogramm.
- `giveup_scan --summary` → Vokabular-Klassen und Top-Dateien.
- `pending_extract` → Klassifikation PENDING/WAIT/OPEN über `~/.local/share/opencode/opencode.db`.
