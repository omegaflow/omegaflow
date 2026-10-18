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

## Was mechanisch an freie Agenten kann (offen, nicht gemessen)

Der Operator fragte zusätzlich, ob **Offen-Stand lesen** (Sessionanfang) und **Session abschließen**
an freie Agenten können. Das ist **Befolgen**, nicht Urteilen — plausibel ja, aber **ungetestet**.
Kriterium liegt vor (Zahlentreue · Vollständigkeit · keine Erfindung · Fehler benannt).
**Kein stilles `pending`: der Test ist vorbereitet und nicht gefahren.**
