<!--
  title: Survey — Pan-STARRS1-Quasarkatalog: PS1-STRM, Zugang und Eignung für die 88 ICRF-Anker ohne Gaia (2026-10-21)
  class: survey
  date: 2026-10-21
  sha256: 1158f71287c29ad27292a8a9a43a717ffa05a2d6d0f0e4ce76eba2fc2ed7bf45
  status: live
  see-also: projects/quasare.md docs/surveys/survey-2026-10-21-gaia-dr4-vorbereitung.md tools/measure/src/bin/panstarrs_catalog_probe.rs
-->
# Survey — Pan-STARRS1-Quasarkatalog: PS1-STRM

**Zweck.** Gesucht war der echte Pan-STARRS1-Quasarkatalog, nicht SDSS, und
seine Eignung für die **88 ICRF-Anker ohne Gaia-Zuordnung**
(`/tmp/opencode/gaia/missing88.csv`). Das Ergebnis der Suche ist unbequem:

1. Einen dedizierten, spektroskopisch bestätigten **„Pan-STARRS1 Quasar
   Catalog“** vom Zuschnitt des SDSS-Katalogs gibt es nicht. Das breite
   Pan-STARRS1-Produkt ist **PS1-STRM** (Beck et al. 2020, MNRAS 500, 1633):
   eine ML-Klassifikation (GALAXY/STAR/QSO/UNSURE) plus Photo-z für **alle**
   2.902.054.648 Quellen des PS1-3π-DR1-Katalogs. „QSO“ ist dort eine
   **photometrische Klasse**, keine bestätigte Identität.
2. Die 88 Anker liegen zu **78** im PS1-Footprint (δ ≥ −30°), zu **10** außerhalb.
3. Ein 2″-Matching ist möglich. Es liefert **68** PS1-Gegenstücke, davon **59**
   mit Versatz < 0,5″ und **46** < 0,1″. Der Kern sitzt bei **~31 mas**; das ist
   die ehrliche Grenze, nicht 0,1 mas.
4. Die **QSO-Klasse pro Anker** wurde nicht abgerufen: PS1-STRM ist entweder
   über MAST-CasJobs (Login) oder als 2,4–11,9-GB-declination-strips
   zugänglich. Der anonyme Weg ist damit versperrt; die strips sind gemessen,
   aber für 78 Einzelpositionen nicht sinnvoll lösbar.

Die im Auftrag genannte Abgrenzung **J/ApJS/253/19 ist SDSS DR5** ist selbst
falsch: eine VizieR-Abfrage am 2026-10-21 liefert für `J/ApJS/253/19` die
**LAMOST-M-Sterne** von Zhang et al., weder PS1 noch SDSS DR5. Der Katalog
wurde nicht verwendet. SDSS-Kataloge wurden insgesamt nicht als PS1-Ersatz
herangezogen.

## Katalog/Weg (gemessen, nicht angenommen)

| | Messung 2026-10-21 |
|---|---|
| Produkt | PS1-STRM — Pan-STARRS1 Source Types and Redshifts with Machine Learning |
| Papier | Beck, Szapudi, Flewelling, Holmberg, Magnier, Chambers 2020, MNRAS **500**, 1633–1644, DOI `10.1093/mnras/staa2587` |
| HLSP-Seite | `https://archive.stsci.edu/hlsp/ps1-strm` — **HTTP 200** |
| ReadMe | `https://archive.stsci.edu/hlsps/ps1-strm/hlsp_ps1-strm_ps1_gpc1_all_multi_v1_readme.txt` — **HTTP 200**, 4.610 B |
| DOI / Lizenz | `10.17909/t9-rnk7-gr88`; HLSP-Seite: **CC BY 4.0** (`creativecommons.org/licenses/by/4.0/`) |
| Abdeckung | PS1 3π DR1, **δ ≥ −30°**, ca. 3/4 Himmel (~30.000 deg²), galaktische Ebene eingeschlossen |
| Quellenzahl | **2.902.054.648** Objekte (ReadMe und Papier-Abstract) |
| Spalten | **19**, maschinenlesbar aus dem ReadMe: `objID, uniquePspsOBid, raMean, decMean, l, b, class, prob_Galaxy, prob_Star, prob_QSO, extrapolation_Class, cellDistance_Class, cellID_Class, z_phot, z_photErr, z_phot0, extrapolation_Photoz, cellDistance_Photoz, cellID_Photoz` |
| Klassen | `GALAXY`, `STAR`, `QSO`, `UNSURE`; Entscheidungsgrenze b = 0,7 |
| QSO-Güte | 96,6 % Klassifikationsgenauigkeit für nicht-extrapolierte Quellen im Validierungssatz (Papier) |
| Dateien | 33 declination-strips, `hlsp_ps1-strm_ps1_gpc1_<decmin>-<decmax>_multi_v1_cat.csv.gz`, **headerlos**, 19 CSV-Felder |
| Strip-Größen | gemessen per HEAD, HTTP 200: `m15-m12` 9.716.636.714 B; `p01-p05` 11.887.130.072 B; `p25-p28` 7.411.531.530 B; `p38-p41` 6.515.030.881 B; `p77-p90` 2.425.134.617 B |
| Range-Zugriff | `Range: bytes=0-999999` auf `p01-p05` → **HTTP 206**, 1.000.000 B; `gzip -dc` liefert 2.000 Zeilen mit **19 Feldern** und den Klassen `GALAXY/STAR/QSO/UNSURE` |
| Positionsfelder | `raMean`, `decMean` sind die PS1-DR1-Mean-Positionen; dieselben Größen liefert der PS1-DR1-ConeSearch |
| Positionsgenauigkeit | PS1-Formalerrors der 68 Treffer: Median **17,5 mas (RA)** / **17,4 mas (Dec)**; beobachteter Anker-Offset im Kern **31,4 mas** Median |
| QSO-Klasse je Anker | **nicht abgerufen** — CasJobs verlangt Login; kein anonymer SQL-Zugang gemessen |
| CasJobs | `https://mastweb.stsci.edu/mcasjobs/` — **HTTP 200**, Seite meldet „Not Logged in“ |
| ConeSearch | `https://gsss.stsci.edu/webservices/vo/ConeSearch.aspx?CAT=PS1V3OBJECTS&RA=…&DEC=…&SR=…` — **HTTP 200**, VOTable, 2″-Radius |
| MAST-TAP | `https://mast.stsci.edu/vo-tap/api/v0.1/ps1dr1/sync` — **HTTP 200**, aber `dbo.objectthin` ist nicht räumlich indiziert; Geometrie-Queries werden mit „non-indexed table“ abgewiesen. Deshalb ConeSearch statt TAP. |
| VizieR | `https://tapvizier.cds.unistra.fr/TAPVizieR/tap/sync` — **HTTP 200**; PS1-STRM ist dort **nicht** registriert; `II/349` (PS1 DR1, 1.919.106.885 Quellen) hat **keine** QSO-Klasse |

**Kein zweiter breiter PS1-Quasarkatalog.** Die Alternativen sind entweder
zu klein oder Gaia-abhängig:

- **PS-ELQS** (Schindler et al. 2019, ApJS 243, 5): nur die extrem
  leuchtkräftigen Quasare, kein flächendeckender Katalog.
- **PS1 z > 5.6 Quasar Survey** (Bañados et al. 2016; Schindler et al. 2023):
  einige hundert Objekte, viel zu dünn für 88 beliebige ICRF-Anker.
- **CatNorth** (Fu et al. 2024, ApJS 271, DOI `10.3847/1538-4365/ad2ae6`):
  verbesserter **Gaia-DR3**-Quasar-Kandidatenkatalog mit PS1 und CatWISE.
  Er setzt eine Gaia-DR3-Detektion voraus und ist damit für die 88 Anker, die
  gerade **ohne** Gaia sind, definitionsgemäß ungeeignet.

## Messung: 88 Anker gegen PS1

Methode: Für jeden Anker eine ConeSearch mit **2,0″** Radius gegen
`PS1V3OBJECTS`; nächster Treffer nach Winkelabstand; `raMean/decMean` und
`RAmeanErr/DECmeanErr` aus demselben PS1-DR1-Mean-Katalog, den PS1-STRM
klassifiziert. Kein Katalogtreffer wird als „kein Gegenstück“, nicht als
Nullposition geführt.

| | Zahl |
|---|---:|
| Anker geprüft | **88** |
| im PS1-Footprint (δ ≥ −30°) | **78** |
| außerhalb des Footprints | **10** |
| Treffer innerhalb 2,0″ | **68** (87,2 % der 78) |
| Treffer innerhalb 0,5″ | **59** (75,6 % der 78) |
| Treffer innerhalb 0,2″ | **52** |
| Treffer innerhalb 0,1″ | **46** |
| ohne Treffer innerhalb 2,0″ (im Footprint) | **10** |

Versatz der 68 Treffer (Ankerposition minus PS1-Mean-Position):

| Größe | alle 68 | Kern (≤ 0,5″) |
|---|---:|---:|
| Median | **38,17 mas** | **31,44 mas** |
| RMS | 511,23 mas | 123,05 mas |
| p95 | 1.387,86 mas | 291,95 mas |
| Maximum | 1.904,05 mas | 401,64 mas |

Der RMS der vollen Menge ist von **9 Ausreißern > 0,5″** getragen; sie sind
mit hoher Wahrscheinlichkeit Zufallsbegegnungen im 2″-Zylinder, keine
Quasar-Gegenstücke. Der Kern von 59 Quellen clustert bei **~30 mas**, passend
zu den PS1-Formalerrors von **17,5 mas**. Die harte Aussage bleibt: Pan-STARRS
liefert **10–30 mas**, also zwei bis drei Größenordnungen über einem
0,1-mas-Anker. Als ICRS-Referenz für die 88 Anker ist PS1 damit unbrauchbar;
als optische Identifikationshilfe und Farb-/Photo-z-Vorprüfung ist es brauchbar.

## Abdeckung der 88

**Außerhalb des Footprints (10, δ < −30°), kein PS1-Matching möglich:**
`0229-398, 1049-534, 1204-613, 1207-319, 1306-395, 1511-558, 1530-536,
1732-593, 1802-381, 1815-553`.

**Im Footprint, aber ohne PS1-Quelle innerhalb 2″ (10):**
`0013-184, 0415+398, 0544+273, 0547+234, 0556+238, 0633+595, 1726-269,
1749-299, 1845-273, 2233-234`.

**Im Footprint, Treffer > 0,5″ (9, Zuordnung zweifelhaft):**
`0648-165` 512,7 mas; `1708-250` 908,3; `1711-208` 1.418,7; `1718-259`
1.286,3; `1822+012` 1.758,5; `1825-214` 1.408,5; `1928+154` 1.349,5;
`1929+226` 1.265,4; `2013+163` 1.904,1.

Damit bleiben nach strenger Lesart **59 von 78** Ankern mit einem
plausiblen PS1-Gegenstück; **19** haben keines oder nur eine verdächtige
2″-Begegnung. Die volle Einzelzeile steht in
`/tmp/opencode/panstarrs-quasar-88.tsv` (Spalten u. a. `association`,
`sep_mas`, `ps1_objid`, `ps1_ra_err_mas`, `g_psf`…`y_psf`).

## Grenzen

- **QSO-Label fehlt.** Die 68 Treffer sind PS1-Quellen, nicht als Quasar
  bestätigt. `prob_QSO` aus PS1-STRM wurde für keinen Anker gelesen. Eine
  Aussage „X von 88 sind Quasare“ ist mit dieser Messung nicht belegt.
- **ML-Klasse, keine Spektroskopie.** PS1-STRM erreicht 96,6 % QSO-Genauigkeit
  im nicht-extrapolierten Validierungssatz; per-Objekt-Purity und
  Extrapolationsflag sind nicht gemessen. `UNSURE` ist ein echter Klassenwert,
  kein NaN-Ersatz.
- **Astrometrie.** PS1-DR1-Mean-Positionen sind ~10–30 mas genau, nicht
  0,1 mas. Die 88 Anker sind VLBI/ICRF-Positionen; ein PS1-Offset von 30 mas
  ist ein PS1-Fehler, kein Anker-Fehler. PS1 kann Gaia nicht ersetzen.
- **2″-Zylinder.** Ein 2″-Matching ist ein Assoziationsfilter, keine
  Identifikation. In dichten Feldern ist die Zufallswahrscheinlichkeit
  nicht vernachlässigbar; die 9 Ausreißer > 0,5″ sind der sichtbare Teil
  davon.
- **Footprint.** 10 der 88 liegen südlich von −30° und sind mit PS1 3π
  grundsätzlich nicht erreichbar. Das ist eine Himmelsgrenze, kein
  Zugangsproblem.
- **Zugang.** Die 2,4–11,9-GB-strips sind per Range lesbar, aber nicht
  positionsindiziert; für 78 Punkte wären viele GB Stream nötig. CasJobs ist
  der richtige Weg, verlangt aber einen Login. Dieser Survey hat keinen
  Login verwendet.

## Reproduktion

```sh
cd /home/johannes/projects/omegaflow
cargo build --release -p omegaflow-measure --bin panstarrs_catalog_probe
./target/release/panstarrs_catalog_probe
```

Das Werkzeug misst HLSP-Seite, ReadMe, Spalten, Quellenzahl, vier
Strip-HEADs, einen 1-MB-Range-Sample (19 Felder, Klassen), den Footprint der
88 und je Anker die 2″-ConeSearch. Es schreibt
`/tmp/opencode/panstarrs-quasar-88.tsv` und druckt die Bilanz nach stdout.
Laufzeit des vollen Laufs am 2026-10-21: **1 min 23 s** (88 ConeSearch-Aufrufe,
120 ms Pause). Optionen: `--metadata-only`, `--limit N`,
`--radius-arcsec F`, `--anchors PFAD`, `--out PFAD`, `--no-strip-sample`.

Suchkanäle, die in diesem Lauf ausfielen oder leer blieben:

- `archive_search --mwmbl "Pan-STARRS1 quasar catalog"` → **absent**.
- `archive_search --vizier "Pan-STARRS1 quasar catalog"` → **absent**, obwohl
  PS1-STRM existiert; der VizieR-Index führt es nicht.
- `archive_search --arxiv "Pan-STARRS1 quasar"` → **kein Output**, Laufzeit
  > 45 s (Timeout). Direkter `arxiv.org/search` und `arxiv.org/abs/1910.10167`
  antworteten mit **HTTP 200**; die arXiv-API `export.arxiv.org` lief in einen
  50-s-Timeout.
- `archive_search --verdict` und `--sniff` sind in diesem Build **nicht
  implementiert** (die Hilfe führt sie nicht; vorhanden sind `--stats` und
  `--leads`). Die Suche lief deshalb über `--searxng`, `--regtap`, Crossref,
  DataCite, MAST und VizieR-TAP.

Nicht committet, nicht gepusht. Keine Änderung an `phi/sources.φ` oder
Handover-Dateien.
