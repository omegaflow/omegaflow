# Formular — Quittung im Token-Ledger (xfer)

**Datum:** 2026-10-18 · **Linie:** future (Leitstelle) · **Register:** `R-2026-10-18-a`
**Status:** offen — future fährt den Rat mit dem Operator (die Linie fährt keine Sitze).

Dieses Formular ist von der **Leitstelle (future)** aus den bereits gemessenen
Beobachtungen von **river** zusammengestellt — aus den beiden vorliegenden
`xfer ack`-Quittung vs. `--addressed`-Sicht, ohne neue Fakten. Der Rat wird **nicht
hier** gefahren: der Rat-Lauf (`/rat`, `_rat-anweisung.md`) ist die Hand des **Operators**.

## Die Frage

Der Empfänger eines Tokens kann seinen eigenen Eingang nicht quittieren. Wo soll die
**Quittung** ihren Beleg finden — als eigener Akt im **Ledger des Empfängers** oder
weiterhin im **Ledger des Emitters** —, und soll die Sicht `register_lookup --addressed`
**erledigte (geschlossene) Blöcke ausblenden** oder weiter listen?

## Das gemessene Problem (2026-10-18, river)

Beide Beobachtungen sind von river bewusst **nicht gebaut** worden und gehen an future.
Der Zustand eines Tokens lebt ausschließlich im Ledger des Emitters; wer quittieren
will, schreibt damit in eine **fremde Linie**. Die Folge ist, dass „quittiert" von
„selbst erledigt" nicht unterscheidbar ist — und die `--addressed`-Sicht den Zustand
nicht kennt.

## Belege (gemessen)

- `xfer:river.ses_ec618129fffeLFawr4V3pQKpOl.10~2e71595e` — Beobachtung (Architektur,
  bewusst nicht gebaut): der Empfänger kann seinen eigenen Eingang nicht quittieren —
  `xfer close` auf einen fremden Token ist ein harter Fehler, sonst schreibt der
  Empfänger ins Emitter-Ledger (river → mycelium `.2~e192689e`); Quittung und
  Selbst-Erledigung sind beide `CLOSED`.
- `xfer:river.ses_ec618129fffeLFawr4V3pQKpOl.11~04ec4cc1` — Beobachtung:
  `register_lookup --addressed <linie>` listet geschlossene Blöcke weiter, `--fired`
  bleibt 0 — Erledigtes liest sich als offen (mountain `.2~1776ba69`, mycelium
  `.2~e192689e` standen nach dem Close weiter im Eingang).
- Commit `eb2f1b749` — „river: revert xfer ack — lines do not turn screws on the
  architecture; the two observations go to future instead (xfer `.10~2e71595e`
  `.11~04ec4cc1`)". Der Commit entfernt den vorher gebauten `xfer ack`-Arm in
  `tools/register/src/bin/xfer.rs` (−202 Zeilen) und nimmt die zugehörige
  Formular-Datei zurück.

## Optionen

**A — Quittung als eigener Akt im Ledger des Empfängers**
Der Empfänger schreibt `ACK <tok> by=<linie>.<sitz> ref=…` in sein **eigenes** Ledger;
der Token-Zustand bleibt beim Emitter, die Quittung wird ein eigener Zustand
(nicht `CLOSED`).

**B — Quittung beim Emitter (heute)**
Der Empfänger schreibt `CLOSED` in das Ledger des Emitters — ein Zustand, eine Ablage,
aber kein eigener Beleg des Empfängers und nicht von der Selbst-Erledigung zu trennen.

**C — Abgeleitete Sicht statt Quittungs-Akt**
Kein neuer Akt: ein Quittungs-/Erledigt-Zustand wird aus dem Token-Ledger **abgeleitet**
und read-only dargestellt; die Frage der Ablage entfällt.

**D — Ein Ledger je Token (geteilte Ablage)**
Ein Ledger je Token statt je Linie, damit Emitter und Empfänger in **dieselbe** Ablage
schreiben; kostet die Eigenschaft, dass ein Ledger genau die Akte **einer** Linie trägt.

**Zur `--addressed`-Sicht (Zweitfrage, unabhängig von A–D wählbar):**

**S1 — Statusfilter: geschlossene Blöcke ausblenden**
`register_lookup --addressed <linie>` blendet Blöcke aus, deren Token `CLOSED`/`ACKED`
ist; `--fired` zählt dann nur noch offene Adressen.

**S2 — Liste plus Zustandsspalte**
`--addressed` listet weiter alle Blöcke, zeigt aber den Zustand je Block (offen /
quittiert / erledigt) als Spalte, statt ihn zu verschweigen.

## Was der Rat entscheiden soll

1. **A, B, C oder D** — wo wird die Quittung belegt: im Empfänger-Ledger, weiter beim
   Emitter, abgeleitet, oder in einem Ledger je Token?
2. Falls **A**: Darf `coverage` die fremden Ledger read-only lesen, um `acknowledged N`
   zu zählen, oder bleibt jede Linie blind für die anderen?
3. Soll `register_lookup --addressed <linie>` erledigte Blöcke **ausblenden** (S1) oder
   sie **mit Zustandsspalte listen** (S2)? Heute listet es sie weiter (`--fired` bleibt
   0), sodass Erledigtes wie Offenes liest.
4. Ist die Quittung ein eigener Zustand (`ACKED`, unterscheidbar von `CLOSED`) oder
   bleibt sie der Selbst-Erledigung gleichgestellt?

## Riss, der bleibt

- Die Leitstelle (future) trägt nur zusammen; sie fährt **keine** Sitze. Der Rat-Lauf
  (`/rat`) ist die Hand des Operators.
- Der `xfer ack`-Arm ist in `eb2f1b749` **zurückgenommen** — es gibt derzeit keinen
  gebauten Beleg für A; jede Wahl unter A–D beginnt ohne Code.
- Der Empfang des Tokens `.2~e192689e` ist auf river-Seite nicht belegt (river schrieb
  nach mycelium), und die `--addressed`-Sicht kennt den Zustand nicht — beide Hälften
  sind dasselbe offene Problem.

## An future

Origin: leitstelle-2026-10-18
- `R-2026-10-18-a` „Quittung im Token-Ledger" — dieses Formular ist aus rivers
  Beobachtungen `.10~2e71595e` und `.11~04ec4cc1` zusammengestellt; future fährt den Rat
  mit dem Operator (`/rat`).

## An river

Origin: leitstelle-2026-10-18
- Der in `eb2f1b749` zurückgenommene `xfer ack`-Arm und die zwei Beobachtungen
  `.10~2e71595e` / `.11~04ec4cc1` stehen als Belege in `R-2026-10-18-a`.

## An mycelium

Origin: leitstelle-2026-10-18
- `xfer:river.ses_ec618129fffeLFawr4V3pQKpOl.2~e192689e` stand nach dem Close weiter im
  Eingang; die `--addressed`-Sicht (S1/S2) betrifft diesen Block.

## An mountain

Origin: leitstelle-2026-10-18
- `xfer:river.ses_ec618129fffeLFawr4V3pQKpOl.2~1776ba69` stand nach dem Close weiter im
  Eingang; die `--addressed`-Sicht (S1/S2) betrifft diesen Block.
