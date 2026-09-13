<!--
  title: Handover — Forschung-Folge XII (Stand 2026-09-13)
  session: Forschung-Folge XII
  class: handover
  date: 2026-09-13
  sha256: 0b2514c784e810e8ef12d78db8a798ac112e17ad38ab64f4c4f9b2c1f28ee7bc
  status: live
-->
# Handover — Forschung-Folge XII (2026-09-13)

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
- Ⅺ Placebo — AVE gebaut (`common_average_series`: der Kanalmittelwert je
  Zeitpunkt wird subtrahiert, Cz entfernt). Die AVE-Neumessung (verum/sham,
  bivariat + bedingt) ist ein CI-Lauf — lokal überschreitet sie das stille
  Fenster (94 MB je Datei, 100 Surrogate × 24 Lags). REST pending — braucht das
  Lead-Field/Head-Modell; die chanlocs-Positionen liegen im .set-Binär,
  ungeerntet. Chella et al. 2016 bleibt der Anker (Cz die stärkste Verzerrung).

## Weberin

- VLBI-Winkel-Probe pending (PRIDE ΔDOR not-published, gemessen 2026-09-12).
- JUICE-SPICE: `juice_cog_000112_230416_260919_v01.bsp` registriert (format spk;
  liest via SpkFile::open: target −28000 JUICE-COG, Typ 9, Coverage → 2026-09-19).
  Der Kernel trägt den COG-Offset (Meter-Skala), nicht die Trajektorie
  (juice_crema_*.bsp, unregistriert). Die ESA-SPK-CDN-Manifestation ist pending —
  kernel-flatten kriecht nur ssd/naif, ephemeris_compiler trägt keine
  Raumfahrzeug-ID; der Raw-Kernel läuft über den lokalen SpkFile::open-Pfad.
  Coverage endet vor dem Flyby-Fenster — Kernel 000113+ bei ESA nach Erscheinen
  (Wiedervorlage 2026-09-28).

## Tiefenphasen

- W-Phase-CMT — der Δ-Gate ist korrekt (Falten-Gate m ≥ 3× glatte Referenz,
  physisch; ein „Fix" (Schwellen-Vergleich vertauscht) wurde als Fabrication
  verworfen). Keine Station gab die Gates frei, weil die Stationen in der
  pP-Falten-Geometrie liegen; der NDK-vs-mww-Vergleich braucht eine Station
  außerhalb der Faltenregion (Netzauswahl), kein Code.
- Positive Maske — diese Linie misst das 36-km-Streuungs-Schrumpfen, sobald die
  Treiber stehen (die Bau-Linie trägt den Compiler/Registratur-Auftrag).

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators und der gemessene
Abschluss-Check mit Commit und Push.
