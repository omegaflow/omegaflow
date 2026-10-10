<!--
  title: Survey — Kapitulationen & Pendings: Register-Inventur (Stand 2026-09-14)
  class: survey
  date: 2026-09-14
  sha256: 521e7578fcd8cb9e0c8243cc7757b748e1bee0c85493221793357bd8b5608381
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

## 2. Gesperrt + Register-Verweise — `phi/blocked_sources.φ` (6)

| Zeile | Klasse | Quelle |
|---|---|---|
| 14 | `blocked key` | AQS Data Mart API (EPA) — key-pflichtig |
| 19 | `blocked account` | Babamul (Caltech LSST-Broker) — HTTP 401 Auth-Wall |
| 129 | `blocked parser-def las-laz` | NOAA NOS Coastal Lidar — LASzip-Chunk-Dekoder offen |
| 133 | `blocked parser-def las-laz` | USGS 3DEP EPT — LASzip-Chunk-Dekoder offen |
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

## Register-Gegenprobe (2026-10-11)

Die 32 offenen Marker (26 pending aus §1 + 4 blocked/2 reg aus §2) je mit `sgrep` gegen
`phi/sources.φ` / `phi/declined_sources.φ` / `phi/dead_sources.φ` / `phi/blocked_sources.φ`
gemessen: **31 gedeckt, 1 ungedeckt.** Die Zeilenspalte der Tabellen §1/§2 ist Stand
2026-09-14; die Register-Verweise unten sind der heutige Stand.

| # | Quelle | Verdikt | Register (2026-10-11) |
|---|---|---|---|
| 1 | Pioneer-10 ATDF S-Band | gedeckt | `sources.φ:19655` |
| 2 | NOAA MarineCadastre AIS | gedeckt | `declined_sources.φ:2516,2752` |
| 3 | US-CRN (CDS) | gedeckt | `sources.φ:1065` |
| 4 | NOAA-NODD NRS bioacoustic | gedeckt | `declined_sources.φ:4349` |
| 5 | NOIRLab Astro Data Lab TAP | gedeckt | `sources.φ:21327` |
| 6 | COSMIC-2 GNSS-RO | gedeckt | `sources.φ:10986` |
| 7 | GNIP Niederschlags-Isotope | gedeckt | `declined_sources.φ:1476` |
| 8 | ONC Hydrophon | gedeckt | `sources.φ:11089` |
| 9 | VOTable/TAP-Kataloge | gedeckt | `sources.φ:21307`; `blocked_sources.φ:8` (votable-reader) |
| 10 | ESO tap_cat | gedeckt | `sources.φ:20067,21276` |
| 11 | ESO tap_obs | gedeckt | `declined_sources.φ:1046` |
| 12 | WFAU OSA (ATLAS DR1) | gedeckt | `sources.φ:10467` |
| 13 | WFAU SSA (SuperCosmos) | gedeckt | `sources.φ:10480` |
| 14 | WFAU VSA (VISTA/VVV) | gedeckt | `declined_sources.φ:171` |
| 15 | WFAU WSA (UKIDSS) | gedeckt | `declined_sources.φ:175` |
| 16 | VLASS (`cirada.ca`) | gedeckt | `sources.φ:20433,21087` |
| 17 | NOAA CORS (RINEX) | gedeckt | `sources.φ:11565` |
| 18 | NOAA ERI imagery | **ungedeckt** | nur `phi/pipeline/catalog/noaa_nodd_disposition.φ:36` (pending Register-Duty) |
| 19 | GK2A AMI | gedeckt | `sources.φ:1075` |
| 20 | GOES-16 ABI | gedeckt | `sources.φ:1084` |
| 21 | Himawari-8 AHI | gedeckt | `sources.φ:1111` |
| 22 | GDP Drifter | gedeckt | `sources.φ:1120` |
| 23 | OCS Hydrodata | gedeckt | `declined_sources.φ:5852` |
| 24 | WOD Ozeanprofile | gedeckt | `sources.φ:11644` |
| 25 | NEXRAD Level II | gedeckt | `sources.φ:4370` |
| 26 | SuperDARN FITACF Direktroute | gedeckt | `sources.φ:19578` |
| 27 | AQS Data Mart API (`blocked key`) | gedeckt | `sources.φ:1536,1700` |
| 28 | Babamul (`blocked account`) | gedeckt | `sources.φ:965` |
| 29 | NOAA NOS Coastal Lidar (las-laz) | gedeckt | `declined_sources.φ:5848` (terrain) |
| 30 | USGS 3DEP EPT (las-laz) | gedeckt | `declined_sources.φ:5857` (terrain) |
| 31 | AQS-API-Doku (`reg`) | gedeckt | mit #27 |
| 32 | Babamul-Signup (`reg`) | gedeckt | mit #28 |

**Ungedeckter Rest:** NOAA ERI imagery (#18) — JPEG-in-TIFF-Dekoder; nur als
pending Register-Duty in `phi/pipeline/catalog/noaa_nodd_disposition.φ:36`, in keiner der
vier Register.

## Methode

- `rg -c '^decline'` / `'^dead'` → 808 / 410; `rg -c '^(decline|dead)\s*$'` → 157 klassenlos.
- `rg -o '^(decline|dead)\s+\S+' … | awk '{print $1,$2}' | sort | uniq -c | sort -rn` → Klassen-Histogramm.
- `giveup_scan --summary` → Vokabular-Klassen und Top-Dateien.
- `pending_extract` → Klassifikation PENDING/WAIT/OPEN über `~/.local/share/opencode/opencode.db`.
