<!--
  title: Survey — Q-Band (43 GHz) VLBI: Lanyi+2010, QCAL-1 und die vierte Radio-Linie der Frame-Wache (2026-10-21)
  class: survey
  date: 2026-10-21
  sha256: 0ec3927e862fe1db8c370634f1e3a0c9f19461c7b41b8be6a4988b0a25a43180
  status: live
  see-also: docs/surveys/survey-2026-10-20-frame-wache.md tools/measure/src/bin/qband_catalog_probe.rs
-->
# Survey — Q-Band (43 GHz) VLBI: Lanyi+2010, QCAL-1 und die vierte Radio-Linie

**Zweck.** Die Frame-Wache (`survey-2026-10-20-frame-wache.md`) führt drei
Radio-Linien (ICRF3 S/X, K, X/Ka) und eine optische Gegenlinie (Gaia-CRF3).
Gesucht war eine **vierte Radio-Linie bei 43 GHz (Q-Band)**: eine andere
Frequenz, kompaktere Kerne, damit ein Struktur-/Frame-Test, der nicht nur die
ICRF3-Konstruktion wiederholt. Jede Zahl unten stammt aus einer HTTP-,
Header- oder Tabellenmessung vom 2026-10-21; der Lauf ist als Werkzeug
reproduzierbar (`qband_catalog_probe`).

**Befund vorab.** Es gibt **zwei maschinenlesbare 43-GHz-Quasarkataloge**, und
nur einer trägt vollständige Positionen; dazu kommen **zwei Paper-CRFs**
(QCRF 2021, KCRF 2024) ohne offene Tabelle:

| Kandidat | Träger | Quellen | Was gemessen wird | Taugt als vierte Linie? |
|---|---|---:|---|---|
| **Lanyi+2010** | VLBA, 10×24 h über 5 Jahre | **131** | Positionen + formale Fehler bei 43 GHz | **ja, als Struktur-Test**; nein als Rahmen-Ursprung |
| **QCRF 2021** | Krásná+2024, VLBA, 3×24 h 2021 | **428** | Q-Band-CRF, 203/353 µas; **A1-Poster, 0 Quellenzeilen** | ja als Struktur-Test; **Tabelle nicht offen** |
| **KCRF 2024** | de Witt+2024, VLBA | **1.315** | K-Band-CRF, 52/91 µas; **PDF, 0 Quellenzeilen** | K-Schwester, kein Q-Band |
| **QCAL-1** | KVN (Korea), 2011-12-18/19/22 | **637** Fluss / **33** Positionen | korrelierte Flussdichten in drei Baseline-Bereichen | **nein** als Frame-Linie; ja als Kompaktheits-Zensus |
| EVN 43 GHz | EVN-Netz (u. a. Yebes 40 m, EVN Observation Planner) | — | **Netz, kein Katalog**: 43-GHz-Beobachtungen existieren (M81*, DOI `10.22323/1.072.0090`); `planobs.jive.eu` 200; JIVE-Archiv 206 | **kein Katalog**, nicht „keine Beobachtungen" |

## Kandidaten und Suchweg (gemessen)

Gesucht wurde mit `archive_search` (`--mwmbl`, `--arxiv`, `--vizier`) und
direkt über CDS/VizieR-TAP (ADQL gegen `TAP_SCHEMA.tables`) sowie arXiv.

- `archive_search --mwmbl "KVN 43 GHz VLBI quasar catalog"` → Treffer
  `http://astrogeo.org/qcal1` (QCAL-1).
- `archive_search --arxiv "QCAL-1"` → `http://arxiv.org/abs/1207.5872v2`.
- `archive_search --vizier "43 GHz VLBI"` → **absent**; die VizieR-Suche des
  Werkzeugs matcht nur `description LIKE`, nicht Katalogtitel.
- Direkte TAP-Suche `description LIKE '%43 GHz%'` → genau ein Quasar-Katalog:
  `J/AJ/139/1695/table3` (Lanyi+2010). `LIKE '%QCAL%'` → `J/AJ/144/150/table3`
  und `table4`. `LIKE '%Q-band%'` liefert nur VLA-Sternentstehung
  (`J/MNRAS/504/338`), keine Quasare.
- `LIKE '%KVN%'` liefert ausschließlich Moleküllinien-Kataloge
  (`J/A+A/630/A69/...`), keinen 43-GHz-AGN-Katalog.
- EVN 43 GHz: kein eigener Katalog in VizieR; die EVN-Beiträge zu 43 GHz
  stecken in Einzelprogrammen, nicht in einem Katalogdatensatz.

**Webweite Suche (2026-10-21).** `archive_search --all 'EVN 43 GHz'` (Serper,
Semantic Scholar, Crossref u. a.) plus `--mwmbl`/`--arxiv`/`--verdict`:

- `https://planobs.jive.eu/` — EVN Observation Planner, 23/43/86 GHz, HTTP 200.
- `https://doi.org/10.22323/1.072.0090` — „The VLBI morphology of M81* at
  43 GHz“, HTTP 206.
- `https://archive.jive.nl/` — JIVE-Archiv, HTTP 206 (Visibilities/
  Experimente, kein Katalog).
- Yebes 40 m nennt 43 GHz in der EVN-Ausschreibung (SERP-Treffer;
  `rt40m.oan.es` antwortet nicht, nicht verifiziert).

Ergebnis: EVN 43 GHz ist ein **Netz/Archiv**, kein Quasarkatalog. Die erste
Formulierung „absent“ war zu weit; richtig ist **„kein Katalog“**.

## Gemessener Zugang

Alle Zeilen aus einem Lauf von `qband_catalog_probe` (curl, HTTP-Status,
Bytes, sha256 des empfangenen Körpers):

| Kanal | URL | HTTP | Bytes | sha256 |
|---|---|---:|---:|---|
| ICRF3 S/X (Referenz) | `https://hpiers.obspm.fr/icrs-pc/icrf/icrf3sx.txt` | 200 | 745.282 | `83abfa7c8ece43066cff93cd256e8744ecd46cdb4c1f6cde6ed4c411b0bab3bb` |
| QCAL-1 Seite | `http://astrogeo.org/qcal1` | 301→200 | 7.093 | `65177daaa55a0b1c5a976c6862f4298ce4f9a64049ff10ffe2953c521b6f8294` |
| QCAL-1 table3 roh | `http://astrogeo.org/qcal1/datafile1.txt` | 200 | 63.678 | `daa29bf00984e7ddccac5a680922f6a8584572892e7647dae1eee0103fd935fa` |
| QCAL-1 table4 roh | `http://astrogeo.org/qcal1/datafile2.txt` | 200 | 4.725 | `c11802429b8586b0f2015379ef8c576f1706c701e7a4b8ecdebc07cc820e9e4d` |
| QCAL-1 table3 (VizieR) | `J/AJ/144/150/table3` via TAP-CSV | 200 | 60.275 | `b10c21f3c7d20d18f0e29840eae9ea9b60b333c467bf1b36546254f1983c3fd1` |
| QCAL-1 table4 (VizieR) | `J/AJ/144/150/table4` via TAP-CSV | 200 | 2.844 | `5a172e569702c7c1b3fddb2a4928485220e25ba67f19277d560afbc778b9a5cc` |
| Lanyi table3 (VizieR) | `J/AJ/139/1695/table3` via TAP-CSV | 200 | 12.021 | `6486bbca84e4d7f06b870742d2b3eb8794c7d6053792b84538fc69adea8776fb` |
| QCAL-1 ReadMe | `https://cdsarc.cds.unistra.fr/ftp/cats/J/AJ/144/150/ReadMe` | 200 | 8.510 | `e2bf7609c1f300ce5945bb41dfbacf02a23063fd131f83c2e53c3b5c51d4365b` |
| Lanyi ReadMe | `https://cdsarc.cds.unistra.fr/ftp/cats/J/AJ/139/1695/ReadMe` | 200 | 6.426 | `a825617db3c010d1f7143e69e114b92a59833fedd6dde24323430399217d5da1` |

Content-Types: ICRF3 `text/plain`, QCAL-1-Seite `text/html; charset=iso-8859-1`
(Weiterleitung 301 auf `/qcal1/`, letzte Änderung nicht im Header),
`datafile*.txt` `text/plain`, TAP-CSV `text/csv;charset=UTF-8`, ReadMes
`text/plain; charset=utf-8`.

**Lizenz/Terms.** Die VizieR-/CDS-Kataloge sind öffentlich und zitierpflichtig;
ein explizites SPDX-/CC-Feld ist in den gemessenen Datensätzen **nicht**
vorhanden — „frei nutzbar mit Zitat" ist die belegbare Aussage, mehr nicht.
Die astrogeo-Seite trägt dagegen eine ausdrückliche Einschränkung:
„Content from this Web site cannot be displayed on-line on any other servers
without written permission from authors." Deshalb ist **VizieR der kanonische
maschinenlesbare Weg**; astrogeo nur als Rohspiegel derselben CDS-Tabellen.

## Spalten und Quellenzahl

ReadMe-deklarierte Zeilenzahlen und geparste CSV-Zeilen stimmen in allen drei
Datensätzen überein (`match ok`):

| Datensatz | deklariert | geparst | Spalten |
|---|---:|---:|---|
| QCAL-1 t3 | 637 | 637 | `recno, JName, BName, T, N, F1, e_F1, F2, e_F2, F3, e_F3, RAJ2000, DEJ2000` |
| QCAL-1 t4 | 33 | 33 | `recno, JName, BName, RAJ2000, e_RAJ2000, DEJ2000, e_DEJ2000, Corr, N1, N2` |
| Lanyi t3 | 131 | 131 | `recno, JName, BName, S43, RAJ2000, e_RAJ2000, DEJ2000, e_DEJ2000, C_pos, Nep, Nobs` |

Die Rohspiegel `datafile1.txt`/`datafile2.txt` ergeben dieselben Zeilenzahlen
637/33 (Zählung der Datenzeilen, nicht des Kopfes). Die QCAL-1-Seite ist
**HTML mit Textlink auf den Katalog und 24 FITS-Links** — kein eigener
maschinenlesbarer Endpunkt.

**Epoche und Frequenz.** Lanyi+2010: VLBA, 43 GHz, zehn 24-h-Sessions über
fünf Jahre (`Nep` 1–4, `Nobs` 3–553, `S43` 0,26–18,4 Jy). QCAL-1: KVN,
43 GHz, drei Sessions am 2011-12-18/19/22 (`N` 2–48 Detektionen).

**Positionsgenauigkeit (formale Fehler, gemessen aus den Fehlerspalten).**

| Datensatz | n | median e_RA | median e_DE |
|---|---:|---:|---:|
| Lanyi t3 | 131 | **0,24 mas** | **0,35 mas** |
| QCAL-1 t4 | 33 | 20,0 mas | 16,0 mas |
| QCAL-1 t3 | 637 | keine Positionsfehler-Spalte | — |

## Abdeckung gegen ICRF3 S/X

Nächster Nachbar in ICRF3 S/X (4.536 Quellen, 303 D), Toleranz 1,5″; „>5 mas"
zählt Kandidaten mit Kernverschiebung zwischen 43 GHz und S/X:

| Katalog | Zeilen | Treffer | davon D | Anteil | Median-Abstand | >5 mas | max |
|---|---:|---:|---:|---:|---:|---:|---:|
| Lanyi+2010 (Positionen) | 131 | **131** | **53** | **100,0 %** | **0,49 mas** | 6 | 20,89 mas |
| QCAL-1 t3 (Fluss) | 637 | 605 | 80 | 95,0 % | 0,91 mas | 15 | 38,48 mas |
| QCAL-1 t4 (33 Positionen) | 33 | 16 | 0 | 48,5 % | 18,50 mas | 15 | 38,48 mas |

Der stärkste Lanyi-Ausreißer ist `J2129-1538` (IERS `2126-158`) mit 20,89 mas;
es folgen `J2311+4543` (14,57 mas), `J0509+0541` (12,60 mas) und `J0648-3044`
(6,51 mas, ICRF3-D). Das sind **Struktur-Kandidaten**, keine Rahmen-Aussage.

**Himmelsabdeckung.** Lanyi: Deklination +81,88° bis −33,65°, **keine** Quelle
südlich −40°. QCAL-1 t3: +83,32° bis −41,17°. ICRF3 S/X ist Vollhimmel; die
Süd-Hemisphäre unter −40° fehlt beiden Q-Band-Katalogen vollständig.

## Bewertung als vierte Linie

**Lanyi+2010 ist die einzige brauchbare vierte Radio-Linie — aber als
Struktur-Test, nicht als Frame-Ursprung.**

Was trägt:

- **Andere Frequenz, kompaktere Kerne.** 43 GHz statt 2,3/8,4/24/32 GHz; der
  Kern ist näher am jet-Basis und die Kernverschiebung zum S/X-Kern wird
  messbar. Genau das ist der Struktur-Kanal, den die Frame-Wache bisher nur
  zwischen S/X, K und X/Ka hat.
- **Vollständige Positionen mit Fehlern.** 131 Quellen, Median 0,24/0,35 mas
  formal; 100 % haben einen ICRF3-S/X-Partner, 53 davon D. Damit lässt sich
  derselbe Mahalanobis-/Rotationsapparat wie in der Frame-Wache fahren.
- **Gemessener Median-Abstand 0,49 mas** liegt in derselben Größenordnung wie
  die S/X–K-Streuung (1,29 mas) und unter S/X–X/Ka (2,93 mas). Das ist
  plausibel für Kernverschiebung plus Epochendifferenz.

Was **nicht** trägt:

- **Kein unabhängiger Rahmen.** VLBA ist dieselbe Teleskopklasse wie ICRF3;
  die Quellen sind aus dem ICRF-Umfeld ausgewählt; die Epochen liegen
  überwiegend vor 2008. Es ist ein Konsistenz-/Struktur-Test, kein
  Unabhängigkeitsbeweis — dieselbe Einschränkung, die die Frame-Wache für
  S/X, K und X/Ka bereits benennt.
- **131 Quellen = 2,9 % von ICRF3 S/X.** Für einen Rotationsfit über D-Quellen
  bleiben 53; das ist eine kleine Ensemble-Basis, und die Süd-Hemisphäre fehlt.
- **Kein Vollhimmel, keine Eigenbewegung.** Die fünf Jahre sind kein
  astrometrischer Zeitbasissatz; eine QSO-Eigenbewegung wird nicht modelliert.

**QCAL-1 ist keine vierte Frame-Linie.** Der 637-Zeilen-Katalog enthält keine
Positionen und keine Positionsfehler (nur `RAJ2000/DEJ2000` aus der
Target-Liste); nur 33 Quellen haben echte KVN-Positionen mit 16–20 mas
formalen Fehlern. Als 0,1-mas-Linie unbrauchbar. Wert hat er als
**Kompaktheits-/Fluss-Zensus**: `F1/F2/F3` sind korrelierte Flussdichten in
drei Baseline-Projektionsbereichen (<21,44 Mλ, 21,44–42,88 Mλ, >42,88 Mλ),
also ein Maß dafür, wie viel Fluss bei 43 GHz in den kompaktesten Skalen
bleibt. 95 % der 637 Quellen haben einen ICRF3-S/X-Partner.

**Nebenbefund, der quer liegt.** QCAL-1 table4 nennt 33 Quellen „never before
observed with VLBI"; **16 dieser 33** haben dennoch einen ICRF3-S/X-Partner
innerhalb 1,5″ (Median 18,5 mas Abstand). Entweder ist die Papierformulierung
auf den Stand vor 2012 bezogen und ICRF3 hat die Quellen später aufgenommen,
oder die Positionen sind zufällig nahe. Bei 4.536 ICRF3-Quellen ist ein
Zufallstreffer in 1,5″ rechnerisch vernachlässigbar; die Koinzidenz ist real.
Die Aussage „nie zuvor VLBI-beobachtet" wird hier **nicht** übernommen.

## Grenzen

- **Frequenz ist keine Unabhängigkeit.** 43 GHz teilt Teleskope, Korrelator
  und Quellenpool mit den bestehenden Linien; der Gewinn ist Auflösung, nicht
  ein zweiter Ursprung.
- **Kein Süd-Himmel.** Beide Kataloge enden bei −33,65° bzw. −41,17°; ICRF3
  deckt den ganzen Himmel. Jede Süd-Aussage wäre unbelegt.
- **Epoche vs. Struktur.** Der gemessene Median-Abstand (0,49 mas) ist größer
  als die formalen Fehler (0,24/0,35 mas); die 6 Quellen >5 mas sind echte
  Kernverschiebung oder Epochenmischung, nicht automatisch ein Rahmen-Riss.
- **Fehlerbalken sind formal.** Wie bei der Frame-Wache gilt: `chi2/dof > 1`
  wäre zu erwarten; eine Skalierung ist hier nicht behauptet.
- **Lizenz nur indirekt belegt.** VizieR/CDS: zitierpflichtig, kein
  gemessenes SPDX-Feld; astrogeo verbietet Spiegelung ohne schriftliche
  Erlaubnis.
- **QCAL-1-Positionen sind keine Astrometrie.** 33 Quellen, 16–20 mas; die
  übrigen 604 Zeilen tragen nur Target-Koordinaten.

## Reproduktion

```sh
cd /home/johannes/projects/omegaflow
cargo build --release -p omegaflow-measure --bin qband_catalog_probe
./target/release/qband_catalog_probe \
  --sources /tmp/opencode/qband-sources \
  --out /tmp/opencode/qband-crossmatch.tsv
```

Optionen: `--sources <dir>` (Cache-Wurzel, Default `sources`), `--offline`
(nur Cache lesen; meldet dann ehrlich `absent/unreadable`, wenn nichts liegt),
`--out <pfad>` (Quellen-Crossmatch als TSV). Das Werkzeug lädt ICRF3 S/X,
QCAL-1-Seite, beide Rohspiegel, beide ReadMes und die drei VizieR-TAP-CSVs per
HTTP, misst Status/Bytes/sha256/Content-Type, parst die CSV-Köpfe, zählt
Zeilen gegen die ReadMe-Deklaration, matcht die Quellen gegen ICRF3 S/X und
gibt Abdeckung, formale Fehler und Deklinationsbereich aus.

Einzelabfrage ohne Werkzeug, identisch:

```sh
curl -sS 'https://tapvizier.cds.unistra.fr/TAPVizieR/tap/sync?REQUEST=doQuery&LANG=ADQL&FORMAT=csv&QUERY=SELECT%20*%20FROM%20%22J%2FAJ%2F139%2F1695%2Ftable3%22'
```

Lauf vom 2026-10-21 (Auszug):

```
ICRF3 S/X reference: parsed 4536 | D 303
QCAL-1 table3 (VizieR): rows 637 | declared 637 | match ok
QCAL-1 table4 (VizieR): rows 33 | declared 33 | match ok
Lanyi+2010 table3 (VizieR): rows 131 | declared 131 | match ok
Lanyi+2010 Q-band positions | 131 | 131 | 53 | 100.0% | 0.49 | 6 | 20.89
QCAL-1 table3 (637 flux catalog) | 637 | 605 | 80 | 95.0% | 0.91 | 15 | 38.48
```

Der Lauf schreibt 802 Zeilen (Kopf + 637 + 33 + 131) in den Crossmatch-TSV.
