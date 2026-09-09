<!--
  title: Auftrag — Ortungs-Test: ein Ereignis, drei Kegel, die Sonne als Kalibrier-Referenz
  class: auftrag
  date: 2026-09-08
  probe-commit: b0b7b7d
  sha256: 9cf49d764b7caf5191bbc8bef976993bdd7480aa1cdef67ed3698b95a2d8eaeb
  status: archived
  see-also: docs/auftrag/auftrag-dispersionsrelation.md docs/TODO.md docs/befund/befund-2026-09-09-dispersions-ortungstest.md
-->

# Auftrag: der Ortungs-Test (Kreuzprobe der Dispersion)

## Zweck

Die gemessene Band-Latenz muss das Ereignis auf den bekannten Sonnenort
abbilden. Ein solares Ereignis, drei Kegel (drei unabhängige Mess-Wege mit
eigener Latenz), die Sonne als Kalibrier-Referenz (bekannter Ursprung aus
Ephemeriden), Fehlerkreis Δt·c/Basis. Ortung statt Wartung: das Dispersion-
Verdikt wird von außen geprüft, nicht von innen bestätigt.

## Eingaben

- **Ein** solares Ereignis (zu identifizieren aus dem gemessenen Bestand,
  nicht erfunden — kein Ereignis ohne Beleg).
- **Drei** Kegel: drei unabhängige Wege mit eigener gemessener Latenz.
- Die **Sonne** als Kalibrier-Referenz: bekannter Ort zur Ereigniszeit.
- Die Basis zwischen den Kegeln; die Skala des Fehlerkreises ist Δt·c/Basis.

## Vorgehen

Je Kegel: gemessene Ankunftslatenz → Abstand über die Band-Geschwindigkeit.
Der Schnitt der drei Kreise ist der Fehlerkreis; verglichen wird gegen den
Sonnenort, nicht gegen eine Erwartung.

## Verdikt

Benannt, was gemessen wurde: die Ortung trifft den Kalibrier-Ursprung
innerhalb des Fehlerkreises, oder der Offset ist die Zahl des Befunds — ein
Befund trägt keine Deutung.

## Lieferung

Befund-Blatt (`docs/befund/`, `class: befund`, `status: done`): die drei
Kegel, die gemessenen Δt, der Fehlerkreis, die Antwort gegen die Sonne.
Register-Zeile im Register des Auftrags.

## Selbsttragend

Dieser Auftrag ist das einzige Blatt der Kybernautin — er trägt alles, sie
trägt nur ihn (Kontext-Hygiene). `probe-commit` wird gefüllt, sobald die
Dispersions-Probe committet ist; bis dahin `pending`.

## Register

- 2026-09-09: Befund `docs/befund/befund-2026-09-09-dispersions-ortungstest.md`
  (status: done) — ein Ereignis (Beleg: XRSB 24-s-Median-Spitze 4.647e-4 W/m²,
  2013-05-14 01:11:36, unix 1368493896, Rang 2 von 22 Kandidaten, skalenfrei
  gerankt); drei Kegel XRSA/94A/335A (Δt = 264.3 / 480.3 / 504.3 s; eps =
  −240.0 / −24.0 / −0.0 s); Fehlerkreis leer (Lücke 5.756e10 m); die Antwort
  gegen die Sonne: Offset 2.638e10 m = 88.0 s — die per-Kegel-eps sind die
  Zahlen des Befunds. Probe: `tools/measure/src/bin/dispersion_ortung_probe.rs`.

