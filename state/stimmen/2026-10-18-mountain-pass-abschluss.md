# Mountain — Pass-Abschluss (R-2026-10-18-c, 4/4 · 2026-10-18)

**Linie:** mountain · **Trigger:** `.opencode/commands/mountain.md` (deepseek-v4.1-flash-free#low)
**Vorgänger:** `docs/handover/handover-2026-10-18-mountain-folge360.md` (unverändert, 1000 B)

## Messung

| Schicht | Wert |
|---|---|
| Post (`register_lookup --addressed mountain`) | 2 Blöcke — alle Token bereits `CLOSED` (river `.1~d03aa55a`, sensory `.1-.5,.11`) |
| Post (`ls post/in/*-an-mountain*.md`) | 0 — `post/in` nicht im Baum |
| Trigger (`register_lookup --fired mountain`) | 0 |
| Transfer (`xfer coverage --line mountain`) | emitted 5 · disposed 2 · **open 3** (`.3/.4/.5`, Ziel `future`) |

Priorität Post > Trigger > Transfer: Post und Trigger sind beglichen; es bleibt der
Transfer. Die drei offenen Token sind **Empfänger-Arbeit bei `future`** (kein
`future`-Ledger im Baum) — in dieser Sitzung nicht schließbar, **kein Ledger-Schluss
gesetzt** (R-2026-10-18-i/F1 offen).

## Delegation und Prüfung

Drei freie `general`-Subagenten (`opencode/deepseek-v4.1-flash-free`), je ein Punkt,
done-when vom Pass: Belegen/Widerlegen der gemessenen Behauptungen in den drei
Formularen. Belege geprüft gegen den Baum.

## Geschlossen (zwei Punkte)

- **GRS80-Formular, Direktiv-Arm** — Zitat `src/archivar/parse.rs`:
  `"gm"` 2282 · `"radius"` 2288 · `"norm"` 2297 · `"tide"` 2303 · `"max_degree"` 2304;
  `"f"`/`"omega"`/`"grs80"` = 0 Treffer, das einzige `"a"` (2027) ist KeplerMap.
  Das Formular trug `2270-2299` (tide/max_degree außerhalb). Korrigiert auf `2282-2304`.
- **P10-Restarme-Formular, Quellen** — Zitat `-p10-em.φ:123-124`:
  „Ende (HEAD dieser Tranche): field 7526 · quantity 382" / „Δ field −241 · Δ quantity +241";
  FDSN-BHZ ×2 in `-p10-migration.φ:59`; `kg/(m^2 s)`-Token in `-unit-token.φ:8-12`
  (`channel`-Direktive, `unknown channel unit`, nicht Deskriptor-Parser). Belege nachgezogen.

**Beleg:** Commit `406528e14` (beide Formulare). Zitate oben.

## Geprüft ohne Befund

gfc-Formular: Asset-Größen `-gfc-asset-plan.φ:65-71` (5540/3211/2190/760/719),
Tail `-gfc-depth2.φ:69-71` = 5480 (nicht 5285), `gfc.bin` 404, Epoche durchgehend
`absent` — die Formularzahlen tragen.

## Nicht geschlossen (Urteil bleibt bei der Linie)

- R-2026-10-18-b (GRS80-Register), R-2026-10-18-f (gfc-Block), R-2026-10-18-g
  (P10-Restarme) — der Operator wählt die Sitze (`_rat-anweisung.md`), nicht delegierbar.
- Die drei offenen Transport-Token `.3/.4/.5` — Ziel `future`, Empfänger-Arbeit.
