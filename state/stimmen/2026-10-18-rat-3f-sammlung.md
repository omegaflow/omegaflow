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

## Was als Nächstes gemessen werden muss

- **F1:** Warum wurde `xfer ack` in `eb2f1b749` zurückgenommen? (Commit + Diff lesen, nicht raten.)
- **F2:** Inkonsistenz-Rate bei `cancel mid-write` — ein Lauf, der während des Schreibens abgebrochen wird.
- **F3:** Ob B (Blöcke ins Handover) bei hoher Dichte unlesbar wird — und ob A ohne RAT-ID überhaupt zulässig ist.

## Rohmaterial

`2026-10-18-schwarm-44/` (Schwarm) · `2026-10-18-rat-3f-deepseek.txt` (paid) ·
`2026-10-18-rat-3f-fugu.txt` · `2026-10-18-rat-3f-lumo.txt` (UI)
