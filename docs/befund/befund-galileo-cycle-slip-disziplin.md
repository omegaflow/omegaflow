<!--
  title: Befund — Cycle-Slip-Disziplin der lauten Galileo-Floor-Pässe (Richtung CS): die laut-Pass-Transienten sitzen an Lock-/Cycle-Grenzen — über die Roh-TDF-Caches der bedeckten 5 Pässe tragen 12/12 Transient-Ereignisse im engen ±2-s-Fenster einen RCVR_LOCK0-/DOPPLER_GOOD0-Flag oder SLIPPED_CYCLE > 0 (Empfangs-Cycle-Slip/Lock-Verlust; M2 st14 19:15:50 = Counter-Slip mit nominalen Flags), die Send-Predict-/Rampen-Klasse ist n = 0; die resid-Lock-Marker (>1000 Hz) liegen eingebettet in die Transient-Region oder am Laufende in den Lock-Verlust; 2 cache-freie laute Pässe resid-Alone Lock-adjacent (Flag pending)
  class: befund
  date: 2026-09-06
  sha256: a822d89a60cdfe53953e0338c78209466067e4e9ea9c3712eaa7cef7b4a043cd
  status: draft
  see-also: docs/befund/befund-galileo-simultan-intrapass-trk225.md docs/befund/befund-galileo-floor-stufen-te.md docs/befund/befund-galileo-ops-aera-floor.md
-->
# Befund: Cycle-Slip-Disziplin der lauten Galileo-Floor-Pässe — Transienten an Lock-/Cycle-Grenzen gegen Sende-Predict-/Rampen-Sprünge (Richtung CS)

## Frage & Bindung

Der Ded-27-Intra-Pass-Befund lässt die lauten Anker-Pässe als
end-/front-konzentrierte Transienten nahe der Lock-Grenze (|resid| bis
~990 Hz bei 1000-Hz-Schwelle) stehen und benennt als `pending`: sind diese
Transienten Cycle-Slips der Empfangsstation oder Predict-/Rampen-Sprünge der
Sendeseite. Dieser Lauf (Richtung CS) misst, ob die laut-Pass-Transienten an
Lock-/Cycle-Grenzen sitzen — über die Feld-Flags des TRK-2-25-Records
(RCVR_LOCK0 item 25, DOPPLER_GOOD0 item 17, DOPPLER_TOL0 item 18,
SLIPPED_CYCLE item 76) in den Roh-TDF-Caches der betroffenen Tage und über
die resid-lock-cut-Marker (|resid| > 1000 Hz), die die Floor-Pipeline als
Lock-Übergang ausschließt.

Bindungen wie die Vorlagen: Boden-Sample = Stärke exakt −2560 (AGC-Klemmwert)
mit |resid| ≤ 1000 Hz; Lauf = Sample-Lauf mit tdb-Lücke ≤ 600 s; laut = Lauf-RMS
≥ 1 Hz um den Lauf-Mittelwert (n ≥ 30); Transient-Sample = |resid| ≥ 100 Hz im
Lauf; Transient-Ereignis = zusammenhängende Transient-Samples (Lücke ≤ 60 s);
lock-adjacent = Ereignis liegt ≤ 60 s an einem |resid| > 1000-Hz-Sample oder an
einer Lauf-Kante. Flag-Semantik aus `docs/reference/trk-2-25-atdf.txt`:
RCVR_LOCK0 0 = In Lock / 1 = Out of Lock; DOPPLER_GOOD0 0 = Good / 1 = Bad;
DOPPLER_TOL0 0 = In Tolerance / 1 = Out; SLIPPED_CYCLE = gezählte Slipped
Cycles (Pioneer-PNAV-Deduktion 0, `src/archivar/atdf.rs` trägt item 76 als
`Tracking.slipped_cycle`, n_slipped je Datei im Reduce-Pfad).

Daten: `data/pds-ppi.igpp.ucla.edu/galileo_resid.bin` (GASR, 8 × f64: tdb,
resid_hz, station, mode, dtype, doppler_ref, sampler, strength) und die vier
Roh-TDF-Caches `galileo_tdf_cache_5327328A.TDF` (1995-11-23/24),
`5337339A.TDF` (1995-12-04/05), `5340341A.TDF` (1995-12-07),
`6177179A.TDF` (1996-06-26/27). Neue Probe:
`tools/measure/src/bin/galileo_cycle_slip_discipline.rs` (`cargo check -p
omegaflow-measure --bin galileo_cycle_slip_discipline`, RUSTFLAGS
`-D warnings`, 0/0). Report `/tmp/opencode/galileo_cycle_slip_discipline_report.txt`.

## n zuerst (0 geehrt)

- 7 laute Anker-Pässe, Tag-Zellen der Referenz exakt reproduziert (n/RMS
  deckungsgleich, s. Tabelle): je ein lauter Lauf.
- Flag-Abdeckung über die vier Roh-Caches: **5 von 7 laut-Pässen** tragen
  einen Roh-Cache ihres Tages (M1/M2 st14 1995-11-24, M1 st63 1996-06-26,
  M3 st43 1995-12-04, M3 st14 1995-12-05); **2 laut-Pässe ohne Roh-Cache**
  (M3 st63 1995-11-27, M1 st43 1996-11-04) — dort gilt das resid-Alone-Maß
  (0 geehrt für die Flag-Achse).
- Transient-Ereignisse ≥ 100 Hz: **n 17 Ereignisse** über die 7 Pässe
  (7+2+1+1+1+4+1), davon **12 auf flag-bedeckten Pässen**, 5 auf
  cache-freien.
- Empfangs-Signatur an den Ereignissen (Flag oder Slipped-Cycle im engen
  Fenster): **12 von 12 flag-bedeckten Ereignissen**; Send-Predict-/Rampe-
  Signatur (coherent, flag-frei, kein Slipped, im gelockten Lauf): **n = 0**.

## Messung 1 — Lock-/Cycle-Grenz-Marker relativ zu den Transienten

Die als |resid| > 1000 Hz ausgeschlossenen Samples (Lock-Cut-Marker) je
laut-Pass, lokalisiert gegen die Transient-Ereignisse:

| Anker (Mode st Tag) | Lauf (UTC) | Lock-Marker im Tag | Marker ±60 s um den Lauf (vor / in / nach) | Transient-Ereignisse |
|---|---|---|---|---|
| M1 st14 1995-11-24 | 16:47–18:55 | 4065 | 1471 (59 / 1352 / 60) | 7 Einzel-Samples 654–986 Hz, je 1 s vom nächsten Marker |
| M2 st14 1995-11-24 | 19:02–20:13 | 685 | 574 (46 / 495 / 33) | 2 (988 Hz-Burst 13 Samples; 169 Hz-Band 52 Samples), Marker-Distanz 0–1 s |
| M1 st63 1996-06-26 | 21:40–04:26 | 986 | 176 (50 / 78 / 48) | 1 (992 Hz, 29 Samples, am Laufende, Marker-Distanz 0 s) |
| M3 st43 1995-12-04 | 23:05–01:31 | 635 | 199 (1 / 78 / 120) | 1 (907 Hz, 2 Samples, am Laufende) |
| M3 st14 1995-12-05 | 16:23–18:11 | 888 | 127 (60 / 7 / 60) | 1 (987 Hz, 62 Samples, am Laufende, 60 Marker direkt nach dem Lauf) |
| M3 st63 1995-11-27 | 09:41–10:59 | 170 | 132 (1 / 114 / 17) | 4 (559–839 Hz, Burst 225 Samples mit Markern durchzogen) |
| M1 st43 1996-11-04 | 05:05–06:35 | 759 | 107 (0 / 47 / 60) | 1 (933 Hz, 11 Samples, am Laufende) |

Gemessen: Die Lock-Cut-Marker sitzen **nicht nur an den Pass-Rändern** — sie
liegen überwiegend **innerhalb der Lauf-Spanne** (M1 st14 1352/1471, M2 st14
495/574, M3 st63 114/132 in-Lauf-Marker) und damit **eingebettet in die
Transient-Region**; bei den End-Transient-Pässen (M3 st43, M3 st14, M1 st43)
liegen die Marker konzentriert direkt hinter dem Laufende (120 / 60 / 60 nach
dem Lauf) — der Lauf endet dort in den Lock-Verlust. Jedes der 17
Transient-Ereignisse sitzt ≤ 1 s an einem |resid| > 1000-Hz-Sample oder an
einer Lauf-Kante.

## Messung 2 — Cycle-Slip-Disziplin je laut-Pass (Flag-Kreuzung)

Für die 5 flag-bedeckten Pässe wurden die Roh-Records des Anker-Tages mit
RCVR_LOCK0 / DOPPLER_GOOD0 / DOPPLER_TOL0 / SLIPPED_CYCLE gelesen und je
Ereignis im engen Fenster (±2 s) gegen die resid-Transienten gekreuzt.
Je Pass: liegen die Transienten an einem Lock-/Good-Flag-Wechsel oder mitten
im gelockten Lauf?

| Anker | Flag-Zensus Tag (lock0: in/out) | Ereignis-Klasse (eng) |
|---|---|---|
| M1 st14 1995-11-24 | 11630 Records: 1165 in / 10465 out (out = 90 %) | 7/7 Ereignisse: 5/5 Records out-of-lock + good-bad, Slipped 2–17; Lauf durchgehend out-of-lock geflaggt |
| M2 st14 1995-11-24 | 7434: 3037 in / 4397 out (59 %) | Ereignis 19:15:50 (988 Hz): 17/17 Records in-lock + good, aber Slipped 15 → Empfangs-Counter-Slip im gelockten Lauf; Ereignis 19:24:56 (169 Hz): 95/95 out-of-lock, Slipped 42 |
| M1 st63 1996-06-26 | 25187: 20145 in / 5042 out (20 %) | 1/1 (Laufende): 34/34 out-of-lock, Slipped 33 |
| M3 st43 1995-12-04 | 6385: 6231 in / 154 out (2 %) | 1/1 (Laufende): 5/5 out-of-lock, Slipped 3; der ruhige Lauf sonst 11/6242 out-of-lock |
| M3 st14 1995-12-05 | 7355: 6104 in / 1251 out (17 %) | 1/1 (Laufende): 66/66 out-of-lock, Slipped 65 |

Je laut-Pass benannt:
- **M1 st14 1995-11-24**: Transienten **Lock-gebunden** — der Lauf trägt auf
  6106/6106 AGC-Boden-Records den Out-of-Lock-Flag; die 7 Ereignisse liegen in
  diesem durchgehend out-of-lock-geflagten Lauf an |resid| > 1000-Markern.
- **M2 st14 1995-11-24**: gemischt: Ereignis 19:15:50 ist ein
  **Empfangs-Counter-Slip im gelockten Lauf** (Flags nominal, SLIPPED_CYCLE
  15/17 — die Deduktion-0-Maske der Pioneer-PNAV-Erfahrung); Ereignis
  19:24:56 ist **Lock-gebunden** (95/95 out-of-lock).
- **M1 st63 1996-06-26, M3 st43 1995-12-04, M3 st14 1995-12-05**:
  Transienten **Lock-gebunden am Laufende** — der ruhige Lauf endet in einem
  Out-of-Lock-Block (17–20 % bzw. 2–17 % Out-of-Lock-Anteil im Tag, 100 % im
  Ereignis-Fenster), gefolgt von den resid-Lock-Markern.
- **M3 st63 1995-11-27, M1 st43 1996-11-04**: kein Roh-Cache des Tages
  (0 geehrt); resid-Alone-Maß: alle 5 Ereignisse sitzen ≤ 1 s an einem
  |resid| > 1000-Marker oder an der Lauf-Kante → Lock-Nachbarschaft gemessen,
  Flag-Achse `pending`.

## Messung 3 — Empfangs-Cycle-Slip gegen Sende-Predict/Rampe

Kriterium der Trennung: koinzidiert ein Transient mit einem Lock-/Good-Flag-
Wechsel oder einem Slipped-Cycle → Empfangs-Cycle-Slip (Downlink/Receiver);
erscheint ein Transient ohne Lock-Flag plötzlich in einem gelockten Lauf als
kohärenter Frequenz-Sprung (Größenordnung der 30-Hz-Predict-Regel) →
Sende-Predict-/Rampen-Sprung (Uplink).

Gemessen über die flag-bedeckten Pässe (12 Ereignisse):
- **n = 12** Ereignisse tragen im engen ±2-s-Fenster einen
  Out-of-Lock-/Good-bad-Flag (11 Ereignisse) oder einen SLIPPED_CYCLE > 0
  (12 Ereignisse; das 19:15:50-Ereignis von M2 st14 nur Slipped, Flags
  nominal). Klasse: **Empfangs-Cycle-Slip / Lock-Verlust**.
- **n = 0** Ereignisse zeigen die Send-Predict-Signatur: kein Transient
  erscheint in einem nominal gelockten, nicht-geslippten Lauf als kohärenter
  Sprung ohne Lock-/Good-/Slipped-Evidenz.
- Die resid-lock-cut-Marker (> 1000 Hz) koinzidieren auf den bedeckten Pässen
  überwiegend mit dem Out-of-Lock-Flag (M1 st14 4065/4065, M3 st43 143/143,
  M3 st14 882/888, M1 st63 941/986, M2 st14 455/685 der > 1000-Hz-Records
  out-of-lock) — der Lock-Cut der Floor-Pipeline ist auf diesen Pässen ein
  Empfangs-Lock-Verlust, kein Sende-Sprung.

## Verdikt

**Die lauten Pass-Transienten sitzen an Lock-/Cycle-Grenzen, nicht als
kohärente Sprünge im gelockten Lauf.** Über die flag-bedeckten Pässe tragen
12/12 Transient-Ereignisse eine Empfangs-Signatur (Out-of-Lock-Flag und/oder
Slipped-Cycle); die Send-Predict-/Rampen-Klasse ist dort n = 0 gemessen. Die
resid-Lock-Marker (> 1000 Hz) liegen überwiegend eingebettet in die
Transient-Region (M1 st14, M2 st14, M3 st63) oder direkt am Laufende in den
Lock-Verlust (M3 st43, M3 st14, M1 st43) — das Muster der Vorlagen
(End-Transienten, Multi-Burst, Lärmband) ist damit als Empfangs-Lock-Verlust/
Cycle-Slip-Familie benannt. Die 2 cache-freien lauten Pässe (M3 st63
1995-11-27, M1 st43 1996-11-04) zeigen resid-Alone dieselbe Lock-Nachbarschaft;
ihre Flag-Klasse bleibt `pending` (0 geehrt).

## Grenzen

- Die Flag-Kreuzung ist über die vier gezielten Roh-Caches geführt (5 der 7
  lauten Anker-Tage); die zwei cache-freien Tage sind am Flag nicht
  entscheidbar (0 geehrt, resid-Alone-Maß benannt).
- RCVR_LOCK0/DOPPLER_GOOD0 sind auf dem lauten st14-Tag 1995-11-24 über den
  ganzen Lauf out-of-lock geflaggt (6106/6106 AGC-Boden-Records) — der Flag
  markiert dort einen andauernden Empfangs-Zustand, kein einzelnes
  Ereignis-Bit; die Ereignis-Kreuzung ist zusätzlich über den Slipped-Cycle-
  Zähler und die resid-Lock-Marker geführt.
- Die Send-Predict-Klasse ist über das Fehlen von Empfangs-Flags definiert
  (kein Ereignis erfüllt das Kriterium); die 30-Hz-Predict-Größenordnung
  (HANDBK6 §5.1.2.3) ist als Kriterium benannt, nicht als Quelle im Repo
  geführt.
- Die Day-Cell-Reproduktion deckt sich mit der Referenz auf alle
  ausgegebenen Dezimalen (n und RMS je Anker), die Ereignis-Zählung ist an
  die 100-Hz-/60-s-Definition gebunden (benannt, nicht geglättet).

## Register-Satz

*Die Cycle-Slip-Disziplin der 7 lauten Galileo-Floor-Anker-Pässe ist gemessen
(Richtung CS): die Transient-Ereignisse sitzen an Lock-/Cycle-Grenzen — über
die Roh-Caches der bedeckten 5 Pässe tragen 12/12 Ereignisse im engen ±2-s-
Fenster einen RCVR_LOCK0-Out-of-Lock- oder DOPPLER_GOOD0-Flag oder einen
SLIPPED_CYCLE > 0 (11 out-of-lock-geflaggt, 12 mit Slipped-Cycle; das
19:15:50-Ereignis von M2 st14 1995-11-24 ist ein Empfangs-Counter-Slip mit
nominalen Flags und Slipped 15/17 — die Deduktion-0-Maske); die
Send-Predict-/Rampen-Klasse (kohärenter Sprung ohne Lock-/Slipped-Evidenz im
gelockten Lauf) ist auf den bedeckten Pässen n = 0. Die resid-Lock-Marker
(|resid| > 1000 Hz) liegen überwiegend eingebettet in die Transient-Region
(M1 st14 1352/1471, M2 st14 495/574, M3 st63 114/132 in-Lauf) oder direkt am
Laufende in den Lock-Verlust (M3 st43 120, M3 st14 60, M1 st43 60 Marker nach
dem Lauf); die Day-Cells reproduzieren die Referenz exakt. Die zwei
cache-freien lauten Pässe (M3 st63 1995-11-27, M1 st43 1996-11-04) zeigen im
resid-Alone-Maß dieselbe Lock-Nachbarschaft (alle 5 Ereignisse ≤ 1 s an einem
resid-Lock-Marker oder an einer Lauf-Kante); ihre Flag-Klasse bleibt pending
(0 geehrt).*
