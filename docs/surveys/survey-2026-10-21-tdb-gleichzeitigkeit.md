<!--
  title: Survey — Gleichzeitigkeit auf der TDB-Uhr: der naive MJD-Vergleich und der Rømer-Versatz (2026-10-21)
  class: survey
  date: 2026-10-21
  sha256: 9400332ee4e088cbbbf50290e943116c6f68d52e5f5927e6afbcb99358b5fa3f
  status: live
  owner: river
  ack: future 2026-10-21T12:35Z
  see-also: tools/measure/src/bin/tdb_coincidence_probe.rs src/archivar/lsk.rs src/archivar/motion.rs src/archivar/membrane.rs
-->
# Survey — Gleichzeitigkeit auf der TDB-Uhr (2026-10-21)

**Frage.** Zwei Ereigniszeiten gelten als gleichzeitig, wenn ihre rohen
MJD-/UTC-Zahlen nah beieinanderliegen. Wie weit liegt dieses naive Fenster
daneben, sobald man auf die TDB-Uhr wechselt und die Richtung der Sichtlinie
(Rømer, Lichtlaufzeit) mitnimmt?

## Kette (gemessen, keine erfundenen Zahlen)

| Glied | Quelle im Code | Funktion |
|---|---|---|
| UTC → TDB | `src/archivar/lsk.rs` | `LeapSeconds::unix_to_tdb` (eingebettete `naif0012.tls`) |
| Leap-Tabelle | `src/archivar/membrane.rs` | `embedded_lsk` |
| MJD → Unix | `src/archivar/jwst.rs` | `mjd_to_unix` |
| Bahnposition | `src/archivar/motion.rs` | `body_barycenter_position` auf `ephemeris_sun.bin`, `ephemeris_earth.bin` |
| Rømer-Term | `tools/measure/src/bin/tdb_coincidence_probe.rs` | `roemer_term_s` = `n · (r_Erde − r_Sonne) / c` |
| Ephemeriden | `omegaflow-work/sources/ssd.jpl.nasa.gov/` | lokal gelesen; Netz nur als Fallback |

Der Beobachter ist der **Erd-Baryzentrum**, die Referenz die **Sonnenposition**.
Die Diurnal-Verschiebung einer Bodenstation bleibt eine benannte Schranke
(Erdradius/c, unten gemessen), kein stiller Term.

## Kommando

```
cd /home/johannes/projects/omegaflow-work
/home/johannes/projects/omegaflow/target/debug/tdb_coincidence_probe \
  --ra1 0 --dec1 0 --t1 61334.0 \
  --ra2 180 --dec2 0 --t2 61334.0 --window-s 60
```

`t` ist MJD/UTC (Default), `--kind` erlaubt `mjd|jd`, `--scale` erlaubt
`utc|tai`. Das Binary akzeptiert auch eine Paar-Datei (`--file`).

## Messung 2026-10-21 (MJD 61334.0 = 2026-10-21T00:00:00Z)

| Größe | Wert |
|---|---:|
| TDB − UTC zur Epoche | **+69.184 s** (+69 184.000 ms) |
| Ereignis A | RA 0°, Dec 0°, MJD/UTC 61334.0 |
| Ereignis B | RA 180°, Dec 0°, MJD/UTC 61334.0 |
| Roemer A | +441.726648 s |
| Roemer B | −441.726648 s |
| **naive Rohuhr-Differenz A−B** | **0.000000 s** |
| **TDB+Rømer-Differenz A−B** | **883.453295 s** |
| **Fehler des naiven Fensters** | **883.453295 s (883 453.295 ms)** |
| Urteil bei 60 s Fenster | **separated** |

Das naive Fenster sagt „gleichzeitig“ (0 s), die ehrliche Uhr sagt
**883.45 s auseinander** — bei zwei entgegengesetzten Sichtlinien und
derselben Ankunftszeit. Die Differenz ist der projizierte
Zwei-Sichtlinien-Rømer-Term; die Term-Budget-Zeile des Binarys nennt
2 AU/c = **998.0 s** als obere Amplitude und einen Erd-Sichtlinien-Drift
von **60.1 s/Woche**.

## Gegenproben

| Paar | naiv | TDB+Rømer | Aussage |
|---|---:|---:|---|
| gleiche Richtung (RA 0/0), 100 s Abstand | −99.999999 s | −99.995288 s | Rømer fällt fast heraus; Rest **4.711 ms** |
| UTC vs. TAI, derselbe physikalische Moment | `riss` (Skalen verschieden) | **0.000000 s** | TAI-UTC wird korrekt herausgerechnet, nicht als Himmelszeit verkauft |
| entgegengesetzte Richtung, gleiche Zeit | 0.000000 s | 883.453295 s | Richtungsabhängigkeit ist die ganze Lücke |

Der TAI-Fall zeigt die zweite Ehrlichkeitsregel: ein Vergleich über
verschiedene Rohskalen wird **nicht** stillschweigend gebildet, sondern als
`riss` benannt; erst die TDB-Faltung macht ihn messbar.

## Term-Budget (vom Binary aus Konstanten gedruckt)

| Term | Wert |
|---|---:|
| einfache Sichtlinie (1 AU/c) | 499.0 s |
| entgegengesetztes Paar (2 AU/c) | 998.0 s |
| Erd-Sichtlinien-Drift | 60.1 s/Woche |
| Diurnal-Schranke (Erdradius/c) | ≤ 21.3 ms |

## Status der Kette

- **live / vollständig:** UTC→TDB, Leap-Tabelle, Sonne- und Erde-Ephemeris,
  Rømer-Projektion, naives Fenster und korrigierte Differenz.
- **absent:** nichts in der gemessenen Kette. Die Ephemeriden liegen lokal;
  fehlen sie, fällt das Binary benannt auf `EphemerisVoid`/`pending` zurück
  und erfindet keinen Rømer-Wert.
- **pending:** der eingebettete `sha256` dieser Tafel (setzt der Operator);
  ferner die Bodenstations-Parallaxe (nur als Schranke 21.3 ms geführt) und
  Shapiro/Proper-Motion (nicht Teil dieses Funkens).

## Artefakt

`tools/measure/src/bin/tdb_coincidence_probe.rs` — 7 Unit-Tests:
`cargo test -p omegaflow-measure --bin tdb_coincidence_probe` grün,
`cargo build -p omegaflow-measure --bin tdb_coincidence_probe` grün.
