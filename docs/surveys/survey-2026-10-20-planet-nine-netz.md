<!--
  title: Survey — Planet Neun, Netz/Rangliste: PNRP-3.0-Bande gegen den KBO-Katalog (2026-10-20)
  class: survey
  date: 2026-10-20
  sha256: 329246016cefa7c0708e2d8527d6b042ddabaf364c0d537a20b0e6bea61ae039
  status: live
  see-also: docs/paper/planet-nine-kbo-residue.md tools/measure/src/bin/planet_nine_netz_probe.rs
-->
# Survey — Planet Neun: welcher Faden trägt in der PNRP-Bande?

**Frage.** Nicht „gibt es einen Pfeil?", sondern: Welcher Faden des vorhandenen
Katalogs zeigt in der vorhergesagten Bande der Planet Nine Reference Population
3.0 den stärksten Überschuss — und hält er dem Max-Null über alle Fäden stand?

**Grenze vorab.** Es gibt keine Ephemeride für Planet Neun. Gemessen wird die
breite Ortsbande der PNRP 3.0, nicht ein Punkt am Himmel. Die Bande ist eine
Verteilung, kein Zielort. Ein Überschuss in ihr ist ohne Abdeckungsmodell keine
Entdeckung; die Rotation der Bande über den Himmel ist deshalb der entscheidende
Null, nicht der Katalog-Shuffle.

**Probe.** `tools/measure/src/bin/planet_nine_netz_probe.rs` (neu, dieser
Survey). Das Papier `docs/paper/planet-nine-kbo-residue.md` bleibt unangetastet.

## Bilanz

| | Zahl |
|---|---:|
| PNRP-3.0-Mitglieder | **100.000** |
| KBO-Records im heutigen Asset | **7.195** |
| P9-Auswahl (a ≥ 250 AU, q ≥ 30 AU) | **44** |
| P9-Auswahl in der Bande | **21** |
| Fäden in der Rangliste | **7** |
| Null-Ziehungen je Nullfamilie | **10.000** |
| Fäden über dem Rotations-Max-Null (95 %) | **0** |
| Fäden über dem Shuffle-/Joint-Max-Null (95 %) | **1** (Ω) |
| benannte Risse | **10** |

## 1. PNRP 3.0: Quelle und Bande

Die Bande ist belegt, nicht erfunden:

- **Datensatz:** Brown, M., *Planet Nine reference population, Version 3.0*,
  CaltechDATA, DOI `10.22002/8fjad-x7y61`, publiziert 2023-04-05.
- **Datei:** `reference_population.csv`, 100.000 synthetische Planet-Nine-Orbits,
  gezogen aus den Verteilungen von Brown & Batygin (2021a), Epoche
  **JD 2458270.0 (1. Juni 2018)**, J2000 heliozentrische Bahnelemente; die
  CSV-Spalten `ra`, `dec` sind die Himmelslage zu dieser Epoche.
- **Weitere Dateien:** `README.TXT` (Spaltenlegende), `mag_limit.fits`
  (95-%-Vollständigkeit der kombinierten ZTF/PS1-Suche), `overview_map.pro`
  (Beispielplots). Der Datensatz trägt ZTF-, DES- und PS1-Flags; **kein
  LSST-Flag und keine MPC-Abdeckung**. LSST/MPC ist damit `absent`, nicht null.
- **Bandenkonstruktion dieser Probe:** RA/dec → ekliptikale Länge/Breite J2000
  (Schiefe 23,439291111°), 72 × 36 Zellen zu 5° × 5°, 3 × 3-Binomialglättung
  (Länge zyklisch, Breite am Pol geklemmt), Zellen in absteigender Dichte bis
  **90 %** der PNRP-Masse. Das ergibt **293 Zellen** = **17,4 % der Himmelsfläche**
  und hält **95,2 %** der PNRP (die letzte Zelle überschießt die 90 %; die
  Zellgranularität ist der Grund, nicht ein anderer Konturwert).

PNRP-Diagnostik aus dem Lauf:

| Größe | Wert |
|---|---:|
| ekliptikale Länge, Kreis-Mittel | **66,6°** |
| ekliptikale Länge, Rayleigh R | **0,219** |
| ekliptikale Breite, Mittel | **−1,7°** |
| \|b_gal\| < 10° | **25,8 %** |
| \|b_gal\| < 20° | **45,9 %** |
| ZTF-detektierbar | **56.173 (56,2 %)** |
| DES-detektierbar | **10.187 (10,2 %)** |
| PS1-detektierbar | **68.745 (68,7 %)** |
| irgendein Flag (ZTF∨DES∨PS1) | **78,3 %** |
| LSST / MPC | **absent** (kein Flag im v3.0-CSV) |

Die Aussage „~40 % nahe der galaktischen Ebene" ist damit an der 20°-Definition
bestätigt (45,9 %), an der 10°-Definition schwächer (25,8 %). Beide Zahlen werden
geführt; die Definition wird nicht stillschweigend gewechselt.

## 2. Katalog und P9-Auswahl

- **Katalog:** `kbo_elements.json` aus dem Haus-CDN
  (`ssd-api.jpl.nasa.gov/kbo_elements.json`), 7.195 Records. Das Papier nennt
  7.180; die 15 Differenz wird benannt, nicht rekonstruiert. Kein Record wurde
  von dieser Probe hinzugefügt oder entfernt.
- **P9-Auswahl:** a ≥ 250 AU und q = a(1−e) ≥ 30 AU → **n = 44**, identisch mit
  dem Papier.
- **Kontrollzahl:** Die globale ϖ-Rayleigh der P9-Auswahl ist **R = 0,0581**
  (Mittel 68,1°). Das Papier nennt **5,83 × 10⁻²**; die Winkel-/Katalogkette ist
  damit konsistent. Das ist die einzige Zahl, die aus dem Papier als
  Kontrollwert dient; alle Bandzahlen sind aus dem heutigen Lauf.
- **Himmelslage:** heliozentrische ekliptikale J2000-Position aus den
  oszillierenden Elementen bei der jeweiligen Katalogepoche (überwiegend
  JD 2461200.5). Kein Serienausfall: 0 von 7.195.
- **Bande:** 3.124 von 7.195 Records (**43,4 %**) liegen in der PNRP-Bande;
  von der P9-Auswahl **21 von 44 (47,7 %)**.

## 3. Fäden und Statistik

Sieben Fäden, alle aus dem vorhandenen Katalog, keiner synthetisch:

| # | Faden | Statistik |
|---:|---|---|
| 1 | Perihel-Länge ϖ in der Bande | Rayleigh R der ϖ der P9-Objekte **in** der Bande |
| 2 | Knoten Ω in der Bande | Rayleigh R der Ω der P9-Objekte **in** der Bande |
| 3 | Bahnneigung i | Mittelwert i **in** minus **außerhalb** der Bande |
| 4 | Große Halbachse a | Mittelwert log10 a **in** minus **außerhalb** der Bande |
| 5 | P9-Anreicherung | log[(n_P9_in/n_P9)/(n_all_in/n_all)] |
| 6 | ϖ global | Rayleigh R der ϖ aller 44 P9-Objekte |
| 7 | Ω global | Rayleigh R der Ω aller 44 P9-Objekte |

**Kein V-Test.** Ein V-Test braucht eine einzelne vorhergesagte Richtung. Die
PNRP 3.0 liefert eine Himmelsbande, keinen einzelnen ϖ-Winkel; ein V-Test gegen
den Bandmittelpunkt wäre eine erfundene Vorhersage. Die Richtungsprüfung bleibt
deshalb bei Rayleigh. Das ist eine benannte Methodenwahl, kein weggelassener
Test.

**Residuum/TE.** Der Faden R(t) und TE(R → ϖ) aus dem Papier wird **nicht neu
gerechnet**. Das Papier ist `fam-machine: pre-fix`; seine Zahl ist unten als
Zeile geführt und aus dem Max-Null ausgeschlossen. Die Begründung steht in §7.

**Nicht geführter Faden: LSST/MPC-Abdeckung.** Die PNRP 3.0 trägt ZTF-, DES- und
PS1-Flags, aber **kein LSST-Flag und keine MPC-Abdeckung**. Ein Faden
„Detektionsrate/Abdeckung im PNRP-Band (LSST/MPC)" ist damit `absent` und geht
nicht in Rangliste oder Max-Null ein. Die 78,3 % „irgendein Flag" sind eine
PNRP-Detektierbarkeit, keine KBO-Detektionsrate; sie ersetzen LSST/MPC nicht.

## 4. Null und Max-Null

Drei Nullfamilien, je **10.000** Ziehungen, deterministischer Seed
`0x0D1E9A173C6B0020`:

- **(a) Katalog-Shuffle:** Bahnelemente (a, e, i, Ω, ω, ϖ) werden den
  Himmelspositionen zufällig neu zugeordnet; die P9-Auswahl wird nach der
  Permutation neu berechnet. Die Bande bleibt fest. Das prüft, ob die
  Elemente überhaupt mit der Himmelslage zusammenhängen.
- **(b) Bandrotation:** Die PNRP-Bande wird in ekliptikaler Länge um einen
  uniformen Winkel δ ∈ [0°, 360°) gedreht; die Elemente bleiben fest. Das prüft,
  ob die **vorhergesagte Himmelslage** gegenüber anderen Lagen besonders ist.
- **(c) Joint:** Rotation **und** Shuffle in jeder Ziehung.

Jede Ziehung berechnet **alle sieben Fäden** neu. Die z-Werte eines Fadens
verwenden Mittelwert und Standardabweichung **derselben** Nullfamilie; das
Max-Null ist das Maximum von |z| über alle sieben Fäden pro Ziehung. Angegeben
sind das 95-%- und 99-%-Quantil sowie
`p_look = (#Null-Max ≥ beobachtet + 1) / (N + 1)`.

Die globalen Fäden 6 und 7 haben als Null eine **Uniform-Winkelziehung** (wie im
Papier), nicht die Permutation; die Bandfäden 1–5 haben die jeweilige
Shuffle-/Rotationsnull. Das ist eine bewusste Mischung: der Max-Null ist die
Look-elsewhere-Korrektur über alle Fäden, nicht ein einziges gemeinsames
Nullmodell für alle.

**Empirische Bias-Gewichte:** nicht angewandt. Die Rotationsnull erhält die
beobachtete KBO-Himmelsdichte; sie erhält nicht die Belichtungs-/Airmass-Historie
der Surveys, weil diese im Haus nicht als pro-Survey-Karte vorliegt. Der
galaktische Riss wird in §8 quantifiziert, nicht weggerechnet.

## 5. Ergebnis: Rangliste

Beobachtet gegen Joint-Null (Mittel ± sd), z unter allen drei Nullfamilien:

| Faden | beobachtet | Null-Mittel | Null-sd | z_shuf | z_rot | z_joint | p2_joint |
|---|---:|---:|---:|---:|---:|---:|---:|
| ϖ Rayleigh R in der Bande | **0,3194** | 0,1583 | 0,0881 | 1,75 | 0,43 | **1,83** | 0,051 |
| Ω Rayleigh R in der Bande | **0,5113** | 0,1743 | 0,0936 | 3,43 | 0,67 | **3,60** | 0,002 |
| i-Mittel-Differenz in−out | **−2,9438°** | 0,0277 | 3,1513 | −0,95 | 0,97 | **−0,94** | 0,355 |
| log10 a-Mittel-Differenz in−out | **−0,0458** | −0,0001 | 0,0685 | −0,67 | −0,79 | **−0,67** | 0,521 |
| P9-Anreicherung in der Bande | **0,0946** | −0,0130 | 0,1711 | 0,62 | 1,08 | **0,63** | 0,514 |
| ϖ Rayleigh R global | **0,0581** | 0,1334 | 0,0685 | −1,11 | −1,10 | **−1,10** | 0,275 |
| Ω Rayleigh R global | **0,1006** | 0,1343 | 0,0701 | −0,49 | −0,49 | **−0,48** | 0,652 |

**Rangliste nach Bandüberschuss:** Ω (3,60) > ϖ (1,83) > P9-Anreicherung
(0,63) > a (−0,67) > i (−0,94). Global ist die P9-Auswahl **leiser** als
uniform: ϖ R = 0,0581 gegen Null-Mittel 0,1334; Ω R = 0,1006 gegen 0,1343.
Das reproduziert die Papieraussage für ϖ und erweitert sie auf Ω.

**Nicht in dieser Tabelle:** der R(t)/TE-Faden des Papiers (pre-fix, §6) und der
LSST/MPC-Abdeckungsfaden (`absent`, §3). Beide sind benannt, keiner ist still
durch eine Zahl ersetzt.

**Max-Null über alle sieben Fäden:**

| Nullfamilie | beobachtetes Max | q95 | q99 | p_look | stärkster Faden | Verdikt |
|---|---:|---:|---:|---:|---|---|
| Shuffle | **3,43** | 2,77 | 3,58 | **0,0124** | Ω in der Bande | über q95, unter q99 — explorativ |
| Bandrotation | **1,10** | 2,42 | 3,04 | **0,9050** | ϖ global | unter q95 — kein Überschuss |
| Joint | **3,60** | 2,82 | 3,62 | **0,0105** | Ω in der Bande | über q95, unter q99 — explorativ |

Die entscheidende Zeile ist die **Bandrotation**: Sie fragt, ob die
vorhergesagte Himmelslage gegenüber anderen Lagen besonders ist. Das ist sie
nicht. Das beobachtete Maximum (1,10) liegt unter dem 95-%-Quantil (2,42), und
90,5 % der rotierten Banden erzeugen ein größeres Maximum. Der Ω-Faden steht
unter der Rotation nur bei z = 0,67.

## 6. Verdikt

**Kein robuster Überschuss.** Der stärkste Faden ist Ω in der Bande
(z_joint = 3,60; p2 = 0,002), aber er hält dem Max-Null nicht stand:

- Unter Shuffle und Joint liegt er über dem 95-%-Quantil (p_look ≈ 0,010–0,012),
  aber **unter dem 99-%-Quantil** (3,58 bzw. 3,62).
- Unter der **Bandrotation** fällt er auf z = 0,67; das beobachtete Gesamtmaximum
  (1,10) liegt unter q95 = 2,42, p_look = 0,9050.
- Der Grund ist die bekannte Himmels-/Entdeckungsgeometrie: Für die P9-Auswahl
  korreliert ϖ mit der ekliptikalen Länge λ mit **r = 0,743**, Ω mit λ mit
  **r = 0,239**. Die Bande bei λ ≈ 60° schneidet daher eine Ω-Verteilung, die
  bei Ω ≈ 100–115° häuft. Der Shuffle zerstört diese Korrelation und erzeugt
  Signifikanz; die Rotation erhält sie und findet nichts.

**Wortlaut:** Ω ist **explorativ** unter der Shuffle-/Joint-Null, **nicht** unter
der Bandrotations-Null. Es ist keine Entdeckung. Die PNRP-Bande zeigt in diesem
Katalog **keinen** Faden, der dem Max-Null über alle Fäden standhält.

Der Papier-Faden R(t)/TE wird als **pre-fix, nicht vergleichbar** geführt:

> Papier n = 44: ϖ R = 5,83 × 10⁻² gegen Uniform-Null-Mittel 1,35 × 10⁻¹,
> Schwelle 2,770 × 10⁻¹ → still; fam = 7,2822 × 10⁻¹ (pre-fix), keine Familie
> darüber. Diese Zeile ist übernommen, nicht neu gemessen, und **nicht** im
> Max-Null.

## 7. Post-fix: was vergleichbar ist und was nicht

- Das Papier trägt `fam-machine: pre-fix`. Sein Familienband (fam = 7,2822 × 10⁻¹)
  liegt nach der Papier-Fußnote **höher** als das post-fix-Band; eine Stille unter
  dem höheren Band ist post-fix a fortiori konservativ. Das gilt für den
  Papierlauf, nicht für diesen Survey.
- Dieser Survey benutzt **keine** Phasen-Randomisierung und **keine** TE. Sein
  Null ist Permutation/Rotation. Er ist damit **kein Post-fix-Rerun** des
  Papiers und darf nicht als solcher gelesen werden.
- Der Winkel-/Katalogteil ist vergleichbar: Die globale ϖ-Rayleigh ist
  R = 0,0581 gegen die Papierzahl 0,0583. Der TE-Teil ist es nicht.
- Keine stillen Zahlenübernahmen: Die Papierzahlen stehen nur in der zitierten
  Pre-fix-Zeile; die Bandzahlen sind ausschließlich aus dem heutigen Lauf.

## 8. Bias-Risse (was die Messung nicht trägt)

1. **Galaktische Ebene.** Die PNRP-Bande liegt zu **25,8 %** innerhalb
   |b_gal| < 10° und zu **45,9 %** innerhalb 20°; der Katalog liegt dort nur zu
   **4,7 %** bzw. **14,6 %** (P9: 4,5 % / 11,4 %). Die Bande enthält einen
   galaktischen Streifen, den der Katalog kaum abdeckt. Ein Überschuss dort wäre
   ein Abdeckungsartefakt, keine Detektion.
2. **Himmels-/Element-Korrelation.** ϖ–λ = 0,743, Ω–λ = 0,239. Die
   In-Band-ϖ-Häufung (R = 0,3194, Mittel 3,1°) ist weitgehend eine
   Himmelsauswahl; der Shuffle-Null macht daraus scheinbare Signifikanz, die
   Rotation nicht.
3. **Die Bande ist breit.** 17,4 % der Himmelsfläche, 293 Zellen, 95,2 % der
   PNRP. Ein P9-Anteil von 47,7 % in der Bande ist für sich genommen kein
   Signal; die Fläche ist zu groß für eine „Pencil-Beam"-Aussage.
4. **Kleines n.** 44 P9-Objekte, 21 in der Bande. Rayleigh R aus 21 Winkeln hat
   ein breites Nullband; die Rangliste ist entsprechend empfindlich.
5. **Epochenversatz.** PNRP-Epoche JD 2458270.0 (2018), Katalogepochen
   überwiegend JD 2461200.5 (2026). Planet Neun bewegt sich bei ~500 AU in
   8 Jahren um ~0,2°; vernachlässigbar, aber nicht null.
6. **Frame-Riss.** Die CSV nennt `ra`/`dec` ohne geozentrisch/heliozentrisch;
   diese Probe konvertiert sie nach ekliptikal und vergleicht mit
   heliozentrischen KBO-Positionen. Für 40 AU sind das bis ~1,4°, für 500 AU
   ~0,2° — unter der 5°-Zelle, aber benannt.
7. **LSST/MPC fehlt.** Das v3.0-CSV trägt nur ZTF/DES/PS1. Die im Auftrag
   genannte LSST-North-Ecliptic-Spur-Abdeckung ist **nicht** im Asset und wird
   nicht erfunden. Ein Detektionsraten-Faden „LSST/MPC im PNRP-Band" ist
   `absent`; die 78,3 % „irgendein Flag" sind kein LSST-Ersatz.
8. **Keine Bias-Gewichte.** Die Rotationsnull erhält die KBO-Himmelsdichte,
   nicht die Airmass-, Wetter- und Pointing-Historie der Surveys. Es gibt im
   Haus keine pro-Survey-Exposure-Karte für diese Bande.
9. **Der Katalog ist entdeckungsbiasiert.** Das Papier korrigiert keine
   Selektionsfunktion; dieser Survey erbt diese Grenze. Die 43,4 %
   „alle KBOs in der Bande" sind eine Himmelsdichte-Aussage, keine
   Detektionsrate.
10. **Katalogumfang.** 7.195 Records heute gegen 7.180 im Papier. Die 15
    Records werden nicht rekonstruiert; die P9-Auswahl bleibt bei n = 44.

## 9. Reproduktion

```bash
# PNRP 3.0 (CaltechDATA, DOI 10.22002/8fjad-x7y61)
curl -sSfL -o /tmp/opencode/p9/reference_population.csv \
  https://data.caltech.edu/records/8fjad-x7y61/files/reference_population.csv
curl -sSfL -o /tmp/opencode/p9/PNRP_README.txt \
  https://data.caltech.edu/records/8fjad-x7y61/files/README.TXT

# KBO-Katalog (Haus-CDN, ssd-api.jpl.nasa.gov)
curl -sSfL -o /tmp/opencode/p9/kbo_elements.json \
  https://github.com/omegaflow/sources/releases/download/ssd-api.jpl.nasa.gov/kbo_elements.json

cargo build -p omegaflow-measure --bin planet_nine_netz_probe
./target/debug/planet_nine_netz_probe \
  --kbo /tmp/opencode/p9/kbo_elements.json \
  --pnrp /tmp/opencode/p9/reference_population.csv \
  --draws 10000
```

Seed `0x0D1E9A173C6B0020`; 10.000 Ziehungen je Nullfamilie; Laufzeit ~3,5 min
auf dem Referenzrechner. Der Lauf ist deterministisch; die Ausgabe ist oben
zitiert, nicht abgeschrieben.

## 10. Ablage

- Probe: `tools/measure/src/bin/planet_nine_netz_probe.rs` (neu).
- Dieser Survey: `docs/surveys/survey-2026-10-20-planet-nine-netz.md`.
- Papier `docs/paper/planet-nine-kbo-residue.md` wurde **nicht** überschrieben.
- `phi/sources.φ` wurde **nicht** angefasst; keine CDN-Uploads.

## Anhang: P9-Auswahl in der Bande (21 Objekte)

Ekliptikale Länge λ/Breite β aus der heliozentrischen Position; b_gal aus der
ICRS-Position. Sortiert nach Ω.

| Objekt | Ω | ϖ | i | a (AU) | λ | β | b_gal |
|---|---:|---:|---:|---:|---:|---:|---:|
| (2025 MB352) | 6,9° | 316,8° | 13,8° | 268,0 | 330,5° | −8,3° | −55,0° |
| (2014 SR349) | 34,9° | 16,6° | 18,0° | 304,1 | 339,8° | −14,9° | −66,1° |
| 474640 Alicanto (2004 VN112) | 66,0° | 33,3° | 25,5° | 342,1 | 58,4° | −3,6° | −29,0° |
| (2013 RF98) | 67,7° | 19,9° | 29,5° | 375,8 | 56,7° | −6,1° | −31,8° |
| (2012 VP113) | 90,9° | 25,2° | 24,0° | 267,5 | 56,7° | −14,0° | −35,6° |
| 765133 (2013 SL102) | 94,8° | 0,5° | 6,5° | 321,8 | 32,5° | −5,8° | −51,1° |
| 765047 (2013 RA109) | 104,9° | 8,0° | 12,4° | 480,1 | 37,2° | −11,5° | −51,0° |
| (2021 RR205) | 108,5° | 317,1° | 7,6° | 949,0 | 355,7° | −7,1° | −66,9° |
| (2019 EU5) | 109,1° | 218,0° | 18,2° | 1341,3 | 136,9° | 8,7° | 44,6° |
| 523622 (2007 TG422) | 113,0° | 38,9° | 18,6° | 555,3 | 84,0° | −9,2° | −9,8° |
| (2015 JB13) | 113,8° | 271,4° | 11,9° | 304,6 | 213,4° | 11,7° | 54,1° |
| (2014 WB556) | 115,0° | 350,5° | 24,1° | 284,5 | 34,5° | −23,9° | −58,8° |
| (2010 GB174) | 130,6° | 118,0° | 21,5° | 360,2 | 194,1° | 19,5° | 73,6° |
| 90377 Sedna (2003 VB12) | 144,5° | 95,6° | 11,9° | 543,7 | 61,3° | −11,8° | −30,7° |
| (2015 DM319) | 165,9° | 209,0° | 6,8° | 278,7 | 254,9° | 6,8° | 16,5° |
| (2016 SA59) | 174,8° | 15,4° | 21,5° | 252,6 | 54,0° | −18,7° | −40,0° |
| (2014 TU115) | 192,3° | 57,8° | 23,5° | 360,2 | 54,8° | −16,4° | −38,3° |
| (2018 VM35) | 192,4° | 135,7° | 8,5° | 307,0 | 89,2° | −8,3° | −4,8° |
| (2015 KG163) | 219,1° | 251,1° | 14,0° | 640,2 | 258,0° | 8,9° | 14,8° |
| (2021 DK18) | 322,3° | 196,6° | 15,4° | 859,3 | 129,2° | 3,6° | 35,3° |
| (2025 OY736) | 350,2° | 303,3° | 11,3° | 312,4 | 337,2° | −2,6° | −55,2° |

Die Häufung bei Ω ≈ 100–115° (neun Objekte) ist der stärkste Einzelbeitrag zum
Ω-Faden. Sie ist im Katalog real, aber sie ist an die Himmelslage gekoppelt und
überlebt die Bandrotation nicht.
