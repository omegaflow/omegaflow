<!--
  title: Handover — Forschung-Folge XVII (Stand 2026-09-13)
  session: Forschung-Folge XVII
  class: handover
  date: 2026-09-13
  sha256: ba38366d28a04c65ac6db538dfcd03e7c9fc7fe62ea85833f6b12d5d03e631d6
  status: live
-->
# Handover — Forschung-Folge XVII (2026-09-13)

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
  openneuro.org-Release. REST-Lead-Field + benannte Regularisierung stehen; die
  Fiducial-Basis ist gebaut (tools/measure/src/fiducial.rs — Umeyama-Rigid,
  BEM-Leser, Montage-Fiducial-Leser; die Montage trägt 3 FID-Landmarken in
  chaninfo.nodatchans: Nasion, L/R präaurikular — kein Inion, gemessen).
  Pending: die MNI-seitigen Fiducials — Kandidat benannt, ungeerntet: fieldtrip
  template/electrode/standard_1005.elc (Labels LPA/RPA/Nz + Iz, mm, gemessen
  dieser Session; Nz↔Nasion ist eine zu registrierende Konvention, kein stiller
  Schluss). Die Montage→MNI-Ko-Registrierung braucht diese Datei, keine
  Literaturkonstante; die Ernte + Registrierung in phi/sources.φ ist der
  offene Pflichtschritt.

## Weberin

- Kernel 000113+ bei ESA nach Erscheinen (Wiedervorlage 2026-09-28).

## Tiefenphasen

- Positive Maske — die σ-Reduktions-Sonde steht (depth_phase_driver_scatter_probe,
  zwei Treiber getrennt, Permutations-Null). Benannt, nicht gebaut: die
  Per-Station-Pfadbedingung über einen Ray-Tracer (der Within-Event-36-km-Riss)
  und bedingte TE ab n ≥ 32 (Zweierpotenz-Boden). Der Gestalt-Treiber
  (GLO-30-DEM-CDN-Dispatch) steht noch offen.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators und der gemessene Abschluss-Check.
