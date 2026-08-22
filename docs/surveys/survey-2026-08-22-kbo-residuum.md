<!--
  title: Das Blatt des unsichtbaren Begleiters — das Gravitations-Residuum der KBO-Bahnen (Nadel VI)
  class: survey
  date: 2026-08-22
  sha256: 14e29b08cfaf0d5d1bc5dca2ce281e1fe131d0aff3ee38bbc96a0ed64cda9bcd
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

## Das Verdikt

Kein Pfeil trägt über die Familien-Schwelle. Die stärksten sub-fam-Werte
tragen die Resonanz-Familien (3:2, 5:2) — der Neptun-Kick strukturiert das
Residuum, wie es das Modell erwarten lässt; die Richtungen wechseln (ϖ→R und
R→ϖ), kein konsistenter kausaler Pfeil. Der ETNO-Wert liegt am Sweep-Rand
(lag 63/64) und unter fam — ein Rand-Befund, kein Träger. Das Blatt misst
das Residuum, kein Urteil vorab: still unter der Familien-Schwelle heißt
nicht „kein Planet" — es heißt, die geernteten Bahnen tragen über das
Planeten-Modell hinaus keine TE-Signatur, die den fam-Standard übersteht.
Daneben stehen die Bias-Analysen (Shankman et al. 2017 OSSOS: Clusterung
von Survey-Bias dominiert; Bernardinelli et al. 2020 DES: eTNOs isotrop;
Napier et al. 2021: Verteilungen mit uniformer Population verträglich) —
die Klinge spricht nicht für das Phantom, die Grenze ist benannt.
