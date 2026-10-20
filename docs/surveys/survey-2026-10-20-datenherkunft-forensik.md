<!--
  title: Datenherkunft aus Fragmenten — was die Literatur weiß (2026-10-20)
  class: survey
  date: 2026-10-20
  sha256: 590a33fc138edc48497d76d593fbdcc40199a20e1968b1f74dab7ae11f008cd1
  status: live
  see-also: docs/surveys/survey-2026-10-20-zwillinge-signatur.md docs/blatt/blatt-2026-10-19-entscheidung-17-lizenzarme.md
-->
# Datenherkunft aus Fragmenten — was die Literatur weiß

**Frage:** Gibt es etabliertes Wissen, um aus Fragmenten (eine Zahl, ein Spaltenkopf,
ein Tabellenausschnitt, ein Bild ohne Textschicht) forensisch auf den Ursprungs-
datensatz zu schließen — und lässt sich das als **Kette von Bedingungen** fassen?

**Kurzantwort:** Es gibt die Bausteine, aber **kein Feld**, das genau diese Kette
beschreibt. Vier Literaturen berühren sich:

## 1. Provenance und Lineage

| Arbeit | Beitrag |
|---|---|
| *Conceptual Modeling of Data with Provenance* (2000), doi:10.15760/etd.133 | Provenance als Modell, nicht als Nachweis |
| *Provenance in bioinformatics workflows* (2013), doi:10.1186/1471-2105-14-s11-s6 | Workflow-Lineage: woher ein Wert kam, wenn die Pipeline bekannt ist |
| *Data Lineage Inference* (2024), arXiv:2411.15796 | Lineage aus Daten selbst **inferieren** — der verwandteste Ansatz zur Fragestellung, dort Datenschutz-Motivation |

Gemeinsamer Nenner: Lineage setzt **Metadaten** voraus. Fehlt die Provenance, beginnt
das eigentliche Problem — und dort hört diese Literatur auf.

## 2. Schema Matching und Dataset-Fingerprints

| Arbeit | Beitrag |
|---|---|
| *Dataset proximity mining for supporting schema matching* (2021), doi:10.5821/dissertation-2117-345323 | Datensätze über Ähnlichkeit paaren, ohne gemeinsames Schema |
| *In Situ Neural Relational Schema Matcher* (2024), doi:10.1109/icde60146.2024.00018 | Spalten zuordnen aus Werten + Kopf |
| *Semi-Automatic schema matching* (2018), doi:10.5281/zenodo.1419495 | Überblick über Matcher und ihre Grenzen |

Das ist die Technik hinter „derselbe Spaltenkopf, andere Datei" — aber sie liefert
**Ähnlichkeit**, keinen Herkunftsnachweis.

## 3. Tabellen ohne Textschicht

| Arbeit | Beitrag |
|---|---|
| *DeepDeSRT* (2017), doi:10.1109/icdar.2017.192 | Struktur (Zeilen/Spalten) aus Tabellenbildern |
| *ClusTi* (2021), doi:10.1007/s11036-021-01759-9 | Clustering-Verfahren für Tabellenstruktur |
| *Table Structure Recognition in Scanned Images* (2020), doi:10.1007/978-3-030-63083-6_12 | Gescannte Tabellen, Zellzuordnung |
| *Table Recognition in Scanned Documents* (2022), doi:10.1007/978-3-031-16014-1_58 | Überblick |

Damit ist die **Position** eines Werts in einer Bildtabelle rekonstruierbar — die
Bedingung „dieser Wert steht in Spalte X" ist also maschinell prüfbar, ohne OCR des
Fließtexts.

## 4. Forensik: Carving und Fragment-Klassifikation

| Arbeit | Beitrag |
|---|---|
| *Carving contiguous and fragmented files* (2007), doi:10.1016/j.diin.2007.06.017 | Dateien aus Fragmenten zusammensetzen |
| *Systematic Literature Review on Data Carving* (2013), doi:10.1016/j.protcy.2013.12.165 | Überblick |
| *Transformer-Based File Fragment Type Classification* (2025), doi:10.34190/eccws.24.1.3552 | Fragment → Dateityp |

Diese Literatur rekonstruiert **Bytes**, nicht **Bedeutung**. Sie sagt „das ist HDF5",
nicht „das ist die ICRF3-SX-Tabelle von 2019".

## 5. Reuse und Zitierbarkeit

| Arbeit | Beitrag |
|---|---|
| *The Anatomy of a Data Citation* (2011), doi:10.7710/2162-3309.1035 | Was eine Datenzitation leisten muss |
| *On the Reuse of Scientific Data* (2017), doi:10.5334/dsj-2017-008 | Wiederverwendung als eigene Praxis, oft ohne Lizenzbezug |

## Die Kette von Bedingungen — was fehlt, aber baubar ist

Die Literatur liefert je Glied etwas; die **Verkettung** ist der eigene Beitrag:

1. **Namensbedingung.** Ein Produktname ist oft selbst eine Koordinate:
   `2357-326` heißt RA 23h57m, Dec −03°26′ (B1950). Der gemessene Wert
   (RA 0,0850°, Dec −32,35°) liegt **nicht** in diesem Fenster — er passt zur
   B1950-Namenskonvention nur nach Präzession. Solche Widersprüche sind Filter,
   keine Fehler.
2. **Spaltenbedingung.** Ein Wert `v` kann nur in Spalte `j` stehen, wenn
   `range_j` ihn enthält; bei mehreren Kandidaten entscheidet die Anzahl der
   passenden Zeilen (Eindeutigkeit).
3. **Einheitenbedingung.** `m3/s` schließt einen Temperaturwert aus; die
   Einheit im Kopf ist stärker als die Zahl.
4. **Zeitbedingung.** MJD-Fenster, Monotonie der Zeitachse, Epochenbezug
   (B1950 vs. J2000) — ein Wert außerhalb des Fensters ist ein anderer Datensatz.
5. **Strukturrbedingung.** Spaltenzahl, Trennzeichen, Zeilenzahl, Dateigröße
   (z. B. `3 061 447 552 B` für eine EDGES-Tagesdatei).
6. **Byte-Bedingung.** IEEE-754-LE-Bytes eines Werts müssen im Kandidaten
   vorkommen (`pattern=<hex>`).

Die Kette ist ein **Constraint-Satisfaction-Problem**: jede Bedingung schneidet
Kandidaten weg; ein Treffer ist erst dann ein Beleg, wenn **alle** Glieder halten.
Genau das macht der Sniffer, wenn er `bait` + `values` ausgibt: er liefert die
Bedingungen, nicht das Urteil.

## Was daraus für die Praxis folgt

- Für **Texttabellen** ist die Kette stark (Kopf + Einheit + Spaltenzahl).
- Für **Bildtabellen** braucht es die Struktur-Rekognition (Abschnitt 3), sonst
  fehlt die Spaltenbedingung.
- Für **Binärdateien** entscheidet die Byte-Bedingung, nicht der Dateiname.
- Ein **einzelner Wert** ist nie ein Beweis — er ist eine Hypothese, die an
  Struktur, Einheit und Zeit scheitern kann.

## Anker-Merge: die Weberin (Prototyp `bin/weave`)

Die Bedingungskette braucht keine **identische** Tabelle. Sie braucht **Anker**:

- **Reihenfolge egal** — die Spalte wird ueber den normierten Kopf gefunden
  (`Discharge [m3/s]` → `discharge`), nicht ueber die Position.
- **Teilmengen erlaubt** — jede Quelle bringt, was sie hat; fehlende Werte
  bleiben leer, nicht null.
- **Kollisionen benannt** — gleiche Spaltennamen aus zwei Dateien werden zu
  `spalte@datei`, und die Herkunft steht in `<out>.map`.
- **Union der Zeilen** — der Anker ist der Schluessel; Quellen mit
  disjunkten Ankern ergeben eine Vereinigung, keine Schnittmenge.

`tools/utils/src/bin/weave.rs`:

```
weave --on time --out merged.csv a.csv b.csv
→ time,temp,flow
→ time 1: a liefert temp, b liefert flow
→ time 3: nur b
```

Drei Tests decken die Kernfaelle (Einheiten im Kopf, Anfuehrungszeichen,
ungleiche Reihenfolge/Teilmengen). Damit ist „aus verschiedenen Daten das
gewuenschte Endprodukt mergen" ein Werkzeug, kein Wunsch — die Grenze bleibt:
der Anker muss in **jeder** Quelle existieren, sonst wird die Tabelle
uebersprungen und das wird gemeldet, nicht verschwiegen.
