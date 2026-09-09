<!--
  title: Befund — Galileo-Floor-Lautheit gegen die Betriebs-Ära der Bodenstationen (Richtung A): keine Stufen an den dokumentierten Betriebswechseln — BVR/DGT unbesetzt, einzige Stufen-Konzentration am 5.12.1995 datendünn, die Lautheit wechselt je Station tages-scharf
  class: befund
  date: 2026-09-05
  sha256: b92053c13348d0844365d4d84e8228407eea8684ef322afa1fcdce956ea1d0fe
  status: draft
  antwortet-auf: docs/befund/befund-galileo-receiver-floor-ursache.md docs/befund/befund-galileo-floor-4d-form-farbe.md
  see-also: docs/handover/archiv/handover-2026-09-05-galileo-tiefe-rotor-spin-receiver.md docs/befund/befund-galileo-alpha-zeit-sonnenzyklus.md docs/befund/befund-galileo-mode2-station-split.md
-->

# Befund: Galileo-Floor-Lautheit gegen die Betriebs-Ära der Bodenstationen — die dokumentierten Betriebswechsel tragen die station-gebundene Lautheit nicht als Stufen

## Frage & Bindung

Der 4D-Befund (done) und der Receiver-Ursachen-Befund (done) messen, dass die
Floor-Lautheit (AGC-Klemmwert −2560, Zell-RMS je (Modus, Tag, Station)) bei
identischer 4D-Position station-gebunden 0,02–500 Hz liest und dass die kodierte
Receiver-Identität (ref uniform 5, Nummern absent) diese Lautheit nicht trägt.
Dieser Lauf prüft Richtung A: folgt die station-gebundene Floor-Lautheit den
**kalendarischen Betriebs-/Wartungs-Ären** der 70-m-Stationen — konkret den
bekannten DSN-Betriebswechseln 1995/96 —, nicht der Geometrie? Die Anker der
Betriebs-Ära stammen aus der externen Recherche
(`~/Schreibtisch/rechercheauftrag-extern-galileo-dsn_findings.md`, TDA Progress
Report 42-125/42-133, Beyer et al.; hier als Referenz geführt, nicht neu
verifiziert): BVR-Netzstandard ~18.9.1995, 5.12.1995-Modulationswechsel
(suppressed-carrier 90° → residual-carrier 58°), DGT-Phase-2 ~Mai 1996,
Full-Array-Routine ab ~Nov 1996, 1995er-Wartungszyklus der drei 70-m-Antennen
(Apr/Mai/Aug/Sep 1995).

Gebunden wie die Vorlagen: Modus 1 primär (Floor-Träger), Modus 2 sekundär; Lock
(|resid| > 1000 Hz) vor dem Rauschen getrennt; Stärke 0 nie klassiert; Boden =
Stärke exakt −2560, stark = Stärke ≥ −1750; Rausch-Zelle = (Modus, TDB-Tag,
Station, Klasse)-RMS um den Zellen-Mittelwert; laut = Zell-RMS ≥ 1 Hz. Metrik der
Vorgänger unverändert; n je Zelle wird ausgewiesen, Zellen mit n < 30 Proben
(dünn) werden separat gezählt und die Stufen-Analyse wird zusätzlich auf n ≥ 30
gerechnet. Probe `tools/measure/src/bin/galileo_ops_era_station_step_probe.rs`
(neu, einzige Repo-Änderung außer diesem Blatt; `cargo check` 0/0). Vollständige
Zell-Tabellen: `/tmp/opencode/galileo_ops_era_station_step_report2.txt`. Daten:
`data/galileo_resid.bin`, Geometrie je TDB-Tag aus `galileo_daily` + `earth`.

## n zuerst (0 geehrt)

| Klasse | Modus | Zellen (Tag,Station) | Tage | Spanne | laut | Median Zell-RMS |
|---|---|---|---|---|---|---|
| Boden | 1 | 268 | 113 | 1994-12-18 .. 1997-02-28 | 143 | 1,125 Hz |
| Boden | 2 | 124 | 75 | 1995-11-23 .. 1997-02-25 | 60 | 0,868 Hz |

Boden Modus 1 je Station: st14 82 Zellen/82 Tage/42 laut/Median 1,061 · st43
84/84/45/1,147 · st63 98/98/55/1,631 · st24 1 Zelle (1994-12-18, 0,032 Hz ruhig)
· st34 2 (1997-02, ruhig) · st45 1 (1997-01-31, 454 Hz, n 21). Boden Modus 2 je
Station: st14 39/17/0,414 · st43 35/16/0,578 · st63 50/27/1,955. Der Floor der
70-m-Stationen beginnt 1995-11-23/24 (ein einzelner früher Boden-Tag 1994-12-18
liegt an st24, nicht an 14/43/63); zwischen 1995-12-31/1996-01-14 und 1996-06-26
liegt eine 163–192 Tage lange Floor-Abwesenheit (Feb–Mai 1996: n = 0 Boden-Zellen
an allen Stationen), ebenso ist der Oktober 1996 leer.

Dünne Zellen (n < 30) — der n-Vorbehalt der Laut-Zählung: Modus 1 st14 20 Zellen
(10 laut), st43 20 (10 laut), st63 30 (21 laut); das n ≥ 30-Teilset: st14 62/32,
st43 64/35, st63 68/34. Der st63-Laut-Anteil (55 → 34) wird also wesentlich von
dünnen Zellen getragen (die tiefe Konjunktion 1997 liest oft nur wenige
Boden-Proben je Tag, aber mit hoher Streuung); st14/st43 verlieren je 10 Laut-
Zellen. Modus 2 ist fast vollständig gut besetzt (st14 n ≥ 30: 38/17, st43
35/16, st63 39/22). Die Stufen-Analyse wird unten für alle Zellen und für n ≥ 30
geführt; Zentralbefunde, die beide Lesarten tragen, werden als solche benannt.

## Betriebs-Ära-Anker gegen die Floor-Serie — Messung 1: Stufen-Suche

Je (Modus, Station) wird die Tages-RMS-Reihe des Bodens auf Stufen geprüft —
(a) datengetrieben (bester Ein-Schnitt: maximale |Mittel-log10-RMS|-Differenz,
je Seite ≥ 2 Zellen), (b) ankergebunden (±120-Tage-Fenster um jeden
Betriebs-Anker, je Seite n gezählt; Seite mit n < 2 bleibt absent).

**Bester datengetriebener Ein-Schnitt (alle Zellen):**

| Serie | Schnitt nach | links (n, med, laut) | rechts (n, med, laut) | Lücke | nächster Anker |
|---|---|---|---|---|---|
| M1 st14 | 1995-12-07 | 13, 0,090 Hz, 3/13 | 69, 1,186 Hz, 39/69 | 1,00 dB | 5.12.1995 (2 d) |
| M1 st43 | 1995-11-25 | 2, 0,044 Hz, 0/2 | 82, 1,732 Hz, 45/82 | 1,77 dB | 5.12.1995 (7 d) |
| M1 st63 | 1995-12-02 | 9, 0,106 Hz, 0/9 | 89, 15,08 Hz, 55/89 | 1,58 dB | 5.12.1995 (2 d) |
| M2 st14 | 1995-12-01 | 9, 0,031 Hz, 1/9 | 30, 2,878 Hz, 16/30 | 1,15 dB | 5.12.1995 (3 d) |
| M2 st43 | 1996-12-22 | 33, 0,436 Hz, 14/33 | 2, 79,34 Hz, 2/2 | 1,90 dB | Array 11/1996 (51 d) |
| M2 st63 | 1995-12-05 | 12, 0,720 Hz, 4/12 | 38, 11,42 Hz, 23/38 | 1,14 dB | 5.12.1995 (0 d) |

Fünf der sechs Serien legen ihren größten Schnitt ans Ende 1995, 0–7 Tage vom
5.12.1995-Anker — aber der linksseitige Bestand ist datendünn (n 2–13) und fällt
mit dem Floor-Beginn bzw. Konjunktions-Eintritt (ε 11°→1°) zusammen. Auf dem
n ≥ 30-Teilset bleibt dieser Schnitt nur für st14 (M1, links n 13/0,090/3-13,
rechts n 49/4,10/29-49) und st63 (M1 links n 7/0,105/0-7, rechts n 61/15,08/34-61)
sowie st14/st63 (M2) tragfähig; bei st43 (M1 und M2) kollabiert er (st43-M1-n≥30
liest links n 62/2,24/35-62 gegen einen Zwei-Tage-Schwanz rechts — die
st43-Boden-Lautheit ist über die ganze Ära verteilt, nicht in eine frühe ruhige
und späte laute Phase geteilt).

**Ankergebundene Fenster (Modus 1, ±120 d):** BVR 18.9.1995: vor n = 0, nach
n = 0 → unbesetzt (der 70-m-Floor existiert vor dem BVR-Netzstandard nicht; ein
einziger früher Boden-Tag 1994-12-18 an st24). Wartungs-Anker 1995 (Apr/Mai/Aug/
Sep): vor und nach n = 0 → unbesetzt. DGT Mai 1996: vor n = 0 (Feb–Mai 1996
bodenleer), nach: st14 n 7/med 7,92 · st43 n 7/0,021 · st63 n 8/27,2 → der
Übergang über das DGT-Datum ist auf der Floor-Serie nicht messbar. 5.12.1995:
vor n 10/med 0,179 (st14), 5/0,044 (st43), 11/0,108 (st63) → nach n 6/13,33,
3/0,187, 8/297,4 — Niveauanstieg an allen drei Stationen über das Fenster, aber
nach n 3–8 (dünn) und im Konjunktions-Eintritt. Array Nov 1996: vor n 3–4 (die
lauten September-Zellen, med 61,9/6,8/97,8) → nach n 58–70 (med 1,06/2,47/1,65) —
st14/st63 fallen über die Lücke, st43 nicht; zwischen vor und nach liegt ein
leerer Oktober, kein tag-scharfer Schritt am Anker.

## Tages-scharfer Zustandswechsel — Messung 2: Nachbar-Tages-Flips

Die Floor-Tages-Reihe je (Modus, Station) wechselt laut/ruhig zwischen
**kalendarisch benachbarten Boden-Tagen** (Lücke exakt 1 Tag, beide Tage mit
Boden-Zellen): Modus 1 st14 24, st43 28, st63 28; Modus 2 st14 9, st43 12,
st63 13 → **114 Nachbar-Tages-Flips** (n ≥ 30-Teilset: 18/14/15 bzw. 9/12/11 —
die Flips überleben die Dünn-Zell-Korrektur). Beispiele bei nahezu konstanter
Geometrie (Δε < 1,5° je Tag) und konstanter kodierter Receiver-Identität (ref 5):
st14 1995-11-24 25,85 Hz → 1995-11-25 0,090 Hz; st14 1995-11-26 14,37 Hz →
1995-11-27 0,145 Hz; st63 1996-06-26 20,64 Hz → 1996-06-27 0,068 Hz; st43
1996-12-27 55,88 Hz → 1996-12-28 154,4 Hz → 1996-12-29 0,780 Hz. Von den 114
Flips liegen 27 (24 %) innerhalb ±7 Tagen des nächsten dokumentierten Ankers
(15 am 5.12.1995-Anker, 12 am Array-Nov-1996-Anker), 41 innerhalb ±30 Tagen; die
übrigen 76 % (87 von 114) liegen an keinem Betriebs-Anker. Die Flips sind über die gesamte
Ära verteilt — auch innerhalb eines konstanten Betriebs-Zustands wechselt die
Lautheit tag-scharf in beide Richtungen.

## Die Anker-Tage 1995-11-24 und 1996-06-26 — Messung 3: Zerlegung

| Tag | ε / r | Modus 1 Boden je Station | Modus 2 Boden je Station |
|---|---|---|---|
| 1995-11-24 | 20,4° / 5,272 AU | st14 **25,849 Hz laut** (n 7463) · st43 0,031 Hz (n 39991) · st63 0,091 Hz (n 5847) | st14 **31,77 Hz laut** (n 6487) · st43 0,866 Hz (n 3085) · st63 1,993 Hz (n 10572) |
| 1996-06-26 | 169,9° / 5,191 AU | st14 0,021 Hz (n 19083) · st43 0,021 Hz (n 25792) · st63 **20,640 Hz laut** (n 24201) | st43 **2,792 Hz laut** (n 7503); st14/st63 n = 0 |

(Zusätzlich am 1995-11-24: st14 stark 37,36 Hz, n 102 — der laute Tag ist dort
nicht auf die Boden-Klasse beschränkt.) Zerlegung: **der laute Boden wird an den
zwei Anker-Tagen von verschiedenen Stationen getragen** — Modus 1 1995-11-24 von
st14, 1996-06-26 von st63; st43 ist im Modus 1 an beiden Tagen ruhig. st14 ist
eine identische Station mit Zustandswechsel laut→ruhig zwischen den Tagen, st63
wechselt ruhig→laut; die Wechselrichtung ist je Station verschieden, kein
netzweiter Schritt. Im Modus 2 liegt am 1996-06-26 nur st43 im Boden (n = 0 an
st14/st63) — die Modus-2-Zelle ist an diesem Tag eine Ein-Station-Zelle.

## Gegenhypothese Geometrie im selben Fenster — Messung 4

(a) Gleicher Tag = identische Geometrie für alle Stationen: die Zerlegung oben
liest am selben (x,y,z,t) 0,021–25,85 Hz (1995-11-24) bzw. 0,021–20,64 Hz
(1996-06-26) je Station — die Lautheit ist keine Geometrie-Größe (reproduziert,
4D-Vorbefund). (b) Gleiche Geometrie-Band über zwei Epochen, Floor bei ε < 30°
(Konjunktion), Modus 1: Konj 1995 st14 n 17/med 0,179/laut 7-17 · st43 n 9/0,079/
3-9 · st63 n 20/0,144/7-20 gegen Konj 1997 st14 n 34/1,093/20-34 · st43 n 41/3,90/
26-41 · st63 n 45/15,08/30-45 — dieselbe ε-Band liest 1997 an allen drei Stationen
lauter (6–100×), n der 1997-Seite tragfähig. (c) Spearman log10(Zell-RMS) gegen ε/
α/r/Tag über die ganze Floor-Ära, Modus 1: st14 −0,03/+0,03/−0,01/−0,01 ·
st43 −0,45/+0,45/−0,34/+0,30 · st63 −0,24/+0,24/+0,01/+0,13. Die drei Stationen
folgen keiner gemeinsamen Geometrie-Linie (st14 ist ε-flach), und die
Nachbar-Tages-Flips (Messung 2) laufen bei Δε < 1,5° ab.

## Verdict

**Richtung A ist als Kalender-Stufen-Modell nicht getragen (gemessen).** Die
dokumentierten Betriebswechsel tragen die station-gebundene Floor-Lautheit nicht
als Stufen: BVR (18.9.1995) und der 1995er-Wartungszyklus sind auf der
70-m-Floor-Serie unbesetzt (vor n = 0 — der Floor beginnt erst mit dem
BVR-Betrieb), DGT (Mai 1996) ist vor n = 0 (Feb–Mai 1996 bodenleer) — beide
Übergänge sind auf diesen Daten nicht messbar, nicht als ruhige Seite vorhanden.
Der einzige Anker mit Niveauänderung im Fenster (5.12.1995) liegt 0–7 Tage vom
größten Ein-Schnitt von fünf der sechs (Modus, Station)-Serien — aber datendünn
(links n 2–13, n ≥ 30 nur für st14/st63 tragfähig) und untrennbar vom
Konjunktions-Eintritt (ε 11°→1°), an dem der Modulationswechsel selbst als
Gegenmaßnahme dokumentiert ist; eine Ursachen-Trennung Stufen-Wechsel =
Betriebsdatum gegen Stufen-Wechsel = Konjunktions-Geometrie ist hier nicht
möglich (Confound, benannt, nicht geglättet). Die dominante gemessene Struktur
ist dagegen ein tages-scharfer, station-gebundener Zustandswechsel: 114
Nachbar-Tages-Flips bei nahezu konstanter Geometrie und konstanter kodierter
Receiver-Identität, 76 % ohne Betriebs-Anker in ±7 Tagen, in beide Richtungen
und über die ganze Ära verteilt; und der laute Floor wandert zwischen den zwei
Anker-Tagen von st14 zu st63, während st43 ruhig bleibt. Die Floor-Lautheit ist
damit ein (Station, Tag)-Zustand auf Pass-Zeitskala, schneller als jede
dokumentierte Betriebs-Grenze; ihr physikalischer Sitz (per-Pass-Verschaltung,
Schleifen-/AGC-Zustand, Array-Teilnahme) bleibt wie im Receiver-Befund
`pending`. Die Geometrie trägt die Lautheit im selben Fenster ebenfalls nicht
(Messung 4); der Befund fügt der Negativ-Kette — nicht 4D-Feld, nicht kodierte
Receiver-Identität — das dritte gemessene Negativ hinzu: nicht die kalendarische
Betriebs-Ära der Stationen.

## Grenzen

- Die 1995/96-Fenster sind datendünn: der 70-m-Floor beginnt 1995-11-23/24, der
  Vor-BVR-Floor fehlt (n = 0), Feb–Mai 1996 und Okt 1996 sind bodenleer (n = 0).
- Ein Viertel der Modus-1-Boden-Zellen ist dünn (n < 30); die Laut-Zählung von
  st63 hängt teilweise an diesen Zellen. Alle Stufen-Aussagen sind auf n ≥ 30
  gegengerechnet; die st43-Serie trägt keinen frühen-ruhig/spät-laut-Schnitt.
- Der 5.12.1995-Stufen-Konzentration ist der Konjunktions-Eintritt überlagert —
  Betriebsdatum und Geometrie sind dort nicht trennbar (gemessener Confound).
- Die Anker-Daten der Betriebs-Ära sind externe Referenz (TDA 42-125/42-133 in
  den Recherche-Findings), hier nicht neu verifiziert; die DGT/Array-Anker sind
  Monats-/Fenster-Genauigkeit, nicht tag-genau.
- Der Tag-Schlüssel ist der TDB-Tag (Mittags-Konvention der Vorlagen); die
  Zell-Definition (alle Boden-Zellen, laut ≥ 1 Hz) ist unverändert zur
  Referenz-Linie; n ≥ 30-Zählungen sind ergänzend, nicht ersetzend.

## Register-Satz

*Die station-gebundene Floor-Lautheit folgt den dokumentierten DSN-Betriebswechseln
nicht als Kalender-Stufen: BVR 9/1995 und DGT 1996 sind auf der Floor-Serie
unbesetzt (n = 0 vor dem Anker), die einzige Stufen-Konzentration (5.12.1995,
0–7 Tage vor dem größten Ein-Schnitt von fünf der sechs Serien) ist datendünn
(links n 2–13) und konjunktions-konfundiert; die dominante Struktur ist ein
tages-scharfer Zustandswechsel (114 Nachbar-Tages-Flips, 76 % ohne Anker in ±7
Tagen, bei konstanter Geometrie und kodierter Receiver-Identität), und der laute
Floor wandert zwischen den Anker-Tagen von st14 (1995-11-24) zu st63
(1996-06-26) — die Betriebs-Ära-Hypothese als Schritt-Modell ist damit gemessen
negativ, der per-Pass-Zustand je (Station, Tag) bleibt pending.*

## Status

`draft` (Entwurf für die Haupt-Session/Rat; TODO-Registerzeile ergänzt die
Haupt-Session später). Probe `galileo_ops_era_station_step_probe` committet
(`cargo check` 0/0), Report
`/tmp/opencode/galileo_ops_era_station_step_report2.txt`. Zerlegt die
Betriebs-Ära-Hypothese (Richtung A) auf der Floor-Serie, wo die Daten es
zulassen; die n-leeren Anker (BVR/Wartung/DGT) sind ausgewiesen, nicht
interpoliert.
