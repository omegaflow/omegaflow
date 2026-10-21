<!--
  title: Survey — LOFAR-Quasarkatalog: LoTSS-DR1-Quasarprobe, LoLSS DR1 und ILTJ als Niedrigfrequenz-Linie (2026-10-21)
  class: survey
  date: 2026-10-21
  sha256: 66bbb199ae6b57e6d86db76f62739119698354ecf0a8ac8bfc69dfb42de45889
  status: live
  see-also: docs/surveys/survey-2026-10-21-gaia-dr4-vorbereitung.md docs/surveys/survey-2026-10-20-frame-wache.md tools/measure/src/bin/lofar_catalog_probe.rs
-->
# Survey — LOFAR-Quasarkatalog: LoTSS-DR1-Quasarprobe, LoLSS DR1 und ILTJ

**Zweck.** Gesucht war ein LOFAR-Quasarkatalog, der als **unabhängige
Niedrigfrequenz-Linie** für Position und Identifikation taugt — ausdrücklich
nicht als 0,1-mas-Frame. Geprüft wurden die drei im Auftrag genannten Wege
LoLSS, LoVAR und ILTJ sowie der einzige in VizieR explizit als Quasar-Katalog
ausgewiesene LOFAR-Datensatz. Jede Zahl unten stammt aus einer Messung vom
2026-10-21; der Lauf ist als Werkzeug reproduzierbar.

**Befund vorab.** „LoVAR" ist im erreichbaren Katalogbestand **nicht
vorhanden**. LoLSS DR1 und LoTSS DR3 (ILTJ) sind Radio-Surveys ohne
Quasar-Kennzeichnung; der explizite Quasarkatalog ist die
**LoTSS-DR1-Quasarprobe** `J/A+A/622/A15/ldr1qsos` mit **21.812** Quasaren, von
denen **3.261** eine LDR1-Radioposition und **3.079** eine 144-MHz-Leistung
tragen. Damit ist eine Identifikationslinie auf Bogensekunden-Niveau möglich,
keine 0,1-mas-Positionslinie.

## Katalog und Weg (gemessen)

| Katalog | VizieR/TAP | Rolle | Quellenzahl | Frequenz | Auflösung |
|---|---|---|---:|---|---|
| LoTSS-DR1-Quasarprobe | `J/A+A/622/A15/ldr1qsos` | **expliziter Quasarkatalog** (SDSS DR7/DR12 im LDR1-Feld) | **21.812** | 144 MHz (LDR1) | 6″ (LDR1) |
| LoLSS DR1 v1.1 | `J/A+A/673/A165/lolss1s` | Radio-Survey, keine Quasar-Kennung | **42.463** | 54 MHz (41–66 MHz) | 15″ |
| LoTSS DR3 (ILTJ) | `J/A+A/707/A198/lotssdr3` | Radio-Komponenten, ILTJ-Namen, keine Quasar-Kennung | **13.667.877** | 144 MHz (120–168 MHz) | 6″ (9″ unter Deklination 10°) |
| LoVAR | — | **absent** | **0** | — | — |

Die TAP-Abfrage `description LIKE '%LoVAR%'` und `table_name LIKE '%LoVAR%'`
liefert je **0** Zeilen; `description LIKE '%LOFAR%'` liefert **17** Tabellen.
LoVAR ist damit kein gemessener Katalog, sondern eine benannte Lücke.

## Messung

### Zugang, Status und Lizenz

| Gegenstand | URL | HTTP | Format | Lizenz (gemessen) |
|---|---|---:|---|---|
| Quasarkatalog-Daten | `https://cdsarc.cds.unistra.fr/ftp/J/A+A/622/A15/ldr1qsos.dat` | **200** | `text/plain`, 9.531.844 B | Artikel DOI `10.1051/0004-6361/201833821`; Crossref führt die EDP-Sciences-Copyright-Seite, **keine CC-Kennung** |
| Quasarkatalog-ReadMe | `https://cdsarc.cds.unistra.fr/ftp/J/A+A/622/A15/ReadMe` | **200** | `text/plain`, 10.652 B | CDS-Zugang frei, kein Login |
| LoLSS-DR1-Daten | `https://cdsarc.cds.unistra.fr/ftp/J/A+A/673/A165/lolss1s.dat.gz` | **200** | `application/x-gzip`, 5.234.377 B | Artikel DOI `10.1051/0004-6361/202245389`; **CC BY 4.0** |
| LoLSS-DR1-ReadMe | `https://cdsarc.cds.unistra.fr/ftp/J/A+A/673/A165/ReadMe` | **200** | `text/plain`, 9.515 B | CDS-Zugang frei |
| LoLSS-FITS-Katalog | `https://lofar-surveys.org/public/lolss/catalogues/LoLSS_DR1_v1.1.srl.fits` | **200** | `image/fits`, 5.351.040 B | Projektseite; der CC-BY-2.0-Hinweis dort gilt nach Messung dem **Website-Template**, nicht dem Katalog |
| LoTSS-DR3-ReadMe | `https://cdsarc.cds.unistra.fr/ftp/J/A+A/707/A198/ReadMe` | **200** | `text/plain`, 10.074 B | im ReadMe keine Lizenzzeile; Crossref-Treffer für DR3 am 2026-10-21 nicht gefunden |
| TAP-Sync | `https://tapvizier.cds.unistra.fr/TAPVizieR/tap/sync` | **200** | `text/csv` | keyless, kein Login |

Der `dat`-Abruf des Quasarkatalogs trägt
`sha256=616dbadd9a137ec9b05cb6caaffb290e203759fae4eba3adee816b07816ace1d`;
der LoLSS-`dat.gz`-Abruf
`sha256=343809cb533fc8d8943982e963424a48a1f8a23b1e699ec05a6fea99beb3602b`.

### Spalten

- **Quasarkatalog** (36 Spalten, CSV-Kopf gemessen):
  `SDSS, THINGID, z, BALVI, BICIVt, LOBALVIMG, LOBALVIAL, r_logMBH,
  logMBH, logLbol, e_logLbol, logedd, f_FIRST, RP20, e_RP20, LDR1, RAdeg,
  DEdeg, Maj, Size, LLS, Res, L144, e_L144, logR144, LDR1opt, RAodeg,
  DEodeg, FIRST, L1400, e_L1400, FIRSTLLS, SimbadName, _RA, _DE` (plus
  `recno`). Enthalten sind Rotverschiebung, BAL-Flags, Schwarzes-Loch-Masse,
  Bolometrie, LDR1-Name (ILTJ), LDR1-Position und 144-MHz-Leistung.
- **LoLSS DR1** (25 Spalten): `Source, RAJ2000, e_RAJ2000, DEJ2000,
  e_DEJ2000, Ftot, e_Ftot, Fpeak, e_Fpeak, Maj, e_Maj, Min, E_Min, PA, e_PA,
  DCMaj, e_DCMaj, DCMin, e_DCMin, DCPA, e_DCPA, Islrms, SCode, Mosaic`
  (plus `recno`). Keine Rotverschiebung, keine optische Klasse.
- **LoTSS DR3** (28 Spalten): `Source, RAJ2000, e_RAJ2000, DEJ2000,
  e_DEJ2000, SpeakTot, e_SpeakTot, Speak, e_Speak, Maj, e_Maj, Min, e_Min,
  PA, e_PA, DCMaj, e_DCMaj, DCMin, e_DCMin, DCPA, e_DCPA, IslFTot,
  e_IslFTot, Islrms, SCode, HEALPIX, Res` (plus `recno`). Keine
  Quasar-Kennung, keine Rotverschiebung.

### Zahlen und Positionsgenauigkeit

| Messung | Wert | Quelle |
|---|---:|---|
| Quasare im Katalog | **21.812** | TAP `COUNT(*)` |
| davon mit LDR1-Radioposition | **3.261** | TAP `COUNT(RAdeg)` |
| davon mit 144-MHz-Leistung | **3.079** | TAP `COUNT(L144)` |
| LoLSS-DR1-Quellen | **42.463** | TAP `COUNT(*)` |
| LoTSS-DR3-Komponenten | **13.667.877** | TAP `COUNT(*)` |
| LoLSS e_RA, Mittel | **0,855″** | TAP `AVG(e_RAJ2000)` |
| LoLSS e_DE, Mittel | **0,761″** | TAP `AVG(e_DEJ2000)` |
| LoLSS e_RA, Maximum | **13,16″** | TAP `MAX(e_RAJ2000)` |
| LoTSS DR1 e_RA, Mittel (Elternsurvey) | **0,545″** | TAP `AVG(e_RAJ2000)`, N = 325.694 |
| LoTSS DR1 e_DE, Mittel (Elternsurvey) | **0,457″** | TAP `AVG(e_DEJ2000)` |
| LoTSS DR3 e_RA, Mittel | **0,814″** | TAP `AVG(e_RAJ2000)` |
| LoTSS DR3 e_DE, Mittel | **0,652″** | TAP `AVG(e_DEJ2000)` |

Der LoLSS-ReadMe nennt 15″ Auflösung und 54 MHz; der LoTSS-DR1-ReadMe nennt
6″ Auflösung, 120–168 MHz und eine Positionsgenauigkeit innerhalb 0,2″; der
LoTSS-DR3-ReadMe nennt 6″ (9″ unter Deklination 10°), 120–168 MHz und eine
zusätzliche systematische Astrometrie-Komponente von mindestens 0,24″.
Der Quasarkatalog selbst trägt **keine per-Zeile-Astrometrieunsicherheit**;
seine 3.261 Radiopositionen erben die LoTSS-DR1-Astrometrie.

## Eignung

**Taugt als Identifikations- und Positionslinie auf Bogensekunden-Niveau.**
Der Quasarkatalog ist der einzige gemessene Datensatz, der Quasar, Rotverschiebung
und LOFAR-Position in einer Tabelle verbindet. Die 3.261 Radiopositionen sind
über den ILTJ-Namen direkt anschlussfähig; die 3.079 144-MHz-Leistungen erlauben
eine unabhängige Niedrigfrequenz-Flusskontrolle. LoLSS DR1 ergänzt dieselbe
Linie bei 54 MHz, jedoch nur als Radioquelle ohne Quasar-Etikett.

**Nicht tauglich als 0,1-mas-Frame.** Die gemessenen Unsicherheiten liegen bei
0,5–0,9″ im Mittel (LoLSS bis 13,16″ im Ausreißer), die systematischen Anteile
bei 0,2–0,24″. Das ist drei bis vier Größenordnungen über 0,1 mas. Der Katalog
kann eine Identifikation stützen und eine unabhängige Frequenzachse liefern,
aber keinen Frameanker ersetzen.

**Quasare/AGN.** Im expliziten Quasarkatalog: ja, 21.812 SDSS-Quasare. In LoLSS
DR1 und LoTSS DR3: Radioquellen, darunter AGN, aber **ohne Quasar- oder
AGN-Kennzeichnung**; eine Quasarzuordnung ist dort nur über einen externen
Kreuzabgleich möglich und in diesem Survey nicht behauptet.

## Grenzen

- LoVAR wurde nicht gefunden: 0 Zeilen in `TAP_SCHEMA.tables` für Name und
  Beschreibung. Das ist eine gemessene Abwesenheit im VizieR-Bestand, keine
  Aussage über unveröffentlichte oder nicht indexierte LoVAR-Daten.
- LoLSS DR1 deckt nur das HETDEX-Spring-Feld (650 deg²) ab, nicht den ganzen
  Nordhimmel; die 42.463 Quellen sind ein Feldkatalog.
- Der Quasarkatalog enthält **alle** SDSS-Quasare im LDR1-Feld; nur 3.261 haben
  tatsächlich eine LDR1-Radioposition. Die übrigen 18.551 sind für eine
  Positionslinie nicht verwendbar.
- LoTSS DR3 ist mit 13,67 Mio. Komponenten kein Quasarkatalog; die ILTJ-Namen
  sind lediglich das Namensschema.
- Die Lizenz des Quasarkatalogs ist nicht als CC-BY im Crossref-Datensatz
  markiert; die des LoTSS-DR3-Katalogs war am 2026-10-21 nicht belegbar.
- Die 0,1-mas-Grenze wird nicht erreicht; jede Aussage über eine
  Sub-Milliarcsec-Koinzidenz wäre mit diesem Katalog unbelegt.

## Reproduktion

```sh
cd /home/johannes/projects/omegaflow
cargo build --release -p omegaflow-measure --bin lofar_catalog_probe
./target/release/lofar_catalog_probe
```

Das Werkzeug misst für jeden Katalog: TAP-CSV-Status, Spaltenkopf, Zeilenzahl,
Statistikfelder, `dat`-Status/Größe/SHA-256, ReadMe-Status/Größe/SHA-256 und die
ReadMe-Zeilen zu MHz und Auflösung. Es meldet `machine_readable=no` und
`unreadable`/`absent`, wenn eine Stufe nicht trägt. Der Lauf vom 2026-10-21
endet mit:

```
search=LoVAR-description n=0
search=LoVAR-table n=0
search=LOFAR-tables n=17
summary=catalogs=3 machine_readable=3
summary=lovar=absent
```

Einzelabfrage ohne Werkzeug, identisch:

```sh
curl -sS 'https://tapvizier.cds.unistra.fr/TAPVizieR/tap/sync?REQUEST=doQuery&LANG=ADQL&FORMAT=csv&QUERY=SELECT%20COUNT(*)%20AS%20n%20FROM%20%22J%2FA%2BA%2F622%2FA15%2Fldr1qsos%22'
```
