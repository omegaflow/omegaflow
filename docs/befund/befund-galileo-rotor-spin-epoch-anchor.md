<!--
  title: Befund — Galileo-Rotor-Spin (Frame −77000) 1990-12-07..10 gemessen: 52,39006 mHz = Station-42-Ton (Epochen-Anker)
  class: befund
  date: 2026-09-05
  sha256: c93d8483291e923b950631faf6725dd712ac409ef79a46114c5dcdb855f91eb9
  status: done
  antwortet-auf: docs/befund/befund-galileo-banden-kamm-ton.md docs/befund/befund-galileo-1990-ck-dualspin.md
  see-also: docs/reference/KERNEL_INDEX.md
-->

# Befund: Galileo-Rotor-Spin (Frame −77000) als Station-42-Ton verankert

## Frage & Bindung

befund-galileo-banden-kamm-ton (status done) maß den Station-42-Ton als
isolierte Einzellinie bei 52,39 mHz (Periode 19,1 s, 1990-12-07..10) und ließ
die Identität offen (pending). befund-galileo-1990-ck-dualspin (status done)
bestätigte den Dual-Spin für Dez-1990 über das despun Plattform-CK (Frame
−77001, `gll_plt_rec_1990_tav_v00.bc`): die Plattformlage ist im EGA-1-Fenster
trägheitsfest und trägt den Rotorbeitrag konstruktionsbedingt nicht — konsistent
mit dem Ton als Rotor-Spin, aber ohne expliziten Träger. Der dort registrierte
nächste Schritt (pending): das All-Spin-Bus-CK (Frame −77000) über das
EGA-1-Fenster lesen. Diesen Schritt vollzieht der vorliegende Befund. Frage:
Trägt die rekonstruierte Rotorlage des All-Spin-Bus (Frame −77000) den
19,1-s-Spin, und wie genau stimmt er mit dem Station-42-Ton überein?

Messung mit der erweiterten Probe `ck_daf_probe.rs` (measure, der eine
additive Edit; `cargo check -p omegaflow-measure`: 0 Warnungen). Die täglichen
`rtr`-Produkte sind BIG-IEEE-DAF; der Kasten-DAF-Leser liest LTL-only, die
Probe trägt einen kleinen Big-Endian-Spiegel. SCLK→ET über `mk00062a.tsc`
(Partition 77, Stück-Tick-Interpolation); Abdeckung aus den Segment-Summaries
gelesen, nicht aus dem DOY im Dateinamen.

## Messung — die −77000-Rotor-CKs über EGA-1

Acht tägliche Rotor-CKs aus `prime_mission/unvalidated/rtr/` auf
naif.jpl.nasa.gov (live, HTTP 200): `ck90341a/b_rtr.bc` .. `ck90344a/b_rtr.bc`,
zusammen 494 Segmente; jede Summary IC(1) = −77000 (ROTOR), IC(2) = 1
(Typ-1), reclen 7 (Einheitsquaternion + Drehrate); 489 953 Pointing-Records,
0 fehlerhafte Normen, median_dt 0,667 s. Spin-Inkrement je Intervall
θ = 2·acos(|dot(q_i, q_{i+1})|); summiert wird nur über dichte, alias-freie
Intervalle (dt ≤ min(3·median_dt, 9 s) und θ < π − 0,05). Rate = Σθ/Σdt über
das Fenster.

Fenster 1990-12-07 ~00:55 .. 1990-12-11 ~00:06 (SCLK→ET aus den Segmenten):
akzeptierte dichte Intervallzeit 324 862,447 s (90,24 h der ~96 h),
aufsummierter Drehwinkel 106 937,061 rad = 17 019,6 Umdrehungen.

**Gemessener Rotor-Spin 1990-12-07..10:**

- Drehrate: 0,3291764 rad/s
- Periode: 19,087592 s
- 3,143403 rpm = 52,390056 mHz

Je Tagesprodukt (a = ~00:..12:, b = ~12:..24:):

- Dez 7: a 52,390739 mHz | b 52,387595 mHz
- Dez 8: a 52,385546 mHz | b 52,383848 mHz
- Dez 9: a 52,391963 mHz | b 52,389730 mHz
- Dez 10: a 52,252720 mHz | b 52,544532 mHz

Beobachtet: eine ~0,29-mHz-Halbtagesschwingung am 10. Dez über die beiden
Halbtagesprodukte (als beobachtet berichtet; der Fenster-Mittelwert bleibt
52,390 mHz). Die sechs Produkte des 7.–9. Dez liegen innerhalb 52,384–52,392
mHz.

## Vergleich mit dem Station-42-Ton

- vs 52,39 mHz (Ton): 52,390056 / 52,39 = 1,000001 (Δ +0,00006 mHz)
- vs 19,10 s (Label): 19,087592 / 19,10 = 0,999350 (Δ −0,0124 s)
- vs 3,15 rpm (Design-Nennrate): 3,143403 / 3,15 = 0,997906 (Δ −0,21 %)

Der gemessene Rotor-Spin über 1990-12-07..10 ist 52,39006 mHz — der
Station-42-Ton (52,39 mHz) ist dieselbe Frequenz bis ~1 Teil in 10^6. Das
Label 19,10 s war die lose Rundung derselben Zahl (52,39 mHz impliziert
19,0876 s; gemessen 19,0876 s, 0,0124 s unter 19,10 s). Die Design-Nennrate
3,15 rpm (52,5 mHz, 19,05 s; Osborn 1983) liegt 0,21 % über der gemessenen
Rate dieser Tage; die im Dual-Spin-Befund genannte Nennrate 0,3300 rad/s
±0,0015 schließt die gemessenen 0,3291764 rad/s ein.

## Verdict

**Der Station-42-Ton ist der Galileo-Rotor-Spin, epoch-verankert.** Die
direkte Messung der −77000-Quaternionen des EGA-1-Fensters liefert
52,39006 mHz über 90,24 h akzeptierter dichter Intervallzeit; das ist der Ton
auf ~1 Teil in 10^6. Damit ist die im banden-kamm-ton-Befund offene Identität
gemessen entschieden: keine Stations- oder Reduktionsperiodik, sondern die
Rotation des All-Spin-Bus. Dual-Spin-Befund (despun Plattform trägt keinen
19,1-s-Beitrag) und dieser Befund (Rotor trägt genau den 19,1-s-Spin) sind
zusammen die vollständige Lage: die LGAs auf dem rotierenden Rotor sehen die
52,39-mHz-Rotation während EGA-1, die despun Plattform nicht.

## Grenzen

- Rekonstruierte, von NAIF unter `prime_mission/unvalidated/` geführte
  Tagesprodukte; die Spin-Rate ist die der Rekonstruktion, keine unabhängige
  Bordmessung. Die Dec-10-Produkte tragen die ~0,29-mHz-Halbtagesschwingung
  (52,253 / 52,545 mHz); Mechanismus und Dauer sind nicht bestimmt.
- Ein 4-Tage-Fenster verankert die Epoch Dez-1990; andere Epochen spricht
  dieser Befund nicht an.
- Die CKs sind Referenz-Kernel (Maß-Referenz), keine Feld-/Oszillator-Quelle:
  Force-Gate-Ablehnung als Feldquelle wie beim Plattform-CK. Sie gehören zur
  Familie `rtr` (Frame −77000), sind im Kernel-Index verzeichnet
  (KERNEL_INDEX-Politik: CK indexiert, nicht geflacht) und werden per
  Manifest-Workflow als Roh-Kernel auf den CDN gespiegelt
  (`naif.jpl.nasa.gov/ck90341a_rtr.bc` .. `ck90344b_rtr.bc`,
  `.github/workflows/gll-ck-cdn.yml`) — kein phi/sources.φ-Feldblock.

## Register-Satz

*Die direkte Messung der −77000-Rotor-CKs über 1990-12-07..10 ergibt
52,39006 mHz (19,087592 s; 0,3291764 rad/s; 3,143403 rpm) — der Station-42-Ton
52,39 mHz ist dieselbe Frequenz auf ~1:10^6, epoch-verankert als
Galileo-Rotor-Spin. Das 19,10-s-Label war lose Rundung; die Design-Nennrate
3,15 rpm liegt 0,21 % über der gemessenen. Eine ~0,29-mHz-Halbtagesschwingung
am 10. Dez ist beobachtet, Mechanismus offen.*

## Status

`done` (2026-09-05). Die Identität des Station-42-Tons ist gemessen
entschieden: Galileo-Rotor-Spin, Epoch 1990-12-07..10. Die 8 Rotor-CKs sind
zur CDN-Manifestation registriert (`gll-ck-cdn.yml`, gleicher Manifest wie das
Plattform-CK); der CI-Manifestator ist der einzige Schreiber.
