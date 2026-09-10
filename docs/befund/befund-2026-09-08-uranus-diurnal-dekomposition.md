<!--
  title: Befund — Uranus-Diurnal-Dekomposition (korrigiert): die Camargo-Tabellen tragen die topozentrische Parallaxe (c_par ≈ 0.95) und keine Diurnal-Aberration — topozentrisch astrometrisch, das Papier bestätigt
  class: befund
  date: 2026-09-08
  sha256: 67fce571f656ef39b4b39d68ee23254798c97a0a013641cdfa323108051d130a
  status: done
  see-also: docs/handover/archiv/handover-2026-09-08-uranus-riss-diurnal-reduktion.md docs/befund/befund-2026-09-08-uranus-riss-schiedsspruch.md docs/handover/archiv/handover-2026-09-09-mechanische-reste.md
-->

# Befund: Uranus-Diurnal-Dekomposition (korrigiert)

## Frage & Bindung

Die Übergabe (handover-2026-09-08-uranus-riss-diurnal-reduktion.md) stellte das
Atom: den Diurnal-Term im absoluten Satelliten-Residuum zerlegen — Parallaxe
gegen Diurnal-Aberration —, die volle topozentrische + Aberrations-Reduktion
bauen, den absoluten Baryzentrum-Offset sichtbar machen, den Schiedsspruch
entscheiden. Proben:
`tools/measure/src/bin/uranus_diurnal_decomposition_probe.rs` (Zerlegung) und
`tools/measure/src/bin/uranus_floor_decomposition_probe.rs` (Boden).

## Die Instrumenten-Archäologie (ehrlich benannt)

Die erste Instrument-Version (Commit `47a8d92`) nutzte `body_fixed_to_icrs`
für die Station — der Matrix-Pfad wählt die **nächste** Rotationsmatrix
(32-Tage-Raster der 32d-Chebyshev-Granulen). Gemessen: die Parallaxen-Signatur
war innerhalb einer Nacht **eingefroren** (par_ra konstant über 1,3 h statt
der Stundenwinkel-Signatur). Die erste Messung (c_par ≈ 0, „Tabellen
geozentrisch") war ein Instrumenten-Artefakt. Das Kalibrier-Gate schützte
nicht: es prüft die Fit-Maschinerie gegen sich selbst (injizierte Signatur →
exakte Rückgewinnung), nicht die physikalische Form der Signatur — der
physikalische Prüfstein ist die Stundenwinkel-Variation innerhalb einer Nacht.

Die Korrektur: die Station ist jetzt **probe-lokal** — WGS84-Geodätik + IAU
1982 GMST-Rotation + analytische Rotationsgeschwindigkeit ω×r (eine Station
für alle drei Linien, ein physikalischer Beobachter). Zwei weitere Befunde am
Weg: der IAU-Fallback `iau_rotate_to_icrs` in `src/archivar/motion.rs` bildet
den Erdpfahl (δ0 = 90°) falsch ab (gemessene Zenith-Breite +14.6° statt
−22.5°) — register-pending, `src/` ist durch die Parallel-Session belegt; und
der Matrix-Snap selbst (32-Tage-Einfrierung) ist eine gemessene Schwäche des
Archivar-Pfads. Das Kalibrier-Gate (alle vier Fälle) läuft auch mit der
korrigierten Station exakt.

## Die Messung (korrigiert)

Gepoolt (alle Satelliten je Linie):

| Linie    | c_par        | c_aber       | RMS roh → reduziert (mas) | c0 (ΔRA·cosδ, ΔDec, mas) |
|----------|--------------|--------------|---------------------------|--------------------------|
| de441    | 0.95 ± 0.00  | −0.06 ± 0.01 | 242.9 → 77.5              | (−26.9, −19.6)           |
| inpop19a | 0.95 ± 0.00  | −0.03 ± 0.01 | 222.6 → 73.6              | (−13.3, −42.9)           |
| epm2021  | 0.94 ± 0.00  | −0.02 ± 0.01 | 228.9 → 73.3              | (−5.7, +3.8)             |

Pro Satellit: c_par 0.95–0.97 für die vier großen Monde, 0.80–0.81 für Miranda
(ihr Rest bleibt 106–109 mas — das schwächste Mondmodell). c_aber ≈ 0 für alle
Linien.

**Die publizierten Camargo-Tabellen tragen die topozentrische Parallaxe im
Wesentlichen vollständig (c_par ≈ 0.95) und keine Diurnal-Aberration — sie
sind topozentrisch astrometrisch, exakt wie das Papier sagt** (Camargo+ 2015
§4: SOFA/NOVAS-Verknüpfung „for an observer at the Pico dos Dias
Observatory"; Anhang: „We stress that our positions are topocentric"). Die
Diurnal-Term-Hypothese der Übergabe ist bestätigt: der ~200-mas-Term war die
Parallaxe (Signatur-Mittel 203 mas, Max 405 mas). Das 0.95 (statt 1.00) liegt
auf der Skala des Mondmodell-Leaks (ura111 ≈ ura184 auf 5–25 mas).

## Die Konsequenz für das Atom

- **Der absolute Offset ist jetzt sichtbar.** Nach der Parallaxen-Reduktion
  (gepoolte Konstanten c0): epm2021 (−5.7, +3.8) mas → Betrag 6.9 mas,
  de441 (−26.9, −19.6) → 33.3 mas, inpop19a (−13.3, −42.9) → 44.9 mas.
- **Der Boden ist geschlossen.** Das reduzierte Residuum liegt bei
  RMS 72.9–76.9 mas — **unter** der per-row-Skala (⟨σ⟩ = 87.8 mas); die
  Boden-Probe misst nachts-konstant ~72 %, intra-Nacht ~28 % der Restvarianz
  und einen Zenith/Refraktions-Koeffizienten von nur 4–8 mas Amplitude. Kein
  unbenannter Boden bleibt: der ~165-mas-„Floor" der Vormessung war die
  Parallaxe selbst.
- **Der Riss steht.** Rohe paarweise Mittelwert-Differenzen 32.1/39.5/47.0 mas
  (der Anker der Übergabe §4); die paarweisen c0-Differenzen nach der
  Reduktion 27.0/31.5/47.3 mas — dieselbe Ordnung, stabil.
- **Der 598-mas-Befund der Vorsitzung** („naive Topozentrik macht es
  schlechter") ist erklärt: eine falsch verortete/gesetzte Station addierte
  die Parallaxen-Schwingung auf die ohnehin topozentrischen Tabellen.

## Verdict

(ii) **Die Beobachtungen schlichten nicht.** Nach der Reduktion: RMS 72.9
(epm2021) / 73.1 (inpop19a) / 76.9 (de441) mas, ΔRMS 0.3–3.9 mas ≪ X = 175.6;
die schlechteste Linie liegt weit unter X, das Residuum unter der
per-row-Unsicherheit. Der Riss (32–47 mas) bleibt der Eichanker: er ist echt
und gemessen, aber die Camargo-Wurzel trägt weiterhin keinen Schiedsspruch —
die drei Linien liegen nach der korrekten Reduktion alle auf dem
Rausch-Niveau der Astrometrie.

## Register-Zeilen

- (1) `iau_rotate_to_icrs` bildet den Erdpfahl falsch ab (gemessen: Zenith
  +14.6° statt −22.5°) — pending, `src/` belegt (Parallel-Session).
- (2) Matrix-Snap in `body_fixed_to_icrs` (32-Tage-Raster, Station
  eingefroren) — pending, `src/` belegt; die Proben tragen ihre eigene
  WGS84+GMST-Station.
- (3) INPOP/EPM-Earth-Bins ohne body-fixed Orientierung — behoben in
  `tools/harvest` (--pck pck00010/00011), lokal neu kompiliert und audit-
  verifiziert (120/120); die CDN-Re-Manifestation ist ein CI-Lauf — pending.
- (4) roemer_fold siebenfach probe-lokal — Heben in die Archivar pending bis
  `src/` ruhig.
- Die beiden alten Pendings „~170-mas-Boden zerlegen" und „Papier-gegen-
  Tabellen-Diskrepanz" schließen mit diesem Befund: der Boden war die
  Parallaxe, eine Diskrepanz besteht nicht.
