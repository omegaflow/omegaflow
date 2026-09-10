<!--
  title: Befund — die Planeten-Selektion wählte de721_full.bsp statt de441: die 19-MB-Kurzform, gemessen und gepinnt
  class: befund
  date: 2026-09-09
  sha256: 21f37dbc31d7ea750d3f44b8d5c50e59555f9f7e90aa60fa4ddbef7d599a08c3
  status: live
  see-also: docs/handover/archiv/handover-2026-09-09-mars-rekompilat.md docs/handover/archiv/handover-2026-09-09-mars-rekompilat-dispatch.md
-->

# Befund: die Planeten-Selektion nahm die Kurzform

## Frage & Bindung

Die Handover-Linie mars-rekompilat erwartete, der kernel-flatten-bodies-Job
regeneriere die ssd-Linie in der 183-MB-Vollform (Rat, Verdikt a: volles
de441.bsp, 32-d, 30 390 Jahre). Die Re-Verifikation (de441-mars-Anker-Δ in die
~0-km-Klasse) hing daran. Gemessen statt behauptet: der Flatten-Step des Runs
34387873054 kompilierte ~19-MB-Bins und lud sie auf das CDN.

## Befund

`select_system("planets")` (`tools/harvest/src/bin/ephemeris_compiler.rs`) gruppiert
die `spk-planets`-Indexeinträge nach Basis und nimmt die höchste `numeric_of`-Zahl.
`de721_full.bsp` (NAIF `a_old_versions`, 152 MB, Spanne CAL-ET 1599-09-04 bis
3000-03-03 ≈ 1400 Jahre) liefert `numeric_of` = 721 und schlug damit `de441` (441)
und `de442` (442). Die Flatten lief über die 1400-Jahre-Kurzform → ~19-MB-Bins
(earth 36 020 Granulen / 19 018 776 B; mercury/venus/moon/jupiter/neptune/pluto
15 985 Granulen). Der Run überschrieb damit die 183-MB-Bins auf dem CDN (gemessen:
sun 19 018 832 B, earth 19 018 776 B, mars 13 822 248 B, mercury 8 440 296 B,
moon 12 084 968 B). Der `de441-cdn-watch` (sun+earth ≥183 MB) ist nicht grün.

## Verdict (Council, alle fünf Stimmen)

Die Basis `de441` per Namen pinnen — spiegelbildlich zum `asteroids`-Zweig, der
`de441.bsp` bereits per Namen pinnt. Ein numeric-max ist eine Zahl an der Stelle der
Sache: jede künftige de730/de8xx würde wieder lautlos gewinnen. Der Pin macht den
nächsten Kernel-Wechsel zu einem sichtbaren Akt (Verdikt + Code-Edit), nicht zu
einem stillen Re-Ranking.

## Fix & Stand

`b59bbfb` pinnt die Planeten-Basis auf `de441`; `cargo check -p omegaflow-harvest`
sauber (0 Fehler, 0 Warnungen). Re-Dispatch Run `34393748385` (head b59bbfb). Die
183-MB-Form kehrt mit dem grünen Flatten zurück; die Re-Verifikation bleibt pending.

## Register-Schluss

Die 19-MB-earth-Produzenten-Frage (pending seit mars-rekompilat §4) ist gemessen
geschlossen: Produzent war die Planeten-Flatten unter `de721_full.bsp` — kein fremder
Compile, kein partieller Download.
