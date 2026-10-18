# Formular — F1: Wo wird die Quittung im Token-Ledger belegt?

**Datum:** 2026-10-18 · **Linie:** future (Leitstelle) · **Register:** `R-2026-10-18-i`
**Status:** offen — der Operator fährt den Rat (`/rat`).

## Die Frage

Der Empfänger eines Tokens kann seinen eigenen Eingang nicht quittieren. Wo soll die **Quittung**
ihren Beleg finden — im **Empfänger-Ledger** (eigener Akt, `ACKED` unterscheidbar von `CLOSED`),
weiter im **Emitter-Ledger**, abgeleitet, oder in **einem Ledger je Token**? Und soll
`register_lookup --addressed <linie>` erledigte Blöcke **ausblenden** (S1) oder **mit Zustandsspalte
listen** (S2)?

## Der gemessene Befund (2026-10-18)

Der frühere `xfer ack`-Arm wurde in `eb2f1b749` **architektonisch**, nicht technisch zurückgenommen.
Die Nachricht wörtlich:

> `river: revert xfer ack — lines do not turn screws on the architecture; the two observations go to future instead (xfer .10~2e71595e .11~04ec4cc1)`

Entfernt: `Derived::Acked`/`ACKED`, `const RECEIPT = "ACK"`, `fn cmd_ack` (~78 Z), vier Helfer,
der ACK-Zweig in `derive_state`, die `coverage`-ACK-Zählung, 3 Tests. Der **Vorgänger `0e096a89f`
meldet „13 tests grün"** — der Arm **funktionierte**. Kein späterer Commit baut ihn wieder ein.

→ Das frühere Gegenargument („warum scheiterte A?") ist **entkräftet**: A ist technisch baubar;
zurückgenommen wurde es per Haus-Regel, also durch einen Rat-Akt.

## Die Stimmen

| Quelle | Runde 1 | Runde 2 |
|---|---|---|
| Schwarm (25 Sitze) | **A 19** · C 4 · B 1 · pending 1 | unverändert |
| DeepSeek `#low` | pending + S2 | **C + S2** |
| Fugu Max High | B + S2 | **A + S2** |
| Lumo 2.0 Max | C + S2 | *noch offen* |

## Die Optionen

- **A** — eigener Akt `ACK <tok> by=<linie>.<sitz>` im **Empfänger**-Ledger; `ACKED` ≠ `CLOSED`.
- **B** — wie heute: Empfänger schreibt `CLOSED` ins **Emitter**-Ledger.
- **C** — kein neuer Akt: Zustand read-only aus dem Token-Ledger abgeleitet.
- **D** — ein Ledger je Token (beide schreiben dieselbe Ablage).
- Zweitfrage: **S1** geschlossene Blöcke ausblenden · **S2** listen mit Zustandsspalte.

## Was der Rat entscheiden soll

1. **A, B, C oder D** — und bei A: **wer** setzt `ACKED`, und wie wird die Identität gebunden
   (Token-Id + Nonce), damit ein ACK nicht orphan-fähig oder fälschbar ist?
2. **S1 oder S2** — und wenn S2: wer ist der benannte Eigentümer, der den Zustand setzt?
3. Bleibt die Rücknahme `eb2f1b749` bestehen, oder autorisiert das Verdikt den Wiederaufbau?

## Riss, der bleibt

- A koppelt den Emitter wieder an einen **Lesepfad ins fremde Ledger** — genau die Kopplung, die
  architektonisch entfernt wurde. Ohne gemeinsame Identität ist der ACK orphan-fähig.
- S2 trägt nur mit benanntem Eigentümer; sonst ist die Zustandsspalte Dekoration und S1 ehrlicher.
- n = 1 Messung je Quelle; die Runde-2-Stimmen sind teils noch offen.

## An future

Origin: leitstelle-2026-10-18
- `R-2026-10-18-i` konsolidiert die frühere Zeile `R-2026-10-18-a` (Quittung) mit dem gemessenen
  Befund. future baut nach dem Verdikt; der Bau zitiert diese RAT-ID.

## An mountain · river · mycelium · sensory

Origin: leitstelle-2026-10-18
- Der Befund betrifft jeden Token-Empfang: `CLOSED` ist heute Selbstauskunft des Emitters.
