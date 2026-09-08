<!--
  title: Befund — Uranus-Riss-Schiedsspruch: die Wurzel (Satelliten) misst den Riss (~32–47 mas), aber der Mondmodell-Fehler (~220 mas) trägt keinen Schiedsspruch
  class: befund
  date: 2026-09-08
  sha256: 6005a81bc930215ec214984be4f6ba6d26d6e2e37d53e0be2bcea04e059c629f
  status: done
  see-also: docs/handover/handover-2026-09-08-uranus-riss-schiedsspruch.md docs/handover/handover-2026-09-08-weberin-zweitlinien-geschlossen.md docs/concepts/die-weberin.md
-->

# Befund: Uranus-Riss-Schiedsspruch

## Frage & Bindung

Die Übergabe (handover-2026-09-08-uranus-riss-schiedsspruch.md) stellte das Atom:
die drei Ephemeriden (DE441, INPOP19a, EPM2021) gegen die Camargo+2015
Astrometrie auswerten und den Eisriesen-Riss einem Schiedsspruch zuführen. Das
Atom maß in zwei Schichten: zuerst das abgeleitete Blatt (`uranu_j`), dann die
beobachtete Wurzel (die fünf Satelliten-Tabellen). Proben:
`tools/measure/src/bin/uranus_riss_schiedsspruch_probe.rs` (Blatt),
`tools/measure/src/bin/uranus_satellite_schiedsspruch_probe.rs` (Wurzel).

## Das Blatt — `uranu_j` trägt keinen Schiedsspruch (drei Konfunde)

1. **Geozentrisch, nicht topozentrisch.** Die Übergabe §7 las „topozentrisch".
   Gemessen gegen Horizons: die `uranu_j`-Position ist **geozentrisch astrometrisch** —
   der topozentrische Ansatz trägt die ~0,45″-Parallaxe fälschlich ins Residuum.
2. **DE432-gewurzelt.** „Positions of Uranus … are not observed ones" (Abstract) —
   die Position ist DE432+ura111 abgeleitet, kein Beobachtungssignal.
3. **Baryzentrum ≠ Planetenzentrum.** Die Bins tragen SPK 7 (Baryzentrum),
   `uranu_j` das Planetenzentrum; der Offset (~37 mas @2011) liegt common-mode.

Residuum (RMS, mas): de441 249.5, inpop19a 226.8, epm2021 233.9, ΔRMS 7.1 mas —
keine Linie trägt näher; das Residuum ist der DE432-Träger, kein Blatt.

## Die Wurzel — die fünf Satelliten

Geerntet: die fünf beobachteten Satelliten-Tabellen (`ariel_j` … `miran_j`,
9797 Positionen, geozentrisch astrometrisch — gegen Horizons auf ~0,1 mas
verifiziert) + der ura111-SPK (Satellit − Baryzentrum, baryzentrum-unabhängig).
Reduktion: `Satellit = Baryzentrum(DE441/INPOP19a/EPM2021) + (Satellit − 7)(ura111)` —
der Mondmodell-Fehler ist common-mode, er kürzt sich in den Differenzen.

Residuum (RMS, mas): de441 242.9, inpop19a 222.6, epm2021 228.9, ΔRMS 6.3 mas.

Mittlerer Residuen-Vektor (mas, 9797 Epochen):

| Linie    | ΔRA·cosδ | ΔDec  |
|----------|----------|-------|
| de441    | +149.4   | −71.6 |
| inpop19a | +128.2   | −47.5 |
| epm2021  | +116.2   | −92.9 |

Paarweise Mittelwert-Differenzen — **der beobachtete Riss** (Mondmodell-Fehler
kürzt sich; Standardfehler ~1,3 mas):

- de441 − inpop19a: ΔRA +21.2, ΔDec −24.1 → **32.1 mas**
- de441 − epm2021:  ΔRA +33.2, ΔDec +21.3 → **39.5 mas**
- inpop19a − epm2021: ΔRA +12.0, ΔDec +45.4 → **47.0 mas**

## Verdict

Der Riss ist echt und durch die Wurzel messbar: **~32–47 mas** zwischen den drei
Ephemeriden. Aber er ist kleiner als der Mondmodell-Fehler (ura111, ~220 mas
gegen ura184, das auf ~0,1 mas trägt) und kleiner als die Datenfehler (~88 mas).
Deshalb trägt die Astrometrie **keinen Schiedsspruch**: sie misst, dass die drei
Ephemeriden divergieren, aber der absolute Offset (welche Linie am nächsten an
der Wahrheit liegt) steckt unter dem Mondmodell-Fehler. Der RMS-Vergleich bleibt
unentschieden (ΔRMS 6–7 mas), weil der common-mode-Fehler das Signal überragt.

Der Schiedsspruch braucht ein **baryzentrum-unabhängiges Mondmodell auf mas-Niveau**
(ura184-Niveau, aber ohne den DE441-Baryzentrum eingebacken) — das ist die offene
Pflicht.

## Register-Zeilen

- (1) Baryzentrum-unabhängiges Mondmodell auf mas-Niveau (ura184-Split) — `pending`;
  damit wäre der Schiedsspruch entscheidbar.
- (2) `roemer_fold`/`light_time_sc_pos` existiert sechsfach probe-lokal — Heben in
  die Archivar beim nächsten Template-Griff.
- (3) Bins tragen das Uranus-System-Baryzentrum (SPK 7), nicht das
  Planetenzentrum (799) — für Astrometrie-gegen-Zentrum fehlt das Zentrum.
- (4) Camargo-TSV- und ura111-SPK-Manifestation aufs CDN — `pending`; URLs in
  `phi/sources.φ` registriert, kein Raw-Manifestor gebaut.
