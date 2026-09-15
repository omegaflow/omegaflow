<!--
  title: Handover — Bau & Code (Stand 2026-09-11, Bau5)
  session: Bau-Folge
  class: handover
  date: 2026-09-11
  sha256: f0e9cb845140a9419942d620e59ea09ecc3f249c916118fddffde6fecd84ebdd
  status: archived
-->
# Handover — Bau & Code (2026-09-11, Bau5)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab, wie sie kann — Sub-Agenten tragen eigenen Kontext, die Anzahl
ist kein Aufwand. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

## Zeugen-Kreuzbereich

- antares-Loci-Vollkreuzmatch — der Code steht und ist committet (316d226):
  skydirection_compiler paginiert die ANTARES-Loci über page[limit]=1000/
  page[offset] und hält den gemessenen Vollbestand (meta.count = 10000,
  gemessen 2026-09-11; der bare limit-Parameter ist tot, ignoriert);
  direction_z_join ist gechunkt mit Wiederaufnahme (CHUNK 512, Checkpoint
  = das --out-Asset skydirections_zchunk.bin, Resume über den CDN-Download);
  skydirection-cdn.yml erntet bedingungslos (kein download||harvest-Kurzschluss,
  --antares = Vollbestand); sky-crossmatch-cdn.yml lädt den zchunk-Checkpoint
  und fährt den Join mit --out --ci-mode. Der Commit ist nicht gepusht —
  origin/main..HEAD trägt neben meinen zwei Commits (316d226, 39c6127) fremde
  Commits (ernte/Entscheid/Bau4/jup365/forschung); git status ist leer, aber
  die fremden Commits liegen vor und nach meinen, ein Push trüge sie alle.
  Offen: push, dispatch skydirection-cdn.yml, dispatch sky-crossmatch-cdn.yml,
  witnesses.φ-Verdikt aus dem Log (die 2026-09-07-Zählung — 10 Loci, 0 mit
  Distanz — steht noch im Register).

## Membran

- M02–M07 offen (unverändert): M02 ESP32-S3-Radiatorium-Firmware (no_std),
  M03 Audio-Gain ohne tanh, M04 Navigation/Nebra-Kalibrierung, M05/M06
  Stations-Sensoren als SI-4-Token, M07 ⌘K-Palette (fuzzy).
