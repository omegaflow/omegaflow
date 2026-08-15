# K05-Mond-PA — Übergabe

Stand: 2026-08-15. Der FK-Teil von K05 ist committed (6a394e9, 8c7d531).
Die bpc-Komposition wurde in einer Session komplett gemessen und verifiziert,
dann auf Wunsch wieder aus dem Arbeitsbaum genommen — hier steht das Rezept,
damit die nächste Session die Komposition ohne die Sackgassen dieser Session
wieder einbauen kann.

## Was bewiesen ist (Messstand)

Datei `moon_pa_de440_200625.bpc` (12,86 MB, DAF: ND=2 NI=5, Segment 31008/1/typ2,
Adressen 641–1280644 = exakt 40000 Records à 32 Doubles; Leser in `src/bpc.rs`
ist spec-konform, NAIF-pck.req geprüft).

**Die drei Polynomialsätze sind die DE440-Lunarlibrationswinkel (Park et al.
2021, AJ 161 105, §2.4):**

- **Slot 0 („RA") = φ — Knotenlänge, in GRAD.** Bei J2000: −0,054° ≈ 0 ✓
- **Slot 1 („DEC") = θ — Neigung der Mantel-Äquatorebene zur ICRF-XY-Ebene,
  in RADIANT.** Bei J2000: 0,424855 rad = 24,343°; trackt die
  Nodal-Oszillation (22,09° nach 5 Jahren ✓).
- **Slot 2 („W") = ψ — Twist ab Knoten, in RADIANT.** Drift 0,229987 rad/Tag
  = **13,177°/Tag = exakt die Rotationsrate** (Header-Konstante
  0,229944858937522340 im DE440-ASCII-Header bestätigt). Der frühere
  „0,23°/Tag-Mysterienbefund" war Grad-vs-Radiant.

**Komposition (Paper §2.4, Gl. (8)):** r_I = R3(φ)·R1(θ)·R3(ψ)·r_PA
(Standard-Spaltenmatrizen). **Gl. (19):** r_MER = R_x(0,2785″)·R_y(−78,6944″)·
R_z(67,8526″)·r_PA — die .tf-TKFRAME_31009 speichert davon die **Inverse**
(„angles negated, sequence reversed"), also M_tk = N⁻¹ → M_ICRF←ME = M_PA·M_tk.

**Verifikation (Probe `--probe 2451545.0 --bpc … --fk …`, 2026-08-15):**

```
me = (RA −90,014° ≡ 269,986°, DEC 65,672°, W 41,159°), W-Drift +13,186°/Tag
IAU-Vollmodell J2000:     (269,9949°,      65,654°,     41,236°)
Abweichung:                 0,009°           0,018°       0,077°  ✓
```

Die Restdifferenz ist die DE440-Integration vs. die IAU-Serien — genau die
Korrektur, die der Kanal tragen soll. Referenz: Horizons ObsSub mit
**CENTER='500@399' + TARGET='301'** (sub-Erdpunkt auf dem MOND — die
umgekehrte Query liefert den sub-Mondpunkt auf der ERDE, das war die
Phantom-Diskrepanz dieser Session).

## Das Rezept (in `src/bin/ephemeris_compiler.rs` wieder einbauen)

1. `matmul` auf Spaltenlayout umstellen (flach = Spalten konkateniert,
   Spaltenvektor-Konvention): `o[3*j+i] = a[i]*b[3j] + a[3+i]*b[3j+1] + a[6+i]*b[3j+2]`.
   **Achtung:** `rotation_matrix_from_angles` baut sein [T,E,U] direkt ohne
   `matmul` — die stype-3-Röhre bleibt unberührt. Nur der Libration-Pfad nutzt
   `matmul`.
2. `libration_matrix(phi_deg, theta_rad, psi_rad)`:
   `R3(φ)·R1(θ)·R3(ψ)` mit Standard-Spaltenmatrizen
   (`r3=[c,s,0,−s,c,0,0,0,1]`, `r1=[1,0,0,0,c,s,0,−s,c]`).
3. `full_orientation`: bpc liefert (φ,θ,ψ) → `M_pa = libration_matrix(...)` →
   falls `fk.tkframe_child_of` (ANGLES-Kind) existiert: `M_me = matmul(M_pa, rot)`
   → IAU-Extraktion:
   - Pol = Spalte 2: `ra = atan2(m[7], m[6])`, `dec = asin(m[8])`
   - W: n = (−p_y, p_x, 0)/|n| (aufsteigender Knoten), T auf die Äquatorebene
     projizieren, `W = atan2((n×T_p)·p, n·T_p)`
   - Zweig-Umklappung gegen die Linear-Dec (|Δ|>90° → ra+180, −dec, w+180).
4. `linear_orientation`: RA/DEC-Basis = `pole_ra_at`/`pole_dec_at` (enthalten
   die Text-PCK-Serien — konsistent mit der Runtime, die `nut_ra`/`nut_dec`
   addiert); PM-Basis bleibt linear (`pm_at`, die Runtime hat keine PM-Serie).
5. `select_system` (planets): `moon_pa*.bpc` + `moon_de440*.tf` via `pick_spk`
   hinzufügen (CI zieht sie dann automatisch).
6. Probe-Modus nutzt dieselben Helfer.

## Fallen dieser Session (nicht wiederholen)

- Die „W-Drift 0,23°/Tag" war die Rotationsrate in Radiant — kein Fehler.
- Der frühere „R nicht konstant"-Befund stammte von der falschen
  Horizons-Referenz (sub-Mondpunkt auf der Erde).
- `rotation_matrix_from_angles` = R3(ra)·R1(dec)·R3(pm−ra) — die
  Compiler-Parametrisierung. **Nicht anfassen**: Runtime-Konsum (stype-3,
  `geodetic_to_icrs` Spaltenlesung) ist mit ihr selbstkonsistent, die
  Erd-Stationen sind live verifiziert.
- Offen für die nächste Session: der exakte Delta-Raum der stype-4-Röhre
  (Runtime `orientation_angles_at` = linear + nut + deltas) — die hier
  gemessene IAU-Extraktion passt auf den Pol-Kanal (RA/DEC/W), der
  stype-3-Weg für Mond-Oberflächenframes sollte daneben geprüft werden.
- Die Parallel-Sessions haben den Arbeitsbaum einmal auf HEAD
  zurückgesetzt — K05-Arbeit früh committen.

## Artefakte

- `docs/plans/K05_runtime_auftrag.md` (historisch), TODO.md (K05-Eintrag,
  Commit 8c7d531) — Messwerte und Befunde.
- /tmp/opencode/omegaflow_k02/: `moon_pa_de440_200625.bpc`, `moon_de440_250416.tf`,
  `de440.bsp`, `header.440`, Horizons-Referenzen (`hz_obs_moon.txt`,
  `hz_vec_multi.txt`), `park2021.txt` (Paper-Text, Gl. 8/19 extrahiert).
- `src/fk.rs` steht (committed, 6a394e9) — Parser, TKFRAME-Auflösung, Tests.
