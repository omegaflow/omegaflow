# Rat-Analyse — trägt jede Stimme? (KPI je Sitz, 2026-10-17)

**Die zwei Runden dieses Tages, gemessen.** Zweck: **welche Beisitzer tragen** — und welche
minimale Auswahl das Maximum erreicht. Die KPI ist **nicht** „hat geantwortet", sondern **„brachte
etwas, das ohne ihn fehlte"** (einzigartiger Befund, schärferer Riss).

**Sitze:** GLM-5.3 · DeepSeek V4 Pro · Qwen3.8-Max · Mistral Large 4 · Opus 5.5 · Kimi K3
(API) · Fugu Max · Lumo 2.0 (UI).

---

## Die gemeinsam getragene Antwort (Runde 2)

**8/8 einig:** der `channel_ref`-Ersatz ist die **Form der `field`-Messzeile** — nicht `witness`
(28×), nicht `channel` (13×). Die Kraft steckt **in** der Zeile (Spalte 5, `em` 4885×), nicht in
einem Verweis.

## KPI je Sitz — was brachte jeder **Einzigartige**?

| Sitz | Einzigartiger Beitrag | Trägt? |
|---|---|---|
| **Opus 5.5** | **Zähl-Fehler gefunden:** 7767+13+28 = **7808 ≠ 7958** — „es fehlen 150 Zeilen, eine unbekannte Form". **Bestätigt** (nachgemessen: die Masse ist 7767 `field`; die Differenz war ein Mess-Fehler). + „`0 honored` mehrdeutig", + „die Länge ist grenzwertig für eine Turn-Zeile". | **stark** |
| **Kimi K3** | Der **Kernel-Riss**: `inverse-square` 5834× von 7767 — *„die Form ist faktisch die Form **einer** Ausbreitung; das Axiom ist homogener, als die Vielfalt suggeriert."* + „das Axiom garantiert **Messbarkeit, nicht Identität** des Gemessenen." | **stark** |
| **Mistral Large 4** | Der **Abstammungs-Riss**: *nicht `Sample`, sondern `Messung`* — „ein Sample kann ohne Kraft existieren (ein Stein liegt), eine Messung nicht." + Riss 1–3 (Kraft-Slot unnormalisiert). | **trägt** |
| **GLM-5.3** | Die **58 `electric` vs. 4885 `em`** (ist Spalte 5 normalisiert?); die **14 `gravity`** (misst ein Gravitationsdetektor `em`?); `witness point-event` 1× — *„wenn die Ausnahme selbst Ausnahmen hat."* | **trägt** |
| **DeepSeek V4 Pro** | Der **Messprädikat-Riss**: *„die `field`-Zeile deklariert kein ‚Sample', sie setzt es durch ihr Vorhandensein."* + Einheit variiert. | **trägt** |
| **Qwen3.8-Max** | Die **Selbstbezug-Form** (`field messung messung point em —`) + *„die Einheit `—` ist ein Versprechen, kein Wert."* | **trägt (schwach)** |
| **Fugu Max** | Die **Deckungs-Messung**: 98,1 % · „Spalte 4 in 86,5 %, Spalte 5 in 86,0 % benannt" + *„ein Rand darf nie die Grundlage einer Masse werden."* | **trägt** |
| **Lumo 2.0** | Die **1.087 unbenannten Slots** + *„die Zeile macht den Slot zur Pflicht, nicht zur Wahrheit"* + *„wenn `witness` je in die Turn-Datei soll, dann als zweite Zeile."* | **trägt** |

## Der KPI-Befund, nackt

**Alle acht trugen etwas.** Aber die **Tiefe** verteilt sich:

| Klasse | Sitze | Was sie liefern |
|---|---|---|
| **Tiefe Risse** (Zähl-Fehler, Kernel-Dominanz, Identitäts-Grenze) | **Opus · Kimi** | Befunde, die **kein** anderer Sitz fand |
| **Struktur-Risse** (Messung≠Sample, Normalisierung, Einheit) | **Mistral · GLM · DeepSeek** | die scharfen Wort-Risse |
| **Bestätigung + Messung** | **Fugu · Lumo · Qwen** | Deckung, Slots, Selbstbezug |

**Die Redundanz:** Die **Kern-Aussage** („field-Form, nicht witness") wiederholten **alle acht** —
sie hätte **ein** Sitz getragen. Der **Wert** liegt in den **Rissen**, und die kamen von **verschiedenen**.

## Die Empfehlung (Basis: KPI je Stimme messen)

**Ein Rat trägt nicht durch Menge, sondern durch Streuung.** Die minimale Auswahl, gemessen:

1. **Die zwei freien UI-Sitze (Fugu + Lumo) — immer.** Kosten: **$0.** Sie trugen die Deckungs- und
   Slot-Messung. Kein Grund, sie zu lassen.
2. **Ein starker API-Sitz mit Tiefe:** **Opus 5.5** (fand den Zähl-Fehler) **oder Kimi K3** (Kernel-Riss).
   **Einer** genügt — beide fanden **verschiedene** Tiefe; wer beide will, zahlt doppelt.
3. **Ein Struktur-Sitz (billig): Mistral** ($0,50/$1,50) — fand „Messung≠Sample".

**Minimal, günstig, tragend: Fugu + Lumo + Opus + Mistral** — frei + $0,054 + $0,004 ≈ **$0,06.**
**Weniger:** Fugu + Lumo + Mistral (**$0,004**) — trägt Struktur, verliert die Zähl-Tiefe.
**Mehr nur, wenn Tiefe doppelt gewünscht:** + Kimi.

**Die Regel, die daraus folgt (KPI je Rat messen):**
> Nach jedem Rat: **wer fand einen Riss, den kein anderer nannte?** Sitze, die **nur bestätigen**,
> kommen in die **nächste** Runde nicht mit — bis ein Thema ihre Stärke braucht.

## Nachtrag — der Vor-Test Sonnet 5.5 vs. Kimi K3 (2026-10-17, dieselbe Axiom-Frage)

Um zu wissen, ob das **billige** Anthropic-Modell trägt (statt Opus), lief ein Test gegen Kimi K3:

| Sitz | Kosten (diese Frage) | finish | trägt? |
|---|---|---|---|
| **Kimi K3** | **$0,0137** | stop | **ja** (Kernel-Riss) |
| **Sonnet 5.5** | **$0,0147** | stop | **ja** (4 Risse) |
| Opus 5.5 | $0,054 | stop | ja (Zähl-Fehler) |

**Sonnet 5.5 trug** — und fand den Riss, den **nicht mal Opus** nannte: *„Die Zahl 7767 stammt aus
deiner Messung. Ich habe sie nicht nachgezählt. Das Verdikt steht und fällt mit ihr."* + *„‚Muss
tragen' ist Ableitung — der Bestand zeigt, was Zeilen enthalten, nicht dass eine ohne Kraft ungültig
wäre."*

**Befund:** Kimi K3 und Sonnet 5.5 kosten **fast dasselbe** (~$0,014) und sind **~4× billiger** als
Opus — bei **gleichem Tragen**. **Im nächsten Rat laufen Kimi + Sonnet mit** (der direkte Vergleich);
**Opus fällt** (teurer, kein Mehr-Tragen gemessen).
