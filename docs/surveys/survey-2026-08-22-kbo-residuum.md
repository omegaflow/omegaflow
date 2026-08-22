<!--
  title: Das Blatt des unsichtbaren Begleiters — das Gravitations-Residuum der KBO-Bahnen (Nadel VI)
  class: survey
  date: 2026-08-22
  sha256: c8d590ff28f567ab52d4641de1f16863ca51ca59b58419e161d78e053dd0f09d
  status: live
  see-also: archive/handover/handover-2026-08-22-planet-neun.md
-->

# Das Blatt des unsichtbaren Begleiters — das Gravitations-Residuum der KBO-Bahnen (Nadel VI)

Titel: Das Gravitations-Residuum der KBO-Bahnen
R(orbit) = Bahn_geerntet − Bahn_Modell(planeten)     = gemessen
TE(Residuum → Bahn) je Familie                       = gemessen
Lag + Richtung des Pfeils                            = gemischt, kein fam-Träger
n (KBOs, Fenster)                                    = 7180 geerntet, 7116 mit Serie
Verdikt: das Residuum ist still unter der Familien-Schwelle — kein fam-tragender Pfeil.

## Die Konstruktion

R(t) = |Kepler(Bahn_geerntet) − N-Körper(Sun+8, Leapfrog, Drift-Kick, dt 30 d)| in AU;
Bahn = ϖ(t) aus dem N-Körper-Lauf; 256 Samples je Objekt; Fenster ±400 yr
(Deckungs-Bound der ephemeris-Bins 2305328.50–2816848.50 JD, einheitlich je Familie);
Mittel-Reihe je Familie. TE-Schwelle: phasenrandomisierte Surrogate (f64-FFT,
10 Realisierungen, mean + 2σ); fam = max Surrogat-Schwelle der Runde.
Bahn_geerntet = Kepler-Propagation der geernteten Oskulationselemente (kepler.rs);
Bahn_Modell = Leapfrog mit Planeten-Zuständen aus den ephemeris-Bins
(body_barycenter_position, GM aus BodyProperties, SI). Der Katalog kommt aus
kbo_elements.json (CDN-Asset, sources.φ-Block keplermap) — 7180 Objekte,
MPC-Distant-Kreuzcheck: 4802 agree / 912 disagree / 1328 epoch-fern.

## Die Messung (fam = 7.2822e-1)

| Familie | n | TE | Richtung | Lag | Schwelle | Wort |
|---|---|---|---|---|---|---|
| übrig | 661 | 5.3526e-1 | ϖ→R | 62 | 2.1925e-1 | über Schwelle, unter fam |
| klassisch | 2854 | 1.8055e-1 | R→ϖ | 58 | 2.5576e-1 | still |
| gestreut | 1663 | 3.6958e-1 | ϖ→R | 36 | 2.2705e-1 | über Schwelle, unter fam |
| 3:2 | 601 | 6.0680e-1 | ϖ→R | 43 | 4.3436e-1 | über Schwelle, unter fam |
| 2:1 | 211 | 5.1751e-1 | R→ϖ | 54 | 5.8033e-1 | still |
| 5:2 | 137 | 6.1317e-1 | R→ϖ | 64 | 5.8391e-1 | über Schwelle, unter fam |
| 7:4 | 628 | 2.7336e-1 | R→ϖ | 58 | 3.4124e-1 | still |
| 4:3 | 75 | 5.2104e-1 | ϖ→R | 29 | 3.4140e-1 | über Schwelle, unter fam |
| 5:3 | 202 | 3.2096e-1 | ϖ→R | 38 | 3.6556e-1 | still |
| etno | 84 | 3.1016e-1 | R→ϖ | 63 | 2.9669e-1 | über Schwelle, unter fam |
| NK II kalt | 1060 | 1.8402e-1 | R→ϖ | 54 | 3.0938e-1 | still |

Residuum-Maße: klassisch mittel 2.32 AU (max 12.96), gestreut max 297 AU,
etno mittel 82.2 AU (max 1419 AU) — die hohen Werte tragen die chaotischen
Hoch-e-Objekte. NK I (Sun-only-Selbstlauf): max R = 9.537e-4 AU — Integrator
und Kepler-Referenz stimmen. Modell-Lücken: 64 von 7180 (Extrem-Elemente,
Serie void — gezählt, nicht ersetzt). Alle Familien n ≥ 30.

## Die Sonden — der zweite Tracer (andere Orte, gemessene Bögen)

Bahn_geerntet = das Horizons-Langfenster (Tracking-Rekonstruktion, 32-d-Raster;
Voyager 1 ab 1981-01-01, Voyager 2 ab 1989-10-01, New Horizons ab 2015-08-01 —
die Flybys bleiben draußen, Nadel II trägt sie); Modell = Sun+8 ab Fensterstart,
dt 30 d. R = |gemessen − Modell|; TE je Sonde als eigenes Objekt (n = 1 je
Bahn, die Familien-Schwelle gilt über die ganze Runde). Lag → Ort über die
c-Laufzeit (signal_reach-Gesetz des Archivars: d = lag × Sample-Tage × c).

| Sonde | Fenster | max R | TE | Richtung | Lag | Schwelle | Wort |
|---|---|---|---|---|---|---|---|
| voyager1 | 18272 d | 9.6551e-1 AU | 2.2817e-1 | ϖ→R | 64 | 1.3709e-1 | über Schwelle, unter fam |
| voyager2 | 15072 d | 1.1136e-1 AU | 2.1419e-1 | ϖ→R | 64 | 8.3484e-2 | über Schwelle, unter fam |
| new_horizons | 5632 d | 1.8943e-1 AU | 1.9135e-1 | R→ϖ | 62 | 2.6628e-1 | still |

Die Sonden-Pfeile liegen am Sweep-Rand (lag 62–64/64) und unter fam — nach
der Bz-Regel ein Rand-Befund, kein Träger; die c-Laufzeit-Karte am Rand ist
bedeutungslos. Die Residuum-Maße der Sonden (max ~1 AU über 50 yr) liegen
im Maßstab bekannter unmodellierter Effekte (Sonnendruck, RTG-Wärme,
Pioneer-artige Anomalie ~1e-10 m/s²) — benannt, nicht zugerechnet. Der
SPK-Weg (merged Voyager-Kernel) bleibt offen: der Reader trägt Typ 1
(Modified Difference Arrays, 71-Double-Records nach JPL-Memorandum 163)
nicht — das Langfenster über horizons_compiler ist die Standard-Schiene.

## Der direkte Test — die ϖ-Häufung der geernteten Bahnen (Rayleigh)

Der Wissenschafts-Test (Batygin & Brown 2016) an den geernteten Bahnen
selbst, `kbo_residue_probe --cluster-only`: Rayleigh R der Perihel-Richtungen
ϖ = Ω+ω je Familie, Null aus gleichverteilten Winkeln (1000 Realisierungen,
mean+2σ; die Permutation taugt hier nicht — R ist reihenfolge-invariant).
Ohne Bias-Korrektur (die tragen Shankman/Bernardinelli/Napier).

| Familie | n | R | ϖ̄ | Null-Schwelle | Wort |
|---|---|---|---|---|---|
| etno q>30 | 44 | 0.0583 | 68.1° | 0.2770 | still |
| etno | 84 | 0.0401 | 64.3° | 0.2014 | still |
| klassisch | 2884 | 0.0418 | 276.7° | 0.0341 | clustered |
| gestreut | 1666 | 0.0634 | 261.8° | 0.0447 | clustered |
| 5:2 | 137 | 0.1881 | 258.2° | 0.1546 | clustered |
| 7:4 | 635 | 0.0960 | 255.8° | 0.0734 | clustered |
| 3:2 | 614 | 0.0553 | 347.2° | 0.0729 | still |
| 2:1 | 211 | 0.0632 | 319.3° | 0.1253 | still |
| 4:3 | 75 | 0.1740 | 6.4° | 0.2167 | still |
| 5:3 | 212 | 0.0208 | 303.4° | 0.1232 | still |

**Die P9-Auswahl (a ≥ 250 AU, q ≥ 30 AU, n = 44) ist still — R liegt
unter dem Gleichverteilungs-Mittel: die geernteten ETNO-Bahnen sind
gestreuter als Zufall.** Die Anti-Ausrichtungs-Probe gegen die Planeten-ϖ̄
(77.3°, R = 0.57) ergibt |ϖ̄_etno − ϖ̄_planeten| = 9.3° statt ~180° —
die berühmte Sechser-Häufung (Batygin & Brown 2016) ist in der
vollständigen Ernte nicht sichtbar, im Einklang mit den Bias-Analysen.
Die schwachen Häufungen der großen Familien (klassisch/gestreut/5:2/7:4)
sind bekannte Gürtel-Struktur, kein ETNO-Befund.

## Das Verdikt

Kein Pfeil trägt über die Familien-Schwelle — weder die KBO-Familien noch
die Sonden an ihren ganz anderen Orten. Die stärksten sub-fam-Werte
tragen die Resonanz-Familien (3:2, 5:2) — der Neptun-Kick strukturiert das
Residuum, wie es das Modell erwarten lässt; die Richtungen wechseln (ϖ→R und
R→ϖ), kein konsistenter kausaler Pfeil. Der ETNO-Wert liegt am Sweep-Rand
(lag 63/64) und unter fam — ein Rand-Befund, kein Träger. Das Blatt misst
das Residuum, kein Urteil vorab: still unter der Familien-Schwelle heißt
nicht „kein Planet" — es heißt, die geernteten Bahnen und die gemessenen
Sondenbögen tragen über das Planeten-Modell hinaus keine TE-Signatur, die
den fam-Standard übersteht. Daneben stehen die Bias-Analysen (Shankman et
al. 2017 OSSOS: Clusterung von Survey-Bias dominiert; Bernardinelli et al.
2020 DES: eTNOs isotrop; Napier et al. 2021: Verteilungen mit uniformer
Population verträglich) — die Klinge spricht nicht für das Phantom, die
Grenze ist benannt.
