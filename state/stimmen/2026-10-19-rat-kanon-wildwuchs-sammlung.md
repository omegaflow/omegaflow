# Sammlung — R-2026-10-19-a (Kanon statt Wildwuchs)

**Fragebogen:** `state/stimmen/2026-10-19-rat-kanon-wildwuchs-formular.md`
**Rahmen:** `docs/granit.md` (fünf Axiome, wörtlich) · `docs/council.yaml` (fünf Stimmen, wörtlich)
**Stand:** 2026-10-19 · Rohantworten getrennt; diese Seite ist die **Bilanz**, keine Verschmelzung.

## Die Matrix — hier eintragen

`ja` / `nein` / `pending` + Kurzriss. Leere Zellen sind **pending**, nicht „nein".

| Sitz | A Kanon | B Meßlatte | C Recherchepflicht | D Schreiben | E Ablage | F4 zwei Arten | F5 Dateien | erster Ort | Kernriss | Beleg |
|---|---|---|---|---|---|---|---|---|---|---|
| **Fugu Max** (UI, eingeloggt) | ja | ja — 4 Zahlen | nein, aber „gemessenes Offen" | ja + Gate für Kanon | | | | | „Der Kanon kanonisiert die Stille" | `…-fugu.txt` |
| **Lumo 2.0 Max** (UI, anonym) | ja | ja | nein | ja (Gate) | opencode.json allein | nein — „Mechanismus" fehlt | 2 | opencode.json | „260 README-lose Ordner = Datenpunkt 261" | `…-lumo.txt` |
| **Duck GPT-6** (UI, anonym) | ja | ja — 20 % Turns | nein | nein zu Rechten, ja Gate | zentrale Allowlist + CI | ja | 3 | AGENTS.md | „erstarrter Kanon" | `…-duck.txt` |
| **Perplexity** | | | | | | | | | blockiert: Login-Wand | — |
| **Qwen 3.8-Max** | | | | | | | | | blockiert: Login | — |
| **Claude** (Operator) | | | | | | | | | | |
| **DeepSeek Chat** (Operator) | | | | | | | | | | |
| **Z.ai GLM-5.3** (Operator) | | | | | | | | | | |
| **Mistral Vibe** (Operator) | | | | | | | | | | |
| **SciSpace** (Operator) | | | | | | | | | | |
| **Fugu 2. Lauf** (falls nötig) | | | | | | | | | | |

## Wo sie auseinanderfallen (nach dem Eintragen prüfen)

- **C**: Fugu „nein, aber gemessenes Offen" · Lumo „nein" · Duck „nein" — bislang **keine** Stimme für eine Pflicht.
- **F5**: Fugu offen · Lumo **2** · Duck **3** — und der erste Ort ist **strittig** (`opencode.json` vs. `AGENTS.md`).
- **F4**: Lumo sagt **nein** (dritter Typ „Mechanismus"); Duck sagt ja.
- **D**: Fugu will Schreibrecht je Ordner **plus** Gate für den Kanon; Duck/Lumo wollen **nur** das Gate.

## Vorbehalte, die im Ergebnis stehen bleiben

- Die ersten drei Läufe (vor der Granit-/council-Korrektur) sind in den Rohdateien als
  `VERWORFEN` markiert und zählen **nicht**.
- Perplexity verlangte nach der zweiten Frage einen Login; Qwen blockiert das Senden mit
  „Welcome to Qwen". Beide sind damit **nicht** gefahren, nicht „pending".
- Fugu orchestriert mehrere Modelle; ein Lauf dauert Minuten. Wer nach 20 s „pending" notiert,
  misst den Lauf nicht.

## Auswertung (Operator)

- **Tragende Stimme** (Streich-Test): welcher Riss würde das Verdikt kippen, wenn man ihn streicht?
- **Konvergenz ohne Konsens ist kein Beschluss** — die Bilanz nennt beide Seiten.
- **Entscheid:** offen. Der Operator schließt.
