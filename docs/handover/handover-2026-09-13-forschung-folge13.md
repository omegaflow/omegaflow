<!--
  title: Handover — Forschung-Folge XIII (Stand 2026-09-13)
  session: Forschung-Folge XIII
  class: handover
  date: 2026-09-13
  sha256: e04c8250224974c2a726e499b3ed453b0b79671bfa0dbfc104408a5de87f2c1b
  status: live
-->
# Handover — Forschung-Folge XIII (2026-09-13)

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
- Ⅺ Placebo — die AVE-Neumessung steht als CI-Lauf (placebo-ave-cdn.yml,
  workflow_dispatch; `--a`/`--b` default E1/E2; openneuro_compiler `--ci-mode`
  landet die .set, die Probe läuft 100 Surrogate × 24 Lags, der Verdict geht ins
  openneuro.org-Release). Der Lauf selbst ist noch nicht dispatched — die Messung
  steht aus (Operator-Wort zum Dispatch). REST pending: braucht das
  Lead-Field/Head-Modell; die chanlocs-Positionen sind jetzt geerntet
  (open_set_chanlocs, X/Y/Z), das Rohmaterial steht, Chella et al. 2016 bleibt
  der Anker (Cz die stärkste Verzerrung).

## Weberin

- VLBI-Winkel-Probe pending (PRIDE ΔDOR not-published, gemessen 2026-09-12).
- JUICE-SPICE — die Manifestation ist gebaut (spacecraft-Tabelle
  naif_spacecraft_ids.tsv, juice-Zweig in select_system, PCK-Bypass + Frame-Gate,
  Daten-Typ-Filter 2|3|9|13|20, n_dir-Fix in spk.rs; kernel-flatten.yml trägt den
  ESA-Root spiftp.esac.esa.int/data/SPICE/JUICE/ + juice in --systems;
  phi/sources.φ trägt ephemeris_juice.bin / ephemeris_juice_cog.bin +
  format-reference-Zeilen, die format-spk-Cog-Zeile ist gestrichen).
  - ephemeris_juice.bin partial: die crema-Trajektorie trägt 513 Segmente, nur 262
    (599-zentriert) lösen sich über den single-parent-Chain; die Sun-Kreuzfahrt
    (10), der Erd-Start (399), Mars (301), Venus (299) und die Mond-Baryzentren
    (503/504 — die Flyby-Bögen) bleiben pending auf eine zentrums-agnostische
    SSB-Kette. Census getragen, nie als Abwesenheit geschrieben.
  - ephemeris_juice_cog.bin pending: der COG-Frame ist nicht-inertial (−28000),
    das Frame-Gate überspringt korrekt; eine nicht-inertial→inertial-Rotation
    ist pending.
  - Der n_dir-Fix (⌊(n−1)/100⌋) trägt noch keinen Unit-Test bei n ≡ 0 mod 100 —
    die 0/514-Messung deckt crema+cog, kein Type-2-Korpus.
  - Kernel 000113+ bei ESA nach Erscheinen (Wiedervorlage 2026-09-28).

## Tiefenphasen

- W-Phase-CMT — die Ring-Streu-Auswahl steht (select_spread_stations: eine Station
  je Δ-Bin, die äußerste je Bin; der Δ-Gate bleibt unberührt). Die CI-Messung
  (depth-phase-mww.yml / cmt-ndk-fleet.yml, Dispatch) steht aus: ob Stationen
  außerhalb der Faltenregion die Gates freigeben und der NDK-vs-mww-Vergleich
  Stationen gewinnt, entscheidet die Messung.
- Positive Maske — diese Linie misst das 36-km-Streuungs-Schrumpfen, sobald die
  Treiber stehen (die Bau-Linie trägt den Compiler/Registratur-Auftrag).

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators und der gemessene
Abschluss-Check mit Commit und Push.
