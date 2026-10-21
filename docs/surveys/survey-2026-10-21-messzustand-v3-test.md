<!--
  title: v3 am Bestand — 24 Warten, 10 Riss-Cluster, 16 Fälle (2026-10-21)
  class: survey
  date: 2026-10-21
  sha256: 80bb2ff6f99557aacc1546a1ce7440729b956a02a52bcedad9739c7b8beb9b73
  status: live
  owner: future
  ack: future 2026-10-21T14:55Z
  next: explizite Zustandsfelder im Register; dann voller Bestand
  orphan-after: none
  see-also: docs/concepts/messzustaende-v3.md rat/2026-10-21-rat-messzustaende-v2.md
-->
# v3 am Bestand — 24 Warten, 10 Riss-Cluster, 16 Fälle

**Frage.** Trifft v3 (drei Achsen, Totalordnungen) auf echte Records **genau
eine Zelle je Achse** — oder bleiben Felder mit 0 oder ≥2 Zellen?

**Test.** `state/wartend.φ` (24 Zeilen) mit einem Keyword-Klassifikator; die 10
Riss-Cluster und die 16 Red-Team-Fälle manuell. Der Klassifikator ist ein
erster Pass, kein NLP.

## Ergebnis

| Testmenge | Records | 0 Zellen | ≥2 Zellen | Urteil |
|---|---:|---:|---:|---|
| `state/wartend.φ` | **24** | **17 → pending (Default)** | **0** | v3 trägt, aber 17 sind **stumm** |
| Riss-Cluster | 10 | 0 | 0 | 9 `pending`, 1 Begriffsriss |
| Red-Team-Fälle | 16 | 0 | 0 | je Achse genau eine Zelle |

**Die 17 stummen Warten** sind der Befund: Sie tragen **kein** explizites
`pending` — der Default macht sie zu einer Zelle. Der Bestand sagt den Zustand
nicht, das Modell nimmt ihn an. Genau das ist die **stille Lücke** aus
`survey-2026-10-21-versteckte-auftraege.md`.

**Die 7 mit State-Wort** (`emodnet-hfr`, `juice-recon`, `trisp-keller`,
`cindi-nwm`, `ceic-d18000`, `lro-lola-altimetrie`, `rbn-lizenz`, `dosis-3d`)
tragen meist `gemessen <datum>` als **Provenienz**, nicht als Zustand. Nach der
Trennung „`gemessen <datum>` = Stand, nicht Zustand" bleiben sie bei einer
Zelle.

## Die 10 Riss-Cluster

Unter v3: **9 `pending`** (Kette/Methode/Zugang), **1 Begriffsriss**
(Kanal-Ontologie). Kein Datenriss. Die Totalordnung `absent > riss > null-echt >
obergrenze` ändert daran nichts — sie verhindert nur, dass zwei Qualifier
gleichzeitig zählen.

## Die 16 Red-Team-Fälle

Jeder Fall wird durch v3 auf genau eine Zelle je Achse abgebildet:

- `null-echt` vs. `obergrenze` → Totalordnung (Fugu, Qwen, Gemini, GLM).
- `riss` vs. `obergrenze` → Totalordnung (Perplexity, Mistral, MiniMax).
- `obergrenze == L_D` → `≤` (Lumo, DeepSeek).
- `declined + LOCK` → Achse 2 + Achse 3 (Sonnet, SciSpace, Kimi).
- `gemessen + LOCK` vs. `pending/nicht_verfügbar` → `gemessen` + Achse 3,
  Projektion „versiegelt" (Opus, Claude).
- Teildatensatz → `pending/teil_datensatz` (Kimi).

## Der Klassifikator (Auszug)

```awk
# Achse 1: pending | gemessen | absent | riss | null-echt | obergrenze
p = (note ~ /pending|offen|fehlt|ausstehend|nicht gemessen/)
g = (note ~ /gemessen|measured/) && (note !~ /gemessen 20[0-9][0-9]-/)
a = (note ~ /absent/); r = (note ~ /riss/)
n = (note ~ /null-echt/); o = (note ~ /obergrenze|< ?L/)
c1 = p+g+a+r+n+o
if (c1==0) c1 = 1   # Default pending
# Achse 2: offen | wartend | declined; Achse 3: frei | LOCK
```

**Grenze.** Der Klassifikator liest Prosa. Er kann `gemessen <datum>`
(Provenienz) von `gemessen` (Zustand) nur über die Datumsform trennen; ein
Record mit beiden Bedeutungen bleibt mehrdeutig. Deshalb ist das Ergebnis
**24/24 eine Zelle** nur unter der Default-Regel — nicht, weil der Bestand
eindeutig ist.

## Was folgt

1. **Explizite Zustandsfelder** im Register: `zustand` (Achse 1), `lebenszyklus`
   (Achse 2), `zugang` (Achse 3). Prosa bleibt Beleg, nicht Zustand.
2. **`gemessen <datum>`** wird als `stand` geführt, nicht als `zustand`.
3. Danach der **volle Bestand** (`phi/sources.φ`, Handover, Projekte) — nicht
   vorher.

**Status:** v3 ist am kleinen Bestand konsistent; die stille Lücke ist die
nächste Arbeit, nicht eine weitere Runde.
