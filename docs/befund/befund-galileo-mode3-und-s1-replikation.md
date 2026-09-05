<!--
  title: Befund — Mode-3-(three-way-)Fenster über die ganze Mission und S1-Isolat-Replikation (st63 9883–9908)
  class: befund
  date: 2026-09-05
  status: done
  sha256: 24e5159aa23ca633fbd3e0f417080fa0ceb4b7ce3c99b20f38736b1b66a22ccd
  antwortet-auf: docs/befund/befund-galileo-te-spec.md
-->

# Befund — Mode-3-(three-way-)Fenster über die ganze Mission und S1-Isolat-Replikation (st63 9883–9908)

## Auftrag

Zwei offene Posten des `befund-galileo-te-spec` (Register-Satz), beide als Messung
geschlossen bzw. neu vermessen:

1. (A) Mode-3 (three-way): Gibt es über die GANZE Mission Fenster mit genug
   zusammenhängenden Tagen (≥ 10) für eine Noise/Spec-Analyse? Falls nicht: die
   tatsächliche Mode-3-Tagesstruktur (gemessene Grenze, 0 honored).
2. (B) Die einzige era-bedingt signifikante Vorwärts-Zelle der Spec-TE-Batterie
   (st63, Tage 9883–9908, mode-1-kontrolliert, median|r|-Metrik, lag 3,
   TE 1.43e-1, cTE 1.35e-1 vs cThr 1.28e-1, Marge 0.007): Replikation über Seeds
   und verschobene Fenster.

## Daten & Messschritt

Quelle `data/galileo_resid.bin` (GASR, 14 077 825 Records, TDB-Tage 7637..9920;
[0]=tdb [1]=resid [2]=station [3]=ground_mode [4]=data_type [5]=doppler_ref/10
[6]=sampler_s [7]=signal_strength). Reinigung unverändert (Konvention
`galileo_te_floor_direction`/`galileo_spec_te`): resid endlich, |resid| ≤ 1000 Hz,
strength ≠ 0. Gemessen: cleaned 12 076 707, lock 1 994 510, zero-strength 6 608,
non-finite 0 — exakt die Spec-TE-Zahlen. Mode-Totals cleaned:
m1 8 171 124, m2 2 951 553, m3 954 030 (7,90 % der cleaned).

Tages-Konvention unverändert (Spec-TE): qualifying Tag = ein (Tag, Station,
Mode)-Bin hat ≥ 30 cleaned Samples; der Tag wird über den dominanten Mode-Bin
geführt (dom); Noise/ref aus den Samples genau dieses Bins. Neu additiv gemessen:
„Mode-3-Sample-Tag" = Tag mit ≥ 30 drei-way-Samples (unabhängig vom dom) — die
permissive Grenze für eine drei-way-Sample-only-Serie.

Sonde: `tools/measure/src/bin/galileo_mode3_s1_repl.rs` (additiv, `cargo check`
0 Warnungen). Validierung: die Isolat-Zelle wird bit-identisch reproduziert
(siehe (B)) — die Messkette stimmt mit der Original-Sonde überein.

## (A) Mode-3-Struktur der ganzen Mission

Per Station: qualDays | dom1/dom2/dom3-Tage | m3dom = drei-way-dominante Tage
(Segmente) | längstes m3dom-Fenster | m3Sample = Tage mit ≥ 30 drei-way-Samples
(Segmente) | längstes m3Sample-Fenster:

| st | qualDays | dom 1/2/3 | m3dom Tage | längstes m3dom | m3Sample Tage | längstes m3Sample |
|---|---|---|---|---|---|---|
| 12 | 8 | 0/7/1 | 1 (1 Segm., 7643) | 1 d | 6 (1 Segm.) | 6 d (7643–7648) |
| 14 | 141 | 92/45/4 | 4 (4×len1) | 1 d | 55 (29 Segm.) | 8 d (9459–9466) |
| 15 | 1 | 0/0/1 | 1 (7643) | 1 d | 1 | 1 d |
| 24 | 1 | 1/0/0 | 0 | – | 0 | – |
| 34 | 1 | 1/0/0 | 0 | – | 0 | – |
| 42 | 5 | 0/4/1 | 1 (7643) | 1 d | 4 (2 Segm.) | 3 d (7646–7648) |
| 43 | 147 | 102/40/5 | 5 (5×len1) | 1 d | 43 (19 Segm.) | 15 d (9457–9471) |
| 61 | 4 | 1/3/0 | 0 | – | 3 (2 Segm.) | 2 d |
| 63 | 149 | 98/46/5 | 5 (5×len1) | 1 d | 25 (13 Segm.) | 5 d (9457–9461) |

Mission: **17 mode-3-dominante Tage insgesamt, in 17 Segmenten der Länge 1** —
nirgends in der Mission zwei aufeinanderfolgende drei-way-dominante Tage an
derselben Station (alle Segment-Längen-Verteilungen zeigen nur len1). Das längste
mode-3-dominante Fenster der ganzen Mission ist **1 Tag**. → Es gibt **kein**
Mode-3-Fenster mit ≥ 10 zusammenhängenden Tagen im dominanten-Mode-Sinn.

Permissiv (Mode-3-Sample-Tage): 137 Stations-Tage mit ≥ 30 drei-way-Samples; das
einzige Fenster ≥ 10 Tage ist **st43 9457–9471 (15 Tage)** — dort sind die Tage
nach dom 1/2 geführt (drei-way ist die Minderheits-Samples des Tages), es ist kein
drei-way-dominiertes Fenster. Zweitlängstes: 8 Tage (st14 9459–9466).

**Verdikt (A): Mode-3 ist eine gemessene Daten-Grenze, kein Urteils-Vorbehalt.**
Über 2283 Missions-Tage gibt es keinen Lauf von zwei aufeinanderfolgenden
drei-way-dominanten Tagen; 17 dominante drei-way-Tage, alle isoliert. Im
dominanten-Mode-Sinn trägt kein Fenster eine Noise/Spec-Analyse. Einziges
≥-10-Tage-Fenster irgendeiner Art ist st43 9457–9471 (15 Tage, drei-way als
Minderheitsmode) — messbar als drei-way-Sample-only-Serie, hier nicht ausgeführt
(registriert, siehe Register).

## (B) S1-Isolat-Replikation st63 9883–9908

Publizierte Zelle: S1 ref→med|era, mode-1-kontrolliert, lag 3, n = 21.
Reproduziert exakt (alle Lags 1,2,3,5 identisch zur Original-Sonde):
lag 3 D->T TE 1.4280e-1 | thrPh 3.3162e-1 | thrBl 1.3356e-1 | cTE 1.3500e-1 |
cThr(seed0, 20 Surr) 1.2829e-1.

Fensterstruktur (gemessen): 9878 und 9882 sind für st63 keine qualifying Tage
(der 9858–9877-Lauf endet bei 9877; 9879–9881 sind drei vereinzelte Tage),
9909–9910 nicht (9911–9914 sind ein dom-2-Block). Die verschobenen Fenster
realisieren sich daher als Kappungen des zusammenhängenden Blocks [9883, 9908]:
shift-left → [9883, 9903] (n mode1 = 18), shift-right → [9888, 9908] (n = 17).

Seed-Replikation: 10 Seeds, je eine frische 20-Surrogat-Null (mean+2σ, exakt die
publizierte Definition) + gepoolte Null aus 600 Surrogaten (seed0):

| Fenster (mode1 n) | cTE lag3 | Cross 20-Surr, 10 Seeds | gepoolte Null N600 (mean / sd / thr) | z | cTE − thr |
|---|---|---|---|---|---|
| [9858,9877] (15) | 7.779e-2 | 0/10 | 6.368e-2 / 4.993e-2 / 1.635e-1 | 0.28 | −8.574e-2 |
| [9883,9903] (18) | 1.2883e-1 | 1/10 | 8.161e-2 / 3.741e-2 / 1.564e-1 | 1.26 | −2.760e-2 |
| [9883,9908] (21) | 1.3500e-1 | 6/10 | 7.916e-2 / 2.976e-2 / 1.387e-1 | 1.88 | −3.682e-3 |
| [9888,9908] (17) | 1.5417e-1 | 2/10 | 9.057e-2 / 3.350e-2 / 1.576e-1 | 1.90 | −3.401e-3 |

Exakte Zelle: cTE übersteigt die 20-Surrogat-Null nur in **6 von 10 Seeds** (der
publizierte Seed gehört zu den 6) und bleibt gegen die gut bestimmte
600-Surrogat-Null **unter** mean+2σ (z 1.88, Marge −3.7e-3). Beide Kappungsfenster
replizieren nicht (1/10 und 2/10 Seeds; z 1.26 und 1.90; Marge negativ). Das
Vorlauf-Fenster ist null (0/10, z 0.28). Die rms-Metrik derselben Zelle liegt weit
unter ihrer Null (cTE 2.39e-2, cThr0 1.92e-1). Die alternative Leseart
strength→med|era an derselben Zelle ist null (0/10, z 0.62).

Nebenbefund (Kontext, kein Urteil): strength→med|era am st63-Vorlauf-Fenster
9858–9877 (n 15) kreuzt in 10/10 Seeds (z 2.22, Marge +1.44e-2) — eine einzelne
Zelle in einer Batterie, im Mehrfachtest-Erwartungsbereich, nicht registriert.

**Verdikt (B): Die S1-Isolat-Zelle repliziert nicht.** Sie sitzt auf der
Null-Grenze: im exakten Fenster hängt das Urteil am Seed (6/10 bei der
20-Surrogat-Null; der publizierte Seed allein entschied die Marge +0.007), gegen
die 600-Surrogat-Null bleibt sie unter mean+2σ (z 1.88, negative Marge), und in
beiden verschobenen Fenstern verschwindet sie (1/10 bzw. 2/10). Der publizierte
Befund — eine isolierte, nicht replizierende Zelle — wird bestätigt: der Treffer
liegt im Zufalls-/Mehrfachtest-Rahmen, kein echter era-bedingter ref→noise-Pfad
messbar.

## Register

- Spec-Träger → resid: bleibt **entkoppelt/era-koinzident (null)**. Der
  Replikationsauftrag der S1-Isolat-Zelle ist erfüllt: die Zelle repliziert über
  Seeds/Fenster nicht → als Zufalls-Treffer geschlossen.
- Mode-3-Fenster: als Daten-Grenze gemessen — 0 drei-way-dominante Fenster ≥ 10
  Tage in der ganzen Mission. Einziger ≥-10-Tage-Lauf irgendeiner Art: st43
  9457–9471 (15 Tage, drei-way = Minderheitsmode). `pending` (registriert): eine
  ref→noise-Analyse auf der drei-way-Sample-only-Serie st43 9457–9471 (nicht
  mode-3-dominiert, daher kein Urteil über drei-way-Dominanz; 0 honored).

## Status

`done` (Rat gehalten, 2026-09-05). Mode-3 ist eine gemessene Daten-Dünn-Grenze
(0 Fenster ≥10 durchgängige Tage, 17 isolierte Einzeltage); die S1-Isolat-Zelle
repliziert nicht (6/10 Seeds, unter der 600-Surrogat-Null, verschwindet über
verschobene Fenster). Probe `tools/measure/src/bin/galileo_mode3_s1_repl.rs`,
`cargo check` 0 Warnungen.
