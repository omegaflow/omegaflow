# Formular — Quittung im Token-Ledger (xfer)

**Datum:** 2026-10-18 · **Linie:** river · **Register:** `R-2026-10-18-a`
**Status:** offen — die Sitze sind noch nicht gefahren (Operator wählt).

## Die Frage

Der Empfänger eines Tokens kann seinen eigenen Eingang nicht quittieren. Soll die
Quittung ein **eigener Akt im Ledger des Empfängers** sein (Variante A, gebaut), oder
soll der Empfänger weiterhin **in das Ledger des Emitters** schreiben (Variante B, heute)?

## Der gemessene Defekt (2026-10-18, river)

`xfer close <tok> --line <empfänger>` endet hart:

```
xfer: close: --line river differs from the token line mycelium
```

Der Grund ist die Ablage: der Zustand eines Tokens lebt ausschließlich in
`state/zustand/ledger/<emitter>/<sitzung>.log`. Wer quittieren will, schreibt in eine
**fremde Linie**. Folgen, alle drei am 2026-10-18 aufgetreten:

1. River hat den Mycelium-Token `.2~e192689e` mit `--line mycelium` geschlossen — river
   hat in das Ledger von mycelium geschrieben.
2. Rivers eigene `coverage` zeigt davon nichts; der Empfang ist dort nicht belegt.
3. Die Quittung ist nicht unterscheidbar von der Selbst-Erledigung des Emitters: beide
   sind `CLOSED`.

## Die Optionen

**A — Quittung beim Empfänger (gebaut, `xfer ack`)**
`xfer ack <tok> --line <empfänger> --ref <commit|pfad>` schreibt `ACK <tok> by=<linie>.<sitz> ref=…`
in das **eigene** Ledger. Der Token-Zustand bleibt beim Emitter; `coverage` liest die
Quittungen fremder Linien read-only und zeigt `acknowledged N`. Kein Schreibzugriff über
Liniengrenzen. Nur der Adressat darf quittieren (`to=` aus dem SENT-Satz).

**B — Quittung beim Emitter (heute)**
Der Empfänger schreibt `CLOSED` in das Ledger des Emitters. Ein Zustand, eine Ablage —
aber der Empfänger hat keinen eigenen Beleg, und „quittiert" ist nicht von „selbst
erledigt" zu unterscheiden.

**C — Geteilte Ablage**
Ein Ledger je Token statt je Linie. Löst das Problem an der Wurzel, kostet aber die
Eigenschaft, dass jedes Ledger genau die Akte **einer** Linie trägt.

## Was der Rat entscheiden soll

1. A, B oder C?
2. Darf `coverage` fremde Ledger lesen (A), oder bleibt jede Linie blind für die anderen?
3. Soll `register_lookup --addressed <linie>` erledigte Blöcke ausblenden? Heute listet es
   sie weiter (`--fired` bleibt 0), sodass Erledigtes wie Offenes liest — das ist die
   zweite Hälfte desselben Problems: die Sicht kennt den Zustand nicht.

## Was schon gebaut ist (nicht zurückgestellt)

`tools/register/src/bin/xfer.rs`: `ack` als Akt des Adressaten, `ACK` als Quittung
(eigener Zustand `ACKED`, nicht `CLOSED`), `coverage` trennt eigene Token von Quittungen
und zählt `acknowledged`. 13 Tests grün. Das ist additiv: `mark`/`fold`/`close`/`manifest`
verhalten sich unverändert. Der Rat kann A verwerfen und C bauen — dann fällt `ack` weg,
nichts anderes ändert sich.

## Riss, der bleibt

Ich kann die Sitze nicht fahren: `~/projects/omegaflow/.secrets.local` ist für mich
gesperrt (`permission.rejected`). Der Rat braucht den Operator.
