# Sammlung — drei Leitstellen-Fragen (F1/F2/F3), 2026-10-18

**Vier Quellen:** freier Schwarm (44 gemessene Träger, $0) · DeepSeek V4 Pro `#low` (paid) ·
Fugu Max High (UI) · Lumo 2.0 Max (UI).

**Paid-Lage (gemessen 2026-10-18):**
- **Mistral Large 4: blockiert** — `{"detail":"Your API key expired on 2026-10-17."}`
- **DeepInfra: blockiert** — `User is not authorized to access this resource`
- **DeepSeek V4 Pro `#low`: gefahren** (`deepseek/deepseek-v4-pro`, finish=stop, 3325 out-tok)
→ nur **ein** Paid-Sitz statt der geplanten zwei.

## Die Bilanz je Frage

### F1 — Quittung im Token-Ledger

| Quelle | Verdikt |
|---|---|
| Schwarm (25 Sitze) | **A 19** · C 4 · B 1 · pending 1 |
| DeepSeek (paid) | **pending + S2** — A ist der einzige belegte Weg zu ACKED, aber der zurückgenommene ack-Arm ist ungeklärt; D ungemessen |
| Fugu (UI) | **B + S2** — der Empfänger kann nicht quittieren, also darf der ACK-Akt nicht bei ihm liegen |
| Lumo (UI) | **C + S2** — read-only ableiten, weil es keinen ACK-Akt gibt |

**Kein Konsens.** Der Schwarm sagt A, beide UI-Sitze sagen **nicht A**, der Paid-Sitz sagt `pending`.
Die UI-Sitze begründen ihre Ablehnung mit demselben Beleg, den der Schwarm als Riss führt
(`eb2f1b749`). **F1 bleibt offen** — es fehlt die Messung, *warum* der ack-Arm scheiterte.

### F2 — `register-coverage`-Concurrency

| Quelle | Verdikt |
|---|---|
| Schwarm (24 Sitze) | **A 14** · B 9 · pending 1 |
| DeepSeek (paid) | **C** — A und B teilen das mid-write-Risiko; Debounce vermeidet Überlappung |
| Fugu (UI) | **B** — per-SHA erzeugt mehr Läufe als refs; C ist eine harte Zeitschwelle |
| Lumo (UI) | **B** — per-ref bündelt dieselben Ereignisse |

**Kein Konsens.** Schwarm A, Paid C, beide UI B. **F2 bleibt offen.**
Die eine ungemessene Größe, die alles entscheidet, ist die **Inkonsistenz-Rate bei `cancel mid-write`** —
alle vier Lager zitieren Kimi, keines misst.

### F3 — Register liest `state/stimmen` nicht

| Quelle | Verdikt |
|---|---|
| Schwarm (21 Sitze) | **A 12** · B 8 · C 1 |
| DeepSeek (paid) | **B** — respektiert „Linien ändern keine Architektur"; RAT-ID-Zirkularität vermieden |
| Fugu (UI) | **B** — nur ein Pfad, den `register_lookup` schon adressiert |
| Lumo (UI) | **C** — eigener Index als kleinste stabile Adresse |

**Kein Konsens.** Schwarm A, Paid+beide UI gegen A (B, B, C). **F3 bleibt offen** — A wäre ein
Reader-/Architektur-Akt und braucht die RAT-ID vor dem Bau.

## Der Befund

1. **Der Schwarm trägt nicht** — genau wie R-k sagt. Er zeigt bei allen drei Fragen nach A;
   die bezahlten/UI-Sitze wählen B, C oder `pending`.
2. **Die UI-Sitze sind nicht redundant.** In F1 und F2 kippen sie die Schwarm-Mehrheit.
3. **Kein Verdikt ist entschieden.** Drei offene Fragen, vier Quellen, kein Konsens.
   Das ist kein Fehler des Laufs — es ist das Ergebnis.
4. **Der Paid-Rat ist unterbesetzt:** von zwei geplanten Sitzen fiel einer aus (Mistral-Key
   abgelaufen, DeepInfra nicht autorisiert). Der eine Sitz trägt keine zweite Stimme.

## Die drei Messlücken — geschlossen (2026-10-18)

### F1 — der `ack`-Arm fiel **architektonisch**, nicht technisch
Commit `eb2f1b749`, Nachricht wörtlich:
> `river: revert xfer ack — lines do not turn screws on the architecture; the two observations go to future instead (xfer .10~2e71595e .11~04ec4cc1)`

Entfernt: `Derived::Acked`/`ACKED`, `const RECEIPT = "ACK"`, `fn cmd_ack` (~78 Z), Helfer
(`belongs_to`, `addressee_of`, `ledger_lines`, `receipts_for`), der ACK-Zweig in `derive_state`,
`coverage`-ACK-Zählung, 3 Tests. Der Vorgänger `0e096a89f` meldet **„13 tests grün"** — der Arm
**funktionierte**. Kein späterer Commit baut ihn wieder ein.

**Folge:** Der Schwarm-Riss („warum scheiterte er?") ist beantwortet: **er scheiterte nicht.**
A ist technisch baubar; die **Haus-Regel** („Linien drehen keine Architektur-Schrauben") war der
Grund. Damit fällt das stärkste Gegenargument gegen A weg — und A bleibt ein Rat-Akt, kein Linien-Bau.

### F2 — `register-coverage` **schreibt gar nicht**
`.github/workflows/register-coverage.yml` baut nur und ruft zwei **lesende** Prüfungen:
`:32 register_lookup --orphans --fail`, `:34 register_lookup --tokens --noecho-fail`.
`cancel-in-progress: false` (`:20`). **Kein `--write`.**

Der mid-write-Riss trifft also **nicht dieses Gate**, sondern die echten Register-Writer:
`register_ttl_apply.rs:176`, `register_terms_apply.rs:161`, `register_block_remove.rs:107` —
alle `fs::write` **in place** (truncate, kein temp+rename).

Kill-Test (50 MB, `kill -9` nach 150 ms):
```
INPLACE  bytes=2097152  old_intact=no
ATOMIC   bytes=12       old_intact=yes  tmp=3145728
```

**Folge:** Für `register-coverage` ist das mid-write-Risiko **null**; die F2-Debatte (per-SHA vs
per-ref) wird davon nicht entschieden. Die Writer **brauchen** temp+rename — das ist ein eigener,
belegter Punkt.

### F3 — A braucht **keine** RAT-ID; B passt nur **einmal**
Hook-Test (`tools/gate/commit_msg.sh`): `tools/register/src/bin/register_lookup.rs` ohne RAT-ID →
**exit 0**; `docs/concepts/x.md` ohne RAT-ID → **exit 1**. Das Architektur-Regex ist
`^(tools/gate/|\.opencode/|AGENTS\.md$|opencode\.json$|docs/concepts/)` — `tools/register/**` fehlt.
→ **Option A ist ohne RAT-ID baubar.**

Blockgröße `## An <linie>`: **521 B** (`2026-10-17-rat-offen-stand.md` → mycelium) und **542 B**
(`2026-10-18-rat-gate-rat-id.md` → future). Kappe `HANDOVER_CAP = 1024` (`commit_check.rs:138`):
`1024 / 542 = 1,89` → **1 ganzer Block** pro Handover; der zweite sprengt die Kappe.
**7 von 9** lebenden Handovern liegen bereits über 1024 B (z. B. `mycelium-folge361` = 1140 B) —
eingefroren, nicht konform.

**Folge:** Option B ist strukturell auf **einen** Block je Handover gedeckelt und trägt mehrere
Verdikte nicht. Option A ist die einzige skalierbare — und braucht keine RAT-ID.

## Was das für den Rat heißt

Die drei „offenen Messungen" sind keine Messungen mehr; sie sind **Befunde**:

1. **F1:** A ist baubar und wurde nur per Haus-Regel zurückgenommen → das Gegenargument ist entkräftet.
2. **F2:** mid-write ist für `register-coverage` irrelevant (read-only); die Writer brauchen temp+rename.
3. **F3:** A ist ohne RAT-ID baubar; B ist auf 1 Block/Handover gedeckelt.

Damit ist der Weg für eine **zweite, kleinere Runde** frei — oder für ein Operator-Wort. Die
Verdikte selbst bleiben beim Rat; die Leitstelle hat nur die Belege geliefert.

## Rohmaterial

`2026-10-18-schwarm-44/` (Schwarm) · `2026-10-18-rat-3f-deepseek.txt` (paid) ·
`2026-10-18-rat-3f-fugu.txt` · `2026-10-18-rat-3f-lumo.txt` (UI)
