<!--
  title: Handover — Forschung-Folge VIII (Stand 2026-09-12)
  session: Forschung-Folge VIII
  class: handover
  date: 2026-09-12
  sha256: b5a2007aba3ee784a16ae9667eb40f71b32235083293890d936f614f7631fba0
  status: live
-->
# Handover — Forschung-Folge VIII (2026-09-12)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

## Sitzungs-Stand

Folge VII abgearbeitet — sechs Bau-Linien stehen und sind verifiziert (cargo check
0 Fehler / 0 Warnungen für core/measure/harvest, Tests still): CMT
Archivar-Verdrahtung (parser-def ndk nach src/archivar/ndk.rs portiert,
blocked_sources.φ aufgelöst, Probe auf archivar::ndk::fetch_events umverdrahtet,
measure-lib-Kopie gelöscht — ein Pfad), LAIC Kyoto-Dst (gemessen: dstdir/dst_final/
YYYYMM ist 404; der Träger ist dst_final/{YYYYMM}/dst{YYMM}.for.request dann
dst_provisional/…, 120-Byte-Fixed-Width, 9999→absent, TE-Paar Dst↔F),
Placebo-EEGLAB-Reader (Text-.set-Header + .fdt little-endian f32), Stromboli-
Stationskorrektur (StationTerm + apply_station_term, die gemessene Null ist absent),
S1-Incidence-Band (inc aus annotation/iw-vv.xml, MAGIC S1SR→S1S2), DEM-Compiler
(GeoTIFF/COG, zeuge GL30-Gestalt, sources.φ-Register, copernicus-dem-cdn.yml).

## Analyse (Flut-Satellit)

- S1-Incidence-Band gebaut — gemessen: inc liegt nicht in calibration-*.xml,
  sondern in annotation/iw-vv.xml <geolocationGridPointList> (geolocationGridPoint
  mit line/pixel/incidenceAngle); MAGIC S1SR→S1S2 (Stride 40→56, inc + presence-Flag,
  absent = 0.0-Pad + Flag 0, NaN nie am Wire). Offen: s1-sar-cdn.yml lädt das alte
  s1_sar_diff.bin (`||`-Kompilat) — das S1S2-Asset erscheint erst, wenn das alte
  Release-Asset ersetzt/entfernt wird (Operator-Entscheidung); kein Laufzeit-Konsument
  des alten Records (repo-weit gemessen) — nichts bricht.
- DEM — copernicus_dem_compiler steht (GeoTIFF/COG float32, Deflate, Predictor
  1/2/3, zeuge GL30, sources.φ-Register-Zeile). Offen: copernicus-dem-cdn.yml-
  Dispatch ausstehend (workflow_dispatch) — CDN-Manifestation-Duty bis zum Dispatch.

## Nadeln

- Ⅰ Jeans-Residuum bis Gaia DR4 (Wiedervorlage 2026-12-02).
- Ⅱ JUICE-Flyby 28./29.9. (Wiedervorlage 2026-09-28) · Europa Clipper 3.12.
  (Wiedervorlage 2026-12-03).
- gaia-dr4-iapetus (Wiedervorlage 2026-12-02).
- Ⅺ Placebo — Text-.set/.fdt-Reader gebaut, aber ds005034 trägt kein Text-.set/.fdt:
  alle _eeg.set sind MATLAB-5-MAT (miCOMPRESSED, Daten eingebettet — gemessen
  2026-09-12), der Reader liest absent (0-Kanon, keine fabrizierte 0). Das echte Paar
  braucht einen std-only MAT-v5-Reader — nächstes Atom. Kein Descope: der Konsument
  (placebo_pair_eeg_probe) steht.

## Galileo-Floor

- Rausch-Kurve — Format-1/2-Zweig gebaut (odf.rs). Die Galileo-ODF-Format (1 vs 2)
  bleibt ungemessen (kein lokales galileo_odf.bin) — Compiler-Lauf (CI,
  galileo-trk-noise.yml) verifiziert die Kurve.

## Weberin

- VLBI-Winkel-Probe pending (PRIDE ΔDOR not-published, gemessen 2026-09-12).

## Tiefenphasen

- CMT — parser-def ndk steht in src/archivar/ndk.rs (fetch_events = fetch → cache →
  parse); blocked_sources.φ aufgelöst; sources.φ trägt die GCMT-NDK-Zeile. Offen:
  Kalibrier-Gate (die sechs Stationsazimute des Feldpilot-Laufs us10003re5 sind in
  keinem Register — Rückgewinnung braucht einen Feld-Neulauf, heavy fetch → CI),
  Probe-Lauf (jan76_dec25.ndk ~24 MB, schwerer Fetch → CI).
- W-Phase-CMT (M9-Nachfolger) — Endpoint verifiziert (USGS FDSN, magnitudetype=mww,
  Moment-Tensor am by-eventid-quakeml-Pfad). Bau-Linie offen: Zentroid-Inversion auf
  USGS-Mww statt des M9.1-Pickers.
- Stromboli — StationTerm-Wiring gebaut (apply_station_term, keyed NET.STA); die
  gemessene Null bleibt absent (nie eine 0.0-Korrektur). Offen: Vorzeichen/Anwendung
  des StationTerms auf das differentielle pP−P-Lag ungemessen (nur die Null gemessen,
  neutral).
- Die Erde als Sender (Tonga 2022) — Kreuz-Abgleich-Probe gebaut; die
  Roh-Druckwellenform bleibt blockiert (CTBTO-vDEC 403), nicht descoped.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators und der gemessene
Abschluss-Check. Der Baum trägt diese Session's uncommittete Arbeit; gepusht
wird erst, wenn er ruhig ist.
