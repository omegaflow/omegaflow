<!--
  title: Befund — atdf2ascii-Sender-Widerspruch, Simultanitäts-Test und Intra-Pass-Struktur der lauten Galileo-Floor-Pässe (Richtung M, Deduktion-27): kein Sende-Stations-Feld im TRK-2-25-Record (atdf2ascii 3-Weg-Xmtr = Ramp-Record-Verknüpfung, Kandidat; 2-Weg = Empfangs-Duplikat); 55 simultane Zellen, 50 robust (Mode 1: 16 eine-laut / 2 beide-laut / 32 beide-ruhig; Mode 2/3/4 n≈0); laute Anker-Pässe 7/7 nicht gleichmäßig — end-/front-konzentrierte Lock-Grenz-Transienten, H2 ungestützt
  class: befund
  date: 2026-09-06
  sha256: 6616a9979806ba15a3edadcdd43b27d30231225d1b53a7379c0784cd8e0952e6
  status: draft
  see-also: docs/befund/befund-galileo-dreiweg-sendempfang-floor.md docs/befund/befund-galileo-floor-stufen-te.md docs/befund/befund-galileo-ops-aera-floor.md docs/befund/befund-galileo-receiver-floor-ursache.md docs/befund/befund-galileo-pass-segmentierung.md
-->
# Befund: atdf2ascii-Sender-Widerspruch, Simultanitäts-Test und Intra-Pass-Struktur der lauten Galileo-Floor-Pässe (Richtung M, Deduktion-27)

## Frage & Bindung

Der Deduktion-27-Split (Uplink gegen Downlink als Sitz der station-gebundenen
Floor-Lautheit) steht seit dem Dreiweg-Blatt auf `pending`: der rohe
TRK-2-25-Record trägt die Sende-Station nicht. Dieser Lauf (Richtung M) klärt
drei Mess-Fragen an den vorhandenen Daten und Quellen:

1. **atdf2ascii-Sender-Widerspruch** — gibt das Werkzeug
   `atdf2ascii` (ashokverma24/atdf2ascii) eine Transmitter-Spalte aus, und woher
   stammt ihr Wert: aus einem echten Sende-Stations-Feld des 288-Byte-Records,
   als Duplikat der Empfangsstation im 2-Weg, oder aus einer dritten Quelle?
2. **Simultanitäts-Test** — in welchen tdb-Fenstern nehmen zwei verschiedene
   Stationen (14/43/63) gleichzeitig (gleicher Modus) Boden auf, und sind dort
   beide laut, eine laut eine ruhig, oder beide ruhig?
3. **Intra-Pass-Struktur** — wie ist das resid auf den lauten Anker-Pässen über
   die Passzeit verteilt: gleichmäßig (H2: stationäre Schleifen-Bandbreite),
   am Pass-Anfang geballt (H3: Acquisition/Predict), oder in Bursts/
   Cycle-Slips (H3)?

Bindungen wie die Vorlagen: Boden-Sample = Stärke exakt −2560 (AGC-Klemmwert),
Lock (|resid| > 1000 Hz) vor dem Rauschen getrennt; laut = Zell-RMS ≥ 1 Hz um
den Zellen-Mittelwert; Lauf = Sample-Lauf mit tdb-Lücke ≤ 600 s; robuste Zelle
= n ≥ 30. Dreiweg = ground_mode 3/4 (TRK-2-25 item 13). Die Feldsemantik stammt
aus der lückenlosen Feldtabelle `docs/reference/trk-2-25-atdf.txt`
(Table TRK-2-25-3), die item 10 als einziges Stations-Wort (Station Number,
Bits 165–172) belegt; item 12 (4 Bit) = Data Type mit 6 = Ramp; item 13
= Ground Mode; item 64 = Uplink Frequency Band / Source ID.

**Quellen (heruntergeladen 2026-09-06, sha256):**

- `tdf_unpack.pdf` — „Interpretation and Use of Binary ATDF/TDF Data" (PDS Radio
  Science Documentation Bundle, `dsn_trk-2-25`; Simpson), URL
  `https://pds-geosciences.wustl.edu/radiosciencedocs/urn-nasa-pds-radiosci_documentation/dsn_trk-2-25/tdf_unpack.pdf`,
  sha256 `2a826779dfa96bfe39658711e936eb156e324ce2c1c41cd526c4b93588bcc33a`.
- `atdf2ascii` (Verma et al. 2022, SoftwareX; arXiv 2208.03865), GitHub
  `ashokverma24/atdf2ascii`, Commit `main` `a68bc4d24a3f48d47d378af38a6711d3daa54cb5`,
  Quelldateien `src/atdf2ascii.py`, `src/Doppler.py`, `src/Reader.py`,
  `src/Ramp.py` (sha256 je Datei im Report, s. Status).

Daten: `data/galileo_resid.bin` (GASR, 8 × f64: tdb, resid_hz, station, mode,
data_type, doppler_ref, sampler, strength) für die Tests 2 und 3; die vier
Roh-TDF-Caches `data/pds-ppi.igpp.ucla.edu/galileo_tdf_cache_5327328A.TDF`,
`5337339A.TDF`, `5340341A.TDF`, `6177179A.TDF` für den Sender-Zensus. Neue
Proben: `tools/measure/src/bin/galileo_ded27_tdf_sender.rs`,
`galileo_ded27_simultan.rs`, `galileo_ded27_intrapass.rs` (je `cargo check -p
omegaflow-measure --bin <name>`, RUSTFLAGS `-D warnings`, 0/0). Reports:
`/tmp/opencode/ded27_tdf_sender.txt`, `ded27_simultan.txt`, `ded27_intrapass.txt`.

## n zuerst (0 geehrt)

- Sender-Zensus: **83 614 Dreiweg-Doppler-Samples** aus den vier Caches
  (5327328A 19 608, 5337339A 41 914, 5340341A 14 456, 6177179A 7 636);
  Ramp-Records (data_type 6) 68 / 52 / 41 / 17 je Datei.
- Simultanität: **4 647 387** Floor-Trio-Boden-Samples (gesamte Ära im
  resid.bin); **55 simultane Zellen** (Mode 1: 54, Mode 2: 1, Mode 3: 0,
  Mode 4: 0), davon **50 robust** (nA ≥ 30 und nB ≥ 30, alle Mode 1).
- Intra-Pass: 7 laute Anker-Pässe (Tag-Zellen der Referenz exakt reproduziert:
  n und RMS deckungsgleich, s. u.), je ein lauter Lauf je Anker-Tag (n 4292…
  24 201).

## Messung 1 — atdf2ascii und die Sende-Station des TRK-2-25

### Was atdf2ascii ausgibt (Quellcode, gemessen)

Die .msr-Ausgabe trägt die Spalten `Xmtr` und `Rcvr` (Kopfzeile in
`src/Doppler.py`). Der `Rcvr` ist immer die Stations-Nummer des Records
(item 10). Der `Xmtr` wird je data_type gesetzt (`get_xmtr`, `src/Doppler.py`
Zeilen 105–127):

- **1-Weg** (data_type 1): `Xmtr = "S/C"` — der Sender ist das Raumfahrzeug.
- **2-Weg** (data_type 2): `Xmtr = stn` — die **Empfangsstation wird
  dupliziert**. Das ist geometrisch korrekt: im 2-Weg sendet und empfängt
  dieselbe Station. Es ist keine zweite Feld-Lesung.
- **3-Weg** (data_type 3): der Xmtr wird **nicht** dupliziert und nicht aus dem
  288-Byte-Record gelesen, sondern aus den **Ramp-Records derselben Datei**
  verknüpft: `read_ramp` (dtype 6, ground_mode 0) liefert je (Start, Ende,
  Station, Band); getroffen wird ein Ramp-Intervall mit
  `start ≤ t < end`, gleichem Uplink-Band und `ramp_station ≠ Empfangsstation`.
  Ohne Treffer gibt `get_xmtr` None zurück und der 3-Weg-Record wird nicht
  geschrieben.

Der Verdacht „2-Weg-Duplikation, im 3-Weg falsch" ist damit am Quellcode
präzisiert: die 2-Weg-Duplikation existiert (korrekt per Definition); im
3-Weg ist die Quelle eine Ramp-Record-Verknüpfung, keine Duplikation — und
kein Feld des Doppler-Records.

### Trägt der Record eine Sende-Station? (Feldtabelle + Roh-Caches, gemessen)

Die Feldtabelle TRK-2-25-3 (`docs/reference/trk-2-25-atdf.txt`) belegt item 10
als **einziges Stations-Wort** des 288-Byte-Records. Über alle 83 614
Dreiweg-Samples der vier Caches ist der Zensus über die Identitäts-Items
konstant und stations-los; ein zweiter stations-wert (11..99, ungleich der
item-10-Station) kommt in **keinem** Feld vor (kein Feld trägt einen zweiten
Stations-Wert — konsistent zum Dreiweg-Befund). Die Ramp-Records der Caches
tragen ihrerseits item 10 (68/52/41/17 je Datei, ground_mode durchgehend 0,
Uplink-Band-Code durchgehend 1); ihre Stations-Wörter verteilen sich auf alle
im File präsenten Stationen (14/43/63).

### Die Ramp-Verknüpfung auf den Caches (gemessen)

Für die 3-Weg-Samples ist eine „aktive" Ramp (anderes Stations-Wort, gleiches
Band, Zeit ≥ Ramp-Start) fast überall vorhanden: 19 606/19 608, 41 912/41 914,
14 456/14 456, 7 636/7 636 Samples. Die **Kandidaten-Zahl je Sample** ist aber
uneinheitlich gemessen: in 5327328A tragen 13 908/19 608 Samples **zwei**
andere Stations-Kandidaten, 5 698 tragen einen; in 5337339A 35 529/41 914 mit
zwei; in 5340341A und 6177179A tragen alle Samples genau einen. atdf2ascii
nähme in den zwei-deutigen Files den ersten Treffer in Datei-Reihenfolge — der
Wert hinge dann von der Record-Ordnung ab, nicht von der Physik. Zusätzlich
zeigt die Empirie, dass auch die empfangende Station selbst über ihrem
3-Weg-Block aktive Ramp-Records führt (Bedingung „Ramp aktiv" ist also eine
notwendige, keine hinreichende Sende-Bedingung; die Empfangsstation hält einen
programmierten Ramp-Zustand, der kein Senden belegt).

**Verdikt Messung 1: der TRK-2-25-Record trägt die Sende-Station als Feld
nicht — item 10 ist das einzige Stations-Wort, gemessen über 83 614
Dreiweg-Samples. atdf2ascii gibt eine Xmtr-Spalte aus, deren 3-Weg-Wert aus
einer Ramp-Record-Verknüpfung derselben Datei stammt (Kandidat, in zwei der
vier Caches zwei-deutig) und deren 2-Weg-Wert die Empfangsstation dupliziert
(geometrisch korrekt). Unser Parser ist auf dieser Achse nicht unvollständig:
ein per-Sample-Sende-Stations-Feld existiert im Record nicht; die Felder, die
eine Ramp-Verknüpfung bräuchte (item 12/13/64, RAMP-Rate) stehen bereits im
TKFORM-Katalog. Ded-27 ist damit aus dem Record **nicht** datenintern schließbar
— der Kandidat der Ramp-Verknüpfung ist eine notwendige Bedingung, keine
gemessene Identität.**

## Messung 2 — Simultanitäts-Test (Ded-27)

### Simultane Zellen (gemessen)

Über die gesamte Boden-Ära: **55 simultane Zellen** (tdb-Überlappung zweier
Läufe verschiedener Stationen, gleicher Modus). Verteilung: Mode 1 → 54
Zellen, Mode 2 → 1 Zelle (1996-12-23 st14×st43, n 7 je Station — nicht
robust), **Mode 3 → 0, Mode 4 → 0** (0 geehrt). Die 50 robusten Zellen liegen
damit alle in Mode 1.

**Muster der 50 robusten Mode-1-Zellen (nA ≥ 30, nB ≥ 30):**

| Muster | n | Zellen |
|---|---|---|
| beide laut | 2 | 1996-11-06 st14×st43 (2,5/70,0 Hz), 1997-02-27 st14×st63 (15,3/97,5 Hz) |
| eine laut, eine ruhig | 16 | 7× A-laut-B-ruhig, 9× A-ruhig-B-laut (st14/st43/st63-Mischung; laut-Seite je 1,1–97 Hz) |
| beide ruhig | 32 | RMS je Seite 0,01–0,9 Hz |

Mode 1 ist One-Way-Doppler: die beiden Stationen empfangen denselben
Raumfahrzeug-Downlink, ein Boden-Uplink ist im Signalweg nicht beteiligt. Die
16 asymmetrischen Zellen (eine laut, eine ruhig auf demselben Signal)
lokalisieren die Lautheit an der lauten Station (Empfangsort); die 2
beide-laut-Zellen sind der gemeinsame-Ursache-Kandidat (Signalpfad/Sender).
Ein Muster „beide laut bei einer gemeinsamen Uplink/Rampe" kann in Mode 1
nicht entstehen (kein Uplink) und in den Uplink-tragenden Modi 2/3/4 ist die
Simultanität n = 0.

### Der Anker 1995-11-24, passgenau (gemessen)

Der Anker des GLM („st14 laut, st43/st63 ruhig") ist **kein
Simultan-Zeuge**: der laute M1-st14-Lauf des Tages läuft 16:47:31–18:55:35 UTC
(n 6106, Lauf-RMS 28,6 Hz); die einzige simultane Mode-1-Zelle des Tages
st14×st43 (21:14:53–21:37:11 UTC, n 1338/1338) ist **beide-ruhig**
(0,12/0,15 Hz). st43/st63 führen an diesem Tag also außerhalb des lauten
st14-Fensters Boden. Die laut/ruhig-Trennung des Ankers ist zeitlich, nicht
simultan.

**Verdikt Messung 2: simultane Boden-Aufnahme existiert über die Ära fast nur
in Mode 1 (50 robuste Zellen), und dort überwiegt die Asymmetrie (16 eine-laut
gegen 2 beide-laut, 32 beide-ruhig) — die Lautheit sitzt in diesen Zellen an
der lauten Empfangsstation, nicht im gemeinsamen Signal. In den
Uplink-tragenden Modi 2/3/4, in denen Ded-27 zwischen Sende- und
Empfangsseite trennen müsste, ist die Simultanität n = 0 (Mode 2 nur eine
n = 7-Zelle); der Sender/Empfänger-Split ist über Simultanität dort nicht
entscheidbar. Der Anker 1995-11-24 trägt als passgenau gezählter Fall die
Empfangsseiten-Lesart nicht (beide-ruhig im Überlapp, Lautheit zeitlich
getrennt).**

## Messung 3 — Intra-Pass-Struktur der lauten Pässe

Sieben laute Anker-Pässe (Mode, Station, Tag); die Referenz-Tag-Zellen sind
exakt reproduziert (n/RMS): M1 st14 1995-11-24 n 7463 / 25,8492 Hz; M2 st14
1995-11-24 n 6487 / 31,7698 Hz; M3 st14 1995-12-05 n 6467 / 52,92 Hz; M3 st63
1995-11-27 n 4292 / 186,47 Hz; M1 st63 1996-06-26 n 24 201 / 20,64 Hz; M1 st43
1996-11-04 n 7333 / 23,14 Hz; M3 st43 1995-12-04 n 14 895 / 10,52 Hz. Je Anker
ein lauter Lauf (n ≥ 30, Lauf-RMS ≥ 1 Hz). Struktur über 12 gleiche
Zeitfenster (Fenster-RMS um den Fenster-Mittelwert; Lauf-RMS um den
Lauf-Mittelwert):

| Anker (Mode st Tag) | Lauf (UTC) | Fenster-Muster (internes RMS, Hz) | |resid|-Verteilung | Struktur |
|---|---|---|---|---|
| M1 st14 1995-11-24 | 16:47–18:55 | Fenster 0–2 und 9–11 ruhig (0,05–3,1); Fenster 3–8 laut (34–58) | p50 1,5, p90 4,2, p99 37, max 986; >1 Hz 4119/6106 | mittlere Passhälfte als Lärmband, Flanken ruhig |
| M2 st14 1995-11-24 | 19:02–20:13 | Fenster 2–3 laut (bis 215), danach ruhig (0,1–7) mit ansteigender Mittellage | p50 11,0; >100 Hz 62 | Anfangslast (front20 0,78) |
| M3 st14 1995-12-05 | 16:23–18:11 | Fenster 0–10 ruhig (0,043–0,054); Fenster 11 laut (171, Mittelwert −52,8) | p50 0,044; >100 Hz 61/6466 | End-Fenster-Transient |
| M3 st63 1995-11-27 | 09:41–10:59 | Fenster 1, 3, 6, 7 laut (28/38/172/412, Fenster 7 mit +470-Offset) | p50 0,73; >100 Hz 230/4292 | mehrere getrennte Bursts |
| M1 st63 1996-06-26 | 21:40–04:26 | Fenster 0–10 ruhig (0,01–0,07 bei Mittel 0,21); Fenster 11 laut (67, Mittel +7,6) | p50 0,21, p99 0,31; >100 Hz 26/24 201, max 993 | End-Fenster-Transient |
| M1 st43 1996-11-04 | 05:05–06:35 | Fenster 0–10 ruhig (0,017–0,043); Fenster 11 laut (82, Mittel +11,6) | p50 0,17; >100 Hz 11/5357 | End-Fenster-Transient |
| M3 st43 1995-12-04 | 23:05–01:31 | alle Fenster ruhig (intern ≤ 0,06); Tages-RMS 10,5 Hz stammt aus 2 Samples am Laufende (~±907 Hz) | p50 0,067, p99 0,18 | Laufende-Transient (2 Samples) |

**Verdikt Messung 3: in keinem der 7 lauten Anker-Pässe ist die Lautheit
gleichmäßig über den Pass verteilt (H2 ungestützt, n = 0 gleichmäßige Pässe).
Das dominante Bild sind zeitlich konzentrierte Transienten nahe der
Lock-Grenze (|resid| bis ~990 Hz bei einer Lock-Schwelle von 1000 Hz):
5 der 7 Pässe tragen ihre Lautheit im letzten Zeitfenster oder am Laufende,
1 Pass ist Anfang-lastig (M2 st14, front20 0,78), 1 Pass trägt mehrere
getrennte Burst-Fenster (M3 st63), 1 Pass ein mittleres Lärmband (M1 st14).
Die hohen Tages-RMS der Referenz stammen überwiegend aus wenigen extremen
Samples (z. B. M1 st63 1996-06-26: 26 Samples > 100 Hz unter 24 201 ruhigen;
M3 st43 1995-12-04: 2 Samples) — das Muster ist Cycle-Slip-/Verriegelungs-
Transient (H3-Familie), nicht stationäre Schleifen-Bandbreite.**

## Verdikt

**Teil 1 (gemessen):** der TRK-2-25-Record trägt die Sende-Station nicht.
atdf2ascii gibt die Xmtr-Spalte aus, aber ihr 3-Weg-Wert ist eine
Ramp-Record-Verknüpfung derselben Datei (in zwei der vier Caches zwei-deutig),
ihr 2-Weg-Wert eine Duplikation der Empfangsstation; der 288-Byte-Doppler-
Record hat kein Sende-Stations-Feld (item 10 einzige Stations-Wort über
83 614 Dreiweg-Samples). Ded-27 ist aus dem Record nicht datenintern schließbar.

**Teil 2 (gemessen):** 55 simultane Zellen (50 robust, alle Mode 1): 16
eine-laut, 2 beide-laut, 32 beide-ruhig. In den Uplink-tragenden Modi 2/3/4
ist die Simultanität n = 0 — der Ded-27-Split ist über Simultanität auf den
Dreiweg-Daten nicht entscheidbar. Der Anker 1995-11-24 ist kein Simultan-Zeuge
(der einzige Überlapp des Tages ist beide-ruhig; die Lautheit liegt zeitlich
getrennt).

**Teil 3 (gemessen):** die lauten Anker-Pässe (7/7) sind nicht gleichmäßig
laut; das dominante Muster sind end-/front-konzentrierte Transienten nahe der
Lock-Grenze und mehrere Burst-Fenster — H3-Familie, H2 ungestützt (n = 0).

**Was `pending` bleibt (kein Ersatz, keine Schätzung):** die Sender-Identität
der lauten Dreiweg-Zellen (braucht den DSN-Tracking-/Pass-Plan oder ein
ODF/TRK-2-18-Metadatum mit Sender-Angabe); die Unterscheidung, ob die
gemessenen Lock-Grenz-Transienten Cycle-Slips der Empfangsstation oder
Predict-/Rampen-Sprünge der Sendeseite sind; die Ursache der 2
beide-laut-Mode-1-Zellen (gemeinsamer Signalpfad).

## Grenzen

- Der Sender-Zensus und die Ramp-Verknüpfung sind über die vier gezielten
  Roh-Caches geführt (83 614 Dreiweg-Samples, die Stichprobe des
  Dreiweg-Blatts); die Simultanitäts- und Intra-Pass-Messungen über das ganze
  `galileo_resid.bin` (4 647 387 Boden-Trio-Samples). Die Feld-Semantik stammt
  aus `docs/reference/trk-2-25-atdf.txt` und der atdf2ascii-Quelle; der
  empirische Scan bestätigt sie, ersetzt sie nicht.
- Die Ramp-Verknüpfung ist hier ohne die Band-/Intervall-Treue von atdf2ascii
  nachgebaut (Zeit ≥ erster Ramp-Start der Station im Band); die
  Kandidaten-Zahlen sind damit eine obere Schranke der atdf2ascii-Treffer.
  Die 3-Weg-Samples tragen durchgehend Uplink-Band-Code 1; die
  zwei-deutigen Kandidaten-Fälle (2 je Sample) sind als Nicht-Eindeutigkeit
  benannt, nicht als Messung eines Senders.
- atdf2ascii klassiert nur ground_mode 3 als 3-Weg (ground_mode 4, drei-Weg
  kohärent, fällt in dessen Sortierung heraus) — benannt, nicht Teil dieser
  Messung; die drei-Weg-Zählung hier folgt der Referenz (Mode 3 und 4).
- Die Intra-Pass-Fenster-Analyse nutzt Fenster-RMS um den Fenster-Mittelwert;
  da einzelne extreme Samples (nahe der 1000-Hz-Lock-Schwelle) ein
  Fenster-RMS dominieren können, sind die Verteilungs-Quantile und die
  >1/10/100-Hz-Zählungen je Lauf mitgeführt (genannt, nicht geglättet).
- Die Tag-Zell-Grenzen folgen der Referenz-Konvention (Mittags-zu-Mittag-
  Fenster, `floor(tdb/86400)`, Offset exakt reproduziert: die Anker-Zell-n und
  -RMS decken sich mit der Faden-Serie auf alle ausgegebenen Dezimalen).

## Register-Satz

*Der atdf2ascii-Sender-Widerspruch ist am Quellcode und an den Roh-Caches
geklärt (Richtung M): atdf2ascii gibt eine Xmtr-Spalte aus, deren 3-Weg-Wert
aus einer Ramp-Record-Verknüpfung derselben Datei stammt (get_xmtr in
src/Doppler.py; Ramp = data_type 6, ground_mode 0; Bedingung xmtr-Station ≠
Empfangsstation) und deren 2-Weg-Wert die Empfangsstation dupliziert; der
288-Byte-TRK-2-25-Record trägt über 83 614 Dreiweg-Samples aus vier Caches
kein Sende-Stations-Feld (item 10 einzige Stations-Wort, station-like 11..99
in keinem Identitäts-Item), die Ramp-Kandidaten je 3-Weg-Sample sind in
5327328A/5337339A mehrheitlich zwei-deutig (13 908/19 608 bzw. 35 529/41 914
Samples mit 2 Kandidaten), in 5340341A/6177179A eindeutig (14 456/14 456 bzw.
7 636/7 636 Samples mit 1 Kandidat) — Ded-27 bleibt aus dem Record datenintern
ungeschlossen. Der Simultanitäts-Test über das ganze resid.bin findet 55
simultane Zellen (50 robust, alle Mode 1: 16 eine-laut, 2 beide-laut, 32
beide-ruhig; Mode 2 eine n = 7-Zelle, Mode 3/4 n = 0); der Anker 1995-11-24 ist
kein Simultan-Zeuge (einziger Überlapp des Tages st14×st43 21:14–21:37 beide
ruhig, lauter M1-st14-Lauf 16:47–18:55 ohne Gegen-Boden) — der Uplink/Downlink-
Split ist über Simultanität auf den Uplink-tragenden Modi 2/3/4 nicht
entscheidbar (n = 0 geehrt). Die Intra-Pass-Struktur der 7 lauten
Anker-Pässe (Tag-Zellen exakt reproduziert) ist in 7/7 nicht gleichmäßig:
5 Pässe tragen ihre Lautheit end-konzentriert (M3 st14 1995-12-05, M1 st63
1996-06-26, M1 st43 1996-11-04, M3 st43 1995-12-04 mit nur 2 Samples am
Laufende, dazu M2 st14 front20 0,78), M3 st63 1995-11-27 mehrere getrennte
Burst-Fenster (bis 412 Hz), M1 st14 1995-11-24 ein mittleres Lärmband — das
Muster sind Lock-Grenz-Transienten/Cycle-Slips (H3-Familie), H2 (gleichmäßige
Schleifen-Bandbreite) ungestützt; pending bleibt die Sender-Identität der
lauten Dreiweg-Zellen (Pass-Plan/ODF-Metadatum) und die Trennung von
Empfangs-Cycle-Slips gegen Sende-Predict-/Rampen-Sprünge.*

## Status

`draft` (Entwurf für die Haupt-Session/Rat; TODO-Registerzeile ergänzt die
Haupt-Session). Proben `galileo_ded27_tdf_sender`, `galileo_ded27_simultan`,
`galileo_ded27_intrapass` committet (`cargo check` je Bin, RUSTFLAGS
`-D warnings`, 0/0), Reports `/tmp/opencode/ded27_tdf_sender.txt`,
`/tmp/opencode/ded27_simultan.txt`, `/tmp/opencode/ded27_intrapass.txt`.
Referenz-Sha256: tdf_unpack.pdf `2a826779…`, atdf2ascii Commit `a68bc4d`.
`src/archivar/atdf.rs` wurde nicht erweitert (die geprüften Items stehen im
Katalog; es fehlt kein Sende-Stations-Feld).
