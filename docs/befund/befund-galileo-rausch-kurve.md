<!--
  title: Befund — Galileo-Rausch-Kurve: Mode-Split (1 vs 2/3) und SEP-Geometrie ≤5 AU
  class: befund
  date: 2026-09-05
  sha256: a886812b6a2608b44fdd2831b1ee0aa84f1f1a1766d970dd8c37cb80817909cb
  status: done
  antwortet-auf: docs/auftrag/auftrag-quiet-zone-uebertragung.md
  see-also: docs/befund/befund-galileo-gwe-bestand.md docs/auftrag/auftrag-quiet-zone-vorfilter.md docs/TODO.md
-->

# Befund: Galileo-Rausch-Kurve — Mode-Split (1 vs 2/3) und SEP-Geometrie ≤5 AU

## Kurzfassung

Die Rausch-Kurve ist auf dem vollen Bestand gezeichnet — 14 077 825
Residuum-Samples aus 138 TDF-Dateien (1990–97, CDN-Asset
`pds-ppi.igpp.ucla.edu/galileo_resid.bin`, 901 MB), gegen die
Horizons-Ephemeride `galileo_daily` getragen. Das vorab gebundene Protokoll
(n je Mode je Distanzband **zuerst**, dann Kurve; Lock-Übergänge als eigene
Klasse mit `n_lock`) hat entschieden:

- **Die Distanz-Achse ist nur am fernen Ende beprobt.** n reicht nur bei
  5–6 AU (Jupiter): Mode 1 = 152 Tage, Mode 2 = 96, Mode 3 = 77. Die
  sonnennahen Bänder (0–2 AU, Venus-/Erd-Flybys) tragen 4–8 Tage — zu dünn
  für eine statistische Distanz-Kurve. Keine Distanz-Kurve über ≤5 AU ist aus
  diesem Bestand zeichenbar (0 honored — nicht extrapoliert).
- **Die SEP-Achse trägt das Signal.** Kohärentes Doppler fällt 12–28× vom
  sonnennahen zum sonnenfernen Fenster; Einweg bleibt flach.

**Vorfilter-Verdikt (gemessen, nicht behauptet):** Galileos kohärentes
Tracking (Mode 2/3) ist plasma-dominiert — die SEP-Geometrie greift, die
Rezept-Vorbedingung (medium-getrieben) ist erfüllt. Galileos Einweg-Tracking
(Mode 1) ist flach — Oszillator-Selbst-Rauschen, keine Geometrie. Der
Mode-Split ist exakt die Achse, die das Rezept an- oder abschaltet. Galileos
eigenes „ruhiges Fenster" (relativ, ≤5 AU): **Mode 2, SEP 150–180°, 5–6 AU →
1,5 Hz Boden.**

## Messung

Datenkette: `galileo_atdf_compiler` (TRK-2-25 → GASR-Residuum-Serie) →
`galileo_noise_geo` (Residuum-RMS je Tag, je Mode, je Station, gegen
Distanz/SEP aus Horizons). Lock-Übergang = |resid| > 1000 Hz (der Residuum-
Feldbereich ist ±2²⁰ Hz; die ruhigen Werte liegen bei wenigen Hz).

- Samples: 14 077 825 · Zellen (Mode, Tag): 361.
- Mode 1 (Einweg): 163 Tage, 9 743 574 Samples (69 %), 1 568 246 Lock-Übergänge.
- Mode 2 (Zweiweg): 109 Tage, 3 110 045 Samples (22 %), 157 784.
- Mode 3 (Dreiweg): 89 Tage, 1 224 206 Samples (9 %), 268 480.
- Stationen (10): 43 (6 157 151, 147 Tage), 14 (3 905 929, 141), 63 (3 614 078,
  151) = 97 %; die 34m-Unterstationen 12/15/24/34/42/45/61 nur tageweise.

## Die Distanz-Achse — n zuerst, dann Kurve

| Mode | 0–1 AU | 1–2 AU | 4–5 AU | 5–6 AU |
|---|---|---|---|---|
| 1 | 4 Tage · 73,3 Hz | 4 · 8,2 | 3 · 12,2 | **152 Tage · 8,5 Hz** |
| 2 | 5 · 32,7 | 8 · 52,0 | — | **96 Tage · 1,6 Hz** |
| 3 | 4 · 82,7 | 8 · 117,0 | — | **77 Tage · 4,0 Hz** |

Die n-Zahl entscheidet: eine Distanz-Kurve über ≤5 AU ist **nicht** zeichenbar —
nur das ferne Ende (5–6 AU) trägt genug Tage. Die sonnennahen Bänder sind im
archivierten TDF-Bestand schwach belegt (die lauten Fenster wurden selten
getrackt, oder die Pässe fehlen). Das ist eine Daten-Grenze, keine
Reduktions-Lücke.

## Die SEP-Achse — das Signal

| Mode | SEP 0–30° (sonnennah) | SEP 150–180° (sonnenfern) | Faktor |
|---|---|---|---|
| 1 | 8,2 Hz (12 Tage) | 7,5 Hz (128 Tage) | **flach (1,1×)** |
| 2 | 42,0 Hz (18 Tage) | 1,5 Hz (73 Tage) | **28×** |
| 3 | 79,5 Hz (16 Tage) | 6,7 Hz (56 Tage) | **12×** |

Kohärentes Doppler (Mode 2/3) fällt steil mit wachsender Sonnenelongation —
die Signatur plasma-getriebenen Rauschens (R^−3,45, Woo & Armstrong 1979,
skaliert mit dem Stoßparameter der Sichtlinie). Einweg (Mode 1) bleibt flach —
kein Abstandsgang, das freilaufende Bordoszillator-Rauschen hat keine
Sonnen-Geometrie.

## Vorfilter-Verdikt

1. **Stabilisierung:** Dual-Spin, passiv — besteht (kein 3-Achsen-Selbst-Rauschen).
2. **Rauschquelle:** mode-abhängig gemessen — **Mode 2/3 = plasma-getrieben**
   (SEP-Geometrie, Rezept greift); **Mode 1 = selbst-getrieben** (flach, Rezept
   blind).
3. **Die Achse ist ≤5 AU komprimiert:** die Distanz-Kurve ist n-leer, aber die
   **SEP-Achse ersetzt sie** — Galileos „ruhiges Fenster" ist ein Winkel
   (Sonnenelongation), keine Distanz. Der Boden: Mode 2, SEP 150–180° → 1,5 Hz.

Galileo bleibt damit, was der Vorfilter versprach: Reserve mit eigener,
**relativ**-ruhiger Achse. Der Rezept-Nachbau (>50 AU) ist unmöglich (n-leer),
aber die **Mode-getrennte SEP-Kurve ist Galileos eigene Quiet-Zone** — und der
Mode-Split (1 flach, 2/3 steil) ist die sauberste Trennung von Selbst- und
Medium-Rauschen, die diese Daten erlauben.

## Register-Satz

*Galileos Rauschen hat eine Geometrie — aber nur auf der kohärenten Achse.
Einweg bleibt flach, Zweiweg fällt mit der Sonnenferne. Die stille Zone ist
bei ≤5 AU kein Ort, sondern ein Winkel.*

## Offen / `pending`

- Die sonnennahen Distanzbänder (0–2 AU) sind n-leer (4–8 Tage) — eine
  Distanz-Kurve bräuchte die fehlenden lauten Fenster-Pässe (im TDF-Bestand
  nicht vorhanden; `pending`, kein Fabricat).
- Mode 1 bei SEP 0–30° trägt nur 12 Tage — die Einweg-Flachheit ist am
  sonnennahen Ende schwach belegt (der Kontrast 2/3-steil vs 1-flach bleibt
  der Befund).
- Die 34m-Unterstationen (12/15/24/34/42/45/61) erscheinen tageweise — die
  Banden-Brücke (20-s-Stationen) ist effektiv 14/43/63.

## Status

`done`. Rausch-Kurve gezeichnet (2026-09-05), Vorfilter-Verdikt gemessen. Der
Mode-Split (1 flach vs 2/3 SEP-steil) ist der Befund, den die Route trägt;
die Distanz-Achse ist am fernen Ende n-beschränkt, die SEP-Achse trägt das
Signal.
