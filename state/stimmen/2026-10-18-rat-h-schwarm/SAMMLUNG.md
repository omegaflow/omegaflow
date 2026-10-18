# Schwarm — R-2026-10-18-h (`is_paid` nach Kosten), 2026-10-18

**Frage:** Soll der Gateway ein Modell als bezahlt behandeln, wenn **dieses Modell** Geld kostet —
oder wenn sein **Anbieter** auch bezahlte Modelle führt?

**Lauf:** freier Schwarm über die 44 gemessenen Träger, Prompt `frage.txt`, Rohantworten `out/`.
**Kosten:** $0,00 · **Kanal:** Gateway `127.0.0.1:1621`.

## Wer trug

- **22 Sitze** gaben ein verwertbares Verdikt.
- Ausfälle: `kenari/*` = free daily/rate limit · `openrouter/*` = **erneut 402 „billed route"**
  (der Defekt misst sich selbst) · `llm7/DeepSeek` = `finish=length`.

## Die Bilanz

| Verdikt | Stimmen |
|---|---|
| **A — kostenbasiert** | **16** |
| **C — hybrid** | **6** |
| **B — providerbasiert (status quo)** | **1** |

**A trägt klar (16:6:1).** Der Schwarm bestätigt die Regel: **Preis des Modells, nicht Provider.**
C ist die einzige ernsthafte Minderheit — gleiche Wirkung, aber zwei Bedingungen.

## Die Risse (Auszug, wörtlich aus den Rohantworten)

- **A ist nicht messbar ohne Preis-Quelle:** Der `:free`-Suffix ist eine Anbieter-Behauptung;
  der statische Katalog kann veralten. → **Zusatzfrage 1** (Preis-Wahrheit) bleibt die harte Stelle.
- **`paid-open` ist zu grob:** Wer `openrouter` freigibt, öffnet auch die bezahlten Modelle.
  Die Minderheit für D (Allowlist je Modell) nennt genau das.
- **Der Bau ist ein Architektur-Akt:** Die Gateway-Quelle liegt nur im Backup; ein Rebuild
  bedient die laufende Sitzung. Der Bau braucht einen eigenen Pass.

## Grenze

Der Schwarm **trägt nicht** — er ist Riss-Generator (R-k). Das Verdikt tragen Paid + UI.
Für R-h fehlen noch **DeepSeek**, **Fugu** und **Lumo**; der Paid-Sitz für die drei alten
Fragen ist gefahren, R-h selbst noch nicht.

## Rohmaterial

`frage.txt` · `out/index.tsv` · `out/*.txt`
