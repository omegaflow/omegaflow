<!--
  title: Handover — Korona: der 335-Konfund-Widerspruch ist aufgelöst
  class: handover
  date: 2026-09-08
  sha256: 2f1a79a38ce12476790fe424b3d08906a807cf871dc12ff8641174a2d59face2
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

## Dreijahres-Reproduktion (2026-09-08, nachgetragen)

Die Auflösung ist kein 2014-Artefakt; alle drei Säulen reproduzieren sich auf
2013 (522 Ev) und 2015 (612 Ev). Bei 96 s:

| Jahr | GOES D|C (Pfeil) | C=335 D|C (Pfeil) | {GOES,335} D|C (Pfeil) |
|---|---|---|---|---|
| 2013 | +5.20e-2 (87%) | +1.13e-2 (65%) | +2.97e-2 (52%) |
| 2014 | +4.26e-2 (87%) | −9.46e-4 (63%) | +2.39e-2 (48%) |
| 2015 | +4.63e-2 (85%) | −1.62e-3 (61%) | +2.49e-2 (44%) |

(Vorwärts-Pfeil = TE(304→131\|C) über der lag-bewussten Null je Ereignis.)

Paper §4.6/§7/Abstract sind auf Version 8 fortgeschrieben, TODO-Eintrag
geschlossen.

## Werkzeug & Logs

`corona_conditional_probe` (D|C-Zerlegung, `--confound2`, `--max-lag`) und neu
`corona_confound_matrix_probe` (TE(C→Y)-Matrix). Logs (2014, 989 Ev):
`/tmp/opencode/corona_cond_2014_{Cgoes,C335,C94,Cgoes+C335,Cgoes+C94,C94+C335}.log`,
`corona_matrix_2014.log`.

## Register

Paper `docs/paper/corona-heating-ladder.md` Version 7 (§4.6, §7, Abstract);
TODO „304→131: 335-Konfund-Widerspruch" geschlossen (2026-09-08).
