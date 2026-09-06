<!--
  title: Befund — ruhige Basis der Galileo-Floor-Residuen über die Floor-Ära 1995-11-23..1997-02-28: das ruhige Tages-Niveau (Resid-Mittel/Median) liegt klein-positiv (+0,02…+0,29 Hz) und senkt sich in den Mode-1-Serien langsam (M1 st43 −0,41 Hz und M1 st63 −0,42 Hz über 27 Monate, p_t 0,0017/0,0004) als Saison-Lage-Änderung, nicht als Monats-Rampe; Modi 2/3 flach und im Vorzeichen gemischt; die ruhige Rausch-Decke (Tages-RMS 0,04–0,09 Hz) driftet nicht systematisch — Boden-/Reduktions-Eigenschaft, kein Signal
  class: befund
  date: 2026-09-06
  sha256: bbf642e7322f247a599f7560eca965c6b21cda3cfcb2d1bf06d63ad5531c77bf
  status: draft
  see-also: docs/befund/befund-galileo-floor-stufen-te.md docs/befund/befund-galileo-floor-pass-episodisch.md
-->
# Befund: Ruhige Basis der Galileo-Floor-Residuen über die Floor-Ära — langsame Mode-1-Niveau-Senkung (~0,4 Hz über 27 Monate), keine Monats-Rampe, kein Drift über die Modi; die ruhige Rausch-Decke driftet nicht

## Frage & Bindung

Dieser Lauf (Richtung D2) prüft, ob die ruhige Basis der Galileo-Floor-Residuen
über die Floor-Ära (1995-11-23..1997-02-28) **langsam drifft** — auf der Achse
des Resid-Niveaus (Tages-Mittelwert und Tages-Median der Residuen, nicht das
RMS der laut-Gipfel) und der ruhigen Rausch-Decke (Tages-RMS der ruhigen Tage).
Gebunden wie die Vorlagen: Tages-Zelle (Mode, Station, Tag) über die
in-track-Boden-Residuen (Stärke exakt −2560, |resid| ≤ 1000 Hz; Lock vor dem
Rauschen getrennt); robust = Zell-n ≥ 30; ruhig = Zell-RMS < 1 Hz, laut =
≥ 1 Hz; Tag = gerundeter TDB-Zivil-Tag. Das **Tages-Niveau** einer ruhigen
Zelle ist der Tages-Mittelwert und der Tages-Median der Residuen (die Ebene
der Residuen selbst), die **ruhige Decke** ist das Tages-RMS um den
Zellen-Mittelwert.

Trend-Fit: OLS der Tages-Werte gegen Tage seit Ära-Beginn (1995-11-23); die
Kalender-Lücken (bodenleere Monate) bleiben als leere Tage im x-Abstand stehen
(0 geehrt, kein Wert gefüllt); SE aus der Restvarianz (n−2 df), p zweifach —
Student-t (zweiseitig) und Permutations-Null (1999 y-Permutationen bei festem
x, Seed je Serie). Probe `tools/measure/src/bin/galileo_floor_basis_drift.rs`
(neu, einzige Repo-Änderung außer diesem Blatt; `cargo check` 0/0,
RUSTFLAGS `-D warnings`), Report
`/tmp/opencode/galileo_floor_basis_drift_report.txt`. Daten:
`data/galileo_resid.bin` (Pfad-Notiz: unter
`data/pds-ppi.igpp.ucla.edu/galileo_resid.bin`).

## n zuerst (0 geehrt) — die Zellen und die Boden-Lücken

Der Zensus reproduziert das Register exakt: **400 robuste (Mode, Station,
Tag)-Zellen, 207 laut / 193 ruhig** (n ≥ 30; je Serie unten). Dünne Tage
(1..29 Proben) werden ausgewiesen, nie klassiert.

| Serie | robust | laut | ruhig | dünn | Boden-Samples der Serie |
|---|---|---|---|---|---|
| M1 st14 | 62 | 32 | 30 | 20 | 531 677 |
| M1 st43 | 64 | 35 | 29 | 20 | 963 535 |
| M1 st63 | 68 | 34 | 34 | 30 | 593 263 |
| M2 st14 | 38 | 17 | 21 | 1 | 520 729 |
| M2 st43 | 35 | 16 | 19 | 0 | 804 569 |
| M2 st63 | 39 | 22 | 17 | 11 | 554 218 |
| M3 st14 | 40 | 25 | 15 | 5 | 248 147 |
| M3 st43 | 33 | 16 | 17 | 2 | 344 468 |
| M3 st63 | 21 | 10 | 11 | 1 | 86 781 |

Boden-Samples der Ära auf den 70-m-Stationen (Mode 1..3): 4 647 387 in-track,
1 815 098 Lock-Samples ausgeschlossen. Bodenleere Monate (0 Floor-Samples in
allen Serien), gemessen: **Februar–Mai 1996, Juli–August 1996, Oktober 1996**
(die Register-Nennung Feb–Mai und Okt 1996 bestätigt; Jul–Aug 1996 zusätzlich
leer gemessen, 0 geehrt).

## Messung 1 — Lage des ruhigen Tages-Niveaus (Offset um 0?)

Je Serie über die ruhigen robusten Tage: Serien-Median der Tages-Mediane und
der Tages-Mittelwerte, das n-gewichtete Mitglieds-Mittel der Residuen und die
ruhige Decke (Tages-RMS):

| Serie | ruhige Tage | Mitglied-Samples | Median der Tages-Mediane Hz | Median der Tages-Mittel Hz | Mitglieds-Mittel Hz | ruhige Decke (RMS-Median) Hz |
|---|---|---|---|---|---|---|
| M1 st14 | 30 | 314 742 | +0,157 | +0,071 | +0,167 | 0,075 |
| M1 st43 | 29 | 603 995 | +0,169 | +0,171 | +0,186 | 0,044 |
| M1 st63 | 34 | 354 867 | +0,028 | +0,061 | +0,086 | 0,064 |
| M2 st14 | 21 | 322 517 | +0,087 | +0,085 | +0,104 | 0,047 |
| M2 st43 | 19 | 401 002 | +0,078 | +0,072 | +0,021 | 0,085 |
| M2 st63 | 17 | 235 648 | +0,128 | +0,130 | +0,114 | 0,064 |
| M3 st14 | 15 | 76 299 | +0,086 | +0,079 | +0,028 | 0,129 |
| M3 st43 | 17 | 186 700 | +0,086 | +0,087 | +0,100 | 0,077 |
| M3 st63 | 11 | 44 149 | +0,116 | +0,116 | +0,286 | 0,285 |

**Das ruhige Tages-Niveau liegt damit nicht bei 0, sondern klein-positiv**:
Serien-Mediane der Tages-Mittel +0,06…+0,17 Hz, Mitglieds-Mittel +0,02…+0,29 Hz
(acht Serien unter +0,19 Hz, M3 st63 +0,29 Hz bei der höchsten ruhigen Decke).
Der Offset ist klein gegen die laut-Gipfel-Skala (ruhige Basis ≈ 1–4× der
ruhigen Decke, laut-Tage 1–530 Hz), aber systematisch positiv über alle neun
Serien — die Residuen der ruhigen Tage sitzen im Mittel eine Zehntel-Hz über
der Null. Die Streuung der Tages-Werte (SD der Tages-Mediane) liegt in
acht Serien bei 0,11–0,52 Hz; M2 st43 trägt mit 7,34 Hz den −31,9-Hz-
Versatz-Tag. **Kohärente Versatz-Tage:** drei ruhige Tage (von 193, gemessen,
je einer in M1 st14, M2 st43 und M2 st63) tragen ein ganz-tägiges Niveau
|Median| > 1 Hz bei Tages-RMS < 1 Hz — M2 st43 1996-01 −31,9 Hz
(Tages-RMS 0,085), M2 st63 1996-01 +2,2 Hz (RMS 0,996), M1 st14 +1,08 Hz.
Diese Tage sind keine laut-Gipfel (geringe Streuung, großer konstanter Versatz), sondern eine eigene Ebene des Niveaus; sie treiben Mittelwerte, nicht Mediane.

## Messung 2 — linearer Trend des ruhigen Niveaus über die Ära

OLS auf den Tages-Werten (ruhige Tage) gegen Tage seit Ära-Beginn; p_t
Student-t, p_perm Permutation; Drift = Steigung × Spanne:

| Serie | n Tage | Monate | Spanne d | Steigung Hz/Tag | SE Hz/Tag | p_t | p_perm | Drift über Ära Hz |
|---|---|---|---|---|---|---|---|---|
| M1 st14 (Med) | 30 | 7 | 461 | −3,5e-4 | 3,1e-4 | 0,260 | 0,264 | −0,16 |
| M1 st14 (Mittel) | 30 | 7 | 461 | −3,6e-4 | 3,0e-4 | 0,243 | 0,245 | −0,17 |
| M1 st43 (Med) | 29 | 8 | 462 | −8,8e-4 | 2,5e-4 | **0,0017** | **0,0055** | −0,41 |
| M1 st43 (Mittel) | 29 | 8 | 462 | −8,5e-4 | 2,5e-4 | **0,0024** | **0,0030** | −0,39 |
| M1 st63 (Med) | 34 | 7 | 461 | −9,0e-4 | 2,3e-4 | **0,0004** | **0,0010** | −0,42 |
| M1 st63 (Mittel) | 34 | 7 | 461 | −8,6e-4 | 2,3e-4 | **0,0006** | **0,0005** | −0,40 |
| M2 st14 (Med) | 21 | 7 | 460 | −2,7e-4 | 1,9e-4 | 0,156 | 0,184 | −0,12 |
| M2 st43 (Med) | 19 | 7 | 394 | +7,1e-3 | 1,1e-2 | 0,512 | 0,874 | +2,78 |
| M2 st63 (Med) | 17 | 7 | 455 | −4,9e-4 | 6,3e-4 | 0,447 | 0,723 | −0,22 |
| M3 st14 (Med) | 15 | 7 | 453 | −3,2e-5 | 2,5e-4 | 0,900 | 0,903 | −0,01 |
| M3 st43 (Med) | 17 | 6 | 460 | −2,1e-4 | 1,4e-4 | 0,119 | 0,147 | −0,10 |
| M3 st43 (Mittel) | 17 | 6 | 460 | −3,0e-4 | 1,6e-4 | 0,080 | 0,086 | −0,14 |
| M3 st63 (Med) | 11 | 6 | 393 | +5,4e-4 | 5,8e-4 | 0,376 | 0,425 | +0,21 |

**Gemessen: eine langsame Niveau-Senkung in den Mode-1-Serien.** Alle drei
M1-Steigungen sind negativ; M1 st43 und M1 st63 sind über der Null
(p_t 0,0017 bzw. 0,0004, p_perm 0,0055 bzw. 0,0010) bei einer Drift von
≈ −0,41 Hz über die 27 Monate — ein langsames Sinken des ruhigen
Resid-Niveaus um etwa vier bis neun Einheiten der ruhigen Decke. M1 st14 ist
im Vorzeichen gleich, aber nicht signifikant (p 0,26). **Die Modi 2 und 3
(Zwei-/Dreiweg, uplink-gebunden) tragen keinen signifikanten Niveau-Trend**:
die Steigungen sind im Vorzeichen gemischt (M2 st14/st63, M3 st43 negativ;
M2 st43, M3 st63 positiv), alle p_t ≥ 0,08; die großen M2-Steigungen
(M2 st43 +7,1e-3, M2 st63 −5,2e-3 im Teil-Körper) hängen an einzelnen
kohärenten Versatz-Tagen (siehe Messung 4). Ein gemeinsamer Boden-/Drift über
alle Modi und Stationen ist damit **nicht** gemessen — die Senkung sitzt in
den Mode-1-Serien (One-Way-Downlink), nicht in den uplink-tragenden Modi.

## Messung 3 — die ruhige Decke über die Zeit

OLS auf dem Tages-RMS der ruhigen Tage (die Decke ~0,04–0,09 Hz, nicht die
lauten Gipfel):

| Serie | n Tage | Steigung Hz/Tag | p_t | p_perm | Drift über Ära Hz | log10-Steigung /Tag |
|---|---|---|---|---|---|---|
| M1 st14 | 30 | +2,1e-4 | 0,471 | 0,471 | +0,10 | +1,4e-4 |
| M1 st43 | 29 | +4,7e-4 | **0,041** | **0,032** | +0,22 | +1,1e-3 |
| M1 st63 | 34 | −1,0e-4 | 0,247 | 0,273 | −0,05 | −1,1e-3 |
| M2 st14 | 21 | +1,7e-4 | 0,382 | 0,506 | +0,08 | +4,0e-4 |
| M2 st43 | 19 | −1,4e-4 | 0,682 | 0,710 | −0,05 | +4,2e-4 |
| M2 st63 | 17 | −4,0e-4 | 0,308 | 0,310 | −0,18 | −3,6e-4 |
| M3 st14 | 15 | +3,1e-4 | 0,306 | 0,341 | +0,14 | +7,9e-4 |
| M3 st43 | 17 | +9,6e-4 | **0,0045** | **0,0035** | +0,44 | +2,1e-3 |
| M3 st63 | 11 | +5,7e-4 | 0,418 | 0,415 | +0,22 | +9,1e-4 |

**Die ruhige Decke driftet nicht systematisch.** Sieben von neun Serien sind
flach (p_t 0,25–0,68); die zwei positiven Tendenzen (M1 st43 +0,22 Hz, M3 st43
+0,44 Hz über die Ära) hängen an wenigen **späteren, höher-ruhigen** Tagen
(die ruhigen Tage nahe der 1-Hz-Grenze, Monats-RMS bis ~0,8 Hz in 1996-12 und
1997-02) und sind in der Monats-Block-Struktur keine gleichmäßige Rampe. Die
Decken-Lage je Serie (RMS-Median 0,044–0,085 Hz in sechs Serien, M3 st14
0,13 Hz, M3 st63 0,28 Hz) reproduziert die Referenz.

## Messung 4 — Monats-Block: Saison-Lage-Differenz, keine Monats-Rampe

Die Tages-Niveau-Senkung der M1-Serien wird aufgelöst (Serien-Median der
Tages-Mediane je Kalendermonat, ruhige Tage; n Tage je Monat in Klammern):

| Monat | M1 st43 | M1 st63 | M1 st14 |
|---|---|---|---|
| 1995-11 | +0,719 (3) | +0,717 (7) | +0,775 (4) |
| 1995-12 | +0,033 (2) | +0,026 (5) | +0,032 (6) |
| 1996-06 | +0,389 (3) | +0,239 (2) | +0,224 (2) |
| 1996-09 | +0,191 (2) | – | – |
| 1996-11 | +0,149 (5) | −0,078 (5) | +0,313 (5) |
| 1996-12 | +0,193 (6) | +0,170 (8) | +0,188 (4) |
| 1997-01 | +0,003 (3) | −0,009 (2) | +0,495 (4) |
| 1997-02 | +0,003 (5) | +0,006 (5) | +0,029 (5) |

Die Struktur ist **kein glatter Monats-Ramp**: M1 st43/M1 st63 liegen 1995-11
hoch (+0,72 Hz), fallen nach 1995-12 auf ~0,03 Hz, heben Mitte/Ende 1996 auf
+0,15…+0,39 Hz und sinken zu 1997-01/02 auf ~0,00 Hz. Der OLS-Trend über die
Ära beschreibt damit eine **langsame Niveau-Senkung zwischen den Saisonen**
(1995-11 und 1996 über 1997), deren Monats-Mediane (Spanne ~0,1–0,2 Hz) in der
gleichen Größenordnung liegen wie die gemessene Gesamt-Senkung (~0,4 Hz) — bei
1–8 ruhigen Tagen je Monat ist eine gleichmäßige Monats-Rampe nicht von einer
groben Lage-Differenz zu trennen. Der Teil-Körper-Fit auf das 1996-97-Innere
(Tage ≥ 1996-06-01) bestätigt: M1 st43 −9,3e-4 Hz/Tag (n 24, p_t 0,028,
p_perm 0,027), M1 st63 −1,1e-3 (n 22, p_t 0,046, p_perm 0,046) — grenzwertig
signifikant, von der Lage-Differenz Mitte/Ende 1996 gegen 1997 getragen, ohne
innere Monats-Rampe (1996-11..1997-02 bei M1 st63: −0,078/+0,170/−0,009/
+0,006 Hz). Die übrigen Serien tragen im Teil-Körper keine Signifikanz; die
starken M2-Teil-Körper-Steigungen verschwinden, wenn der einzelne
kohärente Versatz-Tag entfernt wird (M2 st43: p_t 0,0055 → 0,847 nach
1996-06-01; M2 st63: p_t 0,0001 → 0,195) — ein Maß der Empfindlichkeit gegen
Einzel-Tage, benannt, nicht geglättet.

## Größenordnung & Einordnung

Die gemessene Niveau-Senkung der M1-Serien beträgt **≈ −0,4 Hz über 27 Monate**
(Steigung ≈ −9×10⁻⁴ Hz/Tag). Das ist klein gegen die Skala der laut-Gipfel
(Tages-RMS 1–530 Hz, die 1-Hz-Laut-Schwelle) und gegen die bekannte
Boden-/Bahn-Dynamik der Floor-Residuen; es ist zugleich vier bis neun Einheiten
der ruhigen Decke (0,04–0,09 Hz) und damit die einzige gemessene langsame
Bewegung auf der Niveau-Achse. Eine Pioneer-größenordnungsmäßige
unmodellierte Akzelerations-Rampe (a ≈ 8,7×10⁻¹⁰ m/s²) über 27 Monate würde
auf dem S-Band-Downlink (≈ 2,3 GHz) eine Zweiweg-Doppler-Änderung von grob
~1 Hz erzeugen (Δf/f ≈ 2aT/c ≈ 4×10⁻¹⁰) — die gemessene −0,4-Hz-Senkung
liegt damit **unter** dieser Rampe, ist aber in derselben Größenordnung.
Gegen eine solche Rampe sprechen die gemessene Struktur (Saison-Lage-
Differenz mit 1995-11- und 1996-Hochs und einem 1997-Null, keine monotone
Monats-Rampe, station-abhängige Details: M1 st14 mit hohem 1997-01 bei
steigender Decke bricht die negative Linie) und der Sitz (nur Mode 1, nicht
die uplink-tragenden Modi 2/3). **Die ruhige Basis ist damit als
Boden-/Reduktions-Eigenschaft benannt, nicht als Signal: die Zahlen tragen
keinen Anomalie-Anspruch.**

## Verdikt

**Die ruhige Basis der Floor-Residuen drifft über die Floor-Ära nicht als
gemeinsamer, gleichmäßiger Trend — gemessen ist eine langsame
Mode-1-Niveau-Senkung.** Das ruhige Tages-Niveau (Resid-Mittel/Median) liegt
über alle neun Serien klein-positiv (Serien-Median der Tages-Mittel
+0,06…+0,17 Hz, Mitglieds-Mittel +0,02…+0,29 Hz) und ist damit um eine
Zehntel-Hz von 0 versetzt, nicht auf 0. Über die 27 Monate senkt sich dieses
Niveau in den drei Mode-1-Serien (M1 st43 −8,8×10⁻⁴ Hz/Tag, n 29, p_t 0,0017 /
p_perm 0,0055; M1 st63 −9,0×10⁻⁴, n 34, p_t 0,0004 / p_perm 0,0010;
M1 st14 −3,5×10⁻⁴, p 0,26) um ≈ −0,4 Hz. Diese Senkung ist in der
Monats-Block-Struktur eine **Saison-Lage-Differenz** (1995-11 und 1996 um
+0,1…+0,7 Hz, 1997 ≈ 0 Hz), nicht als gleichmäßige Monats-Rampe auflösbar
(Monats-Median-Streuung ~0,1–0,2 Hz bei Gesamt-Senkung ~0,4 Hz); im
1996-97-Inneren ist sie grenzwertig (p 0,028/0,046). Die Modi 2/3 tragen
keinen signifikanten Niveau-Trend (Vorzeichen gemischt, p_t ≥ 0,08); die
starken M2-Steigungen hängen an einzelnen kohärenten Versatz-Tagen
(ruhige Tage mit ganz-tägigem |Niveau| bis 31,9 Hz bei Tages-RMS < 1 Hz, drei
von 193). **Die ruhige Rausch-Decke (Tages-RMS 0,04–0,09 Hz) driftet nicht
systematisch** (7/9 flach; M1 st43 und M3 st43 positiv, von späteren
höher-ruhigen Tagen nahe der 1-Hz-Grenze getragen). Größenordnung: |Drift|
≤ 0,4 Hz über 27 Monate — unter der 1-Hz-Laut-Schwelle, ein kleiner Bruchteil
der laut-Gipfel-Skala, und unter der ~1-Hz-Größenordnung einer
Pioneer-magnituden Zweiweg-Akzelerations-Rampe auf S-Band; die nicht-monotone,
nur-Mode-1-Struktur benennt die ruhige Basis als Boden-/Reduktions-Eigenschaft.
0 geehrt: die bodenleeren Monate (Feb–Mai 1996, Jul–Aug 1996, Okt 1996)
liegen als leere x-Abstände im Fit, nie als Werte.

**Was `pending` bleibt:** der Sitz der Mode-1-Niveau-Senkung — Uhr (Raumfahrzeug-
Oszillator, One-Way), Ephemeriden-/Geometrie-Term oder Reduktions-/Modell-
Änderung sind auf der Tages-Achse nicht getrennt; die Sub-Tages-/Pass-Struktur
der Niveau-Bewegung und die drei kohärenten Versatz-Tage (ihre Ursache:
Versatz-Klasse bei Tages-RMS < 1 Hz) sind unbenannt. Der ruhige
Basis-Offset selbst (klein-positiv, nicht 0) ist gemessen, nicht erklärt.

## Grenzen

- Die Tages-Serie ist dünn (1–8 ruhige Tage je Monat, 11–34 je Serie) und in
  Läufe von 1–10 Tagen geklumpt; die Permutations-Null bricht die
  Autokorrelation der Läufe (anti-konservativ), die Student-p die
  Nicht-Normalität der Tages-Werte. Beide p liegen dicht beieinander; die
  Monats-Block- und Teil-Körper-Fits tragen die Struktur-Aussage.
- Der OLS gibt jedem Tag gleiches Gewicht; die Zell-n reichen von 30 bis
  > 30 000. Da die Tages-Streuung (0,1–0,5 Hz) die Schätz-Streuung
  (Decke/√n ≪ 0,01 Hz) weit übersteigt, ist die ungewichtete Wahl getragen,
  nicht geglättet.
- Die ruhige-Decke-Steigungen der zwei positiven Serien hängen an wenigen
  späten Tagen nahe der Laut-Grenze; die 1-Hz-Schwelle trennt die Klassen, und
  ein Tag bei RMS 0,996 Hz (M2 st63 1996-01) liegt knapp ruhig — die
  Nähe-Klasse ist benannt, nicht verschoben.
- Drei kohärente Versatz-Tage (|Niveau| bis 31,9 Hz bei ruhigem Tages-RMS)
  sind im Niveau-Fit als Einzel-Tage enthalten und über die
  Median-Achse abgeschwächt; eine getrennte Klasse ist nicht gebildet.
- M1 st63 und M3 st14 tragen je einen Tag mit Tages-RMS exakt 0
  (Zero-Spread-Tag); die log10-Decken-Steigung ist dort über die positiven
  Tage geführt (0 nicht logarithmierbar, 0 geehrt).
- Die Einordnung gegen die Pioneer-Skala ist eine Größen-Rechnung auf dem
  S-Band-Downlink (genannt), kein Fit einer Akzelerations-Rampe.

## Register-Satz

*Die ruhige Basis der Galileo-Floor-Residuen über 1995-11-23..1997-02-28
(400 robuste Zellen, 207 laut/193 ruhig, exakt reproduziert): das ruhige
Tages-Niveau (Resid-Mittel/Median) liegt klein-positiv (+0,06…+0,17 Hz
Serien-Median der Tages-Mittel, Mitglieds-Mittel bis +0,29 Hz) und senkt sich
in den Mode-1-Serien langsam um ≈ −0,4 Hz über 27 Monate (M1 st43 −8,8e-4
Hz/Tag n 29 p_t 0,0017/p_perm 0,0055; M1 st63 −9,0e-4 n 34 p_t 0,0004/p_perm
0,0010; M1 st14 −3,5e-4 p 0,26) — in der Monats-Block-Struktur eine
Saison-Lage-Differenz (1995-11 ~+0,7 Hz, Mitte/Ende 1996 ~+0,2…+0,4 Hz, 1997
~0 Hz), nicht als Monats-Rampe auflösbar; die Modi 2/3 tragen keinen
signifikanten Niveau-Trend (Vorzeichen gemischt, große M2-Steigungen an
kohärenten Versatz-Tagen), die ruhige Rausch-Decke (Tages-RMS 0,04–0,09 Hz)
driftet nicht systematisch (7/9 flach; zwei positive Tendenzen an späten
höher-ruhigen Tagen). |Drift| ≤ 0,4 Hz unter der 1-Hz-Laut-Schwelle und der
laut-Gipfel-Skala — ruhige Basis als Boden-/Reduktions-Eigenschaft, kein
Anomalie-Anspruch; Sitz der Mode-1-Niveau-Senkung und der drei kohärenten
Versatz-Tage (|Niveau| bis 31,9 Hz bei ruhigem RMS) bleiben pending.*

## Status

`draft` (Entwurf für die Haupt-Session/Rat; TODO-Registerzeile ergänzt die
Haupt-Session). Probe `galileo_floor_basis_drift` committet (`cargo check`
0/0, RUSTFLAGS `-D warnings`), Report
`/tmp/opencode/galileo_floor_basis_drift_report.txt`. Richtung D2 (Drift der
ruhigen Basis) ist gemessen; der Sitz der Senkung bleibt pending.
