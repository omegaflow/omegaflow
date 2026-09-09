<!--
  title: Handover — Orientierungs-Sonde gebaut und gemessen: der Matrix-Pfad trägt den Fix nur auf de441 earth, fünf stale Bins als Rekompilat-Stein
  class: handover
  date: 2026-09-09
  sha256: c73ac3d3cf3c328378913cb4fd312edf3afe5cf895f3ffe307ff9f1807183c42
  status: live
  see-also: docs/TODO.md docs/befund/befund-2026-09-09-orientierung-stale-matrizen.md docs/handover/handover-2026-09-09-matrix-fix-nachtrag.md
-->

# Handover — die Orientierungs-Sonde mißt die Matrix-Pfade

Übergabe für die nächste Sitzung. Der Membran-Folgeauftrag ist gebaut und
gemessen: die Sonde mißt die Orientierung, statt sie zu behaupten — und die
Messung widerspricht einer Behauptung des vorigen Handovers. Der Rekompilat
der stale Bins gehört dem konsolidierten Follow-up.

## 1. Was gebaut wurde

`tools/measure/src/bin/orientation_probe.rs` (headless, Muster
`eclipse_shadow_probe`). Je Linie (de441/de440/de442 ssd, inpop19a, epm2021)
earth+sun, plus de441 mars. Zwei Pfade: Matrix-Pfad (Bins wie gelesen) gegen
Analytik-Pfad (IAU-Zweig, Klon mit geleerten `rotation_matrices`). Sub-Solar-
Punkt über `light_time_worldline` + `icrs_to_body_surface`; Δ in km.
Funktions-Check: Sonnen-Elevation an DSS43 gegen die unabhängige
Lehrbuch-Formel (RA/Dec + IAU-1982-GMST + Ost-Länge), Schwelle 1,0°.
2 Tests (synthetisch Matrix≡Analytik + Lehrbuch-Gate, skip bei absent);
`cargo check -p omegaflow-measure` 0/0.

## 2. Was gemessen wurde (die Zahlen stehen im Befund)

- de441 earth: Matrix ≡ Analytik (0,0 km; Anker 11.8598/−95.8416 gegen
  11.8597/−95.8415); DSS43 Analytik −27,302° vs Lehrbuch −26,853° = 0,449° —
  der Fix hält, die Schwelle steht.
- **Kern-Befund:** de440/de442/inpop19a/epm2021 earth + de441 mars tragen
  Vor-Fix-Matrizen (Großkreis-Δs ~120–138°, Anker ~13 500–15 000 km) — exakt
  die Vor-Fix-Signatur. Das widerspricht
  `handover-2026-09-09-matrix-fix-nachtrag.md` §4 („INPOP/EPM tragen die
  korrigierten Matrizen schon"). Nur de441 earth (36 020 Matrizen, frisch
  rekompiliert) trägt den Fix.

## 3. Offene Steine (dem konsolidierten Follow-up übergeben)

1. **Rekompilat der fünf stale Bins** (de440/de442/inpop19a/epm2021 earth +
   de441 mars) mit dem fixierten Code — der `kernel_flatten`/
   `ephemeris_compiler`-Pfad (CDN-Manifestation-Duty). Registerzeile steht.
2. **Re-Verifikation** (galileo_elevation_match re-laufen nach dem Recompile)
   — der offene Stein der Ursprungs-Übergabe.
3. **Vorbestehend benannt, nicht mitgeführt:** TNO-Split
   (`asteroid_gm_sb441.φ` absent auf CI), `--omega-g` liest `solar_omega_g.φ`
   void, der ssd-earth-Struktur-Unterschied (18 MB/36 020 vs 183 MB/346 876
   Granulen, ungemessen).

## 4. Zustand des Arbeitsbaums

Fremde uncommittet (Parallel-Sitzung, unangetastet): `src/mathematikerin/te.rs`
(M), `docs/auftrag/auftrag-betriebspunkt-sweep.md` (neu). Nicht angefasst.
Diese Sitzung trägt: `orientation_probe.rs` (neu), das Befund-Blatt (neu),
diese Übergabe (neu), die TODO-Zeile.
