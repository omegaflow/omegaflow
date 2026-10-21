<!--
  title: Survey — Frame-Wache: ICRF3 S/X, K, X/Ka und Gaia-CRF3, selbst gezählt (2026-10-20)
  class: survey
  date: 2026-10-20
  sha256: e653326c6b46c2204eb62b1fc3a393d915dfeab39801f6844ac685021e75dfe4
  status: live
  see-also: tools/measure/src/bin/frame_wache_probe.rs
-->
# Survey — Frame-Wache: ICRF3 S/X, K, X/Ka und Gaia-CRF3

**Anlass:** Ein externer Text (GLM, vom Operator als Vorlage gegeben) nannte
ICRF3-Zahlen (4.536/824/678 Quellen, 600 Dreifach-Anker, 171 D×3, Median 2,86σ,
64 Quellen >5σ, 0743-006 mit 10,3σ). Nichts davon war im Haus. Dieser Survey
**zählt selbst** aus den offiziellen Katalogen; wo die Vorlage abweicht, gilt die
eigene Messung — und die Abweichung wird benannt, nicht geglättet.

**Vorzeichen-Fix vorab:** Die Deklination wird aus dem Vorzeichen des
Grad-Tokens gelesen (`-00` ist negativ), nicht aus dem Zahlenwert `dd < 0.0`.
Der Unterschied ist nicht kosmetisch: `-00 44 17.5` wird sonst zu `+0.738°`,
also 1,476° daneben. In allen drei Radio-Katalogen gleichsinnig falsch bleibt
der Fehler im Paarvergleich fast unsichtbar (0743-006: 12,93σ statt 17,02σ),
gegen Gaia wäre er katastrophal. Die Probe trägt den Fix an einer Stelle.

**Probe:** `tools/measure/src/bin/frame_wache_probe.rs` (neu, dieser Survey).
Aufruf aus `omegaflow-work`: `cargo run --release -p omegaflow-measure --bin
frame_wache_probe`. Rohdaten liegen unter `sources/<netloc>/`; die
Provenienz steht in `sources/cdsarc.cds.unistra.fr/PROVENANCE-frame-wache.md`.

## Kanäle (gemessen, nicht angenommen)

| Kanal | URL | HTTP | Bytes | sha256 |
|---|---|---:|---:|---|
| ICRF3 S/X | `https://hpiers.obspm.fr/icrs-pc/icrf/icrf3sx.txt` | 200 | 745.282 | `83abfa7c…bab3bb` |
| ICRF3 K | `https://hpiers.obspm.fr/icrs-pc/icrf/icrf3k.txt` | 200 | 136.511 | `ef653890…df49b9a` |
| ICRF3 X/Ka | `https://hpiers.obspm.fr/icrs-pc/icrf/icrf3xka.txt` | 200 | 112.570 | `22e89235…a27aa19` |
| Gaia-CRF3 Crossmatch | `https://cdsarc.cds.unistra.fr/ftp/cats/I/355/gcrf3xm.dat.gz` | 200 | 57.852.137 | `9ea2ac57…531300` |
| Gaia-Positionen (TAP) | `I/355/gaiadr3`, 512 IDs | 200 | 33.004 | `e4a9bff3…54f1e7` |

Die ICRF3-Köpfe nennen 4536/824/678 Quellen. Die geparsten Datenzeilen sind
4536/824/678 — kein Kanal fehlt, kein Kopf wird ungeprüft übernommen. Gaia
liefert 1.614.173 Crossmatch-Zeilen; 512 der 600 Dreifach-Anker haben eine
Gaia-CRF3-Zuordnung (85,3 %), 88 nicht.

## Bilanz

| | Zahl |
|---|---:|
| ICRF3 S/X-Quellen | **4.536** |
| ICRF3 K-Quellen | **824** |
| ICRF3 X/Ka-Quellen | **678** |
| Dreifach-Schnitt | **600** |
| davon D in allen drei Bändern | **171** |
| D in S/X (breitere Rahmen-Ebene) | **303** |
| davon mit K-Position | **193** |
| davon mit X/Ka-Position | **176** |
| S/X–X/Ka Median-σ (2D, korreliert) | **2,930** |
| S/X–X/Ka >5σ | **111** |
| 0743-006 σ (S/X–X/Ka) | **17,02** |
| Gaia-Abdeckung des Dreifach-Schnitts | **512 / 600** |

Die Strukturzahlen 4.536/824/678/600/171 sind exakt reproduziert. Der externe
Median 2,86σ liegt nahe an den gemessenen 2,930σ, ist aber nicht identisch. Die
externen 64 Quellen >5σ sind **nicht** reproduziert: die eigene Zählung ergibt
111 (korrelierte 2D-Metrik) bzw. 88 (unkorrelierte 2D-Metrik). Auch 0743-006
liegt mit 17,02σ nicht bei 10,3σ. Die Abweichung wird nicht weggeglättet; die
Reproduktion ist das Ergebnis.

## Methode

Für ein Quellenpaar (a,b) wird im Tangentialraum um die mittlere Position
gerechnet:

- `Δα* = (α_a − α_b)·cos δ`, `Δδ = δ_a − δ_b`
- `C = C_a + C_b`, mit `C = [[σ_α², ρσ_ασ_δ],[ρσ_ασ_δ, σ_δ²]]`
- `σ = sqrt(Δᵀ C⁻¹ Δ)` (2D-Mahalanobis, 2 Freiheitsgrade)

`σ_α` ist die ICRF3-Fehlerangabe in Sekunden Zeit × 15 × cos δ; `σ_δ` in
Bogensekunden. Die Korrelation `ρ` wird aus beiden Katalogen summiert. Die
unkorrelierte Variante (ρ=0) steht als Kontrolle daneben. Ein Median von 1,18
wäre bei korrekten Fehlern und 2D-Gauß zu erwarten; die gemessenen Werte zeigen
unterschätzte Formfehler oder echte Quellstruktur.

| Paar | n | Median σ | >3σ | >5σ | max σ |
|---|---:|---:|---:|---:|---:|
| S/X – K | 600 | 1,293 | 53 | 16 | 17,82 |
| K – X/Ka | 600 | 2,594 | 221 | 27 | 7,99 |
| S/X – X/Ka | 600 | 2,930 | 291 | 111 | 21,72 |
| Gaia – S/X | 512 | 2,178 | 165 | 75 | 29,87 |

Kontrolle ohne Korrelationen: S/X–K 1,281/16, K–X/Ka 2,480/26, S/X–X/Ka
2,900/88 (Median/>5σ). Die externe 2,86 liegt knapp unter beiden Varianten; die
externen 64 >5σ werden von keiner der beiden getragen. Eine reine
Achsenzählung (|Δα*/σ_α|>5 oder |Δδ/σ_δ|>5) ergibt 57 — näher an 64, aber ein
anderes Kriterium, kein 2D-Schnitt.

## 0743-006 (D×3)

`J074554.0-004417` = IERS `0743-006`, D in S/X, K und X/Ka.

| Paar | σ | Abstand |
|---|---:|---:|
| S/X – K | 3,95 | — |
| K – X/Ka | 7,38 | — |
| S/X – X/Ka | **17,02** | **1,246 mas** |

Die externe 10,3σ ist damit nicht reproduziert. Mit dem `-00`-Fehler (alle drei
Kataloge gleichsinnig um +1,476° verschoben) ergäbe dieselbe Rechnung 12,93σ —
auch das ist nicht 10,3. Die Abweichung ist methodisch, nicht durch den
Vorzeichen-Fix allein erklärbar; eine andere Fehlerkombination oder ein anderer
Katalogstand ist die naheliegende, hier nicht belegte Erklärung.

## Zwei getrennte Listen

### Struktur-Kandidaten (einzelne Quellen)

Kriterium: mindestens ein Paar-Sigma >5 in der korrelierten 2D-Metrik. Das ist
eine Aussage über die Quelle (Kernverschiebung zwischen Wellenlängen), nicht
über den Rahmen. D-Flags werden mitgeführt, damit ein D×3-Ausreißer nicht
automatisch als Rahmen-Riß gelesen wird.

| Quelle | IERS | D S/X,K,X/Ka | S/X–K | K–X/Ka | S/X–X/Ka |
|---|---|---|---:|---:|---:|
| J072550.6-005456 | 0723-008 | —, —, — | 15,47 | 4,77 | **21,72** |
| J213638.5+004154 | 2134+004 | —, —, — | **17,82** | 4,48 | 16,55 |
| J074554.0-004417 | 0743-006 | D, D, D | 3,95 | 7,38 | **17,02** |
| J160207.2+332653 | 1600+335 | —, —, — | 6,69 | **7,99** | 15,98 |
| J043311.0+052115 | 0430+052 | —, —, — | **12,70** | 3,08 | 11,29 |
| J014922.3+055553 | 0146+056 | —, —, — | 7,75 | 5,41 | **12,62** |
| J115951.9-214853 | 1157-215 | —, —, — | **10,16** | 2,97 | 8,23 |

(Die vollständige Liste druckt die Probe; sie enthält **120** Quellen mit
maximalem Paar-Sigma >5. Die Tabelle zeigt die sieben stärksten.)

### Rahmen-Verdacht (nur Ensemble der D3-Quellen)

Kriterium: kleine Rotation `ω` zwischen zwei Rahmen, gefittet über die 171
Quellen, die in allen drei Bändern definierend sind; Unsicherheit aus der
Normalmatrix, skaliert mit `chi2/dof`. Ein einzelner Ausreißer ist **kein**
Rahmen-Verdacht.

| Paar | n | ωx | ωy | ωz | \|ω\| | chi2/dof | Urteil |
|---|---:|---:|---:|---:|---:|---:|---|
| S/X – K | 171 | +0,009 ± 0,011 | +0,007 ± 0,011 | +0,009 ± 0,007 | 0,014 | 1,2 | Placed |
| K – X/Ka | 171 | +0,015 ± 0,026 | +0,003 ± 0,027 | −0,034 ± 0,017 | 0,038 | 4,9 | Placed |
| S/X – X/Ka | 171 | +0,031 ± 0,025 | +0,019 ± 0,026 | −0,028 ± 0,018 | 0,046 | 7,9 | Placed |
| Gaia – S/X | 148 | −0,004 ± 0,024 | −0,009 ± 0,025 | −0,029 ± 0,026 | 0,030 | 7,8 | Placed |

Alle Komponenten liegen unter 3σ; es gibt **keinen** Rahmen-Riß. Die
`chi2/dof`-Werte 4,9–7,9 sagen etwas anderes: die formalen Fehler sind zu
klein oder die Quellen tragen echte Struktur. Das ist ein Struktur-Befund,
kein Rahmen-Befund.

### 303 D in S/X — die breitere Rahmen-Ebene (gemessen 2026-10-21)

Der D3-Fit nutzt nur die 171 Quellen, die in **allen drei** Bändern D tragen.
Die breitere Menge „D in S/X" umfasst **303** Quellen. Davon haben **193** eine
K-Position und **176** eine X/Ka-Position. Der Rotationsfit über diese Mengen:

| Paar | n | ωx | ωy | ωz | \|ω\| | chi2/dof | Urteil |
|---|---:|---:|---:|---:|---:|---:|---|
| S/X – K | 193 | +0,008 ± 0,010 | +0,008 ± 0,011 | +0,008 ± 0,007 | 0,014 | 1,2 | Placed |
| S/X – X/Ka | 176 | +0,032 ± 0,025 | +0,019 ± 0,025 | −0,028 ± 0,018 | 0,046 | 7,7 | Placed |

Alle Komponenten liegen unter **2σ**; die Rahmen-Ebene bleibt **Placed**.
`chi2/dof = 7,7` beim S/X–X/Ka-Paar bestätigt den Struktur-Befund: Die formalen
Fehler tragen die Streuung nicht. Die **110** D-in-S/X-Quellen ohne K-Position
und **127** ohne X/Ka-Position sind eine benannte Abdeckungslücke, kein Riss.

## Unabhängigkeit (benannt, nicht behauptet)

S/X, K und X/Ka sind VLBI-Messungen, häufig mit denselben Teleskopen,
demselben Quellensatz und überlappenden Beobachtungszeiträumen. Der
Dreifach-Schnitt ist deshalb ein **Konsistenztest** der ICRF3-Konstruktion,
kein Unabhängigkeitsbeweis. Gaia-CRF3 ist die optische Gegenlinie: andere
Wellenlänge, andere Instrumente, andere Systematik. Aber auch sie ist kein
absolut unabhängiger Rahmen — Gaia-CRF3 wurde zur Orientierung an ICRF3
ausgerichtet. Sie prüft daher die Quellenpositionen und die optisch-radio
Verschiebung, nicht den Ursprung des Rahmens. 88 der 600 Anker haben keine
Gaia-Zuordnung; die Gaia-Aussage gilt für 512 Quellen, nicht für den ganzen
Schnitt. Gaia-Positionen sind Epoche 2016; die Radio-Positionen sind
Epochenmittel, überwiegend 1990er. Eine QSO-Eigenbewegung wird nicht
modelliert (Größenordnung 0,1 mas über 25 Jahre, als Grenze notiert).

## Wache als Vorschlag

**Trigger** (jeder einzeln):

1. neue ICRF-Version oder Update von `icrf3sx/k/xka` (Hash-Abweichung),
2. neue Gaia-Datenfreigabe (DR4) oder neue Gaia-CRF,
3. vor Phase 1 eines Papiers, das den ICRS/Rahmen berührt,
4. Abweichung eines der fünf sha256 in der Provenienz.

**Probe je Trigger:** neu laden, Größe/Status/sha messen, Köpfe gegen
Datenzeilen prüfen (4.536/824/678), Vorzeichen-Fix anwenden, Dreifach-Schnitt
und D3 zählen, Median und >5σ je Paar, Struktur-Liste, Rahmen-Rotation über
D3 **und über die 303 D in S/X**, Gaia-Abdeckung und Gaia-Rotation.

**Urteil:** Placed ≤3σ; Beobachten 3–5σ; Riß >5σ **nur auf Rahmen-Ebene**
(Ensemble der definierenden Quellen). Einzelquellen >5σ bleiben
Struktur-Kandidaten und lösen keinen Riß aus.

**Tagebuch-Zeile:**

`2026-10-20 frame-wache: ICRF3 4536/824/678; triple 600; D3 171; S/X–X/Ka median 2,93σ, >5 111; Rahmenrotation max 2,0σ -> Placed; 303-D: n 193 (K) / 176 (X/Ka), max 1,5σ, chi2/dof 7,7 -> Placed; Gaia 512/600, median 2,18σ; kein Riß, Struktur offen.`

## Was quer liegt

- Die externen 64 >5σ sind unter keiner der geprüften Metriken reproduziert
  (111 korreliert, 88 unkorreliert, 57 Achsen-Union). Die Abweichung ist groß
  genug, dass die externe Liste nicht als Beleg übernommen werden darf.
- 0743-006 ist mit 17,02σ ein starker Struktur-Kandidat, kein Rahmen-Riß. Die
  extern genannte 10,3σ bleibt unerklärt.
- 88 Dreifach-Anker ohne Gaia-CRF3-Zuordnung sind eine benannte Lücke; die
  Gaia-Linie deckt 85,3 % des Schnitts.
- Die 303-D-Rahmenrotation deckt nur **193** (K) bzw. **176** (X/Ka) der 303
  Quellen; 110 bzw. 127 haben dort keine Position. Das ist eine
  Abdeckungslücke, kein Riss.
- Die `chi2/dof`-Werte der Rahmen-Fits (bis 7,9) zeigen, dass die formalen
  Fehlerbalken die Streuung nicht tragen. Eine echte Rahmen-Aussage mit
  Fehlerbalken braucht eine Skalierung oder ein robusteres Schätzverfahren;
  beides ist hier nicht behauptet.
