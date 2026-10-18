# Lage — generiert 2026-10-18T15:44:36Z

HEAD `2e402983c`

## Sitze / Routen (Gateway)
- Katalog (tsv, letzter Start: 2h): 1917 · frei: 47 · Preis unbekannt: 1007 · lebendig: 9 (3h alt)
- Freier Schwarm: live aus den Preis-0-Zeilen + OpenCode-free — keine feste Zahl
- Guthaben (USD):
  openrouter	-1.7770357660000116
  deepseek	214.24
- Rat-Sitze: `state/stimmen/_rat-anweisung.md` (Leiter; 17.10.-Tabelle überholt)

## Offen je Linie (Handover)
  mountain   1 offen ·  1 Braucht ·  0 offene Traeger · handover-2026-10-18-mountain-folge363.md (Folge 2026-10-18 · Commit 0h)
  mycelium   3 offen ·  0 Braucht ·  1 offene Traeger · handover-2026-10-16-mycelium-folge361.md (Folge 2026-10-16 · Commit 0h)
  river      3 offen ·  3 Braucht ·  3 offene Traeger · handover-2026-10-18-river-folge227.md (Folge 2026-10-18 · Commit 0h)
  sensory    2 offen ·  2 Braucht ·  0 offene Traeger · handover-2026-10-14-sensory-folge259.md (Folge 2026-10-14 · Commit 0h)
  future     5 offen ·  5 Braucht ·  0 offene Traeger · handover-2026-10-18-future-folge1.md (Folge 2026-10-18 · Commit 2h)

## Rat-Fragen (offen | rat-konvergiert)
  | R-2026-10-18-i | 2026-10-18 | **F1 — Quittung im Token-Ledger** — ACK im Empfänger-Ledger vs. `CLOSED` im Emitter; `--addressed` S1/S2 | Fugu · Lumo · DeepSeek | **Runde 2 gefahren** — 2:1 **A + S2** (Fugu, Lumo), DeepSeek **C + S2**; Schwarm R1 A. **rat-konvergiert → Schließung** | `2026-10-18-f1-quittung-formular.md` | →future | `2026-10-18-rat-3f-runde2-sammlung.md` |
  | R-2026-10-18-j | 2026-10-18 | **F2 — Concurrency-Schlüssel `register-coverage`** — per-SHA vs. per-ref vs. Debounce; das Gate schreibt nicht | Fugu · Lumo · DeepSeek | **Runde 2 gefahren** — 2:1 **B** (Lumo, DeepSeek), Fugu **A**; Schwarm R1 A. **rat-offen → `temp+rename`** (`2026-10-18-r-j-temp-rename-formular.md`) | `2026-10-18-f2-concurrency-formular.md` | →future | `2026-10-18-rat-3f-runde2-sammlung.md` |
  | R-2026-10-18-k | 2026-10-18 | **F3 — Register liest `state/stimmen` nicht** — Dokumentwurzel vs. Ableitung vs. eigener Index; RAT-ID vorher? | Fugu · Lumo · DeepSeek | **Runde 2 gefahren** — 3:0 **A**; Zusatz (RAT-ID vorher?) 1:2 **nein** (nur Fugu ja); Schwarm R1 A. **rat-konvergiert → Schließung** | `2026-10-18-f3-register-stimmen-formular.md` | →future | `2026-10-18-rat-3f-runde2-sammlung.md` |
  | R-2026-10-18-l | 2026-10-18 | **River-Triade** — Receiver-Apertur Schritt 3 · Schumann `field` (WGLC-Gitter) · Flyby-`sealed`-Träger | Schwarm R1 (6 von 44 antwortfähig) | **offen** — Schwarm R1: F1 **A 6:0** · F2i **A 4:1** · F2ii **nein 4:0** · F3 **2:3 gespalten**. **rat-offen → Paket first** (`2026-10-18-r-l-paket-first-formular.md`) | `2026-10-18-river-apertur-schumann-flyby-formular.md` | →future · →river · →mountain | `2026-10-18-rat-river-triade.md` |
  | R-2026-10-18-m | 2026-10-18 | **future.md-Zusatz** — Lage-Zeile, Pass-Kontrakt, Bringschuld: behalten oder streichen? | Fugu Max (UI) · Lumo 2.0 (UI) · DeepSeek V4 Pro (paid) | **gefahren** — Fugu **C**, Lumo **C**, DeepSeek **B** (2:1 C); Burn-Zeile alle **nur bei Überschreitung**. **rat-konvergiert → Edit** (edit-deny: Operator-Hand) | `2026-10-18-rat-future-command-formular.md` | →future | `2026-10-18-rat-m-sammlung.md` |

## Letzte Läufe / Sammlungen
  _lage.md
  2026-10-18-rat-m-sammlung.md
  2026-10-18-rat-3f-runde2-sammlung.md
  2026-10-18-rat-h-sammlung.md
  2026-10-18-rat-3f-sammlung.md
  2026-10-15-venus-bild-rat-verdikt.md
  2026-10-15-china-planetendaten-rat-verdikt.md
  2026-10-15-mountain-rat-fragen-sh-kernel-verdikt.md

Regenerieren: `bin/lage`
