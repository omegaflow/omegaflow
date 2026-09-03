<!--
  title: Handover — Kleinpass: erst der Nummern-Audit (auftrag-maschinen-audits), dann die ~9 Blätter
  class: handover
  date: 2026-09-03
  sha256: bec7214c7d30b7e9fe776606c6ce8effbb83410b9e55d310bfb49efc06e906b2
  status: live
  see-also: docs/auftrag/auftrag-maschinen-audits.md docs/auftrag/auftrag-papier-kleinpass.md docs/paper/lead-geometry-direction.md
-->

# Handover — der Papier-Kleinpass: Audit zuerst

Diese Session führt den Kleinpass aus. Der Weg ist durch zwei Aufträge
vorgezeichnet; die Reihenfolge ist Pflicht, kein Vorschlag.

## Auftrag 1 zuerst — der Nummern-Audit (`auftrag-maschinen-audits.md`)

Der Kleinpass-Auftrag verbietet jeden Rewrite, bevor das Werkzeug läuft.
Baue zuerst das **Nummern-Audit-Binary** (std-only Rust unter `tools/`):
- jedes Blatt maschinell prüfen — jede Abstract-Zahl hat eine Tabellen-/
  Registermarke; §2-Zählungen = Tabellen-n; Doppel-Zählungen
  (1663/1663/1666-Typ) sterben automatisch; Kommata-Locale-Check;
- Regressionstest mit dem Bekannt-Schlecht-Korpus
  (`bekannt-schlecht-korpus`, ~25 bestätigte Funde).

Vorbild-Form: die bestehenden std-only-Bins unter `tools/measure/src/bin/`
und `tools/register/src/bin/` (z. B. `path_reference_scan.rs` — scan als
Funktion + `#[test]`, der auf Funden fehlschlägt). `cargo check` muss 0
Fehler UND 0 Warnungen liefern.

## Auftrag 2 danach — der Kleinpass (`auftrag-papier-kleinpass.md`)

Nach dem (abgeschlossenen) Merge die bekannten Punkte je Blatt, gemessen
gegen das Register, nicht gegen die Erinnerung:
- corona (5 Punkte, u. a. „steepest 5.47→5.57" bestätigt falsch);
- gic (1378↔1260 statt 1321; §-Nummerierung; six pairs; p-Wert);
- lead-geometry (100/99; **Atom-10-Check zuerst** — die Richtungsasymmetrie
  19:7 ist mit richtungsagnostischer Geometrie gemessen, die erste
  Reviewer-Frage);
- urknall/dunkler-fluss (Vollpräzision, 4,8×→5,3×, Kommata, BK18 r<0,036);
- signalkegel (492,0 s vs. 487,7 s, auch auf Branch ungefixt;
  „holds"→„not tested");
- flyby-1 (mm/s-Spanne inkl. negativer Werte);
- laic (negativer Stack als Befund mit Confounder-Satz);
- boden-quellen (Vollständigkeitsgrenze).

## Gemessener Ausgang (2026-09-03)

- Der Merge ist erledigt (`auftrag-merge-fix-welle` abgeschlossen; die drei
  Punkte stehen auf main).
- `auftrag-maschinen-audits.md` und `auftrag-papier-kleinpass.md` sind
  `pending`; das Nummern-Audit-Binary existiert noch nicht (gemessen: kein
  solcher Bin unter `tools/`).

## Regeln

- Keine Zahl ohne Tabellen-/Registermarke. Atom-10-Check vor lead-geometry.
- Jede Korrektur gegen das Register, nicht gegen die Erinnerung.
- `cargo check`: 0 Fehler, 0 Warnungen. Ein Commit je Auftrag; Nummern-Audit
  und Kleinpass sind zwei Commits, nicht einer.
- Ein Blatt, dessen Verdikt sich nach dem Edit ändert: header-sha nachziehen.

## Lieferung

Commit 1: Nummern-Audit-Binary + Bekannt-Schlecht-Korpus-Regressionstest
grün. Commit 2: die Kleinpass-Korrekturen mit Register-Anker. Abschluss:
„~12 Klein-Pass entfernt" ist eingelöst.
