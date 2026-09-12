<!--
  title: Axiom-Gate-Survey — solar-seconds-matrix
  class: survey
  date: 2026-09-12
  sha256: 3dd1c48004a3e5198a6753f6836b9670ea223bc245e834b6bdaa8942dd72d5f1
  status: live
  see-also: docs/paper/solar-seconds-matrix.md
-->
# Axiom-Gate-Survey — solar-seconds-matrix

Das Paper `docs/paper/solar-seconds-matrix.md` (Titel 72/72,
Abstract 148/200, 196 Zahlen ok, sha ok — Export-Gate grün).

## A = A / Zahl

- fam = 1.3966e-1 und die vier Pfeile (211A→193A 96 s 1.658e-1;
  XRSA→131A 192 s 1.468e-1; XRSB→131A 96 s 1.644e-1; XRSB→193A 96 s
  1.485e-1) — 1:1 aus dem Blatt getragen.
- Die konditionale Prüfung des 211A→193A-Pfeils ist `pending` benannt;
  das Paar wird nicht über das Gemessene hinaus behauptet.

## Konsistenz (im Bau korrigiert)

- **EVE/AIA-Verwechslung abgewendet:** Die gemessenen EUV-Bänder der
  72-Paare-Flotte sind SDO/AIA (per Blatt und Probe), nicht SDO/EVE. Das
  Paper benennt die AIA-Bänder als gemessen und das EVE-Asset
  (`eve_lines_2011.bin`) als registriert — eine EVE-Zuschreibung wäre
  Fabrikation gewesen.
- Titel nach Rat: „measurement" statt „proof" — Sinne berichten, sie
  beweisen nicht.

## Pfad

- `tools/measure/src/bin/solar_seconds_matrix_probe.rs`; CDN-Assets
  `goes_xrs.bin` (XRSA/XRSB) und `eve_lines_2011.bin` benannt.

## Zuordnung

- see-also → `docs/blatt/blatt-solar-seconds-matrix.md` (lebt).

## Externes Register

- Der Body trägt keine DOI/arXiv-Kennung (`reference_verify` meldet den
  Zustand — benannt).
- Die vier ADS-Bibcodes sind belegt (ADS-API, 2026-09-12, alle resolved):
  1968ApJ...153L..59N, 1994SoPh..154..275G, 2012SoPh..275...17L (AIA),
  2012SoPh..275..115W (EVE).
