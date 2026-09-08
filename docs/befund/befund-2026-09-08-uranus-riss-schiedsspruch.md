<!--
  title: Befund — Uranus-Riss-Schiedsspruch: die Camargo-Linie trägt keinen Schiedsspruch (geozentrisch, DE432-gewurzelt, Baryzentrum)
  class: befund
  date: 2026-09-08
  sha256: 63f080e3d03a75dc7b3de1123c57e60a160a34f6dddea09e32f194e6e47340d6
  status: done
  see-also: docs/handover/handover-2026-09-08-uranus-riss-schiedsspruch.md docs/handover/handover-2026-09-08-weberin-zweitlinien-geschlossen.md docs/concepts/die-weberin.md
-->

# Befund: Uranus-Riss-Schiedsspruch

## Frage & Bindung

Die Übergabe (handover-2026-09-08-uranus-riss-schiedsspruch.md) stellte das Atom:
die drei Ephemeriden (DE441, INPOP19a, EPM2021) gegen die Camargo+2015
Uranus-Astrometrie (`uranu_j`, 3516 Positionen 1992–2011) auswerten — Residuum
(Vorhersage − Beobachtung) je Epoche, je Ephemeride — und den Eisriesen-Riss
einem Schiedsspruch zuführen. Drei vorab genagelte Verdikte: (i) eine Ephemeride
trägt messbar besser, (ii) alle drei liegen innerhalb der Beobachtungs-Unsicherheit,
(iii) die Beobachtungen widersprechen allen dreien. Schwelle X = 2·⟨σ⟩ aus den
per-row-Fehlern des TSV (vor den Residuen, ⟨σ⟩ = 82.5 mas, X = 165.1 mas).
Probe: `tools/measure/src/bin/uranus_riss_schiedsspruch_probe.rs`.

## Die drei gemessenen Konfunde (vor jedem Verdict)

1. **Geozentrisch, nicht topozentrisch.** Die Übergabe §7 las „topozentrisch"
   und forderte die Parallaxen-Entfernung. Gemessen gegen Horizons (geozentrisch,
   DE441): die `uranu_j`-Position ist **geozentrisch astrometrisch** — der
   topozentrische Ansatz trägt die ~0,45″-Parallaxe fälschlich ins Residuum
   (607 mas), der geozentrische trägt (249 mas). Die „topozentrisch"-Aussage der
   Quelle gilt den Satelliten-Positionen, nicht der abgeleiteten Uranus-Position.
2. **DE432-gewurzelt.** `uranu_j` ist nicht beobachtet: „Positions of Uranus …
   are not observed ones" (Abstract) — die Position ist DE432+ura111
   (Satelliten-Offset ~±30–60 mas) + mittlerer Offset, auf einem DE432-Träger.
   Die beobachtete Wurzel sind die Satelliten-Positionen, nicht Uranus.
3. **Baryzentrum ≠ Planetenzentrum.** Die Ephemeris-Bins tragen das
   Uranus-System-Baryzentrum (SPK 7, de441.bsp trägt kein 799); `uranu_j` trägt
   das Planetenzentrum (via ura111). Horizons misst den Offset: 7 − 799 =
   ~518 km = ~37 mas (2011, zeitvariabel mit den Mondphasen). Common-mode in
   allen drei Linien (alle Bins Baryzentrum) — bläht das absolute Residuum,
   verfälscht die relative DE-INPOP-EPM-Reihung nicht.

Zeit: die Bins sind TDB-gebunden; `unix_to_tdb` (LK/naif0012) liefert TT — der
TDB−TT-Wobble (sub-ms, `pending`, `livefeed_gate.rs`) trägt ~40 m = ~3 mas bei
Uranus, unter den Datenfehlern. Gegen Horizons VECTORS (TDB) liegt der Bin auf
~48 m.

## Messung

Residuum (ungewichtetes RMS, mas):

| Linie     | sep    | ΔRA·cosδ | ΔDec  | along | cross |
|-----------|--------|----------|-------|-------|-------|
| de441     | 249.5  | 225.5    | 106.9 | 185.2 | 167.3 |
| inpop19a  | 226.8  | 204.5    | 98.2  | 173.1 | 146.6 |
| epm2021   | 233.9  | 195.0    | 129.2 | 159.0 | 171.5 |

Streitort (paarweise Divergenz DE-INPOP-EPM am Beobachtungsort): Spitze 74.9 mas
bei JD 2455824.6976, Mittel 57.9 mas über 3516 Epochen. ΔRMS(best, second) =
7.1 mas — die drei Linien liegen unentschieden, keine trägt die Beobachtungen
näher.

## Verdict

**(iii) — und ehrlicher: kein Schiedsspruch.** Das Residuum (227–250 mas) ist
~4× der Modell-Divergenz (58 mas) und wird vom DE432-Träger + Baryzentrum-Offset
dominiert, nicht von einem unabhängigen Beobachtungssignal. Die drei modernen
Ephemeriden stimmen untereinander überein (~58 mas) und liegen gemeinsam ~230 mas
von der DE432-gewurzelten Linie. Die Camargo-Uranus-Linie kann den Eisriesen-Riss
nicht schlichten — sie ist kein Blatt, sondern selbst ein Modellprodukt (DE432).

Die echte Wurzel — die beobachteten Satelliten-Positionen (`ariel_j` … `umbri_j`)
gegen ura111/NOE — ist `pending`.

## Register-Zeilen

- (1) Satelliten-Positionen + ura111/NOE als echte Wurzel — `pending`.
- (2) `roemer_fold`/`light_time_sc_pos` existiert fünffach probe-lokal — Heben in
  die Archivar beim nächsten Template-Griff.
- (3) Bins tragen das Uranus-System-Baryzentrum (SPK 7), nicht das
  Planetenzentrum (799) — für Astrometrie-gegen-Zentrum fehlt das Zentrum.
- (4) Camargo-TSV-Manifestation aufs CDN (raw asu-tsv) — `pending`; URL in
  `phi/sources.φ` registriert, kein Raw-tsv-Manifestor gebaut.
