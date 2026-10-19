# ENTSO-E Intervall-Form — Rat ↔ Roster, gewichtet (2026-10-13, mountain-340)

## Frage (verbatim, an alle Seats identisch)

Öffentliche Fachfrage (Datenmodell-Bewertung). Ein punktbasiertes Raumzeit-Feld speichert
Messungen als feste Records: Position (x,y,z), ein skalarer Wert, eine Epoche (Zeit), ein tau
(zeitliche Erstreckung; tau=0 = kein Bestand, Tor zu). Abzubilden sind Ereignis-Intervalle aus
einem Netz-Transparenz-Feed: Start, Ende, kategorialer Ereigniscode, variables Gebiet (EIC-Code,
keine Koordinate), teils eine Menge in MW (teils abwesend). Wie diese Intervalle abbilden, ohne
Werte zu erfinden? (a) epoch=Start + tau=Dauer, Menge als Wert, Gebiet als Kanal-Identitäts-
Qualifier; (b) Aggregat über Zeitfenster; (c) neuer Record-Typ; (d) sonst? Beachte die abwesende
Menge und den fehlenden räumlichen Anker. Antworte mit Verdikt + kurzer Begründung.

## Antworten (gemessen 2026-10-13)

| Seat | Verdikt | Kern |
|---|---|---|
| Rat (5 Stimmen) | **(a)** epoch=Start + tau=Dauer | kein neuer Record; nicht der generische `(t,val,comp)`-Serien-Pfad; `val` mandatory → fehlt Menge, Record übersprungen; Gebiet+Code datengetriebener Kanal-Qualifier |
| Claude (Sonnet 5.5) | **(a) + 2 Pflichtkorrekturen = faktisch (d)** | Abwesenheit über Gültigkeits-Flag (nicht 0/NaN); keine Koordinate erfinden, Position „nicht anwendbar" deklarieren; Code ins Kanalschlüssel-Tupel; kein neuer Typ nötig |
| Qwen | **(c)** neuer Record / Schema-Erweiterung | starres (x,y,z)+Skalar erzwingt Dummy-Koordinaten/Dummy-Werte |
| Duck (GPT-6 Luna) | **(c)** eigener Ereignis-/Intervall-Record | EIC ist kein räumlicher Anker; Code kein Skalar |
| Sakana (Namazu) | **(c)** neuer Record-Typ | kein (x,y,z)-Zwang, kein tau-Missbrauch, keine erfundenen Werte |
| DeepSeek Chat | **(c)** neuer Record-Typ | Menge nullable, eigene Felder |
| DeepSeek V4 Pro (open-weight) | **(c)** neuer Record-Typ | Punktfeld erzwingt Koordinate + einen Skalar |
| Z.ai (GLM-5.3 Deep Think Max) | **pending** | Antwort rechnet noch im Chat (Deep Think Max); nicht abgeholt |
| Perplexity · Mistral · AI Studio (Gemini 3.1 Pro) · MiniMax · Lumo · Kimi | **nicht gestellt** | Tabs offen, Composer nicht gefüllt — kein Verdikt |

Rohmaterial: `state/stimmen/2026-10-13_duck_entsoe.json` · `…_claude_entsoe.json` ·
`…_zai_entsoe.json` (nur Prompt, kein Verdikt).

## Gewichtet (nicht gezählt)

- **Einig (alle):** nichts erfinden — keine Koordinate, kein `0` für abwesende Menge, kategorialer
  Code gehört nicht in `val`.
- **Riss (nicht geglättet):** 5 Seats (Qwen, Duck, Sakana, DeepSeek Chat, DeepSeek V4 Pro) → **(c)
  neuer Record-Typ**; Rat (Struktur-Autorität für Architektur, Operator-Wort 2026-10-06) + Claude
  → **kein neuer Record**. Die Roster-Mehrheit wiegt schwer, sie argumentiert aber über ein
  *generisches* festes Record-System.
- **Auflösung am Kontrakt** (gemessen am Baum, nicht behauptet):
  1. Das Intervall trägt der Punkt-Record über `epoch`=Start + `tau`=Dauer (Rat).
  2. Die **abwesende Menge** hat im Wire einen Ort: den `presence`-Slot (26 × f64,
     `docs/concepts/archivar-mathematikerin.md`) — keine 0, kein NaN.
  3. Der **fehlende räumliche Anker** hat einen Ort: das Gebiet ist die **deklarierte
     Receiver-Weltlinie** (Operator-Wort 2026-10-06; `docs/surveys/survey-2026-10-06-agnostik-llm-verdikt.md:82`)
     — nicht eine erfundene Koordinate. Gebiet + Ereigniscode = Kanal-Identität.
  4. Ein **neuer Record-Typ** widerspricht dem festen Wire-Kontrakt (26 × f64); ein Riss ist kein
     Sample-Slot (`archivar-mathematikerin.md`).
- **Verdikt (mountain-340):** (a) mit den Roster-Korrekturen — **kein neuer Record-Typ**; der
  Punkt-Record trägt `epoch`/`tau`/`val`+`presence`, Gebiet+Code als Kanal-Qualifier, das Gebiet
  als Receiver. Der Roster-Riss bleibt benannt; er ist über Receiver + presence geschlossen, nicht
  geglättet.
