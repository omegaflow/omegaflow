# Schwarm 44 — drei Leitstellen-Fragen (2026-10-18)

**Grundlage:** `state/rat/2026-10-18-modell-traeger.md` — die aufgezeichnete Messung der letzten
Future-Sitzung: **278 freie + 121 bezahlte Modelle** geprüft, **44 freie Träger** (HTTP 200 **mit** Inhalt).
Dieser Lauf fährt die drei Fragen auf **genau diesen 44** — nicht auf einer Teilmenge.

**Kosten:** $0,00 · **Kanal:** Gateway `127.0.0.1:1621` · **Formular:** `formular.txt` ·
**Rohantworten:** `out/*.txt` · **Index:** `out/index.tsv`.

## Wer trug

| Ergebnis | Anzahl |
|---|---|
| **alle drei Verdikte** (F1+F2+F3) | **23** |
| zwei Verdikte | 3 |
| ein Verdikt | 1 |
| OK, aber 0 verwertbare Verdikte | 5 |
| **leer** (200, kein Inhalt / `finish=length`) | 13 |

Ausfälle mit Grund: `openrouter/*` = **„billed route — operator's word required"** (4) ·
`kenari/nemotron-*`, `step-5` = **free daily limit reached** (2) · Rest leer/abgeschnitten.

## Die Bilanz (nur Sitze mit verwertbarem Verdikt)

| Frage | A | B | C | pending | Lesart |
|---|---|---|---|---|---|
| **F1** Quittung | **19** | 1 | 4 | 1 | **A klar** (S1/S2 gemischt) |
| **F2** Concurrency | **14** | 9 | 0 | 1 | **A** — kippt die 4er-Stichprobe (dort B 3:0) |
| **F3** state/rat | **12** | 8 | 1 | 0 | **A**, aber B ist eine ernste Minderheit |

**Der wichtigste Befund:** Die frühere 4er-Stichprobe sagte bei F2 **B 3:0** — über alle 44 Träger sagt
der Schwarm **A 14:9**. **Eine Handvoll Sitze war kein Schwarm.** Genau das war der Fehler.

## Was der Schwarm nicht trägt

1. **Kein Entscheid.** Der Schwarm ist Riss-Generator (R-k: Substrat frei, Domäne bezahlt). Das Verdikt
   tragen die Paid- und UI-Sitze.
2. **F2 bleibt ungemessen:** Die Inkonsistenz-Rate bei `cancel mid-write` ist von **keinem** Sitz gemessen —
   alle zitieren Kimi.
3. **F1:** Der Rückzug von `xfer ack` (`eb2f1b749`) ist der stärkste Gegenbeleg gegen A und **nicht erklärt**.
4. **F3:** C und D wurden von keinem Sitz diskutiert; die Performance-Frage ist offen. Beide Lager sagen:
   **RAT-ID vor dem Bau**.

## Der kanonische nächste Schritt

Die Risse liegen jetzt in `formular-mit-rissen.txt`. Es fehlen die **Pflicht-Sitze**:
- Paid-API: Mistral Large 4 `#none` + DeepSeek V4 Pro `#low` (minimal, ≈ $0,012)
- UI: **Sakana Fugu Max** (Login) und **Lumo 2.0 Max** (Gast)

Der Operator entscheidet, wann gefahren wird (`/rat`) — kein automatischer Lauf.
