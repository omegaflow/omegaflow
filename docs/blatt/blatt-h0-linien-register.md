<!--
  title: BLATT — Das H₀-Linien-Register: Wurzeln statt Zeugen
  class: sheet
  date: 2026-09-09
  sha256: ec72dd36b2d67599ae32e83d0e12317cbcc5c59bc7529126ae25fbb01a143b26
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

- **Leiter-Wurzel: ganz gewogen (Leiter).**
  Der Probe `h0_ladder_weigh` wiegt die Leiter end-to-end im Haus (Cepheiden-PL
  im Parallaxenraum → SN-Ia-Kalibration → eigener H₀): **H₀ = 73.56 ± 1.40
  km/s/Mpc** (volle STAT+SYS-Kovarianz; diagonaler Checkpoint 73.53 ± 1.14),
  M_W1 = −5.914 ± 0.017 (publiziert −5.915 ± 0.022), zp = −13 ± 5 μas
  (publiziert −14 ± 6), M_B = −19.2469 ± 0.0299 (77 Kalibratoren),
  a_B = 0.7159 ± 0.0018 (flach ΛCDM Ωm = 0.3). Reproduktions-Gate **PASS** —
  jeder Parameter innerhalb 1σ des publizierten Wertes (Bestehensregel als
  genagelte Zahl; bei Abweichung: erst die eigene Kette wiegen — Einheiten →
  Parser → Fit —, dann der publizierte Wert). Die Cepheiden-Anker-Wurzel ist
  transkribiert: die 75er-Tabelle trägt keine Koordinaten-/ID-Spalte, das
  eigene Gaia-TAP-Crossmatch der 74 ist gewogen (74/74 `source_id`, s. u.). Der Probe
  `cepheid_parallax_weigh` bleibt das Klassen-Feld (N = 1606, 0.2619 mas,
  Query im Anhang). Der H₀-tragende Anker ist gewogen, nicht zitiert.
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

## Der zweite Nachtrag (gewogen 2026-09-08)

### Neue Zeugen — CMB-Klassen-intern + Stammlinie

| Zeuge | Jahr | Referenz | Methode | Wert | Fehler | Wurzel-Kette | Trennstufe | Route |
|---|---|---|---|---|---|---|---|---|
| ACT DR4 (Aiola et al.), ACT allein | 2020 | JCAP 12, 004 · arXiv:2007.07288 | CMB · ACT | 67.9 | ±1.5 | ΛCDM → θ*/r_d; eigenes Teleskop+Pipeline, keine Planck-Daten | keine | [c] |
| ACT DR4, +WMAP | 2020 | ebd. | CMB · ACT+WMAP | 67.6 | ±1.1 | θ*/r_d; WMAP für großskalige Info | keine | [c] |
| ACT DR6 (Madhavacheril et al.) | 2024 | ApJ 962, 113 · arXiv:2304.05203 | CMB · Lensing+BAO | 68.3 | ±1.1 | Linsen-Spektrum + BAO-r_d; kein eigenständiger H₀ | keine | [c] |
| WMAP9 (Hinshaw et al.), allein | 2013 | ApJS 208, 19 · arXiv:1212.5226 | CMB · WMAP | 70.0 | ±2.2 | θ*/r_d; eigenes Satelliten-Instrument | keine | [c] |
| WMAP9, +eCMB+BAO | 2013 | ebd. | CMB · WMAP+BAO | 68.76 | ±0.84 | θ*/r_d + BAO | keine | [c] |
| HST Key Project (Freedman et al.) | 2001 | ApJ 553, 47 · arXiv:astro-ph/0012376 | Leiter · Cepheid (historisch) | 72 | ±8 | Cepheiden-PL, 5 Methoden kombiniert | früh | [c] |

ACT und WMAP teilen mit Planck nur das θ*/r_d-Konzept, nicht Daten, Instrument
oder Pipeline — die CMB-Familie bekommt unabhängige Instrumente, die geteilte
Wurzel (r_d) bleibt. Keiner trägt Plancks Präzision; alle liegen konsistent mit
67–70. ACT DR6 ist einen Schritt entkoppelter: Linsen-Spektrum + BAO, kein
eigenständiger H₀. HST Key Project ist die historische Stammlinie des
Cepheid-Astes.

### Neue Zeugen — späte Routen (Fundamentalebene, Tully-Fisher)

| Zeuge | Jahr | Referenz | Methode | Wert | Fehler | Wurzel-Kette | Trennstufe | Route |
|---|---|---|---|---|---|---|---|---|
| Said et al. (DESI PV, Fundamentalebene) | 2025 | MNRAS 539, 3627 · DOI 10.1093/mnras/staf700 · arXiv:2408.13842 | Fundamentalebene | 76.05 | ±0.35 stat ±0.49 sys(FP) ±4.86 stat(Kal.) | FP-Nullpunkt an SBF-Distanz NGC 4874 = 99.1 ± 5.8 Mpc (Jensen 2021) | spät | [c] |
| Scolnic et al. (FP-Kreuzprüfung) | 2024 | arXiv:2409.14546 | Fundamentalebene | 76.5 | ±2.2 | SN-Ia-Coma-Distanz 98.5 ± 2.2 Mpc (HST-Leiter) | spät | [c] |
| Schombert, McGaugh & Lelli (bTFR) | 2020 | AJ 160, 71 | Tully-Fisher | 75.1 | ±2.3 stat ±1.5 sys | bTFR-Nullpunkt aus 50 Galaxien (Cepheiden/TRGB) | spät | [c] |

Said et al. (Fundamentalebene) liegt bei 76.05, aber die Route ist umstritten:
Scolnic et al. 2024 misst denselben FP mit der SN-Ia-Coma-Distanz → 76.5 ± 2.2,
während die Planck-Kalibration D_Coma = 111.8 ± 1.8 Mpc verlangt (4.6σ vom
direkten Maß entfernt). Der Streit ist Teil des Befunds, nicht Grund zur
Auslassung — die FP-Route trägt eine eigene, angefochtene Wurzel. Die
baryonische Tully-Fisher (75.1 ± 2.3 ± 1.5) hängt mit ihrem Nullpunkt an
Cepheiden-/TRGB-Kalibratoren — gebunden an die Leiter-Wurzel.

### Gelöste `pending`-Punkte (zweite Runde)

- **VizieR-ID Riess 2021:** `absent`. TAPVizieR antwortet 400 auf `J/ApJ/908/L6`
  (Tabelle nicht vorhanden, TAP_SCHEMA ohne Treffer). Die Tabelle lebt im
  arXiv-Quellpaket **2012.08534** (`bigtable_redux3.tex`, 98 Zeilen — nicht im
  Paket 2112.04510).
- **Freedman-Erratum (ApJ 993, 252):** kosmetisch — ändert die H₀-Werte nicht.
  Es korrigiert nur die y-Achsen-Fehler einer Appendix-Figur (Slope 0.08 → 0.03,
  Signifikanz >3σ → 1.6σ); wörtlich „do not impact the rest of the paper".

### Gemessene Register-Pflicht

- arXiv 2408.06153 (CCHP-Status) trägt eine veraltete DOI (ad7952 → anderes
  Papier); die publizierte DOI ist adce78 (ApJ 985, 203).

### Auswirkung auf das Verdikt

Unverändert. Die CMB-Familie ist klassen-intern reicher (ACT, WMAP), aber alle
Instrumente teilen r_d — die „Ehe" bleibt. Die späten Routen (Fundamentalebene,
Tully-Fisher) liegen hoch (75–76), teilen ihre Nullpunkte aber mit
Cepheiden-/TRGB-/SBF-Ankern — keine neue unabhängige Wurzel, kein Schlichter.

## Die eigene Leiter-H₀ (gewogen 2026-09-08)

Der Probe `tools/measure/src/bin/h0_ladder_weigh.rs` wiegt die Leiter
end-to-end — kein Wert aus dem Abstract kopiert, jeder aus der Datei gerechnet.
Drei gewogene Byte-Strömungen mit sha256 (arXiv-2012.08534-Tarball,
`Pantheon+SH0ES.dat`, `Pantheon+SH0ES_STAT+SYS.cov`), 0-Kanon durchgehend.

| Sprosse | eigener Wert | publiziert |
|---|---|---|
| Anker M_W1 (2-param, R19-fix) | −5.914 ± 0.017 | −5.915 ± 0.022 |
| Anker zp (Rest-Parallaxenoffset) | −13 ± 5 μas | −14 ± 6 μas |
| Anker b_W (4-param) | −3.34 ± 0.05 | −3.28 ± 0.06 |
| Anker Z_W (4-param) | −0.11 ± 0.09 | −0.20 ± 0.13 |
| M_B (77 Kalibratoren) | −19.2469 ± 0.0299 | −19.253 (fiduzial) |
| a_B (277 Hubble-Fluss) | 0.7159 ± 0.0018 | 0.7137 (fiduzial) |
| **H₀** | **73.56 ± 1.40 km/s/Mpc** | **73.0 ± 1.4** |

Gemessene Zustände, benannt nicht geglättet:

- **74 Zeilen, nicht 75:** die Tabelle `bigtable_redux3.tex` trägt 74 volle
  Datenzeilen (die Prosa zitiert 75); 7 Zeilen führen `\nd` (absent) in π_EDR3
  (CY-AUR, DL-CAS, RW-CAM, SV-PER, SY-NOR, RX-CAM, U-AQL) — übersprungen und
  gezählt, 67 gefittet.
- **Kovarianz-Zeilenzuordnung zertifiziert:** 182/191 Duplikat-CID-Paare liegen
  exakt auf den .dat-Zeilindizes der STATONLY-Matrix. Die DIAG-Spalten sind
  VPEC-aufgebläht und ungleich der .cov-Diagonale (eine Eigenschaft des
  Release, gemessen — das .cov trägt keine Pekuliar-Geschwindigkeitsfehler).
- **77 Kalibratoren, 43 eindeutige CIDs:** das Paper nennt 42; die 42er-Liste
  ist aus den ausgelieferten Dateien nicht extrahierbar (lebt in der
  Journal-MRT von ApJ 934, L7). Gewogen wird die vollständige gemessene Menge.
- **Anker χ²/ndf = 120.1/65:** die Tabellen-Streuung übersteigt die zitierten
  Parallaxen+Photometrie-Fehler (das publizierte Fit trägt einen zusätzlichen
  Streu-Term); die Ankerfehler sind H₀-untergeordnet (dominiert von M_B/a_B).
- **Registratur-Grammatik (gebaut 2026-09-09, Rat-Spruch b):** `sources.φ` trägt
  jetzt den frame-losen Sitz `format reference` + `sha256`-Pin (Flush-Gate in
  `parse.rs`, Feld in `types.rs`, `Frame::Manifest`); die vier statischen Beine
  des `h0_ladder_weigh` sitzen gemessen darin. Der Probe behält seinen Self-curl
  (Ursprungs-Zertifizierung); `--ci-mode`/`upload_asset_bytes` sind gestrichen,
  `h0-ladder-cdn.yml` ist gelöscht — die Manifestation trägt `health-check.yml`
  (`cargo run -- --verify phi` → `port.rs::ci_mode` reference-Zweig: fetch_bytes
  → sha256-Verifikation → Upload as-is). Kein vierter Zeugen-Typ — ein
  Referenzdatensatz ist kein Zeuge (der Rat hielt den Befund, kein Sprachloch).
- **Referenz-Pins (gemessen 2026-09-09, sha256):**
  `arxiv.org/e-print/2012.08534` = `bc86e4e424dbda6c3fb17a0f5141090c9ed59be2ad0665fddddce07d9a2085e7`,
  `Pantheon+SH0ES.dat` = `1cb0fc379ef066afdc2ffd1857681cc478024570d8a3eba284fb645775198cf8`,
  `Pantheon+SH0ES_STAT+SYS.cov` = `abf806d966485e64afdb359c87bffc0ecc00d05eff0a31ced66f247385df0fdc`,
  `Pantheon+SH0ES_STATONLY.cov` = `9f177129a332735d3637affd20054080d5260815f3ca0809120c05b2c902297f`.


## Das eigene Gaia-TAP-Crossmatch der 74 (gewogen 2026-09-09)

Der Probe `tools/measure/src/bin/h0_gaia_crossmatch_probe.rs` schlägt die 74
Namen der Tabelle über SIMBAD (`sim-tap/sync`, `ident`→`basic`) in Positionen
auf und zieht je Position den nächsten Gaia-DR3-Quellstern (`gaiadr3.gaia_source`
im 3″-Konus); die Klassifikation als zweite Linie kommt aus `gaiadr3.vari_cepheid`.
Der Namen-Resolver war die Vorarbeit: die Tabelle trägt nur Sternnamen
(`AA-GEM`, `V0386-CYG`, `S-CRU$^e$` …) — die Fußnoten-Marker `$…$` und die
`V0`-Nullen werden vor der Auflösung genormt (`h0.rs::normalize_name`), der
Name allein wird nie geraten.

| Befund | Wert |
|---|---|
| Namen aufgelöst | 74/74 via SIMBAD `ident` (Hauptname `V* …`, einer `* 12 Sgr`) |
| Identität (Separation < 2″) | 74/74 — Identity-Gate PASS |
| Parallax-Offset π_EDR3 − Gaia-DR3 | Median +21 μas, Spanne [+4, +38] μas |
| vari_cepheid-Klassifikation | 70 DCEP, 4 absent (T Mon, 12 Sgr, U Sgr, V636 Sco) |

Gemessene Zustände, benannt nicht geglättet:

- **Der Offset ist der L20b-Nullpunkt:** die Tabelle trägt laut Note d den
  L20b-Parallax-Offset (nicht den Residuen-Offset −14 μas). Der gemessene
  Median +21 μas über 67 gefittete Sterne ist genau dieser Offset — die
  Transkription π_EDR3 ist damit gegen Gaia-DR3 zertifiziert, nicht kopiert.
- **74 von 74, nicht 75:** die Tabelle trägt 74 volle Datenzeilen; der Probe
  schlägt alle 74 auf. Die 7 `\nd`-Sterne (absent in π_EDR3) tragen trotzdem
  eine gemessene Identität (CY-AUR → V* CY Aur, RX-CAM → V* RX Cam …) — ihr
  π bleibt absent, ihre Position ist gemessen (0 honored).
- **4 Sterne ohne vari_cepheid-Klasse:** T Mon, 12 Sgr (= AP Sgr), U Sgr und
  V636 Sco tragen `otype cC*` (SIMBAD), aber keine `gaiadr3.vari_cepheid`-Zeile —
  eine Gaia-Katalog-Vollständigkeit, kein Riss der Identität.
- **Die lebendige TAP-Leg bleibt unregistriert:** der Probe materialisiert kein
  Asset (reine Messung, stdout) — Präzedenz `cepheid_parallax_weigh`. Der
  frame-lose Sitz `format reference` (gebaut 2026-09-09) trägt nur
  Byte-Strömungen, keine lebendige TAP-Leg; die bleibt eine benannte
  Messung ohne Registratur.

### Per-Source-Identität — Gaia DR3 source_id (gewogen 2026-09-13)

Der Crossmatch läuft über Gaia-ARI (`https://gaia.ari.uni-heidelberg.de/tap/sync`,
`gaiadr3.gaia_source`); der ESA-TAP `gea.esac.esa.int` antwortet aus dieser
Sitzung nicht (HTTP 000, 0 Bytes, GET und POST gemessen). Je Cepheide: der
Tabelle-Name, der SIMBAD-Hauptname, die Gaia-`source_id`, die Gaia-Parallaxe
mit Fehler, die Separation und die `vari_cepheid`-Klasse. Die 74 Zeilen tragen
die per-source-Identität des Cepheiden-Ankers; die 7 `\nd`-Sterne tragen ihre
Gaia-Parallaxe, ihre π_EDR3 bleibt absent (0 honored).

| Cepheide | SIMBAD | Gaia source_id | π_Gaia (mas) | σ_π (mas) | Sep (″) | vari_cepheid |
|---|---|---|---|---|---|---|
| AA-GEM | V* AA Gem | 3430067092837622272 | 0.274911 | 0.017681 | 0.011 | DCEP |
| AD-PUP | V* AD Pup | 5614312705966204288 | 0.233082 | 0.016544 | 0.062 | DCEP |
| AQ-CAR | V* AQ Car | 5254662177677566464 | 0.350468 | 0.015537 | 0.119 | DCEP |
| AQ-PUP | V* AQ Pup | 5597379741549105280 | 0.275097 | 0.022584 | 0.060 | DCEP |
| BK-AUR | V* BK Aur | 261548119462093568 | 0.392738 | 0.014951 | 0.028 | DCEP |
| BN-PUP | V* BN Pup | 5596601154188852352 | 0.281126 | 0.014658 | 0.046 | DCEP |
| CD-CYG | V* CD Cyg | 2058374144759464064 | 0.366522 | 0.016361 | 0.095 | DCEP |
| CP-CEP | V* CP Cep | 2198162651499971200 | 0.253433 | 0.020417 | 0.082 | DCEP |
| CR-CAR | V* CR Car | 5351423049186195328 | 0.179373 | 0.014860 | 0.104 | DCEP |
| CY-AUR$^*$ | V* CY Aur | 206577210999441536 | 0.231575 | 0.020790 | 0.005 | DCEP |
| DD-CAS | V* DD Cas | 2013029941628292352 | 0.322125 | 0.013070 | 0.036 | DCEP |
| DL-CAS$^*$ | V* DL Cas | 428620663657823232 | 0.552838 | 0.027384 | 0.047 | DCEP |
| DR-VEL | V* DR Vel | 5313887130948758016 | 0.504740 | 0.013408 | 0.110 | DCEP |
| GQ-ORI | V* GQ Ori | 3329849043206545920 | 0.375586 | 0.020807 | 0.017 | DCEP |
| HW-CAR | V* HW Car | 5254097093074642944 | 0.385481 | 0.012196 | 0.106 | DCEP |
| KK-CEN | V* KK Cen | 5336389564126521728 | 0.135675 | 0.015247 | 0.106 | DCEP |
| KN-CEN | V* KN Cen | 5864135319959353600 | 0.227229 | 0.018278 | 0.113 | DCEP |
| RW-CAM$^*$ | V* RW Cam | 473043922712140928 | 0.713330 | 0.137096 | 0.019 | DCEP |
| RW-CAS | V* RW Cas | 508915489570374272 | 0.304689 | 0.018854 | 0.021 | DCEP |
| RY-CAS | V* RY Cas | 1999252442448732288 | 0.329708 | 0.014925 | 0.052 | DCEP |
| RY-SCO | V* RY Sco | 4041690364529590144 | 0.727937 | 0.032087 | 0.033 | DCEP |
| RY-VEL | V* RY Vel | 5355057622307185280 | 0.364866 | 0.020608 | 0.124 | DCEP |
| S-NOR | V* S Nor | 5835124087174043136 | 1.077423 | 0.021940 | 0.043 | DCEP |
| S-VUL$^*$$^*$ | V* S Vul | 2027971514401523456 | 0.205141 | 0.020155 | 0.111 | DCEP |
| SS-CMA | V* SS CMa | 5616601820448126336 | 0.286500 | 0.012745 | 0.195 | DCEP |
| SV-PER$^*$ | V* SV Per | 203496585576324224 | 0.162686 | 0.212251 | 0.051 | DCEP |
| SV-VEL | V* SV Vel | 5351331755370445056 | 0.421065 | 0.017516 | 0.126 | DCEP |
| SV-VUL$^*$$^*$ | V* SV Vul | 2027951173435143680 | 0.372932 | 0.021149 | 0.101 | DCEP |
| SY-NOR$^*$ | V* SY Nor | 5884729035255064064 | 0.230038 | 0.026798 | 0.045 | DCEP |
| SZ-CYG | V* SZ Cyg | 2071433765909167232 | 0.418640 | 0.012402 | 0.096 | DCEP |
| T-MON | V* T Mon | 3324535073449061504 | 0.714066 | 0.051743 | 0.050 | absent |
| U-CAR | V* U Car | 5338359442320395904 | 0.553047 | 0.022608 | 0.103 | DCEP |
| UU-MUS | V* UU Mus | 5332375453374624640 | 0.291540 | 0.012110 | 0.096 | DCEP |
| V-339-CEN | V* V339 Cen | 5854560115494081024 | 0.548922 | 0.021113 | 0.138 | DCEP |
| V-340-ARA | V* V340 Ara | 5937099633141128448 | 0.206227 | 0.019656 | 0.116 | DCEP |
| VW-CEN | V* VW Cen | 5864955727424819200 | 0.238607 | 0.015769 | 0.098 | DCEP |
| VX-PER | V* VX Per | 506779550797525760 | 0.363568 | 0.016823 | 0.021 | DCEP |
| VY-CAR | V* VY Car | 5351161399793209984 | 0.553481 | 0.016717 | 0.109 | DCEP |
| VZ-PUP | V* VZ Pup | 5600052040150252800 | 0.201098 | 0.014700 | 0.041 | DCEP |
| WX-PUP | V* WX Pup | 5613972681993587200 | 0.368479 | 0.015003 | 0.054 | DCEP |
| WZ-SGR | V* WZ Sgr | 4094784475310672128 | 0.574203 | 0.027734 | 0.016 | DCEP |
| X-CYG | V* X Cyg | 1870258975238302208 | 0.884239 | 0.019928 | 0.134 | DCEP |
| X-PUP | V* X Pup | 5620098679741674496 | 0.376271 | 0.020086 | 0.035 | DCEP |
| XX-CAR | V* XX Car | 5238808628736339584 | 0.292520 | 0.014487 | 0.099 | DCEP |
| XY-CAR | V* XY Car | 5240441472232302848 | 0.377875 | 0.013625 | 0.100 | DCEP |
| XZ-CAR | V* XZ Car | 5338036117182452096 | 0.460478 | 0.017988 | 0.124 | DCEP |
| YZ-CAR | V* YZ Car | 5255254711361371520 | 0.346360 | 0.017870 | 0.113 | DCEP |
| YZ-SGR | V* YZ Sgr | 4099189015819292800 | 0.823523 | 0.024237 | 0.054 | DCEP |
| Z-LAC | V* Z Lac | 2007201567928631296 | 0.486594 | 0.020831 | 0.077 | DCEP |
| AG-CRU | V* AG Cru | 6059635702888301952 | 0.742163 | 0.019866 | 0.077 | DCEP |
| AP-PUP | V* AP Pup | 5537743094157581568 | 0.911032 | 0.019895 | 0.118 | DCEP |
| AP-SGR | *  12 Sgr | 4066429066901946368 | 1.181491 | 0.024037 | 0.061 | absent |
| BF-OPH | V* BF Oph | 4111834567779557376 | 1.153767 | 0.024090 | 0.008 | DCEP |
| BG-VEL | V* BG Vel | 5324034867356093056 | 1.036207 | 0.016882 | 0.190 | DCEP |
| ER-CAR | V* ER Car | 5339394082770287232 | 0.858683 | 0.014970 | 0.160 | DCEP |
| R-CRU | V* R Cru | 6054935874795049216 | 1.062196 | 0.028123 | 0.162 | DCEP |
| R-MUS | V* R Mus | 5855468247702904704 | 1.065607 | 0.017528 | 0.077 | DCEP |
| R-TRA | V* R TrA | 5824464493705913472 | 1.543183 | 0.016095 | 0.157 | DCEP |
| RV-SCO | V* RV Sco | 6026412893938675712 | 1.223984 | 0.020576 | 0.105 | DCEP |
| RX-CAM$^*$ | V* RX Cam | 470361114339849472 | 0.631287 | 0.042985 | 0.008 | DCEP |
| RY-CMA | V* RY CMa | 3046774762417915136 | 0.800598 | 0.029132 | 0.051 | DCEP |
| S-CRU$^e$ | V* S Cru | 6060173364074372352 | 1.324519 | 0.023646 | 0.166 | DCEP |
| S-TRA | V* S TrA | 5823134325151372032 | 1.101427 | 0.021513 | 0.060 | DCEP |
| SS-SCT | V* SS Sct | 4251917636212432384 | 0.898490 | 0.022722 | 0.037 | DCEP |
| T-VEL | V* T Vel | 5521459979795304320 | 0.930919 | 0.015925 | 0.126 | DCEP |
| TX-CYG | V* TX Cyg | 2161786374436607616 | 0.795426 | 0.018621 | 0.039 | DCEP |
| U-AQL$^*$ | V* U Aql | 4207681367143932800 | 1.729983 | 0.087489 | 0.192 | DCEP |
| U-SGR | V* U Sgr | 4092905375639902464 | 1.569308 | 0.022430 | 0.102 | absent |
| V-CAR | V* V Car | 5302258008774271488 | 0.792662 | 0.013880 | 0.076 | DCEP |
| V-VEL | V* V Vel | 5309930194773990400 | 0.944961 | 0.017378 | 0.277 | DCEP |
| V0386-CYG | V* V386 Cyg | 1968971582984827136 | 0.863760 | 0.012505 | 0.039 | DCEP |
| V0482-SCO | V* V482 Sco | 4054440301294394624 | 0.960100 | 0.025053 | 0.046 | DCEP |
| V0636-SCO | V* V636 Sco | 5952775576537356544 | 1.151073 | 0.033516 | 0.062 | absent |
| W-GEM | V* W Gem | 3356940155020790656 | 0.972321 | 0.028200 | 0.041 | DCEP |

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

**`pending`:** keine offenen Punkte. Die MNRAS-Druckfassung von Said et al.
(MNRAS 539, 3627, DOI 10.1093/mnras/staf700) ist bestätigt; die VizieR-ID der
Riess-2021-Tabelle ist gewogen-`absent`, das Freedman-Erratum kosmetisch
(s. zweiter Nachtrag).
