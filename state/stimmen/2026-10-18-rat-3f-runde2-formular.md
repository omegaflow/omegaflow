# Runde 2 — drei Befunde, drei Nachfragen (2026-10-18)

Du bist ein Ratssitz im Hause omegaflow (A = A; 0 honored: null-echt | absent | riss; keine harten Schwellen; PII bleibt im Haus).

Die drei offenen Messlücken der ersten Runde sind geschlossen. Unten steht je Frage der **gemessene Befund** und danach die **frühere Stimme** der vier Quellen (Schwarm A/B/C, Fugu, Lumo, DeepSeek). Deine Aufgabe: **neu abstimmen** — nicht die alte Stimme wiederholen, sondern sagen, ob der Befund sie ändert.

AUFTRAG: Je Frage (1) `Verdikt: <A|B|C|D>` (bei F1 zusätzlich `+ S1|S2`), (2) `geändert: ja/nein` gegenüber der früheren Stimme, (3) **zwei Sätze** Begründung, (4) der **stärkste Riss** (Gegenbeleg oder ungemessene Größe). Erfinde keine Fakten; Zahlen und Zitate nur aus den Befunden unten. Antwort auf Deutsch, maximal ~600 Wörter.

---

## F1 — Quittung im Token-Ledger

**Der gemessene Befund.** Der frühere `xfer ack`-Arm wurde in Commit `eb2f1b749` **architektonisch**, nicht technisch zurückgenommen. Die Commit-Nachricht wörtlich:

> `river: revert xfer ack — lines do not turn screws on the architecture; the two observations go to future instead (xfer .10~2e71595e .11~04ec4cc1)`

Entfernt wurden `Derived::Acked`/`ACKED`, `const RECEIPT = "ACK"`, `fn cmd_ack` (~78 Z), die Helfer (`belongs_to`, `addressee_of`, `ledger_lines`, `receipts_for`), der ACK-Zweig in `derive_state`, die `coverage`-ACK-Zählung und 3 Tests. Der **Vorgänger `0e096a89f` meldet „13 tests grün"** — der Arm **funktionierte**. Kein späterer Commit baut ihn wieder ein.

**Die Frage.** Der Schwarm-Riss der ersten Runde war: „warum scheiterte der ack-Arm?" — er scheiterte **nicht**. A ist technisch baubar; zurückgenommen wurde er per Haus-Regel („Linien drehen keine Architektur-Schrauben"), also durch einen Rat-Akt, nicht durch einen Code-Defekt. Ändert das die frühere Stimme?

**Frühere Stimmen.** Schwarm **A** · Fugu **B** · Lumo **C** · DeepSeek **pending + S2**.

**Optionen (unverändert).**
- **A** — eigener Akt `ACK <tok> by=<linie>.<sitz>` im Ledger des Empfängers; `ACKED` unterscheidbar von `CLOSED`.
- **B** — wie heute: Empfänger schreibt `CLOSED` ins Emitter-Ledger.
- **C** — kein neuer Akt: Quittungszustand read-only aus dem Token-Ledger ableiten.
- **D** — ein Ledger je Token statt je Linie (beide schreiben dieselbe Ablage).
- Zweitfrage `--addressed` (unabhängig): **S1** geschlossene Blöcke ausblenden · **S2** weiter listen, aber mit Zustandsspalte.

**Antwortform.**
`F1 Verdikt: <A|B|C|D|pending> [+ S1|S2] — geändert: <ja|nein> — Begründung: … (2 Sätze) — Riss: …`

---

## F2 — Concurrency-Schlüssel von `register-coverage`

**Der gemessene Befund.** `.github/workflows/register-coverage.yml` **schreibt gar nicht**. Es baut nur und ruft zwei **lesende** Prüfungen: Zeile 32 `register_lookup --orphans --fail`, Zeile 34 `register_lookup --tokens --noecho-fail`. `cancel-in-progress: false` steht in Zeile 20. **Kein `--write`.**

Der mid-write-Riss trifft also **nicht dieses Gate**, sondern die echten Register-Writer: `register_ttl_apply.rs:176`, `register_terms_apply.rs:161`, `register_block_remove.rs:107` — alle `fs::write` **in place** (truncate, kein temp+rename). Kill-Test (50 MB, `kill -9` nach 150 ms):

```
INPLACE  bytes=2097152  old_intact=no
ATOMIC   bytes=12       old_intact=yes  tmp=3145728
```

**Die Frage.** Für `register-coverage` ist das mid-write-Risiko **null**; die F2-Debatte (per-SHA vs. per-ref) wird davon **nicht** entschieden. Die Writer brauchen temp+rename — das ist ein eigener, belegter Punkt. Ändert das die frühere Stimme?

**Frühere Stimmen.** Schwarm **A** · Fugu **B** · Lumo **B** · DeepSeek **C**.

**Optionen (unverändert).** Soll `register-coverage.yml` von heute `group: …-${{ github.ref }}` mit `false` umgestellt werden auf:
- **A** — `group: register-coverage-${{ github.sha }}` + `cancel-in-progress: true` (per-SHA, wie `ci-gate` seit 2026-10-08).
- **B** — `group: …-${{ github.ref }}` + `cancel-in-progress: true` (per-ref; nur der jüngste Lauf lebt).
- **C** — Debounce/Batch statt Concurrency-Umbau.

**Antwortform.**
`F2 Verdikt: <A|B|C|D|pending> — geändert: <ja|nein> — Begründung: … (2 Sätze) — Riss: …`

---

## F3 — Register liest `state/stimmen` nicht

**Der gemessene Befund.** `tools/register/**` braucht **keine** RAT-ID. Hook-Test (`tools/gate/commit_msg.sh`): `tools/register/src/bin/register_lookup.rs` ohne RAT-ID → **exit 0**; `docs/concepts/x.md` ohne RAT-ID → **exit 1**. Das Architektur-Regex ist `^(tools/gate/|\.opencode/|AGENTS\.md$|opencode\.json$|docs/concepts/)` — `tools/register/**` fehlt. → **Option A ist ohne RAT-ID baubar.**

Zugleich: Ein `## An <linie>`-Block ist **521 B** (Beleg `2026-10-17-rat-offen-stand.md` → mycelium) bis **542 B** (Beleg `2026-10-18-rat-gate-rat-id.md` → future). Kappe `HANDOVER_CAP = 1024` (`commit_check.rs:138`): `1024 / 542 = 1,89` → **1 ganzer Block** je Handover; der zweite sprengt die Kappe. **7 von 9** lebenden Handovern liegen bereits über 1024 B (z. B. `mycelium-folge361` = 1140 B) — eingefroren, nicht konform.

**Die Frage.** Option B ist strukturell auf **einen** Block je Handover gedeckelt und trägt mehrere Verdikte nicht; Option A ist die einzige skalierbare — und braucht keine RAT-ID. Ändert das die frühere Stimme?

**Frühere Stimmen.** Schwarm **A** · Fugu **B** · Lumo **C** · DeepSeek **B**.

**Optionen (unverändert).**
- **A** — `state/stimmen/` als zusätzliche Dokumentwurzel im Leser aufnehmen.
- **B** — derive: Verdikte wandern als `## An <linie>`-Blöcke in das lebende Handover der Empfängerlinie.
- **C** — ein neues Register-Dokument `state/stimmen/_register.md` (nur Index) als eigene Quelle.
- **D** — status quo: die Leitstelle verteilt weiter von Hand.
- Zusatz: Muss die Wahl selbst als RAT-ID im Index stehen, bevor sie gebaut wird? (`ja|nein`)

**Antwortform.**
`F3 Verdikt: <A|B|C|D|pending> — geändert: <ja|nein> — Begründung: … (2 Sätze) — Riss: … — Zusatz: <ja|nein>, weil …`
