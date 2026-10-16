<!--
  title: Survey — Ketten-Quellen: harter Taucherlauf über fünf Quellen (2026-10-15)
  class: survey
  date: 2026-10-15
  sha256: 3de10f358d1566d49773c8fe311a9dc76d65f537fe4793a268c6c2b6b1448dec
  status: live
  see-also: phi/sources.φ docs/concepts/source-curation.md docs/concepts/tools-map.md docs/handover/archiv/handover-2026-10-15-river-folge214.md docs/handover/archiv/handover-2026-10-15-mountain-folge351.md
-->
# Survey — Ketten-Quellen: harter Taucherlauf über fünf Quellen (2026-10-15)

Operator-Auftrag 2026-10-15: fünf Kettenquellen (SWIFF · WINDMI · MAGION ·
MOSWOC-Kp · OVATION-AACGM) mit **harten Bandagen** und der weiten Leiter
(`bin/all_wide`) prüfen. Fünf Taucher liefen je Quelle; die Befunde wurden am
Baum und im Netz nachgemessen (Register, Zenodo-API, `--verdict`, `--jina`).
Diese Survey trägt den gemessenen Stand. Keine Zahl kopiert, keine Lücke
geglättet. Gemessen 2026-10-15.

## Methode

- **Fünf Taucher** (ein begrenzter Schritt je Quelle) + die vier von Mountain
  selbst nachgeholten `bin/all_wide`-Läufe (SWIFF · WINDMI · MAGION · MOSWOC-Kp ·
  OVATION-AACGM; Spills in `omegaflow-work/recherche/spill/omegaflow_all_1792055*`).
- **Baum-Verifikation:** `archive_search <name> --root phi` — für alle fünf
  **0 files, 0 hits**; keine der fünf trägt eine `phi/sources.φ`-Zeile.
- **Quellen-Direktmessung:** Zenodo-API (`--jina`), `--verdict`, `--zenodo`.
- **Form-Defekt (gemessen):** der Taucher-Sandbox verweigert `bin/all_wide`
  (dreimal, drei Läufe); Mountain führt den Wrapper ohne Beanstandung aus
  (`bin/all_wide "SWIFF"` → 465 Result-Zeilen). Die Profil-Allowlist deckt den
  Wrapper nicht — die Breite der Taucher lief daher über `archive_search --all`
  und die Einzelmodi. Kein Messverlust, aber der Weg war nicht der kanonische.

## Die fünf, gemessen

### SWIFF — Projekt/Papier/Modell, **kein Datensatz** (Behauptung bestätigt)
- **FP7-Projekt** „Space Weather Integrated Forecasting Framework", CORDIS
  `https://cordis.europa.eu/project/id/263340`, Grant 263340 (KU Leuven,
  2011–2014); Ergebnisseite = **101 Publikationen, kein Datensatz**.
- **Papier:** Lapenta et al. 2013, *J. Space Weather Space Clim.* 3 A05, DOI
  `10.1051/swsc/2013027` (OpenAlex W2115024671) — CC-BY 2.0.
- **Modell:** SPM → **BSPM** (Belgian SWIFF Plasmasphere Model), PITHIA-NRF
  `https://esc.pithia.eu/data-collections/DataCollection_BSPM_2Dmaps/`.
- **Datenprodukt:** keines — absent auf Zenodo/DataCite/re3data/CORE/OSF/Dryad/
  IA/Mendeley/GitHub/NSSDCA/data.europa (gemessen). Der Name SWIFF trifft heute
  nur eine Blog-/Film-Festival-Namensgleichheit plus SEO-Spam.
- **Route:** nur Publikations-Metadaten (CORDIS/OpenAlex) und das Nachfolge-Modell
  BSPM sind erreichbar — beides **kein Datensatz**.

### WINDMI — CCMC-Rechenmodell, **kein Messdatensatz** (Behauptung teil-bestätigt, ein Riss)
- **Modell:** CCMC-Katalog `spase://CCMC/SimulationModel/WINDMI/1.0`,
  physics-based production model; Ausgabe laut Katalog: **Dst (Ringstrom-Energie)
  und AL**. Katalog: `https://ccmc.gsfc.nasa.gov/CMR/WS/get/ModelInfo?SpaseResourceID=spase://CCMC/SimulationModel/WINDMI/1.0`.
- **Live-Nowcast** (HTTP 200): `https://ccmc.gsfc.nasa.gov/cgi-bin/WINDMIpred.cgi`
  — Dst/AL über die letzten 3 Tage, GIF (`idl_images/`). Rolling-Plot-Baum
  `https://iswa.ccmc.gsfc.nasa.gov/iswa_data_tree/model/magnetosphere/WINDMI/`
  (4 Varianten, Jahre 2012–2026, `.gif`, 10-min-Takt) — **kein numerisches
  Produkt, keine DOI**.
- **~1 h voraus:** bestätigt (Mays/Horton/Spencer/Kozyra 2009, DOI
  `10.1029/2008SW000459`, verbatim „approximately 1 h before").
- **Riss:** „Nutzung auf Anfrage" ist am Baum **nicht** getragen — es ist ein
  **automatischer Dauerlauf**; das Runs-On-Request-Portal
  (`https://ccmc.gsfc.nasa.gov/ror/index.php`) führt WINDMI **nicht**, und
  `run_WINDMI.cgi`/`models/modelinfo.php?model=WINDMI` antworten **404**.
- **Ergänzend (Community, nicht CCMC):** Zenodo `10.5281/zenodo.11061738`
  (Substorm-Dataset, CC-BY-4.0), `10.5281/zenodo.11133963` (WindmiModel-Software).

### MAGION — Mikro-Satelliten-Serie, **kein „ein Satelliten-Magnetometer"** (Behauptung korrigiert)
- **MAGION** ist eine Serie tschechischer Mikro-Subsatelliten (MAGION-4 = Interball
  Tail S2-X `1995-039F`; MAGION-5 = Interball-2 Auroral S2-A `1996-050B`). Das
  Magnetometer ist ein separates Instrument: **SG-R8** (Fluxgate, ±65536…±128 nT)
  auf MAGION-4/5, **DMA** auf MAGION-5; Quelle IKI/UFA-CAS (HTTP 200).
- **Kein offenes MAGION-Subsatelliten-Produkt mit DOI.** Die offenen, DOI'ten
  Interball-Magnetometer-Daten sind die des **Hauptsatelliten** (IMAP-3), nicht
  MAGION: CDPP `10.24400/802406/mqcs-ddjr` (Sat-frame), `m8ey-bbm7` (GSM);
  NASA/SPDF `IA_K0_MFI` (CDAWeb/HAPI, CC0, 2-min).
- **Zugang MAGION:** PI-signierter, passwort-gated IKI-Archiv (FTP);
  `http://www.iki.rssi.ru/ida.html`; Host `worsta.iki.rssi.ru` **TCP-Timeout**
  (gemessen) — der „zeitweise unerreichbar"-Befund bestätigt. NSSDCA-Live-Records
  404 (Link-Rot), Wayback-Snapshots existieren.

### MOSWOC-Kp — Zenodo-Paket, **aber restricted** (Riss zu „frei abrufbar")
- **Zenodo:** Version-DOI `10.5281/zenodo.5211482` (Concept `…5211481`), Titel
  „MOSWOC Kp Forecast Verification Dataset", Bingham (Met Office), 2021-08-17.
- **`access_right: restricted`, `files: []`** (eigene API-Messung); `--files`/
  `--access/links` → **HTTP 403**. Kein anonymer Download; Freigabe nur über
  einen Zenodo-Access-Request (POST, login-gated).
- **Lizenz:** Non-Commercial Government Licence v2.0 (in der Beschreibung; kein
  `license`-Feld). **Span:** 2019-03-01 → 09-30 (7 Monate). **Takt:** 3 h.
  **Horizont:** 3 Tage. Kp **und** Kuk je 3-h-Intervall, Dateien `.json`.
- **Riss:** „geprüft erreichbar" gilt für die **Record-Seite** (HTTP 200), **nicht**
  für die **Daten** (restricted). Die zwei Linien stehen getrennt.

### OVATION-AACGM — Zenodo-Paket, CC-BY, **eine Station (Mohe)** (Riss zu „je Station")
- **Zenodo:** Version-DOI `10.5281/zenodo.19651247` (Concept `…19651246`),
  „OVATION-AACGM and OVATION-Dipole Modeled Auroral Distribution Data with
  Analysis Code", Zhao (2026-04-19); `license cc-by-4.0`, `access_right open`.
- **Direkter Abruf (HTTP 200):** `https://zenodo.org/records/19651247/files/OVATION_AACGM.zip?download=1`
  (4.9 MB, md5 `1dfcbbe8e3bd738f8ed3bbf579e7267b`); Paket = **genau eine Datei**.
- **Inhalt:** `.mat`-Zeitreihen der minimalen Distanz OVATION-AACGM-modellierter
  Aurora-Grenzen zu **Mohe** (2023–Jan 2026) plus Dipole-Vergleich, DMSP/SSUSI
  `.nc`, MATLAB-Analysecode. **Nur eine Station** — kein Mehr-Station-Produkt.
- **Riss:** „je Station — direkt relevant für die Stationsplatzierung" ist
  überzogen: eine Validierungs-Datei für einen ostasiatischen Ort, kein
  Platzierungswerkzeug. **Offen:** `.mat`-Spaltennamen, Takt.
- **Zugehöriges Papier:** Zhao et al. 2026, DOI `10.1029/2026JA035287`.

## Register-Stand

Keine der fünf ist registriert (`--root phi` = 0/0 für alle fünf). Sie stehen
bisher nur als Prosa in `handover-2026-10-15-river-folge214.md` (Abschluss) und in
`state/research/queries.φ`. Die Zusicherung river-214:293 „die Zugangswege stehen
oben im Ketten-Punkt" trägt der Baum **nicht** — der Ketten-Punkt (:168-174) nennt
die fünf nicht namentlich; ein Doku↔Baum-Riss.

## Risse (nicht geglättet)

1. **MOSWOC-Kp**: „frei abrufbar" (river) vs. `access_right: restricted` (Baum/API).
2. **OVATION-AACGM**: „je Station" (river) vs. **eine Station Mohe** (Record).
3. **MAGION**: „ein Satelliten-Magnetometer" (river) vs. **Subsatelliten-Serie +
   separates Instrument SG-R8/DMA**; kein offenes MAGION-Produkt.
4. **WINDMI**: „Nutzung auf Anfrage" (river) vs. **automatischer Dauerlauf ohne
   Request-Route**.
5. **river-214:293** „die Zugangswege stehen oben im Ketten-Punkt" vs. der Baum
   (kein solcher Block).

## Was das für die Aneignung heißt

- **Nur OVATION-AACGM** trägt heute eine offene Lizenz (CC-BY-4.0) und einen
  direkten Abruf — aber als Ein-Station-Validierungs-Dataset (`.mat`, ohne
  Parser). Eine **Feld-/Kernel-Zuordnung** wäre eine Pro-Solo-Grenzentscheidung
  (Rat), keine Mountain-Alleinentscheidung; der `format`-Arm fehlt.
- **MOSWOC-Kp** ist als Quelle **nicht frei erntbar** — der Zugang ist ein
  Dritt-Akt (Zenodo-Account + Freigabe-Anfrage) → **per-Akt-Operator-Wort**, kein
  Register-Verdikt.
- **SWIFF / WINDMI / MAGION** liefern **keinen Datensatz** (Modell / Modell /
  gated Archiv) — die Identität als „Quelle" trägt der Baum nicht.
- **Verdikt (Mountain, gemessen):** keine der fünf ist heute als Feldquelle
  zulassungsfähig; eine Registrierung braucht zuerst die Feld-/Kernel-/Format-
  Entscheidung (Rat) bzw. die Zugangs-Freigabe (Operator/Mycelium).

## Leads (aus dem weiten Lauf, nicht verfolgt)

- **BAS IMAGE Auroral Boundary Data** — `https://www.bas.ac.uk/project/image-auroral-boundary-data/`
  — eine echte Aurora-Grenz-Datenquelle (British Antarctic Survey); nicht Teil
  der fünf, aber thematisch benachbart zur Stationsfrage.
- **NASA/SPDF + CDPP Interball-Magnetometer** (IMAP-3, CC0/DOI) — offen und
  DOI't, aber Hauptsatellit, nicht MAGION.
### MOSWOC-Nachbarn — direkt vermessen 2026-10-15 (`--jina` Zenodo-API + `--datacite`)

Der gezielte `all_wide` auf MOSWOC-Kp (Spill `omegaflow_all_1792056472_MOSWOC_Kp.txt`,
436 Zeilen) brachte drei offene Nachbarn; sie sind hier an der Quelle gemessen.

- **Flare-Verifikation `10.5281/zenodo.344886`** — „sophiemurray/moswoc-flare-verification",
  Murray, 2017-03-01, **Software**, `access open`, Lizenz **`other-open`** (kein
  SPDX-Name); eine Datei `…-v1.0.zip` (42 244 B, md5 `1a930e1d…`); Daten+Codes
  zum Paper „Flare forecasting at MOSWOC". **Größe: Sonnen-Flares (X-ray), nicht
  Kp** — direkter Abruf offen.
- **REFM-Elektronenfluenz `10.5281/zenodo.16846575`** — „Operational REFM Forecasts
  from MOSWOC", Bocquet (Met Office; ORCID 0000-0002-1414-4326), 2025-08-13,
  **Dataset**, `open`, **Apache-2.0**; eine Datei `operational_refm_outputs.tgz`
  (1.9 MB, md5 `371458e3…`); Felder: rohe REFM-Elektronenfluenz T+24/48/72,
  Offsets, debiased Fluenz. **Größe: Strahlungsgürtel-Elektronen, nicht Kp** —
  direkter Abruf offen.
- **Kp-Zeitreihe `10.5281/zenodo.18718835`** (Concept `18718106`) — Gamba Torres,
  2026-02-21, **Version 1.1**, **CC-BY-4.0**; Dateien
  `Kp_Hybrid_ML_ARIMA_Processed_Dataset.csv` (378 KB, md5 `4d284794…`) + README +
  drei `.m`-Scripts; **tägliche** Reihe **2010–2024** aus NOAA SWPC + GFZ Potsdam;
  Variablen: Kp-Komponenten, Ap (Komponenten + Tagesmittel), Sunspot Number,
  F10.7, Bartels-Rotation. **Riss zur Behauptung „Kp-Wahrheit":** es ist eine
  **prozessierte, abgeleitete Tages-Reihe**, **keine rohe 3-h-Kp-Reihe** — als
  Wahrheits-Lead nur mit Vorbehalt (die Primär-Kp liegt bei GFZ/NOAA, nicht hier).
- **MOSWOC-Betriebs-Aurora-Modell** (OVATION-Prime 2013): Mooney et al. 2021,
  „Evaluating Auroral Forecasts Against Satellite Observations",
  `10.1029/2020SW002688` (JGR Space Physics, OpenAlex W3021080074); Mooney et al.
  2024, „…Under Different Levels of Geomagnetic Activity", `10.1029/2023JA031478`
  (W4403409033). MOSWOC's Operationsmodell (30-min-Aurora-Oval), gegen
  **IMAGE-FUV**-Aurora-Grenzen validiert; das Modell trifft die äquatorwärtige
  Grenze gut, die polwärtige/Nachtseite schlechter. Beide DOI direkt **403**
  (AGU-Paywall), Wayback-Snapshot vorhanden. **Nicht Teil der fünf**, aber der
  direkte Bezug zur Aurora-/Stationsfrage.

## Offen

- **Feld-/Kernel-/Format-Entscheidung** für OVATION-AACGM (Rat) — erst dann eine
  Registerzeile.
- **`.mat`-Spaltennamen + Takt** von OVATION-AACGM (kein MAT-Leser im Haus).
- **MOSWOC-Kp-Zugang** — Zenodo-Access-Request ist ein per-Akt-Operator-Wort.
- **MAGION-Subsatelliten-Magnetometer** — nur PI-gated (IKI) oder IAP-CAS-Katalog;
  Freigabe = pending.
