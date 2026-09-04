<!--
  title: Auftrag — Sub-Hz-Drift auf der Quiet-Zone-Basis (P10 >50 AU)
  class: auftrag
  date: 2026-09-04
  status: pending
  sha256: 2ef716828b995a449736d4c7400c2e800c4afd572d4f12ea7fa6b08b002acbf9
  see-also: docs/auftrag/auftrag-dunkle-materie-front-c.md docs/auftrag/auftrag-vollmission-redution.md docs/paper/probe-front-dark-matter.md
-->

# Auftrag: Sub-Hz-Drift auf der Quiet-Zone-Basis (P10 >50 AU, 1991–2002)

## Zweck

Der wichtigste offene Lauf des Korpus: die erste Messung der Front, bei der
die ~1-Hz-Anomalie-Signatur *über* dem Median-Boden der Tagesmediane liegt.
Die Quiet-Zone-Isolation (`--zone`, e956712/c3660b7) hat den Median-Boden auf
0,21 Hz gesenkt (p50=0) — der sub-Hz-Drift auf der Zonen-Basis ist der erste
Konfiguration, in der die Anomalie über dem Median liegt. Der Lauf wird
gelesen werden (von dir, von künftigen Sessions); er verdient die volle
Disziplin der Front. Bewusst als **eigene, frische Session** niedergelegt —
diese Session trägt die komplette Front-Historie (42 Deduktionen, drei
Werkzeugumbauten) und ist kontextverschmutzt. Eine Session ist ein Strang;
isolierte Räume = isolierte Ontologie.

## Voraussetzungen (gemessen, nicht zu wiederholen)

- Zonen-Werkzeug steht: `pioneer_navio_negative_fuzzy --zone` (Commit
  e956712), der `--zone`-Pfad refaktorisiert (c3660b7).
- Zonen-Basis: `data/pioneer10_navio_subkhz_zone_daily.bin` (PNDM), **1036
  Tagesmediane**, Zone >50 AU, Era 1991–2002. Median |daily-med| **0,21 Hz**
  (p50=0), RMS der Tagesmediane **255 Hz**, p99 1021 Hz, sub-kHz 98,9 %.
- P11-Kontrolle: 15–30 AU, Era 1984–1990, 606 Tagesmediane, Median 6,8 Hz,
  RMS 304 Hz.
- **Das RMS ist das Ziel des Laufs, nicht der Median.** Der Median sagt „die
  Mitte ist sauber"; der Drift kämpft gegen die RMS-Streuung (255 Hz), nicht
  gegen den Median.

## Bausteine, in dieser Reihenfolge (eigene frische Session)

1. **Verdikt-Bindung vor dem Lauf** — vorregistriertes Protokoll, drei
   Ausgänge, alle als Befund vorbereitet, Schwellen aus der Null (Baustein 3)
   festgeschrieben, *bevor irgendein Modellwert fällt*:
   (a) ∝t²-Präferenz (Kraft-Signatur),
   (b) τ=126,5a-Abkling-Präferenz (Thermal, Amplitude frei — telemetriefrei,
   siehe Deduktion-41-Design),
   (c) keine Präferenz (Grenze).
   Ein Lauf ohne Bindung ist ein Lauf, dessen Ergebnis nachträglich
   interpretiert wird — der Unterschied zwischen flyby-Weg-2-Metrik und
   Post-hoc. Keine post-hoc-Interpretation.
2. **Maskierung vor Regression** — Deduktion-10-Disziplin auf die Zonentage
   (korrupte Cluster/Schwanz-Tage verwerfen, nicht mitteln). Die Zonen-
   Medizahlen (0,21 Hz, p50=0) sind *unmaskiert*; die RMS-Spalte (255 Hz)
   trägt die Zitter-Schwanz-Tage. Ohne Maskierung füttert der Schwanz die
   Steigung — die Fehlergeometrie, die eine Schein-Neigung fabrizieren
   könnte (asynchron verteilte Schwanz-Tage). Dokumentieren, wie viele der
   1036 Tage überleben — die n-Zahl gehört ins Ergebnis.
3. **Surrogat-Null** — Block-/phasen-erhaltendes Design für die dünne,
   autokorrelierte Tagesmedian-Reihe (benachbarte Tagesmediane sind nicht
   unabhängig; naive Phasenrandomisierung zerstört die Autokorrelation und
   fälscht die Schwelle — die Fisher-z-Lehre der Text-als-Daten-Runde, exakt
   derselbe Mechanismus). Null kalibriert an neutralen Segmenten der Zone.
4. **Regression** — linear/∝t²/τ=126,5a gegen die gemaskte Reihe; P10 primär,
   P11 (15–30 AU) als Kontrolle (unterschiedliche Ära, ähnliche Distanz — die
   Kovariaten-Trennung als Nebenbefund).
5. **Registrierung** — Ergebnis + n + Schwellen + Ausgang ins Register; Paper
   v4-Edit (Quiet-Zone-Absatz mit Drift-Verdikt, `probe-front-dark-matter.md`)
   danach, eigener Commit.

## Grenzen

Median ≠ Kampffeld (RMS ist es). Keine Erwartung vor dem Lauf; drei Ausgänge,
alle würdig. Keine Zahl ohne Messung; ein fehlendes Datum ist `pending` mit
Ort, nie ein erfundener Wert (0 honored).
