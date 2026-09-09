<!--
  title: Befund — feinere Tiefen-Inversionsklasse (1 km) + gemeinsame Picking-Lib: die 50-km-Raste im 200–250-km-Band ist gekreuzt (231 km invertiert auf 231 km), der Kern zieht aus den zwei Proben in depthphase.rs, verhaltensneutral getestet
  class: befund
  date: 2026-09-09
  sha256: 635434279bd2d9bea2e11f7b75d2117b07f98af5c471161166696ebebdb99768
  status: done
  see-also: docs/handover/archiv/handover-2026-09-09-tiefenphasen-flotte.md docs/befund/befund-2026-09-09-tiefenphasen-feld.md docs/befund/befund-2026-09-09-tiefenphasen-diagonale.md docs/concepts/die-akteure-im-boden-und-wasser.md
-->

# Befund: die feinere Tiefen-Inversionsklasse (1 km) — die 50-km-Raste im 200–250-km-Band ist gekreuzt

## Frage & Bindung

Der Feldpilot rastete die Tiefe in `DEPTHS_FINE` (50-km-Schritte im Bereich
200–250 km); der Katalog-Wert 231 km lag zwischen zwei Rasterwerten — das
+19-km-Offset trug auch Raster-Rauschen (Befund
`befund-2026-09-09-tiefenphasen-feld.md`, Punkt 4). Der benannte Nachfolger:
unter die 50-km-Raste gehen.

## Was gebaut wurde

`tools/measure/src/depthphase.rs` (gemeinsame Lib, `pub mod depthphase`): der
ganze Picking-/Inversions-Kern der zwei Proben (Bandpass, P-Onset, SNR-Gate,
Korrelation, Lag-Helfer, Inversion) zieht aus `depth_phase_field_probe.rs`
und `depth_phase_probe.rs` in die Lib; beide Bins werden dünne `main()`s.
Verhaltensneutral: die 10 Tests (5 Feldpilot + 5 Diagonale) ziehen mit und
bleiben grün — jetzt 11 Lib-Tests, plus der neue Kreuz-Test unten.

Die Inversions-Klasse ist neu 1 km: `invert_depth_single` und
`invert_depth_multi` rasen über 0–250 km in 1-km-Schritten statt der 21
Werte. Das Vorwärtsmodell (`p_p_grid`/`depth_grid` in `ak135.rs`)
interpoliert die Tiefe ohnehin stetig; die Raste saß allein in der
Inversion. Weil der Lag ~2 h/vp nahezu linear in h ist, liest die 1-km-Raste
die stückweise-lineare Modellkurve getreu.

## Die Messung (der Test als Messung)

Der neue Test `invert_single_resolves_between_the_deep_grid_nodes`: eine
synthetische Quelle bei 231,0 km (dem Katalog-Wert des Piloten) invertiert
auf 231,0 km — Abweichung ≤ 1 km. Die 50-km-Raste im 200–250-km-Band ist
gekreuzt; die Inversions-Klasse ist nicht mehr der Engpass.

## Die benannte Grenze (A = A)

Die Modell-Stützstellen selbst bleiben 50 km auseinander (Knoten 200, 250 km
in `DEPTH_KM`). Die 1-km-Lesart steht auf dem linearen Rampenstück zwischen
ihnen — getreu, weil der Lag dort nahezu linear ist, aber sie trägt dort
keine eigenständige 1-km-Information des Modells. Eine dichtere
`DEPTH_KM`-Leiter bleibt als benannte Folge, falls eine künftige Messung sie
verlangt.

Zweite benannte Grenze: `MAX_DEPTH_KM = 250` in `ak135.rs` kappt die
Tiefenphasen bei 250 km. Die Zonen-Flotte (Tiefherd-Ereignisse, Tonga
410–660 km) kann erst invertieren, wenn das Tiefenmodell über 250 km
erweitert ist — eine Register-Pflicht vor dem Zonen-Lauf, kein stiller Wert.

## Schließung

Der Nachfolger „feinere Tiefen-Inversionsklasse" schließt als gebaut und
getestet (1 km, Kreuz-Test über die 200/250-km-Knoten). Offen bleiben: die
Flotte (Ereignisse × Stationen, σ und √N), die pP-Polarität an tiefer
Geometrie, und — vor dem Zonen-Lauf — die ak135-Tiefenmodell-Erweiterung
über 250 km.
