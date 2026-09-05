<!--
  title: Befund — Galileo-Floor-Gipfel auf der Sub-Tages-Achse: Pass-getaktete Episoden, keine getaktete Steuer-/Kalibrier-Kadenz — keine phasen-kongruente Tageszeit über die laut-Tage (nur die stationseigene Pass-Tageslage), keine dominante Wiederkehr-Periode über der Raster-Kontrolle, 1–3-Tage-Bursts ohne feste Stations-Reihenfolge
  class: befund
  date: 2026-09-05
  sha256: 28e775ff19e7451549953a377eecff3cf6c7b39776af2661dd7ff57a883a56cd
  status: draft
  antwortet-auf: docs/befund/befund-galileo-floor-stufen-te.md docs/befund/befund-galileo-floor-ort-himmelskoerper.md
  see-also: docs/befund/befund-galileo-banden-kamm-ton.md docs/paper/twenty-second-band-ground-chain.md docs/paper/ground-sources-20s-band.md
-->
# Befund: Galileo-Floor-Gipfel auf der Sub-Tages-Achse — Pass-Takt, kein Steuer-/Kalibrier-Signal

## Frage & Bindung

Die Stufen-/Ort-Blätter (draft) lassen als letzten offenen Anker die
**Sub-Tages-Achse**: Pass-Beginn, Episoden-Anfang innerhalb des Tages. Die
Frage (Richtung E): Ist die wiederkehrende station-gebundene Lautheit ein
getaktetes Ereignis — ein Steuersignal, ein Update, eine Kalibrierung, die
wie eine Sinuskurve regelmäßig wiederkommt? Konkret: (a) kommen die
laut-Episoden periodisch/regelmäßig wieder, (b) sind sie phasen-kongruent
(fangen sie zu derselben Tageszeit an), (c) gibt es innerhalb der
laut-Episoden wiederkehrende Muster (Dauern, Bursts, Stations-Reihenfolge)?

Bindung wie die Vorlagen: Boden = Stärke exakt −2560; Zelle = (Mode,
Station, TDB-Tag); Lock (|resid| > 1000 Hz) vor dem Rauschen getrennt;
Zell-RMS um den Zellen-Mittelwert; robuste Serie = Zellen n ≥ 30; laut =
Zell-RMS ≥ 1 Hz. Zusätzliche Sample-Ebene: jeder Floor-Sample des Tages mit
tdb und resid; Pass-/Episoden-Grenze innerhalb des Tages = tdb-Lücke
> 600 s (Gap-Kriterium der TX-Sequence-Probe und der
Pass-Segmentierung). Tageszeit = UTC-Stunde aus tdb (civil_from_days-
Konvention der Vorlagen); Pass-Anfang = tdb der ersten Probe eines
zusammenhängenden Laufs; laut-Probe = |resid − Zellen-Mittel| ≥ 1 Hz
(genannter Lautheits-Grenzwert, konsistent zur 1-Hz-Tagesschwelle).
Zielgröße = die laut-Zellen der robusten Serie. Wichtig (n zuerst): die
Reproduktion der Zell-Definition ergibt **207 laut-Zellen** (18/14/15 +
9/12/11 + 10/9/7 laut je Serie in der Reihenfolge der Register — Summe der
laut-Spalten), **nicht 105**; die „105" der Register ist die **Flip-Zählung**
(Übergänge laut↔ruhig zwischen benachbarten robusten Tagen). Beides wird
gemessen und benannt; alle Sub-Tages-Analysen laufen über die 207
laut-Tag-Zellen mit n zuerst. Probe `galileo_floor_subday_clock_probe.rs`
(neu, einzige Repo-Änderung außer diesem Blatt; `cargo check` 0/0,
RUSTFLAGS `-D warnings`), Report `/tmp/opencode/
galileo_floor_subday_clock_report.txt`. Daten: `data/galileo_resid.bin`.

## Schritt 0 — die analoge Pioneer-Deduktion (benannt)

**Der analoge Fall:** die Pioneer-11-**TX-Sequenz-Sinussuche**
(`pioneer11_tx_sequence_probe.rs`) — eine vermutete getaktete Boden-/
Raumfahrzeug-Struktur (Steuer-/Wechsel-Kadenz), geprüft wie eine Sinuskurve
auf periodische Wiederkehr je Sender-Paar, mit Slope-Detrend je Segment und
Lomb-Scargle-Peak gegen den lokalen Boden. Zusammen mit der
**20-s-Band-Kadenz-Disziplin** (`befund-galileo-banden-kamm-ton.md`,
`ground-sources-20s-band.md`): dort wurde ein vermuteter 20-s-Takt als
Raster-Degeneranz des 60-s-Abtastgitters entlarvt — rohes Lomb-Scargle
allein degeneriert; die Trennung kam aus dem **normierten LS
(Varianz-Erklärung in [0,1])** und der **Rausch-Reihe auf demselben
Zeitgitter** als Kontrolle. Die Lektion wird auf Galileo übertragen:
**Abtast-/Reduktions-Kadenz ist nicht gleich physikalische Periode** — hier
konkret: die Pass-Kadenz der Stationen (täglich ein Pass zu
stations-spezifischer Tageszeit, ~4 min/d Drift durch die Sternzeit) darf
nicht als „Steuersignal-Periode" verkauft werden.

Übernommene Elemente: (1) Lomb-Scargle auf der Tages-RMS-Reihe je
(Mode, Station) mit **normiertem Maß + Surrogat-Raster-Kontrolle**
(dieselben Werte auf demselben Tag-Gitter permutiert); (2)
**Epoch-Faltung** der laut-Tage auf Kandidaten-Perioden mit
Phasen-Bündelungs-Maß (Rayleigh-R); (3) **Kadenz-/Intervall-Rechnung**
(Abstände zwischen aufeinanderfolgenden laut-Tagen je Serie); (4) die
Levy-et-al.-(2009)-Disziplin der Trennung periodisch gegen säkular (hier:
gegen die datenleeren Monate und die Beobachtungs-Fenster-Struktur).

## n zuerst (0 geehrt)

14 077 825 Residuen, 1 994 510 Lock-/Nicht-Endlich-Samples ausgeschlossen;
490 Floor-Zellen über den Trio-Stationen. Reproduktion der robusten Serie
(n ≥ 30, laut = RMS ≥ 1 Hz):

| Serie | robuste Tage | laut | ruhig | Flips |
|---|---|---|---|---|
| M1 st14 | 62 | 32 | 30 | 18 |
| M1 st43 | 64 | 35 | 29 | 14 |
| M1 st63 | 68 | 34 | 34 | 15 |
| M2 st14 | 38 | 17 | 21 | 9 |
| M2 st43 | 35 | 16 | 19 | 12 |
| M2 st63 | 39 | 22 | 17 | 11 |
| M3 st14 | 40 | 25 | 15 | 10 |
| M3 st43 | 33 | 16 | 17 | 9 |
| M3 st63 | 21 | 10 | 11 | 7 |
| **Summe** | **400** | **207** | **193** | **105** |

Die Zahlen reproduzieren die Register exakt (laut 207, Flips 105). **Die
auftragsseitige „105" ist damit die Flip-Zählung, nicht die Zahl der
laut-Tag-Zellen (207).** Beide werden benannt; die Sub-Tages-Messungen
laufen über alle 207 laut-Zellen (0 geehrt, n je Bin/Zelle).

## Messung A — Sub-Tages-Pass-/Episoden-Struktur je laut-Tag

Je laut-Tag-Zelle werden die Floor-Samples des Tages in Läufe (Gap ≤ 600 s)
zerlegt. Beispiele aus dem Report (gemessen):

- 1995-11-24 M1 st14 (rms 25,85 Hz): 2 Läufe, Start 16,8 h und 21,2 h UTC,
  Längen 128/23 min, 7463 Samples, 4423 laut — die Lautheit sitzt im
  ganzen Abend-Fenster (16,8–21,6 h).
- 1995-12-03 M1 st14 (9,74 Hz): 1 Lauf 16,6–18,7 h, 7524 Samples, nur 34
  laut — die Tages-Lautheit sitzt in wenigen lauten Samples im Fenster.
- 1996-06-26 M1 st63 (20,64 Hz): 1 Lauf 21,7 h–4,4 h (über Mitternacht),
  24 201 Samples, 159 laut.
- 1996-09-06 M1 st14 (58,6 Hz): 1 Lauf 0,5–0,8 h UTC, 1166/1166 laut —
  kurzer, vollständig lauter Sub-Pass.
- 1997-02-13 M1 st43 (2,47 Hz): 1 Lauf 18,9 h–7,4 h, 46 819 Samples, 934
  laut.
- 1997-01-18 M1 st14 (79,8 Hz): 17 kurze Läufe 15,7 h–0,4 h, je 1–31 min,
  insgesamt 1572 Samples, alle laut.

**Pass-Struktur:** 207 laut-Tage tragen 402 Sample-Läufe; 123 laut-Tage
haben genau 1 Lauf, 84 haben ≥ 2 Läufe. Die laut-Proben-Anteile am Tag
verteilen sich bimodal: 106 laut-Tage mit ≥ 90 % lauten Proben (die
Lautheit füllt den Pass), 66 laut-Tage mit < 10 % lauten Proben (die
Tages-Lautheit sitzt in wenigen Samples), 35 dazwischen. Die Lautheit ist
damit nicht an einen festen intraday-Zeitpunkt gebunden — sie kann den
ganzen Pass, einen schmalen Sub-Pass oder vereinzelte Samples füllen.

## Messung B — Tageszeit-Kongruenz (Pass-Anfangs-UTC-Stunden gefaltet)

Faltung auf den Tages-Phasen-Kreis (0–24 h), 1-h-Bins mit n:

| Größe | n | Bins (n je 1-h-Bin, 0 geehrt) | mean | Rayleigh-R |
|---|---|---|---|---|
| laut-Onset (erste laut-Probe je laut-Tag) | 207 | 0:11 1:3 2:3 3:3 4:5 5:2 6:2 7:4 8:8 9:10 10:4 11:1 12:23 13:6 14:11 15:12 16:17 17:19 18:4 19:16 20:12 21:13 22:7 23:11 | 17,29 h | 0,304 |
| Pass-Lauf-Start (alle 402 Läufe der laut-Tage) | 402 | 0:20 1:5 2:6 3:5 4:11 5:6 6:6 7:18 8:17 9:27 10:8 11:8 12:36 13:10 14:14 15:20 16:29 17:28 18:14 19:22 20:21 21:23 22:18 23:30 | 17,39 h | 0,192 |
| Episoden-Onset (erste laut-Probe je laut-Run, 137 Episoden) | 137 | 0:5 1:3 2:3 3:0 4:2 5:2 6:1 7:2 8:8 9:8 10:3 11:1 12:12 13:4 14:7 15:9 16:14 17:10 18:3 19:13 20:6 21:9 22:4 23:8 | 17,04 h | 0,315 |
| **Kontrolle: ruhige robuste Tage**, erste Floor-Probe | 193 | 0:5 1:4 2:4 3:3 4:3 5:4 6:1 7:1 8:9 9:11 10:4 11:9 12:30 13:2 14:8 15:4 16:10 17:5 18:19 19:14 20:13 21:14 22:10 23:6 | 16,79 h | 0,250 |

Die laut-Onset-Stunden sind über den ganzen Tag verstreut (alle 24 Bins
besetzt; R = 0,304 — keine Bündelung in einer Phase). Die Kontrolle der
ruhigen Tage zeigt **dieselbe Form** (R = 0,250): Die laut-Tage beginnen zu
denselben Tageszeiten wie alle Boden-Tage — die Tageszeit ist die
**Pass-Tageslage der Station** (wann die Station Galileo sieht), nicht ein
laut-spezifisches Kalibrier-Fenster. Je (Mode, Station) kongruent nur die
stationseigene Pass-Tageslage: Mittel je Serie st14 17–20 h, st43 21–24 h
(Modus 2 st43 23,9 h), st63 10–14 h (R je Serie 0,21–0,69; die Kontrolle
der ruhigen Zellen liegt in denselben Stations-Fenstern, R 0,31–0,95; die
eine scheinbare Ausnahme M2 st43 — ruhig 3,3 h gegen laut 23,9 h — liegt
auf demselben über-Mitternacht-Pass, der die Tagesgrenze kreuzt, n klein,
R ruhig 0,35). Die
laut-Onset-Minuten nach der ersten Floor-Probe des Tages: Median 0 min
(die Lautheit beginnt mit dem Pass), p90 17 min, max 1269 min.

**Verdikt B: keine phasen-kongruente gemeinsame Tageszeit der laut-Episoden.
Die einzige gemessene Kongruenz ist die stations-eigene Pass-Tageslage —
und die teilen die ruhigen Tage.**

## Messung C — Periodische Wiederkunft („gibt es sie öfter")

**Inter-Episoden-Intervalle** (Kalendertage zwischen aufeinanderfolgenden
laut-Tagen je Serie, 198 Intervalle über die 9 Serien): Median 3 d,
124/198 ≤ 5 d, 151/198 ≤ 11 d, 34/198 ≥ 40 d, 10/198 ≥ 150 d (die großen
Abstände sind die datenleeren Monate Feb–Mai 96 und Okt 96 — 0 geehrt,
keine ruhigen Werte dazwischen). Keine feste Wiederkehr-Kadenz in den
Intervall-Listen (je Serie im Report); die kurzen Intervalle (1–11 d)
stammen aus den Bursts der beobachteten Fenster.

**Lomb-Scargle** auf der log10(Tages-RMS)-Reihe je (Mode, Station) über die
robusten Tage mit normiertem Maß (Varianz-Erklärung in [0,1]) und
**Raster-Kontrolle**: 200 Permutationen derselben Werte auf demselben
Tag-Gitter; nur Peaks über dem Surrogat-p95 zählen. Beste Periode je Serie
(in Tagen), Varianz-Erklärung, Surrogat-p95/p100, überlebt?

| Serie | beste Periode (d) | Var-Erkl. | p95/p100 | überlebt |
|---|---|---|---|---|
| M1 st14 | 7,53 | 0,171 | 0,266/0,359 | nein |
| M1 st43 | 6,40 | 0,184 | 0,277/0,312 | nein |
| M1 st63 | 30,08 | 0,245 | 0,265/0,352 | nein |
| M2 st14 | 19,90 | 0,286 | 0,457/0,639 | nein |
| M2 st43 | 7,47 | 0,401 | 0,442/0,555 | nein |
| M2 st63 | 48,94 | 0,427 | 0,415/0,495 | ja (knapp über p95, unter p100) |
| M3 st14 | 2,54 | 0,249 | 0,396/0,447 | nein |
| M3 st43 | 33,19 | 0,449 | 0,490/0,541 | nein |
| M3 st63 | 3,30 | 0,659 | 0,706/0,822 | nein |

Acht von neun Serien überleben die Raster-Kontrolle nicht. Die eine
Ausnahme (M2 st63, 48,9 d) liegt knapp über dem p95 (0,427 gegen 0,415) und
unter dem p100 (0,495) — eine einzelne Serie, ein einzelner Peak, knapp
über der Permutations-Schwelle: als dominante physikalische Kadenz **nicht
getragen** (0 honored; die 48,9 d liegen in der Größenordnung der
Fenster-Abstände der Beobachtung, nicht einer unabhängig gemessenen
Steuer-Kadenz).

**Epoch-Faltung** auf die Kandidaten-Periode: Die laut-Tage der einzigen
Überlebenden (M2 st63, 48,9 d) gefaltet zeigen keine zusätzliche
Phasen-Bündelung über die Pass-Tageslage hinaus (R ~0,3 auf der
Tages-Achse, siehe Messung B). Für die übrigen acht Serien existiert keine
überlebende Kandidaten-Periode, auf die gefaltet werden könnte (0 geehrt).

**Verdikt C: keine dominante, die Raster-Kontrolle überstehende
Wiederkehr-Periode der laut-Episoden; die Intervalle sind Burst- und
Datenlücken-strukturiert, nicht kadenz-fest.**

## Messung D — wiederkehrende Muster innerhalb der laut-Episoden

- **Dauern:** 137 laut-Runs (Episoden = zusammenhängende laut-Tage) über
  alle 9 Serien; Längenverteilung 1 d: 91, 2 d: 27, 3 d: 15, 4 d: 3, 5 d: 1
  → 133/137 (97 %) dauern 1–3 d (reproduziert die Stufen-Zählung „133 von
  137 lauten Episoden 1–3 Tage").
- **Bursts:** laut-Run-Onset-Abstände über Serien (Kalendertage zwischen
  Run-Anfängen): n 136, Median 1 d, max 178 d — die Run-Anfänge folgen
  einander oft tagesweise innerhalb der Fenster; die großen Abstände sind
  die datenleeren Monate.
- **Stations-Reihenfolge (14→43→63?):** Übergänge der lautesten Station von
  Tag zu Tag (je Mode, nur Tage mit genau einer laut-Zelle): M1 n 36, M2
  n 22, M3 n 21. Gemessene Übergänge (je Mode): M1 14→14:0, 14→43:1,
  14→63:7, 43→14:5, 43→43:6, 43→63:4, 63→14:1, 63→43:6, 63→63:6; M2
  14→14:1, 14→43:3, 14→63:2, 43→14:4, 43→43:0, 43→63:3, 63→14:2, 63→43:2,
  63→63:5; M3 14→14:5, 14→43:4, 14→63:3, 43→14:2, 43→43:0, 43→63:2,
  63→14:3, 63→43:1, 63→63:1. **Kein fester 14→43→63-Zyklus**; die
  Übergänge sind diffus (Selbst-Übergänge und beide Richtungen besetzt, n
  klein).

**Verdikt D: 1–3-Tage-Bursts ohne feste Wiederkehr-Kadenz und ohne feste
Stations-Reihenfolge; die einzige wiederkehrende Struktur ist der ruhige
Sockel zwischen den Bursts.**

## Messung E — die Pioneer-Methode ausdrücklich auf Galileo

Die im Schritt 0 benannte Pioneer-Disziplin (LS + normierte Kontrolle +
Epoch-Faltung + Kadenz + periodisch-vs-säkular) ist auf die Galileo-Reihen
angewendet (Messungen A–D). Ergebnis gegen die drei Ausgänge des Operators:
(a) „öfter/periodisch" — **nein, gemessen**: keine Serie überlebt die
Raster-Kontrolle mit tragfähigem Peak (eine knappe Einzel-Ausnahme M2 st63,
48,9 d, unter p100); die Intervall-Struktur ist Burst + Datenlücke. (b)
„deckungsgleich/phasen-kongruent" — **nein, gemessen**: die laut-Onset-
Stunden verstreuen über alle 24 h (R 0,304) und sind von der ruhigen
Kontrolle (R 0,250) in der Form nicht unterscheidbar; die einzige
Kongruenz ist die stationseigene Pass-Tageslage. (c) „innerhalb
wiederkehrende Muster" — nur Dauern 1–3 d und Bursts; keine
Stations-Rotation. Ein **getaktetes Steuer-/Kalibrier-/Update-Signal als
Sinuskurven-artige Ursache ist damit auf der Sub-Tages-Achse widerlegt** —
die gemessene Regelmäßigkeit ist die Pass-/Beobachtungs-Kadenz der
Stationen (Abtast-/Reduktions-Kadenz ≠ physikalische Periode, die
60-s-/Kamm-Lektion auf Tages-Skala).

## Grenzen

- Der Sample-Lautheits-Grenzwert 1 Hz (|resid − Zellen-Mittel|) ist eine
  genannte Lese-Schwelle, konsistent zur 1-Hz-Tagesschwelle; die 66
  laut-Tage mit < 10 % lauten Proben zeigen, dass die Tages-Lautheit teils
  von wenigen Samples getragen wird.
- Die Pass-Segmentierung (600-s-Lücke) zerlegt die Floor-Proben eines
  Tages; Passes über die TDB-Tagesgrenze erscheinen als Läufe in beiden
  Tages-Zellen (Mittags-Konvention der Zell-Definition, benannt).
- Die Tageszeit-Faltung mischt die drei Stations-Longituden; die
  stations-eigene Tageslage ist deshalb erwartbar (die Kontrolle der
  ruhigen Tage trägt dieselbe Lage). Eine gemeinsame UTC-Phase wäre nur als
  Abweichung von dieser Kontrolle sichtbar — gemessen: keine.
- Lomb-Scargle über die ganze Ära ist durch die datenleeren Monate
  begrenzt; Perioden länger als die Fenster-Abstände sind nicht auflösbar
  (0 geehrt). Die 48,9-d-Kandidatin von M2 st63 liegt in dieser
  Fenster-Größenordnung.
- Die „105" des Auftrags ist die Flip-Zählung der Register; die
  laut-Tag-Zellen messen 207. Beide reproduziert; alle Sub-Tages-Analysen
  mit n = 207 geführt.

## Register-Satz

*Die Sub-Tages-Achse der station-gebundenen Galileo-Floor-Gipfel ist
gemessen: kein getaktetes Steuer-/Kalibrier-/Update-Signal getragen. Die
207 laut-Tag-Zellen (robuste Serie; die Register-„105" ist die Flip-Zählung,
reproduziert 105) zeigen keine phasen-kongruente Tageszeit (laut-Onset-
Stunden über alle 24 h verstreut, Rayleigh-R 0,304, gegen 0,250 der ruhigen
Kontrolle — die Kongruenz ist die stationseigene Pass-Tageslage, keine
laut-spezifische Uhr), keine dominante Wiederkehr-Periode über der
Raster-Kontrolle (8/9 Serien unter dem Surrogat-p95; M2 st63 48,9 d knapp
über p95, unter p100) und keine feste Stations-Reihenfolge (Übergänge
diffus). Die Episoden sind 1–3-Tage-Bursts (133/137) auf ruhigem Sockel,
getrennt durch datenleere Monate; die gemessene Regelmäßigkeit ist die
Pass-/Beobachtungs-Kadenz der Stationen, keine physikalische Periode. Der
Sitz der Gipfel bleibt pending (per-Pass-Verschaltung,
Receiver-Identität galileo_receiver.bin als offener Pfad).*
