<!--
  title: Handover — Forschung-Folge XIV (Stand 2026-09-13)
  session: Forschung-Folge XIV
  class: handover
  date: 2026-09-13
  sha256: 82b31b74ab607dd2534e0e5848ea8ceb47958a953fae8e1208fdd9b95b50f2d4
  status: live
-->
# Handover — Forschung-Folge XIV (2026-09-13)

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
- Ⅺ Placebo — die AVE-Neumessung läuft in CI (placebo-ave-cdn, run 34765685872,
  E1/E2); der Verdict landet im openneuro.org-Release. REST pending: braucht das
  Lead-Field/Head-Modell (chanlocs X/Y/Z geerntet, Chella et al. 2016 der Anker).

## Weberin

- VLBI-Winkel-Probe pending (PRIDE ΔDOR not-published, gemessen 2026-09-12).
- Census der zentrums-agnostischen SSB-Kette pending: die Kette steht im Code
  (`state_ssb_multi` folgt dem tatsächlich deckenden Segment-Zentrum über alle
  Kernel, Backtracking bei Flyby-Überlapp, 32-Schritt-Guard + visited) und der
  n_dir-Unit-Test steht (4 Tests, n≡0 mod 100). Ob die 251 nicht-599-zentrierten
  crema-Segmente (Sun 10, Erde 399, Mars 301, Venus 299, Mond-Baryzentren 503/504)
  jetzt zum SSB aufgehen, misst der nächste kernel-flatten-Lauf (oder ein
  Dispatch): ein lokaler cargo-Test war durch die fremde, unfertige
  las-Refaktorierung blockiert (16 Compile-Fehler in src/archivar/las/laszip.rs,
  nicht diese Linie).
- ephemeris_juice_cog.bin pending: der COG-Frame −28000 ist JUICE_SPACECRAFT, ein
  CK-basierter SWITCH-Frame (class 6), aligned zu JUICE_SPACECRAFT_PLAN (−28001) /
  JUICE_SPACECRAFT_MEAS (−28002, beide class 3 CK). Alle Kernel liegen am ESA-Root
  (juice_v46.tf, CK .bc, SCLK .tsc, LSK naif0012.tls). Es fehlt ein CK-Reader +
  SWITCH-Frame-Auflösung im Compiler (kein PCK nötig; die COG-Position ist ein
  fester Offset (0, 0, −1.5322 m) im Körperframe).
- Kernel 000113+ bei ESA nach Erscheinen (Wiedervorlage 2026-09-28).

## Tiefenphasen

- W-Phase-CMT — dispatched: depth-phase-mww (run 34765688099) und cmt-ndk-fleet
  (run 34765690409) laufen in CI; der NDK-vs-mww-Stationen-Vergleich steht aus den
  Artefakten.
- Positive Maske — misst das 36-km-Schrumpfen, sobald die Treiber stehen.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators und der gemessene Abschluss-Check.
