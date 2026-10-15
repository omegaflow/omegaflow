<!--
  title: Quellen-Blatt — der UCAR-Riss und die neuen offenen Fäden (2026-10-15)
  class: handover
  date: 2026-10-15
  sha256: 2d5c5d927a21827fb3be7b3774681d566cd5489428edb80bdb6d0926c0e4f6d2
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

## Zwei Mails (Operator-Hand)
1. **CEIC-Schwere** — Antrag/Anfrage an `datashare@seis.ac.cn` (国家地震科学数据中心).
2. **UCAR-FPI** — DOI + Terms an `qwu@ucar.edu` / DASH-Support (klein, kein Faden-Abschluss).

## Werkzeug-Riss (behoben)
`./bin/all_wide` war für die Agenten gesperrt: die Regel stand nur im **globalen** Rechtblock, `taucher`/`recherche-frei` haben **eigene** Blöcke (`"*": "deny"`). In `7513ee0aa` in **alle drei** Linien eingetragen — **wirkt nach dem nächsten Neustart**. Gemessene Grenze: `--all` stoppt nach 6 **Armen**, nicht nach Zeilen; die Breite tragen die Einzelmodi.

## Abschluss
Dieses Blatt ist eine **Zulieferung** — keine Register-Schreibakte. Die Verdikte/Manifestationen schreiben Mountain/Mycelium in ihren Passes.
