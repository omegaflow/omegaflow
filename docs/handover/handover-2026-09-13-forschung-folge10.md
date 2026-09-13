<!--
  title: Handover — Forschung-Folge X (Stand 2026-09-13)
  session: Forschung-Folge X
  class: handover
  date: 2026-09-13
  sha256: b2f90d2e289d8f47a53c16a53b8d9118cac7d9cbc19fce8c50f3ee0ab8cdace5
  status: live
-->
# Handover — Forschung-Folge X (2026-09-13)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

## Analyse (Flut-Satellit)

- S1 — Wächter: das S1S2-Asset (MAGIC S1S2, Stride 56) am CDN
  sentinel1euwest.blob.core.windows.net messen (s1-sar-cdn Run 34726981855; das
  alte s1_sar_diff.bin-Asset ist gelöscht).
- DEM — Wächter: copernicus_dem_30m_N50_E010.bin am CDN
  copernicus-dem-30m.s3.amazonaws.com messen (copernicus-dem-cdn Run 34726993789).

## Nadeln

- Ⅰ Jeans-Residuum bis Gaia DR4 (Wiedervorlage 2026-12-02).
- Ⅱ JUICE-Flyby 28./29.9. (Wiedervorlage 2026-09-28) · Europa Clipper 3.12.
  (Wiedervorlage 2026-12-03).
- gaia-dr4-iapetus (Wiedervorlage 2026-12-02).
- Ⅺ Placebo — der MAT-v5-Reader liest jetzt das echte ds005034-Paar (mxSTRUCT-
  Array element-major, flattened-EEG-Abbildung); der bivariate Schätzer läuft
  binned (O(n)); der erste Live-Lauf misst: das Placebo hält (verum E1↔E2,
  0 Pfeile über fam-Schwelle). Offen: (a) die bedingte TE mit einem
  --c-Gemeinursache-Kanal (die bedingte TE blieb pending — kein --c genannt);
  (b) die Registratur — ds005034 trägt keine sources.φ-Zeile und keinen
  OpenNeuro-Compiler (CDN-Manifestation-Duty); die Daten liegen nur als lokale
  gitignored-Kopie; (c) der honest lag sitzt am maximalen Lag (24) und die TE
  wächst monoton mit dem Lag — ein Messwert, der nicht als Lag-Kopplung gelesen
  wird.

## Galileo-Floor

- Rausch-Kurve — Wächter: das galileo-trk-noise-Artefakt verifiziert die
  ODF-Format-1-vs-2-Kurve (Run 34726993957); der Format-Zweig bleibt ungemessen,
  bis das Artefakt landet.

## Weberin

- VLBI-Winkel-Probe pending (PRIDE ΔDOR nicht publiziert).

## Tiefenphasen

- CMT — Wächter: die cmt-ndk-fleet-Artefakte (feld-neulauf + probe-lauf, Run
  34726993968) verifizieren den Kalibrier-Gate.
- W-Phase-CMT — der --mww-Gate ist verdrahtet (raw-Tensor → ndk::rp, Zentroid-
  Anker all-or-nothing); der Feld-Lauf mit --mww ist die nächste Messung (der
  Dispatch lief nur --kalibrier; der Vergleich NDK vs mww steht aus).
- Stromboli — das Vorzeichen ist im Code bestimmt (apply_station_term subtrahiert,
  der Test assertet es); offen ist die physikalische Anwendung, die einen
  nicht-nullen Term braucht (Kohärenz-Scan eines Kraterquells);
  stromboli-station-term.yml steht als Instrument (--end für ein
  Paroxysmus-Fenster).
- Tonga 2022 — die Roh-Druckwellenform bleibt blockiert (CTBTO-vDEC 403), nicht
  descoped.
- Positive Maske — pending auf die Ernte-Linie (slab2_compiler ist eine fremde
  Session-Arbeit, nie als eigene geführt); die 3D-Modelle sind eine Registratur-
  Duty (CDN-Manifestation); diese Linie misst das 36-km-Streuungs-Schrumpfen,
  sobald die Treiber stehen. Der DEM-Dispatch (Gestalt-Treiber) steht.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators und der gemessene
Abschluss-Check mit Commit und Push.
