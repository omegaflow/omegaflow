# Roster-Runde — EOP-Zeilenwahl (Mountain, 2026-10-14)

Frage: Für ein Feld „Erdrotation JETZT" (USNO `finals2000A.all`, Toleranz ~0.1 ms UT1 ≈ 7 nrad) — welche Zeile tragen: (a) letzte gemessene Bulletin-A-Zeile (Flag `I`, ~1 Tag), (b) letzte endgültige Bulletin-B-Zeile (~1 Monat), (c) fernste Vorhersage? Und: CelesTrak (`EOP-All.csv`, gemessen inhaltsgleich) behalten oder streichen?

Rohtexte: `state/stimmen/2026-10-14-eop-seat-<seat>.json`.

## Antworten (gemessen)

| Seat | Modell | Verdikt | CelesTrak |
|---|---|---|---|
| Duck.ai | GPT-6 Luna | (a) — jüngster A-Wert mit Flag I, nicht die letzte Datei-Zeile | streichen |
| Tryingopen | Inkling (Thinking Machines) | (a) — reine Messung, nicht der fernste Vorhersage-Block | streichen |
| DeepSeek | DeepSeek (DeepThink+Search) | (a) — letzte A-Zeile mit Flag I; B nur nachträglich | streichen |
| Lumo | Lumo 2.0 Max | (a) — letzte gemessene A-Zeile; B für historisch; (c) nur Lückenfüller | streichen |
| Qwen | Qwen3.7-Plus (Denken) | (a) — einzig korrekte Wahl; Flag `I` (auch `R`) | (nicht adressiert) |
| Z.ai / GLM | GLM-5.3 Deep Think Max | (a) — A-Rapid `I`, ~1 Tag, wenige 10 µs; bisherige Praxis = Bug | streichen (nur Failover) |
| Claude | Sonnet 5.5 Extra hoch | (a) — letzte `I`-Zeile, auf jetzt interpolieren; 1–2 `P`-Tage als Brücke; Stale-Flag nach 2–3 Tagen | streichen (nur kalter Fallback) |
| Mistral | Vibe (Denken) | in-flight (rechnet) | — |
| Kimi | K2.6 | Tageslimit überschritten — gemessene Nicht-Antwort | — |
| MiniMax | Agent-UI | kein Composer/Send erreichbar — gemessene Nicht-Antwort | — |
| Google AI Studio | Gemini 3.1 Pro | Antwort-UI nicht lesbar (`Response ready`, Panel statt Text) | — |

## Ergebnis

**Sieben Seats konvergieren mit Rat + Recherche:** Jetzt-Epoche = **letzte gemessene Bulletin-A-`I`-Zeile** (auf „jetzt" interpolieren; die ersten 1–2 `P`-Tage als Brücke zulässig; Stale-Flag, wenn die letzte `I`-Zeile länger als ~2–3 Tage zurückliegt); Bulletin B für vergangene Zeitpunkte; die fernste Vorhersage nie als „jetzt"; **CelesTrak als eigene Quelle streichen** (höchstens kalter Failover-Spiegel hinter USNO).

Bedien-Lehre (mein Fehler, korrigiert): Enter sendet bei Duck.ai/Z.ai/Tryingopen **nicht** — der Senden-Button ist Pflicht; Suche-Toggle vor dem Senden setzen. Nach der Korrektur antworteten die Seats.
