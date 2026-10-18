# Rat — R-2026-10-18-h: `is_paid` nach Kosten, nicht Provider (2026-10-18)

**Frage:** Soll der Gateway ein Modell als bezahlt behandeln, wenn **dieses Modell** Geld kostet —
oder wenn sein **Anbieter** auch bezahlte Modelle führt?
**Formular:** `2026-10-18-is-paid-kosten-formular.md` · **Index:** `_rat-register.md`

## Die Bilanz — vier Quellen, ein Verdikt

| Quelle | Verdikt | Beleg |
|---|---|---|
| **Freier Schwarm** (44 Träger, 22 verwertbar) | **A 16 : C 6 : B 1** | `2026-10-18-rat-h-schwarm/out/index.tsv` |
| **DeepSeek V4 Pro `#low`** (paid) | **A** | `2026-10-18-rat-h-deepseek.txt` (1193 out-tok, stop) |
| **Fugu Max High** (UI) | **A** | `2026-10-18-rat-h-fugu.txt` |
| **Lumo 2.0 Max** (UI) | **A** | `2026-10-18-rat-h-lumo.txt` |

**Verdikt: A — kostenbasiert.** Alle vier Quellen stimmen überein; C ist die einzige Minderheit
im Schwarm (gleiche Wirkung, zwei Bedingungen).

## Die Regel

> Bezahlt ist ein Modell genau dann, wenn **sein konkreter Preis > 0** ist.
> `:free` / Katalogpreis `0/0` → frei, unabhängig vom Anbieter-Prefix.
> `paid-open` nennt **Modell-IDs**, nicht Routen.

## Die Risse (ungeglättet)

1. **Preis-Wahrheit (Fugu, Lumo):** Der Katalogpreis sagt, was der Anbieter *verspricht*, nicht was
   er *fakturiert*. Fugu: Katalog als Vorentscheid, **gemessene Abrechnung als Korrektiv** — ein
   Betrag > 0 im Ledger überschreibt den Katalog. Das `:free`-Suffix ist **nie** Beweis.
2. **Latency (Lumo):** A braucht Preisabfragen beim Routing oder einen Cache.
3. **`paid-open` als Route ist zu grob:** Wer `openrouter` freigibt, öffnet auch die bezahlten
   Modelle. Beide UI-Sitze: **Modell-IDs** nennen.
4. **Alle Prefixe (Fugu):** Das Prinzip gilt ausnahmslos, auch für `unorouter`; ein fehlender Preis
   darf **keine** pauschale Paid-Annahme begründen (harte Schwelle ohne Messung).

## Was der Bau braucht

- Quelle: `omegaflow-crambler-backup/source/llm_gateway.rs` (nur dort vorhanden).
- Änderung: `is_paid` am **Modellpreis** entscheiden, nicht am Prefix; `paid-open` auf Modell-IDs.
- Der laufende Gateway bedient die Sitzung → Rebuild/Restart in einem eigenen Pass.
- Bau zitiert `R-2026-10-18-h`.

## Rohmaterial

`2026-10-18-rat-h-deepseek.txt` · `2026-10-18-rat-h-fugu.txt` · `2026-10-18-rat-h-lumo.txt` ·
`2026-10-18-rat-h-schwarm/` (Prompt, Index, 44 Rohantworten)
