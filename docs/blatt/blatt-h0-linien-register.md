<!--
  title: BLATT — Das H₀-Linien-Register: Wurzeln statt Zeugen
  class: sheet
  date: 2026-09-08
  sha256: aea40786724c89bc635fa3612998901fb484d8a65f16c7e7761e7dd2a3a9fd28
  status: live
  see-also: docs/concepts/die-weberin.md docs/blatt/blatt-der-grat.md docs/concepts/ein-blatt-axiom.md
-->
# BLATT — Das H₀-Linien-Register: Wurzeln statt Zeugen

**Datum:** 2026-09-08 · **Axiom:** A = A
**Verdikt-Ordnung:** 0 honored — kein Wert erfunden; was die Quelle nicht trägt, ist `absent`; was die Ernte nicht erreicht, ist `pending`; ein unverifizierbarer Link tritt als `absent` in den Anhang, nie als Zeile.

## Die Frage

Nicht „wer hat recht". Sondern: **Wie unabhängig sind die Zeugen wirklich?**
Die Weberin zählt Wurzeln, nicht Zeugen (die-weberin §8). Die Literatur trägt
die H₀-Werte und ihre Fehler — die Abstammungsspalte trägt sie nicht. Dieses
Blatt ist diese Spalte: jede publizierte H₀-Messung eine Zeile, und je Zeile
die Wurzel-Kette — welche Kalibratoren, welche SN-Stichprobe, welche
Parallaxen-Quelle ein Zeuge teilt oder nicht teilt.

Die Hubble-Spannung (Planck ≈ 67,4 gegen die Leiter ≈ 73, ~5σ) ist in
die-weberin §8 als „Illustration des Risses" benannt. Dieses Blatt macht die
Benennung präzise — es löst die Spannung nicht, es kartiert, wo Unabhängigkeit
echt ist und wo vorgespielt, wo die Bäume Wurzeln teilen und wo das weiße Feld
sitzt.

## Die Tabelle

Neun Spalten: Zeuge · Jahr · Referenz · Methode · Wert (km/s/Mpc) · Fehler
(km/s/Mpc) · Wurzel-Kette · Trennstufe · Route. Eine Zeile ist eine Messung
(ein Zeuge). `gewogen` = diese Session misst die Wurzel selbst; `zitiert` =
die Wurzel ist eine zitierte Klasse. Route: `[c]` curl · `[j]` Jina · `[w]`
WebArchive · `[s]` Websuche · `[a]` absent-benannt.

### Leiter

| Zeuge | Jahr | Referenz | Methode | Wert | Fehler | Wurzel-Kette | Trennstufe | Route |
|---|---|---|---|---|---|---|---|---|
| SH0ES — Riess et al. | 2022 | ApJ 934, L7 · arXiv:2112.04510 | Leiter · Cepheid | 73.04 | ±1.04 | N4258-Maser + LMC-DEB + 75 Gaia-EDR3-Cepheiden + 8 WFC3-Scanning (gewogen); SN Pantheon+ 42 Kal. + 277 Fluss (gewogen); Parallaxe Gaia EDR3 (L20b) | früh | [j][c] |
| Riess et al. (R21, MW-Kalibration) | 2021 | ApJ 908, L6 · arXiv:2012.08534 | Leiter · Cepheid | 73.0 (MW allein) | ±1.4 | 75 MW-Cepheiden, Gaia EDR3 (gewogen) | früh | [c] |
| Freedman et al. | 2021 | ApJ 919, 16 · arXiv:2106.15656 | Leiter · TRGB | 69.8 | ±0.6 stat ±1.6 sys | TRGB-Kal. (LMC-DEB, N4258, ω-Cen) gewogen; SN CSP (gewogen); Gaia nur 5 %-Check | früh | [c] |
| CCHP (Freedman, Madore, Hoyt) | 2020 | arXiv:2002.01550 | Leiter · TRGB | 69.6 | ±0.8 stat ±1.7 sys | LMC-DEB gewogen; SN CSP | früh | [c] |
| Hoyt et al. (TRGB-Zeropunkt) | 2023 | Nat. Astron. 7, 590 · arXiv:2106.13337 | Leiter · TRGB | absent | — | TRGB-I-Zeropunkt M_I ≈ −4.05; keine eigene H₀-Anwendung | früh | [c] |
| Huang et al. (erste Mira-H₀) | 2020 | arXiv:1908.10883 | Leiter · Mira | 73.3 (komb.) | ±4.0 | Miras in N4258 + LMC-DEB | früh | [c] |
| Bhardwaj et al. (engster Mira-H₀) | 2025 | arXiv:2507.10658 | Leiter · Mira | 73.06 | ±2.67 | LMC-DEB + N4258 + MW-GC-Mira | früh | [c] |
| Sanders | 2023 | MNRAS · arXiv:2304.01671 | Leiter · Mira | 73.7 | ±4.4 | MW-Gaia-DR3-Mira-PL gewogen | früh | [c] |

Die geteilte späte Sprosse trägt keine eigene H₀-Zeile: **Pantheon+** (Scolnic
et al. 2021, Brout et al. 2021) ist die SN-Ia-Standardisierung der SH0ES-Zeile;
die CCHP/TRGB-Zeilen ziehen ihre SN-Magnituden aus **CSP**. Der sichtbare
Abstand (73.04 vs 69.8) liegt damit teils auf der späten Sprosse, nicht nur am
frühen Anker.

### CMB/BAO — zitierte kuratierte Klasse

| Zeuge | Jahr | Referenz | Methode | Wert | Fehler | Wurzel-Kette | Trennstufe | Route |
|---|---|---|---|---|---|---|---|---|
| Planck 2018 (best estimate) | 2020 | A&A 641, A6 · arXiv:1807.06209 | CMB/BAO · Planck | 67.36 | ±0.54 | ΛCDM → 100θ* = 1.0411±0.0003 → r_d; zitiert | keine | [c] |
| Planck 2018 + BAO | 2020 | ebd. | CMB/BAO | 67.66 | ±0.42 | BAO erbt r_d aus CMB — die Planck-Ehe | keine | [c] |
| eBOSS (Alam et al.), CMB+BAO | 2021 | PRD 103, 083533 · arXiv:2007.08991 | CMB/BAO | 67.60 | ±0.43 | geteiltes r_d mit Planck — Ehe | keine | [c] |
| eBOSS, BAO allein | 2021 | ebd. | CMB/BAO | absent | — | BAO misst nur r_d·H₀/c (dimensionslos) | keine | [a] |
| DESI 2024 VI, BAO+CMB | 2025 | JCAP 2025, 02, 021 · arXiv:2404.03002 | CMB/BAO | 67.97 | ±0.38 | r_d aus CMB — Ehe | keine | [c] |
| eBOSS, BAO+BBN (Kontrast) | 2021 | ebd. | CMB/BAO | 67.33 | ±0.98 | r_d aus BBN: Ω_b h² = 0.02235 ± 0.00037 aus D/H = (2.527 ± 0.030)×10⁻⁵ (Cooke 2018, Quasar) | keine | [c] |
| DESI 2024 VI, BAO+BBN (Kontrast) | 2025 | ebd. | CMB/BAO | 68.52 | ±0.62 | r_d über BBN (Ω_b h² aus Quasar-D/H) | keine | [c] |

Die geteilte Wurzel der Familie ist der **Schallhorizont r_d** (bzw. sein
CMB-Winkelpendant θ*). BAO misst r_d·H₀/c — ohne externe r_d-Kalibrierung
existiert kein H₀ aus BAO (Zeile „eBOSS, BAO allein": absent). Die einzige
r_d-Route außerhalb des CMB ist BBN (Ω_b h²); gewogen 2026-09-08: die Herkunft
ist Quasar-D/H (Cooke 2018), nicht CMB-entlehnt.

### Der junge Forst — gravitativ/radio

| Zeuge | Jahr | Referenz | Methode | Wert | Fehler | Wurzel-Kette | Trennstufe | Route |
|---|---|---|---|---|---|---|---|---|
| GW170817 (helle Sirene) | 2017 | Nature 551, 85 · arXiv:1710.05835 | Sirene · hell | 70.0 | +12.0/−8.0 | Wellenform + EM-z — ungebunden von Leiter UND CMB | keine | [c] |
| Dunkle Sirenen O4a (LVK) | 2025 | arXiv:2509.04348 | Sirene · dunkel | 75.4 | +12.8/−9.1 | d_L leiterfrei; z teilt Katalog + Fluss-Statistik | keine | [c] |
| Dunkle Sirenen O4a (extern) | 2024/26 | Bom, MNRAS 535, 961 · Alfradique, JCAP 08, 068 | Sirene · dunkel | 70.4 (Bom) | +13.6/−11.7 | z teilt Katalog/Fluss | keine | [c] |
| Yu et al. (Bright & Dark Sirens) | 2024 | ApJS 270, 24 · arXiv:2311.11588 | Sirene · Prognose | absent | — | Machbarkeitsstudie, keine Messung | keine | [c][s] |
| PTA-Parallaxe (D'Orazio & Loeb) | 2021 | PRD 104, 063015 · arXiv:2009.06084 | PTA-Parallaxe | absent | — | rein gravitativ; Messreihe existiert nicht | keine | [c] |
| FRB-DM | 2025 | Front. Astron. Space Sci. · arXiv:2502.08509 | FRB-DM | 65.13 (MLE) | ±2.52 (Modell-Spreizung 51–77) | teilt NE2001/YMW16 mit Pulsar-DM; Host-Beitrag unsicher | keine | [c][s] |
| Megamaser (MCP XIII) | 2020 | ApJL 891, L1 · arXiv:2001.09213 | Megamaser | 73.9 | ±3.0 | Kreuz-Wurzel NGC 4258 (Anker im Cepheid-Ast und direkte Route) | keine | [c] |

### Der junge Forst — plasma/optisch

| Zeuge | Jahr | Referenz | Methode | Wert | Fehler | Wurzel-Kette | Trennstufe | Route |
|---|---|---|---|---|---|---|---|---|
| SZ+Röntgen (Bonamente) | 2006 | ApJ 647, 25 | SZ+Röntgen | 76.9 | +3.9/−3.4 stat, +10.0/−8.0 sys | inverse Compton; Hydrostatic-Gasmodell + Chandra | keine | [c] |
| Kosmische Chronometer (Methode) | 2002 | ApJ 573, 37 | Chronometer | absent | — | Sternalter als Uhr; Methodenvorschlag | keine | [a] |
| Kosmische Chronometer (H(z)) | 2016 | JCAP 05, 014 | Chronometer | absent | — | H(z=0.43)=91.8 ± 5.3; SSP-Alterskalibration | keine | [c] |
| H0LiCOW (Zeitverzögerung) | 2020 | MNRAS 498, 1420 | Linsen | 73.3 | +1.7/−1.8 | Linsen-Massenmodell + Kinematik (MST-degeneriert) | keine | [c] |
| TDCOSMO IV (MST-relaxiert) | 2020 | A&A 643, A165 | Linsen | 67.4 | +4.1/−3.2 | MST nicht mehr fixiert; gleiche Route | keine | [c] |
| Gamma-Abschwächung (EBL) | 2019 | ApJ 885, 137 | Gamma | 67.4 | +6.0/−6.2 | EBL-Modell + intrinsisches Blazar-SED | keine | [c] |

### Der Rücken

| Zeuge | Jahr | Referenz | Methode | Wert | Fehler | Wurzel-Kette | Trennstufe | Route |
|---|---|---|---|---|---|---|---|---|
| Perivolaropoulos (Kompilation) | 2024 | PRD 110, 123518 · arXiv:2408.11031 | Kompilation (Review) | Leiter N=20: 72.8; Ein-Schritt N=33: 69.0 | ±0.5 / ±0.48 | Gruppen-Vergleich; KS p = 0.0001 | keine | [c] |

Der Rücken ist kein eigenes Kompilations-Paper, sondern ein Review mit
eingebetteter Kompilation aller CMB-schallhorizont-unabhängigen H₀. Die „33"
sind die Ein-Schritt-Messungen (Chronometer, Gamma, Linsen, Megamaser u. a.
— ohne Leiter); die „20" sind Leiter-Messungen. Der Kerngedanke: die Spannung
sitzt zwischen der Leiter (72.8) und allen übrigen (69.0), nicht nur zwischen
früh und spät.

## Das weiße Feld

Die CMB-Wurzel (Schallhorizont r_d/θ*) und die Leiter-Wurzel (geometrische
Anker: NGC-4258-Maser, LMC-DEB, Gaia-Parallaxen) **teilen keine gemeinsame
Wurzel**. Der junge Forst trägt dritte Fäden, aber kein dritter Faden trägt die
Präzision, beide Bäume zu binden — kein Schiedsspruch ist möglich. Anders als beim Uranus-Riss, wo die rohen
Beobachtungen die drei streitenden Modelle wiegen, gibt es hier kein
Camargo-Analogon, das beide Linien schlichtet.

**Gemeinsame Wurzel: absent.** Das ist nicht die Lücke des Registers — es ist
der Befund selbst: die Unabhängigkeit der zwei Bäume ist der Riss. Ein
Schiedsspruch über die ~5σ wäre Fabrikation.

## Die Asymmetrie

Die zwei Wurzeln stehen nicht gleich da:

- **Leiter-Wurzel: teils gewogen (Klasse), teils zitiert (Auswahl + Abzug).**
  Der Probe `cepheid_parallax_weigh` hat das Gaia-DR3-Parallaxenfeld der
  klassischen Cepheiden (DCEP) selbst gemessen (Query im Anhang):
  **N = 1606 Cepheiden** (type_best_classification = DCEP, Parallaxe > 5σ),
  **inversvarianz-gewichtetes Parallaxen-Mittel 0.2619 mas, Standardfehler
  ±0.0004 mas**, 1/π = 3818.0 pc, Median 0.2270 mas, Spanne 0.059–4.194 mas.
  Der statistische Standardfehler wird vom nicht-angewendeten ~14-μas-Zeropunkt
  dominiert. Das Feld ist das rohe Archiv ohne L20b-Abzug — die publizierte
  SH0ES-Kalibration (Riess 2021) nutzt 75 ausgewählte Cepheiden und zieht
  diesen Abzug ab; der H₀-tragende Anker bleibt `zitiert`, gewogen ist nur die
  Klasse.
- **CMB-Wurzel: zitiert.** Die Planck-Likelihood (θ*, r_d, das CMB-Leistungs-
  spektrum) ist eine Forschungsmaschine, keine Session. Keine Zeile dieser
  Familie wird selbst gerechnet; die Klasse ist benannt, nicht verschwiegen.

## Die Trennstufen-Karte

Wo die Fäden auseinanderlaufen:

- **früh** — die Anker-Sprosse der Leiter: die Parallaxen-Quelle (nur der
  Cepheid-Ast kalibriert direkt mit Gaia-EDR3-MW-Cepheiden + HST-Scanning; der
  TRGB-Ast nutzt Gaia nur als 5 %-Check), der Indikator (Cepheid-Leavitt vs
  TRGB M_I ≈ −4.05 vs Mira-PL), die Anker-Topologie (N4258, LMC-DEB — alle
  Leiter-Äste teilen dasselbe Ankerfundament).
- **spät** — die SN-Ia-Sprosse: SH0ES nutzt Pantheon+ (42 Kal. + 277 Fluss),
  CCHP/TRGB nutzt CSP. Der 73.04-vs-69.8-Abstand liegt teils auf dieser
  Sprosse selbst, nicht nur am frühen Anker.
- **keine** — die CMB/BAO-Schallhorizont-Route und alle Ein-Schritt-Routen.

Der Rücken trägt die Konsequenz: die Spannung ist kein reines früh-vs-spät-
Gefälle, sondern Leiter (72.8) gegen alle übrigen (69.0), KS p = 0.0001.

## Das Verdikt

**Zwei Bäume, gemeinsame Wurzel: absent.** Die CMB-Wurzel und die Leiter-Wurzel
teilen keine Wurzel — ein Schiedsspruch wäre Fabrikation, denn kein dritter
Faden trägt die Präzision, beide Bäume zu binden. Die Unabhängigkeit selbst ist der Riss.

**Der junge Forst trägt breite Fehlerbalken: noch kein Schlichter.** GW170817
±15 %, die dunklen Sirenen ±9–14 km/s, der Megamaser ±3.0 (nahe SH0ES, schließt
Planck bei ≈ 2.1σ nicht aus), FRB-DM mit Modell-Spreizung 51–77. Jeder
Forst-Zeuge ist mit beiden Ankern verträglich; ein Schlichter braucht ≲1–2 %.
Das ist ein Verdikt, kein Aufschub — jede Familie trägt ihren gemessenen Wert
und ihre Fehlerbreite als eigene Zeile; die fehlende Präzision ist `pending`
(die Messung existiert, die Schärfe fehlt).

Die publizierten Kompilationen zählen Zeugen; die Abstammungsspalte (die
Wurzel-Zählung) trägt dieses Blatt. Die Wurzel-Zählung ergibt: der Wald ist
größer als zwei Bäume, die meisten Bäume sind jung, und die zwei alten Bäume
stehen auf getrenntem Grund.

## Der Nachtrag (gewogen 2026-09-08)

Ein externer Nachtrag (Chat zweier Fremd-Sessions) behauptete eine „Korrektur"
und neue Zeugen. Gewogen, nicht übernommen.

**Die falsche „Korrektur":** behauptet wurde, die LVK-Dunkle-Sirenen-Zeile müsse
76.6 +13.0/−9.5 tragen und 75.4 sei „Transkriptionsartefakt". Direkter Fetch von
arXiv:2509.04348 [c]: das Abstract führt **H₀ = 75.4 +12.8/−9.1**; die 76.6
kommt im Dokument nicht vor. Die Blatt-Zeile steht unverändert.

### Neue Zeugen — Leiter, JWST-Ära

| Zeuge | Jahr | Referenz | Methode | Wert | Fehler | Wurzel-Kette | Trennstufe | Route |
|---|---|---|---|---|---|---|---|---|
| Riess et al., JWST-Validierung (Cepheid) | 2024 | ApJ 977, 120 · arXiv:2408.11770 | Leiter · Cepheid (JWST) | 73.4 | ±2.1 | N4258-Einzelanker; JWST-SN-Subprobe | spät | [c] |
| Riess et al., JWST (JAGB) | 2024 | ebd. | Leiter · JAGB (JWST) | 72.2 | ±2.2 | N4258-Einzelanker | spät | [c] |
| Riess et al., JWST (TRGB) | 2024 | ebd. | Leiter · TRGB (JWST) | 72.1 | ±2.2 | N4258-Einzelanker | spät | [c] |
| Riess et al., JWST kombiniert | 2024 | ebd. | Leiter · kombiniert | 72.6 | ±2.0 | N4258; 16 SN (D < 25 Mpc) | spät | [c] |
| Riess et al., HST-Vollsetz | 2024 | ebd. | Leiter · Cepheid (HST) | 73.2 | ±0.9 | 4 Anker; 42 SN | spät | [c] |
| Freedman/CCHP 2025 (TRGB) | 2025 | ApJ 985, 203 · arXiv:2408.06153 | Leiter · TRGB | 70.39 | ±1.22 stat ±1.33 sys ±0.70 σ_SN | N4258-Anker; 24 SN HST+JWST | spät | [c] |
| Freedman/CCHP 2025 (JWST-only TRGB) | 2025 | ebd. | Leiter · TRGB (JWST) | 68.81 | ±1.79 stat ±1.32 sys | N4258; SN JWST-only | spät | [c] |
| Freedman/CCHP 2025 (JAGB) | 2025 | ebd. | Leiter · JAGB (JWST) | 67.80 | ±2.17 stat ±1.64 sys | N4258 | spät | [c] |
| Riess et al., „The Perfect Host" | 2025 | ApJL 992, L34 · arXiv:2509.01667 | Leiter · Cepheid (JWST+HST) | 73.49 | ±0.93 | 19 JWST + 37 HST Hosts; 24+42 SN | spät | [c] |
| Riess et al., „The Perfect Host" +TRGB | 2025 | ebd. | Leiter · Cepheid+TRGB | 73.18 | ±0.88 | +35 TRGB; 55 SN | spät | [c] |

Die JWST-Ära-Zeilen sitzen auf der **späten** Sprosse: sie sind als Gegentest
gebaut, der den frühen Anker auf N4258 reduziert (keine Gaia-Parallaxe, keine
LMC) und die Differenz auf die SN-/Indikator-Auswahl legt. Riess 2024
adressiert die „unrecognized crowding"-Hypothese (Photometrie-Bias in dichten
HST-Feldern); Freedman 2025 rückt mit JWST-Daten leicht näher an SH0ES (70.39
statt 69.8). Die Leiter-interne Spannung (Cepheid vs TRGB) verengt sich, schließt
nicht.

### Neue Zeugen — CMB/BAO, DESI DR2

| Zeuge | Jahr | Referenz | Methode | Wert | Fehler | Wurzel-Kette | Trennstufe | Route |
|---|---|---|---|---|---|---|---|---|
| DESI DR2, BAO+CMB | 2025 | PRD 112, 083515 · arXiv:2503.14738 | CMB/BAO | 68.17 | ±0.28 | r_d aus Planck+ACT-DR6-Lensing (Ehe) | keine | [c] |
| DESI DR2, BAO+BBN (Kontrast) | 2025 | ebd. | CMB/BAO | 68.51 | ±0.58 | r_d aus BBN: Ω_b h² = 0.02218 ± 0.00055 (Quasar-D/H, Schöneberg 2024) | keine | [c] |

DESI DR2 allein (BAO ohne r_d) bleibt `absent` für H₀ — dieselbe Struktur wie
die „BAO allein"-Zeile (misst nur h·r_d = 101.54 ± 0.73 Mpc).

### Neue Zeugen — junger Forst

| Zeuge | Jahr | Referenz | Methode | Wert | Fehler | Wurzel-Kette | Trennstufe | Route |
|---|---|---|---|---|---|---|---|---|
| GWTC-5.0 (LVK, O1–O4b) | 2026 | arXiv:2605.27227 | Sirene · dunkel+hell | 71.7 | +9.4/−7.5 | DES-Y6-Katalog; FullPop-4.0; GW170817 | keine | [c] |
| TDCOSMO 2025 (Birrer et al.) | 2025 | arXiv:2506.03023 | Linsen | 71.6 | +3.9/−3.3 | Pantheon+ als Ωm; SLACS/SL2S; MST konservativ | keine | [c] |
| TRGB-SBF III (Jensen, Blakeslee u. a.) | 2025 | arXiv:2502.15935 | SBF | 73.8 | ±0.7 stat ±2.3 sys | N4258-Anker → JWST-TRGB; SN-Ia-frei; Kreuz-Wurzel N4258 | keine | [c] |

GWTC-5.0 löst GWTC-4.0 ab (Unsicherheit −22 %) und bleibt mit beiden Ankern
verträglich. TRGB-SBF ist SN-Ia-frei (SBF direkt im Hubble-Fluss, Nullpunkt aus
JWST-TRGB an N4258), liegt nahe SH0ES, teilt aber den N4258-Anker mit Leiter und
Megamaser — kein dritter unabhängiger Schlichter.

### Rücken-Update

| Zeuge | Jahr | Referenz | Methode | Wert | Fehler | Wurzel-Kette | Trennstufe | Route |
|---|---|---|---|---|---|---|---|---|
| Pantos & Perivolaropoulos | 2026 | arXiv:2601.00650 | Kompilation | 88 schallhorizont-freie H₀, 4 Klassen | — | Kat.1 Leiter (n=30) 72.73 ± 0.39 · Kat.2 Lokal-ΛCDM 67.61 ± 0.96 · Kat.3 rein-lokal (n=16) 71.03 ± 0.69 · Kat.4 CMB-schallhorizont-frei 69.07 ± 0.44 | keine | [c] |

Interne Spannung Kat.2↔Kat.3: 2.9σ. Das Rücken-Bild (Leiter gegen alle übrigen)
bleibt; die 88 sind die Erweiterung des 53er-Rückens.

### Gelöste `pending`-Punkte

- **BBN-Herkunft:** Quasar-D/H, nicht CMB-entlehnt. eBOSS zitiert
  D/H = (2.527 ± 0.030)×10⁻⁵ (Cooke, Pettini & Steidel 2018); DESI entlehnt
  Ω_b h² = 0.02218 ± 0.00055 aus Schöneberg 2024 (PRyMordial, Eingänge
  Cooke-D/H + Aver-Y_P). Die BBN-Route ist damit echt CMB-frei.
- **Enum-Vokabular `gaiadr3.vari_cepheid`:** `type_best_classification` ∈
  {DCEP, T2CEP, ACEP}; `type2_best_sub_classification` ∈ {BL_HER, W_VIR, RV_TAU};
  `mode_best_classification` ∈ {FUNDAMENTAL, FIRST_OVERTONE, SECOND_OVERTONE,
  MULTI, UNDEFINED, NOT_APPLICABLE}; dazu `multi_mode_best_classification` ∈
  {F/1O, F/2O, 1O/2O, 1O/3O, 2O/3O, F/1O/2O, 1O/2O/3O}. Konsequenz für die
  Asymmetrie: der Probe-Query filtert jetzt auf `type_best_classification =
  'DCEP'` — die gewogene Klasse (N = 1606, 0.2619 mas) ist die klassische
  Cepheiden-Teilmenge der SH0ES-Kalibration; der breite Boden (alle Typen,
  N = 2078) wog 0.2530 mas.

### Gemessene Abweichungen vom externen Nachtrag

- GWTC-5.0: behauptet „71.0 +9.0/−7.1" — gemessen **71.7 +9.4/−7.5** (Abstract).
- Die „Korrektur" 76.6: gemessen falsch (s. o.).

### Auswirkung auf das Verdikt

Unverändert: **zwei Bäume, gemeinsame Wurzel: absent.** Der junge Forst ist
dichter geworden (Sirenen, Linsen, SBF, Rücken), aber noch nicht schärfer — kein
neuer Zeuge trägt ≲1–2 %. Die Leiter-Wurzel ist mit JWST fester geworden, nicht
lockerer; die Spannung wird robuster, nicht aufgelöst.

## Der Anhang

**Route-Kaskade** (docs/SOURCE_PORT.md §Agenten-Rezept): direkter curl → Jina-
Reader (`r.jina.ai/`) → WebArchive (Wayback + CDX) → Websuche (`s.jina.ai/`);
erst nach leerer Kaskade `absent`. Befund: 20 `quelle`-Links, 9 `kontext`-Links,
0 unreachable. ADS-UI antwortet 405 (Records via ADS-API), OUP/APS 403 (über
Crossref/Jina/Wayback), IOP hinter Radware-CAPTCHA (Crossref/Wayback/ADS-Record).

**Korrekturen (gemessen, nicht übernommen):**
- PRD 110.123518 ist nicht eine eigenständige „33er-Kompilation", sondern
  Perivolaropoulos 2024, *Hubble tension or distance ladder crisis?* — ein
  Review mit eingebetteter Kompilation.
- ApJS 270, 24 (Yu et al. 2024) trägt arXiv:2311.11588 — nicht 1902.05569
  (das ist Akeson et al., WFIRST-Whitepaper 2019).
- Der Wert „76.6 +13/−9" (aus einer LIGO-O4a-PDF) ist gemessen falsch: direkter
  Fetch von arXiv:2509.04348 zeigt, dass das Abstract H₀ = 75.4 +12.8/−9.1
  führt; die 76.6 kommt im Dokument nicht vor. Die Blatt-Zeile steht.
- „eBOSS 67.35 ± 0.97" ist nicht im Primär (PRD 103, 083533); die Primärwerte
  sind 67.33 ± 0.98 (BBN+BAO) und 67.60 ± 0.43 (CMB+BAO).

**Probe-Query** (`tools/measure/src/bin/cepheid_parallax_weigh.rs`, Gaia-TAP
`https://gea.esac.esa.int/tap-server/tap/sync`):

    SELECT g.parallax, g.parallax_error
    FROM gaiadr3.vari_cepheid AS c
    JOIN gaiadr3.gaia_source AS g USING (source_id)
    WHERE c.type_best_classification = 'DCEP'
      AND g.parallax > 0 AND g.parallax_error > 0
      AND g.parallax > 5 * g.parallax_error

**Kontext-Links, nie Zeilen** (Blogs/Produktseiten/Suchportale): astrobites
(2018/2021), preposterousuniverse.com (2017), news.berkeley.edu (2023),
timandersen.substack.com, search.proquest.com, autoclaw.z.ai, zcode.z.ai,
ned.ipac.caltech.edu/level5 (Reese 2004, Petroff 2019 — Review-Kontext).

**`pending`:** die VizieR-Katalog-ID der Riess-2021-Cepheidentabelle (CDS
hinter Anubis/401, NAOJ-Spiegel Transportfehler — der arXiv-Quellpfad
`bigtable_redux3.tex` trägt die 75 statt); die Erratum-Wirkung von Freedman 2025
(ApJ 993, 252) — inhaltlich nicht geprüft.
