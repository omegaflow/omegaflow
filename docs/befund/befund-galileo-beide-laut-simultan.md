<!--
  title: Befund — die 2 robusten beide-laut-(Station,Station)-Zellen des Simultanitäts-Tests (Richtung B-L): beide Mode 1, 113 Tage getrennt, keine gemeinsame Ursache auf Sample-Ebene — same-sample beide-laut 0/4633 bzw. 0/1043 matched Paare (Erwartung 46.3 bzw. 10.3), Pearson r 0.037 bzw. −0.55; die Klasse heißt beide-laut-durch-zeitlich-getrennte-per-Station-Ausbrüche im Zellfenster
  class: befund
  date: 2026-09-06
  sha256: 42d8c3920457730edc6058a7943f624d8dac6a96a7f4463606669118566c2e0a
  status: draft
  see-also: docs/befund/befund-galileo-simultan-intrapass-trk225.md docs/befund/befund-galileo-dreiweg-sendempfang-floor.md docs/befund/befund-galileo-receiver-floor-ursache.md
-->
# Befund: die 2 beide-laut-(Station,Station)-Zellen des Simultanitäts-Tests — zeitlich getrennte per-Station-Ausbrüche im geteilten Zellfenster, keine gemeinsame Sample-Ebene (Richtung B-L)

## Frage & Bindung

Der Simultanitäts-Test des Vorgänger-Blattes (Deduktion-27) fand über die
Boden-Ära 55 simultane (Station,Station)-Zellen, 50 robust (alle Mode 1: 16
eine-laut, **2 beide-laut**, 32 beide-ruhig). Die 2 robusten beide-laut-Zellen
sind dort als Kandidaten einer **gemeinsamen Ursache** benannt (ein
Sende-/Signal-/Interferenz-Event, das beide Empfänger gleichzeitig trifft).
Dieser Lauf (Richtung B-L) misst an genau diesen 2 Zellen, ob die lauten
Samples beider Stationen im überlappenden Fenster auf **denselben Samples**
liegen (gemeinsame Ursache) oder auf verschiedenen (getrennte per-Station-
Ausbrüche in derselben Epoche).

Bindungen wie die Vorlagen: Boden-Sample = Stärke exakt −2560
(AGC-Klemmwert), Lock (|resid| ≤ 1000 Hz); Zelle = tdb-Überlapp zweier Läufe
verschiedener Stationen im selben Mode; laut = Zell-RMS ≥ 1 Hz um den
Zellen-Mittelwert; robuste Zelle = n ≥ 30 je Station. Neu hier (gemessen):
beide Stationen samplen mit 1.000-s-Cadence auf **demselben Gitter**
(Median |tA−tB| der matched Paare = 0.0000 s), so dass ein gemeinsames
Empfangs-Event auf denselben Samples erscheinen müsste (Lichtzeit-Differenz
zwischen den Komplexen ≪ 1 s, Lag-Scan ±5 s). Laut-Sample je Station =
|resid − Zellmittel| über dem q-Quantil der eigenen |resid − Zellmittel|-
Verteilung (q = 0.90 und 0.99, daten-eigen, nicht gesetzt); same-sample-
beide-laut = matched Paar, in dem beide Stationen gleichzeitig laut sind;
Unabhängigkeits-Erwartung = Paare × (laut-Anteil A) × (laut-Anteil B) über die
Fenster-Marginalen. Kreuzkorrelation auf dem gemeinsamen 1-s-Gitter und auf
den matched Paaren (Toleranz 0.75 s).

Daten: `data/pds-ppi.igpp.ucla.edu/galileo_resid.bin` (GASR, 8 × f64: tdb,
resid_hz, station, mode, data_type, doppler_ref, sampler, strength). Probe:
`tools/measure/src/bin/galileo_beide_laut_simultan.rs` (`cargo check -p
omegaflow-measure --bin galileo_beide_laut_simultan`, RUSTFLAGS `-D warnings`,
0/0). Report: `/tmp/opencode/galileo_beide_laut_simultan.txt`. Die Zell-Liste
deckt sich mit dem Vorgänger-Report (n und RMS deckungsgleich).

## n zuerst (0 geehrt)

- 4 647 387 Floor-Trio-Boden-Samples der Ära; 55 simultane Zellen, 50 robust,
  davon **2 robuste beide-laut-Zellen** (Mode 1).
- Beide-laut über alle n (inkl. sub-robust): **4** — die 2 robusten (Mode 1)
  plus 2 sub-robuste: st14×st43 1997-01-18 (Mode 1, n 27/92) und st14×st43
  1996-12-23 (Mode 2, n 7/7). Mode 3/4: beide-laut n = 0.
- Stichprobe der robusten Klasse: n = 2 Zellen. Die Messung entscheidet über
  Sample-Koinzidenz innerhalb dieser 2 Zellen; sie überverkauft nicht.

## Messung 1 — die 2 robusten beide-laut-Zellen (gemessen)

| Zelle | Mode | Stationen | Tag | tdb-Fenster (UTC) | nA | nB | RMS A | RMS B |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | st14 × st43 | 1996-11-06 | 02:36:03 … 03:58:27 | 4737 | 4841 | 2.4611 Hz | 69.9899 Hz |
| 2 | 1 | st14 × st63 | 1997-02-27 | 13:51:01 … 14:12:34 | 1052 | 1201 | 15.3005 Hz | 97.5360 Hz |

Die beiden Zellen liegen **nicht am selben Tag** (Abstand ≈ 113 Tage) und ihre
Fenster überlappen zeitlich **nicht**. Mode 1 = One-Way-Doppler (nur
Downlink-Empfang, kein Boden-Uplink im Signalweg): eine gemeinsame Ursache
müsste vom Raumfahrzeug-Signal ausgehen und träfe beide Stationen auf
denselben Samples.

## Messung 2 — gemeinsame oder getrennte Ursache (gemessen)

**Zelle 1 (1996-11-06, st14 × st43):** st14 ist über das Fenster fast ruhig
(|resid|-Median 0.242 Hz, p90 0.684 Hz) und trägt seine Lautheit als
front-konzentrierte Ausreißer (Quartil 0–1: −93 Hz um 02:38:25, +55 Hz um
03:14:02; >10 Hz nur 2 Samples). st43 ist vorn ruhig und hinten extrem laut
(Quartil 2–3, |resid| bis 172 Hz; >100 Hz 1303 Samples). Die laut-Samples
liegen zeitlich **getrennt**: laut-A in Quartil [473, 1, 0, 0], laut-B in
[0, 0, 165, 319]. same-sample beide-laut **0 von 4633** matched Paaren
(Unabhängigkeits-Erwartung 46.3 bei q0.90, 0.5 bei q0.99). Pearson r auf dem
Gitter lag0 = 0.037, First-Difference r = 0.0004 — die resid-Reihen sind
sample-für-sample unkorreliert. Die Zelle ist beide-laut nur auf der
Zell-RMS-Ebene (jede Station hat ihren eigenen lauten Sub-Zeitabschnitt im
geteilten Fenster), nicht auf denselben Samples.

**Zelle 2 (1997-02-27, st14 × st63):** st14 ist über 14:07 hinaus fast ruhig
(Median 0.021 Hz) und bricht um 14:08:16–23 aus (−387 … −103 Hz, 6 Samples
>100 Hz); st63 bricht um 14:10:53–14:11:02 aus (bis +945 Hz; >100 Hz 89
Samples). Die lauten Epochen liegen ≈ 2.5 min auseinander (st14 14:08, st63
14:11); laut-A in Quartil [9, 13, 73, 8], laut-B vollständig in Quartil 3
[0, 0, 0, 120]. same-sample beide-laut **0 von 1043** matched Paaren
(Erwartung 10.3 bei q0.90). Pearson r lag0 = −0.55 (First-Difference −0.065):
die Reihen sind nicht positiv korreliert — ein gemeinsames additives Signal
fehlt; der negative Rohwert trägt die gegenläufige Epochenlage, nicht eine
Sample-Kopplung.

**Sub-robuste beide-laut-Zellen (Kontext, gemessen):** st14×st43 1997-01-18
(n 27/92) hat **0 matched Paare** — die Stationen teilen kein einziges
gemeinsames Sample-Gitter im Zellfenster (Lautheit aus zeitlich
unverbundenen, dünnen Ausbrüchen). st14×st43 1996-12-23 (Mode 2, 6 s, n 7/7)
ist die **einzige sample-koinzidente beide-laut-Zelle der Ära** (alle 7 Samples
beide Stationen laut, Pearson r = −0.9998, First-Difference −0.9988) — aber
**gegenphasig** (st14 +185…+81 Hz, st43 −461…−359 Hz) und mit n 7 datendünn;
eine gemeinsame additive Signalfärbung trägt sie nicht.

## Messung 3 — Kontext

Beide robusten Zellen liegen in Mode 1 (One-Way). Am Tag der Zelle 1
(1996-11-06) trägt st43 zusätzlich weitere simultane st14×st43-Fenster des
Tages (alle ruhig bis auf das eine laute); Zelle 2 (1997-02-27) ist der einzige
simultane st14×st63-Überlapp ihres Tages. Ein dokumentiertes DSN-/Pass-Ereignis
wird hier nicht behauptet (nur die Daten tragen die Aussage). Die 2 robusten
Zellen sind keine gemeinsame Epoche: 113 Tage auseinander, keine weiteren
beide-laut-Zellen dazwischen (die anderen beiden beide-laut-Phänomene der Ära
liegen bei 1996-12-23 und 1997-01-18, n 7 bzw. 27/92). „Gemeinsame Epoche mit
anderen beide-laut-Phänomenen" ist damit n = 0 belegt; die 2 Zellen sind in der
Ära isolierte, singuläre beide-laut-Fälle.

## Verdikt

**Gemeinsame Ursache auf Sample-Ebene: widerlegt für die 2 robusten
beide-laut-Zellen.** In keiner der beiden Zellen ist ein Sample gleichzeitig an
beiden Stationen laut (0 von 4633 bzw. 0 von 1043 matched Paaren; Erwartung
unter Unabhängigkeit 46.3 bzw. 10.3), die resid-Reihen sind nicht positiv
korreliert (r 0.037 bzw. −0.55, First-Difference ≈ 0). Beide Zellen sind laut,
weil **jede Station ihren eigenen, zeitlich getrennten Ausbruch im geteilten
Zellfenster** trägt (Zelle 1: st14 vorn / st43 hinten; Zelle 2: st14 14:08 /
st63 14:11). Als eigene Klasse benannt: **„beide-laut durch zeitlich getrennte
per-Station-Ausbrüche im Zellfenster"** — der beide-laut-Befund auf
Zell-RMS-Ebene ist kein simultanes Signal-Event.

**Stichprobengrenze:** die Klasse hat n = 2 robuste Zellen (plus 2
sub-robuste, davon eine 6-s-sample-koinzidente gegenphasige mit n 7). Der
Befund „keine gemeinsame Ursache" gilt für diese 2 Zellen; er verallgemeinert
nicht auf künftige beide-laut-Zellen, die es in der vorliegenden Ära nicht gibt
(0 weitere robuste). Eine gemeinsame Ursache mit Sub-Sample-Lichtzeit-Versatz
ist durch die 1-s-Cadence und den ±5-s-Lag-Scan abgedeckt; eine
Sample-koinzidente gemeinsame Ursache bleibt für die sub-robuste Mode-2-Zelle
1996-12-23 offen (dort aber gegenphasig, n 7 — datendünn, nicht überverkauft).

## Register-Satz

*Die 2 robusten beide-laut-(Station,Station)-Zellen des Simultanitäts-Tests
(Richtung B-L) sind auf Sample-Ebene gemessen: st14×st43 1996-11-06
02:36–03:58 UTC (n 4737/4841, RMS 2.46/69.99 Hz) und st14×st63 1997-02-27
13:51–14:12 UTC (n 1052/1201, RMS 15.30/97.54 Hz), beide Mode 1, 113 Tage
getrennt; beide Stationen teilen das 1-s-Sample-Gitter (Median |tA−tB| 0.0000 s)
und dennoch ist kein Sample gleichzeitig an beiden Stationen laut — same-sample
beide-laut 0 von 4633 bzw. 0 von 1043 matched Paaren (Unabhängigkeits-Erwartung
46.3 bzw. 10.3), Pearson r 0.037 bzw. −0.55 (First-Difference ≈ 0) — die
Lautheit der Zellen entsteht aus zeitlich getrennten per-Station-Ausbrüchen im
geteilten Fenster (Zelle 1 st14 vorn/st43 hinten, Zelle 2 st14 14:08/st63 14:11),
keine gemeinsame Ursache auf Sample-Ebene; die Klasse heißt beide-laut-durch-
zeitlich-getrennte-per-Station-Ausbrüche. Die einzige sample-koinzidente
beide-laut-Zelle der Ära (st14×st43 1996-12-23, Mode 2, 6 s, n 7/7) ist
gegenphasig (r −0.9998) und datendünn; 1997-01-18 (n 27/92) hat 0 matched
Paare. Probe galileo_beide_laut_simultan committet (cargo check -D warnings
0/0), Report /tmp/opencode/galileo_beide_laut_simultan.txt.*

## Status

`draft` (Entwurf für die Haupt-Session/Rat; die Register-Zeile in
`docs/handover/handover-2026-09-09-mechanische-reste.md` ergänzt die Haupt-Session, diese Session fasst die fremde
TODO-Datei nicht an). Probe committet; cargo check 0 Warnungen.
