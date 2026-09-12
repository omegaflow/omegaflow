<!--
  title: Handover — Forschung-Folge VII (Stand 2026-09-12)
  session: Forschung-Folge VII
  class: handover
  date: 2026-09-12
  sha256: 5d60f11e9d309686ef1832346ad51c7ce1a83f49b20d6671af08c5d455764319
  status: live
-->
# Handover — Forschung-Folge VII (2026-09-12)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

Dies ist die Forschung-Linie: hier steht nur, was diese Linie autonom trägt —
Tasks, die nicht autonom hier erfolgen können, sind in das Handover ihrer Linie
überführt. Wiedervorlage gilt nur für Termine; „Warten auf Rückmeldung" trägt
kein Datum.

## Sitzungs-Stand

Folge VI abgearbeitet. Der Baum trägt fremde uncommittete Arbeit einer aktiven
Parallel-Session (igets_compiler/igets_merge, src/archivar/{geo,main_flow,relay}.rs,
src/mathematikerin/{force,omega,tests}.rs, static/*, phi/dead_sources.φ,
phi/pipeline/catalog/noaa_nodd_disposition.φ, tools/utils/src/bin/source_scanner.rs,
bau11→archiv + bau12) — unberührt. Diese Session committet nur den eigenen Teil
(nadel_gate-Descope, infrared_excess_compiler --far-ir-Entfernung, odf.rs
Format-1/2-Zweig, sources.φ/witnesses.φ-Edits, diese Übergabe) auf das
Consent-Wort des Operators; gepusht wird erst, wenn der Baum ruhig ist.

## Analyse (Flut-Satellit)

- Robustheits-Gates, Oberflächen-Artefakt-Format und Kalibrier-Gate gebaut.
  Offen: Incidence-Band (inc) — der per-Pixel-Einfallswinkel liegt in der
  SAFE-Kalibrier-XML, die s1_sar_compiler nicht erntet (absent).
- DEM-Terrain/Layover — Register-Zeile gebaut (Copernicus GLO-30, witnesses.φ
  gestalt, anonym HTTP 200, COG-GeoTIFF 1°×1°-Kacheln, 1 arcsec, EGM2008-Höhe).
  Offen: TIFF/COG-Reader + Compiler + CDN-Asset (nächstes Atom, Konsument =
  S1-Layover/Shadow-Korrektur).

## Descope (Rat 2026-09-12, einstimmig)

- Nadel V far-IR-Achse descoped — die vier Skalar-Schwellen (W3−W4, IRAS
  F25/F12, MSX E/C, akari 18/9) tragen kein belastbares Zitat: Literaturrecherche
  2026-09-12 misst auf keinem der vier Paare einen peer-reviewed Skalar-Cutoff;
  die publizierten Kriterien sind Zwei-Farben-Belegungszonen (van der Veen &
  Habing 1988, A&A 194, 125, [12]−[25]/[25]−[60], 8 Regionen; Ishihara et al.
  2011, A&A 534, A79 / Ita et al. 2010, [K]−[9]/[9]−[18]; Koenig & Leisawitz
  2014, WISE-Ebenen; Ortiz et al. 2005, MSX), Grenzen unretrieved; der eine
  publizierte Skalar W1−W2 ≥ 0.8 (Stern et al. 2012) ist Nah-IR-AGN-Farbe und
  lebt als AGN-Keil weiter. Nie gebaut, nicht gebraucht — die AGN-Keil-Linie
  trägt den Natural-Dimmer-Ausschluss; ein Zwei-Farben-Achsenbau wäre ein neues
  Atom mit eigener Bedarfsmessung, keine Wiederbelebung der vier Skalare. Der
  Descope ist ausgeführt: far-IR-Achse aus nadel_gate, --far-ir aus
  infrared_excess_compiler, sources.φ-Block (msxc6/akari_fis/akari_irc/irasfsc)
  entfernt; der Archivar-Leser ir_{iras,msx,akari}.bin + CDN-Asset folgen in
  denselben Descope (kein Konsument).

## Nadeln

- Ⅰ Jeans-Residuum bis Gaia DR4 (Wiedervorlage 2026-12-02).
- Ⅱ JUICE-Flyby 28./29.9. (Wiedervorlage 2026-09-28) · Europa Clipper 3.12.
  (Wiedervorlage 2026-12-03).
- gaia-dr4-iapetus (Wiedervorlage 2026-12-02).
- Ⅳ LAIC — INTERMAGNET-F zugangsblockiert (Account-Pflicht); anonymer Dst-Index
  (Kyoto, dst_final/YYYYMM, HTTP 200 gemessen) nicht verdrahtet.
- Ⅺ Placebo — Substrat gemessen (2026-09-12): kein placebo-spezifisches Paar auf
  PhysioNet; OpenNeuro ds005034 (tACS verum/sham, 25 Probanden, 129 Kanäle,
  .set anonym HTTP 200) ist der Träger, ds001849 (TMS active/sham) das
  Geschwister. Offen: placebo_pair_eeg_probe auf das OpenNeuro-.set-Paar
  re-verdrahten.

## Galileo-Floor

- Rausch-Kurve — TRK-2-18 Format-1/Format-2-Zweig gebaut (odf.rs, Format-ID
  Bits 129–131; Format-1 aus SIS 1988 Table 3b, Format-2 golden-verifiziert;
  das Observable ist Hz in beiden Formaten — die Phase lebt in TRK-2-34/TNF).
  Die Galileo-ODF-Format (1 vs 2) bleibt ungemessen (kein lokales
  galileo_odf.bin) — Compiler-Lauf (CI) verifiziert die Kurve.

## Weberin

- PSA TNF — Register-Zeile gebaut. VLBI-Winkel-Probe bleibt pending (PRIDE
  ΔDOR not-published, gemessen 2026-09-12).

## TE

- Phase-Null — sweep gemessen: 22 Punkte, 12 PASS / 10 FAIL (Punkt 21
  phase-binned FAIL, Punkt 22 ksg PASS); Block/Shift-n-Grenze im Sheet.

## Tiefenphasen

- Quell-Strahlungsterm (CMT) — NDK-Parser + M-Tensor + R_P verdrahtet. Offen:
  Kalibrier-Gate (die sechs Stationsazimute des Feldpilot-Laufs us10003re5 sind
  in keinem Register — gemessen 2026-09-12: der Feldpilot trägt nur 4×+/2×−
  ohne Stationsnamen/Azimute; Rückgewinnung braucht einen Feld-Neulauf, heavy
  fetch → CI), Probe-Lauf (jan76_dec25.ndk ~24 MB, schwerer Fetch → CI),
  Archivar-Verdrahtung parser-def ndk (Parser lebt in measure-lib).
- W-Phase-CMT (M9-Nachfolger) — Endpoint verifiziert (2026-09-12, anonym):
  USGS FDSN event, magnitudetype=mww (nicht includemagnitudetype); der Tensor
  liegt am by-eventid-quakeml-Pfad (momentTensor mrr/mtt/mpp/mrt/mrp/mtp +
  nodal-plane-1/2 + scalar-moment); GCMT trägt kein W-Phase-Produkt. Bau-Linie
  offen: Zentroid-Inversion auf USGS-Mww statt des M9.1-Pickers.
- Stromboli — INGV-Route + Probe gebaut; der Lauf maß kein kohärentes
  Kratersignal (0 honored). Offen: Wiring zur Tiefenphasen-Stationskorrektur
  (baut auf die gemessene Null auf).
- Die Erde als Sender (Tonga 2022) — Kreuz-Abgleich-Probe gebaut; die
  Roh-Druckwellenform bleibt blockiert (CTBTO-vDEC 403), nicht descoped.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators und der gemessene
Abschluss-Check. Der Baum trägt diese Session's uncommittete Arbeit; gepusht
wird erst, wenn er ruhig ist.
