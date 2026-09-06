<!--
  title: Befund — Galileo-Floor-Lautheit gegen Pioneer 10 auf der Pass-Zeit-Scheiben-Ebene gemeinsamer (Station, Tag)-Zellen 1995-11-23..1997-02-28: die gemeinsamen Station-Tage sind pass-scharf getrennt, die einzige robuste gemeinsame laut-Zelle liest gegenläufig im eigenen Zeitfenster, der laut-Tag ist überwiegend ein laut-Pass
  class: befund
  date: 2026-09-06
  sha256: 1927faaa27935506c65330a2a2209aa8ca198bdb7432a11ad72c677fc41ebe3a
  status: draft
  antwortet-auf: docs/befund/befund-galileo-pioneer-stationsfloor-kreuz.md
  see-also: docs/auftrag/auftrag-subhz-drift-quiet-zone.md docs/befund/befund-galileo-floor-subtages-recurrenz.md
-->

# Befund: die Galileo-Floor-Lautheit ist auf gemeinsamen Station-Tagen pass-/zeitscheiben-scharf von Pioneer 10 getrennt; die eine robuste gemeinsame laut-Zelle liest laut nur im eigenen Galileo-Fenster

## Frage & Bindung

Der Vorgänger-Befund (Richtung K) hat auf der (Station, Tag)-Ebene gemessen:
Galileo-Floor-Laut-Tage (AGC-Klemmwert −2560, Zell-RMS je (Mode, Station,
TDB-Tag), laut ≥ 1 Hz) und Pioneer-10-Boden (negative-fuzzy-Tagesmedian)
koinzidieren im Fenster 1995-11-23..1997-02-28 auf den 70-m-Stationen DSS-14/
43/63 in **null** laut–laut-Zellen; nur 6 gemeinsame (Station, Tag)-Zellen
existieren, und die eine robuste gemeinsame Zelle mit Galileo-laut (st14
1995-11-26, m1 14,4 Hz / m3 75,0 Hz) liest Pioneer-ruhig (Tagesmedian
−0,008 Hz). Diese Richtung (L) verfeinert auf die **sub-tägliche
Pass-Zeit-Scheiben-Ebene**: Eine 70-m-Antenne verfolgt nur eine Sonde zur
Zeit — an einem gemeinsamen Station-Tag müssen Pioneer- und Galileo-Samples
in nicht-überlappenden Zeit-Scheiben liegen. Gemessen wird (a) die
Pass-/Zeit-Scheiben-Zerlegung beider Sonden je gemeinsamer Zelle, (b) ob die
Zeitfenster überlappen oder getrennt sind, (c) ob die Galileo-Lautheit auf
das eigene (Galileo-)Zeitfenster begrenzt ist statt den ganzen Station-Tag zu
füllen, und (d) ob auf Galileo-lauten Tagen die Einheit der Pass ist (laut-Tag
= laut-Pass?).

## Daten, Metrik, Konvention

Galileo-Resid `data/pds-ppi.igpp.ucla.edu/galileo_resid.bin` (GASR, 8 Felder:
[0] tdb, [1] resid_hz, [2] station, [3] mode, [4] dtype, [5] ref, [6] sampler,
[7] strength); Floor = Stärke exakt −2560; Mode 1..3; Lock |resid| > 1000 Hz
vor dem Rauschen ausgeschlossen. Floor-Tages-Zelle = (Mode, Station, TDB-Tag),
Tages-RMS um den Zellen-Mittelwert, laut ≥ 1 Hz, robust n ≥ 30. Pioneer-10-
Residuum `data/spdf.gsfc.nasa.gov/pioneer10_navio_residuum.bin` (P11R; Feld
[1] = Residuum nach der negative-fuzzy-Pipeline der Referenz-Leser:
pro-Block quadratischer Detrend mit Lücke 900 s und Block ≥ 8, pro-Station-
Median-Abzug; Feld [5] = Station); Zelle = (Station, TDB-Tag), ruhig-Band
|Median| ≤ 5 Hz. **Pass = zusammenhängende Samples mit Lücke ≤ 600 s**
(gleiche Konvention wie die Galileo-Pass-Proben); Fenster-RMS/Median um das
Fenster-Mittel. Probe `tools/measure/src/bin/galileo_pioneer_pass_slices.rs`
(`cargo check -p omegaflow-measure --bin galileo_pioneer_pass_slices`, 0/0
Warnungen, RUSTFLAGS `-D warnings`); Report
`/tmp/opencode/galileo_pioneer_pass_slices_report.txt`.

## n zuerst — Reproduktion und gemeinsame Zellen

**Reproduktion der Referenz-Zahlen (S0/S0b):** Galileo-Floor-Tages-Zellen im
Fenster je (Mode, Station) — robust/laut: m1 st14 62/32, st43 64/35, st63
68/34; m2 st14 38/17, st43 35/16, st63 39/22; m3 st14 40/25, st43 33/16,
st63 21/10 — Summe **207 robuste laute Floor-Zellen**, dedupliziert auf
(Station, Tag) **154** laut-Zellen (st14 54, st43 52, st63 48), exakt die
Zahlen der Richtung K. Pioneer-10-Floor-Zellen im Fenster: **64**
(st14 18, st43 23, st63 23), exakt die 64 Pioneer-Tage der Richtung K.

**Gemeinsame (Station, Tag)-Zellen im Fenster: n = 6** — exakt dieselben sechs
Zellen wie der Vorgänger: st14 1995-11-26, st43 1995-11-26, st43 1995-12-06,
st63 1996-12-30, st63 1997-01-12, st63 1997-01-14.

## Messung 1 — Pass-/Zeit-Scheiben der beiden Sonden auf gemeinsamen Zellen

Je gemeinsamer Zelle werden die Galileo-Floor-Pässe (Lücke ≤ 600 s, RMS um das
Fenster-Mittel) und der Pioneer-Pass mit Zeitfenster, n und RMS gemessen:

| (Station, Tag) | Pioneer-Fenster | Galileo-Floor-Pässe | Überlappung |
|---|---|---|---|
| st14 1995-11-26 | 8,59–9,33 h, n 43, med −0,008, RMS 0,139 [ruhig] | m3 16,90–18,64 h n 10846 RMS 75,0 [LOUD]; m1 18,81–19,12 h n 2216 RMS 14,4 [LOUD]; m2 19,25 h–0,71 h n 29222 RMS 0,414 [ruhig] | getrennt |
| st43 1995-11-26 | 13,97–14,31 h, n 21 [dünn] | m2 2,25–9,36 h RMS 0,022; m3 23,83–2,24 h RMS 0,023 [ruhig] | getrennt |
| st43 1995-12-06 | 17,42–17,90 h, n 30, RMS 0,043 [ruhig] | m1 20,62–21,64 h RMS 0,094 u. 2,34–7,82 h RMS 0,053; m2 22,69–2,32 h RMS 0,578; m3 21,65–22,67 h RMS 0,059 [ruhig] | getrennt |
| st63 1996-12-30 | 0,96–3,77 h, n 186, RMS 3,620 [ruhig] | m1 9,52 h, n 7 RMS 1,257 [dünn] | getrennt |
| st63 1997-01-12 | 17,29–19,52 h, n 147, RMS 0,667 [ruhig] | m1 8,74 h, n 7 RMS 1,653 [dünn] | getrennt |
| st63 1997-01-14 | 19,96–21,93 h, n 130, RMS 1,727 [ruhig] | m1 8,64 h, n 6 RMS 0,461 [dünn] | getrennt |

Alle **6/6 gemeinsamen Station-Tage tragen Pioneer- und Galileo-Floor-Samples
in nicht-überlappenden Zeit-Scheiben** (Spannen getrennt, keine Überlappung in
Sekunden). Die Ein-Antennen-Eine-Sonde-Regel ist auf jeder gemeinsamen Zelle
erfüllt. Pioneer war auf den st63-Tagen in seinem eigenen (nächtlichen bzw.
abendlichen) Fenster ruhig, während Galileo dort nur dünne Floor-Reste (n 6–7,
9 Uhr) trug — 0 geehrt für die fehlende robuste Galileo-Zeuge.

## Messung 2 — ist die Lautheit auf das Galileo-Fenster begrenzt?

Die einzige gemeinsame Zelle mit robust-lautem Galileo-Floor ist **st14
1995-11-26**. Dort ist die Tages-Zelle in drei disjunkte Galileo-Mode-Fenster
und ein getrenntes Pioneer-Fenster zerlegt:

- Pioneer 8,59–9,33 h: n 43, RMS 0,139 Hz, Median −0,008 Hz — ruhig.
- Galileo m3 16,90–18,64 h: n 10846, RMS 75,0 Hz — laut.
- Galileo m1 18,81–19,12 h: n 2216, RMS 14,4 Hz — laut.
- Galileo m2 19,25 h–0,71 h: n 29222, RMS 0,414 Hz — ruhig (selbe Station,
  selber Tag, direkt anschließendes Fenster).

Die Station ist an diesem gemeinsamen Tag **nicht über die volle Tageslänge
laut**: Das Pioneer-Fenster (anderer Zeit-Scheib) ist ruhig, und auch das
Galileo-eigene m2-Fenster direkt nach den lauten m1/m3-Fenstern ist ruhig. Die
Lautheit ist auf die Galileo-Floor-Zeitfenster (m1/m3) begrenzt und füllt den
Station-Tag nicht. Auf den übrigen fünf gemeinsamen Zellen ist Galileo-Floor
ruhig oder dünn und Pioneer ruhig — kein station-weiter Laut-Zustand, 0 geehrt
für einen Gegenbeleg.

## Messung 3 — innerhalb Galileo: ist laut-Tag = laut-Pass?

Über **alle 207 robusten lauten Floor-Tages-Zellen** des Fensters (n ≥ 30,
Tages-RMS ≥ 1 Hz) wird die Floor-Zeitreihe je Zelle in Pässe (Lücke ≤ 600 s)
zerlegt und je Pass das RMS um das Pass-Mittel gemessen:

- **139 von 207 laut-Zellen** tragen genau **einen robusten Pass** — die
  Tages-Lautheit ist dort die Lautheit dieses einen Passes.
- 20 laut-Zellen: mehrere robuste Pässe, alle laut.
- 42 laut-Zellen: mehrere robuste Pässe, davon mindestens ein ruhiger Pass am
  selben Tag (Tag laut, aber nicht alle Scheiben laut).
- 6 laut-Zellen: Floor-Samples nur in dünnen Pass-Fenstern (n < 30).

Die Tages-Auflösung ist also auf der Mehrheit der laut-Zellen eine
Pass-Auflösungsgrenze: **laut-Tag = laut-Pass** in 139/207 Fällen, und wo der
Tag mehrere Scheiben trägt, koexistiert in 42 Fällen ein ruhiger Pass. Anker
der intra-day-Struktur (S4, robust laut, je Fenster mit Uhrzeit/n/RMS):
st63 1995-12-16 m1 — Tag n 101, RMS 530 Hz — ein Pass: 16,02–16,12 h n 101
RMS 530 Hz; st43 1997-01-22 m1 — Tag n 68, RMS 440 Hz — zwei Pässe: 20,16–20,45
h n 53 RMS 423 Hz laut, 20,67–20,69 h n 15 RMS 3,2 Hz ruhig-dünn; st43 1997-01-31
m1 — Tag n 67, RMS 390 Hz — 8,04–8,06 h n 62 RMS 405 Hz laut bei 19,51 h n 5
RMS 1,1 Hz ruhig-dünn; st63 1996-09-08 — m1 17,09–17,36 h n 592 RMS 384 Hz,
m2 17,71–23,21 h n 19573 RMS 32 Hz, m3 15,94–16,94 h n 3279 RMS 36 Hz — drei
Mode-Scheiben desselben Tages, jede in einem eigenen Pass laut; st63
1997-01-22 m2 — Tag n 531, RMS 360 Hz — Pässe mit stark unterschiedlichem
Median (12,02 h n 82 med −995 Hz RMS 1,6 Hz; 10,96–11,22 h n 210 med −0,2 Hz
RMS 92 Hz): die Tages-Lautheit wird dort von einer Scheibe mit großem
Median-Versatz und von einer Scheibe mit großem Streu-RMS getragen. Die
fünf Anker sind die fünf robust-lauten (Mode, Station, Tag)-Zellen des
Fensters mit dem höchsten Tages-RMS (S4-Auswahl der Probe), keine
handgewählte Stichprobe.

## Verdikt

**Sitz = per-Pass-/per-Galileo-Kette dieser Station, nicht station-weiter
Tageszustand.** Gemessen auf der sub-täglichen Ebene: (1) alle 6 gemeinsamen
(Station, Tag)-Zellen tragen Pioneer- und Galileo-Samples in getrennten
Zeit-Scheiben; (2) auf der einzigen gemeinsamen Zelle mit robust-lautem
Galileo-Floor (st14 1995-11-26) ist der Tag nicht durchgehend laut — das
Pioneer-Fenster (8,6–9,3 h) und das Galileo-m2-Fenster (19,25 h–0,71 h) sind
ruhig, die Lautheit (m1 14,4 Hz / m3 75,0 Hz) liegt in den Galileo-Fenstern;
(3) innerhalb Galileo ist die Tages-Zelle überwiegend eine Pass-Zelle
(139/207 laut-Zellen mit genau einem robusten Pass; 42 laut-Zellen mit ruhigem
Pass am selben Tag). Die Floor-Lautheit des Fensters ist damit an den
Galileo-Pass gebunden.

**Stichprobengrenze:** Die gemeinsamen Zellen sind n = 6 von 64 Pioneer-Tagen
bzw. 304 Galileo-Floor-Tagen; nur **eine** gemeinsame Zelle ist robust-laut auf
der Galileo-Seite (st14 1995-11-26). Die Aussage „Lautheit auf Galileo-Fenster
begrenzt" ruht auf dieser einen Zelle plus der Pioneer-ruhigen Kontrolle am
selben Tag in getrenntem Zeitfenster; sie ist kein Vollbeweis gegen einen
Station-Mechanismus, der nur in den Galileo-Pass-Zeiten wirkte (Pioneer kann in
diesen Zeiten nicht messen — eine Antenne, eine Sonde). Die Pioneer-tägliche
Ruhe auf st63-Zellen mit RMS bis 3,6 Hz steht im ruhig-Band (|Median| ≤ 5 Hz),
ist aber quantitativ nicht verschwindend.

**Was `pending` bleibt:** der Galileo-Boden an den Pioneer-lauten Tagen
(Jan/Aug 1996 bodenleer, 0 geehrt), die Ursache der lauten Pässe (hier nicht
Gegenstand) und jede Aussage über Station-Zustände in den Galileo-Zeitfenstern
selbst, die eine zweite Sonde nicht bezeugen kann.

## Register-Satz

*Richtung L (Pass-Zeit-Scheiben gemeinsamer Station-Tage) ist gemessen: alle
6 gemeinsamen (Station, Tag)-Zellen 1995-11-23..1997-02-28 tragen Pioneer-10-
und Galileo-Floor-Samples in nicht-überlappenden Zeit-Scheiben (0 s
Überlappung). Auf der einzigen robusten gemeinsamen laut-Zelle (st14
1995-11-26) ist die Tages-Lautheit auf die Galileo-Floor-Fenster begrenzt
(m3 16,90–18,64 h RMS 75,0 Hz, m1 18,81–19,12 h RMS 14,4 Hz), während das
Pioneer-Fenster (8,59–9,33 h, RMS 0,139 Hz) und das Galileo-m2-Fenster
(19,25 h–0,71 h, RMS 0,414 Hz) ruhig sind — der Station-Tag ist nicht
durchgehend laut. Innerhalb Galileo sind 139 von 207 robusten lauten
Floor-Tages-Zellen Ein-Pass-Zellen, 42 laut-Zellen tragen einen ruhigen Pass
am selben Tag — laut-Tag ist überwiegend laut-Pass. Der Sitz der
Floor-Lautheit ist damit als per-Pass-/per-Galileo-Kette dieser Station
gemessen; ein station-weiter Tageszustand hat auf den gemeinsamen Zellen
keinen Beleg (n = 6 gemeinsame Zellen, eine robust-laut).*

## Status

`draft` (Entwurf für die Haupt-Session/Rat). Probe
`galileo_pioneer_pass_slices` committet (`cargo check` 0/0, RUSTFLAGS
`-D warnings`), Report `/tmp/opencode/galileo_pioneer_pass_slices_report.txt`.
Reproduktion exakt auf den Zahlen der Richtung K (154 laut-(Station, Tag)-
Zellen, 64 Pioneer-Floor-Tage, 6 gemeinsame Zellen, gleiche
Pioneer-Tageswerte); die Pass-Zerlegung (Lücke ≤ 600 s) zerlegt jede
gemeinsame Zelle in getrennte Scheiben.
