<!--
  title: Übergabe — Korona-Leiter & Kaskade (AIA-fam, Bandbreite, Volljahr, Richtungs-Test)
  class: handover
  date: 2026-09-06
  sha256: aede458ce54e1c91969d4c55d45a25db813a54802204e00a329eab59da5db40b
  status: live
  see-also: docs/paper/corona-heating-ladder.md docs/surveys/survey-ein-blatt-korona-heizung.md docs/TODO.md
-->

# Übergabe: Korona-Leiter & Kaskade

Diese Übergabe fasst den Zustand der Korona-Arbeit zusammen, damit eine einzelne
empfangende Session sie fortsetzen kann. Korona ist eines der drei zu
verschickenden Papers (GIC, Pioneer/20-s, Korona). Der Operator will **kein
Null-Paper** — die Messung soll bestimmen, was die Daten tragen.

## Ergebnis in einem Satz

Die koronale **Aufwärts-Kaskade 193→211→335→94 Å** (~96-s-Alfvén-Lag) ist ein
**reproduzierbares, gerichtetes, aber unter-fam Signal**: sie reproduziert sich
in drei unabhängigen Jahren (2013: 524, 2014: 1019, 2015: 281
GOES-15-Ereignisse), und die Per-Ereignis-Richtung ist **konsistent positiv
(76%), nicht alternierend**. Sie erreicht die strenge fam-Schwelle nicht; der
heißeste Rung (335→94) stärkt sich mit mehr Ereignissen nicht.

## Committet (mein Stand, englische Nachrichten)

- `353598d` AIA-fam: `aia_ladder_probe` rechnet den Full-Round-Family-Bound.
- `3874960` `eve_lines`/`aia_lines` series_parse_bin-Registrierung.
- `b4e7617` + `dc11735` Estimator-Sweep-Härtung; nicht-kanonischer KDE-h-Sweep
  (`transfer_entropy_lag_h`, kanonisch unberührt, factor=1.0 byte-identisch) +
  `corona_ladder_probe --h/--surr`.
- `b1065c0`, `dffc62d` AIA-fam-Befund + Vollkorpus-Manifest (CDN).
- `511acf5` Paper v4 (ehrliche Null — Operator lehnt Null-Paper ab, Neu-Rahmung
  offen).
- `7c8c978` Volljahr-2014-Messung (1019 Ev, Kaskade unter-fam).
- `33f04b1` 2015-Reproduzierbarkeit (281 Ev, Kaskade reproduziert).
- `0882b8b` Per-Ereignis-Richtung konsistent (76% positiv, nicht alternierend).

## Gemessene Zahlen (verlässlich)

- **AIA-2014** (Volljahr, 1019 Ev): 193→211 +1.78e-1, 211→335 +7.15e-2,
  335→94 +1.24e-1; fam 1.96e-1. 3 Monate (194 Ev) dagegen: 335→94 +1.42e-1,
  fam 1.89e-1 (0.75× → 0.63×fam: mehr Daten stärken den heißesten Rung nicht).
- **AIA-2013** (Volljahr, 524 Ev): 193→211 +1.67e-1, 211→335 +7.71e-2,
  335→94 +1.01e-1, alle ~96-s-Lag; fam 1.71e-1. Kaskade reproduziert sich.
- **AIA-2015** (281 Ev): 193→211 +1.21e-1, 211→335 +9.37e-2, 335→94 +1.30e-1;
  fam 1.75e-1. Kaskade reproduziert sich.
- **Per-Ereignis 335→94 (2015)**: 76% positiv, jeder Monat mehrheitlich positiv
  (posfrac 0.56–1.00), Monats-Mittel durchweg positiv. **Richtung konsistent,
  nicht alternierend** → die Wellen-/Alternations-Hypothese ist auf Ereignis-
  und Monats-Skala nicht getragen; unter-fam = moderate Amplitude gegen das
  konservative fam, nicht Richtungs-Kompensation.
- **EVE-2011** (109 Ev): 1032→131 D 5.11e-1 > fam 4.70e-1 (kanonische
  Bandbreite). h-Sweep: hält h∈[0.5,1.75], fällt bei h≥2.0 (Over-Smoothing).

## Offen / nächste Hebel

1. **Framing des Korona-Papers** nach Operator-Vorgabe (kein Null-Paper).
   Messgestützte Optionen: die reproduzierbare gerichtete Kaskade als Befund
   berichten (mit fam ehrlich benannt), oder ein gerichtetes Gate (nur die 3
   heißen Rungs / nur die ~96-s-Lags) prüfen.
2. **Korrelation der Kaskaden-Stärke mit Sonnenstruktur** (Operator-Idee, die
   Stärke der Maschine): Amplitude der Kaskade / Flare-Eigenschaften gegen
   aktive Breiten, Polkappen-Verschiebung, Magnetogramme auf **derselben
   Raumzeitachse**. Die Richtung ist konsistent — was moduliert die Amplitude?
3. **2013** (drittes Bestätigungsjahr): **ERLEDIGT** — Monat 11 via aia-cdn CI
   (34042431334) aufs CDN manifestiert (12/12), Volljahr gemergt und gemessen
   (524 Ev): Kaskade reproduziert sich (193→211 +1.67e-1, 211→335 +7.71e-2,
   335→94 +1.01e-1, ~96-s, fam 1.71e-1 unter-fam). Drei Jahre tragen die
   gerichtete Aufwärts-Kaskade.

## Datenlage (wichtig)

- **Dauerhaft:** die AIA-Monats-Assets liegen als CDN-Release
  `jsoc.stanford.edu/aiaYYYY_MM.bin` (2013 12/12, 2014 12/12, 2015 12/12),
  20–31 MB je Monat. Die GOES-15-Trigger liegen dauerhaft unter
  `data/ncei.noaa.gov/goes15_2013/` (363, 2013) und `data/ncei.noaa.gov/goes15/`
  (357, 2014) im Repo-Datenpfad; die gemergten Volljahr-Bins unter
  `data/jsoc.stanford.edu/aia2014_fullyear.bin` und `aia2013_fullyear.bin`.
  phi/sources.φ verweist weiter auf `aia2014_lines.bin`
  (dangling — das Einzel-Asset wurde gelöscht, die Monats-Assets sind die
  stabile Form; phi-Referenz ist zu aktualisieren).
- **Transient/neu zu ziehen:** die 2015-AIA- und 2015-GOES-Trigger liegen nur
  auf dem CDN bzw. NCEI — nicht lokal. 2013/2014-Volljahre sind dauerhaft unter
  `data/` abgelegt. Die dauerhaften Monats-Assets auf dem CDN sind die Quelle
  der Wahrheit; eine lokale Ernte in `/tmp` ist kein Ersatz für sie.
- **GOES-Trigger:** NCEI 2-s XRS (goes15/gxrs-l2-irrad), je Tag
  `sci_gxrs-l2-irrad_g15_d{YYYYMMDD}_v0-1-0.nc`. 2013 (363) und 2014 (357) unter
  `data/ncei.noaa.gov/goes15_2013/` resp. `goes15/`. 2015 bei Bedarf von NCEI
  ziehen (Version v2-2-1).

## Workflow-Notiz

- AIA-Ernte läuft nur stabil als **Monats-Pakete** (ein Volljahr = JSOC-Query-
  Status-6/Timeout-Risiko; 7-Tage-`@12s` wird von JSOC abgelehnt). Dispatch:
  `aia-cdn.yml` start/end je Monat, asset `aiaYYYY_MM.bin`.
- CI-Runner des Repos sind durch die schweren parallelen Fremd-Sessions belegt →
  Ernten sind langsam/kontendiert; Monats-Batches (je 3) geduldig durchlaufen
  lassen.

## Standing debt (unabhängig, separat zu erledigen)

- Deutsche Commit-Nachrichten meiner frühen Corona-Commits (`3874960` `353598d`
  `b4e7617` `1afbfe0` `dc11735` `b1065c0` `dffc62d` `511acf5` `7c8c978`) sind
  ins Englische umzuschreiben. **Nicht** per Force-Push auf den kontendierten
  main — separat, wenn die parallelen Sessions pausieren.

## Register

Alle Befunde sind in `docs/TODO.md` (Nadel Ⅲ, Abschnitt „EVE-Bandbreiten-…
/AIA-…-Messung") eingetragen. Der Paper-Text (v4, `511acf5`) ist eine ehrliche
Null und damit NICHT die finale Fassung — die Neu-Rahmung hängt an der
Operator-Entscheidung (Punkt 1 oben).
