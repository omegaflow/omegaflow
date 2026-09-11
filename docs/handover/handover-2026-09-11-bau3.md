<!--
  title: Handover — Bau & Code (Stand 2026-09-11, Bau3)
  class: handover
  date: 2026-09-11
  sha256: b926f29f83df456fab5fe58bc57c8f4e1a8127c818a93394aa6bf667dae7f2b9
  status: live
-->
# Handover — Bau & Code (2026-09-11, Bau3)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab, wie sie kann — Sub-Agenten tragen eigenen Kontext, die Anzahl
ist kein Aufwand. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

## Gaia-XP

- xp_spectra-Gesamtarchiv-Parse-Verifikation offen — der Pilot ist verifiziert
  (99,777 %, 28 Grenzfälle = publizierte Rundung, Shift-Bindung hält); das
  ssd.jpl-Asset (xp_spectra.bin, parallax>20, gaia-xp-cdn.yml) ist ungetestet
  gegen den geteilten Parser (parse_xp_spectra_bin) — der Compiler trägt nur den
  Selbst-Roundtrip, kein Gesamtarchiv-Verdikt.

## CDN

- xp_pilot_p6144.bin — url-Zeile registriert (dc.g-vo.org); der Compiler lädt
  --ci-mode nur auf ssd.jpl.nasa.gov (kein --release-tag), kein Workflow-Job
  deckt den Piloten, das --source-range-Rezept ist unverzeichnet (lokales Asset
  data/dc.g-vo.org/xp_pilot_p6144.bin). Manifestationsweg offen.
- OPeNDAP-3-Dokument — kein `format opendap` in sources.φ, kein Referenz-url
  registriert; die Skiplisten-Einträge (fetch.rs/port.rs) sind korrekt (eigener
  opendap-Dekoder main_flow.rs:1204-1270). Offen: das Dokument selbst als
  reference-Quelle manifestieren (url fehlt).

## Zeugen-Kreuzbereich

- antares-Loci-Vollkreuzmatch offen — direction_distance_join +
  direction_z_join über das volle skydirections.bin-Asset; der 2026-09-07-Verdikt
  deckte 10 gehaltene Loci (0 mit Distanz gemessen). Die Loci sind S²-Zeuge
  (witnesses.φ), distanzlos τ=0; der Feldkanal ist descoped (Council 2026-09-11).

## Membran

- M02–M07 offen (unverändert): M02 ESP32-S3-Radiatorium-Firmware (no_std),
  M03 Audio-Gain ohne tanh, M04 Navigation/Nebra-Kalibrierung, M05/M06
  Stations-Sensoren als SI-4-Token, M07 ⌘K-Palette (fuzzy).
