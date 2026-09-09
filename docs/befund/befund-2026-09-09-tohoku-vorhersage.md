<!--
  title: Befund — Tōhoku-Vorhersage (GEBCO √(g·d)): die Flachwasser-Rechnung trifft die offenen Ozean-Wege auf ~1–18 min; zwei Wege (Adak, Hilo) über Schelf-Features sind über-langsam — die echte Welle beugt (schnellster Weg), das Großkreis-Routing nicht
  class: befund
  date: 2026-09-09
  sha256: 08c4957e1cce28070ef8e7b021a0fc47a71bda4e12dd8d37dccb060bdc3e8a3d
  status: done
  see-also: docs/handover/handover-2026-09-09-tiefenphasen-flotte.md docs/befund/befund-2026-09-09-tohoku-pegsel.md
-->

# Befund: die Tōhoku-Vorhersage (GEBCO-Flachwasser-Rechnung)

Feld-Herkunft: Σ Segment/√(g·d) über die Tiefenkarte ist die offizielle
Laufzeitkarten-Methode der Warnzentren (Murty 1977; TTT-Software, Wessel;
Huygens-Prinzip) — der Großkreis ist die Streckenvariante davon, der Nachfolger
ist Eikonal über das volle Gitter.

## Frage & Bindung

Der Vorhersage-Schenkel der Kette: aus dem Katalog-Ort (38,297 N / 142,373 E)
und der Tiefenkarte die Ankunftszeit an jeden Pegel rechnen, gegen die gemessene
Staffelung (`docs/befund/befund-2026-09-09-tohoku-pegsel.md`) legen. Probe:
`tools/measure/src/bin/tohoku_tsunami_vorhersage_probe.rs` — Großkreis-Slerp,
GEBCO2020-Tiefe an 25 Punkten je Weg (OpenTopoData-Punktabfrage), Laufzeit =
Σ Segment / √(g·d), Tiefen-Boden 200 m (Schelf-Konvention).

## Die Rechnung gegen die Messung

| Pegel | Dist [km] | vorhergesagt | gemessen | diff [min] |
|---|---|---|---|---|
| Crescent City | 7542 | 15:49 | 15:48 | **+1,4** |
| Honolulu | 5961 | 13:35 | 13:30 | **+4,9** |
| Kwajalein | 4153 | 10:55 | 11:06 | −11,1 |
| Midway Island | 3874 | 10:54 | 10:36 | +18,1 |
| Adak Island | 3495 | 11:33 | 10:36 | +57,2 |
| Hilo | 6301 | 15:16 | 13:54 | +81,8 |

## Das Verdikt — die Physik trägt, das Routing nicht überall

Vier von sechs Wegen treffen die gemessene Ankunft auf **1–18 min** — die
offenen Ozean-Wege. Das ist die Bestätigung der Flachwasser-Geschwindigkeit
√(g·d) als Vorhersage-Instrument: die Warnzeit (der offene Ozean) stimmt.

Zwei Wege sind über-langsam: **Adak** (+57 min) und **Hilo** (+82 min). Beide
Großkreise streifen flache Features — Adak die Aleuten-Schwelle (Tiefen-Minimum
20 m bei 51,4 N / 179,2 E), Hilo die Hawaii-Plattform (Minimum 2 m an der
Küste). Die per-Segment-√(g·d)-Rechnung verlangsamt die Welle dort, wo sie in
Wahrheit **um das Flache herumbeugt** (schnellster Weg, Fermat). Das
Großkreis-Routing kennt keine Beugung.

## Benannt

- Tiefen-Boden 200 m = „Ankunft an der 200-m-Isobathe" (Schelf-Konvention).
- Die Punkt-Abfrage (OpenTopoData GEBCO2020) ist kein volles Gitter; der
  schnellste-Weg (Eikonal/Huygens über das Gitter) ist der Nachfolger.

## Folge (Register)

- Vorhersage-Schenkel gebaut; Positivkontrolle teilweise bestanden (4/6 offene
  Wege auf Minuten, 2 Wege an Schelf-Beugung über-langsam — benannt, nicht
  gedeutet).
- Offen: schnellster-Weg über das volle GEBCO-Gitter (Referenz-Kernel, pending);
  M9.1-Picker; die volle Kette mit eigener Ortung statt Katalog-Ort.
