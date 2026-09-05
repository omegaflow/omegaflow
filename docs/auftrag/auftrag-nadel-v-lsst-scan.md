<!--
  title: Auftrag — Nadel V: LSST-Live-Galaxie-Scan der Technosignatur
  class: auftrag
  date: 2026-09-05
  status: pending
  sha256: f40f86b2cd415e1abcb949f17015b02c84324c35e43886470f5419be8a68ddb3
  see-also: docs/concepts/kybernetische-astrophysik.md
-->

# Auftrag: Nadel Ⅴ als LSST-Live-Galaxie-Scan wieder öffnen

## Zweck

Nadel Ⅴ (achromatische Opazitäts-Anomalie) war als struktureller Techno-Scan
über die ZTF/WISE-Epoche geschlossen (2026-08-28, Kandidat ausgeschlossen,
Limit 0 honored). LSST läuft seit 29.06.2026 (~1,4 Mio Quellen/Woche, 10 a) —
die wachsende Live-Fläche, auf der der achromatische Dip-Scan in Galaxie-
Maßstab fortgesetzt wird. Dies ist der Galaxie-Scan der Technosignatur,
getrennt von der Atmosphären-Biosignatur (Nadel ⅩⅢ).

## Umfang

1. **LSST-Live-Zugang messen:** ob der Broker-Stream (Alert-Stream der
   LSST-Broker) maschinenlesbar erreichbar ist (HTTP prüfen, nicht annehmen),
   welche Quellen/Woche er trägt, und ob er den achromatischen-Dip-Test über
   die g/r/i-Z-Bänder trägt. Die ztf_anomaly_probe-Logik (Achromatizitäts-
   Ratio, DIP_SIG, Ausschluss-Filter) auf die LSST-Mehrband-Lichtkurven legen.
2. **Nicht-periodische, achromatische Dips über der wachsenden Fläche suchen**
   (das Kriterium der Nadel Ⅴ: OA = Variabilität_erwartet / Chromatizität, mit
   dem Ausschluss natürlicher Klassen).
3. **IR-Exzess-Kreuz** (IRAS/AKARI → LSST-Ära-WISE/LSST-IR) für den
   Doppel-Anomalie-Katalog.

## Kernregel (0 honored)

Jede Live-Quelle erst per HTTP verifizieren, bevor sie zählt. Ein Dip ohne
Ausschluss natürlicher Klassen ist kein Kandidat. Die Stille über der
wachsenden Fläche ist ein quantitatives, wachsendes Limit — ebenso eine
Messung.

## Lieferung

Quellen-Befund (LSST-Broker erreichbar?) + der wieder-geöffnete Scan auf einer
realen ersten LSST-Woche, committed. Nadel Ⅴ von „GESCHLOSSEN" auf die Live-
Runde gestellt.
