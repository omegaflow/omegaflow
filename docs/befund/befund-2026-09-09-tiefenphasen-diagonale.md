<!--
  title: Befund — Tiefenphasen-Diagonale (pP/sP-Lag als Tiefenmaß): der Lag liest die Quelltiefe direkt (~2 h/vp, 1 s Pick-Streu = 3,2 km) — die scharfe Tiefe wird aus den Laufzeiten selbst, nicht aus dem flachen P-Gitter
  class: befund
  date: 2026-09-09
  sha256: ea670c4400ff7c9857baeb0634b189615994cffde3202f7e03217a344fe7a108
  status: done
  see-also: docs/handover/handover-thematisch-tiefenphasen-flotte.md docs/befund/befund-2026-09-09-ak135-tiefenphasen.md docs/befund/befund-2026-09-09-seismische-ortung-tiefe.md
-->

# Befund: die Tiefenphasen-Diagonale

## Frage & Bindung

TODO-Registerzeile „Scharfe Tiefe": das P-Gitter war flach bestimmt (0–50 km
ununterscheidbar, Katalog 10 km). Die pP/sP-Laufzeitkurven stehen (Atom
`ak135-tiefenphasen`); dieses Atom macht sie zum Tiefenmaß. Probe:
`tools/measure/src/bin/depth_phase_probe.rs`.

## Die Diagonale

Der pP-P-Lag und der sP-P-Lag sind reine Funktionen der Quelltiefe (der
Ursprungszeit-Term kürzt sich — eine Differenzmessung). Gemessen (ak135, Quelle
us6000tkt2):

| Δ | h=10 km | h=20 km | h=35 km | h=50 km | h=100 km |
|---|---|---|---|---|---|
| 30° | 3,0 / 4,3 s | 6,1 / 8,6 | 10,1 / 14,3 | 12,9 / 18,9 | 22,4 / 34,0 |
| 90° | 3,4 / 4,5 | 6,6 / 9,0 | 11,0 / 15,0 | 14,6 / 20,1 | 26,3 / 36,9 |

(je Zelle pP-P / sP-P). Der Lag wächst ~2 h/vp: **1,5 s je 5 km**, also trägt
**1 s Pick-Streu ≈ 3,2 km** — das flache P-Gitter konnte 0–50 km nicht trennen,
der Lag trennt auf ~3 km.

## Der Nachweis

- Synthetische Positivkontrolle: pP-P-Lags der Katalogtiefe (10/20/35 km)
  invertieren exakt auf die Tiefe zurück (1D-Minimierung über die Lag-Diagonale);
  mit +0,5 s Bias bleibt die Tiefe im Band.
- `p_p_lag(Δ, 0) == 0` exakt (die Reflexion trägt bei h=0 keine Zeit);
  sP-P > pP-P; der Lag wächst über das flache Band monoton.

## Benannt

- Am Moho koppelt die Strahlgeometrie den Ray-Parameter: bei kleinem Δ (20°)
  kehrt der Lag schwach um (35→50 km: 10,43→10,17 s) — die 35–50-km-Stufe liegt
  im schnellen Mantel. Ein gemessener Umkehrpunkt, keine Tabelle; die Inversion
  trägt ihn (Minimierung über die volle Diagonale).
- Für dieses Beben (10 km) liegt der pP-P-Lag (~3 s) an der P-Pick-Streu
  (σ≈2 s): das echte Picken ist grenzwertig. Für tiefere Ereignisse (≥20 km,
  Lag ≥6 s) löst die Phase die Tiefe sauber — das ist die Feldlage (das Feld
  pickt pP/sP bei Tiefen-Ereignissen, nicht bei Flachquellen).

## Folge (Register)

- „Scharfe Tiefe" rückt auf „Tiefenphase als Tiefenmaß gebaut, synthetisch
  bestanden; echtes pP/sP-Picken an einem tieferen Ereignis ist der Nachfolger".
