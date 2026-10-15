<!--
  title: Quellen-Blatt — der UCAR-Riss und die neuen offenen Fäden (2026-10-15)
  class: handover
  date: 2026-10-15
  sha256: da43716ddf9b9e3c87ec8ef75712ae1ea6057f7a41c96997803268573c08d967
  status: live
  see-also: docs/handover/handover-2026-10-14-river-folge212.md, docs/handover/handover-2026-10-14-mountain-folge346.md, docs/surveys/survey-2026-10-14-gewebe-faeden.md
-->
# Quellen-Blatt — der UCAR-Riss und die neuen offenen Fäden (2026-10-15)

Dieses Blatt trägt nur Offenes: einen **Riss** (zwei Linien, eine Zeile) und die **neuen offenen Quellen** aus der weiten Rechercherunde. Register-Schreibakte bleiben bei ihren Linien (Mountain die Verdikte, Mycelium die Manifestation).

## Riss — UCAR-FPI Athabasca-Instrumentenkoordinate

- **River ist gemessen (river-213), Mountain ist stale.**
  - **River (`river-folge212.md:102,230`):** `on earth 54.60282 -113.64442 693` — **AUGO II / AUGSO** (Athabasca University Geophysical Observatory, Dunkelstandort). Quelle `https://augo.athabascau.ca/location/index.php` (Wayback `20221219132851`), gestützt durch die FPI-eigene Publikation (Zhang et al. 2025, JASTP 268). Am Baum: `phi/sources.φ:414` trägt dieselbe Zeile für die ERG-SC-Athabasca-VLF-Quelle — beide Zeugen auf einem Ort. `alt` = orthometrisch (MSL/EGM96), SRTM30m am Punkt 696.0 m.
  - **Mountain (`mountain-folge346.md:198-211`, `folge347` gleichlautend):** *„die FPI-eigene Athabasca-Instrumentenkoordinate ist ungemessen"* — **stale**.
- **Verdikt:** der Baum gewinnt (River-213). **Kein Operator-Akt nötig** — keine UCAR-Mail für die Koordinate.
- **Offen:** (1) **Mountain** — die Register-Zeile schreiben (die zwei `field`-Zeilen stehen) und die stale „ungemessen"-Zeile korrigieren; (2) **Mycelium** — die Manifestation ziehen.
- **UCAR bleibt nur für die DOI/Terms-Frage** (Lizenz gemessen: UCAR/DASH-Nutzungssatz → `local-only`; kein FPI-DOI benannt) — eine kurze Mail, kein Faden-Abschluss.

## Neue offene Quellen (weite Runde, 2026-10-15)

Jede Zeile gemessen (`archive_search`, Endpunkt-Verdikt, Lizenz) — Routen für **URL/`origin`/`compiler`** (Mycelium), Verdikte (`terms`/`field`/`ttl`) **Mountain**.

| Quelle | Route | Lizenz | Key |
|---|---|---|---|
| **BGI Absolutgravimetrie** | `https://bgi.obs-mip.fr/data-products/gravity-databases/absolute-gravity-data/` (HTTP 200) | „free of charge without any restriction" (Nennung, kein Weiterverkauf) | nein |
| **IGETS (GFZ ISDC)** | `http://isdc.gfz-potsdam.de/igets-data-base/` — Einzelstationen mit DOI `10.5880/igets.*` | **CC-BY-4.0** | Einzel-DOIs keyfrei; Rohdaten sftp-Account (im Register) |
| **GSRAS (offen)** | `http://www.gsras.ru/ftp/` (Apache-Autoindex; Teleseismic_bulletin 2006–2024, Teleseismic_Catalog 1993–2024, verifizierter Download 716 KB) | keine formale Open-Lizenz (nur Copyright) | nein |
| **WDC-B Moskau (Spiegel)** | `http://www.wdcb.ru/sep/seismology/cat_OBN.html` — Obninsk-Katalog 1993–2024 | offen (kein Key) | nein |
| **ISC (Aggregation)** | `http://www.isc.ac.uk/fdsnws/event/1/query`; Agentur **MOS** = GSRAS/Obninsk | offen | nein |
| **CMONOC GNSS** | Zenodo `records/7325306`, `7316100`, `7316321`, `5794666`, `21271035`, `7798617` (Velocity-Feld), `20192157` (TEC) | **CC-BY-4.0** | nein |
| **CEIC-Seismikkatalog** | Zenodo `10.5281/zenodo.8084659` (CEDC catalog) | CC-BY-4.0 | nein |
| **CEIC-Schwere (Schwestern)** | Zenodo `7112529`/`7855090` (CC-BY-4.0), `6365137` (restricted) | CC-BY-4.0 / restricted | nein |
| **CB (China National Seismic Network)** | FDSN-Metadaten `service.earthscope.org/fdsnws/station/1/query?network=CB` (`10.7914/SN/CB`) | offen (Metadaten) | nein |
| **NGL-Station ARDA** | `https://geodesy.unr.edu/NGLStationPages/stations/ARDA.sta` | NBMG/UNR Policy (Nennung erbeten) | nein |

**Belegte Risse (nicht glätten):**
- **`ARDA`** = **GNSS-Stationscode** (Ardahan, Türkei) — **kein** Gravimetrie-/Seismik-Knoten; die Register-Zeile ist eine Kategorienverwechslung.
- **„EGGN"** = Fehlname (die realen Akronyme: ECGN/UEGN/EGGP); die Schwere-Daten liegen bei IGETS/BGI. EQUIP-G bleibt `pending` (Daten erst nach 2029, Zugang per E-Mail-Registrierung).
- **re3data CEIC** `r3d100011154` führt „dataAccess=open", die Baum-Messung zeigt Login/订单 — **Riss**.

## Korrektur (die drei Zeilen — Fehlbeschriftung, nicht Riss)
Ein echter Riss = zwei **wahre** Zeilen, die sich nicht einigen (der bleibt). Diese drei sind **falsch beschriftet** — die Korrektur nennt beide Enden.

1. **ARDA** (Mountain): die Gravimetrie-/Seismik-Zeile ist eine **Kategorienverwechslung** — `ARDA` ist der **GNSS-Stationscode** der NGL/MAGNET-Station (Ardahan, Türkei; `https://geodesy.unr.edu/NGLStationPages/stations/ARDA.sta`, offen, kein Key). Umklassifizieren: **GNSS-Zeuge**, nicht Schwere/Seismik.
2. **„EGGN"** (Mountain): **Fehlname**. Die Zeile zeigt auf **EQUIP-G** (`pending`; Daten erst nach 2029; Zugang per E-Mail-Registrierung `https://equip-g.eu`) + die realen Schwestern **IGETS/GFZ** (`http://isdc.gfz-potsdam.de/igets-data-base/`, **CC-BY-4.0**) und **BGI** (frei ohne Einschränkung).
3. **re3data `r3d100011154`** (Mountain schreibt die gemessene Sperre; Operator-Mail an re3data): unsere Zeile trägt **Login + 离线获取/订单**; re3data behauptet `dataAccess=open`.

## `alt` — keine neue Größe, aber eine Deklaration
`on earth <alt>` trägt **eine** Haus-Skala: **orthometrisch (MSL/EGM96)**. Das ist der offene Mycelium-316-Punkt — **schließen**, nicht umbauen.

- **Kein neuer Eintrag, keine neue Spalte.** Die Deklaration steht **einmal** (Vertrag / `docs/concepts/archivar-mathematikerin.md`).
- **Regel gegen stillen Mix:** liefert eine Quelle nur **ellipsoidische** Höhe (oder ein anderes Datum), wird das **benannt** — ein Wert **ohne Datum** ist `pending`. AUGO ist entschieden: **693 orthometrisch** (SRTM30m am Punkt = 696.0 m stützt die orthometrische Skala; die ellipsoidische Hypothese verlangte ≈710 m) — benannt, nicht geglättet.
- **Echter Riss bleibt:** wo zwei Skalen **beide gemessen** vorliegen und nicht konvergieren, stehen **beide** (nie gemittelt).

## Mail an re3data (Operator-Hand)
> **To:** `info@re3data.org` — **Subject:** Correction — r3d100011154 (China Earthquake Data Center): `dataAccess` measured gated
>
> Hello,
>
> Your record `r3d100011154` (China Earthquake Data Center) lists Database and Data access as *open*. Measured 2026-10-14: the gravity and GNSS datasets at `data.earthquake.cn` require login (用户登录) and an order / offline request (数据共享方式: 离线获取 / 下订单); only the station metadata is free without an account. The record's own description already notes "It is only open in the Seismological Bureau."
>
> Suggest correcting the access fields, or noting the measured condition. Our measurements: `https://data.earthquake.cn/datashare/report.shtml?PAGEID=normalordercheck&dtid=…` (login wall), `https://data.earthquake.cn/D18000/index.html` (重力测量数据).
>
> Thanks for maintaining the index.


## Zwei Mails (Operator-Hand)
1. **CEIC-Schwere** — Antrag/Anfrage an `datashare@seis.ac.cn` (国家地震科学数据中心).
2. **UCAR-FPI** — DOI + Terms an `qwu@ucar.edu` / DASH-Support (klein, kein Faden-Abschluss).

## Werkzeug-Riss (behoben)
`./bin/all_wide` war für die Agenten gesperrt: die Regel stand nur im **globalen** Rechtblock, `taucher`/`recherche-frei` haben **eigene** Blöcke (`"*": "deny"`). In `7513ee0aa` in **alle drei** Linien eingetragen — **wirkt nach dem nächsten Neustart**. Gemessene Grenze: `--all` stoppt nach 6 **Armen**, nicht nach Zeilen; die Breite tragen die Einzelmodi.

## Abschluss
Dieses Blatt ist eine **Zulieferung** — keine Register-Schreibakte. Die Verdikte/Manifestationen schreiben Mountain/Mycelium in ihren Passes.
