# Schwarm — drei Leitstellen-Fragen (2026-10-18)

**Formular:** `formular.txt` · **Kosten:** **$0,00** (Gateway, free-Modelle) · **Fahrzeit:** ~40 s je Sitz.
**Sitze (gemessen antwortfähig, `_rat-anweisung.md §3b`):** `cf/@cf/openai/gpt-oss-120b` ·
`kenari/agnes-3-0-flash:free` · `pollinations/openai-fast` · `ollama/gemma4:31b`.
**Koordinator:** `kenari/agnes-3-0-flash:free` — Eingabe ist die wörtliche Sitzantwort (`koordinator-eingabe.txt`),
die Einzelantworten habe ich **nicht** gelesen, nur als Diagnose unten gesammelt.

**Grenze:** der Schwarm **trägt nicht**. Sein Verdikt ist eine Mindermeinung; seine Risse sind der Wert.
Kein Register-Schreibakt aus diesem Lauf.

## Der Ausfall — sichtbar gemacht

| Sitz | Zustand | in / out | finish |
|---|---|---|---|
| kenari/agnes-3-0-flash | **OK** | 1420 / 1001 | stop |
| ollama/gemma4:31b | **OK** | 1403 / 404 | stop |
| cf/@cf/openai/gpt-oss-120b | **abgeschnitten** (F1+F2, kein F3) | 1349 / **2400** | **length** |
| pollinations/openai-fast | **LEER** (HTTP 200, kein Inhalt) | 1590 / 1500 | length |

→ **4 von 4 „200 OK", 2 von 4 brauchbar.** Derselbe Befund wie im Benchmark: HTTP-Erfolg ≠ Antwort.

## Die Bilanz

| Frage | Schwarm | Paid-Vorrunde | Lage |
|---|---|---|---|
| **F1** Quittung im Ledger | **A 2 : B 1** (A+S1: kenari, gemma · B+S2: gpt-oss) | offen (R-a, nie gefahren) | drei Sitze, zwei Ablageorte |
| **F2** `register-coverage`-Schlüssel | **B 3 : A 0** | **3 : 3 Patt** | Schwarm schlägt eine Seite — er löst das Patt nicht |
| **F3** `state/rat` lesen | **A 2 : 0** (kenari, gemma; gpt-oss fehlt) | offen (nie gefahren) | beide sagen „RAT-ID vorher: ja" |

## Die Risse, die nur ein Sitz hatte (der Wert)

**F1**
- **kenari:** S1 löst die „zwei offenen Fälle" **nur anzeigeseitig** — der Zustand bleibt im Emitter-Ledger.
- **kenari:** „Wenn A gilt, muss klar sein, **warum** der frühere `xfer ack`-Arm (`eb2f1b749`, −202 Zeilen)
  fehlerhaft war" — sonst reintroduziert A dieselbe Komplexität.
- **gemma:** A verteilt den Zustand auf **zwei** Ledger (Verwaltungsaufwand).
- **gpt-oss (Mindermeinung):** der **Rückzug** von `xfer ack` ist der stärkste Gegenbeleg gegen A.

**F2**
- **kenari:** `cancel mid-write` kann einen inkonsistenten Registerzustand hinterlassen → das Gate muss
  `cancelled` **ignorieren** (Kimis Einwand, hier nur **zitiert, nicht gemessen**).
- **gpt-oss (abgeschnitten):** das per-SHA-Lager liefert **keinen** Nachweis, dass mehr Granularität
  **keine** inkonsistenten Zustände erzeugt.

**F3**
- **gemma:** Rauschen/Performance-Last im `register_lookup`, falls `state/rat/` viele Dokumente trägt —
  **ungemessen**.
- **kenari + gemma, beide:** die Wahl ist ein **Architektur-Akt** → **RAT-ID muss vor dem Bau im Index stehen**.

## Was der Schwarm nicht trägt

1. **F2:** Drei Mal B heißt nicht, dass B richtig ist — der Paid-Rat stand 3:3. Die Schwarm-Seite
   stützt sich auf **Kimi zitiert**, nicht auf eine eigene Messung des `mid-write`-Falls.
2. **F1:** Die Mindermeinung B+S2 stützt sich auf **einen abgeschnittenen** Sitz — das ist
   ein Stimmengewicht, dem die zweite Hälfte fehlt.
3. **F3:** Option **C** (separater Index) und **D** (status quo) wurden von **keinem** Sitz diskutiert —
   sie stehen im Formular, aber nicht in der Bilanz. Das ist die Lücke der Schwarm-Runde.

## Rohmaterial

`F1F2F3-kenari-agnes-3-0-flash.txt` · `F1F2F3-ollama-gemma4-31b.txt` ·
`F1F2-gpt-oss-120b-abgeschnitten.txt` · `pollinations-openai-fast-LEER.txt` ·
`koordinator-eingabe.txt` · `koordinator-sammlung.txt`
