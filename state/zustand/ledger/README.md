# Ledger — die Beleg-Ablage der Linien-Token (Rat M2, Schritt A)

Append-only. `state/zustand/ledger/<line>/<session>.log`, eine Zeile je Ereignis:

```
<ISO-8601-Zeit> SENT   <token> by=<line.session> to=<line|self>
<ISO-8601-Zeit> FOLDED <token> by=<line.session> into=<line>
<ISO-8601-Zeit> CLOSED <token> by=<line.session> ref=<commit|path>
```

Leser: `register_lookup --tokens` (`ledger_disposed`/`ledger_sent_events`) und `bin/xfer`.
Ein `FOLDED`/`CLOSED`-Ereignis begleicht einen Token dauerhaft, auch wenn die Übergabe den
Punkt umformuliert. Der Ledger ersetzt keinen Commit-Witness: `--dropped` löst weiterhin nur
über einen Commit auf; diese Ablage trägt die Transport-Token.

Angelegt 2026-10-17 (Mycelium) auf dem Rat-Verdikt M2 (`4f50553f5`): erst A (Ledger +
Einzel-Auflösung `state/zustand/dropped-reconciliation.φ`), dann B (Baseline-Freeze).
