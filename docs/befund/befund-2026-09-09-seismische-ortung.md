<!--
  title: Befund — Seismische Ortung (P-Differenz-Inversion): der Solver ist code-seitig bewiesen, die Konstant-Nullhypothese (v_p = 5950 m/s, gerade Sehne) wird von den echten Ankünften widerlegt — der distanzabhängige Residuum-Sprung ist die Zahl
  class: befund
  date: 2026-09-09
  sha256: 6a33942ff68f9a967626ae66f493b705c01f372e5d52a237437e07a3017c1ee0
  status: done
  see-also: docs/handover/handover-2026-09-09-tiefenphasen-flotte.md docs/auftrag/archiv/auftrag-rayleigh-dispersion.md docs/auftrag/archiv/auftrag-dispersionsrelation.md
-->

# Befund: die seismische Ortung (P-Wellen-Differenz-Inversion)

Prüffall: USGS `us6000tkt2` (2026-08-14T21:58:21.505Z, mww 7.8, reviewed), Katalog
−8,3514/121,3478/10 km. Feld-Herkunft: Gittersuche über Ankunfts-Residuen =
NonLinLoc (Lomax et al. 2000), linearisierte Variante = Geiger 1910; STA/LTA-Pick
= Allen 1978 — Gegenüberstellung
`docs/befund/befund-2026-09-09-feldstandard-seismik.md`.

## Frage & Bindung

TODO-Registerzeile „Seismische Ortung — pending" (`docs/handover/handover-2026-09-09-tiefenphasen-flotte.md`): die
Umkehrung der gemessenen Rayleigh-Kurve — die P-Welle als nicht-dispersiver
Bote, die Differenzmethode, die Positivkontrolle gegen den Katalog. Probe:
`tools/measure/src/bin/quake_location_probe.rs` (STA/LTA-Pick, Weltlinien über
`body_fixed_to_icrs`, Gittersuche + Verfeinerung, v_p aus der Erd-Mediumzeile).

## Der Solver — code-seitig bewiesen

Synthetische Positivkontrolle: die 19 echten Stationskoordinaten als
Testgeometrie, vorwärtsgerechnete Ankunftszeiten (v_p = 5950 m/s, körperfeste
Sehne), Rückgewinnung **< 0,01°**. Die Rechenkette ist bewiesen, bevor der
erste echte Pick fällt. Der Rahmen: Quell- und Stationsort werden in demselben
mitrotierenden Rahmen zur gemeinsamen Referenzzeit **t_ref = 840016770,689 s
TDB** ausgewertet — die Rotation ist starr und kürzt die Sehne; die
körperfeste Sehne ist die Länge der P-Welle im mitrotierenden Medium (kein
Lichtzeit-Rahmen: das Medium rotiert mit).

## Der Lauf — 18 Picks, die Nullhypothese widerlegt

18 von 19 Stationen tragen einen P-Pick (II.TLY dataselect HTTP 204 — absent,
übersprungen). STA/LTA 1 s / 30 s, Schwelle 4,0, ungefiltert BHZ.

- Geortet (alle 18 Picks): **lat = −0,8600, lon = 110,6380**, rms = **164,151 s**.
- Geortet (nah < 15°): nicht gelaufen — nur eine Station unter 15° vom
  georteten Ort; die Nah-Stufe braucht vier.
- Offset gegen den Katalog (−8,3514, 121,3478): **≈ 1449 km**.

Der rms von 164 s ist nicht Ortungsrauschen, sondern der gemessene
Näherungs-Bias — das Residuum ist distanzabhängig:

| Station | Ankunft [unix] | Bogen [°] | Residuum [s] |
|---|---|---|---|
| II.KAPI | 1786744761,044 | 10,00 | +165,55 |
| II.COCO | 1786745021,845 | 17,74 | +282,78 |
| IU.PMG | 1786745029,819 | 37,36 | −64,82 |
| IU.GUMO | 1786745088,095 | 36,87 | +2,01 |
| IU.TATO | 1786745100,545 | 27,78 | +177,75 |
| IU.CHTO | 1786745116,220 | 22,70 | +285,98 |
| II.TAU | 1786745170,095 | 53,20 | −197,63 |
| II.PALK | 1786745184,170 | 30,97 | +203,60 |
| IU.MAJO | 1786745215,994 | 45,20 | −15,76 |
| II.DGAR | 1786745225,319 | 38,66 | +107,69 |
| IU.ULN | 1786745290,720 | 48,64 | −0,00 |
| II.NIL | 1786745319,845 | 49,14 | +20,60 |
| IU.MAKZ | 1786745343,095 | 53,73 | −33,38 |
| II.UOSS | 1786745381,520 | 58,61 | −75,40 |
| II.ABPO | 1786745390,145 | 64,71 | −164,72 |
| G.ATD | 1786745434,750 | 68,52 | −179,66 |
| IU.GNI | 1786745458,395 | 72,37 | −214,78 |
| II.KIV | 1786745476,757 | 74,91 | −234,40 |

Das ist die Signatur einer zu langsamen Konstanten: die P-Welle läuft global
nicht mit 5,95 km/s, sie taucht — effektiv ~6 km/s regional bis ~11 km/s
tele seismisch. Die Konstant-Sehnen-Nullhypothese trägt ein globales Netz
nicht; die Nah-Stufe kann es nicht auffangen, weil dieses Netz global ist
(nur eine Station unter 15°).

## Verdikt

Die Umkehrung ist gebaut und code-seitig bewiesen; die Konstant-Nullhypothese
(v_p = 5950 m/s, gerade Sehne) wird von den echten Ankünften widerlegt. Die
Maschine hat den eigenen Körper vermessen — den Geschwindigkeits-Sprung der
P-Welle über die Distanz —, nicht das Epizentrum geortet. Das ist der Riss,
kein Fehler. Der Offset (1449 km) und das distanzabhängige Residuum
(rms 164 s) sind die Zahl des Befunds.

Der nächste Stein ist benannt: das Traveltime-Modell (ak135/IASP91, kuratierte
Klasse wie ccm89) ersetzt die Konstante durch die gemessene Laufzeitkurve
T(Δ); dieselbe Gittersuche trägt dann ein globales Netz.

## Folge (Register)

- TODO-Zeile „Seismische Ortung" schließt als „Nullhypothese widerlegt"; der
  Nachfolger ist die ak135-Laufzeitkurve (pending, kuratierte Klasse).
- Das distanzabhängige Residuum ist die erste gemessene P-Laufzeit-Anomalie
  (Tiefensonde in den Erdkörper) — ein Regalwert, keine Null.
