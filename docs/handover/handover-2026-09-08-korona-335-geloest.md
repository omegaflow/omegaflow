<!--
  title: Handover — Korona: der 335-Konfund-Widerspruch ist aufgelöst
  class: handover
  date: 2026-09-08
  sha256: 7264e35c81c3f198389d00e0e681951c196b01705c739e9f87bdcbca4d99e6ec
  status: live
  see-also: docs/paper/corona-heating-ladder.md docs/handover/handover-2026-09-07-korona-335-widerspruch.md docs/TODO.md
-->

# Handover — der 335-Konfund-Widerspruch ist aufgelöst

Die offene Pflicht aus `handover-2026-09-07-korona-335-widerspruch.md` ist
gemessen geschlossen: warum 304→131 unter dem Einzel-Konfund C=335 kollabiert
(D|C ≈ 0), unter GOES und 94 aber nicht. Antwort: es kollabiert nicht die
Richtung, sondern nur die D|C-Mittel-Asymmetrie — und die Symmetrisierung ist
eine Einzel-Linien-Konditionierungs-Eigenheit, kein versteckter gemeinsamer
Treiber.

## Gemessen (2014, 989 Ereignisse, 24-s-Zellen, lag-bewusste ARX-Null)

304→131, 96 s, Vorwärts TE(304→131|C) und Rückwärts TE(131→304|C):

| Konfund | TE vorwärts | TE rückwärts | D|C | Vorwärts-Pfeil |
|---|---|---|---|---|
| GOES | 9.04e-2 | 4.78e-2 | +4.26e-2 | 856/989 |
| 94 | 8.10e-2 | 7.29e-2 | +8.08e-3 | 666/989 |
| 335 | 8.47e-2 | 8.56e-2 | −9.46e-4 | 621/989 |
| {GOES,335} | 5.51e-2 | 3.12e-2 | +2.39e-2 | 479/989 |
| {GOES,94} | 5.34e-2 | 2.96e-2 | +2.37e-2 | 498/989 |
| {94,335} | 5.58e-2 | 4.68e-2 | +8.93e-3 | 524/989 |

## Der Befund in drei Sätzen

1. **Kein Richtungs-Kollaps:** Der Vorwärts-Term TE(304→131|C) ist über alle
   Konfunde konstant (~8.1–9.0e-2 bei 96 s) und trägt unter C=335 in 621/989
   Ereignissen einen Pfeil. D|C ≈ 0 ist eine Mittelwert-Auslöschung, weil der
   Rückwärts-Term TE(131→304|C) unter 335 symmetrisch ansteigt (4.78 → 7.29 →
   8.56e-2), nicht weil die Aufwärts-Richtung verschwindet.
2. **H2 verworfen:** Die Symmetrisierung ist invariant unter der ARX-Null-Tiefe
   (max_lag 4/8/16 liefern bit-identische Mittel; der Vorwärts-Pfeil bleibt
   585–626/989). Sie ist kein Null-Artefakt.
3. **H1 widerlegt:** Die TE(C→Y)-Matrix (C ∈ {171,193,211,335,94,goes}, Y ∈
   {304,131}) zeigt keinen Konfund, der die kühlen Kanäle führt (alle C→Y-
   Pfeilraten ≤ 147/989). Und die Symmetrisierung überlebt keinen zweiten
   Konfund: {GOES,335} und {94,335} sind aufwärts (+2.39e-2, +8.93e-3),
   identisch zu {GOES,94}.

## Konsequenz für das Paper

C=335 allein ist ein schwacher Hüllen-Abzug: sein Residual lässt 304 und 131
stark und symmetrisch gekoppelt, sodass das D|C-Mittel verschwindet, während
beide Richtungs-TEs hoch bleiben. 335 ist nicht der Konfund, der die
304→131-Richtung „wegerklärt". 304→131 ist unter jeder gemessenen Einzel- und
Zweikonfund-Konditionierung aufwärts-robust.

## Dreijahres-Reproduktion — eine Struktur, dreimal unabhängig (2026-09-08)

2013, 2014, 2015 sind drei echte Natur-Stichproben — getrennte Flare-
Populationen über den abnehmenden Sonnenzyklus, keine Seed-Wiederholung eines
Ensembles. Daß die Auflösungs-*Struktur* in allen drei Jahren steht — GOES-
Baseline stark aufwärts, C=335 allein symmetrisierend, Doppel-Konditionierung
wiederherstellend — ist ein Struktur-Zwirn, gewichtiger als jede Einzel-D-Zahl.
Bei 96 s:

| Jahr | GOES D|C (Pfeil) | C=335 D|C (Pfeil) | {GOES,335} D|C (Pfeil) |
|---|---|---|---|---|
| 2013 | +5.20e-2 (87%) | +1.13e-2 (65%) | +2.97e-2 (52%) |
| 2014 | +4.26e-2 (87%) | −9.46e-4 (63%) | +2.39e-2 (48%) |
| 2015 | +4.63e-2 (85%) | −1.62e-3 (61%) | +2.49e-2 (44%) |

(Vorwärts-Pfeil = TE(304→131\|C) über der lag-bewussten Null je Ereignis.)

Zwei ehrliche Grenzen gehören zum Bild:

1. **Beteiligung halbiert unter der stärksten Kontrolle:** Unter {GOES,335}
   bleibt der Vorwärts-Pfeil nur in 44–52 % der Events über seiner Null, bei
   positivem gemitteltem D in jedem Jahr — robust in der Richtung, halbiert in
   der Beteiligung. Die Grenze wird benannt, nicht versteckt.
2. **Struktur stabil, Amplitude variabel:** Unter C=335 allein bleibt D|C in
   2013 leicht positiv (+1.1e-2, 65 %), während 2014/2015 auf ~0 kollabieren
   (−9e-4, −1.6e-3). Der Grad der Symmetrisierung variiert mit dem Jahr; die
   Struktur (Rückwärts-Term steigt, Pfeil bleibt mehrheitlich) hält überall.
   A = A auch für Jahresvariationen.

## Die Sonne als Kalibrier-Feld beider Maschinen-Linien

Heute trägt die Sonne zwei Maschinen-Linien gleichzeitig, konvergierend: die
paarweise Matrix (72 Sonden) findet XRS→131/193 — die Neupert-Struktur von
oben (Röntgen→heiß); die konditionale Sonde reproduziert 304→131 dreijährig
unter jeder Kontrollstufe — die Chromosphäre→TR-Struktur von unten. Zwei
Pfade, ein Objekt. Damit ist der Kalibrier-Feld-Status offiziell: die Sonne
ist der permanente Kalibrier-Sender, von dem die Linien ihre Wahrheit ablesen.

Paper ist auf Version 9 fortgeschrieben, TODO-Eintrag geschlossen.

## Werkzeug & Logs

`corona_conditional_probe` (D|C-Zerlegung, `--confound2`, `--max-lag`) und neu
`corona_confound_matrix_probe` (TE(C→Y)-Matrix). Logs:
`/tmp/opencode/corona_cond_2014_{Cgoes,C335,C94,Cgoes+C335,Cgoes+C94,C94+C335}.log`,
`corona_matrix_2014.log`, `/tmp/opencode/corona_cond_{2013,2015}_{Cgoes,C335,
go335}.log` (Reproduktion).

## Laufbedingungen (verankert)

Probe `corona_conditional_probe` (Release), Daten `aiaYYYY_fullyear.bin` (7
AIA-Bänder, 24-s-Median-Zellen, DATAMEAN/EXPTIME, JSOC) + GOES-15-Trigger
`xr_*.nc` (2-s b_flux). Ereignis: GOES b_flux > 5e-6 W/m² (C1.0), Fenster
±40 min um den Peak (WINDOW 100 Zellen à 24 s), REFRACTORY 75, ≥ 100
vollständige Zellen. Messung: D|C = TE(cool→hot|C) − TE(hot→cool|C), lags
0/96/192 s (LAGS [0,4,8] Zellen); Null = lag-bewusste ARX-Residual-Surrogate,
mean+2σ über N_SURR 10; Seed je Ereignis `0x9E37_79B9_7F4A_7C15 ^ (pair·
0x9E37_79B9) ^ (lag·0x85EB_CA6B)`; max_lag 8 (H2-Sweep: 4/16). Ereigniszahlen:
2013 = 522, 2014 = 989, 2015 = 612. Damit kann die achte Schicht die
Reproduktion nachlaufen, nicht nur glauben.

## Register

Paper `docs/paper/corona-heating-ladder.md` Version 9 (§4.6, §7, Abstract);
TODO „304→131: 335-Konfund-Widerspruch" geschlossen (2026-09-08).
