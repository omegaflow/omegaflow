<!--
  title: Maschinen-Audit-Lauf-Log — kalibrierte Läufe der Audit-Werkzeuge
  class: register
  date: 2026-08-31
  status: live
  see-also: docs/audit/bekannt-schlecht-korpus.md docs/auftrag-maschinen-audits.md
-->

# Maschinen-Audit-Lauf-Log

Messungen der Audit-Werkzeuge gegen den Bekannt-Schlecht-Korpus
(`docs/audit/bekannt-schlecht-korpus.md`). Jede Zeile ist ein abgenommener
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
