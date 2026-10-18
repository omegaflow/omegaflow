# Formular — future.md: Der Zusatz (Pass-Kontrakt + Bringschuld) — behalten oder streichen?

**Datum:** 2026-10-18 · **Linie:** future · **Register:** `R-2026-10-18-m`
**Status:** offen — UI (Fugu, Lumo) + Paid (DeepSeek, ggf. Mistral) fahren.

## Die Frage

Die Kommando-Datei `future.md` wuchs in drei Commits von **12 auf 21 Zeilen**:
(a) `Lage zuerst: sread state/stimmen/_lage.md` (2 Zeilen, R-2026-10-18-k),
(b) Pass-Kontrakt: „So viel wie möglich, nicht mindestens eins …" (ersetzt „mindestens eins"),
(c) „Bringschuld vor der Übergabe: Jeder offene Punkt nennt `Braucht` …".

Gemessen: der letzte Leitstellen-Pass auf dieser Datei kostete **2,46 EUR** (Betreiber-Rechnung;
opencode.db zählt 0,27). Der Betreiber nennt den Zusatz „compliance theater" und will die Datei klein.

**Optionen:**
- **A — alles behalten.** Die drei Sätze steuern den Pass (mehr abschließen, Bringschuld).
- **B — zurück auf 12 Zeilen.** „Lage zuerst", Pass-Kontrakt-Zusatz und Bringschuld streichen;
  der Auftrag (Status erfassen, Aufgaben verteilen) steht schon im Kern.
- **C — nur die Bringschuld behalten**, „Lage zuerst" und den „so viel wie möglich"-Satz streichen.
- **D — nur „so viel wie möglich" behalten**, Bringschuld streichen.

**Zusatzfrage:** Gehört die Kosten-Zeile (Burn) in jede Übergabe, oder nur, wenn sie über der Kappe liegt?

## Was der Rat entscheiden soll

1. A, B, C oder D — und warum?
2. Zusatzfrage: Burn-Zeile immer oder nur bei Überschreitung?

## Belege

- `git log -p -- .opencode/commands/future.md` (omegaflow-personal): `af6a27f`, `9b7914d`, `c303530`
- `state/handover/…` — der Pass: `handover-2026-10-18-future-folge1.md` (Burn `close 2.46`)
