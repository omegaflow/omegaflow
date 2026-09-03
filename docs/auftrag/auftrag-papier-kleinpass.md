<!--
  title: Auftrag — Papier-Kleinpass nach dem Merge
  class: auftrag
  date: 2026-08-30
  status: pending
  see-also: docs/paper/corona-heating-ladder.md docs/paper/gic-causal-driver.md docs/paper/lead-geometry-direction.md docs/paper/big-bang-echo-sheet-12.md docs/paper/dark-flow-sheet-8.md docs/paper/signal-cone-audit-sheet.md docs/paper/flyby-path-1-cold-cases.md docs/paper/laic-arrow-direction.md docs/paper/ground-sources-20s-band.md
-->

# Auftrag: Papier-Kleinpass (nach dem Merge)

## Zweck

Nach dem Merge (`auftrag-merge-fix-welle.md`) werden die bekannten
Kleinpass-Punkte je Blatt abgearbeitet — gemessen, nicht aus dem Bauch.

## Kernregel (0 honored)

Keine Zahl ohne Tabellen-/Registermarke. Der Atom-10-Check zuerst (siehe
lead-geometry). Jede Korrektur gegen das Register, nicht gegen die Erinnerung.

## Die Punkte je Blatt

- **corona:** 5 Punkte, v. a. „steepest 5.47→5.57" (bestätigt falsch).
- **gic:** 1378↔1260 statt 1321; §-Nummerierung; six pairs; p-Wert.
- **lead-geometry:** 100/99; **Atom-10-Check zuerst** — die Richtungs-
  asymmetrie 19:7 ist mit einer Geometrie gemessen, die das Register selbst
  als richtungsagnostisch führt („rückwärts gespiegelte TE-Bedingung,
  Leakage, falsche Stille", Commit 5427beb). Das ist die erste
  Reviewer-Frage.
- **urknall/dunkler-fluss:** Vollpräzision aus Register, 4,8×→5,3×, Kommata,
  BK18 r<0,036, Pixelindex-Instrumentenabsatz.
- **signalkegel:** 492,0 s vs. 487,7 s (auch auf dem Branch ungefixt);
  „holds"→„not tested".
- **flyby-1:** mm/s-Spanne inkl. negativer Werte.
- **laic:** negativer Stack als Befund mit Confounder-Satz; CDN-Formulierung.
- **boden-quellen:** Vollständigkeitsgrenze.

## Lieferung

Je Blatt die korrigierten Zahlen/Formulierungen, committed, mit Register-
Anker. Der Nummern-Audit (`auftrag-maschinen-audits.md`) läuft vor jedem
weiteren Rewrite.

## Abschluss

Alle Punkte abgearbeitet = „~12 klein-Pass entfernt" ist eingelöst.
