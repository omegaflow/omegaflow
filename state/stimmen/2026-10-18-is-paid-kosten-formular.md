# Formular — `is_paid` nach Kosten, nicht nach Provider

**Datum:** 2026-10-18 · **Linie:** future (Leitstelle) · **Register:** `R-2026-10-18-h`
**Status:** offen — der Operator fährt den Rat (`/rat`, `_rat-anweisung.md`).

## Die Frage

Soll der Gateway ein Modell als **bezahlt** behandeln, wenn **dieses Modell** Geld kostet —
oder wenn sein **Anbieter** auch bezahlte Modelle führt?

Heute gilt das Zweite. Folge: `openrouter/…:free` (Katalogpreis **0/0**) wird als
„billed route" mit **402** geblockt, obwohl der Aufruf **nichts** kostet.

## Der gemessene Defekt (2026-10-18)

Quelle: `/home/johannes/projects/omegaflow-crambler-backup/source/llm_gateway.rs`
(die einzige Quelle; im Live-Repo fehlt sie — folge260 Punkt 3).

```rust
fn is_paid(prefix: &str) -> bool {                    // :1644
    matches!(prefix, "deepinfra" | "deepseek" | "mistral" | "sambanova"
             | "hf" | "google" | "zen" | "inception" | "unorouter"
             | "orcarouter" | "nim" | "openrouter")   // :1658
}
...
if is_paid(route.prefix) && !paid_allowed(route.prefix) {   // :1725
    last_code = 402;   // "billed route"
}
```

Der Modellname ist zu diesem Zeitpunkt **bekannt** (`:1700`), wird aber **nicht** geprüft.
Der Guard fragt „könnte dieser Anbieter Geld nehmen?" statt „nimmt er für **dieses** Modell Geld?".

**Gemessene Folge (Schwarm-Lauf 2026-10-18, `2026-10-18-schwarm-44/out/index.tsv`):**

| Modell (Katalogpreis 0/0) | Antwort des Gateways |
|---|---|
| `openrouter/nvidia/nemotron-3-nano-omni-30b-a3b-reasoning:free` | 402 „billed route" |
| `openrouter/nvidia/nemotron-3-ultra-550b-a55b:free` | 402 „billed route" |
| `openrouter/poolside/laguna-s-2.1:free` | 402 „billed route„ |
| `openrouter/xingchen-agi/xing4.0-29b-a4b:free` | 402 „billed route" |

**Belegt frei:** `models.dev` weist für alle `:free`-Modelle `cost = 0/0` aus
(`2026-10-18-modell-faehigkeiten.md:5`). `post/paid-open` nennt heute
`mistral`, `deepseek`, `deepinfra` — `openrouter` fehlt, also blockt der Prefix.

## Der Kategorienfehler

> **„theoretisch bezahlbar" ≠ „wird bezahlt".**

Ein Provider kann gleichzeitig kostenlose und kostenpflichtige Modelle führen. Die
Bezahl-Schwelle an den **Provider** zu hängen bestraft das kostenlose Modell für die
Existenz des bezahlten Nachbarn. Derselbe Fehler wie im Rat R-k beschrieben:
**die Adresse (Provider) wird mit der Sache (Preis) verwechselt.**

## Die Optionen

**A — kostenbasiert (die klare Regel).**
`is_paid` prüft den **Preis des gewählten Modells**: `cost.input/output > 0` oder
fehlender Preis → Operator-Wort. `:free` / `0/0` → frei, durchlassen, unabhängig vom Provider.
Kosten: eine Bedingung im Guard; keine neue Ablage.

**B — providerbasiert (status quo).**
Jeder Modell-Prefix in der Liste ist bezahlt. `openrouter` bleibt gesperrt, auch für `:free`.
Kosten: keine Änderung; die vier freien Modelle bleiben unerreichbar.

**C — hybrid, aber explizit.**
Ein Modell ist frei, wenn **Provider frei** *oder* **Modellpreis 0**. Bezahlt nur, wenn
Preis > 0 **und** Provider nicht frei. Gleiche Wirkung wie A, aber zwei Bedingungen.

**D — Allowlist je Modell statt je Route.**
`paid-open` nennt **Modell-IDs**, nicht Provider. Wer ein bezahltes Modell will, nennt es
namentlich; alles andere bleibt frei. Kosten: `paid_allowed` muss Modell statt Prefix matchen.

## Was der Rat entscheiden soll

1. **A, B, C oder D** — wird „bezahlt" am **Preis des Modells** oder am **Provider** gemessen?
2. Falls A/C: Woher kommt die Preis-Wahrheit — der **statische Katalog** (`models.dev`),
   der **`:free`-Suffix**, oder die **Anbieterantwort**? Was gilt, wenn der Katalog fehlt?
3. Soll `paid-open` weiter **Routen** nennen (öffnet alle Modelle der Route) oder **Modell-IDs**
   (öffnet genau das genannte Modell)?
4. Gilt die Regel auch für **`unorouter`** (tot, 70× HTTP 000) und die übrigen Prefixe —
   oder nur für `openrouter`?

## Riss, der bleibt

- Die Gateway-Quelle liegt **nur im Backup** (`omegaflow-crambler-backup`), nicht im Live-Repo.
  Jeder Bau muss sie zuerst zurückholen — und der laufende Gateway bedient die Sitzung;
  ein Rebuild/Restart kappt die eigene Leitung. Der Bau gehört in einen eigenen Pass.
- Ein `:free`-Suffix ist eine **Behauptung des Anbieters**, kein gemessener Preis. A und C
  brauchen eine Entscheidung, ob der Suffix als Preis-Wahrheit zählt oder der Katalog.
- `paid-open` als Route-Allowlist ist heute **gröber** als die Frage: `openrouter` freizugeben
  öffnet auch die bezahlten openrouter-Modelle. D ist die feinere, aber teurere Bauform.

## An future

Origin: leitstelle-2026-10-18
- `R-2026-10-18-h` — Formular aus dem gemessenen Guard-Defekt (`llm_gateway.rs:1644/1725`)
  und dem blockierten Schwarm-Lauf. future baut nach dem Verdikt; der Bau braucht diese RAT-ID.

## An mountain · river · mycelium · sensory

Origin: leitstelle-2026-10-18
- Der Defekt betrifft **jeden** Gateway-Aufruf: freie Modelle auf bezahlfähigen Routen werden
  als „billed" geblockt. Bis zum Verdikt gilt der Workaround: die Route in `post/paid-open`
  nennen — bewusst und mit dem Wissen, dass er die ganze Route öffnet.
