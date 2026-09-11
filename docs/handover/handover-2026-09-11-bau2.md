<!--
  title: Handover — Bau & Code (Stand 2026-09-11, Bau2)
  class: handover
  date: 2026-09-11
  sha256: b014ae9d0070b4b4668329666a17b06dc0e527a110e52c0027ff8e00b92e50a0
  status: live
-->
# Handover — Bau & Code (2026-09-11, Bau2)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab, wie sie kann — Sub-Agenten tragen eigenen Kontext, die Anzahl
ist kein Aufwand.

## Compiler & Format

- antares_loci-Konsumption offen — parse_anr steht (geo.rs), kein Kanal in den
  ω()-Kreislauf; registriert sources.φ:5530–5536.

## Reader / Struktur

- Parquet-Struktur-Reader offen.
- GRIB-2-Struktur-Reader offen (OPeNDAP ist gebaut und live-verifiziert).

## Gaia-XP

- xp_spectra-Gesamtarchiv-Parse-Verifikation offen — der Pilot ist verifiziert
  (99,777 %, 28 Grenzfälle = publizierte Rundung, Shift-Bindung hält); das
  ssd.jpl-Asset ist ungetestet gegen den geteilten Parser.

## CDN

- OPeNDAP-3-Dokument-Manifestation offen (in fetch.rs/port.rs-Skiplisten benannt).
- antares_loci + xp_pilot: url-Zeilen registriert, Manifestationsweg offen — die Compiler tragen keinen --ci-mode-Pfad für diese Assets, kein Workflow-Job deckt sie, die Release-Tags fehlen.

## Ledger

- phi/pipeline/ledger.φ-Inhalt offen — die 117 Census-Einträge sind
  Laufzeit-Artefakte eines regtap-census-Imports; Weg zurück ist die erneute
  Ernte.

## TE-Bau

- cycle_phase_shift_surrogate — descoped-Befund: „nie gebaut, nicht gebraucht
  (2026-09-11: einziger Aufrufer mit y.len(); keine Periodenmessung in irgendeinem
  Probe; Primitiv + Tests bleiben)".

## Membran

- M02–M07 offen (unverändert).
