<!--
  title: Befund — die 105 tages-scharfen Floor-Gipfel (Station, Mode, Tag) sind sichtlinien- und himmelskörper-unabhängig: alle 105 Flips wechseln bei <1° Sichtlinien-Schritt, an einem Tag mit identischer Geometrie und Jupiter-Distanz sind Stationen gemischt laut/ruhig; eine Mond-Vorbeiflug-Achse ist offline nicht messbar (kein Asset) — pending
  class: befund
  date: 2026-09-05
  version: 1
  sha256: 58ff837b04c6d91eacfa28cd59cc212380eec7d0e7cf2f4078ad668bff9c7cb1
  status: draft
  antwortet-auf: docs/befund/befund-galileo-floor-stufen-te.md
  see-also: docs/befund/befund-galileo-ops-aera-floor.md docs/befund/befund-galileo-1996-rest-kontrast.md docs/befund/befund-galileo-alpha-zeit-sonnenzyklus.md docs/handover/archiv/handover-2026-09-05-galileo-tiefe-rotor-spin-receiver.md
-->
# Befund: die station-gebundenen Floor-Gipfel gegen Himmels-Position, Erde–Galileo- und Galileo–Jupiter-Distanz

## Frage & Bindung

Das Stufen-TE-Blatt (draft) benennt die 105 robusten tages-scharfen Flips
(n ≥ 30, laut/ruhig) als isolierte 1–3-Tage-Gipfel auf ruhigem Sockel und
lässt deren Sitz als per-Pass-Zustand je (Station, Tag) offen. Dieser Lauf
prüft die zwei verbleibenden Achsen, die die Tages-Struktur tragen könnten:
**Ort** (steht der laute Gipfel-Tag an einer besonderen Stelle der
durchlaufenen Sichtlinie Erde→Galileo im ICRS?) und **Himmelskörper-Nähe**
(steht er an besonderer Distanz zu Erde bzw. Jupiter, nahe Perijove oder an
dokumentierten Trabanten-Vorbeiflügen?). Frage des Operators: ist das
station-gebundene Gipfel-Muster ort- und himmelskörper-unabhängig?

Gebunden wie die Vorlagen: Floor = Stärke exakt −2560; Zelle = (Mode,
Station, TDB-Tag); Lock (|resid| > 1000 Hz) und Nicht-Endliche vor dem
Rauschen getrennt; Zell-RMS um den Zellen-Mittelwert; robuste Serie = Zellen
n ≥ 30; laut = Zell-RMS ≥ 1 Hz. Die Zielgröße ist die (Station, Mode,
Tag)-Zelle des lauten Gipfels auf der robusten Serie. Geometrie je TDB-Tag
aus `ephemeris_galileo_daily` + `ephemeris_earth` + `ephemeris_jupiter`
(ICRS, SSB): **u** = Einheitsvektor (Galileo − Erde) = die Sichtlinien-
Richtung (RA/Dec ICRS); rE = |Galileo − Erde| in AU; J = |Galileo − Jupiter|
in R_J (71 492 km). Die Geometrie ist je Tag für alle Stationen identisch —
jede Station-Mischung an einem Tag hält Ort und Distanzen fest. Die
Ära-Verwebung ist benannt: die Sichtlinie wandert mit der Zeit (Galileo
steht bei Jupiter; die Erde läuft um), die Distanzen mit dem Kalender; die
Zellen werden daher zusätzlich als Kalender-Monate offengelegt, nicht
geglättet. Probe
`tools/measure/src/bin/galileo_floor_sky_body_probe.rs` (neu, einzige
Repo-Änderung außer diesem Blatt; `cargo check` 0/0, RUSTFLAGS `-D
warnings`), Report `/tmp/opencode/galileo_floor_sky_body_report.txt`.

## n zuerst (0 geehrt)

14 077 825 Residuen, 1 994 510 Lock-/Nicht-Endlich-Samples ausgeschlossen;
490 Floor-Trio-Zellen (Stationen 14/43/63, Modi 1–3) über 1995-11-23 ..
1997-02-28; 90 dünn (n 1..29, ausgewiesen, nicht klassiert); 400 robuste
Tages-Zellen (n ≥ 30), davon **207 laut / 193 ruhig**. Geométrie: 464
Ära-Tage voll aufgelöst (galileo+earth+jupiter), 0 Tage absent (0 geehrt).
Robuste Tag-Serien je (Mode, Station) — laut/ruhig, in Klammern die
Flips des Stufen-Blatts:

| Serie | n≥30-Tage | laut | ruhig | (Flips robust) |
|---|---|---|---|---|
| M1 st14 | 62 | 32 | 30 | (18) |
| M1 st43 | 64 | 35 | 29 | (14) |
| M1 st63 | 68 | 34 | 34 | (15) |
| M2 st14 | 38 | 17 | 21 | (9) |
| M2 st43 | 35 | 16 | 19 | (12) |
| M2 st63 | 39 | 22 | 17 | (11) |
| M3 st14 | 40 | 25 | 15 | (10) |
| M3 st43 | 33 | 16 | 17 | (9) |
| M3 st63 | 21 | 10 | 11 | (7) |

Die Flip-Zählung über die robusten Serien reproduziert die **105 Flips des
Stufen-Blatts exakt** (18/14/15, 9/12/11, 10/9/7). Die robusten
Tages-Zahlen decken sich mit den Vorlagen (M1 st14/43 62/64, M2 38/35/39;
M1 st63 68 wie im Ops-Ära-Blatt — das Stufen-Blatt druckt dort 67; M3 st14
40 gegen 39 dort — je eine Tag-Differenz, benannt, nicht geglättet).

## Messung A — Sichtlinien-Position und Erde–Galileo-Distanz (Ort)

**Die lauten Gipfel-Zellen liegen über den ganzen durchlaufenen
Sichtlinien-Bogen verstreut.** Über die 84 eindeutigen lauten Daten der Ära
wandert die Sichtlinien-Richtung Erde→Galileo (die ICRS-Richtung der
Jupiter-Position, von der Erde aus) von RA ≈ 260,6° / Dec ≈ −23,4°
(Nov 1995, Konjunktions-Seite) über RA ≈ 285° (Opposition Juni 1996) zu
RA ≈ 311° / Dec ≈ −18,5° (Feb 1997) — ein Bogen von ~50°. Laute Zellen
existieren auf dem gesamten Bogen, in jedem getrackten Fenster der Ära
(Monats-Zellen der lauten Tage je Serie: Nov–Dez 1995, Jun 1996, Sep 1996,
Nov–Dez 1996, Jan–Feb 1997; die bodenleeren Monate Feb–Mai 1996 und Okt 1996
tragen keine Zellen — 0 geehrt). Die Resultanten-Längen R der
Himmelsrichtungs-Verteilungen sind für laut und ruhig gleich hoch (R
0,95–0,98 je Serie) und die Zentroid-Trennung laut/ruhig ist klein (1,7–10,5°,
typisch 3,6–7,3°) — beide Populationen sitzen auf demselben Sichtlinien-
Bogen, keine besondere Himmelsregion der Gipfel.

**Der entscheidende Kontroll-Maßstab sind die Flips selbst:** jeder der 105
robusten Flips wechselt den Zustand laut/ruhig zwischen zwei kalendarisch
benachbarten robusten Tagen. Der gemessene Sichtlinien-Schritt zwischen den
zwei Zuständen eines Flips hat einen Median von **0,18°** (Maximum 0,26°);
**alle 105 Flips liegen unter 1°** Himmelswinkel-Trennung, der
Erde–Galileo-Distanz-Sprung hat einen Median von 0,0052 AU (max 0,026 AU).
Die laute und die ruhige Seite eines jeden Flips teilen also dieselbe
Himmels-Position und dieselbe Distanz — der Zustandswechsel läuft bei
fester Geometrie ab.

**Zusätzlich ist der Ort an einem Tag station-gebunden:** an Tagen mit ≥ 2
robusten Stationen im selben Mode misst die Zählung (eine Geometrie, alle
Stationen) — Mode 1: 61 geteilte Tage, davon 31 gemischt (mindestens eine
Station laut, eine ruhig); Mode 2: 27 geteilte Tage, 16 gemischt; Mode 3:
28 geteilte Tage, 17 gemischt. Dieselbe Sichtlinie und dieselbe Distanz
tragen an demselben Tag gleichzeitig laute und ruhige Stationen.

**Erde–Galileo-Distanz:** rE über die Gipfel-Zellen spannt 4,191–6,294 AU —
das gesamte Band der Boden-Ära. Die lautesten Gipfel existieren an beiden
Enden: Opposition Juni 1996 (rE ≈ 4,19 AU, die nächste Erd-Passage der Ära)
und Konjunktions-Seite Nov/Dez 1995 bzw. Jan 1997 (rE ≈ 6,1–6,29 AU, die
fernste). Die Median-Differenz laut gegen ruhig je Serie ist im Vorzeichen
gemischt (M1 st14 6,055 gegen 6,096 AU, M2 st14 6,022 gegen 6,186, M3 st43
6,022 gegen 6,196 — laut näher; M1 st43 6,084 gegen 6,047, M3 st14 6,078
gegen 6,034, M3 st63 6,201 gegen 6,002 — laut ferner): **keine konsistente
rE-Bindung der Gipfel-Zellen.** Die Ära-Verwebung ist offen: rE ist
saisonal (Oppositions-Minimum nur Juni 1996, Konjunktions-Maximum
Dez/Jan), und die lauten Zellen je Serie füllen diese Fenster in
serien-spezifischer Mischung.

**Verdikt A: die Gipfel-Tage sind über die durchlaufene Sichtlinie und über
die rE-Achse verstreut (ortunabhängig); der (Station, Tag)-Zustand wechselt
bei < 1° Sichtlinien-Schritt und ist an ein- und derselben Geometrie je Tag
station-gebunden gemischt.**

## Messung B — Galileo–Jupiter-Distanz und Perijove

Die Galileo–Jupiter-Distanz J über die Boden-Ära (Tagesraster, Minimum je
Monat aus der Ephemeride, gemessen): Nov 1995 104,9 R_J (Anflug) →
**Dez 1995 9,6 R_J am 1995-12-08** (der gemessene Perijove der
Orbit-Einfang-Ära) → 1996 ferne Apoapsis-Seiten (Jan–Mai 1996, 155–266 R_J,
bodenleer) → **Jun 1996 10,9 R_J am 1996-06-29** → Sep 1996 7,1 R_J am
09-08 → **Nov 1996 5,6 R_J am 11-07** → **Dez 1996 6,7 R_J am 12-20** →
**Jan 1997 7,6 R_J am 01-21** → **Feb 1997 6,7 R_J am 02-21**. Die
monatlichen Perijove liegen damit messbar bei ~6–11 R_J.

**J-Distanz der lauten gegen die ruhigen Zellen je Serie (Median, R_J):**

| Serie | laut n | J laut med [min..max] | ruhig n | J ruhig med [min..max] | ≤ Q25 laut / ruhig |
|---|---|---|---|---|---|
| M1 st14 | 32 | 49,0 [6,7 .. 159,9] | 30 | 49,1 [7,6 .. 150,6] | 31 % / 20 % |
| M1 st43 | 35 | 44,9 [6,7 .. 123,6] | 29 | 49,1 [5,6 .. 159,9] | 31 % / 17 % |
| M1 st63 | 34 | 35,4 [6,7 .. 90,6] | 34 | 64,2 [8,1 .. 168,4] | 26 % / 24 % |
| M2 st14 | 17 | 26,6 [6,7 .. 159,9] | 21 | 74,0 [11,5 .. 168,4] | 53 % / 5 % |
| M2 st43 | 16 | 37,6 [6,7 .. 168,4] | 19 | 39,9 [7,1 .. 162,7] | 38 % / 16 % |
| M2 st63 | 22 | 27,0 [6,7 .. 159,9] | 17 | 64,4 [9,7 .. 179,1] | 36 % / 12 % |
| M3 st14 | 25 | 31,7 [6,7 .. 150,6] | 15 | 39,9 [9,7 .. 182,1] | 28 % / 20 % |
| M3 st43 | 16 | 36,8 [6,7 .. 159,9] | 17 | 89,3 [7,1 .. 168,4] | 44 % / 12 % |
| M3 st63 | 10 | 114,7 [6,7 .. 168,4] | 11 | 27,0 [14,9 .. 150,6] | 30 % / 27 % |

Die J-Spanne der lauten Zellen reicht in jeder Serie vom Perijove (~6,7 R_J)
bis zum fernen Anflug/Apoapsis (~160–168 R_J); laute Zellen existieren bei
fast jedem Wert der überstrichenen J-Achse. Die Median-Richtung laut gegen
ruhig ist über die Serien **nicht einheitlich**: sieben Serien lesen laut
näher am Jupiter (M2 st14 am stärksten: 26,6 gegen 74,0 R_J), M1 st14 ist
flach (49,0/49,1), **M3 st63 ist invertiert** (laut 114,7 gegen ruhig
27,0 R_J, weil seine lauten Zellen im fernen Nov-1995-Anflug sitzen). Ein
gemeinsamer J-Selektions-Mechanismus der Gipfel-Zellen ist damit nicht
getragen — die Median-Verschiebung je Serie folgt der serien-eigenen
Verteilung der lauten Zellen über die Ära-Fenster (Ära-Verwebung, offen
gelegt, nicht geglättet). Die Spearman-Werte log10(Tages-RMS) gegen J über
die Serien sind ebenfalls gemischt (−0,65 bis +0,08).

**Der entscheidende Kontroll-Maßstab auch hier — Zustand bei fester J:**
die 105 Flips ändern J zwischen den zwei Zuständen um im Median nur
9,3 R_J (ein Tagesschritt im ~35-Tage-Orbit) bei 0,18° Sichtlinien-Schritt;
und an ein- und demselben Tag mit identischer J sitzen laut und ruhig
nebeneinander (siehe A): z. B. 1996-12-20 (J 6,7 R_J) ist für mehrere Serien
laut und für andere ruhig, 1997-02-21 (J 6,7 R_J) ebenso. Die J-Distanz
setzt den (Station, Tag)-Zustand nicht.

## Das 1996er-Fenster 06-26 .. 06-30 (der Operator-Wecker), gemessen

J (Galileo→Jupiter) je Tag und der robuste Floor-Zustand (L laut, q ruhig;
Zellen ohne robusten Boden an dem Tag bleiben absent):

| Tag | J (R_J) | laute robuste Zellen | ruhige robuste Zellen |
|---|---|---|---|
| 1996-06-26 | 31,7 | M1 st63, M2 st43, M3 st14 | M1 st14, M1 st43 |
| 1996-06-27 | 21,9 | M2 st14, M2 st43, M2 st63, M3 st14, M3 st43 | M1 st14, M1 st63, M3 st63 |
| 1996-06-28 | 11,5 | M1 st14, M2 st43, M2 st63, M3 st14, M3 st63 | M1 st43, M1 st63, M2 st14 |
| 1996-06-29 | 10,9 | M1 st14, M1 st63 | M1 st43, M2 st43 |
| 1996-06-30 | 20,9 | — | M2 st43, M3 st63 |

Das gemessene Perijove des Fensters liegt bei **~8,7 R_J am 1996-06-28**
(Halb-Tages-Abtastung; die Galileo-Ephemeride ist ein 1-Tages-Raster-Chebyshev,
Halb-Tages-Werte sind Fit-Interpolationen, benannt). Die lauten Mode-2-Tage
des Fensters (06-26/27/28) laufen also **auf den Perijove zu** (31,7 → 21,9 →
11,5 R_J), während die ruhigen Mode-2-Tage desselben Fensters (M2 st43 an
06-29/30) **am und knapp nach dem Perijove** sitzen (10,9 und 20,9 R_J). Am
06-28 und 06-29 — zwei Tagen mit nahezu identischer J (11,5 gegen 10,9 R_J) —
wechselt die Lautheit zwischen den Stationen/Modi (06-28: M2 st43/st63 und
M1 st14 laut, M2 st14 und M1 st43/st63 ruhig; 06-29: M1 st14/st63 laut, M2
st43 ruhig). Die lauten 1996er-Tage liegen damit messbar dicht am Perijove —
aber die ruhigen Tage desselben Fensters liegen ebenso dicht daran, und der
Zustand reassigniert zwischen den zwei fast-J-gleichen Tagen. **Die
Jupiter-Distanz unterscheidet die lauten von den ruhigen Tagen des Fensters
nicht.**

## Trabanten-Vorbeiflüge: Asset-Status und was messbar ist

Ein Jupitermond-Ephemeriden-Asset (Io/Europa/Ganymed/Callisto, NAIF 501–504)
liegt im Bestand **nicht vor**: `data/` trägt nur die Planeten- und
Sonden-Tages-Ephemeriden plus den Erdmond (`ephemeris_moon.bin`); keine
Vorbeiflug-Epochen-Tabelle offline in `docs/` (glob/grep nach Ganymed/flyby/
perijove/IO-Europa über den Galileo-Kontext: nur Nicht-Galileo-Treffer).
Damit ist die **Mond-Vorbeiflug-Distanz-Achse `pending` (kein Asset)** — die
Messung der Distanz zu Ganymed an einem Vorbeiflug-Tag ist auf diesen Daten
nicht möglich, und eine Tabelle wird nicht konstruiert. Der vom Operator
benannte, extern dokumentierte Termin **G1 Ganymed 1996-06-27** ist eine
Extern-/Operator-Angabe (kein offline Asset): gemessen ist nur, dass die
lauten Zellen 06-27/28 auf diesen Termin und zugleich auf das gemessene
Perijove des Fensters (06-28, 8,7 R_J) fallen und dass Galileo am 06-27
21,9 R_J vom Jupiterzentrum stand (Ganymed-Bahnradius ~15 R_J) — der Abstand
zum Mond selbst bleibt ungemessen. Als messbarer Stellvertreter für die
Vorbeiflug-Nähe dient die Jupiter-Distanz/Perijove-Achse (oben): sie trennt
die lauten von den ruhigen Tagen des 1996er-Fensters nicht, und der
(Station, Tag)-Zustand mischt an identischer J.

## Verdikt

**Das station-gebundene Gipfel-Muster ist sichtlinien-ortunabhängig und in
seinem (Station, Tag)-Zustand auch himmelskörper-distanzunabhängig —
gemessen, nicht als Ursachen-Ausschluss der Vorbeiflug-Distanz.** (1) Alle
105 robusten Flips wechseln laut/ruhig zwischen zwei Tagen mit < 1°
Sichtlinien-Trennung (Median 0,18°) und 0,0052 AU rE-Sprung — die
Gipfel-Zustände entstehen bei fester Himmels-Position; (2) an Tagen mit
identischer Geometrie und identischer Jupiter-Distanz sind die Stationen
gemischt laut/ruhig (Mode 1: 31 von 61 geteilten Tagen gemischt, Mode 2: 16
von 27, Mode 3: 17 von 28), und im 1996er-Fenster reassigniert der Zustand
zwischen 06-28 und 06-29 bei 11,5 gegen 10,9 R_J; (3) die lauten Zellen
liegen über den ganzen Sichtlinien-Bogen (~50°, RA 260,6–311,1°) und über
die ganze J-Achse (6,7–168 R_J) und die ganze rE-Achse (4,19–6,29 AU)
verstreut; die Median-Verschiebungen laut gegen ruhig auf den
Distanz-Achsen sind über die Serien im Vorzeichen uneinheitlich (J: M3 st63
invertiert, M1 st14 flach; rE: gemischt) und folgen der serien-eigenen
Ära-Verteilung — **keine konsistente Bindung an eine Himmelsregion, eine
Erd- oder Jupiter-Distanz**. Eine echte Bindung an die Trabanten-
Vorbeiflug-Nähe ist damit nicht ausgeschlossen, aber auf diesen Daten nicht
messbar: das Mond-Ephemeriden-Asset fehlt offline (`pending`, kein Ersatz,
keine konstruierte Tabelle); der messbare Stellvertreter Perijove/Jupiter-
Distanz trägt die Bindung nicht.

**Was `pending` bleibt:** die Ganymed-/Io-/Europa-Distanz an den lauten
Tagen (kein Offline-Asset; G1 1996-06-27 ist Extern-Angabe, der gemessene
Jupiter-Abstand ist kein Mond-Abstand); die Ursache des per-Station-Zustands
selbst (per-Pass-Verschaltung, AGC/Schleifen-Zustand, Array-Teilnahme —
unverändert offen); und die Sub-Tages-Achse (Pass-Anfang innerhalb des
Tages), auf der allein ein Flyby-Moment (06-27 ~06:29 G1 gegen den Pass-
Beginn der lauten Zellen) mit der Boden-Struktur verbunden werden könnte.

## Grenzen

- Geometrie einmal je TDB-Tagesanfang (Konvention der Vorlagen); die
  Intra-Tag-Bewegung (ein Flyby ist ein Stunden-Ereignis) liegt außerhalb
  der Tages-Achse. Die Galileo-Tages-Ephemeride ist ein 1-Tages-Raster-
  Chebyshev-Fit; Halb-Tages-Abtastung ist Fit-Interpolation (benannt).
- Die Distanz-Achsen sind mit der Ära verwoben (rE saisonal 4,19–6,29 AU,
  J orbit-periodisch 6–266 R_J, Boden-Fenster nur in bestimmten Monaten);
  die Mediane laut/ruhig je Serie sind dadurch Ära-Kompositionen. Die
  station-gebundene Mischung an identischer Geometrie und die Flip-Schritte
  halten die Geometrie fest — sie sind der tragfähige Kontroll-Maßstab.
- Feb–Mai 1996 und Okt 1996 sind bodenleer (n = 0); die frühe Cruise-Ära
  (1990–94, ~1 AU) trägt keinen Floor — die rE-Aussage gilt nur für das
  Band 4,19–6,29 AU der Boden-Ära, nicht für die Distanz-Achse der ganzen
  Mission.
- M3 st63 trägt nur 10 laute Zellen (21 robuste Tage); sein invertierter
  J-Median steht auf dieser dünnen Basis. Eine Tag-Differenz der robusten
  Serien M1 st63 (68 gegen 67) und M3 st14 (40 gegen 39) zu den
  Stufen-Blatt-Druckzahlen ist benannt (die Flip-Summe 105 reproduziert
  exakt).
- Der Stellvertreter Perijove ist die Jupiter-Distanz, nicht die
  Mond-Distanz; die Trennung der Achsen Mond-Vorbeiflug gegen Jupiter-
  Distanz ist offline nicht möglich (kein Asset).

## Register-Satz

*Die 105 tages-scharfen Floor-Gipfel (n ≥ 30, exakt die robusten Flips des
Stufen-Blatts) sind sichtlinien-ortunabhängig und in ihrem (Station, Tag)-
Zustand himmelskörper-distanzunabhängig: alle 105 Flips wechseln zwischen
zwei Tagen mit < 1° Sichtlinien-Schritt (Median 0,18°) und 0,0052 AU
rE-Sprung; an identischer Geometrie/Jupiter-Distanz sind die Stationen je
Tag gemischt laut/ruhig (M1 31/61, M2 16/27, M3 17/28 geteilte Tage), im
1996er-Fenster reassigniert der Zustand zwischen 06-28 und 06-29 bei 11,5
gegen 10,9 R_J; die lauten Zellen verstreuen über den ganzen Sichtlinien-
Bogen (RA 260,6–311,1°) und die ganze J-Achse (6,7–168 R_J), die
Median-Richtung laut gegen ruhig ist über die Serien uneinheitlich (M3 st63
invertiert, M1 st14 flach). Die Mond-Vorbeiflug-Distanz-Achse ist offline
nicht messbar: kein Jupitermond-Ephemeriden-Asset, keine Vorbeiflug-Tabelle
im Bestand (pending, kein Ersatz); der messbare Stellvertreter
Perijove/Jupiter-Distanz trennt die lauten von den ruhigen Tagen des
1996er-Fensters nicht. Der Sitz des per-Station-Zustands bleibt pending; die
Sub-Tages-/Pass-Anfangs-Achse ist der offene Pfad, an dem allein ein
Flyby-Moment mit der Boden-Struktur verbunden werden könnte.*

## Status

`draft` (Entwurf für die Haupt-Session/Rat; TODO-Registerzeile ergänzt die
Haupt-Session). Probe `galileo_floor_sky_body_probe` committet
(`cargo check` 0/0, RUSTFLAGS `-D warnings`), Report
`/tmp/opencode/galileo_floor_sky_body_report.txt`.
