# Netto-Rollen — Grenzkosten statt Listenpreis (gemessen 2026-10-18)

**Korrektur des Maßstabs.** Bisher habe ich Listenpreise gerankt. „Netto" heißt: Was kostet der
**nächste** Call, gegeben das Kontingent, das der Operator **schon bezahlt hat**?

## 1 — Die Kontingente (wer zahlt was)

| Quelle | Stand | Charakter | Grenzkosten je Call |
|---|---|---|---|
| **OpenCode frei** (`opencode/*-free`) | 10 Modelle, `cost 0/0` | frei, rate-limitiert | **$0** |
| **Freie Träger** (Gateway) | 44 gemessen | frei | **$0** |
| **Mistral** (Abo) | **7,27 €** Rest von 8,50 € | **vorausbezahlt**, verfällt monatlich | **€0 bis erschöpft** (danach ~0,0043 €) |
| **DeepSeek direkt** | „riesen Kontingent" (Operator-Angabe, **ungemessen**) | vorausbezahlt | **$0 bis erschöpft** |
| **DeepInfra** | **$8,49** Balance + **$91,05** Top-up | **Pay-as-you-go, echtes Geld** | **voller Preis** — der kostbarste Topf |

→ **DeepInfra ist der einzige Topf, der bei jedem Call wirklich Geld verbrennt.** Die anderen sind
vorausbezahlt oder frei. Darum: DeepInfra nur für das, was **nichts anderes** trägt.

## 2 — Der Agenten-Test (nicht nur „BEREIT", sondern Urteil + Werkzeug)

Zwei **kostenlose** Modelle als Subagenten, identische Aufgabe: Zeile 35 aus dem Rat-Register
**wörtlich zitieren** (Werkzeug) **und** die Bootstrapping-Fork beantworten, an der Kimi `#none`
gescheitert ist. Richtig ist **(b) Syntax**, weil eine Register-Prüfung den Bau des Hooks selbst blockt.

| Modell | Werkzeug (Zitat) | Fork | Befund |
|---|---|---|---|
| **`opencode/deepseek-v4.1-flash-free#low`** | ✅ korrekt | **✅ (b)** + Bootstrapping-Riss wörtlich | **taugt** — $0 |
| **`opencode/glm-5.3-flash-free#low`** | ✅ korrekt | **❌ (a)** — Register-Prüfung | **taugt nicht für Urteile**; Output zusätzlich mit Sprachmüll („Form邮 малыша") |

**Das ist derselbe Fehlertyp wie Kimi `#none`.** Bezahlter GLM-5.3 **voll** `#low` hatte die Fork
**richtig** gewählt (`rat-gate-rat-id.md:131`). Also: **GLM-5.3-Flash ist die schwache Variante**,
nicht der Provider.

Grenze: **n = 1 Fork je Modell.** Indikativ, nicht endgültig — für ein Urteil sollten 2–3 Forks
laufen. Das Signal deckt sich aber mit dem bekannten Muster.

## 3 — Die ehrliche Netto-Rangfolge

| Rang | Modell | Grenzkosten | Geeignet für |
|---|---|---|---|
| **1** | **`opencode/deepseek-v4.1-flash-free#low`** | **$0** | **Linien + Leitstelle** (Fork bestanden) |
| 2 | `opencode/glm-5.3-flash-free#low` | **$0** | **nur Mechanik** (Fork gerissen) |
| 3 | **`mistral/mistral-large-4`** / `mistral/zai-glm-5-3` | **€0 im Kontingent** (7,27 € ≈ 1 690 Calls) | Rat-Struktur, Urteile (vorausbezahlt) |
| 4 | **`deepseek/deepseek-v4-pro`** (direkt) | **$0 im „riesen" Kontingent** | Tiefe / Backup, wenn frei rate-limitiert |
| 5 | **`deepinfra/…`** (GLM-5.3-Flash, DeepSeek-V4.1-Flash-1017, V4-Pro) | **voller Preis** | **nur wenn 1–4 nicht tragen** (z. B. Rate-Limit-Ausfall) |

## 4 — Die Rollen (aus der Rangfolge, nicht aus dem Listenpreis)

| Rolle | Erste Wahl | Begründung |
|---|---|---|
| **Linien** | `opencode/deepseek-v4.1-flash-free#low` | $0, Fork bestanden, Werkzeug funktioniert |
| **Leitstelle (Future)** | `opencode/deepseek-v4.1-flash-free#low` | $0, Fork bestanden — **nicht** GLM-Flash |
| **Mechanik/Delegation** | `opencode/glm-5.3-flash-free#low` | $0, Werkzeug ok; Urteile nicht |
| **Rat-Struktur** | `mistral/mistral-large-4` | vorausbezahlt, 0,0043 €/Call kalibriert |
| **Tiefe/Backup** | `deepseek/deepseek-v4-pro` | großes Kontingent |
| **Letzte Reserve** | DeepInfra | echtes Geld, nur bei Ausfall der anderen |

## 5 — Was noch fehlt (und was ich nicht messen kann)

- **DeepSeek-Balance:** „riesen Kontingent" ist **Operator-Angabe**, nicht von mir gemessen. Der
  Gateway kann sie nicht abfragen (kein `/user/balance`-Proxy). Wenn du den exakten Stand willst:
  Konsole ablesen oder den Key kurz in einen direkten Call geben.
- **GLM-Free vs. GLM-Flash:** Der Fork-Test ist n = 1. Für die Entscheidung „Leitstelle auf DeepSeek
  Free" reicht das Signal; für „GLM-Flash ganz raus" sollten 2–3 Forks laufen.
- **OpenCode-Rate-Limits** sind ungemessen — der Grund, warum Rang 4/5 als Backup bleiben.

## Belege

`opencode models` (frei: 10; `cost 0/0`; variants low/high/max) · Subagenten-Test (2 Modelle,
identische Fork) · `2026-10-18-modell-inventur-live.md` · `2026-10-18-kosten-kontingente.md`
