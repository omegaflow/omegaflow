<!--
  title: Exposom-Quellenmatrix — somatisch + psychosomatisch (Stand 2026-10-04)
  class: survey
  date: 2026-10-04
  sha256: 829a979f761846251edcc02820561f868477be8a0feb6c442f1c0bb718dda18b
  status: live
  see-also: docs/concepts/kybernaut-native-methodology.md
-->

# Exposom-Quellenmatrix — somatisch + psychosomatisch

Ziel: eine Zeile je Krankheitsklasse = eine TE-Messung. Quelle mit DOI/URL + `--verdict`-Status
(HTTP, gemessen 2026-10-04 via `./bin/archive_search`). Unbekanntes = `pending`, nie 0.

Diese Matrix ist die Quellen-Landkarte für den generischen TE-Lauf: jede Zeile
nennt einen Schlüssel-Review (y↔Exposom-Zusammenhang), eine offene Kohorte/einen
Datensatz, eine feedbare y-Serie aus dem bisherigen y-Bestand, eine offene
x-Serie sowie den Kollokations-Stand (Ort + Zeit in derselben Serie vorhanden?).

Legende Kollokation: `ja` = Ort und Zeit gemessen vorhanden · `pending` = Zeit
vorhanden, Ort nicht gemessen/nicht im Datensatz · `nein` = strukturell nicht
kollokierbar (Zeit verschoben oder Ort maskiert).

Legende Verdikt-Status (`--verdict`, Stufe 1 direkt): `200`/`206` = gefunden ·
`403` = direkte Route blockiert, wayback-Snapshot 200 (Route = key/geo-needed,
nicht die Quelle) · `404` = pending. Jede Zeile nennt den HTTP-Stand.

---

## A. Exposom-Domänen (Spalten) und offene x-Serien

| Domäne | offene x-Serie (URL) | --verdict | Access | Register-Stand (gemessen 2026-10-06) |
|---|---|---|---|---|
| Luft (PM2.5/PM10/O3/NO2) | OpenAQ — https://openaq.org/ | 206 | offen | live `phi/sources.φ:1208` |
| Wasser | WQP/USGS — https://www.waterqualitydata.us/ | 200 | offen (US) | live `phi/sources.φ:18339` |
| Lärm | EEA/EIONET — https://www.eea.europa.eu/en/datahub · https://cdr.eionet.europa.eu/ | 206 | offen | EEA-Noise live `phi/sources.φ:9559`; `cdr.eionet` neu, kein Arm |
| Licht | Black Marble — https://blackmarble.gsfc.nasa.gov/ · VIIRS/LADS — https://ladsweb.modaps.eosdis.nasa.gov/ | 200 | offen | Host 200 (397 B Stub); LADS-API declined `phi/declined_sources.φ:2624` |
| Wetter/Klima (T/p/Feuchte) | Open-Meteo — https://open-meteo.com/ · NASA POWER — https://power.larc.nasa.gov/ | 200/206 | offen | live (`nasa_power_t2m_axis_value_text` `phi/sources.φ:17660`) |
| Geomagnetik/Raumwetter (Kp/Bz/GIC) | OMNIWeb — https://omniweb.gsfc.nasa.gov/ | 200 | offen | live (OMNI2 `phi/sources.φ:613`) |
| Pollen | CAMS/ADS — https://ads.atmosphere.copernicus.eu/ | 200 | Key/Registrierung | ADS = Key; `pollen.copernicus.eu` antwortet nicht → pending |
| Grünraum | MODIS/Sentinel-2 NDVI — https://lpdaac.usgs.gov/ · https://dataspace.copernicus.eu/ | 200 | offen | LPDAAC live (GEDI/MOD11); Copernicus-Katalog declined `phi/declined_sources.φ:1198` |
| gebaute Umwelt | OSM — https://download.geofabrik.de/ · GHSL — https://ghsl.jrc.ec.europa.eu/download.php | 206/200 | offen | GHSL live `phi/sources.φ:17519`; `geofabrik` neu, kein Arm |
| Ernährungsumfeld | USDA Food Access — https://www.ers.usda.gov/data-products/food-access-research-atlas/ | 200 | — | declined `phi/declined_sources.φ:4880` (sozioök. Index, kein physik. Messwert) |
| Arbeitsumfeld | CANJEM / O*NET — (x-Home pending) | pending | offen/Registrierung | OSHA CEHD gebaut (ZCTA-Geocoder, 2026-10-06) |
| Chemikalien | Exposome-Explorer — https://exposome-explorer.iarc.fr/ | 200 | — | declined `phi/declined_sources.φ:1984` (Katalog, keine Konzentration) |

Kern-x-Serie (im Bestand): OpenAQ · Open-Meteo · NASA POWER · OMNIWeb — alle
vier am 2026-10-04 mit HTTP 200/206 gemessen.

**§A-Neumessung 2026-10-06** (`archive_search --verdict`): Wasser/Lärm/Grünraum/gebaute
Umwelt sind bereits live; Ernährungsumfeld und Chemikalien sind bereits declined — die
früheren `pending`-Zeilen waren gegen den Baum veraltet (Riss, benannt). Echt neue,
unregistrierte Kandidaten ohne deckenden Arm (daher keine `phi/sources.φ`-Zeile):
`cdr.eionet.europa.eu`, `download.geofabrik.de`, `ads.atmosphere.copernicus.eu` (Key),
`blackmarble.gsfc.nasa.gov`.

---

## B. Die Matrix — eine Zeile je Klasse

### B1. Somatisch

**1 · kardiovaskulär**
- Domänen: Luft, Lärm, Wetter, gebaute Umwelt, Ernährungsumfeld (gemessen belegt).
- Schlüssel-Review: *Exposome in ischaemic heart disease: beyond traditional risk factors*, Eur Heart J 2024 · DOI 10.1093/eurheartj/ehae001 → https://doi.org/10.1093/eurheartj/ehae001 · verdict direkte Route 403, wayback 200 (20240130150250).
- Offene Kohorte: UK Biobank — https://www.ukbiobank.ac.uk/ · verdict direkt 403, wayback 200 · Access Registrierung/fee; NHANES — https://wwwn.cdc.gov/nchs/nhanes/ · 206 · offen.
- Feedbare y-Serie: **Long-Term ST** — https://physionet.org/content/ltstdb/ · 200 · offen.
- Kollokation: **pending** (ltstdb trägt Zeit, kein Geo; UK Biobank trägt Wohnadresse, aber y-La
  sitzt dort als aggregierte Diagnose).

**2 · metabolisch / Diabetes**
- Domänen: Luft, Ernährungsumfeld, gebaute Umwelt, Chemikalien.
- Schlüssel-Review: *Environmental risk factors of type 2 diabetes—an exposome approach*, Diabetologia 2022 · DOI 10.1007/s00125-021-05618-w → 200 (gefunden).
- Offene Kohorte: NHANES https://wwwn.cdc.gov/nchs/nhanes/ · 206 · offen; UK Biobank (Registrierung, direkt 403/wayback 200).
- Feedbare y-Serien: **ShanghaiT1DM/T2DM** (Home pending gemessen) · **D1NAMO** — https://zenodo.org/record/1421615 · 200 · CC BY-SA · **CGMacros** — https://physionet.org/content/cgmacros/ · 200 · offen · **AZT1D** — https://data.mendeley.com/datasets/gk9m674wcx · 200 · CC BY.
- Kollokation: **pending** (Zeit in allen vier; Ort in D1NAMO/CGMacros/AZT1D nicht gemessen).

**3 · respiratorisch (Asthma/COPD)**
- Domänen: Luft, Pollen, Grünraum, Arbeitsumfeld.
- Schlüssel-Review: *Identifying risk factors for COPD and adult-onset asthma: an umbrella review*, Eur Respir Rev 2023 · DOI 10.1183/16000617.0009-2023 → direkte Route 403, wayback 200 (20230506005346).
- Offene Kohorte: UK Biobank (Registrierung); NHANES 206.
- Feedbare y-Serie: **keine** (negativ, s. D).
- Kollokation: **nein** (keine individuelle Serie im Bestand).

**4 · renal**
- Domänen: Luft, Chemikalien, Wetter (Hitze), Wasser.
- Schlüssel-Review: *The general external exposome and the development or progression of chronic kidney disease: A systematic review and meta-analyses*, Environ Pollut 2024 · DOI 10.1016/j.envpol.2024.124509 → 200 (gefunden).
- Offene Kohorte: NHANES 206; UK Biobank (Registrierung).
- Feedbare y-Serie: **keine** (negativ, s. D).
- Kollokation: **nein**.

**5 · onkologisch**
- Domänen: Luft, Chemikalien, Arbeitsumfeld, Ernährungsumfeld, Geomagnetik (schwach).
- Schlüssel-Review: *Breast Cancer Exposomics*, Life 2024 · DOI 10.3390/life14030402 → direkte Route 403, wayback 200 (20240318132941); ergänzend *Molecular mechanisms of air pollution-induced carcinogenesis…*, Hum Genomics 2025, DOI 10.1186/s40246-025-00880-0 (nicht einzeln verifiziert).
- Offene Kohorte: UK Biobank (Registrierung, 403/wayback 200); All of Us — https://allofus.nih.gov/ · direkt 403, wayback 200 · Registrierung/DUA.
- Feedbare y-Serie: **keine individuelle** (negativ, s. D); TCGA trägt keine Exposom-Achse.
- Kollokation: **nein** (keine feedbare Individual-y).

**6 · Autoimmun (RA/Lupus/MS)**
- Domänen: Luft, Chemikalien, Ernährungsumfeld, Infekt-Trigger, Arbeitsumfeld.
- Schlüssel-Review: *Environment and systemic autoimmune rheumatic diseases: an overview and future directions*, Front Immunol 2024 · DOI 10.3389/fimmu.2024.1456145 → 206 (gefunden).
- Offene Kohorte: UK Biobank (Registrierung).
- Feedbare y-Serie: **BRAINTEASER MS** — https://zenodo.org/record/8083181 · 200 · Restricted Access (Antrag).
- Kollokation: **pending** (MS-Verlauf trägt Zeit; Ort restricted/ungemessen).

**7 · neurodegenerativ (ALS/Parkinson/Alzheimer)**
- Domänen: Luft, Chemikalien, Grünraum, Lärm, Arbeitsumfeld.
- Schlüssel-Review: *Associations of environmental factors with neurodegeneration: An exposome-wide Mendelian randomization investigation*, Ageing Res Rev 2024 · DOI 10.1016/j.arr.2024.102254 → 200 (gefunden).
- Offene Kohorte: UK Biobank (Registrierung).
- Feedbare y-Serien: **BRAINTEASER ALS** — https://zenodo.org/record/8083181 · 200 · Restricted Access · **mPower** — https://www.synapse.org/#!Synapse:syn4993293 · 200 · offen/DUA · **Parkinson-Smartwatch** — https://physionet.org/content/parkinsons-disease-smartwatch/ · 200 · offen · **PADS** — https://physionet.org/content/pads/ · 404 · pending.
- Kollokation: **pending** (Zeit in mPower/Smartwatch; Ort nicht im Datensatz).

**8 · infektiös / Sepsis**
- Domänen: Luft, Wetter (Hitze/Kälte), Chemikalien, Wasser.
- Schlüssel-Review: *Sepsis-related hospital admissions and ambient air pollution: a time series analysis in 6 Chinese cities*, BMC Public Health 2021 · DOI 10.1186/s12889-021-11220-x → 200 (gefunden).
- Offene Kohorte: MIMIC-IV — https://physionet.org/content/mimiciv/ · 200 · credentialed · eICU — https://physionet.org/content/eicu-crd/ · 200 · credentialed.
- Feedbare y-Serie: **Challenge-2019-Sepsis** — https://physionet.org/content/challenge-2019/ · 200 · offen.
- Kollokation: **nein** — MIMIC/eICU sind Zeit-verschoben und Ort-maskiert; die Sepsis-Serie ist strukturell nicht mit einer Exposom-x kollokierbar (gemessene Grenze, kein pending).

### B2. Psychosomatisch / psychiatrisch

**9 · Depression**
- Domänen: Luft, Lärm, Licht, Grünraum, Wetter/Klima, gebaute Umwelt, Geomagnetik (Hypothese).
- Schlüssel-Review: *Air pollution exposure and depression: A comprehensive updated systematic review and meta-analysis*, Environ Pollut 2022 · DOI 10.1016/j.envpol.2021.118245 → 200 (gefunden).
- Offene Kohorte: GLOBEM — https://physionet.org/content/globem/ · 200 · DUA/credentialed.
- Feedbare y-Serien: **LifeSnaps** (Home pending) · **GLOBEM** (DUA) · **Neurai-VN** (Home pending).
- Kollokation: **ja** für GLOBEM (vier US-Campus = Ort, Zeitreihe vorhanden; DUA).

**10 · Angst**
- Domänen: Luft, Lärm, Grünraum, Wetter/Klima, gebaute Umwelt.
- Schlüssel-Review: *Effects of ambient air pollution on psychological stress and anxiety disorder: a systematic review and meta-analysis*, Rev Environ Health 2021 · DOI 10.1515/reveh-2020-0125 → 202 (gefunden).
- Offene Kohorte: GLOBEM — 200 · DUA.
- Feedbare y-Serien: **GLOBEM** (DUA) · **Labbaf** (Home pending).
- Kollokation: **ja** für GLOBEM (Campus+Zeit).

**11 · somatoforme Störung / chronischer Schmerz**
- Domänen: Lärm, Luft, Chemikalien, gebaute Umwelt (Hypothese).
- Schlüssel-Review: **kein exposom-spezifischer Review gemessen** — `--pubmed "environmental exposure chronic pain somatoform review"` lieferte nur 2 Treffer ohne Exposom-Design (u. a. DOI 10.1177/1524838007303196, Trauma Violence Abuse 2007); `--openalex` lieferte keinen passenden Treffer. → pending.
- Offene Kohorte: UK Biobank (Registrierung).
- Feedbare y-Serie: **keine** (negativ, s. D).
- Kollokation: **nein**.

**12 · Schlaf**
- Domänen: Lärm, Licht, Luft, Wetter/Klima, Grünraum, gebaute Umwelt.
- Schlüssel-Review: *Neighborhood environments and sleep among children and adolescents: A systematic review*, Sleep Med Rev 2021 · DOI 10.1016/j.smrv.2021.101465 → 200 (gefunden); ergänzend *Environmental Determinants of Sleep and Hypertension*, Curr Hypertens Rep 2026, DOI 10.1007/s11906-026-01366-7 → 200.
- Offene Kohorte: SHHS/NSRR — https://sleepdata.org/datasets/shhs · direkt pending, wayback 200 · DUA/credentialed.
- Feedbare y-Serien: **GLOBEM** (DUA) · **D1NAMO** — https://zenodo.org/record/1421615 · 200 · CC BY-SA.
- Kollokation: **ja** für GLOBEM (Campus+Zeit); **pending** für D1NAMO (Ort ungemessen).

**13 · Suizidalität**
- Domänen: Luft, Wetter/Klima (Hitze), Licht.
- Schlüssel-Review: *Air Pollution (Particulate Matter) Exposure and Associations with Depression, Anxiety, Bipolar, Psychosis and Suicide Risk: A Systematic Review and Meta-Analysis*, Environ Health Perspect 2019 · DOI 10.1289/EHP4595 → direkte Route 403, wayback 200 (20191219042649); ergänzend *Climate change and suicide epidemiology*, Front Public Health 2024, DOI 10.3389/fpubh.2024.1463676 → 206 (gefunden).
- Offene Kohorte: CDC WONDER — https://wonder.cdc.gov/ · 200 · offen (aggregiert).
- Feedbare y-Serie: **keine individuelle** (negativ, s. D); CDC WONDER ist aggregiert.
- Kollokation: **nein**.

**14 · PTSD**
- Domänen: Wetter/Klima (Extremereignisse), Luft, Grünraum, Lärm.
- Schlüssel-Review: **kein exposom-spezifischer Review gemessen** — `--pubmed "environmental risk factors post-traumatic stress disorder review"` lieferte nur Katastrophen-/Deployment-Folgen, keine Exposom-Expositionsachse; nächstliegend *Effect of Extreme Weather Events on Mental Health*, Int J Environ Res Public Health 2020, DOI 10.3390/ijerph17228581 (nicht exposom-operationalisiert). → pending.
- Offene Kohorte: UK Biobank / Million-Veteran-Program (Registrierung, MVP nicht gemessen).
- Feedbare y-Serie: **keine** (negativ, s. D).
- Kollokation: **nein**.

**15 · Psychose**
- Domänen: Luft, Grünraum, gebaute Umwelt, Lärm, Wetter/Klima.
- Schlüssel-Review: *Impact of air pollution and climate change on mental health outcomes: an umbrella review of global evidence*, World Psychiatry 2024 · DOI 10.1002/wps.21219 → direkte Route 403, wayback 200 (20240522201333); Querschnittsbeleg auch in EHP 2019 (DOI 10.1289/EHP4595).
- Offene Kohorte: UK Biobank (Registrierung); All of Us (Registrierung).
- Feedbare y-Serie: **keine** (negativ, s. D).
- Kollokation: **nein** (nur Kohorten-Ebene, kein Individual-y im Bestand).

**16 · Sucht (SUD)**
- Domänen: Ernährungsumfeld, gebaute Umwelt, Arbeitsumfeld, Chemikalien.
- Schlüssel-Review: *Risky Substance Use Environments and Addiction: A New Frontier for Environmental Justice Research*, Int J Environ Res Public Health 2016 · DOI 10.3390/ijerph13060607 → direkte Route 403, wayback 200 (20180603030423).
- Offene Kohorte: NSDUH — https://www.samhsa.gov/data/data-we-collect/nsduh-national-survey-drug-use-and-health · direkt 403, wayback 200 · offen (Survey).
- Feedbare y-Serie: **keine** (negativ, s. D); NSDUH ist Survey/aggregiert.
- Kollokation: **nein**.

---

## C. Kompakttafel

| # | Klasse | Review-DOI (verdict) | y-Serie | x-Kern | Kollokation |
|---|---|---|---|---|---|
| 1 | kardiovaskulär | 10.1093/eurheartj/ehae001 (403/wb200) | Long-Term ST (200) | OpenAQ/Open-Meteo | pending |
| 2 | metabolisch/Diabetes | 10.1007/s00125-021-05618-w (200) | D1NAMO, CGMacros, AZT1D (200) | OpenAQ/Open-Meteo | pending |
| 3 | respiratorisch | 10.1183/16000617.0009-2023 (403/wb200) | — | OpenAQ | nein |
| 4 | renal | 10.1016/j.envpol.2024.124509 (200) | — | OpenAQ/NASA POWER | nein |
| 5 | onkologisch | 10.3390/life14030402 (403/wb200) | — | OpenAQ | nein |
| 6 | Autoimmun (RA/Lupus/MS) | 10.3389/fimmu.2024.1456145 (206) | BRAINTEASER MS (200, restricted) | OpenAQ | pending |
| 7 | neurodegenerativ | 10.1016/j.arr.2024.102254 (200) | BRAINTEASER ALS, mPower, Smartwatch (200) | OpenAQ/OMNIWeb | pending |
| 8 | infektiös/Sepsis | 10.1186/s12889-021-11220-x (200) | Challenge-2019-Sepsis (200) | OpenAQ/Open-Meteo | nein |
| 9 | Depression | 10.1016/j.envpol.2021.118245 (200) | GLOBEM (DUA), LifeSnaps, Neurai-VN | OpenAQ/Open-Meteo | ja (GLOBEM) |
| 10 | Angst | 10.1515/reveh-2020-0125 (202) | GLOBEM (DUA), Labbaf | OpenAQ | ja (GLOBEM) |
| 11 | somatoform/Schmerz | pending (kein Review) | — | OpenAQ/Lärm | nein |
| 12 | Schlaf | 10.1016/j.smrv.2021.101465 (200) | GLOBEM (DUA), D1NAMO (200), SHHS (DUA) | OpenAQ/Open-Meteo | ja (GLOBEM) |
| 13 | Suizidalität | 10.1289/EHP4595 (403/wb200) | — | OpenAQ/Open-Meteo | nein |
| 14 | PTSD | pending (kein Review) | — | Open-Meteo | nein |
| 15 | Psychose | 10.1002/wps.21219 (403/wb200) | — | OpenAQ | nein |
| 16 | Sucht | 10.3390/ijerph13060607 (403/wb200) | — | OSM/Ernährungsumfeld | nein |

---

## D. Gemessene Negativa (Klasse/Domäne ohne offene individuelle Serie)

- **respiratorisch (Asthma/COPD) individuell** — Review gemessen, aber kein feedbares
  Individual-y im y-Bestand.
- **renal individuell** — desgleichen.
- **onkologisch individuell** — UK Biobank/All of Us tragen Diagnose auf Kohorten-Ebene;
  kein Individual-y.
- **Autoimmun RA/Lupus** — nur MS ist über BRAINTEASER feedbar (restricted); RA/Lupus
  ohne offene individuelle Serie.
- **infektiös/Sepsis** — MIMIC-IV/eICU: Zeit verschoben, Ort maskiert → strukturell
  nicht kollokierbar (Challenge-2019-Sepsis ebenso). Gemessene Grenze, nicht pending.
- **Suizidalität individuell** — nur aggregiert (CDC WONDER 200).
- **PTSD individuell** — kein exposom-operationalisiertes Individual-y.
- **Psychose individuell** — nur Kohorten-Ebene.
- **Sucht (SUD) individuell** — nur Survey/aggregiert (NSDUH, direkt 403/wayback 200).
- **Domänen-Negativa**: Wasser/Lärm/Licht/Pollen/Grünraum/gebaute Umwelt/Ernährungsumfeld/
  Arbeitsumfeld/Chemikalien tragen noch keine gemessene offene x-Home in dieser Matrix
  (`pending`, s. A) — sie sind registriert, nicht als 0 gesetzt.

---

## E. Mycelium-Lauf

Jede Matrix-Zeile ist eine TE-Messung. Der generische Pfad trägt sie ohne
Pre-Select — es wird **nicht** nach Hypothese vorsortiert; die Matrix fährt jede
Klasse gegen jede Domäne, die eine offene x-Serie hat.

**Gewählter Pfad:** `te_pair_probe` (`tools/measure/src/bin/te_pair_probe.rs`)
für die reine Paar-Messung y↔x: zwei Kanal-Dateien, **eine Zahl je Zeile**,
`--a <y.csv> --b <x.csv> --name-a <klasse> --name-b <domäne>` — Ausgabe TE(a→b),
Surrogat-Schwelle (mean + 2σ, phasen-randomisiert) und die Richtung. Für die
volle Feld-Query (Konditionierung, Kanal-Liste, FDR über die Matrix) trägt
`field_te_query` (`tools/measure/src/bin/field_te_query.rs`) dieselbe Zeile über
`matrix <label> rect|full|upper` + `--descriptor <pfad>.te` (Kanal-Dateien, Form
wie `phi/pipeline/descriptors/solar_seconds_matrix.te`).

**Feed-Mechanik:** pro Matrix-Zeile genau eine Zahl je Kanal-Datei und Zeitstempel
(y-Achse = Outcome-Serie, x-Achse = Exposom-Serie), beides auf dieselbe
Zeitbasis/`bin` gebracht; eine Zeile → ein `te_pair_probe`-Lauf → eine Zeile im
Ergebnis-Register. Das ist der generische Lauf: keine Klasse wird ausgewählt,
keine Domäne ausgeschlossen.

**Offener CI-Dispatch:** der Lauf gehört in CI (kein lokaler Heavy-Compute,
`AGENTS.md`): ein Workflow, der die Matrix aus den feedbaren y-Serien + den
offenen x-Serien baut und `te_pair_probe`/`field_te_query` je Zeile fährt. Der
Dispatch steht offen (`pending`): Workflow-Datei noch nicht angelegt,
`gh workflow run <workflow>` noch nicht möglich. Nächster Schritt: Workflow-YAML
unter `.github/workflows/` + Descriptor je X-Klasse, dann `gh workflow run`.

**Kollokations-Gate im Lauf:** eine Zeile ohne gemeinsamen Ort (Kollokation
`nein`/`pending`) fährt nur die zeitliche Kopplung; der Lauf markiert sie als
`time-only` und behauptet **keine** räumliche Kopplung — die fehlende Ortsachse
wird als `pending` getragen, nie als 0.

---

## F. Zählung

- **Krankheitsklassen:** 16 (8 somatisch + 8 psychosomatisch/psychiatrisch).
- **Zeilen mit offener Kollokation:** 7 — `ja` 3 (Depression, Angst, Schlaf über
  GLOBEM) + `pending` 4 (kardiovaskulär, metabolisch, Autoimmun, neurodegenerativ).
- **Negativa:** 9 — respiratorisch, renal, onkologisch, Autoimmun-RA/Lupus,
  infektiös/Sepsis (strukturell), Suizidalität, PTSD, Psychose, Sucht.
- **Offene x-Kern-Serien gemessen:** 4/4 (OpenAQ 206, Open-Meteo 200, NASA POWER
  206, OMNIWeb 200); übrige Domänen-x `pending`.
- **Feedbare y-Serien im Bestand:** 14 (BRAINTEASER ALS/MS, ShanghaiT1DM/T2DM,
  D1NAMO, CGMacros, AZT1D, PADS, mPower, Long-Term ST, Challenge-2019-Sepsis,
  LifeSnaps, Neurai-VN, Labbaf, GLOBEM, NeuroVista) — davon mit gemessenem offenem
  Home: D1NAMO, CGMacros, AZT1D, Long-Term ST, Challenge-2019-Sepsis, mPower,
  Smartwatch, GLOBEM; BRAINTEASER restricted; PADS 404 (pending).

Alles gemessen 2026-10-04 via `./bin/archive_search --pubmed|--openalex|--datacite|--zenodo|--verdict`.

---

## G. Historische TE (Verläufe)

Die TE-Maschine ist zeit-agnostisch: sie nimmt zwei zeitgestempelte Serien und
misst die Richtung — **egal ob live oder archiviert**. Damit ist jede
**historische, zeitgestempelte Kohorte** (GLOBEM 2018–2021, LifeSnaps 2021,
Shanghai-CGM, Labbaf 2020, NeuroVista) bereits ein **Verlauf**, der gegen die
x-Serien desselben Zeitfensters **retrospektiv** gefahren werden kann. Kein
Live-Recording nötig; die Kollokation ist das gemeinsame Zeitfenster (und, wo
vorhanden, der Ort).

Konsequenz für den Lauf: die Matrix ist **rückwärts fahrbar** — für jedes
historische y-Fenster wird die passende x-Serie (Open-Meteo-Archive,
OpenAQ-History, OMNIWeb ab 1963) auf dieselbe Zeitbasis gebracht und die Zeile
durch `te_pair_probe`/`field_te_query` geschickt. Der Mycelium trägt jeden Lauf
generisch, ohne Klassen-Auswahl.

---

## H. Harte Nachmessung (2026-10-04)

Ein zweiter, härterer Taucher-Satz hat mehrere frühere Zeilen **widerlegt** (A=A; beide Linien benannt):

- **BRAINTEASER** — früher „offen (Zenodo 8083181)". Gemessen: **beide Records restricted**
  (`files: restricted`, `/files` HTTP 403) — 8083181 (v0, retrospektiv) und 14857741 (v3,
  +prospektiv 86 ALS, median 270 d). Der frühere „offen"-Wert verwechselte die öffentliche
  Metadaten-Seite (200) mit offenem Datei-Zugang. Offen nur die Ontology (`10.5281/zenodo.12789731`).
- **LifeSnaps** — früher „offen". Gemessen: `access_right: restricted` (Antrag/DUA).
- **Neurai-VN** — früher „offen CC-BY". Gemessen: **restricted + DUA**.
- **GLOBEM** — credentialed DUA **und kein Ort** (bewusst nur feature-level; Sensordaten einbehalten) → x-Kollokation schwach.
- **CGMacros** — Zeit um ±N Tage verschoben → **keine** Kollokation.
- **PADS** — Zeit auf 0 geshiftet → **keine** Kollokation.
- **Long-Term ST** — nur relative Zeit, kein Datum/Ort → **keine** Kollokation.
- **Suizidalität** — bleibt belegtes Negativ (gefundene EMA-Studien IRB-gated/Protokoll).
- **Onkologie** — nur TCGA/SEER (open, aber kein individuelles PRO/Wearable) → Negativ für die Serie.

**Neue offene Treffer (früher als Negativ geführt) — Kollokation gemessen 2026-10-10:**

| Klasse | Serie | Quelle/DOI | Dauer | n | Besonderheit | Ort im Datensatz? | x-Kollokation |
|---|---|---|---|---|---|---|---|
| respiratorisch | **TOLIFE** | `10.5281/zenodo.16642439` | 12 Mon. | 74 | **Umweltsensorik im Paket** (T/Feuchte/Luft/Licht/Schall) | nur Land (GERMANY/SPAIN) | **`pending`** |
| respiratorisch | **AAMOS-00** | `10.7488/ds/3775` (→ `hdl:10283/4761`) | 12 Mon. | 22 | **Wetter/Pollen/Luftqualität im Paket** | UK-admin-Region | **`pending`** |
| Autoimmun | **Wearable+PRO Fatigue** | `10.5281/zenodo.8018238` | 1 Mon. | 183 | SLE/Sjögren, Fitbit+PRO | keiner (nur „United States") | **`pending`** |
| Sucht | **ADARP** | `10.5281/zenodo.6640290` | 14 d | 11 | E4 HR/EDA + EMA, AUD | keiner (kein GPS-Kanal) | **`pending`** |
| Psychose | **CrossCheck** | Kaggle `dartweichen/crosscheck` | 1 J | ~17–63 | passiv + EMA, Relapse | keine Koordinate (nur GPS-abgeleitete Features) | **`pending`** |
| Affekt | **Labbaf** | `10.7280/D1WH6T` | ⌀7,8 Mon. | 21 | CC0, SoCal, PPG/IMU+EMA | keine Koordinate (Venue **request-only**, Region „Southern California") | **`pending`** |

**Konsequenz (korrigiert, gemessen 2026-10-10):** Kein einziger dieser sechs offenen
Datensätze trägt eine **im Datensatz gemessene Koordinate** — alle tragen eine absolute
Zeitachse, aber der Ort ist Land/Region bzw. request-only. Die frühere Zeile „x-Kollokation
`ja`" (Labbaf) war **nicht gedeckt** und ist ein Riss, kein Treffer. Die kollokierbare offene
Basis ist damit **keine** der genannten Zeilen in vorliegender Form: für jede müsste ein
Repräsentativpunkt **extern** gesetzt werden (Annahme, keine Messung) — erst dann ist eine
Ort-Kollokation gegen OpenAQ/Open-Meteo/NASA POWER belegbar.

**Nachtrag je Zeile — gemessen 2026-10-10 (Sensory; `archive_search`/`grind-flash`):**
- **TOLIFE** (`zenodo.org/api/records/16642439` 200/10 621 B; `README.md` 200/22 428 B;
  `index.zip` 200/963 225 B, 758 Einträge, 5 450 942 B entpackt): Zeitachse **absolut**
  (`Timestamp` ISO8601 UTC, 12-h-Aggregat 2/Tag, Mai 2024–Mai 2025), Layout
  `index/{GERMANY,SPAIN}/<KIT-ID>/{EI,SII,SQI,MI,BRI,PRVI,PEI,clinical_indices,exacerbations}.csv`,
  Variablen Temperatur/Feuchte/AQI/Lux (Schall dimensionlos in `SII`); Ort = **nur Land**.
- **AAMOS-00** (`doi.org/10.7488/ds/3775` 200 → final `handle/10283/4761`; der Handle
  `10283/3775` ist ein **anderer** Record — Nanobubbles, Riss): Zeitachse **relativ**
  (Integer-Tagesindex 1…182, Studien-Anker 2021-06-24–2022-06-02), 22 UK-Teilnehmer, 2 054
  Patient-Tage; täglich Temperatur/Druck/Feuchte/Wind/AQI/CO/NO/NO₂/O₃/SO₂/PM/NH₃ + Pollen;
  Ort = **UK-admin-Region** (`patient_info.csv`).
- **Wearable+PRO Fatigue** (`zenodo.org/api/records/8018238` 200/8 596 B; CC-BY-4.0):
  `day_activity_features.csv` 6 706 171 B, `daily_surveys_…` 1 333 345 B; Fitbit-Tagesfeatures
  je `user_id`+`date` (lokal + UTC-Offset), absolute Zeit; **keine** Orts-Spalte (nur „United States").
- **ADARP** (`zenodo.org/api/records/6640290` 200/8 644 B; CC-BY-4.0): `Sensor Data.zip`
  1 456 305 106 B (E4 ACC/BVP/EDA/HR/IBI/TEMP, Session `YYMMDD-HHMMSS`, Epoch), Phone-Survey
  `.xlsm`; absolute Zeit; **kein** GPS-/Location-Kanal.
- **CrossCheck** (Kaggle-Seite 200/11 696 B; Lizenz „Unknown"; Daily 14 996 021 B, Hourly
  122 728 019 B): `day` absolut (YYYYMMDD / ISO); Ort nur **abgeleitete** GPS-Features
  (`loc_*`, `distance_sum`), **keine** Koordinate.
- **Labbaf** (`doi.org/10.7280/D1WH6T` 200/278 108 B; Dryad; File-Stream 403 — request/API):
  Epoch-ms absolut; Ort nur `venue_name` — Doku „only available upon request"; **keine**
  Koordinate/Station, feinste Angabe Region „Southern California".
**Externe Deckung je Datensatz — gemessen 2026-10-10 (Sensory; Taucher + am Baum verifiziert):**
Der Repräsentativpunkt ist für **5/6** Datensätze **extern gedeckt** — die Quelle dokumentiert den
Studien-/Erhebungsort (Site/Stadt), nicht nur Land/Region. Die Koordinate ist im Beleg **nicht** als
lat/lon genannt; der dokumentierte Ort ist der Kandidat-Repräsentativpunkt (**site-level**, nicht
Teilnehmer-Koordinate).

| Datensatz | externer Studienort (Beleg, am Baum verifiziert) | Repräsentativpunkt |
|---|---|---|
| TOLIFE | University of Pisa (IT, Koordinator + klinische Site) · LungenClinic Großhansdorf (DE, Recruiting-Site) · Consorci Mar Parc de Salut de Barcelona/IMIM (ES, Rekrutierung) — `tolife-project.eu/partners` (200) | **ja (Site)** — Datensatz-Index nur GERMANY/SPAIN → Großhansdorf (DE) + Barcelona (ES) |
| AAMOS-00 | Usher Institute, University of Edinburgh (UK) — DataCite `10.7488/ds/3775` (Description: „…at the Usher Institute at the University of Edinburgh…", Mobistudy/Malmö University); die Handle-Seite `datashare.ed.ac.uk/handle/10283/4761` liefert jetzt **403** (Bot), die API trägt die Site | **ja (Site)** — Edinburgh |
| Wearable+PRO Fatigue | keiner — remote über Evidation Health, „in the United States" (`zenodo.org/records/8018238`; Paper `10.3389/fdgth.2023.1099456` §2.1) | **nein** |
| ADARP | Washington State University (WSU), Pullman, WA, USA — `zenodo.org/records/6640290` (Description: „…at Washington State University (WSU), Pullman, WA, USA") | **ja (Site)** — Pullman |
| CrossCheck | Zucker Hillside Hospital, Long Island, NY, USA — Paper `10.1145/2971648.2971740` („a large psychiatric hospital in Long Island, NY"; IRB Zucker Hillside) | **ja (Site)** — Long Island |
| Labbaf | University of California, Irvine, CA, USA — Dryad `10.7280/D1WH6T` („Research facility: University of California, Irvine"; THRIVE Lab) | **ja (Site)** — Irvine |

**Riss (0-Kanon):** der Repräsentativpunkt ist die **dokumentierte Studien-Site**, nicht eine im
Datensatz gemessene Teilnehmer-Koordinate. Die Kollokation gegen die x-Kern-Serien wird damit
**site-level** — sie behauptet keine individuelle Orts-Kopplung; die fehlende Information bleibt
`pending` (nie 0). Wearable+PRO Fatigue trägt **keinen** Site → bleibt `pending`.
- **Braucht:** (a) die Bestätigung, dass eine dokumentierte Studien-Site als Repräsentativpunkt
  trägt (Methodik — Rat), sonst bleibt die Kollokation eine Annahme; dann (b) den Mycelium-Te-Paar-
  Feed je Zeile (Workflow-YAML + Descriptor, `gh workflow run`). Registrierung (`phi/sources.φ`)
  erst mit bestätigtem Punkt (Mountain-Feder).

---

## I. iEEG.org-Kanal-Matrix (SUDEP-Pfad, gemessen 2026-10-04)

Der iEEG.org-Katalog ist hinter der Konto-/T&C-Wand (Gast: „You can't search as a guest.").
Der SUDEP-Pfad Anfall→RR ist dennoch unmittelbar baubar — über **offene OpenNeuro-Assets**:

| Datensatz | Zugang | iEEG | EKG | Lokalisation | Sampling |
|---|---|---|---|---|---|
| **HUP ds004100** (UPenn) | **public CC0** | SEEG ictal+interiktal | **EKG1/EKG2** (gemessen) | `space-fsaverage` (Riss: NEMAR nennt MNI/ICBM152) | 512/1024 Hz |
| AJILE12 (DANDI 000055) | public | ECoG, 12 Pat., 1280 h | `pending` | pending | 500 Hz |
| MSDR/CSR SUDEP | Antrag | EMU-EEG multimodal | pending | pending | — |

Negativa: Omni-iEEG verwirft EKG explizit; ds003029 (Fragility) kein EKG; SWEC-ETHZ kein EKG belegt.
User Agreement (https://main.ieeg.org/?q=user/register): Registrierung mit Institution/Projekt,
personengebunden; Pflicht-Zitat U24NS06930.
