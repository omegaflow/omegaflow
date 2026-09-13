<!--
  title: Handover — Forschung-Folge XV (Stand 2026-09-13)
  session: Forschung-Folge XV
  class: handover
  date: 2026-09-13
  sha256: 3b2407c80ff79cb56376cad6ea5e847365562d3622c68674258c143337ee0fe4
  status: live
-->
# Handover — Forschung-Folge XV (2026-09-13)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

## Nadeln

- Ⅰ Jeans-Residuum bis Gaia DR4 (Wiedervorlage 2026-12-02).
- Ⅱ JUICE-Flyby 28./29.9. (Wiedervorlage 2026-09-28) · Europa Clipper 3.12.
  (Wiedervorlage 2026-12-03).
- gaia-dr4-iapetus (Wiedervorlage 2026-12-02).
- Ⅺ Placebo — AVE-Neumessung läuft in CI (placebo-ave-cdn); der Verdict landet im
  openneuro.org-Release. REST: Lead-Field gebaut (tools/measure/src/rest.rs,
  sphärisches 3-Schalen-Modell aus chanlocs gefittet, Chella et al. 2016 als
  Methoden-Anker, nie als Äquivalenz). Pending: das BEM/Cortex-Mesh (MNI/ICBM-
  Template — ein Datensatz, kein Code), die Fiducial-Basis und die benannte
  Regularisierung.

## Weberin

- VLBI-Winkel-Probe — neu gemessen (playwright, 2026-09-13): das EVN-JIVE-Archiv
  (archive.jive.nl/scripts/portal.php) ist offen (200, public-domain FITS,
  Katalog listarch.php) — aber astrophysikalisches EVN-VLBI, kein PRIDE-ΔDOR.
  Die Positions-Messung ist in offenen Papers publiziert (Icarus 2024 CC-BY,
  Space Sci Rev 2023, A&A 2016); eine offene maschinenlesbare
  Raumsonde-Plane-of-Sky-Datenlinie bleibt pending.
- Census SSB-Kette — gemessen: 54/251 nicht-599-zentrierte crema-Segmente
  (Sun/Venus/Earth/Moon) schließen zum SSB; 197 Galileische Monde
  (Europa/Ganymed/Callisto) bleiben None — es fehlt der Jupiter-Satelliten-SPK-
  Träger (jup*.bsp mit 501/502/503/504→599 und 599→0), kein Kettendefekt.
- ephemeris_juice_cog.bin — CK-Reader + SWITCH-Auflösung gebaut
  (src/archivar/ck.rs + fk.rs, SCLK-Parser, 7 Tests) und im Compiler verdrahtet
  (ephemeris_compiler --juice-cog); .bin produziert (8,2 MB, 18262 Granules,
  Roundtrip median 0,24 m, p99 105 m). Offen: CDN-Manifestation (--ci-mode) und
  die Flyby-Granularität (max 23,5 km am scharfen Jupiter-Mond-Flyby bei
  einheitlichem 6-h-Granule — die adaptive Flyby-Verfeinerung des
  horizons_compiler ist nicht repliziert).
- Kernel 000113+ bei ESA nach Erscheinen (Wiedervorlage 2026-09-28).

## Tiefenphasen

- Positive Maske — misst das 36-km-Schrumpfen, sobald die Treiber stehen.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators und der gemessene Abschluss-Check.
