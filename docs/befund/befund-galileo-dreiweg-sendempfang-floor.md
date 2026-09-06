<!--
  title: Befund — Galileo-Dreiweg (Mode 3) Sende-/Empfangs-Trennung gegen die Floor-Gipfel (Richtung H, Deduktion-27-Split): Tor-1-Verdikt gemessen — der rohe TRK-2-25-Dreiweg-Record trägt die Sende-Station weder pro Sample noch pro Pass (83 614 Dreiweg-Samples aus 4 Floor-Ära-TDF: das einzige Stations-Wort ist item 10 = die aufzeichnende Station, alle übrigen Identitäts-Items konstant, XMTR_POWER/XMTR_FREQ/XMTR_POWER_IND unbesetzt, kein Feld trägt einen zweiten Stations-Wert; auch XMTR_ON0 benennt den Sender nicht, weil er innerhalb und zwischen Dreiweg-Läufen mischt) — der (empfangend×sendend)-Split ist aus dem TRK-2-25-Korpus per Sample nicht ziehbar; die Mode-3-Floor-Zellen selbst sind gemessen (40/33/21 robuste Tage, 25/16/10 laut) und flippen bei konstanter aufgezeichneter Station laut/ruhig, was ein Sender-Urteil ohne Pass-Plan offen lässt
  class: befund
  date: 2026-09-06
  sha256: 5e343afac7361d9c7de1516cbaa1ec98aa2f1ba519fa3cf593958a4488ee4638
  status: draft
  see-also: docs/befund/befund-galileo-receiver-je-pass-floor.md docs/befund/befund-galileo-receiver-floor-ursache.md docs/befund/befund-galileo-mode3-st43-run.md docs/befund/befund-galileo-floor-stufen-te.md docs/paper/twenty-second-band-ground-chain.md docs/paper/ground-sources-20s-band.md docs/auftrag/auftrag-bande-split.md
-->
# Befund: Galileo-Dreiweg Sende-/Empfangs-Trennung — die sendende Station ist im rohen TRK-2-25-Record nicht vorhanden (Tor-1-Verdikt), der Deduktion-27-Split bleibt auf Galileo strukturell unausführbar

## Frage & Bindung

Pioneer-Deduktion 27 (der two-/three-way-Split der 20-s-Bande) fragt, ob die
station-gebundene Laut-Struktur im **Uplink** (sendende Station) oder im
**Downlink** (empfangende Station) sitzt: im dreiweg-Fall sendet Station A und
empfängt Station B, und die gemessene Linie wandert mit dem Empfänger (→
Empfangskette) oder mit dem Sender (→ Sendekette). Auf Pioneer war dieser Split
lokal nie messbar (der 1-s-Satz ist three-way-only, Mode 2 leer — 0 honored);
die dreiweg-Zellen der Mode-3-Serie des Galileo-Floor-Fadens sind die erste
Gelegenheit, den Split an Daten zu stellen. Dieser Lauf (Richtung H,
Deduktion-27-Split auf Galileo) misst zuerst das strukturelle Tor: **trägt der
rohe TRK-2-25-Dreiweg-Record die sendende Station überhaupt** (pro Sample, pro
Pass)?

Gebunden wie die Vorlagen: die Floor-Zelle = (Mode, Tag, Station)-RMS der
Boden-Residuen (Stärke exakt −2560, AGC-Klemmwert) um den Zellen-Mittelwert;
Lock (|resid| > 1000 Hz) vor dem Rauschen getrennt; laut = Zell-RMS ≥ 1 Hz;
dreiweg = ground_mode 3 (TRK-2-25 item 13; item 13 des SIS: 1 = one-way, 2 =
two-way, 3 = three-way, 4 = three-way coherent). Die Feldsemantik stammt aus der
vollständigen Feldtabelle TRK-2-25-3 (`docs/reference/trk-2-25-atdf.txt`), die
den 288-Byte-Tracking-Record lückenlos belegt: **item 10 STATION ist das einzige
Stations-Wort des gesamten Records**; items 36/72/75/115 (Reserved) und 117 (not
used) sind die einzigen unbelegten Bänder. Die Probe
`tools/measure/src/bin/galileo_threeway_txrx_split.rs` (neu, einzige
Repo-Änderung außer diesem Blatt; `cargo check -p omegaflow-measure --bin
galileo_threeway_txrx_split`, RUSTFLAGS `-D warnings`, 0/0) holt eine begrenzte
rohe TDF-Stichprobe der Floor-Ära, parst die Roh-Records, zählt die
Identitäts-Items je Dreiweg-Gruppe und scannt jede Dreiweg-Gruppe auf einen
zweiten Stations-Wert; Report
`/tmp/opencode/galileo_threeway_txrx_split_report.txt`.

## n zuerst (0 geehrt) — die geholte Roh-Stichprobe

Vier gezielte TDF-Dateien (die Dateinamen kodieren die Pass-Spanne als
Jahresziffer + Start-/End-DOY; Header-Datum der Datei verifiziert):
GO-SUN-RSS-1-TDF-V1.0/TDF/5327328A.TDF (Header 1995 DOY 328),
GO-SUN-RSS-1-TDF-V1.0/TDF/5337339A.TDF (1995 DOY 339),
GO-SUN-RSS-1-TDF-V1.0/TDF/5340341A.TDF (1995 DOY 341),
GO-JG-RSS-1-TDF-V1.0/TDF/6177179A.TDF (1996 DOY 179). Das Fenster deckt die
laut/ruhig-Anker 1995-11-24 (st14 laut 25,8 Hz), den Mode-3-st43-Lauf
1995-11-23..12-07 und die Oppositions-Anker 1996-06-26/27 ab.

| TDF | doppler Records | Dreiweg-Records | Abdeckung (Tage) |
|---|---|---|---|
| 5327328A | 133 003 | 19 608 | 1995-11-23/24 |
| 5337339A | 134 037 | 41 914 | 1995-12-04/05 |
| 5340341A | 135 280 | 14 456 | 1995-12-07 |
| 6177179A | 134 806 | 7 636 | 1996-06-26/27 |
| **Summe** | **537 126** | **83 614** | 12 Zellen (Tag, Station) |

Die 83 614 Dreiweg-Doppler-Samples (station 14/43/63, ausschließlich; Station
42 trägt nur one-way, mode 4 im Fenster n = 0) bilden **12 (Tag, Station)-Zellen,
je eine zusammenhängende Dreiweg-Zelle pro Tag und Station** (Gap ≤ 600 s
zerlegt nicht). Darunter tragen **62 779 Samples (75,1 %) den AGC-Klemmwert
−2560** — die Dreiweg-Records der Floor-Ära sind überwiegend selbst der Boden;
die Mode-3-Floor-Zellen der Faden-Serie liegen also auf genau diesen Samples.

## Tor-1-Verdikt: Sende-Station-Reachability

### 1. Per Sample: kein zweites Stations-Wort (gemessen)

Über **alle 83 614 Dreiweg-Samples** der Stichprobe ist der Zensus der
Identitäts-Items je (Tag, Station)-Gruppe konstant und stations-los:
NET_ID = 2, DOWNLINK_BAND = 1, SOURCE_DESIG = 1, UPLINK_BAND = 1,
DOPPLER_CHANNEL = 1, FREQ_STD = 0, DOPPLER_RCVR_REF = 1, RCVR_NUMBER = 0,
AMP_NUMBER = 0, AMP_TYPE = 0, XMTR_POWER_IND = 0; **XMTR_POWER und
XMTR_FREQ sind in jeder der 12 Gruppen durchgehend 0** (nz 0/5700 … 0/21 579; die
Wörter sind auch in den zweiweg-Records unbesetzt); der
Stations-Wort-Scan über alle Identitäts-Items findet in keiner Gruppe einen Wert
11–99 ungleich der aufgezeichneten Station (`station-like second value: none` in
allen 12 Zellen). Der einzige Stations-Wert des Records ist item 10 = die
Station, die den Datensatz erzeugt (Empfang). Kein Feld unterscheidet im
Dreiweg einen Sender vom Empfänger; die einzigen belegbaren Sende-Wörter des
Records sind Absence-Indikatoren des Aufzeichnungsorts, keine Sender-Identität.

### 2. Auch XMTR_ON0 benennt den Sender nicht (gemessen)

Das einzige lebende Sender-Wort (item 26, Transmitter/Exciter On/Off) variiert
**innerhalb und zwischen** Dreiweg-Läufen und ist damit kein Sender-Namensgeber:
st43 1995-11-24 (n 4724) exciter aus in 4724/4724 (reiner Empfang), st14
1996-06-26 (n 5147) aus in 5147/5147, st63 1996-06-27 (n 2489) aus in
2489/2489 — aber st43 1995-11-23 (n 5700) exciter ein in 5695/5700, st43
1995-12-04 (n 6385) ein in 6374/6385, st14 1995-12-05 (n 7355) ein in 5447/7355,
st63 1995-11-23 (n 4363) ein in 3461/4363 und aus in 902/4363 im selben Lauf.
Der Exziter-Zustand mischt in beiden Modi (auch Zweiweg-Blöcke tragen
aus-Anteile, z. B. st14 1995-11-24 zweiweg 19:01–21:14 n 7434, exciter aus
5678/7434) — das physikalische Sende-Bild ist aus dem Record nicht eindeutig
ablesbar, und die dreiweg-Zuordnung eines Samples ist keine Sender-Zuordnung.

### 3. Per Pass: kein Sender im Record, im Header oder im Dateinamen (gemessen)

Der Datei-Kopf (File-Identification-/Transponder-Record) trägt nur Jahr/Tag/
Stunde, SC und Xponder-Frequenz, keinen Stations-Bezug; der Dateiname kodiert
die Pass-Spanne, keine Station. Eine zweite Quelle pro Pass existiert im
Bestand nicht: derselbe Pass erscheint in zwei Volumina nur als byte-gleiche
Datei-Kopie (gemessen an identischen Content-Längen, z. B. 5341342A.TDF in
GO-SUN und GO-J je 39 134 592 Bytes) — keine zweite Perspektive. Was der Record
pro Pass hergibt, ist die Zeitlage der Blöcke: die 12 Dreiweg-Zellen liegen als
einzelne Blöcke (n 2489..21 579, z. B. st43 1995-12-05 18:25–00:30, st14
1995-12-05 15:55–18:17) neben zweiweg-Blöcken derselben oder anderer Stationen.
Eine Sender-Zuordnung über Zeitüberlappung (dreiweg-Blöcke überlappen
zweiweg-Blöcke anderer Stationen, z. B. dreiweg st43 1995-11-24 überlappt
zweiweg st14; dreiweg st14 1995-12-05 überlappt zweiweg st63; dreiweg st14
1996-06-26 überlappt zweiweg st43) ist eine **notwendige-Bedingungs-Inferenz,
keine gemessene Identität** — sie ist im Report als solche benannt
(`candidate uplink, inference`), nicht als Wert geführt.

**Tor-1-Verdikt: die Sende-Station steht im rohen TRK-2-25-Record weder pro
Sample noch pro Pass.** Per Sample ist sie auf keiner Bit-Position des Records
vorhanden (ein Stations-Wort, die übrigen Bänder konstant oder unbesetzt,
gemessen an n = 83 614 Dreiweg-Samples); per Pass ist sie weder aus Header noch
Dateiname noch Meta ableitbar (die Zeitüberlappungs-Lesart bleibt Inferenz).
Der Deduktion-27-(empfangend×sendend)-Split ist damit aus dem TRK-2-25-Korpus
per Sample **nicht ziehbar** — ein strukturelles, gemessenes Verdikt, kein
Ersatz.

## Die Mode-3-Floor-Zellen, die Tor 2 hätte zerlegen sollen (gemessen)

Der ganze Floor der Ära (aus `data/galileo_resid.bin`, Konvention der Vorlagen):
**robuste Dreiweg-Floor-Zellen (n ≥ 30): st14 40 Tage (25 laut), st43 33 Tage
(16 laut), st63 21 Tage (10 laut)**, Summe 94 robuste Tage / 51 laute
(Referenz-Serie `floor-stufen-te`: 39/33/21 — die 1-Tages-Differenz an st14 ist
ein n-armer Tag an der 1997er-Kante, benannt, nicht geglättet). Die
laut/ruhig-Flips laufen **innerhalb einer konstanten aufgezeichneten Station**
ab (st14 dreiweg: 1995-12-04 ruhig 0,044 Hz → 1995-12-05 **52,9 Hz laut** →
1995-12-06 ruhig 0,058 Hz; st43 dreiweg: 1995-11-23 ruhig 0,027 Hz → … →
1995-12-04 10,5 Hz laut → 1995-12-05 9,5 Hz laut → 1995-12-06 ruhig; st63
dreiweg: 1995-11-23 34,7 Hz laut → 1995-11-25 ruhig 0,023 Hz → 1995-11-27
186,5 Hz laut). Anker-Kollokation reproduziert (n zuerst, Hz): 1995-11-24 st14
M1 25,8492 (n 7463) und M2 31,7698 (n 6487) laut gegen st43 M1 0,0312 (n 39991)
und st63 M1 0,0912 (n 5847) ruhig; 1996-06-26 st63 M1 20,6398 (n 24201) laut
gegen st14 M1 0,0211 (n 19083) und st43 M1 0,0214 (n 25792) ruhig — die Zell-
Metrik ist identisch zur Referenz. Auch die Dreiweg-Zellen selbst flippen bei
konstanter aufgezeichneter Station laut/ruhig, und da das Record keine zweite
Stations-Achse trägt, ist **nicht einmal messbar, ob ein Sender-Wechsel die
Flips begleitet** — das ist der Punkt, an dem der Split ohne Pass-Plan endet.

## Verdikt

**Deduktion 27 ist auf Galileo strukturell nicht ausführbar — gemessen, nicht
ersetzt.** Der rohe TRK-2-25-Dreiweg-Record trägt die Sende-Station nicht: über
83 614 Dreiweg-Samples aus vier Floor-Ära-TDF (1995-11-23..1996-06-27) ist item
10 das einzige Stations-Wort (die aufzeichnende/Empfangs-Station), alle übrigen
Identitäts-Items sind konstant oder unbesetzt (XMTR_POWER/XMTR_FREQ/
XMTR_POWER_IND überall 0), kein Feld hält einen zweiten Stations-Wert, und
XMTR_ON0 benennt den Sender nicht (mischt innerhalb und zwischen Dreiweg-Läufen;
auch Zweiweg-Blöcke tragen aus-Anteile). Per Pass trägt weder der Datei-Kopf
noch der Dateiname eine Station; eine zweite Quelle pro Pass existiert nicht
(Volumen-Duplikate sind byte-gleich). Der (empfangend×sendend)-Split der
laut/ruhig-Floor-Zellen ist damit aus dem TRK-2-25-Korpus **nicht ziehbar**.
Die Mode-3-Floor-Zellen selbst sind gemessen und reproduzieren die Faden-Serie:
94 robuste Dreiweg-Floor-Tage (51 laut) über die Ära, die laut/ruhig-Flips
laufen bei konstanter aufgezeichneter Station — was ein Sender-Urteil über den
Uplink weder bestätigt noch verwirft. Die Frage „Uplink oder Downlink" bleibt
auf diesen Daten `pending`.

**Was `pending` bleibt (kein Ersatz, keine Schätzung):** die Sender-Identität
der dreiweg-Zellen. Sie bräuchte den **DSN-Tracking-/Pass-Plan** (welche Station
uplinkte, je Pass/Zeitfenster), ein **ODF/TRK-2-18-Metadatum** mit
Sender-Angabe, falls es den Bestand trägt, oder eine **externe DSN-Schedule**.
Die Zeitüberlappungs-Lesart (dreiweg-Empfang überlappt zweiweg-Betrieb einer
anderen Station) ist im Report benannt, aber als Inferenz geführt — sie wird
nicht zum Verdikt erhoben.

## Grenzen

- Die Roh-Stichprobe umfasst 4 TDF (83 614 Dreiweg-Samples, 12 Zellen) der
  Anker-/Lauf-Tage; die Item-Zensus-Befunde sind über diese Stichprobe gemessen.
  Die Feldsemantik stammt aus der lückenlosen TRK-2-25-3-Feldtabelle (item 10 =
  einziges Stations-Wort); der empirische Scan bestätigt sie, ersetzt sie nicht.
- Die ganze-Ära-Dreiweg-Floor-Serie (94 robuste Tage) reproduziert die
  Referenz-Serie bis auf einen n-armen Tag an st14 (40 vs 39); die Anker-Zell-
  RMS reproduzieren die Referenz exakt (4 Dezimalen).
- Die Zeitüberlappungs-Kandidaten sind Inferenz (notwendige Bedingung), keine
  gemessene Sender-Identität — der Pass-Plan entscheidet, nicht der Record.
- Die XMTR-Wörter (98/116/96) sind im gesamten Bestand unbesetzt (0), auch in
  zweiweg-Records; ihr Fehlen ist gemessen, nicht als Geräte-Abwesenheit
  gedeutet.

## Register-Satz

*Die Sende-Station der Galileo-Dreiweg-Zellen ist im rohen TRK-2-25-Record
weder pro Sample noch pro Pass vorhanden (Richtung H, Deduktion-27-Split): über
83 614 Dreiweg-Samples aus 4 Floor-Ära-TDF (5327328A/5337339A/5340341A GO-SUN,
6177179A GO-JG; 1995-11-23..1996-06-27, 12 Tag-Stations-Zellen, 62 779 davon
AGC-Boden) ist item 10 das einzige Stations-Wort (aufzeichnende Station), die
übrigen Identitäts-Items sind konstant (NET_ID 2, DOPPLER_CHANNEL 1, RCVR/AMP
0 …), XMTR_POWER/XMTR_FREQ/XMTR_POWER_IND in jedem Dreiweg-Sample 0, kein Feld
trägt einen zweiten Stations-Wert, und XMTR_ON0 mischt innerhalb und zwischen
Dreiweg-Läufen (benennt den Sender nicht; auch zweiweg-Blöcke tragen
aus-Anteile) — der (empfangend×sendend)-Split ist aus dem TRK-2-25-Korpus per
Sample nicht ziehbar; pro Pass trägt weder Header noch Dateiname eine Station
(Volumen-Duplikate byte-gleich, keine zweite Quelle), die Zeitüberlappungs-Lesart
ist Inferenz. Die Mode-3-Floor-Zellen selbst sind gemessen: 94 robuste
Dreiweg-Floor-Tage (51 laut; st14 40/25, st43 33/16, st63 21/10), laut/ruhig-
Flips bei konstanter aufgezeichneter Station (z. B. st14 dreiweg 52,9 Hz am
1995-12-05 zwischen ruhigen Nachbartagen) — die Uplink/Downlink-Frage bleibt
pending und bräuchte den DSN-Tracking-/Pass-Plan (oder ODF/TRK-2-18-Metadatum);
die Sende-Station wird nicht ersetzt und nicht geraten.*

## Status

`draft` (Entwurf für die Haupt-Session/Rat; TODO-Registerzeile ergänzt die
Haupt-Session). Probe `tools/measure/src/bin/galileo_threeway_txrx_split.rs`
committet (`cargo check -p omegaflow-measure --bin galileo_threeway_txrx_split`,
RUSTFLAGS `-D warnings`, 0/0), Report
`/tmp/opencode/galileo_threeway_txrx_split_report.txt`. `src/archivar/atdf.rs`
wurde nicht erweitert (alle geprüften Items stehen bereits im Katalog; der
Befund ist rein empirisch gegen den Roh-Record geführt).
