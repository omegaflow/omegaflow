<!--
  title: Auftrag — Technosignatur: Radio/Laser-Narrowband-Kanal
  class: auftrag
  date: 2026-09-05
  status: pending
  sha256: bba365606178e3428eced78c52139a6e107f70168e1de55be5de21edc8164da5
  see-also: docs/concepts/kybernetische-astrophysik.md docs/TODO.md
-->

# Auftrag: der Radio-/Laser-Narrowband-Kanal der Technosignatur

## Zweck

Nadel Ⅴ (achromatischer Dip) ist geschlossen. Der zweite klassische
Technosignatur-Kanal fehlt: eine **monochromatische** oder **narrowband**-Linie,
die kein natürlicher Prozess erzeugt — Radio-SETI (schmale Linien im kHz-Band)
und optisches SETI (Laser-Linien). Der Kanal ist die Umkehrung des Dips: nicht
abwesende Photonen, sondern eine Linie, die sich nicht auf Kontinuum + natürliche
Linien zurückführen lässt.

## Umfang

- Datenquellen erfassen: öffentliche Radio-Teleskop-Archive / Spektren-Dienste
  (nach SOURCE_PORT-Protokoll), optische Hochauflösungs-Spektren-Dienste.
- Ein Probe entwerfen, das eine Linie gegen die natürliche Linien-/Kontinuum-Null
  prüft — Achse = Frequenz, Signal = Überschuss über der lokalen Spektral-Null.

## Kernregel (0 honored)

Kein falsch-positiver Linien-Fund. TE ≈ 0 / fehlende Linie ist der ehrliche
Zustand, solange keine Quelle abgerufen ist — `pending`, nicht „still".

## Lieferung

Ein registrierter Quellen-Befund + ein Probe-Entwurf, committed. Der Kanal wird
damit zur dritten Technosignatur-Messung neben Dip und (Bio-)Disequilibrium.
