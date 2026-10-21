<!--
  title: Survey — SDSS DR16Q gegen die 88 ICRF-Anker ohne Gaia (2026-10-21)
  class: survey
  date: 2026-10-21
  sha256: c30e88306c7c60e1f9cb2970601270fd81f8cf7d1c1e29a4c9620af9906ba4fa
  status: live
  see-also: docs/surveys/survey-2026-10-21-gaia-dr4-vorbereitung.md docs/surveys/survey-2026-10-20-frame-wache.md tools/measure/src/bin/sdss_88_probe.rs
-->
# Survey — SDSS DR16Q gegen die 88 ICRF-Anker ohne Gaia

**Zweck.** Die Frame-Wache lässt **88 der 600 Dreifach-Anker** ohne
Gaia-CRF3-Zuordnung. Dieser Survey prüft, ob der SDSS-Quasar-Katalog DR16Q
diese Lücke schließt: 2″-Positionsmatching gegen `VII/289/dr16q`, Match-Rate,
Separationen und die Frage, ob SDSS-Positionen für Identifikation oder für
Rahmenarbeit tragen.

## Quelle und Zugang (gemessen 2026-10-21)

| | |
|---|---|
| Katalog | VizieR **VII/289** — SDSS quasar catalog, DR16Q (Lyke+ 2020, ApJS 250, 8) |
| Tabelle | `VII/289/dr16q` („SDSS quasar-only catalog from DR16") |
| Landeseite | `https://cdsarc.cds.unistra.fr/viz-bin/cat/VII/289` — HTTP **200**, **40.173 B** |
| Zeilen | **750.414** (`SELECT COUNT(*)`, TAP) |
| ASU-TSV | `https://vizier.cds.unistra.fr/viz-bin/asu-tsv` (Server `VizieR/7.6.1`, `MaxTuples=50000`) |
| TAP | `https://tapvizier.cds.unistra.fr/TAPVizieR/tap/sync` |

Beide Schnittstellen wurden gefahren; sie liefern **dieselben drei
Gegenstücke und dieselben Separationen**. Die ASU-Schnittstelle ist der
Standardweg im Werkzeug, TAP die Gegenprobe.

## Methode

1. **Eingabe:** 88 Zeilen `iers,ra_deg,dec_deg` aus
   `/tmp/opencode/gaia/missing88.csv` (erzeugt von `frame_wache_probe
   --missing-out`).
2. **Suche:** je Anker eine ASU-TSV-Abfrage mit `-c.rs=2` (2″ Radius) und
   `-out=recno,SDSS,RAJ2000,DEJ2000,z,zQN`.
3. **Auswahl:** kleinste Kugel-Distanz (Haversine) unter den Rückgaben; keine
   Nachbearbeitung der Katalogposition. Einheitenkontrolle: `-c.rs=0.05`
   liefert für 0146+056 keine Zeile, `-c.rs=0.5` die Zeile bei 0,096″ — der
   Radius ist in Bogensekunden, nicht in Bogenminuten.
4. **Abdeckungsprobe:** zusätzlich je Anker die Zahl der DR16Q-Zeilen im
   10′-Radius. Sie trennt „kein Gegenstück" von „Katalog deckt das Feld nicht
   ab".
5. **Radius-Kontrolle:** dieselben 88 Anker über TAP mit 60″-Radius. Es kommen
   **keine weiteren** Gegenstücke hinzu; der nächste Nicht-Match liegt
   **137,3″** entfernt (0046+316). Die 2″-Grenze ist damit nicht der Engpass.

## Ergebnis

**3 von 88** Ankern haben eine DR16Q-Zeile im 2″-Radius (Match-Rate
**3,41 %**).

| IERS | SDSS-Name | Separation | z | zQN |
|---|---|---:|---:|---:|
| 0146+056 | `014922.36+055553.4` | **0,0963″** | 2,348000 | 2,345494 |
| 0213-026 | `021542.01-022256.6` | **0,0677″** | 1,177594 | 1,171496 |
| 1222+216 | `122454.46+212246.3` | **0,0268″** | 0,433500 | — (leer im Katalog) |

| Größe | Wert |
|---|---:|
| Matches | **3 / 88** |
| ohne SDSS-Gegenstück | **85** |
| Median-Separation | **0,0677″ = 67,7 mas** |
| Mittel | 63,6 mas |
| Streuung (Stichprobe, n−1) | **34,9 mas** |
| Minimum / Maximum | 26,8 mas / 96,3 mas |

**Abdeckung, nicht nur Treffer.** DR16Q ist ein Quasar-Katalog, kein
Imaging-Katalog. Nur **10 der 88** Felder enthalten überhaupt eine
DR16Q-Zeile im 10′-Radius; **78** enthalten keine. In den 10 abgedeckten
Feldern liegt die Match-Rate bei **3/10**.

| IERS | DR16Q-Zeilen in 10′ | nächste Zeile | Match |
|---|---:|---:|---|
| 0046+316 | 12 | 137,3″ | nein |
| 0146+056 | 8 | 0,096″ | **ja** |
| 0213-026 | 10 | 0,068″ | **ja** |
| 1023+131 | 2 | 310,9″ | nein |
| 1101+384 | 6 | 424,0″ | nein |
| 1222+216 | 3 | 0,027″ | **ja** |
| 1413+135 | 3 | 465,1″ | nein |
| 1504+377 | 8 | 258,6″ | nein |
| 1600+335 | 11 | 189,3″ | nein |
| 1652+398 | 10 | 237,1″ | nein |

Die sieben abgedeckten Nicht-Matches haben ihren nächsten DR16Q-Nachbarn
zwischen **137,3″ und 465,1″** — das ist kein 2″-Radiusproblem, sondern ein
anderes Objekt beziehungsweise keine DR16Q-Zeile für die Quelle.

**Zufallstreffer.** In den 10 abgedeckten Feldern liegen 73 DR16Q-Zeilen in
10′-Kreisen, also **83,7 Quellen/deg²** (73 / 0,8727 deg²). Die erwartete
Zahl zufälliger 2″-Treffer über 88 Anker ist damit
`88 · 83,7 · π · (2/3600)²` ≈ **0,007**. Die drei Treffer sind keine
Zufallstreffer; die Abwesenheit der übrigen ist im abgedeckten Feld real.

## Reichen die SDSS-Positionen?

**Für Identifikation: ja.** Die drei Gegenstücke liegen 26,8–96,3 mas von der
ICRF-Position entfernt, bei einem Suchradius von 2″ und einer erwarteten
Zufallstrefferzahl von 0,007. Das ist eindeutig.

**Für Rahmenarbeit: nein.** Die gemessenen Abstände liegen zwei bis drei
Größenordnungen über der ICRF-Zielgenauigkeit (≈0,1 mas). DR16Q notiert
Positionen mit 10⁻⁶ deg (3,6 mas) Auflösung; die beobachteten Abstände von
27–96 mas sind aber das, was zwischen ICRF- und SDSS-Position übrig bleibt,
nicht der reine Katalogfehler. DR16Q taugt damit als
**Identifikationshilfe** für die 3 Anker, nicht als Referenz für eine
Rahmenrotation.

## Grenzen

- **3 Treffer sind keine Statistik.** Median und Streuung stehen auf n=3;
  die 85 Nicht-Matches tragen keine Separationsinformation.
- **DR16Q ist quasar-only.** 78/88 Felder haben keine DR16Q-Zeile im 10′.
  Das ist in erster Linie eine Abdeckungs-/Selektionsaussage, keine Aussage
  „es gibt dort keinen optischen Gegenpart".
- **Keine Eigenbewegung modelliert.** ICRF-Positionen sind Radio-Epochenmittel
  (überwiegend 1990er), SDSS-Positionen Imaging-Epochen. Bei 27–96 mas
  Gesamtabstand ist die Eigenbewegung nicht separierbar; für Identifikation
  ist sie irrelevant.
- **Ein Radius, eine Quelle, ein Katalog.** Kein zweiter optischer Katalog
  (z. B. Gaia DR3 für die 85) wurde gegengeprüft.
- **Kein Gaia-Bezug für die 3.** Ob die drei SDSS-Gegenstücke Gaia-Quellen
  sind, ist hier nicht gemessen.

## Reproduktion

```sh
cd /home/johannes/projects/omegaflow
cargo build --release -p omegaflow-measure --bin sdss_88_probe
./target/release/sdss_88_probe \
  --input /tmp/opencode/gaia/missing88.csv \
  --out /tmp/opencode/sdss88/asu_matches.csv \
  --raw-dir /tmp/opencode/sdss88/raw_asu \
  --footprint-arcmin 10
```

Einzelabfrage (Beispiel 0146+056, 2″):

```sh
curl -s "https://vizier.cds.unistra.fr/viz-bin/asu-tsv?-source=VII/289/dr16q&-c=27.343212%2B5.931547&-c.rs=2&-out=recno,SDSS,RAJ2000,DEJ2000,z,zQN"
```

TAP-Gegenprobe (Beispiel 1222+216, 2″ = 0,0005556 deg):

```sh
curl -s "https://tapvizier.cds.unistra.fr/TAPVizieR/tap/sync" \
  --data-urlencode "REQUEST=doQuery" --data-urlencode "LANG=ADQL" \
  --data-urlencode "FORMAT=csv" \
  --data-urlencode "QUERY=SELECT recno,SDSS,RAJ2000,DEJ2000,z,zQN FROM \"VII/289/dr16q\" WHERE 1=CONTAINS(POINT('ICRS',RAJ2000,DEJ2000),CIRCLE('ICRS',186.226910022,21.379552356,0.0005556))"
```

Das Werkzeug schreibt die volle Tabelle mit `n_cand`, `footprint_n`,
`footprint_nearest_arcsec`, `recno`, `SDSS`, `z`, `zQN` und den 85 leeren
Zeilen. `--raw-dir` cached die Rohantworten, sodass ein zweiter Lauf ohne
Netz dieselben Zahlen liefert.

## Ablage

- Werkzeug: `tools/measure/src/bin/sdss_88_probe.rs`
- Ankerliste: `/tmp/opencode/gaia/missing88.csv` (nicht dauerhaft)
- Vorgänger: `docs/surveys/survey-2026-10-20-frame-wache.md` (88/600 ohne Gaia),
  `docs/surveys/survey-2026-10-21-gaia-dr4-vorbereitung.md` (Anker-Pfad)

## Offen

- Die 78 nicht abgedeckten Anker brauchen einen anderen optischen Katalog
  (Imaging, nicht Quasar-Selektion), wenn die Lücke geschlossen werden soll.
- Die 7 abgedeckten Nicht-Matches sind Kandidaten für eine gezielte
  Einzelprüfung (Simbad-Typ, SDSS-Imaging ohne Spektrum).
- Die drei Treffer gegen Gaia DR3 prüfen, bevor sie irgendwo als
  Rahmenanker geführt werden.
