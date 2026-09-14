<!--
  title: Handover — Forschung-Folge XVIII (Stand 2026-09-14)
  session: Forschung-Folge XVIII
  class: handover
  date: 2026-09-14
  sha256: 08a80bc5c61c98d5b85466d968a5b3148456814ceb7e6e60f0e33af777a74abb
  status: live
-->
# Handover — Forschung-Folge XVIII (2026-09-14)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

Der erste offene Abschnitt benennt den härtesten undatierten Punkt (datierte
Wiedervorlagen schweigen vor ihrem Datum). Jeder offene Punkt trägt seinen
nächsten Schritt in derselben Zeile — Werkzeug, Datei, URL oder Anfrage;
„Schritt unbekannt — erste Messung: X" ist ein vollständiger Schritt. Kein
Dokument wächst ohne Messung; die Droh-Sprache ersetzt den Schritt nicht.

## Tiefenphasen — Positive Maske

- Gestalt-Treiber (GLO-30-DEM-CDN-Dispatch) — offen: der TIFF-/COG-Reader ist
  pending (witnesses.φ gl30, „der Record-Socket gl30 steht, nie gefaellt").
  Schritt: COG-GeoTIFF-Reader (EPSG:4326, EGM2008-Hoehe, 1 arcsec) + Compiler +
  CDN-Asset. Die Per-Station-Pfadbedingung und die bedingte TE stehen jetzt
  (depthphase.rs `p_p_branch` Clear/BranchUnstable/Fold;
  depth_phase_positive_mask_probe `TE_FLOOR_N = 32`).

## Nadeln

- Ⅰ Jeans-Residuum bis Gaia DR4 (Wiedervorlage 2026-12-02).
- Ⅱ JUICE-Flyby 28./29.9. (Wiedervorlage 2026-09-28) · Europa Clipper 3.12.
  (Wiedervorlage 2026-12-03).
- gaia-dr4-iapetus (Wiedervorlage 2026-12-02).
- Ⅺ Placebo — AVE-Neumessung läuft in CI (placebo-ave-cdn); der Verdict landet
  im openneuro.org-Release. Die MNI-Fiducials sind geerntet und registriert
  (fieldtrip standard_1005.elc, phi/sources.φ:8321; Labels LPA/RPA/Nz/Iz, mm;
  Nz↔Nasion ist die 10-20-Konvention, gemessen; die Montage trägt kein Inion,
  gemessen). Nächster Schritt: der elc-Reader in tools/measure/src/fiducial.rs
  (parse .elc → MNI-Fiducials), dann die Montage→MNI-Ko-Registrierung über die
  bestehende Umeyama-Rigid (rigid_coregister).

## Weberin

- Kernel 000113+ bei ESA nach Erscheinen (Wiedervorlage 2026-09-28).

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`).
