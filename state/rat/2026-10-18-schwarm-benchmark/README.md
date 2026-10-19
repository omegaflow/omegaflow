# Schwarm-Benchmark — freier Rat gegen Paid-Rat (2026-10-18)

**Frage des Operators:** Kann ein Schwarm **freier** Modelle dieselbe Ratsqualität liefern wie
die Paid-Frontier-Sitze — und welche Aufgaben lassen sich ganz an freie Agenten geben?

**Aufbau:** Der Operator stellt nur die Frage. Ein **freies Modell ist Koordinator**, befragt
selbst 4 freie Modelle und verdichtet. Der Agent taucht in die Einzelantworten **nicht** ein
(nur zur Diagnose gelesen, hier ausgewiesen).

**Schwarm (gemessen antwortfähig):** `cf/@cf/openai/gpt-oss-120b` · `kenari/agnes-3-0-flash:free` ·
`pollinations/openai-fast` · `ollama/gemma4:31b`.
**Koordinator:** `kenari/agnes-3-0-flash:free`.

**Ausgeschlossen (HTTP 200, aber leerer Inhalt):** `groq/openai/gpt-oss-120b` · `groq/openai/gpt-oss-20b` ·
`kilo/kilo-auto/free` · `kilo/reflection/beam-free` · `llm7/GLM-5.3-Flash`.
→ **Befund:** HTTP-Erfolg ≠ Antwort. Ein Schwarm aus „200 OK" ist ein Schwarm aus Nullen.

**Kosten:** $0. Laufzeit: ~1 s je Modellantwort.

## Ergebnis gegen die abgeschlossenen Runden

| Frage | Paid-Rat | Freier Schwarm | Übereinstimmung |
|---|---|---|---|
| **R-i** Offen-Stand-Zähler | **A** (4/4) | **B** (3:1) | **✗ konträr** |
| **R-j** `K` identifizierbar | **A** (5/5) | **A** (4/4) | ✅ **gleich** |
| **R-k** Native Fläche | **C** (4/4) | **C** (3:1) | ✅ **gleich** |

**Trefferquote: 2 von 3.**

## Der Fehlgriff — und was er zeigt

**R-i nachgemessen mit einem Koordinator, der nach Argument statt nach Anzahl entscheiden soll**
(`verdikt-i-koordinator-nachArgument.md`): Er klassifiziert diesmal selbst **3:1 für A**
(M1=A, M2=B, M3=A, M4=A) — und wählt **trotzdem B**, ausdrücklich „nach Argument, nicht Anzahl",
mit einem Riss gegen das eigene Verdikt.

→ **Kein Zählfehler, keine Prompt-Schwäche.** Der Koordinator hält B wirklich für das stärkere
Argument. R-i ist ein **Zielkonflikt** („was *ist* ein offener Punkt — Zeile oder Abschnitt?"),
kein Sachverhalt. Darauf gibt es zwei vertretbare Antworten.

## Die Befunde

1. **Bei Sachfragen ist der Schwarm gleichwertig.** R-j und R-k: dasselbe Verdikt, dieselben
   Argumente — und **schärfere Risse** als der Paid-Rat.
   Beispiel (R-k, Koordinator): *„Die native API modelliert synchronen, hierarchischen
   Session-Zustand, während die Haus-Lösung asynchrone, adressierte Delegation macht."*
   Das stand in keinem Paid-Beitrag.
2. **Bei Zielkonflikten wägt der Schwarm anders — nicht schlechter.** Er trägt aber keine
   Verantwortung: das Paid-Verdikt ist im Register und wird von Sitzen getragen, der
   Schwarm-Riss ist eine Mindermeinung ohne Träger.
3. **Der eigentliche Gewinn sind die Risse, nicht das Verdikt.** In **3 von 3** Runden hat der
   Schwarm einen Einwand produziert, den kein Paid-Sitz hatte:
   - R-i: *„Option A misst die Tat, aber misst **inhomogene** Einheiten (Liste vs. Header) —
     damit wird `closed <n>` ein statistisches Artefakt."*
   - R-j: *„Die signifikante bedingte TE beweist eine lokale Kopplung, liefert aber kein Maß
     für ihre Stärke — die Verlockung zur vorzeitigen `K`-Identifikation."*
   - R-k: *„Funktionale Äquivalenz ist nicht semantische Identität — die native Adoption hebt
     die knappe Übergabe auf."*
4. **Der Ausfall ist unsichtbar.** Fünf von neun „freien" Modellen antworten leer bei HTTP 200.
   Ein Schwarm ohne Inhaltsprüfung misst Rauschen.

## Die Regel, die daraus folgt

**Der Schwarm geht VOR die Paid-Sitze — als Riss-Generator, nicht als Entscheider.**

```
1. Frage steht (Formular)
2. Freier Schwarm fährt sie  →  Risse + Mindermeinung  (frei, ~Sekunden)
3. Die Risse wandern INS Formular an die Paid-Sitze
4. Paid entscheidet und trägt          (Urteil + Autorität)
5. Sammlung nennt beide: Paid-Verdikt UND Schwarm-Risse
```

Das ist die Teilung, die der Rat in **R-k selbst** beschlossen hat: **Substrat frei, Domäne bezahlt.**

## Was mechanisch an freie Agenten kann — GEMESSEN (2026-10-18)

**Der erste Test schlug fehl — am Aufbau, nicht am Modell.** Ein Modell über den **Gateway**
(Chat-API) hat **keinen Werkzeugzugang**: `kenari/agnes-3-0-flash:free` antwortete korrekt
*„I cannot execute shell commands"*. Der Kanal erlaubt nur Text.

**Der richtige Kanal ist der OpenCode-Subagent.** Es gibt **sieben freie Modelle als
OpenCode-Provider** (nicht über den Gateway): `opencode/nemotron-3.5-super-vl-free` · `beam-free` ·
`longcat-2.5-preview-free` · `space-bunny-free` · `mimo-v2.6-flash-free` ·
**`deepseek-v4.1-flash-free`** · `glm-5.3-flash-free`.

**Gefahren:** `general`-Subagent auf `opencode/deepseek-v4.1-flash-free`, zwei Aufgaben
(Offen-Stand erfassen, Session-Zustand), Kriterium **vorher** festgelegt.

| Kriterium | Ergebnis |
|---|---|
| **Zahlentreue** | ✅ alle Zahlen wörtlich, keine gerundet |
| **Vollständigkeit** | ✅ alle Befehle ausgeführt |
| **Keine Erfindung** | ✅ jeder Wert folgt aus einer Ausgabe |
| **Fehler benannt** | ✅ — und er hat **meinen** Fehler korrigiert |
| **Form** | ✅ exakt die fünf Zeilen, kein Kommentar |

| Prüfpunkt | erwartet | geliefert |
|---|---|---|
| A1 `--addressed future` | 3 | **3** ✅ |
| A2 `--fired future` | 0 | **0** ✅ |
| A3 `xfer … --open` | 0 | **0** ✅ |
| B1 offene Pfade | 10 (mein Messzeitpunkt) | **13** (sein Zeitpunkt) — beide richtig; die Zahl wuchs laufend (10→13→15) |
| B2 `cost` | `null` (**mein** Filter war falsch) | **2,4105…** ✅ er hat den echten Wert geholt |

**Der Befund:** Der freie Agent war **besser als meine Anweisung** — er hat den fehlerhaften
`jaq`-Filter nicht blind übernommen, sondern den Wert geholt, den die Aufgabe verlangte.

**Die Grenze, die bleibt:** „n Pfade offen" ohne Zeitstempel ist eine Momentaufnahme, kein
Zustand — solange parallel gearbeitet wird, ist die Zahl nicht reproduzierbar.

## Die Teilung, belegt

| Aufgabe | Wer |
|---|---|
| **Offen-Stand lesen** (Sessionanfang) | **freier Subagent** (`opencode/deepseek-v4.1-flash-free`) |
| **Session abschließen** (Kosten, offene Pfade) | **freier Subagent** — inkl. Fehlerkorrektur |
| **Rat bei Sachfragen** | freier Schwarm, gleichwertig |
| **Rat bei Zielkonflikten** | **Paid** — der Schwarm trägt nicht |
| **Urteil + Register-Schreibakt** | **Paid** |
