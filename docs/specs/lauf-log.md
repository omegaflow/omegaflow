<!--
  title: Maschinen-Audit-Lauf-Log — kalibrierte Läufe der Audit-Werkzeuge
  class: register
  date: 2026-08-31
  status: live
  sha256: 3f09742d5e47d19919f9bbae2592d5d29b6dd9b11caed983e9602166c9122fa6
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

- **Lauf 5 — Nummern-Audit (R2/Z-Klasse), Blockstand vermessen (abgenommen):**
  0 Z-Funde, 3 verpasst (die drei Korpus-Zeilen), 0 erfunden — weder die
  Tabellen-n- noch die Nomen-Regel findet einen Z-Fund, ohne erfunden zu
  melden (a/b); die drei Korpus-Zeilen sind zudem im Baum gefixt (c). (a)
  `§2-Zählung ↔ Tabellen-n`: der Live-Korpus (36 Blätter, Vorgabepfad
  `docs/paper/*.md`) trägt genau eine n-Spalte (solar-cycle-dynamo.md:354,
  n = 189 Monate, HMI/WSO-Überlappung); die §2-Zahlen desselben Blatts sind
  andere Serien und Körnungen — 18 Tage (l. 68), 10 Tage (l. 79), 1976
  (l. 124), 32 Jahre (l. 136), 1986/16 %/34 Jahre (l. 139/140), 3 Zyklen
  (l. 154), 217 Rotationen (l. 176) → 9 Treffer, 0 echte, alle in
  solar-cycle-dynamo.md. (b) `gleiches Zähl-Nomen, zwei Werte`: 91
  Nomen-Treffer in 26/36 Blättern; mit Qualifier-Filter („paired", „common
  window", „per ", „each", „max" …) noch 62 in 20/36. (c) Die drei
  Korpus-Zeilen sind im Baum gefixt und nicht mehr nachlaufbar: solar-cycle
  313 Läufe × 36 d = 11 268 d ≈ 30,9 a gegen 1995-06-29 → 2026-03-30
  (30,75 a), 603 Monate ≈ 50,25 a gegen WSO 1976–2026, 402 Monate ≈ 33,5 a
  gegen BiSON 1992-01 → 2025-01 — drei Serien, drei Körnungen; signalkegel
  trennt 79 a F10.7 und n ≈ 3824 Tage gemeinsames Fenster ausdrücklich
  (signal-cone-audit-sheet.md:61); gic 1378 (l. 84) vs 1260 (l. 145/311) ist
  „n paired" nach Lag-Einbettung. Kollision: R5 meldet
  gic-causal-driver.md:84 `1378` und :145 `1260` bereits als unverankerte
  Zählungen; R3 meldet broken-null-control.md:71/141/168 `1190/909/834` (drei
  Fenster desselben Paars) als Doppel-Zählung (9 R3-Funde, Lauf 4). Ein
  Z-Fund braucht die Archiv-Zählung (Records/Zeilen des Harvests) als
  Grundwahrheit, nicht die Syntax. R2 bleibt der Archiv-File-Count
  (`9aad4407f`); Regression 16/16 grün.
