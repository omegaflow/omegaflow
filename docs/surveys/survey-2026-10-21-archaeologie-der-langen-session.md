<!--
  title: Survey — Archäologie der langen Session: was gemessen ist und was offen blieb (2026-10-21)
  class: survey
  date: 2026-10-21
  sha256: dac923cf0fef7f6f81ee36ecffdc2c341387eba2e59b9ea767aac91cc0f603de
  status: live
  see-also: docs/surveys/survey-2026-10-20-frame-wache.md docs/surveys/survey-2026-10-20-planet-nine-netz.md docs/surveys/survey-2026-10-20-hunga-tonga-netz.md docs/surveys/survey-2026-10-20-sturm-komposit.md docs/surveys/survey-2026-10-20-asi-stationshoehen.md docs/concepts/die-weberin.md
-->
# Survey — Archäologie der langen Session

**Zweck.** Diese Archäologie ordnet, was in der langen Session besprochen, angefangen
und tatsächlich gemessen wurde. Sie ist **keine Neumessung**: jede Zeile verweist auf
eine Datei, ein Kommando oder einen Lauf. Wo die Erzählung der Session und die Datei
abweichen, gilt die Datei; die Abweichung wird benannt, nicht geglättet.

**Ausgangspunkt.** Die Session bewegte sich zwischen vier Polen: dem **Stein im Wasser**
(`field_te_query`), dem **Vlies/der Weberin** (ICRS·TDB als ein Feld), den **Proben**
(Planet Neun, Hunga Tonga, Quasare/ICRF3, Mond, Sturm) und der **Ernte** (Sonne-Erde,
Aurora, e-CALLISTO). Der rote Faden ist die Frage, ob der Rahmen selbst trägt — und
die Antwort ist messbar geworden.

**Neben der Weberin.** Die Weberin beschreibt die **Anordnung** (Kette, Schuss, Vlies);
diese Archäologie beschreibt den **gemessenen Bestand** dieser Anordnung. Sie ersetzt
kein Konzept und keine Messung; sie sagt, was davon fertig ist und was nicht.

## 1. Die Landkarte

| Thema | Status | gemessene Kernzahl | Ort | offen |
|---|---|---|---|---|
| **ICRS/Frame-Wache** | **gemessen** | 4.536/824/678 Quellen; 600 Dreifach-Anker; 171 D×3; 303 D in S/X (193 K / 176 X/Ka); Median 2,930σ; Rahmenrotation < 2σ → **Placed** | `survey-2026-10-20-frame-wache.md` | Gaia DR4, Eigenbewegung, 88 Anker ohne Gaia |
| **Quasare/H₀** | Quellen geprüft, eigene Wägung offen | H0LiCOW 73,3; TDCOSMO 67,4/71,6; D/H geprüft | `projects/quasare.md` | Zeitverzögerung nachrechnen, `r_d`, Mass-Sheet |
| **Planet Neun** | **gemessen** | 44 P9-Objekte, 21 in der Bande; Ω z=3,60, aber Bandrotation p=0,905 → **kein robuster Überschuss** | `survey-2026-10-20-planet-nine-netz.md` | LSST/MPC absent, Bias-Gewichte, 10 benannte Risse |
| **Hunga Tonga** | **gemessen** | 6 Netze; Seismik 3,87 km/s; Druck 0,328; Tsunami 0,170; TEC −57 %; GLM-Cluster | `survey-2026-10-20-hunga-tonga-netz.md` | Thermik am Vent gemessen absent (FDCF Fill, FIRMS 0); Aerosol keyless (AODF/AER_AI); TEC 15-min |
| **Stein im Wasser** | Matrix gebaut+gemessen | 15/15 Arme, 210/210 Zellen; Newell→OMNI still (TE 4,5969e-3, p 1,0) | `survey-2026-10-05-stoerungs-experiment-fehlende-faeden.md` | `ozzy` fehlt; Alles-gegen-alles nicht gefahren; Transitivität Konzept |
| **Mond-Faden** | gebaut+gemessen | bins 3: NVE→IVE 0/36, IVE→NVE 3/36 (MAX-NULL ja); roh 6/36 | `survey-2026-10-18-mond-faden.md` | 5-Kond-Baseline vs. +lunar getrennt; keine Mond-Aussage |
| **Signal-cone** | **gemessen** | τ=2 d, d/c=8,155 min, Positivkontrolle **BESTANDEN** | `survey-2026-10-19-signal-cone-positivkontrolle.md` | Kontrolle synthetisch; reale Tabelle leer |
| **Aurora/ASI** | **gebaut, Höhen gemessen** | 24/24 Stationen belegt (± ≤ 10 m); MCGR-Metadatenfehler | `survey-2026-10-20-asi-stationshoehen.md` | Kamera-/SSUSI-Kreuzprobe; **kein Neubau nötig** |
| **Sturm-Komposit** | **gemessen** | 166 Ereignisse; Dst 6,45 \|z\|, Sym-H 6,02, AL 5,92, AE 5,87; Bz 3,09 bei −1 h | `survey-2026-10-20-sturm-komposit.md` | GIC nur 118/166; ECOD am Lagrand; Dst definitorisch |
| **Sonne-Erde/Funk** | teils gemessen | e-CALLISTO 276 FITS; OVRO-LWA 1,94 GB; Zwillings-Test unentschieden | `projects/sonne-erde.md` | `lower_band.cfg`, Digits↔Jy |
| **Flyby/Ephemeriden** | Kette gebaut, Fill offen | Kanäle registriert; RTSW 24-h-Frist | `projects/flyby.md`, `projects/ephemerides.md` | Fill-Run, JUICE-Perigäum, LLR-Konto |

## 2. ICRS — der universelle Abgleich

Die Frage „können wir prüfen, ob unser Koordinatensystem stimmt?" sind **zwei** Fragen.

### 2.1 Stimmt unsere Kopie?

Ja, und das ist ab jetzt automatisch prüfbar. Die drei Kataloge liegen keyless und
mit gemessenem Fingerabdruck:

| Katalog | URL | Bytes | sha256 |
|---|---|---:|---|
| ICRF3 S/X (4.536) | `https://hpiers.obspm.fr/icrs-pc/icrf/icrf3sx.txt` | 745.282 | `83abfa7c…bab3bb` |
| ICRF3 K (824) | `https://hpiers.obspm.fr/icrs-pc/icrf/icrf3k.txt` | 136.511 | `ef653890…df49b9a` |
| ICRF3 X/Ka (678) | `https://hpiers.obspm.fr/icrs-pc/icrf/icrf3xka.txt` | 112.570 | `22e89235…a27aa19` |
| Gaia-CRF3 Crossmatch | `https://cdsarc.cds.unistra.fr/ftp/cats/I/355/gcrf3xm.dat.gz` | 57.852.137 | `9ea2ac57…531300` |
| Gaia-Positionen (TAP) | `I/355/gaiadr3`, 512 IDs | 33.004 | `e4a9bff3…54f1e7` |

Die Köpfe nennen 4.536/824/678 Quellen; die geparsten Datenzeilen sind **4.536/824/678**.
Kein Kanal fehlt, kein Kopf wird ungeprüft übernommen. Der Kopie-Check ist ein
`sha256sum` gegen diese Zeilen; weicht er ab, war es nicht die Datei, mit der die
Wache lief.

### 2.2 Stimmt die Vermessung?

Ein Koordinatensystem ist eine Vereinbarung; „wahr" kann es nicht sein. Prüfbar ist,
ob die unabhängigen Messungen desselben Leuchtturms zusammenpassen. Der eigene Lauf:

| Größe | Wert |
|---|---:|
| Dreifach-Schnitt (in allen drei Bändern) | **600** |
| davon D in allen drei Bändern | **171** |
| Gaia-Abdeckung | **512 / 600** (88 ohne) |
| Median-σ S/X–X/Ka (2D, korreliert) | **2,930** |
| \>5σ S/X–X/Ka (korreliert) | **111** |
| \>5σ unkorreliert | **88** |
| \>5σ Achsen-Union | **57** |
| 0743-006 (D×3) | **17,02σ**, 1,246 mas |
| Rahmenrotation über 171 D×3 | max \|ω\| **0,046**, alle Komponenten < 3σ |
| chi²/dof der Rahmen-Fits | **1,2 / 4,9 / 7,9** |

**Urteil:** Anker-Ebene **Placed**; kein Rahmen-Riss. Die 111 Quellen über 5σ sind
**Struktur-Kandidaten** (der Quasarkern sitzt je nach Wellenlänge anders), keine
Rahmen-Brüche. Die `chi²/dof`-Werte bis 7,9 sagen, dass die formalen Fehlerbalken die
Streuung nicht tragen — ein Struktur-, kein Rahmen-Befund.

**Die externe Vorlage ist nicht reproduziert.** Sie nannte Median 2,86σ, 64 Quellen
über 5σ und 0743-006 mit 10,3σ. Der eigene Lauf misst 2,930σ, 111 (korreliert) bzw.
88 (unkorreliert) und 17,02σ. Die Abweichung ist zu groß, um die externe Liste als
Beleg zu übernehmen. Das ist selbst ein Ergebnis: Die Zahlen der Vorlage stammen aus
einer anderen Metrik oder einem anderen Katalogstand; die eigene Messung steht.

### 2.3 Was der Abgleich nicht kann

- **Kein Unabhängigkeitsbeweis.** S/X, K und X/Ka sind VLBI-Messungen, häufig mit
  denselben Teleskopen und überlappenden Zeiträumen. Der Dreifach-Schnitt ist ein
  **Konsistenztest** der ICRF3-Konstruktion.
- **Gaia ist die optische Gegenlinie, aber kein absoluter Ursprung.** Gaia-CRF3 wurde
  zur Orientierung an ICRF3 ausgerichtet; sie prüft Quellenpositionen und die
  optisch-radio Verschiebung, nicht den Ursprung.
- **Epochenversatz.** Gaia ist Epoche 2016, Radio überwiegend 1990er; eine
  QSO-Eigenbewegung ist nicht modelliert (Grenze 0,1 mas über 25 Jahre).
- **Gaia DR4** offiziell **2026-12-02**; die Vorabveröffentlichung mit **12
  Quellen** (`gaia-dr4-prerelease-epoch-astrometry_2026-06-26.zip`, 625.651 B,
  sha256 `07f0e8d9…35fa0b`) ist live und erlaubt den Methoden-Dry-Run, nicht die
  512-Anker-Korrektur.
- **Die 303 D-Quellen** (D in S/X) sind als Ensemble gefittet (2026-10-21): **193**
  mit K-Position, **176** mit X/Ka-Position; Rotation S/X–K max **1,13σ**
  (chi2/dof 1,2), S/X–X/Ka max **1,52σ** (chi2/dof 7,7) → **Placed**. Die
  110/127 Quellen ohne K/X/Ka-Position sind eine benannte Abdeckungslücke.

## 3. Planet Neun — die Bande trägt nicht

- **Daten:** PNRP 3.0 (100.000 Bahnen, CaltechDATA `10.22002/8fjad-x7y61`), KBO-Katalog
  7.195 Records, P9-Auswahl 44, davon 21 in der Bande (17,4 % der Himmelsfläche).
- **Stärkster Faden:** Ω in der Bande, `z_joint = 3,60`, `p2 = 0,002`.
- **Max-Null:** Unter Shuffle/Joint über q95, unter q99; unter **Bandrotation** fällt Ω
  auf `z = 0,67`, das beobachtete Gesamtmaximum (1,10) liegt unter q95 (2,42),
  `p_look = 0,9050`.
- **Verdikt:** **kein robuster Überschuss.** Ω ist explorativ, keine Entdeckung. Die
  Bande ist eine Verteilung, kein Punkt; ohne Abdeckungsmodell ist ein Überschuss in
  ihr keine Entdeckung.
- **Offen:** LSST/MPC-Abdeckung ist im v3.0-CSV **absent**; Bias-Gewichte fehlen; zehn
  Risse sind benannt (galaktische Ebene, Himmels-/Element-Korrelation, kleines n,
  Epochenversatz, Frame-Riss).

## 4. Hunga Tonga — die Front in sechs Netzen

- **Quelle:** 2022-01-15 04:14:45 UTC, 20,536° S / 175,382° W.
- **Seismik:** fünf Stationen 3,52–4,74 km/s, Fit **3,87 km/s** (Rayleigh-Band).
- **Luftdruck:** fünf Stationen, Fit **0,328 km/s** (7 % über Lamb 0,306).
- **Tsunami:** vier Stationen **0,170 km/s** (Vorhersage ~0,20 für 4 km Tiefe).
- **Ionosphäre:** TEC-Onset 45 min nach t0, Maximum −57 %, aber 15-min-Raster und
  Kp 3,7–4,3 → keine Frontmessung.
- **Blitz:** GLM-Cluster 04:14:42, 2 s vor t0, Schwerpunkt 20–47 km vom Vent.
- **Infraschall:** BGR IS52, Richtung −6,7° gegen −113,1° am NULL-Tag.
- **Verdikt:** mindestens drei Phänomene; das „Netz als Instrument" überträgt sich auf
  **Quellzeit und Geometrie**, nicht auf eine gemeinsame Geschwindigkeit.
- **Aerosol/Thermik:** Die Route ist da; am Vent ist die Thermik **gemessen absent**
  (FDCF `Mask=150`, FIRMS VIIRS 0 Detektionen im 0,5°-Fenster). Die 17
  Vollbild-Pixel sind global, nicht Hunga Tonga. Aerosol: AODF/AER_AI keyless,
  AER_AI-Pixel-Extraktion offen. Der Seismik-Detektor hat drei NULL-Treffer;
  Kahului ist kein Tsunami-Onset; der BGR-Abstand ist 1.316 s bei ~300-s-Raster.

## 5. Der Stein im Wasser

Das Bild ist präzise: `field_te_query` ist der Stein, die Paar-Matrix misst, welche
Kanäle antworten und mit welcher Verzögerung. Die „verschiedenen Steine" sind die
verschiedenen Kanäle und Datensätze, in die derselbe Sucher geworfen wird — nicht
verschiedene Werkzeuge. Der gemessene Stand:

- **7 Kanäle am Draht**, **8 probe-gelesen**, **5 declined/descoped**.
- **Vlies-Matrix 2026-10-11:** 15/15 Arme, 210/210 Zellen. Die `n=0`-Zellen sind
  **alignment pending** — ein Datenfenster, kein Code-Defekt.
- **Newell→OMNI:** still, TE 4,5969e-3, p 1,0, bias `off_table`; kein gemessener
  Kopplungspfad auf dieser Skala.
- **`ozzy` ist nicht gebaut.** Die negative Engine existiert nur als Spec; ohne sie
  bleibt die Matrix eine Korrelationsmaschine, kein Unabhängigkeitstest.
- **Die Alles-gegen-alles-Matrix ist nicht gefahren.** Gefahren sind Einzelpaare und
  die 15×15-Vlies-Matrix; die volle Paar-Matrix und die netz-weite strenge Null fehlen.
- **Transitivität** (X→Y, Y→Z ⇒ X→Z) ist Konzept, nicht Code.

## 6. Mond — gebaut, gemessen, nicht behauptet

- `--cond lunar` ist gebaut und getestet; der Conditioner heißt `lunar`, nicht `moon`
  (das remove-bias-Gate verbietet das Körper-Literal).
- **bins 4:** konditioniert NVE→IVE 0/36, IVE→NVE 0/36; roh NVE→IVE 5/36.
- **bins 3:** NVE→IVE 0/36; IVE→NVE **3/36** (MAX-NULL ja); roh 6/36 bzw. 4/36.
- **Verdikt:** Kein Mond-Effekt wird aus einer Zelle behauptet. Der nächste Schnitt ist
  die **5-Kond-Baseline ohne lunar gegen dieselbe Baseline + lunar**, Schwellen
  getrennt.
- **Riss:** Der 3-h-Lag-Sweep `39154644922` wurde ohne belegten Grund abgebrochen; der
  Mond-Lauf selbst ist gefahren, die Richtungsfrage bleibt offen.

## 7. Signal-cone — die Positivkontrolle

- **Lauf:** `signal_cone_audit_probe --days 730`, exit 0, 275 Zeilen,
  sha256 `ab8419c7…fd2553`.
- **Positivkontrolle im selben Lauf:** τ = 2 d = 1,728e5 s; d(sun↔wind) = 0,9806 AU;
  d/c = 4,893e2 s = **8,155 min**; reach(τ) = 5,180e13 m; **BESTANDEN**.
- **Ehrlich:** Die Kontrolle ist **synthetisch** (AR(1)-Treiber, gepflanzte 2-d-Kopplung);
  sie validiert Detektor, Lag-Rückgewinnung und Kegel-Arithmetik, keinen physikalischen
  Kanal. Die reale Audit-Tabelle bleibt leer (0 fam-Pfeile außer der bekannten
  Richtungszeile).

## 8. Aurora/ASI — schon gebaut, kein Neubau

Die Aurora-Kamera und die ASI-Pipeline waren **bereits gebaut**. Der Neubau war
redundant; die eigentliche Arbeit war die Höhen- und Metadatenprüfung:

- **24/24 Stationen** tragen jetzt eine belegte ellipsoidische Höhe (± ≤ 10 m).
- **WHIT** war der letzte offene Fall: DSM am Punkt 741,40 m H, Boden-Transect
  739,9–741,6 m H → **741,0 ± 3 m**.
- **MCGR ist ein Metadatenfehler, kein Umzug.** Die Köpfe 2013–2015 nennen
  `62,95 / −155,6 / 102 m`; ab 2016/09 springen sie auf `−154,42 / 0,0` — dort ist
  Wilderness. Der Code bleibt am Altstandort **−155,596 / 115,23 m**.
- **Offen ist nicht der Bau, sondern die Kreuzprobe:** Kamera/SSUSI gegen die
  Bodenstationen fehlt; THEMIS/PGM und SSUSI deckten 2024-05-10 nur einen Tag.

## 9. Sturm-Komposit — was wirklich antwortet

- **166 Ereignisse** (Dst < −50 nT), 13 Fäden, 500 Nullziehungen.
- **Rangliste:** Dst 6,45 |z| (+1 h), Sym-H 6,02 (+1 h), AL 5,92 (0 h), AE 5,87 (0 h),
  AU 4,17 (−1 h), SOD dB/dt 3,61 (−1 h), ABK dB/dt 3,29 (0 h), Bz 3,09 (−1 h).
- **Verdikt:** Ringstrom und Auroral-Elektrojet antworten stark; Bz dreht vor dem
  Dst-Onset südwärts. **Dst ist Definitions- und Messfaden zugleich** — sein Spitzenplatz
  ist teils zirkulär. **GIC** liegt mit 1,36 |z| (1,94× Null) über der Schwelle, aber nur
  für **118/166** Ereignisse. **ECOD** erreicht 1,03× am Lagrand (−24 h) — kein
  belastbarer Vorlauf.

## 10. Sonne-Erde/Funk — der neue Zwilling

- **e-CALLISTO/MRO** 2024-05-10: 276 FITS, 75 MB, 900-s-Slots, 361 s je Datei,
  200 Frequenzzeilen; `frq00200.cfg` 45,000–869,938 MHz.
- **OVRO-LWA** 2024-05-10: 1,94 GB, 15,0–84,9 MHz, 0,261 s, `BUNIT Jy`.
- **Zwillings-Test:** unentschieden, nicht negativ. r(0)=0,09; best 0,43 bei −110 min
  über 73 Lags = Multiple-Comparison-Rauschen.
- **Offen:** `lower_band.cfg` (stationlokal, unveröffentlicht), Kalibrierpunkt
  Digits↔Jy, MRO-Achse geliehen.

## 11. Was zu Ende ist — und was nicht

**Zu Ende (mit Messung, Datum, Datei):**

1. ICRS-Rahmen-Wache — 600/171 plus 303 D in S/X (193 K / 176 X/Ka), Rotation < 2σ, Placed (`2026-10-20/21`).
2. Planet Neun — kein robuster Überschuss, 10 Risse benannt (`2026-10-20`).
3. Hunga Tonga — sechs Netze, drei+ Phänomene (`2026-10-20`).
4. Aurora/ASI-Höhen — 24/24, MCGR-Fehler benannt (`2026-10-20`).
5. Signal-cone — Positivkontrolle im selben Lauf (`2026-10-21`).
6. Sturm-Komposit — Rangliste und Risse (`2026-10-20`).
7. Mond-Faden — gebaut und gemessen, Richtungsfrage offen (`2026-10-19`).

**Nicht zu Ende (benannt, nicht vergessen):**

1. **`ozzy`** — die negative Engine fehlt; ohne sie bleibt die Matrix unvollständig.
2. **Alles-gegen-alles-Matrix** — die volle Paar-Matrix und die netz-weite strenge Null.
3. **Rahmen-Wache** — Gaia DR4 (offiziell 2026-12-02; 12-Quellen-Vorab live),
   Eigenbewegung, 88 Anker ohne Gaia.
4. **Planet Neun** — LSST/MPC-Abdeckung, Bias-Gewichte.
5. **Hunga Tonga** — AER_AI-Pixel am Vent, TEC mit höherer Kadenz, besserer
   Seismik-Detektor.
6. **Mond** — 5-Kond-Baseline vs. +lunar getrennt.
7. **e-CALLISTO/OVRO** — `lower_band.cfg`, Kalibrierpunkt.
8. **GIC** — 118/166 Ereignisse; die Reihe endet 2023-09-30.
9. **Aurora** — Kamera/SSUSI-Kreuzprobe, nicht der Neubau.

## 12. Die einfache Antwort auf die ICRS-Frage

Das Haus kann ab jetzt zwei Dinge prüfen:

- **Die Kopie:** `sha256sum` gegen die drei Katalog-Hashes. Kippt eine Ziffer, fällt es
  sofort auf.
- **Die Vermessung:** 600 Leuchttürme, dreifach gemessen; die Rahmenrotation liegt
  unter 3σ → **Placed**. Die 111 Quellen über 5σ sind Quasarstruktur, kein Rahmenriss.

Was niemand prüfen kann, ist, ob eine Vereinbarung „wahr" ist. Prüfbar ist, ob sie
zuverlässig vermessen ist — und das ist sie, soweit die drei Radio-Kataloge reichen.
Die wirklich unabhängige zweite Linie ist Gaia (optisch); sie deckt 512/600 und ist
selbst an ICRF3 ausgerichtet. Deshalb bleibt die Rahmen-Wache eine **stehende Aufgabe**,
kein einmaliger Befund.

## 13. Ablage und Reproduktion

| Probe | Kommando |
|---|---|
| Frame-Wache | `cargo run --release -p omegaflow-measure --bin frame_wache_probe` |
| Hunga Tonga | `cargo build --release -p omegaflow-measure --bin hunga_tonga_netz_probe && ./target/release/hunga_tonga_netz_probe` |
| Planet Neun | `cargo build -p omegaflow-measure --bin planet_nine_netz_probe` (Argumente: `--kbo`, `--pnrp`, `--draws 10000`) |
| Mond | `de2_conditional_te --cond lunar` (Ephemeriden `ephemeris_moon.bin`, `ephemeris_earth.bin`) |
| Signal-cone | `signal_cone_audit_probe --days 730 …` (Log `signal_cone_audit_730d.log`) |
| Sturm-Komposit | `sturm_komposit_probe --data-root ../omegaflow-work/sources` |

Die Surveys liegen unter `docs/surveys/`; die Projekte unter
`projects/{quasare,vlies,flyby,ephemerides,sonne-erde}.md`. Die Rohdaten der
Frame-Wache liegen unter `sources/<netloc>/`, die Provenienz in
`sources/cdsarc.cds.unistra.fr/PROVENANCE-frame-wache.md`.

## Offen

- **Rahmen-Ebene:** Gaia-DR4 als unabhängige optische Linie; die 303-D-Rotation ist
  gemessen (Placed).
- **`ozzy`:** die negative Engine bauen, bevor die Matrix als Unabhängigkeitstest gilt.
- **Planet Neun:** LSST/MPC-Abdeckung nachziehen; ohne sie bleibt die Bande eine
  Himmelsdichte-Aussage.
- **Hunga Tonga:** AER_AI-Pixel am Vent extrahieren; keyed Zwillinge (XAERDT, OMPS,
  FIRMS) nutzen; TEC-Kadenz erhöhen.
- **Mond:** 5-Kond-Baseline gegen dieselbe Baseline + lunar.
- **e-CALLISTO/OVRO:** Achse und Kalibrierpunkt belegen.
