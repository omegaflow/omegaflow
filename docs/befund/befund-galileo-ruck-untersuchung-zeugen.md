<!--
  title: Befund — Untersuchung des einen gemessenen Galileo-Rucks (D4): der Mode-1-Niveau-Sprung 1995-11-30/12-01 sitzt ausschließlich auf der resid-Achse (empfangen minus Modell), nicht auf der Referenzfrequenz-Spur (ref steigt über die Grenze monoton um +29/+42/+57 Hz, ganzzahlig-quantisiert, kein −0,8-Hz-Schritt auflösbar); sub-tägig ist der Übergang Pass-zu-Pass (jeder Pass hält ein konstantes Niveau) zwischen 1995-11-30 03:17 und 18:20 UTC, mit dem anormalen DSS-63-Pass 09:02–11:18 (resid exakt 0,000, n 334) in der Lücke; galileo_skyfreq.bin deckt den Übergang nicht (nur 08./09.12.), Pioneer 10 wurde am 1995-11-30 an keiner der drei Stationen getrackt (0 Zellen) und sein Residuum ist epoch-offset-bereinigt (keine Tages-Niveau-Zeugenachse); M2/M3 tragen keinen Schritt — Verdikt: unbestimmt zwischen realem empfangenen Einweg-Frequenzschritt (−0,8 Hz S/C-seitig) und sub-Hz-Korrektur des Einweg-Modells/Predicts, beides verträgt die gemessene Signatur; externes Betriebs-Event 1995-11-30/12-01 nicht belegt (pending)
  class: befund
  date: 2026-09-06
  sha256: 55f9d03c55155d09f4628683a7764b6bf22d970d944259eea5255fd5b8e71682
  status: draft
  antwortet-auf: docs/befund/befund-galileo-ruhige-basis-ruck-stufe.md
  see-also: docs/befund/befund-galileo-ruhige-basis-drift.md docs/befund/befund-galileo-floor-pass-episodisch.md docs/befund/befund-galileo-pioneer-stationsfloor-kreuz.md
-->
# Befund: der eine Galileo-Ruck (D4) — der −0,8-Hz-Niveau-Sprung 1995-11-30/12-01 sitzt auf der resid-Achse gegen eine glatt laufende, ganzzahlig quantisierte Referenz-Spur; sub-tägig Pass-zu-Pass (nicht inner-Pass); skyfreq und Pioneer tragen keine Zeugen; Verdikt unbestimmt (echtes Empfangs-Event vs. sub-Hz-Modell-/Predict-Korrektur), externes Event nicht belegt

## Frage & Bindung

Dieser Lauf (Richtung D4, aufbauend auf dem Ruck-Befund `b8226ab`) untersucht den
**einen gemessenen Ruck** der ruhigen Galileo-Floor-Resid-Basis — den abrupten,
anhaltenden **Mode-1-nur**-Niveau-Sprung am Era-Übergang **1995-11-30/12-01**
(−0,80…−0,82 Hz Nachbar-Tag, simultan über st14/43/63, ausbleibend in Modi 2/3,
keine Rückkehr zum +0,75-Hz-Öffnungs-Plateau) — auf vier Achsen:

1. **Echt vs. Modell:** liegt der Niveau-Sprung und das +0,7…+0,85-Hz-Öffnungs-Plateau
   im Feld **ref_hz** (Doppler-Referenzfrequenz, GASR-Slot 5) oder nur im **resid**
   (empfangen minus Modell)? Vergleich der Tages-Niveau-Reihen beider Felder über
   den Sprung.
2. **Zeuge Pioneer:** wurde Pioneer 10 am 1995-11-30/12-01 an st14/43/63 getrackt,
   und trägt sein Residuum denselben Sprung?
3. **Sub-Tages-Struktur:** scharf (ein Sample-Zeitpunkt) oder verteilt; in welcher
   Pass-/Tageszeit springt das Niveau; intraday-Anstieg oder sauberer Pass-Schritt?
4. **Externer Kontext:** dokumentiertes Galileo-Raumfahrzeug-/Betriebs-Event um
   1995-11-30/12-01 (Transponder/USO/Modulation/Befehl/Predict/Ephemeriden)?

Gebunden wie die Ruck-Vorlage: Tages-Zelle (Mode, Station, Tag) über die
in-track-Boden-Residuen (Stärke exakt −2560, |resid| ≤ 1000 Hz); robust = Zell-n
≥ 30; ruhig = Zell-RMS < 1 Hz; Tages-Niveau = Tages-Median; Tag-Label = gerundeter
TDB-Tag (die Achse der Ruck-Befunde; der mit D gelabelte Tag umspannt UTC D−1
12:00 bis D 12:00 — die Sub-Tages-Zeiten unten sind echte UTC). Zusätzlich zu den
Tages-Zellen wird je (Station, Tag) der Median der **Referenzfrequenz** (GASR-Slot
5, aus `doppler_ref/10`, in Hz) geführt und je Pass die Segment-Struktur (Lücken
> 300 s) mit Start/Ende in UTC. Daten:
`data/pds-ppi.igpp.ucla.edu/galileo_resid.bin`; Zeugen-Daten
`galileo_skyfreq.bin` (GASF, 14 Felder) und
`data/spdf.gsfc.nasa.gov/pioneer{10,11}_navio_residuum.bin` (P11R).
Probe `tools/measure/src/bin/galileo_ruck_zeugen.rs` (neu, `cargo check` 0/0,
RUSTFLAGS `-D warnings`), Report
`/tmp/opencode/galileo_ruck_zeugen_report.txt`.

## n zuerst (0 geehrt) — die Reihen und die Dichten

Resid-Bin: 14 077 825 Records; Mode-1-Trio-Floor-Samples der Ära
(1995-11-23..1997-02-28): **2 088 475**, Lock-Samples der Mode-1-Trio-Zellen
ausgeschlossen: 1 413 861. Die ruhigen robusten M1-Tages-Zellen werden exakt wie
im Ruck-Befund reproduziert (st14 30, st43 29, st63 34 ruhige Tage); dazu je Zelle
der ref-Feld-Median. 0 geehrt: bodenleere Monate bleiben Lücken; der
skyfreq-Zeuge deckt den Übergang nicht (0); Pioneer 11 hat im Fenster keine
Trio-Zellen (0); Pioneer 10 hat am 1995-11-30 keine Trio-Zelle (0).

## Messung 1 — der Sprung sitzt auf der resid-Achse; die Referenz-Spur läuft glatt durch

Je M1-Serie ist die Referenzfrequenz über die ruhigen Tage **ganzzahlig
quantisiert (Hz) und innerhalb fast aller ruhigen Pässe konstant** (span 0 in den
Plateau-Zellen und den Folge-Zellen). Über die Ära steigt der Tages-Median der
Referenz monoton mit der Geometrie (~+25…+30 Hz/Tag); die mittlere Tages-Schritt-
Größe der ref-Mediane über benachbarte ruhige Tage liegt bei 62 Hz (st14),
88 Hz (st43), 26 Hz (st63), der größte ref-Tages-Schritt bei 852–2149 Hz — die
Median-Tages-Schritte der **resid**-Niveaus derselben Serien liegen bei
0,049/0,026/0,021 Hz. Der Ruck (−0,80…−0,82 Hz) ist damit 15–40× die resid-
Tages-Schritt-Mitte, aber 30–110× **unter** der ref-Tages-Schritt-Mitte und
unter der Integer-Hz-Quantisierung der Referenz.

**Das Ruck-Paar, gemessen (letzte Plateau-Zelle → erste niedrige Zelle):**

| Serie | Paar (Tag-Label) | resid-Schritt Hz | ref-Schritt Hz |
|---|---|---|---|
| M1 st14 | 1995-11-30 → 12-01 | **−0,819** | +29 |
| M1 st43 | 1995-11-30 → 12-02 | **−0,800** | +42 |
| M1 st63 | 1995-11-29 → 12-01 | **−0,802** | +57 |

Die ref-Spur steigt über die Grenze um +29…+57 Hz **weiter an** (Vorzeichen
entgegen dem resid-Schritt, Größe 35–70×) — sie trägt keinen −0,8-Hz-Schritt und
keinen Knick der Ruck-Größenordnung. Das +0,7…+0,85-Hz-Öffnungs-Plateau der
ersten Woche erscheint in der ref-Spur nicht (dort nur die monotone
~+29-Hz/Tag-Geometrie-Rampe); es ist ausschließlich im resid (empfangen minus
Modell) messbar. Da resid = empfangen − Modell und die Referenz die
integer-Hz-zählende Spur ist, ist der gemessene Befund präzise: **der Ruck ist
ein Niveau-Wechsel des resid gegen eine glatt laufende, quantisierte
Referenz-Spur** — er ist auf der ref-Achse nicht nur abwesend, sondern wegen der
Integer-Hz-Quantisierung prinzipiell nicht auflösbar (0 geehrt). Ein sub-Hz-
Predict-/Modell-Korrekturterm (< 1 Hz) würde die integer-quantisierte ref-Spur
ebenfalls nicht verschieben — die ref-Achse trennt die zwei Unter-Fälle nicht.

## Messung 2 — Zeuge Pioneer am 1995-11-30/12-01: kein Tracking am Sprungtag (0)

Pioneer 10 wurde um den Übergang an den Trio-Stationen getrackt, aber **nicht am
1995-11-30** (0 Trio-Zellen an diesem Tag) und an den Nachbar-Tagen je nur kurz
und auf wechselnden Stationen: 11-27 st63 n 68, 11-28 st63 n 59, 11-29 st63 n 25,
**12-01 st43 n 41**, 12-02 st14 n 27, 12-03 st14 n 36. Pioneer 11 hat im Fenster
1995-11-15..12-20 keine Trio-Zelle (0). Die Pioneer-10-Residuen der 11-30-benachbarten Pässe (11-27..12-03) liegen in der
Größenordnung 10³–10⁵ Hz (Tages-Median −36 192…+19 136 Hz bzw.
+13 307/−1 391 Hz je Station; nur der weiter außen liegende 11-26 mit
−20,6/−1,8 Hz) — Pässe mit großen unmodellierten Abweichungen, nicht die
ruhige Klasse — und damit weit über der 0,8-Hz-Skala. Dazu ist die Achse
selbst keine Niveau-Zeugenachse: das P11R-Residuum ist das Residuum des
Navio-Compilers mit **Per-Epochen-Offsets** (fixed-effects, Pass-/Block-Offsets
entfernt) — ein Tages-Niveau-Schritt wird mit dem Offset abgezogen und ist auf
dieser Achse nicht messbar (Eigenart der Reduktion, gemessen, benannt). Beide
Gründe zusammen: **der Pioneer-Zeuge ist leer** (0 geehrt), nicht widerlegend und
nicht bestätigend.

## Messung 3 — Sub-Tages-Struktur: Pass-zu-Pass, jeder Pass auf konstantem Niveau

Die ruhigen M1-Pässe um den Übergang (in-track-Segmente, Lücke > 300 s, in UTC):

| Station | letzter Plateau-Pass | Niveau Hz | erster niedriger Pass | Niveau Hz |
|---|---|---|---|---|
| st14 | 1995-11-29 18:24–18:42 (n 1055, rms 0,179) | +0,847 | 1995-11-30 18:20–18:47 (n 1602, rms 0,026) | +0,028 |
| st43 | 1995-11-30 00:10–03:17 (n 16622, rms 0,079) | +0,833 | 1995-12-02 02:46–07:14 (n 16102, rms 0,026) | +0,033 |
| st63 | 1995-11-29 11:01–11:33 (n 1875, rms 0,144) | +0,848 | 1995-12-01 10:50–11:30 (n 2408, rms 0,085) | +0,046 |

**Jeder Pass hält ein einziges, konstantes Niveau** (inner-Pass-rms 0,026–0,179 Hz
gegenüber dem 0,8-Hz-Schritt); ein intra-Pass-Übergang (Sample-zeitlicher Sprung
innerhalb eines Passes) ist in keinem der Pässe gemessen. Der Niveau-Wechsel
liegt zwischen dem Ende des letzten hohen Passes (st43, 1995-11-30 **03:17 UTC**)
und dem Beginn des ersten niedrigen (st14, 1995-11-30 **18:20 UTC**) — ein
~15-Stunden-Fenster am 1995-11-30, Tag-zu-Tag-Schritt, kein sub-täglicher
intraday-Anstieg/-Abfall. In diesem Fenster liegt genau ein weiterer M1-Pass:
**st63 1995-11-30 09:02–11:18 UTC, n 334, mit resid exakt 0,000 (rms 0,0000) und
ref exakt 22013672 Hz** — ein anormaler Pass (alle 334 Residuen exakt null,
Referenz am später vielfach genutzten Nominalwert), der weder hohes noch niedriges
Niveau trägt und die Lücke nicht schließt; er ist als eigene Pass-Klasse benannt,
nicht als Übergangspunkt.

## Messung 4 — externer Kontext: kein belegtes Event am 1995-11-30/12-01 (pending)

Geprüfte lokale Referenzen: `docs/reference/96-1505.pdf` (Galileo-S-Band-
Telekom-Umbau/DGT, kein Datums-Event am Übergang); die DSN-`appendixA`-OCR-Logs
des `galileo_dsn_sched` („GALILEO USO ACTIVITIES", „GLL Gravitational Wave
Experiment '95") dokumentieren eine **USO-referenzierte Einweg-S-Band-Kampagne**
an DSS 14/43/63 und enthalten eine „30-Nov"-USO-Test-Zeile (Doy 334) —
deren **Jahr ist aus dem OCR-Material nicht verlässlich bestimmbar** (Wochentag
inkonsistent); sie wird deshalb nicht auf 1995 datiert zitiert (0 geehrt, keine
Fabrication). Die Wikipedia-Missionschronologie von Galileo nennt kein
Raumfahrzeug-/Betriebs-Event am 1995-11-30/12-01 (JOI 1995-12-07 ist das nächste
datierte Ereignis). **Damit ist ein dokumentiertes Event am Sprung-Tag
unbelegt — benannt als unbekannt/pending, nicht behauptet.**

## Größenordnung & Einordnung

Der Ruck (−0,80…−0,82 Hz Nachbar-Tag, reproduziert) ist 15–40× die
resid-Tages-Schritt-Mitte der ruhigen Serien (0,021–0,049 Hz), 30–110× unter der
ref-Tages-Schritt-Mitte (26–88 Hz) und unter der Integer-Hz-Quantisierung der
Referenz — er ist damit die einzige sub-Hz-Diskontinuität der ruhigen Basis, die
auf der resid-Achse messbar ist, und er ist auf der ref-Achse prinzipiell nicht
auflösbar. Die **Mode-1-Spezifität** (Modi 2/3 tragen keinen Schritt am selben
Kalender-Tag, Ruck-Befund) benennt den Sitz auf dem Einweg-Pfad: die
uplink-kohärenten Modi sind an die Transponder-Ratio gebunden und teilen den
S/C-Oszillator-Term nicht. Die **Stations-Simultaneität** (gleicher Schritt an
drei unabhängigen Stationen, deren ref-Spuren geometriebedingt verschieden sind)
benennt den Term als dem Raumfahrzeug-Signal bzw. seiner gemeinsamen Modell-
Referenz gemeinsam — nicht Stations-/Uhr-/Geometrie-Eigenschaft. Ein
Pioneer-artiger unmodellierter Akzelerations-Term ist es nicht (Ruck + langsame
Senkung tragen keine monotone Rampe; Drift-Befund).

## Verdikt

**Der gemessene Ruck ist ein Niveau-Wechsel des resid (empfangen minus Modell)
gegen eine glatt laufende, ganzzahlig quantisierte Referenz-Spur.** Gemessen:
(a) das +0,7…+0,85-Hz-Öffnungs-Plateau (erste Woche) und der −0,80…−0,82-Hz-
Nachbar-Tag-Sprung (1995-11-30/12-01, st14/st43/st63, Mode 1) liegen
ausschließlich auf der resid-Achse; die ref-Spur steigt über die Grenze monoton
um +29/+42/+57 Hz (kein −0,8-Hz-Schritt, keine Auflösung unter 1 Hz möglich, 0
geehrt); (b) der Übergang ist Pass-zu-Pass (jeder Pass auf konstantem Niveau,
inner-Pass-rms 0,03–0,18 Hz), zwischen 1995-11-30 03:17 und 18:20 UTC, mit dem
anormalen st63-Null-Pass 09:02–11:18 (resid exakt 0,000, n 334) in der Lücke;
(c) die Zeugen-Achsen sind leer gemessen: `galileo_skyfreq.bin` deckt nur
08./09.12.1995 (Übergang 0), Pioneer 10 wurde am 1995-11-30 an keiner
Trio-Station getrackt (0) und sein Residuum ist epoch-offset-bereinigt
(keine Niveau-Zeugenachse) mit 10³–10⁵-Hz-Residuen in den Nachbar-Pässen;
Pioneer 11: keine Trio-Zelle im Fenster. Damit trennt die Messung **nicht**
zwischen den zwei Unter-Fällen, die beide die Signatur tragen: (i) ein **realer
empfangener Einweg-Frequenzschritt** des Raumfahrzeug-Signals (−0,8 Hz S/C-seitig,
z. B. Oszillator-/Sende-Referenz-Wechsel) und (ii) eine **sub-Hz-Korrektur des
Einweg-Modells/Predicts** (S/C-Frequenz-Term der Vorhersage, wirksam ~1.12.1995),
die die integer-quantisierte ref-Spur nicht verschiebt. Die Mode-1-Spezifität
und Stations-Simultaneität sind mit beiden verträglich; ein gemeinsamer
Modell-/Reduktions-Schritt über alle Modi ist durch das M2/M3-Ausbleiben
ausgeschlossen. **Verdikt: unbestimmt zwischen echtem Empfangs-Event und
Modell-/Predict-Korrektur auf dem Einweg-Pfad** — gemessen sind die Signatur, die
n, die Skalen und das Ausbleiben der Zeugen; die Zahl (0,8 Hz an einem Tag,
n = 30/29/34 ruhige M1-Tage je Serie, 2 088 475 Floor-Samples) trägt keinen
Anomalie-Anspruch über diese Klasse hinaus.

**Was `pending` bleibt:** die Trennung echt-vs-Modell (braucht das Betriebs-/
Predict-Register der DSN um den 1.12.1995 — `docs/reference`-Bereich DSN-Material,
Abschnitt „frequency predictions, operator logs"; im lokalen Bestand nicht
belegt); die Ursache des st63-Null-Passes 1995-11-30 09:02–11:18 UTC; eine
Pioneer-Niveau-Messung auf der Roh-Pass-Ebene (ohne Offset-Entfernung) um den
Übergang — hier nicht gerechnet, weil die P11R-Achse sie nicht trägt und die
Pioneer-Pässe dort kurz und laut sind; ob der +0,7-Hz-Plateau-Offset ein
Anfangs-Predict-Fehler der ersten Messwoche war. Der Verdikt bleibt `draft`
(Entwurf für die Haupt-Session/den Rat).

## Grenzen

- Die ref-Achse ist integer-Hz-quantisiert (span 0 in fast allen ruhigen Zellen)
  und folgt der Geometrie-Rampe — ein sub-Hz-Term (Modell- oder Empfangs-) ist
  auf ihr prinzipiell nicht auflösbar; „die ref trägt den Ruck nicht" ist damit
  eine gemessene Nicht-Auflösbarkeit, keine Widerlegung einer der zwei Klassen.
- Die Tages-Label der Serie (und der Vor-Befunde) sind die gerundeten TDB-Tage;
  die Sub-Tages-Zeiten sind als echte UTC angegeben (Konvention: der mit D
  gelabelte Tag umspannt UTC D−1 12:00..D 12:00). Die Zuordnung ist über die
  Record-Zeiten der Reduktion getragen, nicht über eine unabhängige Uhr.
- Das ~15-h-Übergangsfenster (11-30 03:17–18:20 UTC) enthält außer dem st63-Null-
  Pass keine ruhigen M1-Pässe; wo genau im Fenster der Niveau-Wechsel liegt, ist
  aus den Floor-Daten nicht auflösbar (0 geehrt).
- Der Pioneer-Zeuge ist doppelt leer (kein 11-30-Tracking; Achse offset-
  bereinigt); eine Roh-Pass-Niveau-Messung (pnav, ohne Offset-Entfernung) ist
  nicht gerechnet und bleibt pending.
- Die externen OCR-Schedule-Logs tragen Jahr-Mehrdeutigkeit (Wochentag/DoY
  inkonsistent); sie werden nicht als Datums-Beleg für 1995 zitiert.
- Die M2/M3-Sauberkeit ist aus dem Ruck-Befund übernommen (dort gemessen), nicht
  in diesem Lauf neu vermessen.

## Register-Satz

*Die Untersuchung des einen Galileo-Rucks (1995-11-30/12-01, −0,80…−0,82 Hz,
Mode 1, st14/43/63 simultan, anhaltend): der Niveau-Sprung und das
+0,7…+0,85-Hz-Öffnungs-Plateau liegen ausschließlich auf der resid-Achse —
die Referenzfrequenz-Spur (GASR-Slot 5, integer-Hz-quantisiert, innerhalb der
ruhigen Pässe konstant) steigt über die Grenze monoton um +29 (st14)/+42
(st43)/+57 Hz (st63) weiter (Tages-Schritt-Mitte 26–88 Hz gegen
resid-Tages-Schritt-Mitte 0,021–0,049 Hz); ein −0,8-Hz-Schritt ist auf der
ref-Achse nicht auflösbar (unter der Quantisierung, 0 geehrt). Sub-tägig ist der
Übergang Pass-zu-Pass (jeder Pass auf konstantem Niveau, inner-Pass-rms
0,03–0,18 Hz) zwischen 1995-11-30 03:17 und 18:20 UTC; in der Lücke liegt der
anormale st63-Null-Pass 1995-11-30 09:02–11:18 UTC (resid exakt 0,000, n 334,
ref 22013672 Hz). galileo_skyfreq.bin deckt den Übergang nicht (nur 08./09.12.,
0 geehrt); Pioneer 10 wurde am 1995-11-30 an keiner Trio-Station getrackt (0
Zellen; Nachbar-Pässe 11-27/28/29 st63 n 68/59/25, 12-01 st43 n 41, 12-02 st14 n
27) und sein Residuum ist epoch-offset-bereinigt (keine Niveau-Zeugenachse) mit
10³–10⁵-Hz-Residuen — der Zeuge ist leer, nicht widerlegend; Pioneer 11: keine
Trio-Zelle im Fenster. M2/M3 tragen keinen Schritt (Ruck-Befund). Verdikt:
unbestimmt zwischen einem realen empfangenen Einweg-Frequenzschritt (−0,8 Hz
S/C-seitig) und einer sub-Hz-Korrektur des Einweg-Modells/Predicts (~1.12.1995);
ein gemeinsamer Modell-Schritt über alle Modi ist durch das M2/M3-Ausbleiben
ausgeschlossen. Externes Betriebs-Event am 1995-11-30/12-01 nicht belegt
(unbekannt/pending; die USO-Activity-OCR-Logs tragen Jahr-Mehrdeutigkeit und
werden nicht als 1995-Datums-Beleg zitiert). Pending: echt-vs-Modell-Trennung
(DSN-Predict-/Ops-Register), Ursache des st63-Null-Passes, Pioneer-Roh-Pass-
Niveau-Messung. Probe galileo_ruck_zeugen (cargo check 0/0, -D warnings), Report
/tmp/opencode/galileo_ruck_zeugen_report.txt.*

## Status

`draft` (Entwurf für die Haupt-Session/Rat; die TODO-Registerzeile ergänzt die
Haupt-Session). Probe `galileo_ruck_zeugen` committet (`cargo check` 0/0,
RUSTFLAGS `-D warnings`), Report
`/tmp/opencode/galileo_ruck_zeugen_report.txt`. Richtung D4 (Zeugen des einen
Rucks) ist gemessen; die Echt-vs-Modell-Trennung bleibt pending.
