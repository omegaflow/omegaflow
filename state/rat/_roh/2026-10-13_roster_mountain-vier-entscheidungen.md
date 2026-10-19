<!-- Rohmaterial der UI-Runde (Mountain, 2026-10-13) — der Rat/Session trägt das Verdikt. -->
# UI-Runde — vier Entscheidungen (Mountain-Linie, 2026-10-13)

Frage (eine Runde, ein Akt): vier Entscheidungen, je Zustimmung/Abweichung + stärkster Einwand.
Seats: Duck.ai (GPT-6 Luna) · Claude · Qwen (Qwen3.7-Plus) · Nemotron 3 Ultra (über tryingopen) · GLM/Z.ai (pending, capacity).

## Duck.ai (GPT-6 Luna)
1. Zustimmung. Einwand: erst Höhenbezug und Rasterauflösung des Messwerts als vergleichbar sicherstellen.
2. Zustimmung. Einwand: prüfen, dass die geteilte Datei keine gemeinsam genutzte Verdrahtung enthält, die für die eigenen Leser mitgeändert werden muss.
3. Zustimmung. Einwand: 404 und unbekannte Lizenz sind ungelöste Beschaffungs-/Nutzungsrisiken, keine erledigten Ersatzquellen.
4. Abweichung. Nicht allein deshalb auf Binär umstellen, weil der Leser es erwartet — erst den maßgeblichen Schnittstellenvertrag klären, dann Produzent und Leser konsistent angleichen und per Test absichern.

## Claude
1. Zustimmung mit Auflage. Einwand: ein einzelner Messpunkt prüft schwach; SRTM15+ ≈ 450 m Zellgröße; Differenz kann Gelände oder Höhenbezug (Geoid vs. Ellipsoid) sein. Auflage: Toleranz und Höhenbezug **vor** der Prüfung festlegen; die zwei früheren Global-Registrierungen klären.
2. Zustimmung. Einwand: geteilte Datei kann inkonsistent werden, wenn eigene Blöcke Importe/Reihenfolge/Nachbarn voraussetzen, die die andere Linie ändert. Auflage: zuerst die drei Leser speichern; nur eigene Blöcke stagen; Diff prüfen; Build/Test gegen den aktuellen Fremdstand.
3. Zustimmung. Einwand: „gemessen schon im Register" hängt an der Duplikat-Definition (Name/URL ≠ Inhalt). Auflage: Duplikat per Inhalts-Hash/Messwert belegen; bei der 404-Vorquelle Mirror/Archiv prüfen; unbekannte Lizenz = nicht registrieren.
4. Zustimmung mit Auflage. Einwand: „Vertrag gehört dem Leser" gilt nur, wenn er explizit ist (Endianness, Header, Version); JSON-Programm könnte andere Konsumenten haben. Auflage: Round-Trip mit dem echten Leser vor Register/Veröffentlichung; JSON-Konsumenten prüfen.

## Qwen (Qwen3.7-Plus)
1. Zustimmung. Einwand: lokale Validierung am Messpunkt blindet für globale Inkonsistenzen und Kachelrand-Artefakte.
2. Abweichung. Einwand: blockweises Speichern umgeht Merge-Mechanismen; Line-Shifts der Parallel-Linie → Silent Overwrites. Forderung: Datei logisch trennen oder Rebase erzwingen.
3. Abweichung. Einwand: 404 und Unknown-License sind aktive Defekte, keine bloßen Lücken. Forderung: Quarantäne statt bloßes Behalten.
4. Zustimmung. Einwand: Binär zerstört Debuggbarkeit; ein JSON→Binär-Adapter hätte den Kern entkoppelt.

## Nemotron 3 Ultra (NVIDIA, über tryingopen)
1. Zustimmung. Einwand: eine Kachel deckt lokale SRTM-Artefakte nicht ab; systematischer Bias bliebe unentdeckt.
2. Abweichung. Einwand: block-scoped hilft nicht, wenn die andere Linie denselben Block bearbeitet; ohne Locking/Merge-Regel droht stiller Datenverlust.
3. Zustimmung. Einwand: die zwei Lücken blockieren vollständige Ersetzbarkeit.
4. Abweichung. Einwand: erst Konsumenten des JSON-Inventarisieren, sonst stiller Bruch nachgelagerter Pipelines.

## GLM / Z.ai
`pending` — gemessener Grund: „Model is currently at capacity … currently in peak hours" (Tages-/Kapazitätslimit), keine Antwort. Kein stiller Ausfall.
