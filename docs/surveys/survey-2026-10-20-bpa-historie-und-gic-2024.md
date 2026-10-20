<!--
  title: BPA-GIC: rollierendes Fenster, Wayback-Historie und die unabhängigen Linien 1989/2024
  class: survey
  date: 2026-10-20
  sha256: 1abdd2e92fe2d7d37238a5e2939aa4128960c28c31c2141ed5e4321bf282f496
  status: live
  see-also: auftrag/auftrag-2026-10-20-sonne-erde-faeden.md docs/surveys/survey-2026-10-20-netzausfall-kante.md
-->
# BPA-GIC: rollierendes Fenster, Wayback-Historie und die unabhängigen Linien 1989/2024

**Was das ist.** Die Signatur der BPA-Messung ist nicht der Dateiname, sondern der
Dateiinhalt: die Kopfphrase, der 5-Minuten-Takt, die Punkt-IDs und die Pacific-Time-
Zeitkodierung. Darüber lässt sich prüfen, ob eine Kopie dieselbe Messung ist — oder
eine andere Reduktion. Ergebnis: **ein BPA-Archiv gibt es nicht**, aber zwei
unabhängige Linien für dieselben Stürme.

## 1. Die Quelle selbst

| Merkmal | Wert |
|---|---|
| Endpunkt | `transmission.bpa.gov/business/operations/gic/gic.txt` |
| Messung 2026-10-20 | **200**, 81.921 B (später 81.976 B), `Last-Modified` am selben Tag |
| Inhalt | 1.152 vollständige 5-Minuten-Zeilen, 11 Stationsspalten (A) |
| Kopfphrase | `Transformer Geomagnetically Induced Current` |
| Takt | `at 5-minute intervals, last 4 days` |
| Punkt-IDs | 155882, 64096, 151750, 146807, 61160, 64101, 142578, 170797, 56848, 128265, 35738 (Stand 2026) |
| Zeit | `17Oct2026 23:55, Pacific Time` |
| Quelle | `BPA SCADA system` |

**Die Zeitkodierung ist der Prüfstein.** Eine Datei mit denselben IDs, aber UTC
oder 1-Minuten-Takt, ist **nicht** dieselbe Messung — sie wäre eine andere
Reduktion. Das ist der `NAH`-Fall.

**Korrektur an der Signaturliste:** `198901` kommt in **keinem** der 20 Snapshots
vor. Die erste Punkt-ID ist 2013/2016 `62259`, ab 2020 `155882`. Die übrigen IDs
stimmen.

## 2. Wayback: 20 Snapshots, kein Mai 2024

| Weg | Befund |
|---|---|
| `gic.txt` | **20 Snapshots, 2013-10-08 … 2026-04-14**, je 4 Tage |
| 2013-Snapshot (verifiziert) | 1.159 Zeilen, 60.883 B |
| `gic/`-Verzeichnis | nur `gic.aspx`, `gic.txt`, `gic.png` |
| Mai/Juni 2024 | **kein Snapshot**, beide Pfad-Schreibweisen (`Business`/`business`) geprüft |

Gesichert sind damit **~80 Tage GIC über 13 Jahre**, mit Sturmfenstern 2016-12,
2017-02, 2021-03, 2023-03, 2024-07/09/12, 2025-05, 2026-04. Das ist eine
**Stichprobe**, kein kontinuierliches Archiv.

**Ernte 2026-10-20 (ausgeführt):** 20 von 20 Snapshots geladen, je **1.159 Zeilen**;
Kopfphrase und 5-Minuten-Takt in **allen 20** vorhanden; `sha256` je Datei unter
`sources/web.archive.org/bpa-gic/`. Die Spalten wechseln: 2013 `Sta 1…12` (ohne
Namen), ab 2016 Stationsnamen (Eugene, Spokane, The Dalles …), ab 2020 mit
`155882` als erster Punkt.

**Offen:** Rückdaten bei `TOT-OpInfo@bpa.gov` (steht im Dateikopf) anfragen.

## 3. Mai 2024 (Gannon/G5): nicht bei BPA, aber unabhängig gemessen

| Quelle | Was | Lizenz |
|---|---|---|
| NERC, *May 2024 GMD Event Review* | Industrie-Review mit Messungen | NERC-Dokument |
| arXiv **2507.07009**, *GIC-Related Observations During the May 2024 Geomagnetic Storm in the US* | Beobachtungen | Paper |
| **Zenodo 10.5281/zenodo.22730333** | **411 GIC-Onsets** in den USA, inkl. Gannon; `sea_curves.csv`, Notebooks | **CC-BY-4.0** |

Gemessen am Datensatz (Record 22730333, publiziert 2026-09-13):

- sieben meldepflichtige Stürme 2024, darunter **10.–12. Mai 2024** (G5, Dst −412 nT, X5.8);
- Onsets je Ereignis: 137, 45, 47, 149, 11, 10, 12 = **411**;
- Stationsabdeckung je Ereignis 18–329, Breiten 26,1°–48,0°;
- `event_metadata.csv` führt Datum, G-Level, Flare-Klasse, min. Dst, max. Kp.

**Das ist eine andere Reduktion** — Onset-Zeiten und Superposed-Epoch-Kurven, nicht
die 5-Minuten-BPA-Reihe. Also **kein Ersatz, sondern eine zweite, unabhängige
Messung desselben Sturms**: `contributes`, nicht `mirror-of`.

## 4. USGS: die historische Ausfall-Kante, CC0

| Merkmal | Wert |
|---|---|
| Titel | *North American electricity power-grid and communication-network anomalies for several magnetic storms* |
| DOI | **10.5066/P9N4DVNT** (ScienceBase-Item `61e847e0d34e8b818ad8947f`) |
| Autoren | Love & Murphy, 2022 |
| Lizenz | **CC0 1.0** |
| Abdeckung | März 1940, August 1972, März 1989, Stürme 1946–2000; GIC-Statistik 1969–1972 |
| Dateien | `1989.csv` (229 Zeilen), `1972.csv` (125), `Albertson.csv` (65), `Other-anomalies.csv` (62), `1940-c.csv` (78), `1940-e.csv` (24) |

**Grenze:** Die Koordinaten stehen **nicht** in den Quellen. Sie stammen aus einer
DHS-Anlagenliste, aus georeferenzierten Papierkarten oder aus Internetkarten —
geschätzt, nicht gemessen. Und die Liste endet **2000**: sie paart nicht mit ECOD
(2014–2023) oder dem 166-Sturm-Komposit, sondern liefert eine **eigene,
historische Kante** (1989: Hydro-Québec).

Lokal gesichert unter `sources/www.sciencebase.gov/61e847e0d34e8b818ad8947f/`
(7 Dateien, 116.145 B; sha256 je Datei im Kopf der Ablage).

## 5. Was das für die Kette ändert

| Faden | vorher | jetzt |
|---|---|---|
| Strom: BPA | „nur Live-Fenster" | Live **+ ~80 Tage Wayback** (20 Snapshots) |
| Strom: Mai 2024 | Lücke | **Zenodo 22730333** (411 Onsets, CC-BY-4.0) — unabhängige Linie |
| Netz: Ausfall-Kante | nur ECOD 2014–2023, negativ | **+ USGS 1940–2000** (CC0) als historische Linie |

**Nicht geschlossen:** die rohe BPA-5-Minuten-Reihe für Mai 2024. Sie bleibt
`absent`, bis BPA antwortet. Die Ausfall-Kante hat jetzt aber Daten — und zwar
zwei unabhängige, die man getrennt führen muss.
