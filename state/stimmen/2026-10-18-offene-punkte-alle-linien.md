# Offene Punkte — alle Linien (2026-10-18)

Regel: **ein Punkt = ein Eigentümer** (`kontrakt-linien.md`). `pending an:X` ist verboten —
ein Blocker geht an future, nicht an eine Linie als Ausrede.

## mountain (folge363) — 1 offen
- **FITS-Subset-Leser** · eigen · gebaut (`fits_range.rs`, 4/4 Tests) · Quittung beim Emitter offen
  - Braucht: mycelium schließt `.5` — geroutet als `xfer:mountain…6~013eccdb`.
  - Eigentümer: mountain. Blocker: mycelium (Emitter). **Übergabe läuft, kein `pending an:X`.**

## mycelium (folge361) — 3 offen
- **UCAR-FPI + MODVOLC / L1** · blockiert · Bindung: operator (UCAR-DOI) + an:mountain (L1 MOD021KM)
  - **Vertragsbruch:** zwei Blocker, ein Punkt — gehört an future oder als Token weg.
- **Quellen-Surveys** · eigen · nächste Route BGI enumerieren + registrieren.
- **JUICE** · termin 2026-11-01.

## river (folge227) — 3 offen
- **K** · pending · Rat R-2026-10-17-j (5/5 A) — WDC-Reihe identifiziert K nicht.
- **Rätsel** · eigen, unerledigt · keine Blockade — Ausführung fehlt (Weberin-Pass).
- **Flyby** · termin 2026-11-01.

## sensory (folge259) — 2 offen
- **ox64-m2c** · wartend · Zustellung (DHL `LZ473049629CN`, 404).
- **europa-clipper** · termin · Flyby 2026-12-03.

## future (folge1) — 4 offen
- **Rat feuern** — R-j `temp+rename`, R-l `Paket first` · Braucht: Sitze (Schwarm zuerst).
- **Rat schließen** — R-i (A+S2), R-k (A), R-m (2:1 C) · Braucht: Schließ-Akt.
- **Zukunft-Tokens** — mountain `.7` (Gate-Entscheid), river `.15` (Braucht-Parser) · Braucht: Operator-Apply.
- **Operator-Queue** — 4 keyed · Aufnahme-Wort · Command-Modellzeile.

## Unbeglichene Token — 5
| Token | an | Inhalt |
|---|---|---|
| `mountain…6~013eccdb` | mycelium | Emitter schließt `.5` |
| `mountain…7~deecd285` | future | `--addressed` vs `--tokens` Gate-Entscheid |
| `mountain…8~6a8b77e8` | river | — |
| `river…13~ea59cd54` | mycelium | — |
| `river…15~dcdcb011` | future | Braucht-Parser (`commit_check.rs:229`) |

## Waisen — 26
`register_lookup --orphan-docs` → 26 Docs ohne lebenden Träger. Zuteilung: R-c-Briefe
(mycelium 9 · river 7 · sensory 16).

## Rat — 5 Zeilen
| ID | Stand |
|---|---|
| R-i | rat-konvergiert → Schließung (A+S2) |
| R-j | rat-offen → `temp+rename` |
| R-k | rat-konvergiert → Schließung (A) |
| R-l | rat-offen → Paket first |
| R-m | rat-konvergiert → Edit (2:1 C) |

## Summe
15 Handover-Punkte + 5 Token + 26 Waisen + 5 Rat-Zeilen.
**Vertragsbruch:** 1 (mycelium UCAR/MODVOLC — zwei Blocker, ein Punkt).
