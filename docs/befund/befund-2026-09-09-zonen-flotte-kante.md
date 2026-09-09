<!--
  title: Befund — die 660-Kante benannt: die Inversion klemmt als Sättigung an der Wand; die Tiefe der tiefen Zone ist an der Kante unaufgelöst (Tiefen-Gate gemessen)
  class: befund
  date: 2026-09-09
  sha256: df80ae4d49fb1073c552b182a61dc034ef2bda0b98cb928d1ca82f2059c1fff5
  status: done
  see-also: docs/handover/archiv/handover-2026-09-09-zonen-flotte.md docs/befund/befund-2026-09-09-zonen-flotte.md docs/befund/befund-2026-09-09-ak135-tiefherd-erweiterung.md
-->
# Befund: die 660-Kante benannt — Tiefe unaufgelöst

## Frage & Bindung

Registerzeile des thematischen Handovers „Zonen-Flotte": „Die 660-Kante — benannt, offen. best-at-MAX sättigt; Ereignisse bei Katalog 591–622 km lesen exakt 660. Instrument: eine Rand-Behandlung (Tiefen-Gate gegen die Kante)." Der Council entschied die Form: ein Zustands-Enum, keine Band, kein Epsilon — die Wand ist der 660-Knoten des Modells, nicht ein willkürlich gezogenes Intervall.

## Was gebaut wurde

- `tools/measure/src/depthphase.rs`: die Inversion trägt jetzt einen Zustand statt eines nackten `Option<f64>` — `DepthInversion { Depth(f64), EdgeDiscontinuity, SaturatedBound, Absent }`. `inversion_state` klassifiziert exakt: `best_h == 660.0` → `EdgeDiscontinuity` (die Wand), `best_h == 700.0` → `SaturatedBound` (die Suchdecke `INVERSION_DEPTH_MAX_KM`), kein endliches Residuum → `Absent`. Der Gitterschritt ist 1,0 km, daher exakte f64-Gleichheit — kein Epsilon. `invert_depth_single` und `invert_depth_multi` geben beide den Enum zurück; `measure_station` trägt ihn als `StationMeasure.inversion`.
- Der Wire-/Response-Kontrakt bleibt unberührt — der Zustand ist probe-lokal.
- Kalibrier-Gate (te.rs-Disziplin): `p_p_lag(45,660)` → `EdgeDiscontinuity` (nie `Depth(660)`); `p_p_lag(45,700)` → `SaturatedBound`; `p_p_lag(45,400)` → `Depth(400)` (kein Falsch-Klemmen); ein Δ jenseits des ak135-Bereichs → `Absent`. Die Tiefen-Rückgewinnungs-Tests bleiben grün (300/410/500/600 → `Depth(h)`; 660/700 jetzt als Zustand benannt statt als Tiefe behauptet).

## Die Messung (Fleet-Neulauf, 16 Ereignisse, Zonen-Box)

24 von ~148 Stationen klemmen an der 660-Kante, 2 sättigen an 700 — konzentriert auf die tiefsten Katalogereignisse (596,4 km → 7/11, 615,4 km → 4/7, 591,0 km → 4/8). Die Klemmung liest exakt 660, nie 659/661/700 — die Sättigungssignatur der Wand.

| Größe | mit Klemmung (Median-Regel) | nach Exklusion |
|---|---|---|
| Flotten-Mittelwert Offset | −0,8 km | −40,8 km |
| σ über die Ereignisse | — | 75,6 km |
| se | — | 18,9 km |

## Die Befunde (benannt, nicht geglättet)

1. **Die Exklusion ist nicht bias-neutral — gemessen, nicht angenommen.** Der Council nannte ein Einzelereignis (usb000ruzk) als Beleg, dass Exklusion die Katalogtiefe zurückgewinnt; der volle Fleet misst das Gegenteil: mit Klemmung −0,8 km, nach Exklusion −40,8 km. Der Katalog der Zone (540–623 km) drängt gegen die Wand — die 660-Lesungen waren für die tiefsten Ereignisse physisch real; entfernt man sie, lesen die verbleibenden Stationen derselben Ereignisse ~470–520 km, systematisch flach.

2. **Der vormalige „unverzerrte −0,8 km" war selbst Wandsättigung** — ein Zufall des Medians, keine aufgelöste Tiefe. Das Gate „korrigiert" die Tiefe nicht; es benennt die Wand und legt damit offen, dass die Tiefe der tiefen Zone nahe 660 jenseits der Modellauflösung liegt.

3. **0 geehrt:** das Gate verwandelt eine verborgene Sättigung in einen benannten Zustand. Ein Ereignis, dessen Stationen sämtlich klemmen, wird als abwesend geführt (nie 0) — in diesem Lauf blieb kein Ereignis ohne mindestens zwei gemessene Tiefen.

## Benannt (Pendings, nicht still)

- **Die Tiefe der tiefen Zone bleibt an der Kante unaufgelöst** — kein Code-Instrument löst die Wand; die Auflösung ist eine Modell-/Referenzfrage (TauP/KEB95, `pending`, Instrument benannt).
- **Beide Mittelwerte, nie einer allein:** die Exklusion ist ein Zensur-Operator — der mit-Klemmung-Mittelwert und der nach-Exklusion-Mittelwert werden als zwei benannte Größen gedruckt, das Wort „unverzerrt/unbiased" steht nicht mehr neben diesem Gate.

## Verdikt

Die 660-Kante ist benannt, nicht aufgelöst: **unaufgelöst**. Das Tiefen-Gate tut, was es wahrhaftig kann — es verwandelt eine verborgene Wand-Sättigung in einen benannten Zustand. Register: *„660-Kante benannt, Tiefe unaufgelöst — die Wand trägt den Median, nicht die Tiefe."*
