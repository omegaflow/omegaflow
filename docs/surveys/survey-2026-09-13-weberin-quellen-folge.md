<!--
  title: Survey — Die Weberin: offene Quellen-Routen, Folge (Stand 2026-09-13)
  class: survey
  date: 2026-09-13
  sha256: c3147b277093a8edd565f3be376100f5d2b3b623df3291644eeb64e607db92a1
  status: live
  see-also: docs/concepts/die-weberin.md docs/surveys/survey-2026-09-13-weberin-quellen.md
-->

# Die Weberin — offene Quellen-Routen, Folge (Stand 2026-09-13)

Träger: `docs/concepts/die-weberin.md` (Weberin-Punkte).

Zweiter Durchgang: Vier GLM-Free-Taucher (read-only) haben die Rohausgaben
des Recherche-Werkzeugs `archive_search` zu den im ersten Snapshot
(`survey-2026-09-13-weberin-quellen.md`) noch ungelösten Fällen ausgewertet.
Der Werkzeuglauf (`--verdict`/`--brave`/`--wayback`/`--crossref`/`--arxiv`,
Kaskade direkt → Proton → Wayback) datiert 2026-09-13; jede hier als `live`
gemeldete Route ist zusätzlich vom DeepSeek-Agenten direkt nachgemessen. Der
Free-Agent hat nur analysiert, nie geschrieben.

## Auflösungen (vorher ungelöst → jetzt gemessen)

### IGETS-Bodengravimeter
- `https://isdc.gfz.de/igets-data-base/data-access` — **200**, Text nennt
  Passwort/Registrierung → Zeitreihen L1/L2/L3 = **blocked account** (bestätigt).
- `https://igets.u-strasbg.fr/tr005.php` — direkt **HTTP 0** (lebt nur im
  Wayback-Snapshot 2022) → **not-published** (Korrektur: der erste Snapshot
  nannte 200).

### CTBTO/vDEC-Infraschall
- `https://www.ctbto.org/specials/vdec/` — **403**.
- Neuer Einstiegspfad benannt: `https://www.ctbto.org/resources/for-researchers-experts/vdec`
  — direkt **403**, Wayback 200 → Rohwellenform bleibt **blocked account**
  (Antrag + Vertrag); die Restfrage „heutiger vDEC-Einstieg" ist beantwortet.

### Ocean Networks Canada (Hydrophon)
- `https://data.oceannetworks.ca/api/deployments?method=get&token=…` —
  Token-pflichtig; der Discovery-Service trägt lat/lon/depth/deviceCode →
  **blocked key** (bestätigt). Keine anonyme Datenroute.

### GIC
- `https://zenodo.org/api/records/10594301` (Alberta 2023) — direkt **200**
  (Korrektur: der erste Snapshot nannte 000/Timeout → jetzt **live**).
- `https://transmission.bpa.gov/business/operations/gic/gic.txt` — **200**
  (87 307 B, 5-min TSV, 11 Substationen) → **live**.
- Die Kombination **kontinuierlich + Stationskoordinaten** bleibt
  **not-published**.

### WWLLN-Blitze
- `http://wwlln.net/cgi-bin/output.cgi?format=json&type=stations` — **404**,
  kein Wayback-Snapshot → Stations-/Echtzeit-JSON = **not-published**.
- `https://www.earthdata.nasa.gov/data/catalog/ghrc-daac-wwllnmth-1` —
  **200**: WWLLN Monthly Thunder Hour als registrierter GHRC-Datensatz,
  laut Listing ohne Zugriffsbeschränkung → **live**.
- `https://data.ucar.edu/en/dataset/wwlln-lightning-data` — **200**:
  RELAMPAGO-WWLLN, tägliches kommasepariertes ASCII → **live**.
- Die Thunder-Hour-netCDFs (`wwlln.net/climate/th_yr/data/`) bleiben **live**.

### SuperDARN
- `https://sdc-serv.usask.ca/ascii-download` und `https://superdarn.ca/ascii-download`
  — **200** (67 298 B): FITACF-Konvertierung nach CSV / tab-delimited TXT /
  JSON, wählbar nach Radar/Zeitraum/Beam → **live** (Korrektur: der erste
  Snapshot führte FITACF nur als Globus-`blocked account`; eine offene
  ASCII-Route existiert). Radar-Positionen (`superdarn.ca/radar-info`) live.

### HF-Radar HFRNet
- `https://hfrnet-tds.ucsd.edu/thredds/catalog.html` — **HTTP 0** (bestätigt);
  `https://hfradar.ioos.us/erddap/griddap/index.json` — **400**.
- Gitter-Produkt offen auf ERDDAP: `https://upwell.pfeg.noaa.gov/erddap/griddap/ucsdHfrW1_Lon0360.html`
  und `https://coastwatch.pfeg.noaa.gov/erddap/griddap/ucsdHfrW2.html` —
  beide **200** → **live** (die Gitter-Lücke ist über die ERDDAP-Hosts
  geschlossen, nicht über das THREDDS). Benanntes Migrationsziel laut
  Fundtext: NDBC-THREDDS.

### BGC-Argo / Argovis
- `https://argovis-api.colorado.edu/bgcargoplus` — **200** (6 894 239 B)
  → **live** (Korrektur: der erste Snapshot nannte die Box leer / `pending`).
- Doku `https://argovis-api.colorado.edu/docs/` (Swagger).

### Telescope Array
- `https://zenodo.org/records/8427755` — **200** (62 404 B; Korrektur:
  der erste Snapshot nannte 504) → Einzelereignis-Route **live**.
  Vollkatalog bleibt **not-published**.

### Super-Kamiokande
- `https://zenodo.org/records/8401262` — **200** (104 594 B): Data Release
  „Atmospheric neutrino oscillation analysis … SK I–V" → **live**
  (Korrektur: der erste Snapshot nannte nur die Solar-Seite, Verdikt
  `declined`). Ob per-Event-S²-Richtungen trägt, ist ungemessen.

### JUNO
- Kein offener JUNO-Datensatz gefunden (das Zenodo-Release im Treffer ist
  Daya Bay) → **not-published** (bestätigt). Referenz: Nature
  s41586-026-10538-z, arXiv 2511.14593.

### Fink / ALeRCE
- `https://api.lsst.fink-portal.org/api/v1/objects` — **400** (der Host
  antwortet; Korrektur: der erste Snapshot nannte 000/Hänger). `/api/v1/objects`
  ist der dokumentierte Endpoint (nicht nur `conesearch`); Wayback leer.
- ALeRCE `https://api.alerce.online/alerts/v1/objects/` — **HTTP 0**
  (bestätigt), kein Wayback-Snapshot.

### Gaia Alerts
- `https://gsaweb.ast.cam.ac.uk/alerts/alertsindex` — **200** (10 601 266 B);
  die Gaia-DR3-Doku nennt sie als maschinenlesbaren Weg (Epochen-Photometrie,
  Klassifikationen; Lichtkurven-CSV je Alert) → **live** (Korrektur: der
  erste Snapshot nannte Gaia Alerts `declined` als maschinenlesbare Quelle).

### TNS
- `https://www.wis-tns.org/system/files/tns_public_objects/tns_public_objects.csv.zip`
  — **403** anonym (User-Agent-Gate) → **blocked** (bestätigt). Wayback
  trägt einen Snapshot der ZIP vom 2024-03-27 (historisch). Laut TNS-Doku
  zwei tägliche CSVs unter `/system/files/tns_public_objects/`.

## Korrektur-Bilanz gegen den ersten Snapshot

`live` wurden nach der zweiten Messung: GIC/Zenodo, WWLLN/GHRC-Earthdata +
UCAR, SuperDARN-ASCII, HFRNet-ERDDAP-Gitter, Argovis `/bgcargoplus`,
Telescope-Array-Einzelereignis, Super-K-Zenodo-Release, Gaia-Alerts-Index.
`not-published` präzisiert: WWLLN-Stations-JSON, JUNO. `blocked` bestätigt:
IGETS-Zeitreihen, vDEC, ONC, TNS. Status korrigiert: IGETS `tr005.php`
(200 → direkt 0), GIC/Zenodo (pending → live), Fink (000 → 400).

## Gesamtbild

Der zweite Durchgang hat sieben zuvor `pending`/`declined` geführte Fälle in
gemessene `live`-Routen überführt (GIC-Zenodo, WWLLN-GHRC/UCAR,
SuperDARN-ASCII, HFRNet-ERDDAP, Argovis-BGC+, TA-Einzelereignis,
Super-K-Release, Gaia-Alerts-Index) und vier `blocked`-Grenzen bestätigt.
Unaufgelöst bleiben: IGETS-Zeitreihen, vDEC-Roh, ONC-Daten, TNS anonym
(account/key), JUNO-Release und der TA-Vollkatalog (`not-published`). Kein
Wert ist fabriziert; jeder ungemessene Punkt ist als `pending` benannt.
