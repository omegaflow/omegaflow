<!--
  title: Maschinen-Audit-Lauf-Log — kalibrierte Läufe der Audit-Werkzeuge
  class: register
  date: 2026-08-31
  status: live
  see-also: docs/specs/bekannt-schlecht-korpus.md docs/auftrag/archiv/auftrag-maschinen-audits.md
-->

# Maschinen-Audit-Lauf-Log

Messungen der Audit-Werkzeuge gegen den Bekannt-Schlecht-Korpus
(`docs/specs/bekannt-schlecht-korpus.md`). Jede Zeile ist ein abgenommener
Lauf mit **gefunden / verpasst / erfunden** — der Kalibrationsscore (§3 des
Auftrags). Ein Werkzeug wächst nur mit Messung: eine Relations-Klasse, eine
Toleranzregel, ein Korpus-Lauf, dann die Zahlen.

## Läufe

- **Lauf 2 — Nummern-Audit (Regel 3, D-Klasse), abgenommen:** 3/3 D-Funde auf
  planet-nine, 0 erfunden (klassisch 2854/2884, übrig 661/662, gestreut
  1663/1666). R1 überfeuert 17–39/Datei, R5 überfeuert 9–54/Datei → A- und
  N-Klasse nicht syntaktisch (Proxy unbrauchbar). R2 nicht implementiert.

- **Lauf 3 — Relations-Driver (C-steepest):** 1 gefunden, 0 erfunden,
  0 verpasst (einziger Fund seiner Klasse: corona:232 — Korpus-Fund #1
  erstmals maschinell reproduziert). Kontrollen sauber: Wert-Sense-Claims
  („strongest 5.11e-1", „max 12.96/297") nicht angefasst — zweite
  Relations-Art benannt, nicht implementiert (Operator-Grenze: eine Klasse
  vor der ersten Messung). TOL 3.5e-4, Claim-Fenster über Zeilengrenzen,
  4 Unit-Tests. Z-Klasse (R2) bleibt ungelöst dokumentiert. Der Driver
  wächst nur mit Messung: nächste Klasse = Wert-Sense („Zahl X ist
  Max/Min von Y").

- **Lauf 4 — Nummern-Audit (R4-single-sheet-Locale), abgenommen:** R4 fand im
  Live-Korpus (21 Paper-Blätter) 5 K-Locale-Blätter: big-bang-echo-sheet-12
  (18 Prosa-Kommas), dark-flow-sheet-8 (10), signal-cone-audit-sheet (6),
  twenty-second-band-ground-chain (42), flyby-path-1 (2 Tabellen-Kommas gegen
  Punkt-Prosa — der uneins-Fall, den die alte Regel nur bei beidseitigem Komma
  meldete). Die Tausender-Kontrolle („1,033 rows", „12,074 samples") und die
  Klammer-Set-Kontrolle („{94,335}" als Konfounder-Enumeration, corona) schweigen
  — corona, das die erste Fassung fälschlich meldete, ist nach der
  Klammer-Regel ruhig: R4-erfunden auf dem Live-Korpus = 0. R1 167 / R3 3
  (planet-nine, die drei D-Zeilen) / R5 408 (bekanntes Überfeuern, Lauf 2) /
  R6 19 Kandidaten. R2 (Z-Klasse) bleibt offen dokumentiert: jede syntaktische
  Regel kollidiert mit R3/R5 oder meldet die gefixten Blätter (gic §2.1
  n≈1378 vs §4.1 n≈1260 „paired") — ein Z-Fund braucht die Archiv-Zählung als
  Grundwahrheit. Regression 14/14 grün: Korpus-Integrität (29 Zeilen,
  A+Z+D+K = 21), R1/R3/R4/R5-Detektion, R4-Single-Sheet + die zwei
  R4-Kontrollen, V-Stille, Z-Grenze (getrennte Serien-Zählungen sind keine
  Doppel-Zählung). Die Korpus-Zeilen selbst sind nicht mehr nachlaufbar: die
  Original-Blätter wurden post-c94d431 durch den Kleinpass gefixt (nicht im
  Baum) — der Score misst Fixtures + den Live-Korpus.
