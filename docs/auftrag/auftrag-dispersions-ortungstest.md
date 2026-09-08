<!--
  title: Auftrag — Ortungs-Test: ein Ereignis, drei Kegel, die Sonne als Kalibrier-Referenz
  class: auftrag
  date: 2026-09-08
  probe-commit: pending
  sha256: 22b698730ec97a99926b69bc9d092f3ffd722c0f21417a66e0be95051c88ffe2
  status: pending
  see-also: docs/auftrag/auftrag-dispersionsrelation.md docs/TODO.md
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
