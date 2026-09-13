<!--
  title: Handover — Forschung-Folge IX (Stand 2026-09-12)
  session: Forschung-Folge IX
  class: handover
  date: 2026-09-12
  sha256: 2f4d39e406718caf34526f1cdabad21c9e8b6b70a298976a10614a5ce2091d35
  status: live
-->
# Handover — Forschung-Folge IX (2026-09-12)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

## Analyse (Flut-Satellit)

- S1 — der Compiler trägt S1S2 (MAGIC S1S2, Stride 56, inc + presence-Flag) und
  s1-sar-cdn.yml steht. Offen: das alte s1_sar_diff.bin-Release-Asset entfernen
  (Operator-Wort erteilt) → Dispatch s1-sar-cdn → das S1S2-Asset manifestiert.
- DEM — copernicus_dem_compiler steht; copernicus-dem-cdn.yml-Dispatch ausstehend
  (CDN-Manifestation-Duty).

## Nadeln

- Ⅰ Jeans-Residuum bis Gaia DR4 (Wiedervorlage 2026-12-02).
- Ⅱ JUICE-Flyby 28./29.9. (Wiedervorlage 2026-09-28) · Europa Clipper 3.12.
  (Wiedervorlage 2026-12-03).
- gaia-dr4-iapetus (Wiedervorlage 2026-12-02).
- Ⅺ Placebo — MAT-v5-Reader gebaut und unit-verifiziert (matfile 5/5, eeglab 8/8;
  mxSTRUCT/mxCELL, verschachteltes miCOMPRESSED, EEG-Struct-Abbildung). Offen:
  der Live-Lauf gegen ein echtes ds005034-Paar — der Download-Pfad ist neu zu
  messen (OpenNeuro-API geändert, kein lokales .set, keine sources.φ-Zeile;
  dead_sources.φ trägt die tote API-Root).

## Galileo-Floor

- Rausch-Kurve — Format-1/2-Zweig steht (odf.rs); die ODF-Format (1 vs 2) bleibt
  ungemessen — galileo-trk-noise.yml-Dispatch verifiziert die Kurve.

## Weberin

- VLBI-Winkel-Probe pending (PRIDE ΔDOR nicht publiziert).

## Tiefenphasen

- CMT — depth_phase_fleet_probe --kalibrier (NDK-Quellterm, Stationsazimut +
  Polaritätsvorhersage) und cmt-ndk-fleet.yml stehen. Offen: der Feld-Neulauf
  (us10003re5) und der Probe-Lauf (jan76_dec25.ndk ~24 MB) als CI-Dispatch.
- W-Phase-CMT — usgs_mww_centroid_probe gebaut (QuakeML by-eventid,
  magnitudetype=mww → momentTensor + Zentroid; optionaler --picker-lat/--picker-lon-
  Offset, --gcmt-Abgleich). Offen: Verdrahtung des USGS-Mww-Zentroids in die
  Tiefenphasen-Flotte (ersetzt den M9.1-Picker-Pfad).
- Stromboli — StationTerm-Wiring steht; die gemessene Null bleibt absent. Offen:
  Vorzeichen/Anwendung des Terms auf das pP−P-Lag ungemessen (nur die Null
  gemessen, neutral); ein nicht-nuller Term braucht Daten → heavy fetch/CI.
- Tonga 2022 — Kreuz-Abgleich-Probe gebaut; die Roh-Druckwellenform bleibt
  blockiert (CTBTO-vDEC 403), nicht descoped.
- Positive Maske — Treiber einer nach dem anderen (DEM, Magnetfeld, Slab2,
  3D-Modell) in die bedingte TE legen und messen, ob die 36-km-Streuung
  schrumpft; schrumpft sie, war der Faden ein echter Treiber, bleibt sie, war
  er irrelevant. Die Treiber erntet die Ernte-Linie.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators und der gemessene
Abschluss-Check. Der Baum trägt fremde uncommittete Arbeit; gepusht wird erst,
wenn er ruhig ist.
