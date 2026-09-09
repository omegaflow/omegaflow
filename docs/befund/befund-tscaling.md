<!--
  title: Befund — T-Skalierung am Betriebspunkt (T=150/300/600): Power steigt mit T, FPR bleibt bei 4–5 % (Fig. S8 bestätigt)
  class: befund
  date: 2026-09-09
  sha256: c836a409c10b93ac8a7377a596eaa86cf18567a87c07826ea2ba50ad993ea03f
  status: done
  see-also: docs/handover/archiv/handover-2026-09-09-te-galileo-nsurr-folge.md docs/befund/befund-betriebspunkt-sweep.md docs/befund/befund-klassen-benchmark-pcmci.md
-->
# Befund — T-Skalierung am Betriebspunkt (T=150/300/600)

## Frage & Bindung

Frage (die Duty aus dem Handover `handover-2026-09-09-te-galileo-nsurr-folge.md`):
Trägt die publizierte T-Skalierung (Fig. S8, qualitativ: „Power steigt mit T")
am Betriebspunkt der Maschine — und steht das T=600-Blatt aus Run 34387500520
(FPR 4,88 %, Power med 0,40), das auf seine T=150/300-Gegenstücke wartete?

Bindung (A = A): Betriebspunkt wie Atom 4 festgelegt — Block/KSG, max_lag 2
(τ), null_lag 12, bins 4, block n^(1/3) (auto: 5/7/8 bei T=150/300/600),
n_surr 100, knn 4, p_max 2, alpha 0,05; Seed `0x9E3779B97F4A7C15`. Das Trio
läuft an N=10, c=0,2, a-set1 (der Fig.-S8-Punkt), R=2 Topologien × S=10
Realisierungen je T. Der Befund spricht nur über das gemessene Sheet.

## Das Sheet

CI `te-tscaling.yml` (Run 34401566532, done), ein Punkt: das T-Trio in einem
Lauf (`--tscale`), Commit-SHA `85898ec`. Spalten je T: Power min/med/max über
die realisierten Links, `links>70%`, FPR über neg=3400 (20 Realisierungen × 170
Falschkanten-Slots).

| T | Power min/med/max | links>70% | FPR |
|---|---|---|---|
| 150 | 0,00 / 0,20 / 0,90 | 2/20 | 4,53 % |
| 300 | 0,00 / 0,30 / 1,00 | 5/20 | 4,18 % |
| 600 | 0,10 / 0,40 / 1,00 | 6/20 | 4,88 % |

## Der Verdikt-Satz

Die T-Skalierung trägt am Betriebspunkt: Power wächst monoton mit T (med
0,20 → 0,30 → 0,40; links>70 % 2 → 5 → 6 von 20), während die FPR in einer
engen Fläche von 4,18–4,88 % bleibt — unter der nominalen α=0,05-Schranke,
ohne monotone Tendenz mit T (T=600 trägt mit 4,88 % den höchsten Wert, kein
Trend). Das T=600-Blatt aus Run 34387500520 stimmt in den zwei gemeldeten
Zahlen überein (FPR 4,88 %, Power med 0,40 — kein Byte-Beleg der Vorlage
gezogen). Der Fig.-S8-Befund „Power steigt mit T" gilt am Betriebspunkt.

Zum früheren Betriebspunkt (Residual/Binned/n_surr 10, `befund-klassen-
benchmark-pcmci.md` Zeile „T=150/300/600, c=0.2": med 0,10→0,30, max bis 0,80,
FPR 6,7–7,7 %): dieselbe Achse trägt jetzt med 0,20→0,40, max bis 1,00 bei FPR
4,18–4,88 %. Jede FPR ist die Kalibrierung ihrer eigenen Null — die neue Null
(Block, α=0,05) sitzt bei 4,18–4,88 %, die alte (Residual/Binned/n_surr 10) bei
6,7–7,7 %; die zwei Zahlen sind nicht als eine Achse zu lesen. Der
Betriebspunkt selbst ist Atom 4 (`befund-betriebspunkt-sweep.md`).

## Register-Satz

Register: **T-Skalierung am Betriebspunkt — Power steigt mit T, FPR bleibt bei
4–5 %.** T=150/300/600 (N=10, c=0,2) tragen Power med 0,20/0,30/0,40 und FPR
4,53/4,18/4,88 %; T=600 stimmt in den zwei gemeldeten Zahlen mit Run 34387500520
überein. Die Duty aus dem Handover ist beantwortet.

## Status

`done`. Messung: CI `te-tscaling.yml` (Run 34401566532), Sheet
`report-tscaling.txt` (SHA 85898ec).
