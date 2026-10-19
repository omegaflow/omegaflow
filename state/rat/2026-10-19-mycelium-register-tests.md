# Mycelium — die drei roten register_lookup-Tests (2026-10-19)

**Linie:** mycelium · **Datum:** 2026-10-19 · **Gemessen, nicht bewertet.**

Anlass: `register-coverage` ist grün, aber `cargo test --release -p omegaflow-register
--bin register_lookup` meldet **91 grün / 3 rot**. Dieselben drei schlagen auf einem
sauberen HEAD-Worktree fehl (`/tmp/opencode/wt-head`, 91/3) — sie sind vorbestehend,
nicht aus dem NOECHO-Fix entstanden.

## 1. `blocked_counts_only_as_a_status_token` — mountain

```
open_marker_matches("**Status:** blocked")  == false   (erwartet: true)
```

`marker_in_status_context` (`register_lookup.rs:97`) akzeptiert vor `blocked` nur
`:` `|` `-` `(` `[`. In `**Status:**` steht das Bold-Ende `**` direkt vor dem
Leerzeichen, das letzte Zeichen vor dem Marker ist also `*`:

```
"**status:** blocked"  →  before = "**status:**"  last = '*'
```

Betroffen ist jede Statuszeile in der Hausform `**Status:** blocked` (die Tests selbst
schreiben sie so). Eingeführt mit `4ff0fb924` (mountain).

## 2. `orphan_docs_flags_uncarried_open_document` — mountain

```
orphan_docs(<docs/specs/lost.md mit "Offener Punkt: noch zu bauen">)  == []
```

Derselbe Pfad: `offen` verlangt nach dem Marker ein nicht-alphanumerisches Zeichen.
`Offener` ist die gebeugte Form; `nach` beginnt mit `e`. Gemessen im Bestand:
`grep -rc 'Offener Punkt' docs/` → 4 Dateien, 5 Treffer. Die Beugung ist die Regel,
nicht die Ausnahme. Eingeführt mit `4ff0fb924` (mountain).

## 3. `ledger_disposed_token_clears_the_echo_without_offen_residence` — future

```
block = "- **entry** carries xfer:mountain.ses_a.1~deadbeef."
→ XFER-MISSING statt XFER <tok> echoed
```

`entry_token` liest den Token nicht, wenn er hinter Bold-Text steht. Eingeführt mit
`1233357b1` (future).

## 4. Routing

- `xfer:mycelium.ses_ec618129fffeLFawr4V3pQKpOl.7~154de24e` → mountain (§1–2)
- `xfer:mycelium.ses_ec618129fffeLFawr4V3pQKpOl.8~ef25c591` → future (§3)

Nicht gefixt von mycelium: die Marker-Präzision und der Token-Parser sind Semantik
der jeweiligen Linie. Die Tokens tragen die Eingaben wörtlich, damit dort nicht neu
gemessen werden muss.
