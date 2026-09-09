<!--
  title: Befund — Orientierungs-Sonde: fünf Linien, ein korrigierter Matrix-Pfad — de440/de442/inpop/epm earth + de441 mars tragen Vor-Fix-Matrizen
  class: befund
  date: 2026-09-09
  sha256: 52532df21d2f4d79a5c7685f7ad8fb88af2dbabaa9da68a5ee1d11d566f7f33e
  status: live
  see-also: docs/TODO.md docs/handover/handover-2026-09-09-matrix-fix-nachtrag.md docs/handover/handover-2026-09-09-finsternis-schattenortung.md
-->

# Befund: die Orientierungs-Sonde misst die Matrix-Pfade

## Frage & Bindung

Der Membran-Folgeauftrag (`handover-2026-09-09-matrix-fix-nachtrag.md` §5.2):
eine Sonde, die die Orientierung im ω()-Kreislauf misst, statt sie zu
behaupten. Die Membran nimmt den Matrix-Pfad (`body_fixed_to_icrs`
Nearest-Rotations-Matrix-Zweig, `src/archivar/motion.rs`), sobald ein Bin
`rotation_matrices` trägt. Die Sonde (`tools/measure/src/bin/orientation_probe.rs`,
Muster `eclipse_shadow_probe`, headless, kein visible run) misst den
Sub-Solar-Punkt über zwei Pfade — Matrix-Pfad (Bins wie gelesen) gegen
Analytik-Pfad (IAU-Zweig, Klon mit geleerten `rotation_matrices`) — und die
Sonnen-Elevation an DSS43 gegen die unabhängige Lehrbuch-Formel (scheinbare
RA/Dec + IAU-1982-GMST + Ost-Länge), Schwelle 1,0°. Real Bins, je Linie
earth+sun (de441/de440/de442 ssd, inpop19a, epm2021), plus de441 mars.

## Die Messung

Sub-Solar-Δ = Großkreis-Abstand Matrix-Pfad minus Analytik-Pfad (km) über
9 Sweep-Epochen der Granulen-Spanne + Anker 2017-08-21T18:26:40Z.

| Linie · Körper | Matrizen | Sweep Δ max km | Sweep Δ median km | Anker Δ km | DSS43 Matrix ° | DSS43 Analytik ° | DSS43 Lehrbuch ° |
|---|---|---|---|---|---|---|---|
| de441 · earth | 36 020 | 0,0 | 0,0 | 0,0 | −27,302 | −27,302 | −26,853 |
| de440 · earth | 12 556 | 14 420,7 | 12 760,1 | 13 564,2 | +11,767 | −27,302 | −26,853 |
| de442 · earth | 12 556 | 14 420,7 | 12 760,1 | 13 564,2 | +11,767 | −27,302 | −26,853 |
| inpop19a · earth | 2 340 | 15 336,3 | 10 258,5 | 15 009,2 | +16,241 | −27,302 | −26,853 |
| epm2021 · earth | 4 875 | 15 395,2 | 11 468,4 | 13 564,2 | +11,767 | −27,302 | −26,853 |
| de441 · mars | — | 15 199,5 | 11 603,2 | 6 045,3 | — | — | — |

## Verdict

Funktions-Check bestanden: de441 earth trägt den Matrix-Fix — Matrix ≡
Analytik (Sweep und Anker 0,0 km; Sub-Solar-Anker Matrix 11.8598/−95.8416
gegen Analytik 11.8597/−95.8415). DSS43: Analytik −27,302° gegen Lehrbuch
−26,853° = 0,449°, unter der 1,0°-Schwelle. Der Analytik-Pfad ist über alle
fünf Linien identisch (−27,302°) und gegen das Lehrbuch gesund — die
Analytik ist kein Fehler-Träger.

Kern-Befund (gemessen, widerspricht `handover-2026-09-09-matrix-fix-nachtrag.md`
§4 „INPOP/EPM tragen die korrigierten Matrizen schon"): die Matrix-Pfade von
de440, de442, inpop19a und epm2021 earth sowie de441 mars tragen die
Vor-Fix-Matrizen — Großkreis-Δs ~120–138° (Anker ~13 500–15 000 km), exakt
die Vor-Fix-Signatur (~117°-Klasse, `w = pm − ra` mit vertauschten Zeilen).
Nur das frisch rekompilierte de441 earth (36 020 Matrizen) trägt den Fix.

## Register-Pflicht

Der Rekompilat der fünf stale Bins (de440/de442/inpop19a/epm2021 earth +
de441 mars) mit dem fixierten Code ist ein neuer offener Stein des
konsolidierten Follow-ups — diese Sitzung baute und maß, kompilierte nicht.
Vorbestehend benannt, nicht mitgeführt: der ssd-earth-Struktur-Unterschied
(18 MB / 36 020 vs 183 MB / 346 876 Granulen, ungemessen), der TNO-Split
(`asteroid_gm_sb441.φ` absent auf CI), `--omega-g` liest `solar_omega_g.φ`
void.
