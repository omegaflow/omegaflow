# Netto-Rollen — Grenzkosten statt Listenpreis (korrigiert 2026-10-18)

**Korrektur.** Die erste Fassung verwechselte **DeepInfra** und **DeepSeek** und baute daraus eine
falsche Rangfolge. Beide kosten echtes Geld. Die vom Operator vorgegebene Leiter gilt:

> **free > Mistral inklusive > DeepSeek paid > Mistral paid**
> **DeepInfra nur für (a) PII, wo Mistral nicht ausreicht, und (b) echte Frontier-Probleme (Kimi-Klasse).**
> **PII läuft zuerst über Mistral** (Vertragsdeckung); DeepInfra erst, wenn Mistral die Aufgabe nicht trägt.

## 1 — Die Töpfe (Operator-Angaben, 2026-10-18)

| Topf | Stand | Charakter | Grenzkosten/Call |
|---|---|---|---|
| **OpenCode frei** (`opencode/*-free`) | 10 Modelle, `cost 0/0` | frei | **$0** |
| **Freie Träger** (Gateway) | 44 gemessen | frei | **$0** |
| **Mistral inklusive** | **7,27 €** Rest von 8,50 € | Abo, monatlich | **€0** bis erschöpft |
| **DeepSeek paid** | **$128,86** gewährt (Ausfall-Kompensation) **+ $90** übrig = **$218,86** | vorausbezahlt, echtes Geld | **voller Preis** |
| **Mistral paid** | **20 €** Credits (14.10.) | Pay-as-you-go | **voller Preis** |
| **DeepInfra** | **$8,49** Balance + **$91,05** Top-up | **reserviert, metered** | **voller Preis — schonen** |

**DeepInfra ist kein „letzter Rang", sondern ein zweckgebundener Topf:** PII (nur wo Mistral nicht
ausreicht) und Frontier. Für Linienarbeit wird er **nicht** angefasst. PII zuerst über Mistral —
DeepInfra ist die Ausnahme, nicht der Standard.

## 2 — Der Agenten-Test (kostenlose Modelle, Urteil + Werkzeug)

Zwei freie Modelle als Subagenten, identische Aufgabe: Zeile 35 aus dem Rat-Register wörtlich
zitieren **und** die Bootstrapping-Fork beantworten, an der Kimi `#none` scheiterte.
Richtig ist **(b) Syntax** (eine Register-Prüfung blockt den Bau des Hooks selbst).

| Modell | Werkzeug | Forks (3) | Befund |
|---|---|---|---|
| **`opencode/deepseek-v4.1-flash-free#low`** | ✅ | **1/1** (Bootstrapping ✅) | **taugt — $0** (n = 1, härtester Fork bestanden) |
| **`opencode/glm-5.3-flash-free#low`** | ✅ | **2/3** | Bootstrapping ❌ · ack-Baubarkeit ✅ · Register-RAT-ID ✅ |

**Korrektur:** GLM-5.3-Flash ist **nicht** „keine Urteile". Es besteht **2 von 3** Forks und scheitert
nur am **selbstreferenziellen Bootstrapping-Paradox** (Register-Prüfung blockt den eigenen Bau).
→ Für normale Urteile brauchbar; **nicht** für Architektur-Fragen, die sich selbst autorisieren.
DeepSeek-Free hat den härtesten Fork (Bootstrapping) bestanden — bleibt erste Wahl.

## 3 — Die korrigierte Leiter (für normale Arbeit)

| Rang | Modell | Topf | Grenzkosten |
|---|---|---|---|
| **1** | **`opencode/deepseek-v4.1-flash-free#low`** | OpenCode frei | **$0** |
| **2** | `opencode/glm-5.3-flash-free#low` — **nur Mechanik** | OpenCode frei | **$0** |
| **3** | **`mistral/mistral-large-4`** · `mistral/zai-glm-5-3` | **Mistral inklusive** (7,27 €) | **€0** bis erschöpft |
| **4** | **`deepseek/deepseek-v4-pro`** | DeepSeek paid (≈$90) | voller Preis |
| **5** | `mistral/*` nach Erschöpfung | Mistral paid (20 €) | voller Preis |
| **—** | **`deepinfra/…`** | **reserviert** | **nur PII + Frontier** |

## 4 — Die Rollen

| Rolle | Erste Wahl | Warum |
|---|---|---|
| **Linien** | `opencode/deepseek-v4.1-flash-free#low` | $0, Fork bestanden |
| **Leitstelle (Future)** | `opencode/deepseek-v4.1-flash-free#low` | $0, Fork bestanden — **nicht** GLM-Flash |
| **Mechanik/Delegation** | `opencode/glm-5.3-flash-free#low` | $0, Werkzeug ok, Urteile nein |
| **Rat-Struktur** | `mistral/mistral-large-4` (inkl.) | vorausbezahlt, 0,0043 €/Call kalibriert |
| **Tiefe/Backup** | `deepseek/deepseek-v4-pro` | ≈$90 vorausbezahlt |
| **PII** | **Mistral** (inkl., Vertragsdeckung) — **DeepInfra nur, wo Mistral nicht ausreicht** | reserviert |
| **Frontier (Kimi-Klasse)** | **DeepInfra** (z. B. Kimi K3) | nur hier verfügbar |

## 5 — Reichweite der Töpfe (gerechnet mit gemessenen Preisen)

| Topf | Preis/Call | Calls |
|---|---|---|
| Mistral inklusive (7,27 €) | 0,0043 € | **≈ 1 690** |
| Mistral paid (20 €) | 0,0043 € | ≈ 4 651 |
| **DeepSeek paid ($218,86, V4 Pro)** | $0,00793 | **≈ 27 600** |
| DeepInfra GLM-5.3-Flash ($99,54) | $0,001415 | ≈ 70 350 |
| **DeepInfra Kimi K3 (Frontier)** | ≈ $0,0388 | **≈ 2 560** |

→ **Die Frontier-Reserve ist der kleinste Topf:** ~3 300 Kimi-Calls. Genau deshalb bleibt DeepInfra
für PII und Frontier reserviert, nicht für Linienarbeit.

## 6 — Was offen bleibt

- **DeepSeek-Balance** ist Operator-Angabe (≈$90), nicht vom Gateway gemessen (kein Balance-Proxy).
- **DeepInfra-Grant** ($128,86) und **Balance** ($8,49) sind getrennt zu führen — ob der Grant
  verfällt, ist **nicht entscheidbar**.
- **n = 1 Fork** — für „GLM-Flash ganz raus" sollten 2–3 Forks laufen.

## Belege

`opencode models` (frei: 10, `cost 0/0`, variants low/high/max) · Subagenten-Test (2 Modelle) ·
`2026-10-18-modell-inventur-live.md` · `2026-10-18-kosten-kontingente.md` · Operator-Angaben (Töpfe)
