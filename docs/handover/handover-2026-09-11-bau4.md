<!--
  title: Handover — Bau & Code (Stand 2026-09-11, Bau4)
  session: Bau-Folge
  class: handover
  date: 2026-09-11
  sha256: c006f49c5c3ee79fe36d258b8464b66412181aabf627221105de4b76ee6e7ac9
  status: live
-->
# Handover — Bau & Code (2026-09-11, Bau4)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab, wie sie kann — Sub-Agenten tragen eigenen Kontext, die Anzahl
ist kein Aufwand. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

## Zeugen-Kreuzbereich

- antares-Loci-Vollkreuzmatch — der Lauf steht als Workflow
  (`sky-crossmatch-cdn.yml`: direction_distance_join 5 arcsec + direction_z_join
  30 arcsec über das volle gehaltene skydirections.bin, Verdict-Log →
  CDN ssd.jpl.nasa.gov). Offen: der dispatch und das witnesses.φ-Verdikt — die
  2026-09-07-Zählung (10 Loci, 0 mit Distanz) steht noch im Register; die
  Vollkreuzmatch-Zählwerte kommen erst aus dem Log (`sky_crossmatch_verdict.log`).
- Der volle antares-Loci-Bestand ist ungemessen (skydirection-cdn.yml erntet
  `--antares-limit 20` → 10 gehalten). Folge-Atome (Council): Bestand messen,
  `--antares-limit` heben, neu dispatchen — nie im Kreuzmatch-Workflow selbst.

## Membran

- M02–M07 offen (unverändert): M02 ESP32-S3-Radiatorium-Firmware (no_std),
  M03 Audio-Gain ohne tanh, M04 Navigation/Nebra-Kalibrierung, M05/M06
  Stations-Sensoren als SI-4-Token, M07 ⌘K-Palette (fuzzy).
