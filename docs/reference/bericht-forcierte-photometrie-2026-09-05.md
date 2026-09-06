# Forcierte Photometrie — Messung 2026-09-05

Sitzung: Nadel-V-FP-Front. Alle Angaben sind per HTTP gemessen (curl, Rust-only,
kein Python). VPN-Egress Proton NL. Kein Commit — Bericht an den Operator.

## Befund in Kürze

1. **Der Durchbruch ist gemessen:** Fink/LSST stellt **anonym** einen
   dedizierten Forced-Photometry-Endpunkt bereit — `POST https://api.lsst.fink-portal.org/api/v1/fp`
   mit `{"diaObjectId":"…"}`. HTTP 200, Zeilen mit `r:diaForcedSourceId`,
   `r:scienceFlux`, `r:psfFlux`, `r:band`, `r:midpointMjdTai`. Mehrband-Geschichte
   pro Objekt (g/r/i/u/z/y), dicht über die Visits, inklusive Nicht-Detektions-
   Epochen. Fink/LSST ist damit die **eine beste anonyme FP-Fläche** für die Maschine.
2. ZTF: anonyme FP existiert **nicht** (gemessen). Die echte ZTF-FP
   (`ztfweb.ipac.caltech.edu/cgi-bin/requestForcedPhotometry.cgi`) antwortet
   HTTP 401 (IPAC-Konto, Auftrags-/E-Mail-Verfahren). IRSA `nph_light_curves`
   liefert nur Detektions-Photometrie (keine FP, keine Upper Limits).
   Die nutzbare ZTF-FP für die Maschine ist `Lasair-ZTF /api/object/` → Feld
   `forcedphot` (gemessen, live, mit dem vorhandenen `LASAIR_TOKEN`).
3. Lasair-LSST (`lasair.lsst.ac.uk`, `api.lasair.lsst.ac.uk`) ist **heute vom
   Egress aus unerreichbar** (DNS 192.41.122.98 löst auf, TCP verbindet nicht,
   HTTP 000). Der `psfFlux`-Pfad ist damit heute unverifizierbar.
4. Beide Account-Anfragen sind vorbereitet (Dateien `antares-konto-2026-09-05.md`,
   `fink-konto-2026-09-05.md`). Gemessen: ANTARES-Supportadresse steht in der
   ausgelieferten `config.json` (`antares@noirlab.edu`); Fink-Stream-Credentials
   laufen NICHT über `/joining/` (das ist nur die Kollaborations-Mailingliste),
   sondern über ein eigenes Formular (`forms.gle/2td4jysT4e9pkf889`).

---

## 1. ZTF — Forcierte Photometrie

### 1.1 IRSA `nph_light_curves` — Detektions-Photometrie, keine FP (gemessen)

Endpunkt (wie der `ztf_lightcurves_compiler` ihn nutzt):

```
https://irsa.ipac.caltech.edu/cgi-bin/ZTF/nph_light_curves?POS=CIRCLE+<ra>+<dec>+<radius>&FORMAT=csv
```

Messung 2026-09-05 (gespeicherte Cone-Stichprobe, 12074 Datenzeilen, 78 Objekte):
Spalten `oid,expid,hjd,mjd,mag,magerr,catflags,filtercode,ra,dec,chi,sharp,filefracday,field,ccdid,qid,limitmag,magzp,magzprms,clrcoeff,clrcounc,exptime,airmass,programid`.
Filterverteilung `zr 6005, zg 3787, zi 2282`. **0 Zeilen ohne mag** — es gibt
keine Upper-Limit- und keine Forced-Zeilen. Antwort ist die PSF-Detektions-
Photometrie der ZTF-Objekte (eine Epoche, ein Band). Keine FP, keine beliebigen
Positionen, nur katalogisierte Objekte. Der Endpunkt beantwortet damit die Frage
des Auftrags messbar mit *nein*.

Beispielzeile (gemessen):
```
675103300002930,44835315,2458202.8577286243,58202.353159699996,18.6746521,0.0466535166,0,zg,209.99005510000001,29.997625800000002,…
```

### 1.2 Offizieller ZTF-FP-Dienst — nicht anonym (gemessen)

```
curl https://ztfweb.ipac.caltech.edu/cgi-bin/requestForcedPhotometry.cgi
→ HTTP 401
```

Der ZTF-Forced-Photometry-Dienst (Differenzbild-FP über Aufträge, Ergebnisse per
E-Mail) verlangt ein IPAC-Konto. Kein anonymer Maschinenweg. (`IRSA_USER`/`IRSA_PASS`
liegen in `.secrets.local` vor, aber das Verfahren ist Auftrags-gebunden und
interaktiv — keine REST-FP.)

### 1.3 Lasair-ZTF `forcedphot` — die echte ZTF-FP der Maschine (gemessen, live)

Endpunkt (POST, `Authorization: Token <LASAIR_TOKEN>`):
```
https://lasair-ztf.lsst.ac.uk/api/object/
Form: objectId=<ZTF-ID>
```

Gemessen live am 2026-09-05, HTTP 200, `ZTF26aappdrm`:
- 53 `candidates` (Alerts), 30 `forcedphot`-Zeilen = **10 je Band**
  (fid 1 = g, 2 = r, 3 = i), JD 2461107.78 → 2461124.80
  (2026-02-22 → 2026-03-24).

Beispiel `forcedphot`-Zeile (gemessen) — der negative Fluss zeigt, dass es eine
echte FP-Messung ohne Detektions-Selektion ist:
```json
{"objectid":"ZTF26aappdrm","jd":2461107.7783218,"ranr":148.9504493,"decnr":2.3895383,
 "fid":1,"forcediffimflux":-82.53524017333984,"forcediffimfluxunc":51.35590362548828,
 "magzpsci":26.327999114990234}
```

Tiefe über gespeicherte Samples vom selben Tag:
- `ZTF20acyxple`: 43 `forcedphot`-Zeilen, JD 2460646.96–2460709.87 (2024-12-01 →
  2025-02-02); Alert-Geschichte seit 2023-03-12.
- `ZTF24aabnhvr`: 403 `candidates` (401 in r), 6 `forcedphot`-Zeilen.
- `ZTF21aafavlk`: 20 `candidates`, 0 `forcedphot` (Objekte ohne FP-Lauf).

Kontingent: Nutzer-Token 100 Aufrufe/Stunde (früher gemessen HTTP 429 „wait an
hour"). Abdeckung: FP existiert für Objekte, die im ZTF-Alertstrom liegen — nicht
für beliebige Positionen. Der `forcedphot`-Pfad ist genau der, den `lsst_anomaly_probe
--ztf` bereits liest (Schwelle 3σ, Fluss > 0; sub-gate bleibt absent).

---

## 2. LSST — Forcierte Photometrie

### 2.1 Fink/LSST `/api/v1/fp` — anonyme FP, der Durchbruch (gemessen)

Endpunkt (anonym, POST; GET mit Query-Strings geht ebenso):
```
POST https://api.lsst.fink-portal.org/api/v1/fp
{"diaObjectId":"313998569858662581","output-format":"json"}
```

Messung 2026-09-05: HTTP 200, 1033 Zeilen für Objekt 313998569858662581.
Felder (15, gemessen): `r:diaForcedSourceId`, `r:diaObjectId`, `r:band`,
`r:midpointMjdTai`, `r:scienceFlux`, `r:scienceFluxErr`, `r:psfFlux`,
`r:psfFluxErr`, `r:ra`, `r:dec`, `r:visit`, `r:timeProcessedMjdTai`,
`r:timeWithdrawnMjdTai`, `r:salt`, `r:detector`.

Bandverteilung (Objekt 313998569858662581): g 197, i 385, r 188, u 47, y 20, z 196.
Zeitspanne MJD 61090.18 → 61205.98 = 2026-02-19 → 2026-06-14.

Beispielzeile (gemessen):
```json
{"r:band":"r","r:dec":2.5208047616,"r:detector":62,"r:diaForcedSourceId":170538716508455749,
 "r:diaObjectId":313998569858662581,"r:midpointMjdTai":61205.9837394019,"r:psfFlux":-12200.908,
 "r:psfFluxErr":1779.0398,"r:ra":148.8745712297,"r:salt":"581","r:scienceFlux":46041.09,
 "r:scienceFluxErr":1791.4626,"r:timeProcessedMjdTai":61205.9852013255,"r:visit":2026061400484}
```

Weitere Messungen gleicher Sitzung (anonym, HTTP 200):
- Objekt 313853517569720388: 962 FP-Zeilen (g 180, i 188, r 142, u 6 …).
- Objekt 170028511006818402 (Simbad „GiP"): 215 FP-Zeilen (g 68, i 39, r 44,
  z 42); davon 117 mit negativem `r:psfFlux` — Nicht-Detektions-Epochen im
  Differenzbild, wie sie nur FP erzeugt.

Definition, aus Fink/LSST-eigener Dokumentation (gemessen,
`doc.lsst.fink-broker.org/services/api/forced_photometry/`):
> „'Forced' photometry means a measurement made at a fixed coordinate in an
> image, regardless of whether an above-threshold region was detected there in
> that particular image. For all objects, starting from the second detection,
> forced photometry measurements for previous detections are included."

Fluss-Semantik, aus der Daten-Dokumentation (gemessen,
`doc.lsst.fink-broker.org/data/photometry/`):
- `r:scienceFlux` = erzwungene PSF-Photometrie am Science-Bild an der festen
  Objektposition (Kette `prvDiaForcedSources.scienceFlux`).
- `r:psfFlux` = PSF-Photometrie am Differenzbild an derselben Position.
- `r:diaForcedSourceId` = Kennung der FP-Messung selbst.

Grenze (gemessen aus der Definition): FP beginnt bei der **zweiten Detektion**
eines Objekts. Beliebig-Positions-FP gibt es anonym nicht — wohl aber die dichte
g/r/i/u/z/y-Geschichte genau der Objektmenge, die Nadel V durchmustert
(bereits alertete diaObjects).

### 2.2 Fink/LSST `/api/v1/sources` — anonym, die bisher genutzte Fläche (gemessen)

```
POST https://api.lsst.fink-portal.org/api/v1/sources
{"diaObjectId":"313998569858662581"}
```

HTTP 200, 1045 Zeilen (Objekt 1), Bänder g 205, r 205, i 373, u 43, y 12, z 207,
MJD 61057.31–61205.98 (2026-01-17 → 2026-06-14). `r:scienceFlux`/`r:scienceFluxErr`
pro Zeile; zusätzlich die Alert-Detektionsfelder (`r:snr`, `r:isNegative`,
`r:pixelFlags_*`, …), aber **kein** `r:diaForcedSourceId` — das sind die
diaSource-Detektionen (inkl. Negativ-Ereignissen), nicht die FP-Tabelle.
Beispielzeile (gemessen):
`{"r:band":"r","r:midpointMjdTai":61205.9837394019,"r:scienceFlux":45351.812,"r:scienceFluxErr":1790.4822,"r:psfFlux":-11818.149,"r:snr":6.68}`

### 2.3 Lasair-LSST — heute unerreichbar (gemessen)

```
lasair.lsst.ac.uk        → DNS 192.41.122.98, TCP verbindet nicht, HTTP 000
api.lasair.lsst.ac.uk    → DNS 192.41.122.98, TCP verbindet nicht, HTTP 000
lasair-ztf.lsst.ac.uk    → HTTP 200 (erreichbar)
```

Der `diaSourcesList`/`psfFlux`-Pfad (`/api/cone/`, `/api/object/` mit
`LASAIR_LSST_TOKEN`) kann heute nicht verifiziert werden — die Ursache ist nicht
DNS, sondern die Verbindungsebene. Der Pfad bleibt registered `pending`, nicht
verneint.

---

## 3. Verdikt — die eine beste anonyme FP-Fläche

**`POST https://api.lsst.fink-portal.org/api/v1/fp` (Fink/LSST, anonym, HTTP 200
gemessen 2026-09-05)** ist die eine beste anonyme FP-Fläche der Maschine: dichte
Mehrband-Geschichte pro Objekt (inkl. Nicht-Detektion), keine Anmeldung, REST,
Rust-konsumierbar (curl).

ZTF bleibt zweigeteilt: anonym keine FP; maschinennutzbar nur über
`Lasair-ZTF /api/object/` → `forcedphot` mit `LASAIR_TOKEN` (vorhanden, bereits
in `lsst_anomaly_probe --ztf` verdrahtet).

Was gemessen **nicht** existiert: anonyme FP für beliebige Himmelspositionen
(ZTF wie LSST). FP setzt ein bereits bekanntes Objekt voraus. Das ist für den
Nadel-V-Zweck (Cone über bereits detektierte Objekte) die richtige Fläche.

---

## 4. Account-Anfragen (Messgrundlage)

Alle Einzelheiten + wörtliche Vorlagen: `antares-konto-2026-09-05.md`,
`fink-konto-2026-09-05.md`.

Gemessene Eckdaten hier:
- ANTARES `config.json` (HTTP 200): `ANTARES_SUPPORT_EMAIL = antares@noirlab.edu`,
  API-Basis `https://api.antares.noirlab.edu/v1`, Issue-Tracker
  `https://gitlab.com/nsf-noirlab/csdc/antares/antares/-/issues`.
  `https://antares.noirlab.edu/support` → HTTP 200 (SPA-Shell, Inhalt
  clientgerendert). Im ausgelieferten `app.js` kein einziges Vorkommen von
  „forced" — das ANTARES-Frontend trägt keine FP-Oberfläche; die E-Mail fragt die
  FP-Lage deshalb ausdrücklich als Frage an, nicht als Zusage.
- Fink `https://fink-broker.org/joining/` → HTTP 200: Google-Formular
  `forms.gle/CmvH8vsyyv4AUTpy8` = **Kollaborations-Mitgliedschaft** (Mailingliste,
  General Meetings; nur academic staff/students mit Fink-Mitglied). Gewährt
  **keine** Kafka-Zugangsdaten.
- Fink Kafka/Stream-Credentials: eigenes Formular `forms.gle/2td4jysT4e9pkf889`
  („Fink services subscription") → Credentials per E-Mail → `finkctl auth register`
  (fink-client v12, Python 3.9+). Topics unter `https://lsst.fink-portal.org/schemas`.
  **Gemessene Grenze für die Maschine:** fink-client ist Python-only; der
  Kafka-Stream ist für die Rust-Maschine nicht direkt konsumierbar. Die anonyme
  REST-Fläche (Abschnitt 2) bleibt der Maschinenweg.
