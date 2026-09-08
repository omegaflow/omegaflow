<!--
  title: Handover — Weberin Sonnensystem: die zweite Linie je Körper-Klasse gebaut
  class: handover
  date: 2026-09-08
  sha256: d76dafc94f879778b16a44776a5a17bd57224bf11c7de79f65e2a0fab97fdaea
  status: live
  see-also: docs/handover/handover-2026-09-07-weberin-sonnensystem-kette.md docs/concepts/die-weberin.md docs/auftrag/auftrag-extern-weberin-zweitlinien.md docs/TODO.md
-->

# Handover — Weberin Sonnensystem: die zweite Linie je Klasse

Übergabe der Sitzung, die `handover-2026-09-07-weberin-sonnensystem-kette.md`
ausführte: die zweite unabhängige Positions-Linie je Körper-Klasse gebaut,
gemessen, registriert, manifestiert.

## 1. Gebaut und committet (je Klasse)

| Klasse | zweite Linie | Commit |
|---|---|---|
| Bestands-Inventar (73 Weltlinien) + juno-Namenskollision + voller 72er-Weave | Probe `weberin_body_verdict` register-driven | `1a1d10d` |
| Kleinkörper 2. Linie (TNO erster Kepler) | `mpcorb_compiler` → `mpcorb_distant.bin` (8.082 Distant Object) | `1a1d10d` |
| Kometen (encke) | dcom5-Kometen-Zweig (`CometRec`/`comet_state_at`), `BODY_COMET encke→2P` | `3d02304` |
| Planeten/Mond | INPOP19a (`BodyLine::Inpop`, SPK-Route) + Gaia DR3 SSO (TNO) | `0a79666` |
| Toleranz je Klasse | `PLANET_WEBERIN_TOL_M` 1e5 m (Planet) vs 1e6 m (Kleinkörper/Komet) | `8cf2551` |
| CDN + Register (inpop/dcom5/gaia) | 12 url-Zeilen `sources.φ` | `e0ca8e3` |
| Eisriesen dritte Linie | EPM2021 (IAA RAS) + `three_way_fold` | `4e0e6b3` |
| TNO DES-Survey | DES-Y6 → `des_y6_tno.bin` (814, ICRS-Zustandsvektoren) | `143fbce` |
| TNO CFHT-Survey | OSSOS. VII → `ossos_tno.bin` (840, baryzentrische Elemente) | `85f1811` |
| CFEPS + Buie-DES | `descoped` (dieselbe Wurzel wie OSSOS/DES-Y6) | `281e4fb` |

## 2. Gemessene Befunde (0 honored)

- **Eisriesen:** EPM schlichtet den Riss nicht — DE/INPOP/EPM konvergieren bei
  Uranus/Neptun nicht (uranus spk-inpop 1.59e6 / spk-epm 1.52e6 / inpop-epm
  5.70e5 m; neptun spk-inpop 1.07e6 / spk-epm 8.10e6 / inpop-epm 8.20e6 m). Die
  Divergenz ist echt (drei unabhängige Abstammungen); der Riss ist die Messung.
- **TNOs:** der mpcorb-2-Körper-Kepler weicht von der DES-n-body-Fit um ~5e9 m
  ab (DES-σ ~2e8 m) — die 2-Körper-Näherung ist für TNOs nur ~5e6 km genau.
  Vier unabhängige Wurzeln stehen: MPC, CFHT/OSSOS, DES/DES-Y6, Gaia.
- **Sonden:** keine offene unabhängige Positions-Linie (`not-published`,
  gemessen). VLBI/ΔDOR/Range sind nicht offen publiziert; ISS = TLE, Solar
  Orbiter = ESA-OD (jeweils dieselbe Abstammung).

## 3. Descoped

CFEPS + Buie-DES (dieselbe CFHT-/DES-Wurzel wie die gebauten Zeugen) —
„Kopien zählen als ein Faden", kein Zeugnisgewinn.

## 4. Ehrlich `pending`

- Eisriesen-Schlichtung braucht eine dritte Observable (VLBI-Winkel), die nicht
  offen existiert — der Riss ist gemessen, nicht auflösbar.
- Breite TNO-Menge (8.082) trägt keine offene MPC-unabhängige Linie.
- Sonden-Zweitlinie bleibt `not-published`.

## 5. Register

Befunde + descoped stehen in `docs/TODO.md` (Sonnensystem-Weben) und
`docs/auftrag/auftrag-extern-weberin-zweitlinien.md`. Alles committet; die
CDN-Assets sind manifestiert (mpcorb, dcom5, inpop, gaia, epm, des_y6, ossos).
