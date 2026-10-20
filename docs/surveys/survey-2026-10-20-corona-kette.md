<!--
  title: Survey — die Corona-Kette: 304→131 Å im Anschluss an Flare, CME und Funk
  class: survey
  date: 2026-10-20
  sha256: e4170e0c714c77840e2bd3bc36bb1a6a345a8264b3375e3016fc33c9bdef5906
  status: live
  see-also: docs/paper/corona-heating-ladder.md docs/surveys/survey-2026-10-20-sonne-erde-kette.md
-->
# Survey — die Corona-Kette: 304→131 Å im Anschluss an Flare, CME und Funk

**Anlass.** Das Papier `corona-heating-ladder.md` misst die heiße Kette
193→211→335→94 Å unter dem Familienband; konditioniert kollabiert sie. Was
überlebt, ist der Übergangsregion-Eintritt **304→131 Å** (upward,
bandbreitenstabil, drei Jahre). Dieser Survey fragt nicht noch einmal nach der
Richtung der Sprosse, sondern nach ihrem **Anschluss**: Lässt sich der Faden auf
gemeinsamen Tagen an Flare/GOES-XRS, CME/LASCO und Funk koppeln — und trägt er
damit als Heizungs-Sprosse in die Sonne→Erde-Kette? Gemessen wird Korrelation
und Transferentropie mit Null und Vorlauf; eine Korrelation ist keine Ursache.

**Messgrundlage.** AIA-2013/14/15 (7 Bänder, 12 s), GOES-15 XRS 2 s,
CDAW/LASCO-CME-Liste, SWPC-Funkereignisse. Die Reihen sind im Haus; die
Korpora wurden aus den eigenen Releases geladen, nicht neu geerntet (Ausnahme:
die kleinen monatlichen LASCO-Textlisten 2013–2015, je ~20 kB). Kein Wert ist
aus dem Register abgeschrieben; die Inventur ist am Asset gemessen.

## 1. Inventur — was liegt, in welcher Kadenz, für welchen Zeitraum

Gemessen mit `corona_inventory_probe` an den Haus-Assets (Median-Δt je Linie,
erster/letzter UTC-Tag, Record-Zahl), nicht aus dem Register übernommen. Die
τ-Spalte des Registers ist bei mehreren Reihen gröber als das Asset.

| Reihe | Kanäle / Linien | native Kadenz | Asset-Zeitraum (gemessen) | Lizenz |
|---|---|---|---|---|
| **SDO/EVE L2b** | 11 Linien: 94, 131, 171, 195, 211, 284, 304, 335, 584, 977, 1032 Å | Papierlauf **10 s**; Haus-Asset **60 s** (Median Δt) | 2011-02-09 … 2011-05-10; 1 146 616 Records (davon 129 795 Quad-Diode); MEGS-A-Linien je 128 955, MEGS-B-Linien (335/584/977/1032) je **28 534** (≈ 19,8 d) | `other-attribution` (LASP EVE data access), **nicht CC0** |
| **SDO/AIA** | 7 Bänder: 94, 131, 171, 193, 211, 304, 335 Å | **12 s** (Median Δt) | je ein `*_fullyear.bin` für 2013/14/15; 2014: 18 176 977 Records, 2 593 153 (304) / 2 594 490 (131), 366 Tage (TDB-Tagesgrenze 2013-12-31 … 2014-12-31) | **CC0-1.0** (JSOC) |
| **GOES-15 XRS** | XRS-A (0,05–0,4 nm), XRS-B (0,1–0,8 nm) | **2 s** | `goes15_xrs_2s_2013/14/15.tar`; 2014: 357 Tagesdateien, 14 732 199 gute B- und 13 387 819 gute A-Samples | **PD-USGov** |
| **GOES EUVS** | Lyman-α 1216 Å (irr_1216) | **60 s** nativ (avg1m), Register-Bucket 1 a | g14 2009-09-01 … 2020-03-04; g15 2010-03-25 … 2020-03-04 | **PD-USGov** |
| **SOHO/LASCO CME** | Geschwindigkeit, Breite, PA, Beschleunigung, Masse, Energie | **Ereignis** (erste C2-Sichtung) | Archiv 1996 … heute; Haus-Asset `soho_lasco_cme.bin` nur **193 Records ab 2024-01-01** (rollierend). Für die Messung: 12 Monatslisten 2013/14/15 vom CDAW, 2014 = 2 478 CMEs | **PD-USGov** |
| **SWPC Radio** | Typ II/III: Frequenzband, Dauer, Typ | **Ereignis** | 34 Sweeps, **2026-09-24 …** (rollierend ~25 d); **kein Überlapp mit 2013–2015** | **PD-USGov** |
| **e-CALLISTO/MRO** | 9 Bänder 50–846 MHz | 15-min-Stempel, native **0,25 s** | Stichprobe 2018–2025: 325 Tage, 16 855 Dateien, 4,515 GB (jeder 9. Tag); **beginnt 2016**, kein Überlapp mit AIA 2013–2015; Asset **zurückgezogen** | PI-Kontakt / Nennung, **nicht redistribuierbar** bis Antwort |

Weitere Sonnenreihen im Haus (SOHO/EIT 195 Å 30 d, SOHO/SEM täglich, F10.7
täglich) tragen die Temperaturleiter nicht und bleiben hier außen vor.

**Der erste Riss steht schon in der Inventur.** Das Papier misst EVE auf einem
10-s-Raster (5 862 322 Linien-Records über 90 Tage); das CDN-Asset
`eve_l2b.bin` trägt gemessen **60 s** und 1 146 616 Records. Es ist damit keine
byte-genaue Reproduktion des Papierkorpus, sondern eine gröbere Ableitung.
Zusätzlich fehlen die vier MEGS-B-Linien nach ~20 Tagen: 335/584/977/1032 Å
haben nur 28 534 Records, die MEGS-A-Linien 128 955. Die volle 11-Linien-Leiter
liegt im Haus also nur für ein knappes Viertel des Papierfensters.

## 2. Anschluss — Messung auf gemeinsamen Tagen

**Werkzeug.** `tools/measure/src/bin/corona_chain_probe.rs`. AIA 304 und 131 Å
auf 24-s-Zellen; Flare-Ereignisse aus GOES-15 `b_flux > 5e-6 W/m²` (C5.0),
30 min Refraktärzeit, ±40 min Fenster, mindestens 100 vollständige Zellen. Je
Ereignis der normierte gerichtete Exzess

`D96 = [TE(304→131; 96 s) − TE(131→304; 96 s)] / (|TE(304→131)| + |TE(131→304)|)`

mit 5 phasenrandomisierten Surrogaten je Ereignis (deterministischer Seed).
Tagesziel ist das Mittel der D96 der Ereignisse dieses Tages. Treiber sind
GOES-B/A-Tagesmaxima, die C5-Flare-Zahl, LASCO-CME-Zahl und -Maximalgeschwindigkeit
sowie SWPC-Funk. Gemeinsame Tage, Vorlauf −3 … +3 d (Lead > 0: Treiber
führt), Block-Shift-Null (500 zirkuläre Verschiebungen, mindestens 7 d) und
Tages-TE (0 … 3 d, 20 Phasen-Surrogate, Lücken übersprungen). Das Tagesziel
existiert nur an Tagen mit Flare-Ereignissen;
die Kopplung ist damit selbst eine Flare-Tag-Auswahl.

### 2.1 Der Faden selbst — drei Jahre

| Jahr | Ereignisse | D96 Mittel | positiv | Phase-Null 95 % | GOES-Dateien |
|---|---:|---:|---:|---:|---:|
| 2013 | 525 | **+8,4526e-2** | 332/525 (63,2 %) | +1,8792e-1 | 363 |
| 2014 | 1 348 | **+2,7920e-2** | 763/1 348 (56,6 %) | +1,6841e-1 | 357 |
| 2015 | 614 | **+4,6072e-2** | 357/614 (58,1 %) | +1,8789e-1 | 349 |

Die Richtung reproduziert: D96 ist in allen drei Jahren positiv, mit
Mehrheit der Ereignisse. Die Amplitude bleibt aber unter dem Per-Ereignis-
Phase-Null (95 %) — dasselbe Bild wie im Papier: Richtung gemessen, Signifikanz
unter der Schwelle. Der Faden ist keine fam-clearing Sprosse.

### 2.2 Kopplung an die Kette — 2014 (Hauptlauf)

| Treiber | n Tage | r (best) | Lead (d) | Null 95 % | p | TE(0 d) | TE-Null 95 % |
|---|---:|---:|---:|---:|---:|---:|---:|
| GOES-XRS B max | 232 | −0,1352 | −3 d | 0,1928 | 0,311 | +1,172e-1 | +2,594e-1 |
| GOES-XRS A max | 232 | −0,1300 | −3 d | 0,2067 | 0,313 | +9,762e-2 | +2,548e-1 |
| Flare-Zahl (≥ C5) | 239 | **−0,1991** | 0 d | 0,1771 | **0,026** | +1,589e-1 | +2,585e-1 |
| LASCO CME-Zahl | 238 | −0,1130 | +2 d | 0,1842 | 0,408 | +2,073e-1 | +2,663e-1 |
| LASCO CME max. Speed | 237 | −0,1146 | +2 d | 0,1668 | 0,387 | +2,061e-1 | +2,533e-1 |

Kein Treiber trägt positiv über seinen Block-Shift-Null. Der einzige nominelle
p < 0,05 ist **negativ** (mehr C5-Flares am Tag → kleineres mittleres D96) und
im nächsten Jahr nicht reproduziert (siehe 2.3). Die Tages-TE liegt für jeden
Treiber unter ihrem Phasen-Null; Funk fehlt auf den gemeinsamen Tagen.

### 2.3 Drei Jahre — die Kopplung ist nicht stabil

| Jahr | GOES-B r (Vorlauf, p) | Flare-Zahl r (Vorlauf, p) | CME-Zahl r (Vorlauf, p) | CME-Speed r (Vorlauf, p) |
|---|---|---|---|---|
| 2013 | +0,1317 (+3, 0,574) | +0,1574 (+3, 0,398) | +0,1867 (−2, 0,201) | +0,1880 (+1, 0,185) |
| 2014 | −0,1352 (−3, 0,311) | −0,1991 (0, 0,026) | −0,1130 (+2, 0,408) | −0,1146 (+2, 0,387) |
| 2015 | +0,1115 (+3, 0,745) | **−0,2953** (0, **0,012**) | −0,1246 (+2, 0,671) | +0,1616 (+1, 0,203) |

Die Flare-Zahl-Kopplung wechselt das Vorzeichen zwischen den Jahren (+0,157 /
−0,199 / −0,295); sie ist damit kein reproduzierbarer Zug der Kette, sondern
eine jahresabhängige Amplitude — am ehesten eine Verdünnung: an Tagen mit
vielen Flares geht mehr Gewicht in das Ereignismittel, und die einzelnen
D96 streuen um die Null. Kein Vorlauf ist positiv stabil; kein r kreuzt in
allen drei Jahren denselben Null.

### 2.4 Positivkontrolle — sieht die Maschine die bekannte Kopplung?

Dieselbe Tagesmaschine, Ziel = AIA-Tagesmaximum statt D96:

| Jahr | Ziel | n | r (best) | bester Lead | Null 95 % | p |
|---|---|---|---:|---:|---:|---:|
| 2013 | AIA 131 max | 363 | **+0,8666** | 0 | 0,2593 | **0,012** |
| 2013 | AIA 304 max | 362 | −0,0786 | +1 d | 0,2025 | 0,562 |
| 2014 | AIA 131 max | 357 | **+0,5717** | 0 | 0,2196 | **0,006** |
| 2014 | AIA 304 max | 356 | −0,0957 | +1 d | 0,1736 | 0,641 |
| 2015 | AIA 131 max | 349 | **+0,6621** | 0 | 0,2509 | **0,006** |
| 2015 | AIA 304 max | 348 | −0,0562 | −2 d | 0,2398 | 0,753 |

Die GOES→131-Kopplung ist stark und über dem Null; die 304-Vollscheibe ist
still (dieselbe ~10× Flare-Dämpfung, die das Papier beschreibt). Die Maschine
sieht also die bekannte Kette. Die D96-Stille aus 2.2 ist damit kein
Instrumentenausfall. Der Lead der 131-Kontrolle ist 0; der der 304-Zeile ist
bei r ≈ −0,06 bedeutungslos. Getragen wird die Amplitude, nicht ein Tag.

## 3. Was der Befund trägt — und was nicht

- **Trägt:** Der Faden 304→131 Å ist in drei Jahren positiv und mehrheitlich
  positiv; er ist ein reproduzierbarer Richtungsbefund unter der Schwelle, wie
  im Papier. Die Messtages-Kette ist intakt (Positivkontrolle).
- **Trägt nicht:** Eine Kopplung des Fadens an Flare/GOES-XRS, CME/LASCO oder
  Funk auf gemeinsamen Tagen. Alle Tages-TE liegen unter dem Phasen-Null; die
  einzigen p < 0,05 sind negativ und nicht reproduziert. Funk ist für 2013–2015
  **absent**, nicht null.
- **Nicht behauptet:** Kausalität. Der Vorlauf ist eine gemessene Verschiebung,
  kein Mechanismus. Das Tagesziel ist ein Ereignismittel; die Flare-Zahl-Kopplung
  ist teilweise Auswahl (nur Flare-Tage tragen ein Ziel).
- **Nicht gemessen:** Die Kopplung des *konditionierten* Fadens (D|C, Papier
  §4.6) an CME/Funk. Hier steht der unbedingte Faden; die GOES-Konditionierung
  würde den Flare-Arm definitionsgemäß entfernen. Diese Messung ist `pending`.

## 4. Was fehlt, um die Kette wirklich zu tragen

Geschätzt, nicht geerntet. Volumen sind Rohdaten-Näherungen; die Bin-Assets
wären um ~Faktor 10–100 kleiner.

| Fehlt | Warum es fehlt | Volumen (Schätzung) | Lizenz | Entscheidung |
|---|---|---|---:|---|
| **AIA-Active-Region-Cutouts, 12 s, 7 Bänder** | Vollscheibe dämpft den Flare ~10×; die Leiter misst sonst Sonne-als-Stern | 512²×16 bit ≈ 0,5 MB/Frame; 7 Bänder × 7 200/Tag ≈ **26 GB/Tag**; ~1 000 Flare-Fenster/Jahr ≈ **1,4 TB/Jahr** roh (256²: ~0,4 TB/Jahr) | CC0-1.0 | **nicht geerntet** — nächste Messung, nicht dieser Survey |
| **EVE L2b auf 10 s, volle 11 Linien** | Haus-Asset ist 60 s; MEGS-B-Linien nur ~20 d; MEGS-A 2014 degradiert | 10 s ≈ 6,9 M Records/90 d ≈ **138 MB** Bin; Roh-FITS ~**2–5 GB** | `other-attribution` (LASP) | **nicht geerntet**; 2014 ohnehin nicht heilbar |
| **Historisches SWPC-Typ-II/III-Archiv 2013–2015** | Die rollierende Liste deckt nur ~25 d; kein Überlapp mit AIA | Ereignisliste, **wenige MB** | PD-USGov | **nicht geerntet** — kein Compiler im Haus |
| **e-CALLISTO/MRO für 2013–2015** | MRO beginnt 2016; Asset zurückgezogen; 15-min-Stempel, 0,25 s | 4,515 GB / 325 Tage; volle 8 a ≈ **40 GB** roh | PI-Kontakt, nicht redistribuierbar | **absent by construction** — kein Überlapp |
| **Ersatz für MEGS-A nach 2014** | EVE-MEGS-A verlor Empfindlichkeit; keine 10-s-Sonnenlinien mehr | nicht beschaffbar | — | **absent**; AIA-Imaging bleibt der einzige 12-s-Vollpfad |

Die Kadenz ist nicht der Engpass: 12 s löst die ~96-s-Überquerung mit acht
Zellen auf. Der Engpass ist die **räumliche Auflösung** (Vollscheibe) und ein
**lizenzierter Funk-Überlapp** mit der AIA-Ära.

## 5. Ablage und Reproduktion

Korpora (nicht im Repo): `/tmp/opencode/corona-chain/` — `aia201{3,4,5}_fullyear.bin`,
`goes15_xrs_2s_201{3,4,5}.tar`, `eve_l2b.bin`, `swpc_radio_bursts.bin`,
`lasco201{3,4,5}/univ*.txt` (CDAW-Monatslisten). Die AIA/GOES/EVE/SWPC-Assets
kommen aus den eigenen GitHub-Releases; die LASCO-Monatslisten sind die einzige
neue, kleine Ernte (Ereignis-Text, kein Rohbild).

```
target/release/corona_inventory_probe --eve eve_l2b.bin --aia aia2014_fullyear.bin
target/release/corona_chain_probe \
  --aia aia2014_fullyear.bin --goes-dir goes15 \
  --lasco-dir lasco2014 --radio swpc_radio_bursts.bin
```

## 6. Risse (benannt, nicht geglättet)

- **EVE-Kadenz:** Papier 10 s, Haus-Asset 60 s; MEGS-B-Linien nur 28 534
  Records. Die Inventur korrigiert die Register-Erzählung, nicht das Papier.
- **Funk:** Für 2013–2015 gibt es im Haus **keinen** gemeinsamen Tag. SWPC ist
  rollierend, MRO beginnt 2016, das MRO-Asset ist zurückgezogen. Die Funk-Zeile
  der Kopplung ist `absent`, keine Null.
- **LASCO:** Das Haus-Asset ist rollierend (2024+); die 2013–2015-Listen wurden
  für diese Messung monatlich geholt. Fehlende Monate (2013-01, 2015-06/07/08)
  wurden nachgeladen; ein früherer Lauf mit Lücken ist verworfen.
- **Tagesziel:** D96 existiert nur an Flare-Tagen; die Kopplung ist eine
  bedingte Auswahl. Die negative Flare-Zahl-Korrelation ist am ehesten
  Amplitude/Verdünnung, nicht Physik.
- **Positivkontrolle:** Die 131-Kontrolle sitzt bei Lead 0; der Lead der
  304-Zeile ist bei r ≈ −0,06 bedeutungslos. Die Aussage ist die Amplitude
  (r = 0,57–0,87 über dem Null), nicht ein Tag.
- **Schätzer:** D96 ist der unbedingte KDE-Pfad; das Papier liest den
  konditionierten Pfad. Die beiden Zahlen sind nicht direkt vergleichbar.
- **Ein Korpus je Jahr:** Kein zweiter unabhängiger AIA-Korpus, keine
  räumliche Auflösung; die Kette bleibt eine Vollscheiben-Aussage.
- **Eine Verdichtung:** Der Faden ist als Tagesmittel des normierten D96
  gemessen. Tagesmaximum, Ereignis-Peak oder eine andere Verdichtung sind
  nicht gemessen; die Kopplungsaussage gilt für diese eine Verdichtung.

---

*Daten und Code:* `tools/measure/src/bin/corona_inventory_probe.rs` (Inventur),
`tools/measure/src/bin/corona_chain_probe.rs` (Anschluss), Papier
`docs/paper/corona-heating-ladder.md` (nicht überschrieben), Survey
`docs/surveys/survey-2026-10-20-sonne-erde-kette.md` (Knotenbestand).
Alle Zahlen sind Maschinenausgaben der drei Läufe 2013/2014/2015; 0 honored.
