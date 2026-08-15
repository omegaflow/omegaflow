# K05-Mond-BPC — Wiedereinbau

Stand: 2026-08-15. Session-Plan für den Wiedereinbau der verifizierten
bpc-Komposition (R3(φ)·R1(θ)·R3(ψ) + Gl. 19, M_ICRF←ME = M_PA·M_tk) in
`src/bin/ephemeris_compiler.rs`. Messstand und Fallen: siehe
`docs/plans/K05_mond_bpc_uebergabe.md` (das Rezept ist dort komplett).

## 0. Session-Setup
- Artefakte aus `/tmp/opencode/omegaflow_k02/` prüfen (`moon_pa_de440_200625.bpc`,
  `moon_de440_250416.tf`); falls /tmp weg ist: per `--fetch-from` neu ziehen
  (nach Schritt 4 automatisch) oder curl von NAIF.
- Früh committen (Falle aus der Übergabe: Parallel-Sessions haben den Baum
  zurückgesetzt).

## 1. `matmul` auf Spaltenlayout (Z. 244)
- `o[3*j+i] = a[i]*b[3j] + a[3+i]*b[3j+1] + a[6+i]*b[3j+2]`.
- Blast radius: `apply_fixed_rotation` (Z. 266) und Probe (Z. 1520/1528) —
  beide werden in Schritt 2/5 ersetzt. `fk.rs`-`matmul` bleibt unberührt
  (eigenes Modul).
- Nach Umbau auf tote Helfer prüfen (`mat_transpose`,
  `angles_from_rotation_matrix`, `apply_fixed_rotation`), 0-Warning-Regel.

## 2. `libration_matrix` + `full_orientation` (Z. 275)
- Neu: `libration_matrix(phi_deg, theta_rad, psi_rad)` → R3(φ)·R1(θ)·R3(ψ),
  Spaltenmatrizen flach (`r3=[c,s,0,−s,c,0,0,0,1]` — ist bereits
  Spaltenlayout, konsistent mit Schritt 1).
- `full_orientation`: bpc liefert (φ [GRAD], θ [rad], ψ [rad]) →
  `M_pa = libration_matrix(...)` → falls `tkframe_child_of` (ANGLES)
  existiert: `M_me = matmul(M_pa, rot)` — **ohne** Transpose →
  IAU-Extraktion:
  - Pol = Spalte 2: `ra = atan2(m[7], m[6])`, `dec = asin(m[8])`
  - W über Knoten `n = (−p_y, p_x, 0)/|n|`, T auf Äquatorebene projiziert,
    `W = atan2((n×T_p)·p, n·T_p)`
  - Zweig-Umklappung gegen Linear-Dec (|Δ|>90° → ra+180, −dec, w+180)
- Ohne bpc/TKFRAME: Text-PCK-Fallback (`pole_ra_at`/`pole_dec_at`/`pm_at`,
  Z. 310–315) unverändert.

## 3. `linear_orientation` (Z. 318) — Abweichung vom Übergabe-Rezept
- Das Rezept (RA/DEC-Basis = `pole_ra_at`/`pole_dec_at`) wurde **nicht**
  übernommen: Die Runtime-Binaries tragen kein nut_ra/nut_dec —
  `parse_ephemeris_binary` setzt beide None (stype-1 = 12 Params, keine
  NUT-Serie). Die stype-4-Delta-Basis muss deshalb das pure Linear bleiben
  (`pole_ra_deg + rate·tc`), damit Runtime-Rekonstruktion
  (linear + deltas) = voll exakt ist. Mit der Rezept-Basis würden die
  Text-PCK-NUT-Serien (K02-Röhre, pck00010: Jupiter/Mond tragen Serien)
  im Delta verschwinden — der Flatten-Lauf zeigte den Verlust
  (Jupiter 0 nutation). Gemessen: Basis = Linear (wie zuvor), Jupiter
  12556 Sektionen (Serie fließt über die Röhre), Mond 12556
  (Delta = DE440 − Text-PCK-linear).
- Zweig-Umklappung in `full_orientation` gegen die Linear-Dec
  (`linear_orientation`), nicht gegen `pole_dec_at`.

## 4. `select_system` planets-Zweig (Z. 967)
- `moon_pa*.bpc` (family `bpc`) + `moon_de440*.tf` (family `fk`) via
  `pick_spk` hinzufügen → CI zieht sie automatisch (`download_missing` →
  `classify` konsumiert beide Familien bereits).

## 5. Probe-Modus (Z. 1476–1544)
- Dieselben Helfer wie `full_orientation` (libration_matrix, matmul,
  IAU-Extraktion) — Probe-Ausgabe = Produktionspfad.

## 6. Verifikations-Gate (manuell, `cargo check` reicht nicht)
1. `cargo check`: 0 errors, 0 warnings; `cargo test` grün.
2. Probe J2000: `--probe 2451545.0 --bpc moon_pa_de440_200625.bpc --fk
   moon_de440_250416.tf` → me = (269,986°, 65,672°, 41,159°), W-Drift
   +13,186°/Tag, Abweichung gegen IAU-Vollmodell (269,9949°, 65,654°,
   41,236°) ≤ 0,1° — Layout-Fehler zeigen sich sofort (~24°-Abweichung).
3. Flatten-Lauf Mond: stype-4-Nutationssektion plausibel klein; stype-3-Röhre
   unverändert.
4. Runtime-Gegenprobe: Binary lädt, Erd-Stationen (stype-3) identisch — die
   live verifizierte Röhre nicht gestört.

## 7. TODO.md im selben Commit
- K05-Eintrag: Merge eingebaut (statt „parkt an Rezept") — Commit schließt
  den Eintrag.
- K02-Befund (Z. 63–69) trägt noch den alten Fehlbefund („W-Drift 0,23°/Tag
  statt 13,176°/Tag, parkt deshalb an K05") — bereinigen (Git trägt die
  Historie).

## Ausführung 2026-08-15

Eingebaut und verifiziert (Schritte 1–7, Schritt 3 mit der Abweichung oben):

- Probe J2000: `me = (−90,014° ≡ 269,986°, 65,672°, 41,159°)`,
  W-Drift +13,186°/Tag — exakt die Übergabe-Werte; das Rezept-literal
  `matmul(M_pa, rot)` (ohne Transpose) war korrekt.
- Flatten (de440.bsp + pck00010 + moon bpc/tf): Mond 12556 Granules +
  12556 Rotationen + 12556 stype-4-Sektionen (10,4 MB); Jupiter
  12556 stype-4 (NUT-Serie über die Röhre); Planeten ohne Serie 0.
- `cargo check` 0 W / 0 E, `cargo test` 34 lib + 34 bin grün.
- Runtime-Smoke: Server läuft mit dem neuen /tmp/omegaflow_eph_moon.bin,
  keine Parse-Diagnosen; Browser-Rendering (Fenster nicht schwarz) bleibt
  der Operator-Schritt.
- TODO.md: K05 geschlossen, K02-Befund berichtigt (Grad-vs-Radiant),
  Stand-Zeile aktualisiert.
