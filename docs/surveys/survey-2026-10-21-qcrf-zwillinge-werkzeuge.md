<!--
  title: Survey — QCRF/KCRF-Zwillinge und CRF-Messwerkzeuge: webweite, unscharfe Suche (2026-10-21)
  class: survey
  date: 2026-10-21
  sha256: 33887f36398babca7869f2dc8aecb8191b5c734887f06d8094b66258d01ac7ee
  status: live
  see-also: docs/surveys/survey-2026-10-21-qband-quasar-katalog.md tools/measure/src/bin/qband_catalog_probe.rs tools/measure/src/bin/qcrf_twin_probe.rs
-->
# Survey — QCRF/KCRF-Zwillinge und CRF-Messwerkzeuge

**Zweck.** Die Q-Band-Survey (`survey-2026-10-21-qband-quasar-katalog.md`)
hat Lanyi+2010 und QCAL-1 als maschinenlesbare 43-GHz-Kataloge belegt und
den QCRF-2021-Zenodo-Record als PDF vermerkt. Hier wird nachgezogen: Gibt es
**maschinenlesbare Zwillinge oder Kopien** von QCRF 2021 und KCRF 2024 bei IVS,
VieVS, USNO, CDS/VizieR, Zenodo, GitHub oder DataCite — und welche
**Messwerkzeuge** stehen für CRF-Astrometrie offen? Jede Zeile stammt aus einer
HTTP-, Header-, PDF- oder Tabellenmessung vom 2026-10-21 und ist mit
`qcrf_twin_probe` reproduzierbar.

**Befund vorab.** Es gibt **keinen** maschinenlesbaren Zwilling von QCRF 2021
oder KCRF 2024. Beide Zenodo-Records enthalten **genau eine Datei: das PDF**.
QCRF 2021 ist keine Tabelle, sondern eine **einseitige A1-Posterfolie**; die
Vorgabe „98-zeilige Executive Summary" ließ sich nicht reproduzieren (gemessen:
1 Seite, 162 extrahierte Textzeilen, 0 Quellenzeilen). Die einzigen offenen
43-GHz-Positionskataloge bleiben **Lanyi+2010 (131 Quellen)** und
**QCAL-1 (637 Fluss / 33 Positionen)**. KCRF 2024 hat mit **CRF-K-2025**
(arXiv 2605.27629, 1.317 Quellen) einen Nachfolger auf Papier, aber ebenfalls
keinen belegten Tabellenzugang.

| Kandidat | Träger | Quellen | Maschinenlesbar? | Lizenz/Zugang |
|---|---|---:|---|---|
| **QCRF 2021** | Zenodo 10834955 | 428 | **nein** — 1 PDF, 1 Seite | Record: CC-BY-4.0; PDF-Notes: „All Rights Reserved" |
| **KCRF 2024** | Zenodo 10893171 | 1.315 | **nein** — 1 PDF, 1 Seite | **kein Lizenzfeld** im Record; Notes: „All Rights Reserved" |
| **CRF-K-2025** | arXiv 2605.27629 | 1.317 | **nur Paper** — keine VizieR-Tabelle gefunden | arXiv-Abstract offen; Katalogzugang unbelegt |
| **Lanyi+2010 Q** | CDS/VizieR J/AJ/139/1695/table3 | 131 | **ja** — CSV/TXT, 11 Spalten | öffentlich, zitierpflichtig |
| **QCAL-1 Fluss** | CDS/VizieR J/AJ/144/150/table3 | 637 | **ja** — CSV, 13 Spalten | öffentlich, zitierpflichtig |
| **QCAL-1 Positionen** | CDS/VizieR J/AJ/144/150/table4 | 33 | **ja** — CSV, 10 Spalten | öffentlich, zitierpflichtig |

## Suchweg und Suchbefund

Gefahren wurden `archive_search` mit `--zenodo`, `--datacite`, `--github`,
`--mwmbl`, `--arxiv`, `--verdict`; die VizieR-Suche lief direkt über
TAP/ADQL gegen `TAP_SCHEMA.tables`, weil der `--vizier`-Kanal nur
`description LIKE` matcht. `--all` lieferte am 2026-10-21 **keine Ausgabe**;
`--sniff` erwartet eine Datei, keinen Suchbegriff.

- `--zenodo 'QCRF 2021 VLBI celestial reference frame'` → QCRF-Record und
  ein zweiter QCRF-Poster-Record (13330088); beide nur PDF.
- `--zenodo 'KCRF 2024 K-band VLBI celestial reference frame'` → KCRF-Record,
  weitere K-Band-Poster, aber kein Katalogdatensatz.
- `--datacite 'QCRF VLBI celestial reference frame 43 GHz'` → **absent**.
- `--github 'QCRF VLBI'`, `'QCRF 428'`, `'qband crf'` → **absent**;
  `--github 'VieVS VLBI'` → nur Software (TUW-VieVS).
- `--mwmbl 'Q-band 43 GHz celestial reference frame VLBA data'` → **absent**;
  `'USNO celestial reference frame Q band'` → USNO-CRF-Seite.
- `--arxiv 'Q-band celestial reference frame 43 GHz VLBA'` → Lanyi/Charlot
  2010 und K-Band-Papers, **kein QCRF-2021-Paper**.
- `--verdict 'QCRF 43 GHz'` → Stufe 1 und 2 „pending — no response",
  Wayback „HTTP 200, CDX register carries no snapshot".

**VizieR-TAP (gemessen).** `description LIKE '%Q-band%'` liefert als einzigen
Quasar-Katalog `J/AJ/139/1695/table3` (Lanyi+2010) plus einen VLA-Stern-
entstehungskatalog (`J/MNRAS/504/338`). `LIKE '%Celestial Reference Frame%'`
liefert ICRF2, die K-Band-**Imaging**-Tabellen `J/AJ/165/139/*` und
Gaia-CRF-Variabilität — **kein QCRF 2021, kein KCRF 2024, kein CRF-K-2025**.
`LIKE '%CRF-K%'` → 0 Tabellen.

## Gemessene Zwillinge und Zugänge

Alle Zeilen aus `qcrf_twin_probe` (curl, HTTP-Status, Bytes, sha256 des
empfangenen Körpers, Formaterkennung). `absent` heißt: kein HTTP-Antwortkörper
und kein Cache; es wird nichts geschätzt.

| Label | URL | HTTP | Bytes | sha256 | Format/Befund |
|---|---|---:|---:|---|---|
| qcrf-zenodo-record | `https://zenodo.org/api/records/10834955` | 200 | 8.690 | `2e2471a7…a2a409` | JSON, **1 Datei**, Lizenz `cc-by-4.0` |
| qcrf-pdf | `…/10834955/files/Krasna_etal_IVSGM2024_QCRF.pdf/content` | 200 | **2.011.258** | `0e73afaf…b0d1b2` | PDF, **1 Seite**, 162 Textzeilen, **0 Quellenzeilen** |
| qcrf-2024-poster-record | `https://zenodo.org/api/records/13330088` | 200 | 6.337 | `bfb7d856…8792fc4` | JSON, 1 Datei, CC-BY-4.0 |
| qcrf-2024-poster-pdf | `…/13330088/files/Krasna_IAUGA2024_QCRF_poster.pdf/content` | 200 | 2.063.284 | `cf95d783…453a412` | PDF, 1 Seite, 223 Zeilen, 0 Quellenzeilen |
| kcrf-zenodo-record | `https://zenodo.org/api/records/10893171` | 200 | 8.703 | `e815c36e…2feaec4` | JSON, **1 Datei**, **Lizenz absent** |
| kcrf-pdf | `…/10893171/files/K-CRF_IVS-GM-2024_v3.pdf/content` | 200 | **5.534.683** | `5171d7a4…5998b2` | PDF, **1 Seite**, 141 Textzeilen, **0 Quellenzeilen** |
| kcrf-2023-arxiv | `https://arxiv.org/abs/2306.09747v1` | 200 | 45.075 | `8fca6dad…d2fe9e` | HTML, Paper; Q-Band-Marker 0 |
| crf-k-2025-arxiv | `https://arxiv.org/abs/2605.27629v1` | 200 | 46.438 | `787922be…856da2` | HTML, Paper; Q-Band-Marker 0 |
| vizier-lanyi-q | TAP-CSV `J/AJ/139/1695/table3` | 200 | 12.021 | `6486bbca…a8776fb` | CSV, **131 Zeilen**, 11 Spalten |
| vizier-qcal1-flux | TAP-CSV `J/AJ/144/150/table3` | 200 | 60.275 | `b10c21f3…83c3fd1` | CSV, **637 Zeilen**, 13 Spalten |
| vizier-qcal1-pos | TAP-CSV `J/AJ/144/150/table4` | 200 | 2.844 | `5a172e56…778b9a5cc` | CSV, **33 Zeilen**, 10 Spalten |
| lanyi-readme | `https://cdsarc.cds.unistra.fr/ftp/cats/J/AJ/139/1695/ReadMe` | 200 | 6.426 | `a825617d…217d5da1` | Text; deklariert table3.dat 131 |
| qcal1-readme | `…/J/AJ/144/150/ReadMe` | 200 | 8.510 | `e2bf7609…1d4365b` | Text; deklariert 637/33 |
| usno-crf-global-solutions | `https://crf.usno.navy.mil/global-solutions-crf` | 200 | 162.557 | `bf06b103…a1703c0` | HTML; **q-band 0, 43ghz 0** |
| usno-sx-source-positions | `…/RORFD/Quarterly/current/USNO_source_positions.iers` | 200 | 733.025 | `ebdf0426…c2b47ee` | Text, 5.821 Zeilen, S/X |
| bvid-database | `https://bvid.astrophy.u-bordeaux.fr/database.html` | 200 | 96.689 | `b71bb1a3…dd787a65` | HTML; Q-Band 6, 43 GHz 2, RV146/148/150 |
| ivs-crf-products | `https://ivscc.gsfc.nasa.gov/products-data/products.html` | 200 | 19.251 | `cb08e8c5…032aa8` | HTML; CRF-Produkt, keine Q-Tabelle |
| ivs-bkg-crf-dir | `https://ivs.bkg.bund.de/data_dir/vlbi/ivsproducts/crf/` | **absent** | 0 | `absent` | curl exit 28, Connection refused |
| cddis-crf-dir | `https://cddis.nasa.gov/archive/vlbi/ivsproducts/crf/` | 200 | 10.934 | `1c0648d2…d957ab` | HTML = **Earthdata-Login**, kein Verzeichnis |
| vievs-wiki | `https://vievswiki.geo.tuwien.ac.at/` | 200 | 8.140 | `147c525d…a4b37065` | HTML, Einstieg |
| vievs-github | `https://github.com/TUW-VieVS/VLBI` | 200 | 299.553 | `9499056c…d175082bb` | HTML, GPL-3.0 |
| calc-solve-gsfc | `https://gemini.gsfc.nasa.gov/solve/` | **absent** | 0 | `absent` | curl exit 28, Timeout |
| aips | `https://www.aips.nrao.edu/` | 200 | 38.588 | `ef6da663…fcfa0c7` | HTML, Downloadseite |
| difmap-github | `https://github.com/sabourke/difmap` | 200 | 286.406 | `a20a123f…c7619df` | HTML, Fork ohne SPDX |
| casa | `https://casa.nrao.edu/` | 200 | 15.768 | `8ec413e9…03854456` | HTML, „open source", Bitbucket |
| hops | `https://www.haystack.mit.edu/hops/` | 200 | 71.320 | `9071d9f9…93220ed` | HTML, Postprocessing |

Die sha256-Werte der GitHub- und CDDIS-HTML-Seiten sind **dynamisch** und
ändern sich zwischen Abrufen; Bytes und Status sind stabiler. Für die
Datenrecords (Zenodo-JSON, PDFs, TAP-CSV, ReadMes) sind Bytes und sha256
reproduzierbar.

**Lizenz-Fallstricke (gemessen, nicht interpretiert).** Der QCRF-Record trägt
im Zenodo-Metadatenfeld `license.id = cc-by-4.0`, die PDF-Notes sagen aber
„Copyright ©2024. All Rights Reserved." Der KCRF-Record hat **gar kein**
Lizenzfeld und ebenfalls „All Rights Reserved". Wer die PDFs weiterverwendet,
sollte diese Spannung nicht als CC-BY-Freigabe lesen. Die CDS/VizieR-Tabellen
haben kein gemessenes SPDX-Feld; belegbar ist „öffentlich, zitierpflichtig".

## Kein IVS-/USNO-/VieVS-Zwilling

- **USNO:** Die Global-Solutions-Seite beschreibt ausdrücklich nur S/X- und
  K-Band-Lösungen („ICRF3 S/X catalog … CRF catalogs at K and X/Ka bands").
  Der offene Download `USNO_source_positions.iers` (733.025 B, 5.821 Zeilen)
  ist eine S/X-Positionsliste, **kein Q-Band-Katalog**. Auf der Seite kommen
  „Q-band" und „43 GHz" **null** mal vor.
- **IVS:** Das CRF-Produkt besteht aus ICRF-Updates und verweist auf BKG,
  CDDIS und OPAR. BKG antwortet am 2026-10-21 nicht (Connection refused);
  CDDIS liefert statt eines Verzeichnisses die Earthdata-Login-Seite. Ein
  offener QCRF-Download ist damit **nicht belegt** — der Zugang ist keyed.
- **VieVS:** Die Software ist offen (GitHub, GPL-3.0), aber der QCRF-Record
  enthält keine VieVS-Lösungsdatei. Das Paper nennt „made with VieVS"; ein
  veröffentlichter `VIE-Q-2021`-Katalog wurde nicht gefunden.
- **BVID:** Die Bilddatenbank führt die Q-Band-Sessions **RV146, RV148,
  RV150** und markiert 43 GHz. Sie liefert **Bilder**, keine astrometrischen
  Positionen; ein Positionskatalog ist dort nicht gemessen.
- **GitHub/DataCite:** Keine QCRF-/KCRF-Katalogkopie. DataCite: absent.
  GitHub liefert nur Software (VieVS, Difmap, Struktur-Analyse).

## Messwerkzeuge für CRF-Astrometrie

| Werkzeug | URL | Lizenz (gemessen) | Zugang | Wofür |
|---|---|---|---|---|
| **VieVS** | `https://vievswiki.geo.tuwien.ac.at/`; `https://github.com/TUW-VieVS/VLBI` | **GPL-3.0** (GitHub-API/Repo) | offen, Quellcode | VLBI-Globallösung, CRF/TRF, EOP; QCRF und KCRF wurden damit gerechnet |
| **Calc/Solve** | `https://gemini.gsfc.nasa.gov/solve/` | nicht gemessen | **Seite timeout** am 2026-10-21 | geodätische VLBI-Kleinste-Quadrate-Lösung, CRF; im K-Band-Paper als unabhängige Zweitlösung genannt |
| **AIPS** | `https://www.aips.nrao.edu/` | auf der Seite nicht ausgewiesen | freier Download | Kalibrierung, Fringe-Fitting, Imaging, VLBI-Standardpaket |
| **Difmap** | `https://github.com/sabourke/difmap` | Fork ohne SPDX; Originalseite 404 | Quellcode-Mirror | Modellfitting/Imaging von VLBI-Sichtbarkeiten, Quellenstruktur |
| **CASA** | `https://casa.nrao.edu/` | „open source"; Quellcode auf Bitbucket, kein SPDX auf der Seite | frei | generische Radiointerferometrie, Kalibrierung/Imaging; weniger geodätisch als VieVS/Solve |
| **PIMA** | **kein kanonischer Endpunkt gefunden** | nicht gemessen | im HOPS-Paket | Import/Konvertierung von Mark4/Mark5-Daten in die HOPS-Kette |
| **HOPS** | `https://www.haystack.mit.edu/hops/` | auf der Seite nicht ausgewiesen | frei | Korrelations-Postprocessing, `fourfit`, Fringe-Qualität; PIMA gehört zum Umfeld |

**Ehrlich benannt.** Calc/Solve ist nicht als „offen verfügbar" belegt, weil
die Projektseite am Messtag nicht antwortete. PIMA hat keinen eigenen
öffentlichen Endpunkt; die Zuordnung „im HOPS-Paket" ist Papierwissen, keine
HTTP-Messung. Difmap ist nur über einen Fork greifbar; die originale
Caltech-Seite liefert 404, und der Fork trägt keine SPDX-Kennung. AIPS, CASA
und HOPS sind frei herunterladbar, aber ihre Lizenztexte wurden auf den
gemessenen Seiten nicht ausgewiesen.

## Was die Zwillinge leisten — und was nicht

- **Lanyi+2010** ist der einzige offene 43-GHz-**Positions**katalog: 131
  Quellen, 11 Spalten (`JName, BName, S43, RAJ2000, e_RAJ2000, DEJ2000,
  e_DEJ2000, C_pos, Nep, Nobs`), Median 0,24/0,35 mas formal. Er ist aber
  **nicht** QCRF 2021: andere Epochen (zehn VLBA-Sessions über fünf Jahre),
  andere Quellenliste, keine 428 Quellen.
- **QCAL-1** liefert 637 Flussdichten und nur 33 KVN-Positionen (16–20 mas);
  als Frame-Katalog unbrauchbar, als Kompaktheits-Zensus brauchbar.
- **QCRF 2021** und **KCRF 2024** sind auf Zenodo nur **Poster/Abstracts**.
  Die 428 bzw. 1.315 Quellen sind im Papiertext genannt, aber nicht als
  Zeilen geliefert. Ein „Download des Katalogs" ist damit nicht belegt.
- **CRF-K-2025** (arXiv 2605.27629) nennt 1.317 Quellen und 60/104 µas, ist
  aber ebenfalls Paper; in VizieR wurde keine Tabelle gefunden.

## Reproduktion

```sh
cd /home/johannes/projects/omegaflow
cargo build --release -p omegaflow-measure --bin qcrf_twin_probe
./target/release/qcrf_twin_probe --cache /tmp/opencode/qcrf-twin-probe
```

Optionen: `--cache <dir>`, `--offline` (nur Cache; meldet dann ehrlich
`absent`, wenn nichts liegt), `--timeout <sek>` (Default 40), `--only <substr>`.
Das Werkzeug lädt alle 26 Ziele, misst Status/Bytes/sha256/Content-Type,
erkennt PDF/JSON/CSV/HTML/Text, zählt bei PDFs Seiten und `pdftotext`-Zeilen
sowie J-Namen-Zeilen, bei JSON die Dateizahl und Lizenz, bei CSV Zeilen und
Spalten und bei HTML die Q-Band-Marker.

Lauf vom 2026-10-21 (Auszug):

```text
[qcrf-zenodo-record] ... status 200 | bytes 8690 | format json | files 1 | license cc-by-4.0
[qcrf-pdf] ... status 200 | bytes 2011258 | format pdf | pages 1 | text-lines 162 | source-rows 0
[kcrf-zenodo-record] ... status 200 | bytes 8703 | format json | files 1 | license absent
[kcrf-pdf] ... status 200 | bytes 5534683 | format pdf | pages 1 | text-lines 141 | source-rows 0
[vizier-lanyi-q] ... rows 131 | columns 11
[vizier-qcal1-flux] ... rows 637 | columns 13
[vizier-qcal1-pos] ... rows 33 | columns 10
[usno-crf-global-solutions] ... q-band 0 | 43ghz 0
[ivs-bkg-crf-dir] absent | status none | no body and no cache
[calc-solve-gsfc] absent | status none | no body and no cache
summary | targets 26 | answered 24 | absent 2
```

## Offene Punkte

1. **QCRF-2021-Tabelle fehlt.** Der Record enthält nur das Poster. Ein
   Katalog könnte bei den Autoren (TU Wien/USNO) auf Anfrage liegen; das ist
   nicht gemessen.
2. **KCRF/CRF-K-2025-Tabelle fehlt.** arXiv-Paper ohne belegten Tabellen-
   zugang; VizieR hat (Stand 2026-10-21) keine passende Tabelle.
3. **CDDIS/BKG keyed bzw. unerreichbar.** Ein IVS-QCRF-Produkt ist weder
   offen gelistet noch offen abrufbar; CDDIS verlangt Earthdata-Login.
4. **Calc/Solve ungemessen.** Die Seite antwortete nicht; Lizenz und
   Downloadweg bleiben offen.
5. **PIMA ohne kanonischen Endpunkt.** Die Behauptung „im HOPS-Paket" ist
   nicht HTTP-belegt und sollte vor einer Nutzung am HOPS-Quellbaum geprüft
   werden.
6. **Lizenzspannung.** QCRF: Zenodo `cc-by-4.0` gegen PDF-„All Rights
   Reserved"; KCRF: gar kein Lizenzfeld. Vor Weitergabe klären.
