<!--
  title: Befund — Galileo-laut-Tage gegen Pioneer-10-Boden an derselben DSN-Station (Richtung K): null Koinzidenz-Zellen im Fenster 1995-11-23..1997-02-28, die Tag-Überlappung ist leer bis auf eine gemeinsame robuste Zelle, die gegenläufig liest — der Bodenstation-Sitz bleibt ohne Zwei-Sonden-Beleg
  class: befund
  date: 2026-09-06
  sha256: cf034bdc7accdba8cd306c4386834c64f81e569b621bdf19f5246ea2c37bfeab
  status: draft
  antwortet-auf: docs/befund/befund-galileo-floor-stufen-te.md docs/befund/befund-galileo-ops-aera-floor.md
  see-also: docs/auftrag/archiv/auftrag-subhz-drift-quiet-zone.md docs/befund/befund-pioneer-aera-solarwind-omni2.md
-->

# Befund: Galileo-Floor-Laut-Tage gegen Pioneer-10-Tages-Boden an derselben DSN-Bodenstation — die tagesscharfe Koinzidenz ist leer (n = 0 laut-beide), eine einzige gemeinsame robuste Zelle liest gegenläufig

## Frage & Bindung

Die station-gebundene Galileo-Floor-Lautheit (AGC-Klemmwert −2560, Zell-RMS
je (Modus, Tag, Station), laut ≥ 1 Hz) ist in den Vorlagen als ein
(Station, Tag)-Zustand auf Pass-Zeitskala gemessen; ihr Sitz bleibt `pending`
(Receiver-Befund: kodierte Receiver-Identität trägt sie nicht; Ops-Ära-Befund:
die kalendarischen Betriebswechsel tragen sie nicht als Stufen;
Stufen-TE-Befund: isolierte 1–3-Tage-Gipfel, keine gerichtete Kopplung).
Dieser Lauf (Richtung K) stellt den entscheidenden Stations-Beleg mit einer
**zweiten, unabhängigen Sonde**: fallen die tagesscharfen Floor-Laut-Tage
zweier unabhängiger Sonden an derselben DSN-Bodenstation zusammen, so ist der
Floor-Sitz die Bodenstation selbst — und zwar die gemessene. Konkret werden
Pioneer 10/11 (S-Band, >50 AU, Quiet-Zone-Ära) gegen Galileo (S-Band,
Jupiter-Orbit) im gemeinsamen Zeitfenster **1995-11-23 .. 1997-02-28**
verglichen, je (Station, Tag), auf den 70-m-Stationen DSS-14/43/63.

Gebunden an die Referenz-Metrik der Vorlagen: Galileo-Tages-Zelle =
(Mode, Station, TDB-Tag), Boden = Stärke exakt −2560, Lock (|resid| > 1000 Hz)
vor dem Rauschen getrennt, laut = Zell-RMS ≥ 1 Hz, robuste Zellen n ≥ 30
(dünne Zellen ausgewiesen, nicht klassiert). Pioneer-Tages-Zelle =
(Station, TDB-Tag) auf den negative-fuzzy-Tagesmedian-Feldern der
Quiet-Zone-Pipeline (per-Pass-Quadrat-Detrend, Gap 900 s, Block ≥ 8;
per-Station-Median-Abzug; Lock |resid| > 1000 Hz getrennt; die 566 339 von
856 602 P10-Proben mit |resid| > 1000 Hz bleiben ausgeschlossen); ruhiger
Boden = |Tagesmedian| ≤ 5 Hz (Ded-45-Quiet-Band), lauter Boden = isolierte
Tages-Erhöhung über der Band (|Tagesmedian| > 5 Hz). Probe
`tools/measure/src/bin/galileo_pioneer_stationsfloor_kreuz.rs` (neu, einzige
Repo-Änderung außer diesem Blatt; `cargo check` 0/0, RUSTFLAGS `-D warnings`),
Report `/tmp/opencode/galileo_pioneer_stationsfloor_kreuz_report.txt`. Daten:
`data/galileo_resid.bin` (GASR), `data/spdf.gsfc.nasa.gov/pioneer10_navio_residuum.bin`
und `pioneer11_navio_residuum.bin` (P11R, Station je Sample = Feld [5], Mode =
Feld [7]).

## n zuerst (0 geehrt) — Abdeckung und Stationsführung

**Pioneer-NAVIO-Residuum im Fenster (S1):** Pioneer 10 führt das Fenster an
den 70-m-Stationen (Tages-Zellen: st14 18, st43 23, st63 23 = 64 Zellen);
Pioneer 11 endet 1990-10-05 (Residuum 1973-04-10 .. 1990-10-05) und trägt
**n = 0** Proben im Fenster — die Kontrolle ist im Fenster nicht vorhanden
(0 geehrt), der Kreuz-Vergleich läuft auf Pioneer 10. Das Residuum führt die
Empfangs-Station je Sample (Feld [5]); die P10-Missions-Spuren an den
Trio-Stationen: st14 77 221 Proben (730 Tage), st43 81 625 (920 Tage),
st63 122 785 (1028 Tage) über 1973-10-05 .. 2002-03-03. Die Galileo-Boden-
Ära (Floor-Zellen an den Trio-Stationen, Modus 1–3) liegt vollständig im
Fenster (der älteste Trio-Boden-Tag ist 1995-11-23/24).

**Galileo-Reproduktion im Fenster (S2) — exakt die Referenz-Zahlen:** je
(Station, Mode) Floor-Zellen/laut (Zell-RMS ≥ 1 Hz): Modus 1 st14 82/42,
st43 84/45, st63 98/55; Modus 2 st14 39/17, st43 35/16, st63 50/27; Modus 3
st14 45/28, st43 35/18, st63 22/10. Die robusten (n ≥ 30) laut-Tag-Zellen je
Station (Tag-Zellen über Moden dedupliziert): st14 54, st43 52, st63 48 —
**154 laut (Station, Tag)-Zellen** im Fenster. Die Anker reproduzieren exakt:
1995-11-24 st14 m1 25,849 Hz (n 7463), m2 31,770 Hz; 1996-06-26 st63 m1
20,640 Hz (n 24201) laut bei st43 m1 0,021 Hz ruhig; die Dez-1996/Feb-1997-
lauten Zellen an st14/st43/st63 (m1/m2/m3, 4–360 Hz) stehen wie in den
Vorlagen.

**Pioneer-Boden-Zustand an den Trio-Stationen im Fenster (S1b):** von den
64 (Station, Tag)-Zellen sind 59 robust; davon **ruhig 56, laut 3**, dazu
5 dünne Zellen. Die drei laut-Boden-Tage von Pioneer 10 im Fenster sind
isolierte Ein-Tages-Erhöhungen (Nachbar-Zellen derselben Station ruhig):
st43 1996-01-21 (Tagesmedian +122,2 Hz, Tages-RMS 431,3 Hz, n 98),
st63 1996-01-21 (−124,2 Hz, 162,4 Hz, n 188), st63 1996-08-08 (8,6 Hz,
466,9 Hz, n 234). Alle übrigen Pioneer-Zellen lesen den ruhigen Boden
(Tagesmedian −0,19 … +0,28 Hz, Tages-RMS der ruhigen Zellen 0,04–3,6 Hz) — die
Trio-Stationen sind auf allen Pioneer-Tagen des Fensters mit Ausnahme der drei
genannten ruhig, auch an Tagen, die kalendarisch dicht an Galileo-lauten
Zellen derselben Station liegen (z. B. st14: Pioneer-ruhig 1995-11-26 — selbst ein
Galileo-lauter Tag — und Pioneer-ruhig 1996-07-28/29 zwischen den
st14-lauten Läufen 1996-06-26..29 und 1996-09-06..08).

## Messung — die Koinzidenz-Zählung je (Station, Tag)

**Gemeinsame (Station, Tag)-Zellen im Fenster (S3):** nur **6 Zellen** tragen
zugleich eine Galileo-Floor-Zelle (irgendein n) und eine Pioneer-Zelle
(irgendein n):

| (Station, Tag) | Galileo-Floor-Zellen | Pioneer 10 (n, Tagesmedian, Tag-RMS) | Koinzidenz |
|---|---|---|---|
| st14 1995-11-26 | m1 **14,367 Hz laut** (n 2216), m3 **75,023 Hz laut** (n 10846), m2 0,414 Hz ruhig (n 29222) | n 43, **−0,008 Hz ruhig**, 0,139 Hz | nein — gegenläufig |
| st43 1995-11-26 | m2 0,022 Hz ruhig (n 25586), m3 0,023 Hz ruhig (n 7344) | n 21 dünn, −0,004 Hz | nicht klassierbar |
| st43 1995-12-06 | m1 0,067 Hz ruhig (n 23298), m2 0,578 Hz ruhig (n 13091) | n 30, −0,005 Hz ruhig, 0,043 Hz | ruhig–ruhig |
| st63 1996-12-30 | m1 1,257 Hz dünn (n 7) | n 186, 0,069 Hz ruhig, 3,620 Hz | nicht klassierbar |
| st63 1997-01-12 | m1 1,653 Hz dünn (n 7) | n 147, −0,022 Hz ruhig, 0,667 Hz | nicht klassierbar |
| st63 1997-01-14 | m1 0,461 Hz dünn (n 6) | n 130, −0,021 Hz ruhig, 1,727 Hz | nicht klassierbar |

**Koinzidenz laut–laut (beide Sonden, gleiche Station, gleicher Tag):
n = 0.** Es gibt keine (Station, Tag)-Zelle, an der Galileo und Pioneer 10
gleichzeitig einen lauten Boden messen. Die einzige gemeinsame robuste Zelle
mit einer Galileo-lauten Zelle (st14 1995-11-26) liest **gegenläufig**:
Galileo m1 14,4 Hz und m3 75,0 Hz laut, während Pioneer 10 an derselben
Station am selben Tag den ruhigen Boden misst (Tagesmedian −0,008 Hz, n 43) —
und auch Galileo selbst liest dort m2 mit 0,414 Hz ruhig (eine Drei-Wege-
Dissoziation innerhalb desselben Stations-Tags: m1 laut, m3 laut, m2 ruhig,
Pioneer-Boden ruhig).

**Gegenrichtung (S4):** von den 154 Galileo-lauten (Station, Tag)-Zellen des
Fensters trägt genau **1** eine Pioneer-Zelle derselben Station desselben
Tages (st14 1995-11-26, dort Pioneer ruhig); die übrigen 153 haben keine
Pioneer-Tages-Zelle an der Station (0 geehrt — die Track-Überlappung der
beiden Sonden an derselben 70-m-Station ist über das Fenster fast leer: 6
gemeinsame Zellen von 64 Pioneer-Tagen bzw. von 304 Galileo-Floor-Tagen).
Von den 3 Pioneer-lauten (Station, Tag)-Zellen (1996-01-21 st43/st63,
1996-08-08 st63) trägt keine eine Galileo-Floor-Zelle an derselben Station
desselben Tages (Galileo-Boden an st43/st63 im Januar 1996 und an st63 im
August 1996 nicht vorhanden — 0 geehrt, kein Galileo-Zeuge an diesen Tagen).

**Stations-Zustand über die Zeit (S5):** keine Trio-Station zeigt im Fenster
einen laut-Zustand, der über mehrere Tage hinweg probe-übergreifend stabil
wäre. Pioneer 10 liest die Stationen auf allen 56 robusten ruhigen Zellen
ruhig — auch auf Tagen, die kalendarisch dicht an Galileo-lauten Läufen
derselben Station liegen (st14: Pioneer ruhig 1995-11-26 — der gegenläufige
gemeinsame Tag selbst — und Pioneer ruhig 1996-07-28/29, sechs Wochen nach
dem st14-lauten Lauf 1996-06-26..29; st43: Pioneer ruhig 1996-11-11, direkt
nach dem st43-lauten Lauf 1996-11-08..10, und Pioneer ruhig 1995-12-28/29,
zwei Tage vor den st43-lauten Zellen 1995-12-30/31; st63: Pioneer ruhig
1997-01-12/14, zwei bis vier Tage vor den st63-lauten Zellen 1997-01-16/18).
Die drei Pioneer-lauten Tage sind isolierte Ein-Tages-Erhöhungen ohne
Galileo-Gegenstück an der Station (Galileo-Boden dort abwesend, 0 geehrt).

## Verdikt


**Richtung K ist als Zwei-Sonden-Koinzidenz gemessen leer: null (Station,
Tag)-Zellen, an denen beide unabhängigen Sonden gleichzeitig einen lauten
Boden lesen.** Die tagesscharfen Galileo-Floor-Laut-Tage fallen im gemeinsamen
Fenster 1995-11-23..1997-02-28 mit keinem Pioneer-10-lauten Tag derselben
Station zusammen, und umgekehrt. Die Ursache ist zuerst die gemessene Leere
der Track-Überlappung: Pioneer 10 war an den Trio-Stationen auf nur 64 Tagen
des 15-Monats-Fensters (seine Boden-Zustände fast durchweg ruhig), Galileo
trug an denselben Stationen 304 Boden-Tage mit 154 laut-Zellen — die beiden
Tag-Kalender an derselben 70-m-Antenne überlappen auf nur 6 Zellen. Auf der
**einen** gemeinsamen robusten Zelle, die einen Galileo-lauten Tag mit einem
Pioneer-Tag verbindet (st14 1995-11-26), liest der Befund **gegen** den
Station-Sitz: Galileo m1/m3 laut bei Pioneer-gemessenem ruhigem Stations-Boden
und Galileo-m2-ruhig am selben Tag. Ein Beleg, dass der Sitz die Bodenstation
selbst ist, existiert damit auf diesen Daten nicht — weder als Koinzidenz
(n = 0) noch als station-über-die-Zeit-stabiler Zustand (die eine gemeinsame
Zelle ist gegenläufig). Die Leere ist nicht als Beweis gegen einen
Station-Mechanismus zu lesen, der nur in den Galileo-Pass-Zeiten wirkt: Die
Day-Zelle kann Pass-zu-Pass-Zeiten nicht trennen, und Pioneer war an den
Galileo-lauten Tagen fast nie an derselben Station (0 geehrt). Gemessen ist:
ein gemeinsamer Stations-Tag, an dem die Bodenstation ruhig steht, während
Galileo laut liest — die Lautheit dieses Tages ist damit nicht ein Zustand
der Station, sondern der Galileo-Strecke/des Galileo-Passes.

**Was `pending` bleibt:** die Sub-Tages-Achse — ob Galileo laut und Pioneer
ruhig am selben Stations-Tag zeitlich getrennte Pässe waren (die naheliegende
Betriebs-Lesart, hier nicht aufgelöst, weil die Tag-Zelle die Uhrzeit nicht
führt) — und der Galileo-Boden an den Pioneer-lauten Tagen (Jan/Aug 1996
bodenleer, 0 geehrt). Der Sitz der Galileo-Gipfel-Tage bleibt wie in den
Vorlagen `pending`; Richtung K liefert keinen Stations-Beleg, benennt aber die
einzige vorhandene Zwei-Sonden-Messung als gegenläufig.

## Grenzen

- Pioneer 11 trägt im Fenster n = 0 (Residuum endet 1990-10-05) — der
  Kreuz-Vergleich hat nur Pioneer 10 als zweite Sonde.
- Die Tag-Kalender von Galileo und Pioneer 10 an derselben 70-m-Station
  überlappen auf nur 6 von 64 bzw. 304 Tagen; 153 der 154 Galileo-lauten
  Zellen und alle 3 Pioneer-lauten Zellen haben kein Gegenstück am selben
  (Station, Tag) (0 geehrt, keine Erfindung).
- Die Day-Zelle fasst alle Pass-Zeiten eines Kalendertags zusammen; eine
  zeitliche Trennung (Galileo laut am Vormittag, Pioneer ruhig am Nachmittag
  derselben Station) ist auf dieser Achse nicht auflösbar — die eine
  gegenläufige Zelle ist deshalb ein Grenzfall, kein Vollbeweis.
- Pioneer-Metrik: negative-fuzzy-Tagesmedian mit 1000-Hz-Lock (566 339 der
  856 602 P10-Proben ausgeschlossen) und 5-Hz-Quiet-Band; die drei
  Pioneer-lauten Tage stehen auch bei anderer Band-Wahl (3σ-Excursions-Gate
  der Ded-45-Disziplin auf den Tagesmedianen) als isolierte Erhöhungen —
  quantitativ aber band-abhängig, hier nur die 5-Hz-Band gezählt.
- Galileo-Zählung auf robusten Zellen (n ≥ 30); dünne laute Zellen (z. B.
  st63 1996-12-30 .. 1997-01-14 mit n 6–7) sind ausgewiesen, nicht klassiert.

## Register-Satz

*Richtung K (Zwei-Sonden-Stations-Beleg) ist im Fenster 1995-11-23..
1997-02-28 gemessen leer: von den 154 Galileo-lauten (Station, Tag)-Zellen der
Trio-Stationen und den 3 Pioneer-10-lauten (Station, Tag)-Zellen des Fensters
fällt keine zusammen — Koinzidenz laut–laut n = 0; die Track-Überlappung an
derselben 70-m-Station ist fast leer (6 gemeinsame Zellen von 64 Pioneer-Tagen
bzw. 304 Galileo-Boden-Tagen), Pioneer 11 trägt n = 0 im Fenster. Die eine
gemeinsame robuste Zelle mit Galileo-lautem Tag (st14 1995-11-26, m1 14,4 Hz/
m3 75,0 Hz laut) liest gegenläufig: Pioneer 10 misst dort den ruhigen Stations-
Boden (Tagesmedian −0,008 Hz, n 43) und Galileo m2 0,414 Hz ruhig am selben
Tag — der einzige Zwei-Sonden-Stations-Tag des Fensters spricht gegen einen
rein station-weiten Floor, ein Stations-Sitz ist damit nicht bestätigt und der
Sitz der Galileo-Gipfel bleibt pending; die Sub-Tages-Achse (Pass-Zeiten der
beiden Sonden am selben Stations-Tag) ist der offene, ungemessene Pfad.*

## Status

`draft` (Entwurf für die Haupt-Session/Rat; TODO-Registerzeile ergänzt die
Haupt-Session später). Probe `galileo_pioneer_stationsfloor_kreuz` committet
(`cargo check` 0/0, RUSTFLAGS `-D warnings`), Report
`/tmp/opencode/galileo_pioneer_stationsfloor_kreuz_report.txt`. Galileo-
Reproduktion exakt auf den Referenz-Zahlen der Vorlagen (Modus-1/2-laut-Zellen
je Station 42/17/45/16/55/27; Anker 1995-11-24 st14 und 1996-06-26 st63);
Richtung K liefert null Koinzidenz-Zellen und eine gegenläufige
Zwei-Sonden-Zelle — 0 geehrt für die leeren Gegenrichtungen.
