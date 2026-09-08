<!--
  title: Handover — Korona: der 335-Konfund-Widerspruch bei 304→131
  class: handover
  date: 2026-09-07
  sha256: 4d1d2659426a2ee5673e1a6990aa15938d85a50af9a627fee67bb6f3d5b216a6
  status: live
  see-also: docs/paper/corona-heating-ladder.md docs/TODO.md
-->

# Handover — der 335-Konfund-Widerspruch

Die eine offene Pflicht der Korona-Untersuchung: warum kollabiert der
304→131-Aufwärts-Befund unter dem Konfund C=335, aber nicht unter GOES oder 94?

## 1. Gemessen (2014, 989 Ereignisse, D|C bei 96 s)

| Konfund | 304→131 D|C |
|---|---|
| GOES (Röntgen b_flux) | +4.26e-2 aufwärts |
| 94 Å (Fe XVIII, logT 6.81) | +8.08e-3 aufwärts |
| 335 Å (Fe XVI, logT 6.43) | ≈ 0 (−9.46e-4) |

304→131 ist bandbreiten-stabil (h 0.5–3.0, kippt nie) und dreijährig
reproduziert (2013 +5.20e-2, 2014 +4.26e-2, 2015 +4.63e-2, je GOES-Konfund).
Der einzige Bruch ist C=335. Volle Zahlen: Paper §4.6.

## 2. Der Riss

304 (He II, logT 4.70) und 131 (Fe VIII, logT 5.57) sind kalte Kanäle.
335 (Fe XVI, logT 6.43) ist heiß — aber 94 (Fe XVIII, logT 6.81) ist noch
heißer und bricht 304→131 NICHT. Also ist es nicht „heiß gegen kalt"; es ist
etwas Spezifisches an 335.

## 3. Offene Hypothesen (messen, nicht annehmen)

- 335 trägt eine Zeitstruktur, die die 304→131-Richtung vollständig erklärt
  und die 94/GOES nicht tragen — prüfen: Korrelation 335↔304 und 335↔131
  gegen 94↔304 und 94↔131 im selben Fenster.
- Die lag-bewusste Null (max_lag 8) greift unter 335 womöglich anders — den
  max_lag-Sweep unter C=335 wiederholen.

## 4. Werkzeug

`tools/measure/src/bin/corona_conditional_probe.rs`:
`--confound goes|335|94 --h <f> --max-lag <n> --year <aia.bin> <goes-dir>`.
Fertige Logs: `/tmp/opencode/corona_conditional_2014_C335.log`,
`corona_conditional_2014_C94.log`, `corona_conditional_2014.log` (GOES).
