<!--
  title: Befund — die Zonen-Flotte (Tonga/Fiji, 410–660 km): der Flotten-Mittelwert bleibt unverzerrt (−0,8 km), aber die Streuung (70 km über Ereignisse, 108 km über Stationen) ist das 2–3-fache der Hindu-Kush-Tiefe; die Inversion klemmt an der 660-Kante (best-at-MAX, gemessen)
  class: befund
  date: 2026-09-09
  sha256: 345d7b64d5f4adc697eeb05132d853227c7039c6c0b87c398a6990a9e665c7a1
  status: done
  see-also: docs/handover/archiv/handover-2026-09-09-tiefenphasen-tiefherd.md docs/befund/befund-2026-09-09-ak135-tiefherd-erweiterung.md docs/befund/befund-2026-09-09-tiefenphasen-flotte.md docs/befund/befund-2026-09-09-tiefenphasen-polaritaet.md
-->
# Befund: die Zonen-Flotte (Tonga/Fiji, 410–660 km)

## Frage & Bindung

Registerzeile des thematischen Handovers „Tiefenphasen-Flotte": die Zonen-Flotte (Tonga, 410–660 km) war der benannte Nachfolger; ihre benannte Voraussetzung — die ak135-Tiefherd-Erweiterung über 250 km — ist geschlossen (`befund-2026-09-09-ak135-tiefherd-erweiterung.md`). Das Tor stand; die Flotte stand ungemessen.

## Was gebaut wurde

- `tools/measure/src/bin/depth_phase_fleet_probe.rs`: die Selektion ist parametrisiert — `--region lat0,lat1,lon0,lon1` und `--mindepth`; die Defaults bleiben die Hindu-Kush-Werte, der geschlossene Befund bleibt reproduzierbar.
- `tools/measure/src/bin/ephemeris_structure_probe.rs`: der vorgefundene Bruch (positional nach named im `println!`-Format, committed unter `baab781`) ist gehoben — Variablen benannt statt `#[allow]`. `cargo check -p omegaflow-measure` steht grün (null Fehler, null Warnungen).

## Die Messung (Selektion vor dem ersten Fetch registriert)

Box lat −26…−15, lon −179…−175 (Tonga-Fiji-Slab), mindepth 250 km, M ≥ 6, orderby magnitude, GBCO-Zeuge je Ereignis, Stationsband 30–90°, SNR-Gate ≥ 3. 64 registrierte Tiefereignisse in der Box; die 16 größten gemessen (M6.5–8.2, Katalog-Tiefe 271,0–622,6 km).

Per-Ereignis-Offsets (Median − Katalog) in km:

`[−76, −95, +72, −34, −85, +19, +45, +54, +64, +101, −88, +46, +16, −108, +69, −13]`

| Größe | Zonen-Flotte | Hindu-Kush-Flotte |
|---|---|---|
| N (Ereignisse mit invertierter Tiefe) | 16 | 16 |
| Mittelwert Offset | −0,8 km | +1,7 km |
| σ über die Ereignisse | 70,5 km | 19,0 km |
| se = σ/√N | 17,6 km | 4,7 km |
| typische Stations-Streuung (Median je Ereignis-σ) | 108,1 km | 36,1 km |

## Die Befunde (benannt, nicht geglättet)

1. **Der Flotten-Mittelwert bleibt unverzerrt.** −0,8 km gegen den Katalog; die Zonen-Tiefe (410–660 km) trägt keinen systematischen Bias — der Offset ist Streuung, keine Verschiebung. Dasselbe Bild wie die Hindu-Kush-Flotte (+1,7 km).

2. **Die Streuung wächst mit der Tiefe um das 2–3-fache.** 70,5 km über die Ereignisse, 108,1 km über die Stationen — gegen 19/36 km der Hindu-Kush-Flotte. Die pP/sP-Korrelation verliert an der tiefen Geometrie ihre Eindeutigkeit: die Tiefen eines Ereignisses streuen über hunderte km (z. B. us10006scr: 602–660 km).

3. **Die Inversion klemmt an der 660-Kante (best-at-MAX, gemessen).** Ereignisse bei Katalog 591–622 km lesen wiederholt exakt 660 km: usb000ruzk (615,4 km) — 5 von 7 Stationen auf 660; us10006scr (596,4 km) — 8 von 11; us70005axg (591,0 km) — 5 von 8. Der best-at-MAX der Inversion (in der Tiefherd-Erweiterung benannt: „ein >700-km-Ereignis läse 700") tritt an der 660-Kante als Sättigungspunkt auf. Der pP-Lag bei ~600 km fällt auf den tiefen Ast der Laufzeitkurve; die Inversion setzt ihn an die Kante.

4. **Der take-off-Boden trägt in der Zone.** Stationen nahe der Untergrenze lesen pP absent („pP below the correlation gate"): das tiefe pP kehrt unter steilem Einfall nicht um — der in der Tiefherd-Erweiterung gemessene Boden zeigt sich in der Zone als skip, nie als fabrizierte Zeit.

5. **Nebenbefund (Robustheit):** die Zonen-Läufe tragen deutlich mehr „no decodable record"-Skips (AU/S1/GE/PS-Stationen) als die Hindu-Kush-Läufe — der Datenweg in der Süd-Pazifik-Zone liefert mehr Lücken, keine fabrizierte Tiefe (0 geehrt).

## Benannt (Pendings, nicht still)

- **Streuung an tiefer Geometrie** — der Hebel ist die pP/sP-Korrelation (und der mehrdeutige pP-Zweig bei Δ≈30°), nicht √N; das Instrument bleibt aus der Hindu-Kush-Flotte benannt.
- **Die 660-Kante** — best-at-MAX sättigt; eine Rand-Behandlung (Tiefen-Gate gegen die Kante) ist die Folge-Duty.
- **Quell-Strahlungsterm (CMT)** — unverändert `pending` ohne CMT-Lösung.
- **Externe Tiefen-Referenz (TauP/KEB95)** — unverändert `pending`, Instrument benannt.

## Verdikt

Die Zonen-Flotte ist gemessen: der Mittelwert ist unverzerrt (−0,8 km), die tiefe Zone streut 2–3× stärker als die mittlere Tiefe, und die Inversion klemmt an der 660-Kante — beide benannt, nicht geglättet. Die Zonen-Flotte ist damit keine offene Voraussetzung mehr, sondern eine Messreihe mit benannten Engpässen.
